//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/bindings/value_methods/state.md
//! @prompt-hash 708ec9bf
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// **P506** — Despacha métodos de `Value::State`: `.update()`, `.get()`,
/// `.display()`.
pub(in crate::compiler::eval) fn eval_state_method(
    state: &State,
    method: &str,
    args: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    use crate::compiler::eval::call_dispatch::eval_args;
    let span = args.span();
    match method {
        "update" => {
            let args = eval_args(args, scopes, ctx, engine)?;
            match args.items.as_slice() {
                [value] if args.named.is_empty() => {
                    Ok(crate::compiler::stdlib::state::state_method_update(
                        state.key.clone(),
                        value.clone(),
                    ))
                }
                _ => Err(vec![SourceDiagnostic::error(
                    span,
                    "state.update() requer 1 argumento".to_string(),
                )]),
            }
        }
        "get" => {
            let args = eval_args(args, scopes, ctx, engine)?;
            if !args.items.is_empty() || !args.named.is_empty() {
                return Err(vec![SourceDiagnostic::error(span, "unexpected argument")]);
            }
            state_get(state, ctx, engine, span)
        }
        "display" => {
            let args = eval_args(args, scopes, ctx, engine)?;
            state_display(state, &args, scopes, ctx, engine, span)
        }
        // **P844** (achado #49 de P831) — `.at()`/`.final()` existiam como
        // nativas (`native_state_at`/`native_state_final`, foundations.rs)
        // mas não estavam ligados no dispatch de métodos.
        "at" => state_at_dispatch(state, args, scopes, ctx, engine, span),
        "final" => {
            let args = eval_args(args, scopes, ctx, engine)?;
            // Mensagens verbatim medidas no vanilla 0.15.0:
            // `s.final(1)` → "unexpected argument".
            if !args.items.is_empty() || !args.named.is_empty() {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "unexpected argument".to_string(),
                )]);
            }
            state_final(state, ctx, engine, span)
        }
        _ => Err(vec![SourceDiagnostic::error(
            span,
            format!("state não tem método '{}'", method),
        )]),
    }
}

/// **P844** (achado #49 de P831) — Despacha `state.at(selector)`.
/// Aceita `Location` directa (ex.: `here()`) ou `<label>` resolvida via
/// introspector — paridade vanilla `State::at` (`LocatableSelector`).
/// Mensagens verbatim medidas no vanilla 0.15.0:
/// - `s.at()` → `missing argument: selector`
/// - `s.at(1)` → `expected label, function, location, or selector, found integer`
/// - `s.at("x")` → `text is not locatable`
/// - `s.at(<inexistente>)` → ``label `<inexistente>` does not exist in the document``
///
/// A validação de argumentos precede o gate de contexto (ordem medida:
/// `s.at(1)` fora de contexto → erro de tipo, não gate).
fn state_at_dispatch(
    state: &State,
    args: crate::entities::ast::expr::Args<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    span: Span,
) -> SourceResult<Value> {
    let mut items = args.items();
    let Some(first) = items.next() else {
        return Err(vec![SourceDiagnostic::error(
            span,
            "missing argument: selector".to_string(),
        )]);
    };
    if items.next().is_some() {
        return Err(vec![SourceDiagnostic::error(
            span,
            "unexpected argument".to_string(),
        )]);
    }

    // Resolve o selector para uma Location (label → lookup no introspector).
    let resolve_label = |label: &crate::entities::label::Label,
                         ctx: &EvalContext|
     -> SourceResult<crate::entities::location::Location> {
        resolve_state_label(label, ctx, span)
    };

    match first {
        Arg::Pos(Expr::Label(node)) => {
            let label = crate::entities::label::Label(node.get().to_string());
            let loc = resolve_label(&label, ctx)?;
            state_at_location(state, loc, ctx, engine, span)
        }
        Arg::Pos(expr) => {
            let value = eval_expr(expr, scopes, ctx, engine)?;
            match value {
                Value::Location(loc) => state_at_location(state, loc, ctx, engine, span),
                Value::Label(label) => {
                    let loc = resolve_label(&label, ctx)?;
                    state_at_location(state, loc, ctx, engine, span)
                }
                Value::Content(crate::entities::content::Content::Label(e)) => {
                    let label = crate::entities::label::Label(e.name.to_string());
                    let loc = resolve_label(&label, ctx)?;
                    state_at_location(state, loc, ctx, engine, span)
                }
                Value::Str(_) => Err(vec![SourceDiagnostic::error(
                    span,
                    "text is not locatable".to_string(),
                )]),
                other => Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                        "expected label, function, location, or selector, found {}",
                        vanilla_type_name(&other)
                    ),
                )]),
            }
        }
        Arg::Named(named) => Err(vec![SourceDiagnostic::error(
            named.span(),
            format!("unexpected argument: {}", named.name().as_str()),
        )]),
        Arg::Spread(spread) => Err(vec![SourceDiagnostic::error(
            spread.span(),
            "unexpected argument".to_string(),
        )]),
    }
}

pub(crate) fn resolve_state_label(
    label: &crate::entities::label::Label,
    ctx: &EvalContext,
    span: Span,
) -> SourceResult<crate::entities::location::Location> {
    use crate::entities::introspector::Introspector;
    let result = ctx
        .introspector
        .query_by_label(label)
        .map(Value::Location)
        .ok_or_else(|| {
            vec![SourceDiagnostic::error(
                span,
                format!("label `<{}>` does not exist in the document", label.0),
            )]
        });
    match ctx.observe_context_read(
        crate::compiler::eval::ContextReadRequest::StateResolveLabel {
            label: label.clone(),
        },
        span,
        result,
    )? {
        Value::Location(location) => Ok(location),
        _ => unreachable!("label resolution only produces locations"),
    }
}
