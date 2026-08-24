//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval.md
//! @prompt-hash 8c6f71d7
//! @layer L1
//! @updated 2026-07-22
//!
//! Show rules e set rules — aplicação e intercepção. Extraído de `eval.rs`
//! no Passo 96.1 conforme ADR-0037 (coesão por domínio).

use std::sync::Arc;

use std::str::FromStr;

use crate::compiler::scopes::Scopes;
use crate::entities::args::Args;
use crate::entities::ast::code::{SetRule, ShowRule as ShowRuleNode};
use crate::entities::ast::expr::{Arg, ArrayItem, DictItem, Expr};
use crate::entities::ast::AstNode;
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
use ecow::EcoString;

use super::{
    call_dispatch, eval_expr, font_dict, selector_matching, show_rule_termination,
    EvalContext,
};

/// P636 — mensagem de mismatch de tipo no formato do vanilla
/// (`foundations/cast.rs:325-335`): "expected {expected}, found {actual}".
pub(crate) fn type_mismatch(
    expected: &str,
    found: &Value,
    span: Span,
) -> SourceDiagnostic {
    SourceDiagnostic::error(
        span,
        format!("expected {}, found {}", expected, found.type_name()),
    )
}

/// **P816** (achado #3c de P810) — erro de cast para `Length` no formato
/// do vanilla (`foundations/cast.rs:325-343`): `expected length, found
/// {type}` com hint `a length needs a unit - did you mean {i}pt?` quando o
/// valor recebido é `Int` (hint medido no vanilla, `text(mod.rs)` via
/// `cast.rs:341-343`).
pub(crate) fn expected_length_error(found: &Value, span: Span) -> SourceDiagnostic {
    // Nome do tipo no vocabulário do vanilla (`foundations/cast.rs`,
    // `Value::ty`): o `type_name()` cristalino diverge em `int`/`str`/`bool`
    // (P636); o vanilla diz `integer`/`string`/`boolean` — medido em
    // `#set text(size: 12)` → "expected length, found integer".
    let found_name = match found.type_name() {
        "int" => "integer",
        "str" => "string",
        "bool" => "boolean",
        other => other,
    };
    let mut diag =
        SourceDiagnostic::error(span, format!("expected length, found {found_name}"));
    if let Value::Int(i) = found {
        diag = diag.with_hint(format!("a length needs a unit - did you mean {i}pt?"));
    }
    diag
}

/// **P837** (achado #22 de P831) — erro de cast para `top-edge`/
/// `bottom-edge` no formato verbatim do vanilla 0.15.0 (medido:
/// `error: expected "ascender", "cap-height", "x-height", "baseline",
/// "bounds", or length` para top; `expected "baseline", "descender",
/// "bounds", or length` para bottom; exit 1). String fora do domínio
/// enumerado falha o cast de `TopEdgeMetric`/`BottomEdgeMetric` sem
/// sufixo `found`; outros tipos levam `, found {type}` e, para `Int`,
/// o hint `a length needs a unit - did you mean {i}pt?` (mesmo hint de
/// `expected_length_error`, P816, `foundations/cast.rs:341-343`).
pub(crate) fn edge_cast_error(
    is_top: bool,
    found: &Value,
    span: Span,
) -> SourceDiagnostic {
    let expected = if is_top {
        "\"ascender\", \"cap-height\", \"x-height\", \"baseline\", \"bounds\", or length"
    } else {
        "\"baseline\", \"descender\", \"bounds\", or length"
    };
    if let Value::Str(_) = found {
        return SourceDiagnostic::error(span, format!("expected {expected}"));
    }
    // Nome do tipo no vocabulário do vanilla (mesmo mapeamento de
    // `expected_length_error`, P816).
    let found_name = match found.type_name() {
        "int" => "integer",
        "str" => "string",
        "bool" => "boolean",
        other => other,
    };
    let mut diag =
        SourceDiagnostic::error(span, format!("expected {expected}, found {found_name}"));
    if let Value::Int(i) = found {
        diag = diag.with_hint(format!("a length needs a unit - did you mean {i}pt?"));
    }
    diag
}

/// **P837** — domínios enumerados válidos de `top-edge`/`bottom-edge`
/// (cast de `TopEdgeMetric`/`BottomEdgeMetric`, vanilla
/// `text/mod.rs:1180-1248`). `"bounds"` é válido em ambos (resolvido no
/// vanilla via bbox do glyph; no cristalino cai no fallback defensivo de
/// `edge_offset_pt` — limitação conhecida, ver relatório P837).
const TOP_EDGE_METRICS: &[&str] =
    &["ascender", "cap-height", "x-height", "baseline", "bounds"];
const BOTTOM_EDGE_METRICS: &[&str] = &["baseline", "descender", "bounds"];

/// **P816** (achado #3a de P810) — propriedades nomeadas que o `TextElem`
/// do vanilla aceita em `#set text(...)` (campos settable de
/// `text/mod.rs:182-788`, vanilla 0.15.0, em kebab-case). Nomes fora
/// desta lista são **erro hard** `unexpected argument: {name}` (paridade
/// `foundations/args.rs:262`, exit 1); nomes dentro da lista mas ainda
/// não capturados pelo cristalino mantêm o warning de scope-out do
/// Passo 107 (ADR-0040).
pub(crate) const VANILLA_TEXT_SET_PROPS: &[&str] = &[
    "font",
    "fallback",
    "style",
    "weight",
    "stretch",
    "size",
    "fill",
    "stroke",
    "tracking",
    "spacing",
    "cjk-latin-spacing",
    "baseline",
    "overhang",
    "top-edge",
    "bottom-edge",
    "lang",
    "region",
    "script",
    "dir",
    "hyphenate",
    "costs",
    "kerning",
    "alternates",
    "stylistic-set",
    "ligatures",
    "discretionary-ligatures",
    "historical-ligatures",
    "number-type",
    "number-width",
    "slashed-zero",
    "fractions",
    "features",
    // P836 — `#[fold] #[ghost]` settable no vanilla (`text/mod.rs:846-850`).
    "variations",
];

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
        None => format!("propriedades de set {target} ainda não são capturadas"),
    };
    (msg, hint)
}

/// Helper para warning de `#set` com target desconhecido (Passo 107).
/// Lista actualizada em Passo 133 para incluir `par` (activado).
fn unsupported_target_warn(target: &str) -> (String, String) {
    (
        format!("set: target '{target}' ainda não suportado"),
        "targets suportados: heading, page, figure, text, par, table, smartquote"
            .to_string(),
    )
}

/// **P1030** — parâmetros que **a linguagem** define para cada elemento math,
/// derivados dos campos não-`#[required]` dos elementos math do vanilla
/// (`lab/.../typst-library/src/math/`). Ver `eval.md` §P1030.
///
/// Um parâmetro **fora** desta tabela é erro (`unexpected argument`, a mesma
/// mensagem do vanilla); **dentro** dela, ou é levado à construção
/// (`MATH_SET_LIGADOS`) ou avisa nomeando elemento e parâmetro.
fn math_elem_params(elem: &str) -> Option<&'static [&'static str]> {
    Some(match elem {
        "equation" => &["block", "numbering", "number-align", "supplement", "alt"],
        "mat" => &["delim", "align", "augment", "gap", "row-gap", "column-gap"],
        "vec" => &["delim", "align", "gap"],
        "cases" => &["delim", "reverse", "gap"],
        "cancel" => &["length", "inverted", "cross", "angle", "stroke", "background"],
        "accent" => &["size", "dotless"],
        "frac" => &["style"],
        "lr" => &["size"],
        "stretch" => &["size"],
        "op" => &["limits"],
        "attach" => &["t", "b", "tl", "bl", "tr", "br"],
        "limits" => &["inline"],
        // Elementos math **sem** parâmetro configurável: os seus campos são
        // `#[required]`, `#[positional]` ou variádicos. Estão aqui de
        // propósito — é o que faz `#set math.binom(lower:)` dar
        // `unexpected argument: lower`, como o vanilla (medido P1030).
        "binom" | "root" | "mid" | "class" | "scripts" | "primes" | "underline"
        | "overline" | "underbrace" | "overbrace" | "underbracket" | "overbracket"
        | "underparen" | "overparen" | "undershell" | "overshell" => &[],
        _ => return None,
    })
}

/// **P1030 fatia 1** — os pares que chegam ao construtor. Medição da Fase A:
/// são exactamente aqueles cujo argumento **explícito** já se aplica no
/// cristalino; os restantes precisam da feature antes do `#set` (grupos B/C/D
/// de `eval.md` §P1030).
const MATH_SET_LIGADOS: &[(&str, &str)] = &[
    ("mat", "delim"),
    ("vec", "delim"),
    ("op", "limits"),
    ("cases", "delim"),
    ("cases", "reverse"),
    ("cases", "gap"),
];

/// **P1030** — `#set math.<elemento>(...)`, alvo pontuado.
///
/// `eval_set_rule` extrai o alvo por `text_str()`, que devolve `""` para um
/// `Expr::FieldAccess`; sem este caminho, todo o `#set math.*` caía no
/// fallback com alvo vazio e era ignorado em silêncio (só o aviso
/// `set: target '' ainda não suportado`). O braço de `math.equation` +
/// `numbering` continua **antes** deste e intacto.
fn eval_set_math_rule(
    elem: &str,
    set: SetRule<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    target_span: Span,
) -> SourceResult<Value> {
    let Some(params) = math_elem_params(elem) else {
        // Nome de `math` que não é elemento com parâmetros. O vanilla
        // distingue três casos, medidos em P1030 Fase C: membro inexistente
        // (`module \`math\` does not contain \`foobar\``), função não-elemento
        // (`only element functions can be used in set rules`, ex. `math.abs`) e
        // símbolo (`symbol π is not callable`). Distingui-los exige resolver o
        // nome no scope do módulo — **fora da fatia 1**. Até lá avisa nomeando
        // o alvo, em vez de errar com uma mensagem que poderia ser falsa.
        engine.sink.warn_note(
            target_span,
            &format!(
                "#set math.{elem}(...) não é reconhecido como elemento configurável"
            ),
            "no vanilla isto é erro; a paridade da mensagem está por fazer",
        );
        return Ok(Value::None);
    };

    for arg in set.args().items() {
        let Arg::Named(named) = arg else { continue };
        let nome = named.name().as_str();

        if !params.contains(&nome) {
            // Paridade com o vanilla, que rejeita o argumento em vez de o
            // aceitar e ignorar (medido P1030 Fase A: `#set math.binom(lower:)`
            // → `error: unexpected argument: lower`).
            return Err(vec![SourceDiagnostic::error(
                named.name().to_untyped().span(),
                format!("unexpected argument: {nome}"),
            )]);
        }

        if MATH_SET_LIGADOS.contains(&(elem, nome)) {
            let val = eval_expr(named.expr(), scopes, ctx, engine)?;
            *engine.styles =
                engine.styles.push_custom(format!("math.{elem}.{nome}"), val);
        } else {
            engine.sink.warn_note(
                named.name().to_untyped().span(),
                &format!(
                    "#set math.{elem}({nome}:) é da linguagem mas ainda não tem efeito"
                ),
                "o parâmetro é aceite e ignorado; o documento compila sem ele",
            );
        }
    }

    Ok(Value::None)
}

/// **P863** — Realiza parágrafos antes de aplicar show rules.
///
/// Agrupa, em cada contexto de fluxo (`Sequence`), o conteúdo entre
/// `Content::Parbreak`s em nós `Content::Par`. Sub-árvores matemáticas
/// (`Equation`, `MathSequence` e família `Math*`) são preservadas sem wrapping.
pub(crate) fn realize_paragraphs(content: Content) -> Content {
    realize_node(&content)
}

/// Retorna os itens de um fluxo já realizados. `Sequence` é achatada e os
/// trechos entre `Parbreak` são embrulhados em `Content::Par`.
fn realize_flow(content: &Content) -> Vec<Content> {
    match content {
        Content::Sequence(seq) => {
            let mut out = Vec::new();
            let mut current = Vec::new();
            for child in seq.iter() {
                for piece in realize_flow(child) {
                    if matches!(piece, Content::Parbreak) {
                        if !current.is_empty() {
                            out.push(Content::par(Content::sequence(current)));
                            current = Vec::new();
                        }
                        out.push(piece);
                    } else {
                        current.push(piece);
                    }
                }
            }
            if !current.is_empty() {
                out.push(Content::par(Content::sequence(current)));
            }
            out
        }
        other => vec![realize_node(other)],
    }
}

macro_rules! realize_body {
    ($e:expr, $variant:path) => {{
        let mut elem = Arc::unwrap_or_clone(Arc::clone($e));
        elem.body = realize_node(&elem.body);
        $variant(Arc::new(elem))
    }};
}

fn realize_node(content: &Content) -> Content {
    match content {
        Content::Sequence(_) => Content::sequence(realize_flow(content)),
        Content::Par { body } => Content::par(realize_node(body)),
        Content::Styled(body, styles) => {
            Content::Styled(Box::new(realize_node(body)), styles.clone())
        }
        Content::Document { title, author, date, keywords } => Content::Document {
            title: title.as_ref().map(|t| Box::new(realize_node(t))),
            author: author.clone(),
            date: *date,
            keywords: keywords.clone(),
        },

        // Containers com um único `body`.
        Content::Block(e) => realize_body!(e, Content::Block),
        Content::Boxed(e) => realize_body!(e, Content::Boxed),
        Content::Pad(e) => realize_body!(e, Content::Pad),
        Content::Align(e) => realize_body!(e, Content::Align),
        Content::Hide(e) => realize_body!(e, Content::Hide),
        Content::Transform(e) => realize_body!(e, Content::Transform),
        Content::Columns(e) => realize_body!(e, Content::Columns),
        Content::Repeat(e) => realize_body!(e, Content::Repeat),
        Content::ListItem(e) => realize_body!(e, Content::ListItem),
        Content::EnumItem(e) => realize_body!(e, Content::EnumItem),
        Content::Footnote(e) => realize_body!(e, Content::Footnote),
        Content::Label(e) => realize_body!(e, Content::Label),
        Content::Strong(e) => realize_body!(e, Content::Strong),
        Content::Emph(e) => realize_body!(e, Content::Emph),
        Content::Underline(e) => realize_body!(e, Content::Underline),
        Content::Strike(e) => realize_body!(e, Content::Strike),
        Content::Overline(e) => realize_body!(e, Content::Overline),
        Content::GridCell(e) => realize_body!(e, Content::GridCell),
        Content::GridHeader(e) => realize_body!(e, Content::GridHeader),
        Content::GridFooter(e) => realize_body!(e, Content::GridFooter),
        Content::TableCell(e) => realize_body!(e, Content::TableCell),
        Content::TableHeader(e) => realize_body!(e, Content::TableHeader),
        Content::TableFooter(e) => realize_body!(e, Content::TableFooter),
        Content::Title(e) => realize_body!(e, Content::Title),
        // `OutlineElem` não tem campo `body`; é folha para realização.
        Content::Outline(_) => content.clone(),
        Content::MathStyled(e) => realize_body!(e, Content::MathStyled),

        // Containers com body explícito inline.
        Content::SmallCaps { body } => Content::smallcaps(realize_node(body)),

        // Containers com múltiplos conteúdos.
        Content::Figure(e) => {
            let mut e = Arc::unwrap_or_clone(Arc::clone(e));
            e.body = realize_node(&e.body);
            e.caption = e.caption.as_ref().map(realize_node);
            Content::Figure(Arc::new(e))
        }
        Content::Quote(e) => {
            let mut e = Arc::unwrap_or_clone(Arc::clone(e));
            e.body = realize_node(&e.body);
            e.attribution = e.attribution.as_ref().map(realize_node);
            Content::Quote(Arc::new(e))
        }
        Content::TermItem(e) => {
            let mut e = Arc::unwrap_or_clone(Arc::clone(e));
            e.term = realize_node(&e.term);
            e.description = realize_node(&e.description);
            Content::TermItem(Arc::new(e))
        }
        Content::Grid(e) => {
            let mut e = Arc::unwrap_or_clone(Arc::clone(e));
            e.cells = e.cells.iter().map(realize_node).collect();
            e.header = e.header.as_ref().map(realize_node);
            e.footer = e.footer.as_ref().map(realize_node);
            Content::Grid(Arc::new(e))
        }
        Content::Table(e) => {
            let mut e = Arc::unwrap_or_clone(Arc::clone(e));
            e.children = e.children.iter().map(realize_node).collect();
            e.caption = e.caption.as_ref().map(realize_node);
            Content::Table(Arc::new(e))
        }
        Content::Stack(e) => {
            let mut e = Arc::unwrap_or_clone(Arc::clone(e));
            e.children =
                Arc::from(e.children.iter().map(realize_node).collect::<Vec<_>>());
            Content::Stack(Arc::new(e))
        }
        Content::Terms(e) => {
            let mut e = Arc::unwrap_or_clone(Arc::clone(e));
            e.items = e.items.iter().map(realize_node).collect();
            Content::Terms(Arc::new(e))
        }

        // Matemática e folhas: preservar sem wrapping.
        other => other.clone(),
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

    // P863: se há regras sobre parágrafos, realizar os parágrafos antes de
    // aplicar as show rules. Realização é transparente para o layout.
    let has_par_rule = rules
        .iter()
        .any(|r| matches!(r.selector, Selector::NodeKind(NodeKind::Par)));
    if has_par_rule {
        content = realize_paragraphs(content);
    }

    // Separar regras por tipo para travessias distintas. Lote F-3 inc-2: as
    // regras de **kind dinâmico** (`#show callout:`) viajam pela MESMA travessia
    // que as NodeKind — mesmo `apply_all`, mesma ordem, mesmo guard por `RuleId`.
    let has_node_rules =
        rules.iter().any(|r| selector_matching::is_node_rule(&r.selector));

    if has_node_rules {
        // Única travessia para todas as NodeKind + DynKind rules, incluindo
        // Where com base node-like (P417).
        let node_rules: Vec<ShowRule> = rules
            .iter()
            .filter(|r| selector_matching::is_node_rule(&r.selector))
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
            // P348 (modelo α, ADR-0107): a element rule cujo output **re-casa** é
            // **revisitada** até **ponto-fixo morfológico**. O loop α vive agora em
            // `compiler/eval/show_rule_termination.rs` (Passo 1009), que acrescenta
            // detecção de ciclos por histórico de formas canónicas.
            let full_error = ctx.full_error;
            let (mut work, applied) = show_rule_termination::run_show_rule_loop(
                node.clone(),
                |work| {
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

                        if !selector_matching::selector_matches(work, &rule.selector) {
                            continue;
                        }

                        match &rule.transform {
                            Transformation::Func(func) => {
                                let args =
                                    Args::positional(vec![Value::Content(work.clone())]);
                                engine.active_guards.push(rule.id);
                                let call_result = call_dispatch::apply_func(
                                    func.clone(),
                                    args,
                                    &mut scopes,
                                    ctx,
                                    engine,
                                );
                                engine.active_guards.pop();
                                let produced = match call_result? {
                                    Value::Content(c) => c,
                                    Value::Str(s) => Content::text(s.as_str()),
                                    other => {
                                        return Err(vec![SourceDiagnostic::error(
                                            Span::detached(),
                                            format!(
                                                "show rule deve retornar Content ou String, \
                                                 recebeu {}",
                                                other.type_name()
                                            ),
                                        )])
                                    }
                                };
                                return Ok(Some((produced, rule.id)));
                            }
                            Transformation::Content(c) => {
                                return Ok(Some((c.clone(), rule.id)));
                            }
                            // `Str` só é válida sobre `Selector::Text` (tratada no loop
                            // de texto). Sobre NodeKind/DynKind é erro — paridade com o
                            // comportamento anterior ("recebeu str").
                            Transformation::Str(_) => {
                                return Err(vec![SourceDiagnostic::error(
                                    Span::detached(),
                                    "show rule com selector de tipo requer função ou Content, \
                                     recebeu str"
                                        .to_string(),
                                )])
                            }
                            // Saltado acima; inalcançável.
                            Transformation::Style(_) => continue,
                        }
                    }
                    Ok(None)
                },
                crate::entities::world_types::Route::MAX_SHOW_RULE_DEPTH,
                full_error,
            )?;

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
                    if selector_matching::selector_matches(node, &rule.selector) {
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

            if applied || wrapped {
                Ok(Some(work))
            } else {
                Ok(None)
            }
        };

        content = content.map_content(&mut apply_all)?;
    }

    // Regex rules (P393) — aplicadas a nós de texto que casam.
    // Última-declarada vence (consistente com NodeKind). Transformações
    // Func/Content/Str são suportadas; Style foi rejeitado em parse.
    let regex_rules: Vec<ShowRule> = rules
        .iter()
        .filter(|r| matches!(r.selector, Selector::Regex(_)))
        .cloned()
        .collect();

    if !regex_rules.is_empty() {
        let mut apply_regex = |node: &Content| -> SourceResult<Option<Content>> {
            let Content::Text(text) = node else {
                return Ok(None);
            };
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
                        let call_result = call_dispatch::apply_func(
                            func.clone(),
                            args,
                            &mut scopes,
                            ctx,
                            engine,
                        );
                        engine.active_guards.pop();
                        match call_result? {
                            Value::Content(c) => c,
                            Value::Str(s) => Content::text(s.as_str()),
                            other => {
                                return Err(vec![SourceDiagnostic::error(
                                    Span::detached(),
                                    format!(
                                    "show rule regex deve retornar Content ou String, \
                                     recebeu {}",
                                    other.type_name()
                                ),
                                )])
                            }
                        }
                    }
                    Transformation::Content(c) => c.clone(),
                    Transformation::Str(s) => Content::text(s.as_str()),
                    Transformation::Style(_) => continue,
                };
                return Ok(Some(produced));
            }
            Ok(None)
        };
        content = content.map_content(&mut apply_regex)?;
    }

    // Text rules — P790: `Str` via `map_text` (comportamento pré-existente);
    // `Content`/`Func` por splice em `map_content` — o `Content::Text` é
    // fatiado nas ocorrências do padrão e o replacement é emendado entre as
    // fatias (paridade vanilla `visit_regex_match`; `map_content` não
    // reentra no nó substituído, logo o output não é re-varrido pela mesma
    // regra — equivalente à `Revocation` do vanilla). Match por nó de texto
    // individual (cross-node: scope-out registado no L0, eval.md §P790).
    for rule in rules {
        let Selector::Text(pattern) = &rule.selector else { continue };
        match &rule.transform {
            Transformation::Str(s) => {
                let replacement = s.to_string();
                let mut do_replace =
                    |text: &str| text.replace(pattern.as_str(), &replacement);
                content = content.map_text(&mut do_replace);
            }
            Transformation::Content(replacement) => {
                let mut apply_text = |node: &Content| -> SourceResult<Option<Content>> {
                    let Content::Text(text) = node else { return Ok(None) };
                    selector_matching::splice_text_rule_matches(
                        text.as_str(),
                        pattern,
                        |_| Ok(replacement.clone()),
                    )
                };
                content = content.map_content(&mut apply_text)?;
            }
            Transformation::Func(func) => {
                let mut apply_text = |node: &Content| -> SourceResult<Option<Content>> {
                    let Content::Text(text) = node else { return Ok(None) };
                    selector_matching::splice_text_rule_matches(
                        text.as_str(),
                        pattern,
                        |matched| {
                            let args = Args::positional(vec![Value::Content(
                                Content::text(matched),
                            )]);
                            engine.active_guards.push(rule.id);
                            let call_result = call_dispatch::apply_func(
                                func.clone(),
                                args,
                                &mut scopes,
                                ctx,
                                engine,
                            );
                            engine.active_guards.pop();
                            match call_result? {
                                Value::Content(c) => Ok(c),
                                Value::Str(s) => Ok(Content::text(s.as_str())),
                                other => Err(vec![SourceDiagnostic::error(
                                    Span::detached(),
                                    format!(
                                        "show rule deve retornar Content ou String, \
                                     recebeu {}",
                                        other.type_name()
                                    ),
                                )]),
                            }
                        },
                    )
                };
                content = content.map_content(&mut apply_text)?;
            }
            Transformation::Style(_) => {
                // Rejeitado em `eval_show_rule` para `Selector::Text` —
                // inalcançável (show-set é sobre elementos).
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
    // P498 — passagem de captura do conteúdo original: não aplicar show-rules,
    // apenas registá-las (feito em `eval_show_rule`). Isto preserva os elementos
    // locatable para a introspecção, espelhando o modelo vanilla.
    if !ctx.apply_show_rules {
        return Ok(content);
    }

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

/// **P863** — Intercepção específica de parágrafos no fluxo montado pelo markup.
///
/// Parágrafos são sintetizados a partir de vários nós de texto/`Space`/`Parbreak`,
/// pelo que não têm um único ponto de construção onde `intercept_content` possa
/// atuar. Esta função realiza os parágrafos e aplica **apenas** as regras
/// `NodeKind::Par`, deixando as regras de texto (já aplicadas eager em cada nó
/// de texto) e as restantes regras de elemento (já aplicadas nos seus pontos de
/// construção) intactas.
pub(crate) fn intercept_paragraphs(
    content: Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if !ctx.apply_show_rules {
        return Ok(content);
    }

    let par_rules: Vec<ShowRule> = engine
        .show_rules
        .iter()
        .filter(|r| matches!(r.selector, Selector::NodeKind(NodeKind::Par)))
        .cloned()
        .collect();

    if par_rules.is_empty() {
        return Ok(content);
    }

    apply_show_rules(content, &par_rules, ctx, engine)
}

/// **P791** — Intercepção de wrappers `Content::Label` para regras
/// `Selector::Label` (`#show <sp>: …`). Chamada no ponto de associação
/// retroactiva de `<label>` em markup (Passo 56, `eval/mod.rs`): o wrapper é
/// criado **depois** do corpo já ter viajado pela maquinaria geral de show
/// rules, logo aqui só regras de label casam — as outras já aplicaram nos
/// seus pontos próprios (sem dupla aplicação).
///
/// Semântica (paridade vanilla, `target.label()` — o label é metadado do
/// elemento, não um nó): `it` = o **corpo** rotulado; a saída substitui o
/// wrapper inteiro (label consumido). Aplicação **única**,
/// última-declarada primeiro (innermost-first, P358) — sem revisitação
/// (equivalente à `Revocation` do vanilla). `Content` substitui; `Str` é
/// erro (consistente com `NodeKind`); show-set (`Style`) **não consome o
/// passe** — as que casam são dobradas e embrulham o wrapper em
/// `Content::Styled` (paridade P352).
pub(crate) fn intercept_labelled(
    labelled: Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if !ctx.apply_show_rules || engine.show_rules.is_empty() {
        return Ok(labelled);
    }
    let Content::Label(e) = &labelled else {
        return Ok(labelled);
    };
    // Snapshot (Passo 84.4): slice estável enquanto closures podem mutar
    // `engine.show_rules`.
    let rules = Arc::clone(&*engine.show_rules);
    let mut scopes = Scopes::new(None);

    // Show-set: dobrar todas as regras de label que casam (ordem de
    // declaração, como no loop α) e embrulhar uma vez (collapse = top-wins).
    let mut set_chain = StyleChain::empty();
    let mut any_set = false;
    for rule in rules.iter() {
        if engine.active_guards.contains(&rule.id) {
            continue;
        }
        if let (Selector::Label(l), Transformation::Style(styles)) =
            (&rule.selector, &rule.transform)
        {
            if e.name.as_str() == l.0.as_str() {
                set_chain = set_chain.push(styles.delta().clone());
                any_set = true;
            }
        }
    }

    // Func/Content: primeira regra que casa (última-declarada primeiro),
    // uma única aplicação.
    for rule in rules.iter().rev() {
        if engine.active_guards.contains(&rule.id) {
            continue;
        }
        let Selector::Label(l) = &rule.selector else { continue };
        if e.name.as_str() != l.0.as_str() {
            continue;
        }
        let produced: Option<Content> = match &rule.transform {
            Transformation::Func(func) => {
                let args = Args::positional(vec![Value::Content(e.body.clone())]);
                engine.active_guards.push(rule.id);
                let call_result = call_dispatch::apply_func(
                    func.clone(),
                    args,
                    &mut scopes,
                    ctx,
                    engine,
                );
                engine.active_guards.pop();
                Some(match call_result? {
                    Value::Content(c) => c,
                    Value::Str(s) => Content::text(s.as_str()),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            Span::detached(),
                            format!(
                                "show rule deve retornar Content ou String, \
                                 recebeu {}",
                                other.type_name()
                            ),
                        )])
                    }
                })
            }
            Transformation::Content(c) => Some(c.clone()),
            // `Str` sobre label é erro — paridade com o comportamento sobre
            // `NodeKind` ("requer função ou Content, recebeu str").
            Transformation::Str(_) => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "show rule com selector de label requer função ou Content, \
                     recebeu str"
                        .to_string(),
                )])
            }
            // Show-set: tratada acima (não consome o passe).
            Transformation::Style(_) => None,
        };
        if let Some(out) = produced {
            return Ok(out);
        }
    }

    if any_set {
        let delta = set_chain.collapse();
        if !delta.is_empty() {
            return Ok(Content::Styled(Box::new(labelled), Styles::from_delta(delta)));
        }
    }
    Ok(labelled)
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
        let mut active = false;
        let mut pattern: Option<ecow::EcoString> = None;
        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                if named.name().as_str() == "numbering" {
                    // Defensivo: só String activa a numeração.
                    // Closures, none, ou outros tipos → ignorar.
                    let val = eval_expr(named.expr(), scopes, ctx, engine)
                        .unwrap_or(Value::None);
                    if let Value::Str(s) = val {
                        active = true;
                        pattern = Some(s.clone());
                    }
                }
            }
        }
        // Lote F-2 S1 (P335): em vez de um marcador global `SetHeadingNumbering`,
        // empurra para a chain léxica (`engine.styles` é escopado por
        // `local_styles`). O heading assa este valor na criação
        // (`eval/markup.rs`). Fecha o canal global → escopo de container (DEBT 99.E).
        // P451: transporta também o pattern string para formatação configurável.
        *engine.styles =
            engine.styles.push_custom("heading.numbering", Value::Bool(active));
        if let Some(pattern) = pattern {
            *engine.styles = engine
                .styles
                .push_custom("heading.numbering.pattern", Value::Str(pattern));
        }
        return Ok(Value::None);
    }

    // Lote F-2 S2 / **B1** (P335): `#set math.equation(numbering:)` — target
    // **pontuado** (`FieldAccess` `math.equation`), que `text_str()` não capta
    // (nó interno → ""). Era a lacuna do P331 (sem produtor eval). Empurra para
    // a chain léxica; a equação assa o valor (`eval/mod.rs`). Paridade vanilla:
    // `lab/.../math/equation.rs` — numeração de equação de bloco.
    // P456: guarda o pattern string (não Bool) para formatar via format_counter,
    // análogo a figure.numbering (P454) e heading.numbering.pattern (P451).
    if let Expr::FieldAccess(fa) = set.target() {
        if fa.target().to_untyped().text_str() == "math"
            && fa.field().to_untyped().text_str() == "equation"
        {
            let numbering = set.args().items().find_map(|arg| {
                if let Arg::Named(named) = arg {
                    if named.name().as_str() == "numbering" {
                        // P636: propagar erro de avaliação (variável indefinida,
                        // por exemplo) em vez de descartar com `.ok()`.
                        return Some((
                            named.expr().span(),
                            eval_expr(named.expr(), scopes, ctx, engine),
                        ));
                    }
                }
                None
            });
            match numbering {
                Some((_, Ok(Value::Str(s)))) => {
                    *engine.styles =
                        engine.styles.push_custom("equation.numbering", Value::Str(s));
                }
                Some((_, Ok(Value::None))) => {
                    // Limpa a numeração no escopo (None = ausente).
                    *engine.styles =
                        engine.styles.push_custom("equation.numbering", Value::None);
                }
                Some((_, Ok(Value::Func(f)))) => {
                    *engine.styles =
                        engine.styles.push_custom("equation.numbering", Value::Func(f));
                }
                Some((span, Ok(other))) => {
                    return Err(vec![type_mismatch(
                        "string, function, or none",
                        &other,
                        span,
                    )]);
                }
                Some((_, Err(err))) => return Err(err),
                None => {} // neutro: N16[β] — argumento numbering omitido em set-rule: estilos inalterados
            }
            let number_align = set.args().items().find_map(|arg| {
                if let Arg::Named(named) = arg {
                    if named.name().as_str() == "number-align" {
                        return Some((
                            named.expr().span(),
                            eval_expr(named.expr(), scopes, ctx, engine),
                        ));
                    }
                }
                None
            });
            match number_align {
                Some((span, Ok(value))) => {
                    crate::compiler::stdlib::equation_number_align(&value, span)?;
                    *engine.styles =
                        engine.styles.push_custom("equation.number-align", value);
                }
                Some((_, Err(err))) => return Err(err),
                None => {}
            }
            let supplement = set.args().items().find_map(|arg| {
                if let Arg::Named(named) = arg {
                    if named.name().as_str() == "supplement" {
                        return Some((
                            named.expr().span(),
                            eval_expr(named.expr(), scopes, ctx, engine),
                        ));
                    }
                }
                None
            });
            match supplement {
                Some((span, Ok(value))) => {
                    let value =
                        crate::compiler::stdlib::equation_supplement(&value, span)?;
                    *engine.styles =
                        engine.styles.push_custom("equation.supplement", value);
                }
                Some((_, Err(err))) => return Err(err),
                None => {}
            }
            let alt = set.args().items().find_map(|arg| {
                if let Arg::Named(named) = arg {
                    if named.name().as_str() == "alt" {
                        return Some((
                            named.expr().span(),
                            eval_expr(named.expr(), scopes, ctx, engine),
                        ));
                    }
                }
                None
            });
            match alt {
                Some((span, Ok(value))) => {
                    let value = crate::compiler::stdlib::equation_alt(&value, span)?;
                    *engine.styles = engine.styles.push_custom("equation.alt", value);
                }
                Some((_, Err(err))) => return Err(err),
                None => {}
            }
            return Ok(Value::None);
        }
    }

    // **P1030** — restantes alvos pontuados `math.<elemento>` (`eval.md`
    // §P1030). Tem de vir DEPOIS do braço de `math.equation` acima, que
    // continua a ser o dono de `numbering`.
    if let Expr::FieldAccess(fa) = set.target() {
        if fa.target().to_untyped().text_str() == "math" {
            let elem = fa.field().to_untyped().text_str().to_owned();
            return eval_set_math_rule(&elem, set, scopes, ctx, engine, target_span);
        }
    }

    if target == "document" {
        // **P536** — `#set document(title: ..., author: ..., keywords: ...)`.
        // Extrai os valores, converte arrays para strings separadas por vírgula,
        // e acumula no `EvalContext`. O eval copia isto para o `Module` no final;
        // o pipeline transporta para o exportador PDF (`/Info`).
        fn value_to_eco_string(
            val: &crate::entities::value::Value,
            span: Span,
            allow_array: bool,
        ) -> SourceResult<Option<ecow::EcoString>> {
            match val {
                crate::entities::value::Value::Str(s) => Ok(Some(s.clone())),
                crate::entities::value::Value::Array(arr) if allow_array => {
                    let mut parts = Vec::new();
                    for v in arr.iter() {
                        match v {
                            crate::entities::value::Value::Str(s) => {
                                parts.push(s.as_str())
                            }
                            other => {
                                return Err(vec![type_mismatch("string", other, span)]);
                            }
                        }
                    }
                    if parts.is_empty() {
                        Ok(None)
                    } else {
                        Ok(Some(ecow::EcoString::from(parts.join(", "))))
                    }
                }
                crate::entities::value::Value::None => Ok(None),
                other => {
                    let expected =
                        if allow_array { "string or array of strings" } else { "string" };
                    Err(vec![type_mismatch(expected, other, span)])
                }
            }
        }

        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                let key = named.name().as_str();
                let val = eval_expr(named.expr(), scopes, ctx, engine)?;
                let span = named.expr().span();
                match key {
                    // P637: no vanilla, title é Option<Content> (não array);
                    // author/keywords são OneOrMultiple<EcoString>.
                    "title" => {
                        ctx.document_info.title = value_to_eco_string(&val, span, false)?
                    }
                    "author" => {
                        ctx.document_info.author = value_to_eco_string(&val, span, true)?
                    }
                    "keywords" => {
                        ctx.document_info.keywords =
                            value_to_eco_string(&val, span, true)?
                    }
                    _ => {}
                }
            }
        }
        return Ok(Value::None);
    }

    if target == "page" {
        // #set page(width: .., height: .., margin: ..) — Passo 81.
        // Valores ausentes ficam None e preservam o valor actual em layout.
        // P636: rejeitar tipos inválidos em vez de ignorar silenciosamente.
        // **P757** — `em` em dimensões de página deve resolver contra o
        // tamanho de fonte activo na chain (default 11 pt), não contra zero.
        // Anteriormente usava-se `l.abs.to_pt()`, que descartava a componente
        // `em` e produzia `0pt` silenciosamente para valores como `7em`.
        // **P867** — `width`/`height` aceitam `auto`; `margin` mantém-se
        // length/float/int/none neste passo.
        let size_pt = engine.styles.size();
        fn extract_page_dimension(
            val: &Value,
            span: Span,
            size_pt: f64,
        ) -> SourceResult<Option<crate::entities::layout_types::PageDimension>> {
            match val {
                Value::Length(l) => {
                    Ok(Some(crate::entities::layout_types::PageDimension::Length(
                        l.resolve_pt(size_pt),
                    )))
                }
                Value::Float(f) => {
                    Ok(Some(crate::entities::layout_types::PageDimension::Length(*f)))
                }
                Value::Int(i) => Ok(Some(
                    crate::entities::layout_types::PageDimension::Length(*i as f64),
                )),
                Value::Auto => {
                    Ok(Some(crate::entities::layout_types::PageDimension::Auto))
                }
                Value::None => Ok(None),
                other => {
                    Err(vec![type_mismatch("length, float, int, or auto", other, span)])
                }
            }
        }
        fn extract_pt(
            val: &Value,
            span: Span,
            size_pt: f64,
        ) -> SourceResult<Option<f64>> {
            match val {
                Value::Length(l) => Ok(Some(l.resolve_pt(size_pt))),
                Value::Float(f) => Ok(Some(*f)),
                Value::Int(i) => Ok(Some(*i as f64)),
                Value::None => Ok(None),
                other => Err(vec![type_mismatch("length, float, or int", other, span)]),
            }
        }
        fn extract_rel_length(
            val: &Value,
            span: Span,
        ) -> SourceResult<crate::entities::rel::Rel<crate::entities::layout_types::Length>>
        {
            use crate::entities::layout_types::Length;
            use crate::entities::rel::Rel;
            match val {
                Value::Relative(value) => Ok(*value),
                Value::Ratio(value) => Ok(Rel { rel: value.get(), abs: Length::ZERO }),
                Value::Length(value) => Ok(Rel { rel: 0.0, abs: *value }),
                Value::Float(value) => Ok(Rel { rel: 0.0, abs: Length::pt(*value) }),
                Value::Int(value) => Ok(Rel { rel: 0.0, abs: Length::pt(*value as f64) }),
                other => Err(vec![type_mismatch("relative length", other, span)]),
            }
        }
        let mut width: Option<crate::entities::layout_types::PageDimension> = None;
        let mut paper = None;
        let mut flipped = None;
        let mut binding = None;
        let mut height: Option<crate::entities::layout_types::PageDimension> = None;
        let mut margin: Option<crate::entities::layout_types::PageMarginSpec> = None;
        let mut margin_left = None;
        let mut margin_right = None;
        let mut margin_top = None;
        let mut margin_bottom = None;
        let mut numbering = None;
        let mut number_align = None;
        let mut header = None;
        let mut header_ascent = None;
        let mut footer = None;
        let mut footer_descent = None;
        let mut supplement = None;
        let mut columns: Option<usize> = None;
        let mut bleed = None;
        let mut fill = None;
        let mut background = None;
        let mut foreground = None;
        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                let key = named.name().as_str();
                let val = eval_expr(named.expr(), scopes, ctx, engine)?;
                let span = named.expr().span();
                match key {
                    "paper" => {
                        paper = match val {
                            Value::Str(s) => {
                                crate::entities::page_geometry::Paper::from_name(&s)
                                    .ok_or_else(|| {
                                        vec![SourceDiagnostic::error(
                                            span,
                                            "unknown paper size",
                                        )]
                                    })
                                    .map(Some)?
                            }
                            other => {
                                return Err(vec![type_mismatch("string", &other, span)])
                            }
                        }
                    }
                    "flipped" => {
                        flipped = match val {
                            Value::Bool(v) => Some(v),
                            other => {
                                return Err(vec![type_mismatch("bool", &other, span)])
                            }
                        }
                    }
                    "binding" => {
                        binding = match val {
                            Value::Auto => {
                                Some(crate::entities::page_geometry::PageBinding::Auto)
                            }
                            Value::Align(a)
                                if a.v.is_none()
                                    && a.h
                                        == Some(
                                            crate::entities::layout_types::HAlign::Left,
                                        ) =>
                            {
                                Some(crate::entities::page_geometry::PageBinding::Left)
                            }
                            Value::Align(a)
                                if a.v.is_none()
                                    && a.h
                                        == Some(
                                            crate::entities::layout_types::HAlign::Right,
                                        ) =>
                            {
                                Some(crate::entities::page_geometry::PageBinding::Right)
                            }
                            other => {
                                return Err(vec![type_mismatch(
                                    "auto, left, or right",
                                    &other,
                                    span,
                                )])
                            }
                        }
                    }
                    "width" => width = extract_page_dimension(&val, span, size_pt)?,
                    "height" => height = extract_page_dimension(&val, span, size_pt)?,
                    "margin" => match &val {
                        Value::Dict(d) => {
                            if let Some(key) = d.keys().find(|k| {
                                !matches!(
                                    k.as_str(),
                                    "left"
                                        | "right"
                                        | "top"
                                        | "bottom"
                                        | "inside"
                                        | "outside"
                                        | "x"
                                        | "y"
                                        | "rest"
                                )
                            }) {
                                return Err(vec![SourceDiagnostic::error(
                                    span,
                                    format!("unknown margin key: {key}"),
                                )]);
                            }
                            let logical =
                                d.get("inside").is_some() || d.get("outside").is_some();
                            let physical =
                                d.get("left").is_some() || d.get("right").is_some();
                            if logical && physical {
                                return Err(vec![SourceDiagnostic::error(span, "`inside` and `outside` are mutually exclusive with `left` and `right`")]);
                            }
                            let extract_field = |k: &str| -> SourceResult<Option<f64>> {
                                if let Some(v) = d.get(k) {
                                    extract_pt(v, span, size_pt)
                                } else {
                                    Ok(None)
                                }
                            };
                            let mx = extract_field("x")?;
                            let my = extract_field("y")?;
                            let rest = extract_field("rest")?;
                            margin_left =
                                extract_field(if logical { "inside" } else { "left" })?
                                    .or(mx)
                                    .or(rest);
                            margin_right =
                                extract_field(if logical { "outside" } else { "right" })?
                                    .or(mx)
                                    .or(rest);
                            margin_top = extract_field("top")?.or(my).or(rest);
                            margin_bottom = extract_field("bottom")?.or(my).or(rest);
                            margin =
                                Some(crate::entities::layout_types::PageMarginSpec {
                                    left: margin_left,
                                    right: margin_right,
                                    top: margin_top,
                                    bottom: margin_bottom,
                                    two_sided: if logical || physical {
                                        Some(logical)
                                    } else {
                                        None
                                    },
                                });
                        }
                        Value::Auto => {
                            margin = Some(
                                crate::entities::layout_types::PageMarginSpec::auto(),
                            );
                        }
                        other => {
                            if let Some(m) = extract_pt(other, span, size_pt)? {
                                margin_left = Some(m);
                                margin_right = Some(m);
                                margin_top = Some(m);
                                margin_bottom = Some(m);
                                margin = Some(
                                    crate::entities::layout_types::PageMarginSpec::uniform(m),
                                );
                            }
                        }
                    },
                    "numbering" => {
                        numbering = match val {
                            Value::Str(s) => Some(s),
                            Value::None => Some(ecow::EcoString::new()),
                            other => {
                                return Err(vec![type_mismatch(
                                    "string or none",
                                    &other,
                                    span,
                                )]);
                            }
                        };
                    }
                    "number-align" => {
                        let Value::Align(align) = val else {
                            return Err(vec![type_mismatch("alignment", &val, span)]);
                        };
                        let horizontal = align
                            .h
                            .unwrap_or(crate::entities::layout_types::HAlign::Center);
                        let vertical = match align
                            .v
                            .unwrap_or(crate::entities::layout_types::VAlign::Bottom)
                        {
                            crate::entities::layout_types::VAlign::Top => {
                                crate::entities::page_running::PageNumberVAlign::Top
                            }
                            crate::entities::layout_types::VAlign::Bottom => {
                                crate::entities::page_running::PageNumberVAlign::Bottom
                            }
                            crate::entities::layout_types::VAlign::Horizon => {
                                return Err(vec![SourceDiagnostic::error(
                                    span,
                                    "page number-align cannot use horizon".to_string(),
                                )])
                            }
                        };
                        number_align =
                            Some(crate::entities::page_running::PageNumberAlign {
                                horizontal,
                                vertical,
                            });
                    }
                    "header" => {
                        header = Some(match val {
                            Value::Auto => {
                                crate::entities::page_running::PageMarginal::Auto
                            }
                            Value::None => {
                                crate::entities::page_running::PageMarginal::None
                            }
                            Value::Content(content) => {
                                crate::entities::page_running::PageMarginal::Content(
                                    std::sync::Arc::new(content),
                                )
                            }
                            other => {
                                return Err(vec![type_mismatch(
                                    "auto, none, or content",
                                    &other,
                                    span,
                                )])
                            }
                        });
                    }
                    "header-ascent" => {
                        header_ascent = Some(extract_rel_length(&val, span)?)
                    }
                    "footer" => {
                        footer = Some(match val {
                            Value::Auto => {
                                crate::entities::page_running::PageMarginal::Auto
                            }
                            Value::None => {
                                crate::entities::page_running::PageMarginal::None
                            }
                            Value::Content(content) => {
                                crate::entities::page_running::PageMarginal::Content(
                                    std::sync::Arc::new(content),
                                )
                            }
                            other => {
                                return Err(vec![type_mismatch(
                                    "auto, none, or content",
                                    &other,
                                    span,
                                )])
                            }
                        });
                    }
                    "footer-descent" => {
                        footer_descent = Some(extract_rel_length(&val, span)?)
                    }
                    "supplement" => {
                        supplement = Some(match val {
                            Value::Auto => {
                                crate::entities::page_supplement::PageSupplement::Auto
                            }
                            Value::None => {
                                crate::entities::page_supplement::PageSupplement::None
                            }
                            Value::Content(content) => {
                                crate::entities::page_supplement::PageSupplement::Content(
                                    std::sync::Arc::new(content),
                                )
                            }
                            Value::Str(text) => {
                                crate::entities::page_supplement::PageSupplement::Content(
                                    std::sync::Arc::new(Content::text(text)),
                                )
                            }
                            other => {
                                return Err(vec![type_mismatch(
                                    "auto, none, string, or content",
                                    &other,
                                    span,
                                )])
                            }
                        });
                    }
                    "columns" => {
                        columns = match val {
                            Value::Int(n) if n >= 1 => Some(n as usize),
                            Value::Int(n) if n < 1 => {
                                return Err(vec![SourceDiagnostic::error(
                                    named.span(),
                                    "columns must be at least 1".to_string(),
                                )]);
                            }
                            Value::None => None,
                            other => {
                                return Err(vec![type_mismatch("int", &other, span)]);
                            }
                        };
                    }
                    "bleed" => {
                        use crate::entities::page_canvas::PageBleedSpec;
                        bleed = Some(match &val {
                            Value::Dict(dict) => {
                                if let Some(key) = dict.keys().find(|key| {
                                    !matches!(
                                        key.as_str(),
                                        "left"
                                            | "right"
                                            | "top"
                                            | "bottom"
                                            | "inside"
                                            | "outside"
                                            | "x"
                                            | "y"
                                            | "rest"
                                    )
                                }) {
                                    return Err(vec![SourceDiagnostic::error(
                                        span,
                                        format!("unknown bleed key: {key}"),
                                    )]);
                                }
                                let logical = dict.get("inside").is_some()
                                    || dict.get("outside").is_some();
                                let physical = dict.get("left").is_some()
                                    || dict.get("right").is_some();
                                if logical && physical {
                                    return Err(vec![SourceDiagnostic::error(
                                        span,
                                        "`inside` and `outside` are mutually exclusive with `left` and `right`".to_string(),
                                    )]);
                                }
                                let field = |key: &str| -> SourceResult<
                                    Option<
                                        crate::entities::rel::Rel<
                                            crate::entities::layout_types::Length,
                                        >,
                                    >,
                                > {
                                    dict.get(key)
                                        .map(|value| extract_rel_length(value, span))
                                        .transpose()
                                };
                                let x = field("x")?;
                                let y = field("y")?;
                                let rest = field("rest")?;
                                PageBleedSpec {
                                    left: field(if logical { "inside" } else { "left" })?
                                        .or(x)
                                        .or(rest),
                                    right: field(if logical {
                                        "outside"
                                    } else {
                                        "right"
                                    })?
                                    .or(x)
                                    .or(rest),
                                    top: field("top")?.or(y).or(rest),
                                    bottom: field("bottom")?.or(y).or(rest),
                                    two_sided: if logical || physical {
                                        Some(logical)
                                    } else {
                                        None
                                    },
                                }
                            }
                            Value::Auto => {
                                return Err(vec![type_mismatch(
                                    "relative length or dictionary",
                                    &val,
                                    span,
                                )]);
                            }
                            _ => PageBleedSpec::uniform(extract_rel_length(&val, span)?),
                        });
                    }
                    "fill" => {
                        fill = Some(match val {
                            Value::Auto => crate::entities::page_canvas::PageFill::Auto,
                            Value::None => crate::entities::page_canvas::PageFill::None,
                            Value::Color(color) => {
                                crate::entities::page_canvas::PageFill::Paint(
                                    color.into(),
                                )
                            }
                            Value::Gradient(gradient) => {
                                crate::entities::page_canvas::PageFill::Paint(
                                    gradient.into(),
                                )
                            }
                            Value::Tiling(tiling) => {
                                crate::entities::page_canvas::PageFill::Paint(
                                    (*tiling).clone().into(),
                                )
                            }
                            other => {
                                return Err(vec![type_mismatch(
                                    "auto, none, or paint",
                                    &other,
                                    span,
                                )])
                            }
                        });
                    }
                    "background" => {
                        background = Some(match val {
                            Value::None => None,
                            Value::Content(content) => Some(std::sync::Arc::new(content)),
                            other => {
                                return Err(vec![type_mismatch(
                                    "content or none",
                                    &other,
                                    span,
                                )])
                            }
                        });
                    }
                    "foreground" => {
                        foreground = Some(match val {
                            Value::None => None,
                            Value::Content(content) => Some(std::sync::Arc::new(content)),
                            other => {
                                return Err(vec![type_mismatch(
                                    "content or none",
                                    &other,
                                    span,
                                )])
                            }
                        });
                    }
                    _ => {
                        return Err(vec![SourceDiagnostic::error(
                            named.span(),
                            format!("unknown page property: {key}"),
                        )])
                    }
                }
            }
        }

        // Empurrar as dimensões da página para a StyleChain para que o layout() possa ler
        if let Some(w) = width {
            let v = match w {
                crate::entities::layout_types::PageDimension::Auto => Value::Auto,
                crate::entities::layout_types::PageDimension::Length(x) => {
                    Value::Float(x)
                }
            };
            *engine.styles = engine.styles.push_custom("page.width", v);
        }
        if let Some(h) = height {
            let v = match h {
                crate::entities::layout_types::PageDimension::Auto => Value::Auto,
                crate::entities::layout_types::PageDimension::Length(x) => {
                    Value::Float(x)
                }
            };
            *engine.styles = engine.styles.push_custom("page.height", v);
        }
        if let Some(ml) = margin_left {
            *engine.styles =
                engine.styles.push_custom("page.margin-left", Value::Float(ml));
        }
        if let Some(mr) = margin_right {
            *engine.styles =
                engine.styles.push_custom("page.margin-right", Value::Float(mr));
        }
        if let Some(mt) = margin_top {
            *engine.styles =
                engine.styles.push_custom("page.margin-top", Value::Float(mt));
        }
        if let Some(mb) = margin_bottom {
            *engine.styles =
                engine.styles.push_custom("page.margin-bottom", Value::Float(mb));
        }

        return Ok(Value::Content(Content::SetPage {
            paper,
            flipped,
            binding,
            width,
            height,
            margin,
            numbering,
            number_align,
            header,
            header_ascent,
            footer,
            footer_descent,
            supplement,
            columns,
            bleed,
            fill,
            background,
            foreground,
        }));
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
                    let val = eval_expr(named.expr(), scopes, ctx, engine)?;
                    let span = named.expr().span();
                    match val {
                        Value::Str(s) => {
                            *engine.styles = engine
                                .styles
                                .push_custom("figure.numbering", Value::Str(s));
                        }
                        Value::None => {
                            // Limpa a numeração no escopo (Value::None = ausente).
                            *engine.styles = engine
                                .styles
                                .push_custom("figure.numbering", Value::None);
                        }
                        other => {
                            return Err(vec![type_mismatch(
                                "string or none",
                                &other,
                                span,
                            )]);
                        }
                    }
                }
            }
        }
        return Ok(Value::None);
    }

    if target == "table" {
        // P459 — `#set table(numbering: "1.")` activa numeração automática de
        // tables. O padrão vive na chain léxica (`custom("table.numbering")`),
        // análogo a figure/equation (F-2 S3). O layout lê o gate e prefixa o
        // caption quando ambos (caption + numbering) estão presentes.
        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                if named.name().as_str() == "numbering" {
                    let val = eval_expr(named.expr(), scopes, ctx, engine)?;
                    let span = named.expr().span();
                    match val {
                        Value::Str(s) => {
                            *engine.styles = engine
                                .styles
                                .push_custom("table.numbering", Value::Str(s));
                        }
                        Value::None => {
                            *engine.styles =
                                engine.styles.push_custom("table.numbering", Value::None);
                        }
                        other => {
                            return Err(vec![type_mismatch(
                                "string or none",
                                &other,
                                span,
                            )]);
                        }
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
                            *engine.styles = engine
                                .styles
                                .push_custom("par.leading", Value::Length(l));
                        }
                    }
                    "spacing" => {
                        if let Value::Length(l) = val {
                            *engine.styles = engine
                                .styles
                                .push_custom("par.spacing", Value::Length(l));
                        }
                    }
                    _ => {
                        let (msg, hint) = unsupported_property_warn("par", &key, None);
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

    if target == "smartquote" {
        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                let key = named.name().as_str();
                let val = eval_expr(named.expr(), scopes, ctx, engine)?;
                let span = named.expr().span();
                match key {
                    "enabled" => match val {
                        Value::Bool(b) => {
                            *engine.styles = engine
                                .styles
                                .push_custom("smartquote.enabled", Value::Bool(b));
                        }
                        other => {
                            return Err(vec![type_mismatch("bool", &other, span)]);
                        }
                    },
                    "quotes" => match val {
                        Value::Str(s) => {
                            let char_count = s.chars().count();
                            if char_count != 2 {
                                return Err(vec![SourceDiagnostic::error(
                                    span,
                                    format!(
                                        "expected 2 characters, found {} characters",
                                        char_count
                                    ),
                                )]);
                            }
                            *engine.styles = engine
                                .styles
                                .push_custom("smartquote.quotes", Value::Str(s));
                        }
                        Value::None | Value::Auto => {
                            *engine.styles = engine
                                .styles
                                .push_custom("smartquote.quotes", Value::None);
                        }
                        other => {
                            return Err(vec![type_mismatch(
                                "string, auto or none",
                                &other,
                                span,
                            )]);
                        }
                    },
                    _ => {
                        let (msg, hint) =
                            unsupported_property_warn("smartquote", key, None);
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
                let val =
                    eval_expr(named.expr(), scopes, ctx, engine).unwrap_or(Value::None);
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
                // P665: `bold` e `italic` como argumentos nomeados de `#set text`
                // não existem no vanilla; o vanilla usa `weight: "bold"` e
                // `style: "italic"`. Rejeitar para manter a linguagem alinhada.
                "bold" | "italic" => {
                    return Err(vec![SourceDiagnostic::error(
                        named.name().to_untyped().span(),
                        format!("unexpected argument: {}", key),
                    )]);
                }
                "size" => {
                    // P816 (achado #3c de P810): tipo errado deixa de ser
                    // ignorado em silêncio — erro hard `expected length,
                    // found {type}` (+ hint para Int), paridade vanilla
                    // (`foundations/cast.rs:325-343`).
                    let span = named.expr().span();
                    match val {
                        Value::Length(l) => {
                            *engine.styles =
                                engine.styles.push_custom("text.size", Value::Length(l));
                        }
                        other => {
                            return Err(vec![expected_length_error(&other, span)]);
                        }
                    }
                }
                "fill" => {
                    if let Value::Color(c) = val {
                        *engine.styles =
                            engine.styles.push_custom("text.fill", Value::Color(c));
                    }
                }
                "weight" => {
                    // Int direto ou nome simbólico (FontWeight::from_name) → u16
                    // canónico em Value::Int. P636: rejeitar tipos inválidos.
                    let span = named.expr().span();
                    match &val {
                        Value::Int(n) => {
                            if let Ok(w) = u16::try_from(*n) {
                                *engine.styles = engine
                                    .styles
                                    .push_custom("text.weight", Value::Int(w as i64));
                            } else {
                                return Err(vec![SourceDiagnostic::error(
                                    span,
                                    "font weight must be between 100 and 900".to_string(),
                                )]);
                            }
                        }
                        Value::Str(s) => {
                            if let Some(fw) = FontWeight::from_name(s.as_str()) {
                                *engine.styles = engine.styles.push_custom(
                                    "text.weight",
                                    Value::Int(fw.to_number() as i64),
                                );
                            } else {
                                return Err(vec![SourceDiagnostic::error(
                                    span,
                                    format!("unknown font weight name: {s}"),
                                )]);
                            }
                        }
                        Value::None => {}
                        other => {
                            return Err(vec![type_mismatch(
                                "int or string",
                                other,
                                span,
                            )]);
                        }
                    }
                }
                "style" => {
                    // P665: `style` é a forma canónica do vanilla para itálico.
                    // Aceita os três valores standard; guarda em `text.style`.
                    let span = named.expr().span();
                    if let Value::Str(s) = &val {
                        match s.as_str() {
                            "normal" | "italic" | "oblique" => {
                                *engine.styles = engine
                                    .styles
                                    .push_custom("text.style", Value::Str(s.clone()));
                            }
                            _ => {
                                return Err(vec![SourceDiagnostic::error(
                                    span,
                                    format!("unknown font style name: {s}"),
                                )]);
                            }
                        }
                    } else {
                        return Err(vec![type_mismatch("string", &val, span)]);
                    }
                }
                "variations" => {
                    // P836 (achado #21 de P831): campo `#[fold] #[ghost]`
                    // settable no vanilla (`text/mod.rs:846-850`). Validação
                    // partilhada com o constructor — mensagens/hints verbatim
                    // do vanilla (`variations.rs:217-236`, `tag.rs:85-117`).
                    let span = named.expr().span();
                    crate::entities::font_variations::FontVariations::from_value(
                        &val, span,
                    )?;
                    *engine.styles = engine.styles.push_custom("text.variations", val);
                }
                "tracking" => {
                    // P816: mesma validação de tipo de `size` (Length no
                    // vanilla, `text/mod.rs:333`).
                    let span = named.expr().span();
                    match val {
                        Value::Length(l) => {
                            *engine.styles = engine
                                .styles
                                .push_custom("text.tracking", Value::Length(l));
                        }
                        other => {
                            return Err(vec![expected_length_error(&other, span)]);
                        }
                    }
                }
                "lang" => {
                    // ADR-0052: valida via `Lang::from_str` (erro hard em
                    // inválido); guarda o código canónico (`Value::Str`).
                    if let Value::Str(s) = val {
                        match Lang::from_str(&s) {
                            Ok(lang) => {
                                *engine.styles = engine.styles.push_custom(
                                    "text.lang",
                                    Value::Str(lang.as_str().into()),
                                );
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
                        Expr::Str(node) => vec![Value::Str(EcoString::from(node.get()?))],
                        Expr::Array(arr_node) => {
                            let mut items = Vec::new();
                            for item in arr_node.items() {
                                if let ArrayItem::Pos(expr) = item {
                                    if let Expr::Str(node) = expr {
                                        items.push(Value::Str(EcoString::from(
                                            node.get()?,
                                        )));
                                    } else {
                                        return Err(vec![SourceDiagnostic::error(
                                            span,
                                            "font array must contain only strings"
                                                .to_string(),
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
                            const NAMED_FIELDS: &[&str] =
                                &["family", "variant", "weight", "style", "fallback"];
                            let is_named_fields = dict_node.items().any(|item| {
                                matches!(
                                    item,
                                    DictItem::Named(named_item)
                                        if NAMED_FIELDS.contains(&named_item.name().as_str())
                                )
                            });

                            if is_named_fields {
                                font_dict::parse_font_dict_named_fields(
                                    dict_node, span, scopes, ctx, engine,
                                )?
                            } else {
                                font_dict::parse_font_dict_legacy(
                                    dict_node, span, scopes, ctx, engine,
                                )?
                            }
                        }
                        _ => {
                            return Err(vec![SourceDiagnostic::error(
                                span,
                                "font expects a string, array of strings, or dict"
                                    .to_string(),
                            )]);
                        }
                    };
                    // P816 (achado #3b de P810): warning `unknown font
                    // family` para nomes literais ausentes do FontBook —
                    // paridade vanilla (`check_font_list`,
                    // `text/mod.rs:1577-1588`, chamado no parse do arg
                    // `font` em `text/mod.rs:170-176`). Nomes em regex
                    // (dict `Value::Regex`) não são verificáveis — tal
                    // como no vanilla, que só avisa sobre `FontFamily`
                    // literais. Nome impresso lowercased (normalização
                    // de `FontFamily::new` do vanilla).
                    let missing_families: Vec<String> = {
                        let book = engine.world.book();
                        arr.iter()
                            .filter_map(|item| match item {
                                Value::Str(s) => Some(s.as_str()),
                                Value::Dict(d) => match d.get("name") {
                                    Some(Value::Str(s)) => Some(s.as_str()),
                                    _ => None,
                                },
                                _ => None, // neutro: N16[β] — valores não-nominais filtrados em extracção de famílias tipográficas
                            })
                            .filter(|name| book.select_family(name).next().is_none())
                            .map(|name| name.to_lowercase())
                            .collect()
                    };
                    for family in missing_families {
                        engine.sink.warn_note(
                            span,
                            &format!("unknown font family: {family}"),
                            "",
                        );
                    }
                    *engine.styles =
                        engine.styles.push_custom("text.font", Value::Array(arr));
                }
                "top-edge" => {
                    // P837 (achados #22/#23 de P831) — paridade do cast
                    // `TopEdge` do vanilla (`text/mod.rs:1169-1177`):
                    // métrica do domínio enumerado ou `Length`; o resto é
                    // erro hard verbatim.
                    match val {
                        Value::Str(s) if TOP_EDGE_METRICS.contains(&s.as_str()) => {
                            *engine.styles =
                                engine.styles.push_custom("text.top-edge", Value::Str(s));
                        }
                        Value::Length(l) => {
                            *engine.styles = engine
                                .styles
                                .push_custom("text.top-edge", Value::Length(l));
                        }
                        other => {
                            return Err(vec![edge_cast_error(
                                true,
                                &other,
                                named.expr().span(),
                            )]);
                        }
                    }
                }
                "bottom-edge" => {
                    // P837 — idem, cast `BottomEdge` (`text/mod.rs:1217-1225`).
                    match val {
                        Value::Str(s) if BOTTOM_EDGE_METRICS.contains(&s.as_str()) => {
                            *engine.styles = engine
                                .styles
                                .push_custom("text.bottom-edge", Value::Str(s));
                        }
                        Value::Length(l) => {
                            *engine.styles = engine
                                .styles
                                .push_custom("text.bottom-edge", Value::Length(l));
                        }
                        other => {
                            return Err(vec![edge_cast_error(
                                false,
                                &other,
                                named.expr().span(),
                            )]);
                        }
                    }
                }
                "dir" => {
                    if let Value::Dir(dir) = val {
                        if dir.is_vertical() {
                            return Err(vec![SourceDiagnostic::error(
                                named.expr().span(),
                                "text direction must be horizontal".to_string(),
                            )]);
                        }
                        *engine.styles =
                            engine.styles.push_custom("text.dir", Value::Dir(dir));
                    }
                }
                _ => {
                    // P816 (achado #3a de P810): nome fora da lista de
                    // propriedades settable do `TextElem` vanilla é erro
                    // hard `unexpected argument: {name}` (paridade
                    // `foundations/args.rs:262`, exit 1). Nomes válidos no
                    // vanilla mas ainda não capturados (ex.: `hyphenate`,
                    // `stroke`, `baseline`) mantêm o warning de scope-out
                    // do Passo 107 (encerra DEBT-49, hint ADR-0040).
                    if !VANILLA_TEXT_SET_PROPS.contains(&key.as_str()) {
                        return Err(vec![SourceDiagnostic::error(
                            named.name().to_untyped().span(),
                            format!("unexpected argument: {}", key),
                        )]);
                    }
                    let (msg, hint) =
                        unsupported_property_warn("text", &key, Some("0040"));
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
    // P790 — `page`/`par` como alvo de show: identificadores especiais
    // reconhecidos ANTES de avaliar o selector (no vanilla são element
    // functions em scope; na stdlib cristalina não existem como variáveis —
    // caíam em `unknown variable` fatal, achado P786). Paridade medida por
    // execução (`typst-eval/src/rules.rs:67-95`):
    // - `#show page: <qualquer transformação>` → warning específico, nenhuma
    //   regra registada, compilação prossegue (a regra não tem efeito no
    //   vanilla).
    // - `#show par: set block(...)` com `spacing`/`above`/`below` → warning
    //   específico, nenhuma regra registada (vanilla condiciona a
    //   `BlockElem::above`/`below` na chain).
    // - `#show par: <outra transformação>` → erro explícito (scope-out
    //   documentado no L0, eval.md §P790: no vanilla `show par` é regra viva
    //   sobre `ParElem`; element rule de par é candidata a passo futuro).
    // P863: selector especial `par` pode ser resolvido antes de avaliar a
    // expressão (não existe como variável na stdlib cristalina).
    let mut preselected: Option<Selector> = None;
    if let Some(Expr::Ident(ident)) = show_rule.selector() {
        let rule_span = show_rule.to_untyped().span();
        match ident.as_str() {
            "page" => {
                engine.sink.warn_note(
                    rule_span,
                    "`show page` is not supported and has no effect",
                    "customize pages with `set page(..)` instead",
                );
                return Ok(Value::None);
            }
            "par" => {
                let is_set_block_spacing = matches!(
                    show_rule.transform(),
                    Expr::SetRule(set)
                        if set.target().to_untyped().text_str() == "block"
                            && set.args().items().any(|arg| matches!(
                                arg,
                                Arg::Named(named)
                                    if matches!(
                                        named.name().as_str(),
                                        "spacing" | "above" | "below"
                                    )
                            ))
                );
                if is_set_block_spacing {
                    engine.sink.warn_note2(
                        rule_span,
                        "`show par: set block(spacing: ..)` has no effect anymore",
                        "write `set par(spacing: ..)` instead",
                        "this is specific to paragraphs as they are not considered \
                         blocks anymore",
                    );
                    return Ok(Value::None);
                }
                preselected = Some(Selector::NodeKind(NodeKind::Par));
            }
            _ => {}
        }
    }

    // Avaliar o selector — pode ser uma string ou uma função da stdlib.
    // `selector()` retorna `Option<Expr>` — None significa selector omitido (não suportado).
    let selector = if let Some(sel) = preselected {
        sel
    } else {
        match show_rule.selector() {
            None => {
                return Err(vec![SourceDiagnostic::error(
                    show_rule.to_untyped().span(),
                    "show rule requer um selector".to_string(),
                )])
            }
            Some(sel_expr) => {
                let selector_val = eval_expr(sel_expr, scopes, ctx, engine)?;
                match selector_val {
                    Value::Str(s) => {
                        // P790 — paridade vanilla `selector.rs:110` (medido por
                        // execução): selector de texto vazio é erro, não no-op.
                        if s.is_empty() {
                            return Err(vec![SourceDiagnostic::error(
                                sel_expr.span(),
                                "text selector is empty".to_string(),
                            )]);
                        }
                        Selector::Text(s.to_string())
                    }
                    // P393: regex(pattern) → selector regex sobre texto.
                    Value::Regex(re) => Selector::Regex(re),
                    // P791 — `<lbl>` → selector por label (paridade vanilla
                    // `Selector::Label`). Aplicação dedicada em
                    // `intercept_labelled` (não viaja pela travessia principal).
                    Value::Label(l) => Selector::Label(l),
                    // **P417 (M)** — Selector como valor de primeira classe
                    // (`heading.where(level: 1)`). Converte do selector de query
                    // para o selector de show rule.
                    Value::Selector(sel) => {
                        selector_matching::query_selector_to_show_selector(
                            sel,
                            sel_expr.span(),
                        )?
                    }
                    // Lote F-3 inc-2: elemento de utilizador (fronteira E1) — o
                    // selector `callout` resolve para uma `FuncRepr::Element`, que
                    // não tem fn-ptr nativo. Casa por **kind dinâmico** (nome).
                    Value::Func(ref f) if f.element_name().is_some() => {
                        Selector::DynKind(f.element_name().unwrap().to_string())
                    }
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
                        use crate::compiler::stdlib::{
                            native_emph, native_enum, native_figure, native_footnote,
                            native_heading, native_link, native_list, native_overline,
                            native_par, native_quote, native_raw, native_smallcaps,
                            native_strike, native_strong, native_subscript,
                            native_superscript, native_underline,
                        };
                        use std::ptr::fn_addr_eq;
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
                        Some(addr) if fn_addr_eq(addr, native_underline as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Underline),
                        Some(addr) if fn_addr_eq(addr, native_strike as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Strike),
                        Some(addr) if fn_addr_eq(addr, native_overline as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Overline),
                        Some(addr) if fn_addr_eq(addr, native_smallcaps as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Smallcaps),
                        Some(addr) if fn_addr_eq(addr, native_subscript as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Subscript),
                        Some(addr) if fn_addr_eq(addr, native_superscript as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Superscript),
                        // P494 — selectores de elementos de documento.
                        Some(addr) if fn_addr_eq(addr, native_link as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Link),
                        Some(addr) if fn_addr_eq(addr, native_quote as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Quote),
                        Some(addr) if fn_addr_eq(addr, native_footnote as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Footnote),
                        Some(addr) if fn_addr_eq(addr, native_list as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::List),
                        Some(addr) if fn_addr_eq(addr, native_enum as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Enum),
                        // P863 — `par` como função nativa mapeia para nó de parágrafo.
                        Some(addr) if fn_addr_eq(addr, native_par as fn(_, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Par),
                        Some(_) => return Err(vec![SourceDiagnostic::error(
                            sel_expr.span(),
                            format!(
                                "função '{}' não é um tipo de nó suportado como selector. \
                                 Tipos suportados: heading, figure, strong, emph, raw, \
                                 underline, strike, overline, smallcaps, sub, super, \
                                 link, quote, list, enum, par.",
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
                    }
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            sel_expr.span(),
                            format!(
                                "selector inválido para show rule: {}",
                                other.type_name()
                            ),
                        )])
                    }
                }
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
                     texto — use uma função ou Content"
                        .to_string(),
                )]);
            }
            Transformation::Style(capture_set_styles(set, scopes, ctx, engine)?)
        }
        expr => {
            let value = eval_expr(expr, scopes, ctx, engine)?;
            match value {
                Value::Func(f) => Transformation::Func(f),
                Value::Content(c) => Transformation::Content(c),
                Value::Str(s) => Transformation::Str(s),
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        expr.span(),
                        format!(
                            "transformação de show rule inválida: esperado função, \
                         Content, string ou set rule, recebeu {}",
                            other.type_name()
                        ),
                    )])
                }
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
