//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash e5101c69
//! @prompt 00_nucleo/prompts/rules/show-regex.md
//! @prompt 00_nucleo/prompts/rules/style/font-dict.md
//! @prompt-hash a6b22960
//! @layer L1
//! @updated 2026-06-22
//!
//! Show rules e set rules — aplicação e intercepção. Extraído de `eval.rs`
//! no Passo 96.1 conforme ADR-0037 (coesão por domínio).

use std::sync::Arc;

use std::str::FromStr;

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::entities::args::Args;
use crate::entities::ast::AstNode;
use crate::entities::ast::code::{SetRule, ShowRule as ShowRuleNode};
use crate::entities::ast::expr::{Arg, ArrayItem, Dict, DictItem, Expr};
use crate::entities::content::Content;
use crate::entities::engine::Engine;
use crate::entities::font_book::FontWeight;
use crate::entities::lang::Lang;
use crate::entities::show::{NodeKind, Selector, ShowRule, Transformation};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::style::Styles;
use crate::entities::style_chain::StyleChain;
use crate::entities::value::Value;
use crate::entities::world_types::check_show_depth as route_check_show_depth;
use crate::rules::scopes::Scopes;

use super::{closures, eval_expr, EvalContext};

/// Helper partilhado para construir warning de propriedade não suportada
/// em `#set` (Passo 107, encerra DEBT-49). Formato consistente para o
/// utilizador final.
///
/// Passo 134: parâmetro `adr_ref` opcional — `Some("0040")` referencia
/// catálogo vivo (set text); `None` produz hint genérico (set par, até
/// ADR dedicada existir).
fn unsupported_property_warn(
    target: &str,
    field: &str,
    adr_ref: Option<&str>,
) -> (String, String) {
    let msg = format!("{target}: propriedade '{field}' ainda não suportada");
    let hint = match adr_ref {
        Some(adr) => format!("ver ADR-{adr} para propriedades cobertas por set {target}"),
        None      => format!("propriedades de set {target} ainda não são capturadas"),
    };
    (msg, hint)
}

/// Helper para warning de `#set` com target desconhecido (Passo 107).
/// Lista actualizada em Passo 133 para incluir `par` (activado).
fn unsupported_target_warn(target: &str) -> (String, String) {
    (
        format!("set: target '{target}' ainda não suportado"),
        "targets suportados: heading, page, figure, text, par".to_string(),
    )
}

/// Casa um nó contra um selector de element rule (`NodeKind`/`DynKind`).
/// **Partilhado** (P352) pelo loop α (transformação func/content) e pela passagem
/// de show-set: ambos usam exatamente o mesmo critério de match. `Selector::Text`
/// nunca casa aqui (tratado por `map_text`).
fn selector_matches(work: &Content, selector: &Selector) -> bool {
    // F-5b fatia 1 (P371): `show strong/emph` casam as **variantes próprias**
    // `Content::Strong`/`Emph` (S1, por tipo) — o colapso P101 (que casava
    // `Content::Styled` com bold/italic) foi superado. `#set text(bold)`
    // (`Styled[Bold]`) **não** casa `show strong` (fidelidade 0107).
    match selector {
        Selector::NodeKind(kind) => matches!(
            (work, kind),
            (Content::Heading(_),  NodeKind::Heading)
            | (Content::Figure(_),   NodeKind::Figure)
            | (Content::Raw { .. },      NodeKind::Raw)
            | (Content::Equation { .. }, NodeKind::Equation)
            | (Content::ListItem(_),     NodeKind::ListItem)
            | (Content::Strong(_),       NodeKind::Strong)
            | (Content::Emph(_),         NodeKind::Emph)
        ),
        Selector::DynKind(name) =>
            matches!(work, Content::Dynamic(e) if e.dyn_kind() == name),
        Selector::Text(_) => false,
        Selector::Regex(_) => false,
    }
}

/// Aplica as show rules activas ao Content (Passo 70 — DEBT-23 encerrado).
///
/// NodeKind rules: única travessia `map_content` para todas as regras (O(N)).
/// Dentro da closure, itera o snapshot de regras e salta as que estão em
/// `active_guards` (anti-recursão por rule ID — DEBT-20 encerrado).
///
/// Text rules: aplicadas separadamente via `map_text` após a travessia principal.
pub(crate) fn apply_show_rules(
    mut content: Content,
    rules: &[ShowRule],
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if rules.is_empty() {
        return Ok(content);
    }

    // P394: scope vazio para apply_func; closures de show-rule usam captured
    // scope como parent, e nativas comuns não usam scopes.
    let mut scopes = Scopes::new(None);

    // Limite de aninhamento vanilla (MAX_SHOW_RULE_DEPTH = 64) —
    // paridade com `typst-realize/src/lib.rs:402` (ADR-0033).
    // Pago parcial do DEBT-45 no Passo 93.
    route_check_show_depth(engine.route)?;

    // Separar regras por tipo para travessias distintas. Lote F-3 inc-2: as
    // regras de **kind dinâmico** (`#show callout:`) viajam pela MESMA travessia
    // que as NodeKind — mesmo `apply_all`, mesma ordem, mesmo guard por `RuleId`.
    let has_node_rules = rules.iter().any(|r|
        matches!(r.selector, Selector::NodeKind(_) | Selector::DynKind(_)));

    if has_node_rules {
        // Única travessia para todas as NodeKind + DynKind rules.
        let node_rules: Vec<ShowRule> = rules.iter()
            .filter(|r| matches!(r.selector, Selector::NodeKind(_) | Selector::DynKind(_)))
            .cloned()
            .collect();

        let mut apply_all = |node: &Content| -> SourceResult<Option<Content>> {
            // P348 (modelo α, ADR-0107): a element rule cujo output **re-casa** é
            // **revisitada** até **ponto-fixo morfológico** — o loop re-alimenta o
            // output no mesmo conjunto de regras e para quando a regra é um **no-op
            // morfológico** (`morph_canon(out) == morph_canon(work)`, o `==` do P345).
            // Recursão **não-convergente** (ciclo / divergente) é cortada pelo **teto**
            // backstop (`MAX_SHOW_RULE_DEPTH`, mecânica) e vira erro com a mensagem base
            // do vanilla. O `active_guards` continua a impedir a recursão **durante** a
            // chamada do recipe (criação aninhada); a revisitação é este loop, após o
            // recipe devolver. Divergência consciente vs vanilla (P347d/ADR-0107):
            // `#show heading: it => [= Z]` **converge para "Z"** (ponto-fixo) onde o
            // vanilla erra — o vanilla termina por identidade de instância (mecânica,
            // GEROU em P347b/c), o cristalino por morfologia.
            let mut work = node.clone();
            let mut applied = 0usize;
            // P350c — flag de erro completo: sob a flag, manter o histórico de
            // morfologias do caminho para classificar o erro (cíclico vs
            // não-convergente). Com a flag DESLIGADA (default), `history` fica vazio
            // e nada é alocado/computado — caminho quente intacto. `full_error` é Copy
            // (lido uma vez; `ctx` continua livre para `apply_func`).
            let full_error = ctx.full_error;
            let mut history: Vec<Content> =
                if full_error { vec![work.morph_canon()] } else { Vec::new() };
            let mut cycle = false;
            loop {
                // Aplicar a primeira regra func que casa `work`, uma vez, em ordem
                // **innermost-first** (última-declarada primeiro — P358): no
                // subconjunto onde uma func é efetiva, a última-declarada vence,
                // casando o vanilla (`styles.rs:835` `next_back`). É reordenação, não
                // acumulação (o vanilla não acumula func same-kind — P357). O fold de
                // show-set (abaixo) NÃO é invertido (mantém last-declared-overrides).
                let mut produced: Option<Content> = None;
                for rule in node_rules.iter().rev() {
                    // Saltar se esta regra está em execução (anti-recursão na criação
                    // aninhada — Lote F-3 inc-2; guard por `RuleId`, vale p/ DynKind).
                    if engine.active_guards.contains(&rule.id) {
                        continue;
                    }

                    // Show-set (P352): NÃO consome o passe de func — é tratado após o
                    // loop α (embrulha o nó em `Content::Styled`). Saltado aqui.
                    if matches!(rule.transform, Transformation::Style(_)) {
                        continue;
                    }

                    if !selector_matches(&work, &rule.selector) {
                        continue;
                    }

                    match &rule.transform {
                        Transformation::Func(func) => {
                            let args = Args::positional(vec![Value::Content(work.clone())]);
                            engine.active_guards.push(rule.id);
                            let call_result = closures::apply_func(func.clone(), args, &mut scopes, ctx, engine);
                            engine.active_guards.pop();
                            produced = Some(match call_result? {
                                Value::Content(c) => c,
                                Value::Str(s)     => Content::text(s.as_str()),
                                other => return Err(vec![SourceDiagnostic::error(
                                    Span::detached(),
                                    format!(
                                        "show rule deve retornar Content ou String, \
                                         recebeu {}",
                                        other.type_name()
                                    ),
                                )]),
                            });
                            break;
                        },
                        Transformation::Content(c) => { produced = Some(c.clone()); break; },
                        // `Str` só é válida sobre `Selector::Text` (tratada no loop
                        // de texto). Sobre NodeKind/DynKind é erro — paridade com o
                        // comportamento anterior ("recebeu str").
                        Transformation::Str(_) => return Err(vec![SourceDiagnostic::error(
                            Span::detached(),
                            "show rule com selector de tipo requer função ou Content, \
                             recebeu str".to_string(),
                        )]),
                        // Saltado acima; inalcançável.
                        Transformation::Style(_) => continue,
                    }
                }

                let Some(out) = produced else { break };
                applied += 1;

                // P350c (só sob a flag): registrar a morfologia do output e detectar
                // se uma forma do caminho **repetiu** (ciclo — fato medido pelo `==`
                // do P345 sobre `morph_canon`, sem alterá-los). Não corta cedo: a
                // terminação continua no teto (timing idêntico ao flag-off); só o hint
                // muda. Com a flag off, este bloco não corre.
                if full_error {
                    let out_canon = out.morph_canon();
                    if history.iter().any(|h| *h == out_canon) {
                        cycle = true;
                    }
                    history.push(out_canon);
                }

                // A partir da 2ª aplicação, revisitamos um output: detectar o
                // ponto-fixo (no-op morfológico) e cortar runaway. O caminho comum —
                // uma aplicação cujo output **não** re-casa — NÃO paga `morph_canon`:
                // a 2ª iteração apenas falha o match e sai (M-trigger, P348).
                if applied >= 2 {
                    if out.morph_canon() == work.morph_canon() {
                        work = out;
                        break; // ponto-fixo morfológico
                    }
                    if applied >= crate::entities::world_types::Route::MAX_SHOW_RULE_DEPTH {
                        // Teto backstop (mecânica). Mensagem base + 2 hints
                        // BYTE-IDÊNTICOS ao vanilla (`engine.rs:350`, ADR-0033: a
                        // mensagem é comportamento observável).
                        let mut diag = SourceDiagnostic::error(
                            Span::detached(),
                            "maximum show rule depth exceeded",
                        )
                        .with_hint("maybe a show rule matches its own output")
                        .with_hint("maybe there are too deeply nested elements");
                        // P350c: sob a flag, 3º hint com a classificação — DOIS rótulos
                        // sólidos: **cíclico** (uma morfologia do caminho repetiu — fato)
                        // ou **não-convergente** (teto sem repetição). NÃO há terceiro
                        // rótulo ("converge-fundo") — distinguir divergente de
                        // converge-fundo adivinharia o futuro pós-corte (ADR-0108:
                        // afirmar só o medido). Sem a flag, a mensagem é byte-idêntica.
                        if full_error {
                            diag = diag.with_hint(if cycle {
                                "erro completo: recursão CÍCLICA — uma forma de conteúdo \
                                 repetiu-se no caminho de revisitação (a regra de #show \
                                 reescreve para algo que reaparece)"
                            } else {
                                "erro completo: recursão NÃO-CONVERGENTE — passou do limite \
                                 sem repetir nem estabilizar (verifique se a regra termina, \
                                 ou se é recursão legítima profunda)"
                            });
                        }
                        return Err(vec![diag]);
                    }
                }
                work = out;
            }

            // Show-set (P352/P356, S5): embrulha o output no `Styles` das regras
            // **show-set** que casam o **ELEMENTO** — o nó **original** que entrou na
            // realização (`node`), NÃO o `work` pós-func. **NÃO consome o passe** —
            // espelha `map.apply(transform); continue` do vanilla
            // (`typst-realize/src/lib.rs:458-464` / `styles.rs:504`).
            //
            // **Ordem show-set-vs-func (P356, paridade `lib.rs:341,357`).** O vanilla
            // dobra a show-set na chain (`map`) e aplica a func **sob** `chained =
            // styles.chain(&map)`. Casar contra `node` (o elemento) e embrulhar o
            // `work` (o output da func) reproduz isso: a show-set fica ativa quando a
            // func realiza. Casar contra `work` (pós-func) — o bug que isto conserta —
            // perdia a show-set (o output da func não casa o seletor do elemento).
            // Sem func, `work == node` → idêntico ao P352 (show-set-só e múltiplos
            // show-set por `collapse`). O confinamento vem do `Content::Styled`
            // (fatia 1, `f_fronteira_e1.md §3a.8`). `collapse` resolve top-wins.
            let mut set_chain = StyleChain::empty();
            let mut any_set = false;
            for rule in &node_rules {
                if let Transformation::Style(styles) = &rule.transform {
                    if engine.active_guards.contains(&rule.id) {
                        continue;
                    }
                    if selector_matches(node, &rule.selector) {
                        set_chain = set_chain.push(styles.delta().clone());
                        any_set = true;
                    }
                }
            }
            let mut wrapped = false;
            if any_set {
                let delta = set_chain.collapse();
                if !delta.is_empty() {
                    work = Content::Styled(Box::new(work), Styles::from_delta(delta));
                    wrapped = true;
                }
            }

            if applied > 0 || wrapped { Ok(Some(work)) } else { Ok(None) }
        };

        content = content.map_content(&mut apply_all)?;
    }

    // Regex rules (P393) — aplicadas a nós de texto que casam.
    // Última-declarada vence (consistente com NodeKind). Transformações
    // Func/Content/Str são suportadas; Style foi rejeitado em parse.
    let regex_rules: Vec<ShowRule> = rules.iter()
        .filter(|r| matches!(r.selector, Selector::Regex(_)))
        .cloned()
        .collect();

    if !regex_rules.is_empty() {
        let mut apply_regex = |node: &Content| -> SourceResult<Option<Content>> {
            let Content::Text(text) = node else { return Ok(None); };
            for rule in regex_rules.iter().rev() {
                if engine.active_guards.contains(&rule.id) {
                    continue;
                }
                let Selector::Regex(re) = &rule.selector else { continue };
                if !re.is_match(text.as_str()) {
                    continue;
                }
                let produced = match &rule.transform {
                    Transformation::Func(func) => {
                        let args = Args::positional(vec![Value::Content(node.clone())]);
                        engine.active_guards.push(rule.id);
                        let call_result = closures::apply_func(func.clone(), args, &mut scopes, ctx, engine);
                        engine.active_guards.pop();
                        match call_result? {
                            Value::Content(c) => c,
                            Value::Str(s)     => Content::text(s.as_str()),
                            other => return Err(vec![SourceDiagnostic::error(
                                Span::detached(),
                                format!(
                                    "show rule regex deve retornar Content ou String, \
                                     recebeu {}",
                                    other.type_name()
                                ),
                            )]),
                        }
                    },
                    Transformation::Content(c) => c.clone(),
                    Transformation::Str(s)     => Content::text(s.as_str()),
                    Transformation::Style(_)   => continue,
                };
                return Ok(Some(produced));
            }
            Ok(None)
        };
        content = content.map_content(&mut apply_regex)?;
    }

    // Text rules — map_text por padrão, na ordem de declaração.
    for rule in rules {
        if let Selector::Text(pattern) = &rule.selector {
            if let Transformation::Str(s) = &rule.transform {
                let replacement = s.to_string();
                let mut do_replace = |text: &str| text.replace(pattern.as_str(), &replacement);
                content = content.map_text(&mut do_replace);
            }
        }
    }

    Ok(content)
}

/// Aplica show rules ao Content produzido por eval (Passo 70 — DEBT-20 encerrado).
///
/// Anti-recursão via `active_guards` (stack de RuleId) em vez de booleano global.
/// Permite composição entre regras distintas; snapshot explícito evita borrow
/// conflict durante a travessia (DEBT-22).
pub(crate) fn intercept_content(
    content: Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if engine.show_rules.is_empty() {
        return Ok(content);
    }

    // Passo 84.4 (encerra DEBT-22): snapshot Arc::clone — O(1) refcount.
    // O slice partilhado permite iterar sobre uma cópia estável enquanto
    // `engine.show_rules` pode ser reatribuído por nested `#show` rules
    // sem interferência durante a travessia.
    let rules = Arc::clone(&*engine.show_rules);
    apply_show_rules(content, &rules, ctx, engine)
}

// ── Dispatcher arms: SetRule / ShowRule (Passo 96.2, ADR-0037 Regra 4) ────

pub(super) fn eval_set_rule(
    set: SetRule<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    // Extrair target — deve ser um Ident (ex: "text").
    // Targets suportados: heading, page, figure, text. Outros emitem
    // warning via Sink (Passo 107, encerra DEBT-49).
    let target = set.target().to_untyped().text_str().to_owned();
    let target_span = set.target().to_untyped().span();

    if target == "heading" {
        // #set heading(numbering: "1.1") — activa numeração automática.
        // Outros argumentos de heading ignorados por agora (DEBT-10).
        let active = set.args().items().any(|arg| {
            if let Arg::Named(named) = arg {
                if named.name().as_str() == "numbering" {
                    // Defensivo: só String activa a numeração.
                    // Closures, none, ou outros tipos → ignorar.
                    let val = eval_expr(named.expr(), scopes, ctx, engine).unwrap_or(Value::None);
                    return matches!(val, Value::Str(_));
                }
            }
            false
        });
        // Lote F-2 S1 (P335): em vez de um marcador global `SetHeadingNumbering`,
        // empurra para a chain léxica (`engine.styles` é escopado por
        // `local_styles`). O heading assa este valor na criação
        // (`eval/markup.rs`). Fecha o canal global → escopo de container (DEBT 99.E).
        *engine.styles = engine.styles.push_custom("heading.numbering", Value::Bool(active));
        return Ok(Value::None);
    }

    // Lote F-2 S2 / **B1** (P335): `#set math.equation(numbering:)` — target
    // **pontuado** (`FieldAccess` `math.equation`), que `text_str()` não capta
    // (nó interno → ""). Era a lacuna do P331 (sem produtor eval). Empurra para
    // a chain léxica; a equação assa o valor (`eval/mod.rs`). Paridade vanilla:
    // `lab/.../math/equation.rs` — numeração de equação de bloco.
    if let Expr::FieldAccess(fa) = set.target() {
        if fa.target().to_untyped().text_str() == "math"
            && fa.field().to_untyped().text_str() == "equation"
        {
            let active = set.args().items().any(|arg| {
                if let Arg::Named(named) = arg {
                    if named.name().as_str() == "numbering" {
                        let val = eval_expr(named.expr(), scopes, ctx, engine)
                            .unwrap_or(Value::None);
                        return matches!(val, Value::Str(_));
                    }
                }
                false
            });
            *engine.styles =
                engine.styles.push_custom("equation.numbering", Value::Bool(active));
            return Ok(Value::None);
        }
    }

    if target == "page" {
        // #set page(width: .., height: .., margin: ..) — Passo 81.
        // Valores ausentes ficam None e preservam o valor actual em layout.
        fn extract_pt(val: &Value) -> Option<f64> {
            match val {
                Value::Length(l) => Some(l.abs.to_pt()),
                Value::Float(f)  => Some(*f),
                Value::Int(i)    => Some(*i as f64),
                _                => None,
            }
        }
        let mut width  = None;
        let mut height = None;
        let mut margin = None;
        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                let key = named.name().as_str();
                let val = eval_expr(named.expr(), scopes, ctx, engine).unwrap_or(Value::None);
                match key {
                    "width"  => width  = extract_pt(&val),
                    "height" => height = extract_pt(&val),
                    "margin" => margin = extract_pt(&val),
                    _        => {}
                }
            }
        }
        return Ok(Value::Content(Content::SetPage { width, height, margin }));
    }

    if target == "figure" {
        // #set figure(numbering: "1") — activa numeração automática de figuras.
        // Lote F-2 S3 (P335): empurra para a chain léxica (`custom`), como
        // heading/equation, em vez de mutar o campo global `engine.figure_numbering`.
        // `native_figure` assa o valor lendo a chain (`closures.rs`). Escopo
        // léxico de graça (fecha o DEBT 99.E para figure).
        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                if named.name().as_str() == "numbering" {
                    let val = eval_expr(named.expr(), scopes, ctx, engine).unwrap_or(Value::None);
                    match val {
                        Value::Str(s) => {
                            *engine.styles = engine.styles
                                .push_custom("figure.numbering", Value::Str(s));
                        }
                        Value::None => {
                            // Limpa a numeração no escopo (Value::None = ausente).
                            *engine.styles = engine.styles
                                .push_custom("figure.numbering", Value::None);
                        }
                        // Outros tipos: herdar (não empurra).
                        _ => {}
                    }
                }
            }
        }
        return Ok(Value::None);
    }

    if target == "par" {
        // Passo 133: target `par` activado sem arms concretos.
        // Passo 134: arm `leading` migrado de `text` para `par`
        // (paridade ADR-0033; resolve divergência temporal do 128).
        // Outras propriedades (justify, first-line-indent, etc.)
        // continuam no fallback até serem activadas on-demand.
        // F-5b fatia 2 (P373): `leading` migra de `StyleDelta` tipado para o canal
        // `custom` `"par.leading"` (morph-transparente, transportado). O layout
        // (merge arm) decodifica de volta.
        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                let key = named.name().as_str().to_owned();
                let val = eval_expr(named.expr(), scopes, ctx, engine)?;
                match key.as_str() {
                    "leading" => {
                        if let Value::Length(l) = val {
                            *engine.styles = engine.styles.push_custom("par.leading", Value::Length(l));
                        }
                    }
                    _ => {
                        let (msg, hint) = unsupported_property_warn(
                            "par", &key, None,
                        );
                        engine.sink.warn_note(
                            named.name().to_untyped().span(),
                            &msg,
                            &hint,
                        );
                    }
                }
            }
        }
        return Ok(Value::None);
    }

    // F-item3 (P368, `f_fronteira_e1.md` §3a.11): `#set <elemento-de-usuário>(prop:)`
    // pelo mapa aberto. Se o `target` resolve em `scopes` a um `Func::element`
    // (elemento de usuário registrado), em vez do warn, empurra `("<kind>.<prop>",
    // Value)` no canal `custom` da chain — transparente à morfologia
    // (`is_semantically_empty` ignora o custom, P366). O elemento lê pela chain no
    // layout (precedência: construído explícito > chain > default). Aditivo: os
    // `#set` nativos (acima) não mudam.
    let is_user_elem = matches!(
        scopes.get(&target),
        Some(Value::Func(f)) if f.element_name().is_some()
    );
    if is_user_elem {
        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                let key = format!("{}.{}", target, named.name().as_str());
                let val = eval_expr(named.expr(), scopes, ctx, engine)
                    .unwrap_or(Value::None);
                *engine.styles = engine.styles.push_custom(key, val);
            }
        }
        return Ok(Value::None);
    }

    if target != "text" {
        let (msg, hint) = unsupported_target_warn(&target);
        engine.sink.warn_note(target_span, &msg, &hint);
        return Ok(Value::None);
    }

    // **F-5b fatia 2 (P373, §3a.13/§3a.14)**: o render do `#set text` deixa de assar
    // num `StyleDelta` tipado capturado no node — passa a viajar na chain pelo canal
    // `custom` `"text.<campo>"` (Value canónico), **transparente à morfologia**
    // (`is_semantically_empty` ignora o custom → `#set text X == X`) e levado ao
    // layout pelo transporte aninhado (`eval_markup`). A validação/erro-hard de
    // `lang`/`font` permanece (paridade ADR-0033/0052/0053); só a **saída** muda
    // (typed → custom). O layout (merge arm) decodifica o Value de volta.
    for arg in set.args().items() {
        if let Arg::Named(named) = arg {
            let key = named.name().as_str().to_owned();
            let val = eval_expr(named.expr(), scopes, ctx, engine)?;
            match key.as_str() {
                "bold" => {
                    if let Value::Bool(b) = val {
                        *engine.styles = engine.styles.push_custom("text.bold", Value::Bool(b));
                    }
                }
                "italic" => {
                    if let Value::Bool(b) = val {
                        *engine.styles = engine.styles.push_custom("text.italic", Value::Bool(b));
                    }
                }
                "size" => {
                    if let Value::Length(l) = val {
                        *engine.styles = engine.styles.push_custom("text.size", Value::Length(l));
                    }
                }
                "fill" => {
                    if let Value::Color(c) = val {
                        *engine.styles = engine.styles.push_custom("text.fill", Value::Color(c));
                    }
                }
                "weight" => {
                    // Int direto ou nome simbólico (FontWeight::from_name) → u16
                    // canónico em Value::Int. Tipo/nome inválido: silent skip
                    // (padrão histórico).
                    let w: Option<u16> = match &val {
                        Value::Int(n) => u16::try_from(*n).ok(),
                        Value::Str(s) => FontWeight::from_name(s.as_str()).map(|fw| fw.to_number()),
                        _ => None,
                    };
                    if let Some(w) = w {
                        *engine.styles = engine.styles.push_custom("text.weight", Value::Int(w as i64));
                    }
                }
                "tracking" => {
                    if let Value::Length(l) = val {
                        *engine.styles = engine.styles.push_custom("text.tracking", Value::Length(l));
                    }
                }
                "lang" => {
                    // ADR-0052: valida via `Lang::from_str` (erro hard em
                    // inválido); guarda o código canónico (`Value::Str`).
                    if let Value::Str(s) = val {
                        match Lang::from_str(&s) {
                            Ok(lang) => {
                                *engine.styles = engine.styles
                                    .push_custom("text.lang", Value::Str(lang.as_str().into()));
                            }
                            Err(msg) => {
                                return Err(vec![SourceDiagnostic::error(
                                    named.expr().span(),
                                    msg.to_string(),
                                )]);
                            }
                        }
                    }
                }
                "font" => {
                    // P407 (DEBT-52): string/array/dict aceites. Inspeccionamos o
                    // AST do argumento porque `Value::Dict` tem keys `EcoString`, não
                    // suportando regex keys (que o vanilla exige para `text.font`).
                    // Persiste na chain custom como `Value::Array` de items:
                    //   - `Value::Str(name)` → literal, variants vazio (P292/P373).
                    //   - `Value::Dict` com "name" (Str|Regex) + "variants" (Array[Str])
                    //     → dict form com regex keys.
                    // Zero tipo novo em `Value`: reusa Array/Dict/Str/Regex.
                    let span = named.expr().span();
                    let font_expr = named.expr();
                    let arr: Vec<Value> = match font_expr {
                        Expr::Str(node) => vec![Value::Str(EcoString::from(node.get()))],
                        Expr::Array(arr_node) => {
                            let mut items = Vec::new();
                            for item in arr_node.items() {
                                if let ArrayItem::Pos(expr) = item {
                                    if let Expr::Str(node) = expr {
                                        items.push(Value::Str(EcoString::from(node.get())));
                                    } else {
                                        return Err(vec![SourceDiagnostic::error(
                                            span,
                                            "font array must contain only strings".to_string(),
                                        )]);
                                    }
                                }
                            }
                            if items.is_empty() {
                                return Err(vec![SourceDiagnostic::error(
                                    span,
                                    "font array must not be empty".to_string(),
                                )]);
                            }
                            items
                        }
                        Expr::Dict(dict_node) => {
                            // P414: detectar dict named fields (tem chave `family`)
                            // vs formato legado P407 (chaves são nomes de fonte).
                            const NAMED_FIELDS: &[&str] = &["family", "variant", "weight", "style", "fallback"];
                            let is_named_fields = dict_node.items().any(|item| {
                                matches!(
                                    item,
                                    DictItem::Named(named_item)
                                        if NAMED_FIELDS.contains(&named_item.name().as_str())
                                )
                            });

                            if is_named_fields {
                                parse_font_dict_named_fields(
                                    dict_node, span, scopes, ctx, engine,
                                )?
                            } else {
                                parse_font_dict_legacy(
                                    dict_node, span, scopes, ctx, engine,
                                )?
                            }
                        }
                        _ => {
                            return Err(vec![SourceDiagnostic::error(
                                span,
                                "font expects a string, array of strings, or dict".to_string(),
                            )]);
                        }
                    };
                    *engine.styles = engine.styles.push_custom("text.font", Value::Array(arr));
                }
                _ => {
                    // Passo 107 (encerra DEBT-49): propriedades não suportadas
                    // de `#set text(...)` emitem warning via Sink.
                    let (msg, hint) = unsupported_property_warn("text", &key, Some("0040"));
                    engine.sink.warn_note(named.name().to_untyped().span(), &msg, &hint);
                }
            }
        }
    }

    Ok(Value::None)
}

/// **Show-set (P352)** — captura o efeito de um `#set` como `Styles`, **sem**
/// mutar o estilo global. Reusa `eval_set_rule` (fiel a todos os targets, aos
/// warns e aos erros hard) aplicando-o a uma `StyleChain::empty()` temporária e
/// extrai o delta resultante (`collapse`). Sobre `empty()`, o resultado é só o
/// que o `set` definiu — sem os defaults. `engine.styles` é restaurado mesmo
/// em caso de erro (o swap de volta corre antes do `?`).
fn capture_set_styles(
    set: SetRule<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Styles> {
    let mut scratch = StyleChain::empty();
    std::mem::swap(&mut *engine.styles, &mut scratch);
    let result = eval_set_rule(set, scopes, ctx, engine);
    std::mem::swap(&mut *engine.styles, &mut scratch);
    result?;
    Ok(Styles::from_delta(scratch.collapse()))
}

pub(super) fn eval_show_rule(
    show_rule: ShowRuleNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    // Avaliar o selector — pode ser uma string ou uma função da stdlib.
    // `selector()` retorna `Option<Expr>` — None significa selector omitido (não suportado).
    let selector = match show_rule.selector() {
        None => return Err(vec![SourceDiagnostic::error(
            show_rule.to_untyped().span(),
            "show rule requer um selector".to_string(),
        )]),
        Some(sel_expr) => {
            let selector_val = eval_expr(sel_expr, scopes, ctx, engine)?;
            match selector_val {
                Value::Str(s) => Selector::Text(s.to_string()),
                // P393: regex(pattern) → selector regex sobre texto.
                Value::Regex(re) => Selector::Regex(re),
                // Lote F-3 inc-2: elemento de utilizador (fronteira E1) — o
                // selector `callout` resolve para uma `FuncRepr::Element`, que
                // não tem fn-ptr nativo. Casa por **kind dinâmico** (nome).
                Value::Func(ref f) if f.element_name().is_some() => {
                    Selector::DynKind(f.element_name().unwrap().to_string())
                },
                Value::Func(ref f) => {
                    // Passo 84.3 (encerra DEBT-21): resolver NodeKind
                    // por identidade do function pointer da nativa
                    // subjacente, não pelo nome textual. Aliasing via
                    // `#let alias = heading` (clone do mesmo Arc<Func>)
                    // ou re-registo da mesma fn com nome diferente
                    // continuam a apontar para o mesmo `fn` — match.
                    //
                    // Closures retornam `None` em `native_fn_addr()` —
                    // function pointers de closures não são estáveis.
                    use std::ptr::fn_addr_eq;
                    use crate::rules::stdlib::{
                        native_heading, native_figure, native_strong,
                        native_emph, native_raw,
                    };
                    match f.native_fn_addr() {
                        Some(addr) if fn_addr_eq(addr, native_heading as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Heading),
                        Some(addr) if fn_addr_eq(addr, native_figure as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Figure),
                        Some(addr) if fn_addr_eq(addr, native_strong as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Strong),
                        Some(addr) if fn_addr_eq(addr, native_emph as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Emph),
                        Some(addr) if fn_addr_eq(addr, native_raw as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Raw),
                        Some(_) => return Err(vec![SourceDiagnostic::error(
                            sel_expr.span(),
                            format!(
                                "função '{}' não é um tipo de nó suportado como selector. \
                                 Tipos suportados: heading, figure, strong, emph, raw.",
                                f.name().unwrap_or("<anónima>")
                            ),
                        )]),
                        None => return Err(vec![SourceDiagnostic::error(
                            sel_expr.span(),
                            "o selector de show rule deve ser uma função nativa \
                             ou uma string literal. Closures não são suportadas."
                                .to_string(),
                        )]),
                    }
                },
                other => return Err(vec![SourceDiagnostic::error(
                    sel_expr.span(),
                    format!("selector inválido para show rule: {}", other.type_name()),
                )]),
            }
        }
    };

    // Classificar a transformação. Show-set (`#show k: set …`) é detetado pelo
    // tipo do nó (`Expr::SetRule`) e **capturado sem mutar `engine.styles`**
    // globalmente (P352); as restantes formas avaliam para um `Value` e
    // classificam-se em `Transformation`.
    let transform_expr = show_rule.transform();
    let transform = match transform_expr {
        Expr::SetRule(set) => {
            // Show-set é sobre um elemento (NodeKind/DynKind), não sobre texto
            // nem regex (P393).
            if matches!(selector, Selector::Text(_) | Selector::Regex(_)) {
                return Err(vec![SourceDiagnostic::error(
                    set.to_untyped().span(),
                    "show-set (`#show …: set …`) não é válido para um selector de \
                     texto — use uma função ou Content".to_string(),
                )]);
            }
            Transformation::Style(capture_set_styles(set, scopes, ctx, engine)?)
        }
        expr => {
            let value = eval_expr(expr, scopes, ctx, engine)?;
            match value {
                Value::Func(f)    => Transformation::Func(f),
                Value::Content(c) => Transformation::Content(c),
                Value::Str(s)     => Transformation::Str(s),
                other => return Err(vec![SourceDiagnostic::error(
                    expr.span(),
                    format!(
                        "transformação de show rule inválida: esperado função, \
                         Content, string ou set rule, recebeu {}",
                        other.type_name()
                    ),
                )]),
            }
        }
    };
    let id = ctx.next_rule_id;
    ctx.next_rule_id += 1;
    // Reconstruir o slice com a nova regra (Passo 95 + 109: show_rules
    // é campo do Engine, mutação local via &mut).
    let mut rules = engine.show_rules.to_vec();
    rules.push(ShowRule { id, selector, transform });
    *engine.show_rules = Arc::from(rules);
    Ok(Value::None)
}

/// P414: parsing do dict `text.font` no formato named fields vanilla.
///
/// Campos suportados: `family` (Str|Regex), `variant` (Str),
/// `weight` (Int|Str), `style` (Str), `fallback` (Bool, default true).
/// `stretch` e outros campos são rejeitados.
fn parse_font_dict_named_fields<'a>(
    dict_node: Dict<'a>,
    span: Span,
    scopes: &mut Scopes,
    ctx: &mut EvalContext,
    engine: &mut Engine,
) -> SourceResult<Vec<Value>> {
    use crate::entities::value::Value;

    let mut family: Option<Value> = None;
    let mut variant: Option<EcoString> = None;
    let mut weight: Option<EcoString> = None;
    let mut style: Option<EcoString> = None;
    let mut fallback = true;

    for item in dict_node.items() {
        let named = match item {
            DictItem::Named(n) => n,
            _ => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "font dict named fields must use identifiers as keys".to_string(),
                )]);
            }
        };

        let field = named.name().as_str();
        let value = eval_expr(named.expr(), scopes, ctx, engine)?;

        match field {
            "family" => {
                if !matches!(value, Value::Str(_) | Value::Regex(_)) {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        format!(
                            "font dict field 'family' expects string or regex, recebeu {}",
                            value.type_name()
                        ),
                    )]);
                }
                family = Some(value);
            }
            "variant" => {
                match value {
                    Value::Str(s) => variant = Some(s),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            format!(
                                "font dict field 'variant' expects string, recebeu {}",
                                other.type_name()
                            ),
                        )]);
                    }
                }
            }
            "weight" => {
                let s = match value {
                    Value::Int(n) => EcoString::from(n.to_string()),
                    Value::Str(s) => s,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            format!(
                                "font dict field 'weight' expects integer or string, recebeu {}",
                                other.type_name()
                            ),
                        )]);
                    }
                };
                weight = Some(s);
            }
            "style" => {
                match value {
                    Value::Str(s) => style = Some(s),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            format!(
                                "font dict field 'style' expects string, recebeu {}",
                                other.type_name()
                            ),
                        )]);
                    }
                }
            }
            "fallback" => {
                match value {
                    Value::Bool(b) => fallback = b,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            format!(
                                "font dict field 'fallback' expects boolean, recebeu {}",
                                other.type_name()
                            ),
                        )]);
                    }
                }
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!("unknown font dict field: {}", other),
                )]);
            }
        }
    }

    let family = family.ok_or_else(|| {
        vec![SourceDiagnostic::error(
            span,
            "font dict missing field 'family'".to_string(),
        )]
    })?;

    if !fallback {
        // `fallback: false` semanticamente força fonte única. Como a forma
        // named fields gera um único item, a verificação é documental; o
        // layout respeita a lista resultante.
    }

    let mut entry: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    entry.insert(EcoString::from("name"), family);
    entry.insert(EcoString::from("variants"), Value::Array(vec![]));
    if let Some(v) = variant {
        entry.insert(EcoString::from("variant"), Value::Str(v));
    }
    if let Some(w) = weight {
        entry.insert(EcoString::from("weight"), Value::Str(w));
    }
    if let Some(s) = style {
        entry.insert(EcoString::from("style"), Value::Str(s));
    }

    Ok(vec![Value::Dict(entry)])
}

/// P407: parsing do dict `text.font` no formato legado cristalino.
///
/// Chaves são nomes de família (string literal, identificador ou regex);
/// valores são variant names (string ou array de strings).
fn parse_font_dict_legacy<'a>(
    dict_node: Dict<'a>,
    span: Span,
    scopes: &mut Scopes,
    ctx: &mut EvalContext,
    engine: &mut Engine,
) -> SourceResult<Vec<Value>> {
    let mut items = Vec::new();
    for dict_item in dict_node.items() {
        let (name_val, variants_val) = match dict_item {
            DictItem::Keyed(keyed) => {
                let key_expr = keyed.key();
                let name_val = match key_expr {
                    Expr::Str(node) => Value::Str(EcoString::from(node.get())),
                    Expr::FuncCall(call) => {
                        // regex("...") avalia para Value::Regex.
                        match eval_expr(Expr::FuncCall(call), scopes, ctx, engine)? {
                            Value::Regex(re) => Value::Regex(re),
                            other => {
                                return Err(vec![SourceDiagnostic::error(
                                    span,
                                    format!("font dict key must be string or regex, recebeu {}", other.type_name()),
                                )]);
                            }
                        }
                    }
                    _other => {
                        return Err(vec![SourceDiagnostic::error(
                            span,
                            "font dict key must be string or regex".to_string(),
                        )]);
                    }
                };
                let value = eval_expr(keyed.expr(), scopes, ctx, engine)?;
                let variants = variants_from_value(value, span)?;
                (name_val, variants)
            }
            DictItem::Named(named_item) => {
                // Identificador como key: "Name": value é syntax sugar
                // para string literal lowercased.
                let name = named_item.name().as_str();
                let name_val = Value::Str(EcoString::from(name));
                let value = eval_expr(named_item.expr(), scopes, ctx, engine)?;
                let variants = variants_from_value(value, span)?;
                (name_val, variants)
            }
            DictItem::Spread(_) => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "font dict spread not supported".to_string(),
                )]);
            }
        };
        let mut entry: IndexMap<EcoString, Value, FxBuildHasher> =
            IndexMap::default();
        entry.insert(EcoString::from("name"), name_val);
        entry.insert(EcoString::from("variants"), Value::Array(variants_val));
        items.push(Value::Dict(entry));
    }
    if items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            span,
            "font dict must not be empty".to_string(),
        )]);
    }
    Ok(items)
}

/// Helper partilhado entre os dois parsers de dict de fonte.
fn variants_from_value(value: Value, span: Span) -> SourceResult<Vec<Value>> {
    match value {
        Value::Str(s) => Ok(vec![Value::Str(s)]),
        Value::Array(arr) => {
            let mut vs = Vec::with_capacity(arr.len());
            for v in arr.iter() {
                if let Value::Str(s) = v {
                    vs.push(Value::Str(s.clone()));
                } else {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        "font variants must be strings".to_string(),
                    )]);
                }
            }
            Ok(vs)
        }
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!("font dict value must be string or array of strings, recebeu {}", other.type_name()),
        )]),
    }
}
