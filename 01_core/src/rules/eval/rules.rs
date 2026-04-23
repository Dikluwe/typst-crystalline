//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash 19073424
//! @layer L1
//! @updated 2026-04-22
//!
//! Show rules e set rules — aplicação e intercepção. Extraído de `eval.rs`
//! no Passo 96.1 conforme ADR-0037 (coesão por domínio).

use std::sync::Arc;

use crate::entities::args::Args;
use crate::entities::ast::AstNode;
use crate::entities::ast::code::{SetRule, ShowRule as ShowRuleNode};
use crate::entities::ast::expr::Arg;
use crate::entities::content::Content;
use crate::entities::engine::Engine;
use crate::entities::show::{NodeKind, Selector, ShowRule};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::style_chain::StyleDelta;
use crate::entities::value::Value;
use crate::entities::world_types::check_show_depth as route_check_show_depth;
use crate::rules::scopes::Scopes;

use super::{closures, eval_expr, EvalContext};

/// Helper partilhado para construir warning de propriedade não suportada
/// em `#set` (Passo 107, encerra DEBT-49). Formato consistente para o
/// utilizador final; referencia ADR-0040 como catálogo vivo.
fn unsupported_property_warn(target: &str, field: &str) -> (String, String) {
    (
        format!("{target}: propriedade '{field}' ainda não suportada"),
        format!("ver ADR-0040 para propriedades cobertas por set {target}"),
    )
}

/// Helper para warning de `#set` com target desconhecido (Passo 107).
fn unsupported_target_warn(target: &str) -> (String, String) {
    (
        format!("set: target '{target}' ainda não suportado"),
        "targets suportados: heading, page, figure, text".to_string(),
    )
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

    // Limite de aninhamento vanilla (MAX_SHOW_RULE_DEPTH = 64) —
    // paridade com `typst-realize/src/lib.rs:402` (ADR-0033).
    // Pago parcial do DEBT-45 no Passo 93.
    route_check_show_depth(engine.route)?;

    // Separar regras por tipo para travessias distintas.
    let has_node_rules = rules.iter().any(|r| matches!(r.selector, Selector::NodeKind(_)));

    if has_node_rules {
        // Única travessia para todas as NodeKind rules.
        let node_rules: Vec<ShowRule> = rules.iter()
            .filter(|r| matches!(r.selector, Selector::NodeKind(_)))
            .cloned()
            .collect();

        let mut apply_all = |node: &Content| -> SourceResult<Option<Content>> {
            for rule in &node_rules {
                // Saltar se esta regra está actualmente em execução (anti-recursão).
                if engine.active_guards.contains(&rule.id) {
                    continue;
                }

                let Selector::NodeKind(ref kind) = rule.selector else { continue };

                // Passo 101: `Content::Strong`/`Content::Emph` removidos do enum.
                // `show strong: it => ...` e `show emph: it => ...` passam a
                // casar `Content::Styled` que contenha `Style::Bold(true)` ou
                // `Style::Italic(true)` respectivamente.
                use crate::entities::style::Style;
                let is_bold_styled = matches!(node, Content::Styled(_, ss)
                    if ss.iter().any(|s| matches!(s, Style::Bold(true))));
                let is_italic_styled = matches!(node, Content::Styled(_, ss)
                    if ss.iter().any(|s| matches!(s, Style::Italic(true))));

                let is_match = matches!(
                    (node, kind),
                    (Content::Heading { .. },  NodeKind::Heading)
                    | (Content::Figure { .. },   NodeKind::Figure)
                    | (Content::Raw { .. },      NodeKind::Raw)
                    | (Content::Equation { .. }, NodeKind::Equation)
                    | (Content::ListItem(_),     NodeKind::ListItem)
                ) || (matches!(kind, NodeKind::Strong) && is_bold_styled)
                  || (matches!(kind, NodeKind::Emph)   && is_italic_styled);

                if !is_match {
                    continue;
                }

                match &rule.transform {
                    Value::Func(func) => {
                        let args = Args::positional(vec![Value::Content(node.clone())]);
                        engine.active_guards.push(rule.id);
                        let call_result = closures::apply_func(func.clone(), args, ctx, engine);
                        engine.active_guards.pop();
                        return match call_result? {
                            Value::Content(c) => Ok(Some(c)),
                            Value::Str(s)     => Ok(Some(Content::text(s.as_str()))),
                            other => Err(vec![SourceDiagnostic::error(
                                Span::detached(),
                                format!(
                                    "show rule deve retornar Content ou String, \
                                     recebeu {}",
                                    other.type_name()
                                ),
                            )]),
                        };
                    },
                    Value::Content(c) => return Ok(Some(c.clone())),
                    other => return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!(
                            "show rule com selector de tipo requer função ou Content, \
                             recebeu {}",
                            other.type_name()
                        ),
                    )]),
                }
            }
            Ok(None)
        };

        content = content.map_content(&mut apply_all)?;
    }

    // Text rules — map_text por padrão, na ordem de declaração.
    for rule in rules {
        if let Selector::Text(pattern) = &rule.selector {
            if let Value::Str(s) = &rule.transform {
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
        return Ok(Value::Content(Content::SetHeadingNumbering { active }));
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
        // #set figure(numbering: "1") — activa numeração automática de figuras (Passo 75, DEBT-14).
        // Passo 109 (ADR-0044): `figure_numbering` agora é campo de `Engine`.
        let mut new_numbering = engine.figure_numbering.clone();
        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                if named.name().as_str() == "numbering" {
                    let val = eval_expr(named.expr(), scopes, ctx, engine).unwrap_or(Value::None);
                    new_numbering = match val {
                        Value::Str(s) => Some(s.to_string()),
                        Value::None   => None,
                        _             => new_numbering.clone(),
                    };
                }
            }
        }
        *engine.figure_numbering = new_numbering.clone();
        return Ok(Value::Content(Content::SetFigureNumbering {
            pattern: new_numbering.unwrap_or_default(),
        }));
    }

    if target != "text" {
        let (msg, hint) = unsupported_target_warn(&target);
        engine.sink.warn_note(target_span, &msg, &hint);
        return Ok(Value::None);
    }

    let mut delta = StyleDelta::empty();

    for arg in set.args().items() {
        if let Arg::Named(named) = arg {
            let key = named.name().as_str().to_owned();
            let val = eval_expr(named.expr(), scopes, ctx, engine)?;
            match key.as_str() {
                "bold" => {
                    if let Value::Bool(b) = val { delta.bold = Some(b); }
                }
                "italic" => {
                    if let Value::Bool(b) = val { delta.italic = Some(b); }
                }
                "size" => {
                    if let Value::Length(l) = val {
                        delta.size = Some(l.abs.to_pt());
                    }
                }
                "fill" => {
                    // Passo 102 (ADR-0040): activar `#set text(fill: color)`.
                    // Captura `Value::Color` em `StyleDelta.fill`; propaga para
                    // `TextStyle.fill` via `TextStyle::from(&StyleChain)`.
                    if let Value::Color(c) = val {
                        delta.fill = Some(c);
                    }
                }
                _ => {
                    // Passo 107 (encerra DEBT-49): propriedades não suportadas
                    // de `#set text(...)` emitem warning via Sink em vez de
                    // serem silenciadas. O span do argumento permite ao
                    // utilizador localizar a propriedade exacta.
                    let (msg, hint) = unsupported_property_warn("text", &key);
                    engine.sink.warn_note(named.name().to_untyped().span(), &msg, &hint);
                }
            }
        }
    }

    // Mutação persistente do styles propagado (Passo 94; agora via Engine).
    *engine.styles = engine.styles.push(delta);
    Ok(Value::None)
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
                        Some(addr) if fn_addr_eq(addr, native_heading as fn(_, _, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Heading),
                        Some(addr) if fn_addr_eq(addr, native_figure as fn(_, _, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Figure),
                        Some(addr) if fn_addr_eq(addr, native_strong as fn(_, _, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Strong),
                        Some(addr) if fn_addr_eq(addr, native_emph as fn(_, _, _, _, _) -> _) =>
                            Selector::NodeKind(NodeKind::Emph),
                        Some(addr) if fn_addr_eq(addr, native_raw as fn(_, _, _, _, _) -> _) =>
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

    // Avaliar a transformação (closure ou valor estático).
    let transform = eval_expr(show_rule.transform(), scopes, ctx, engine)?;
    let id = ctx.next_rule_id;
    ctx.next_rule_id += 1;
    // Reconstruir o slice com a nova regra (Passo 95 + 109: show_rules
    // é campo do Engine, mutação local via &mut).
    let mut rules = engine.show_rules.to_vec();
    rules.push(ShowRule { id, selector, transform });
    *engine.show_rules = Arc::from(rules);
    Ok(Value::None)
}
