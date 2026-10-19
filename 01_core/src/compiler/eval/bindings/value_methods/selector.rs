//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/bindings/value_methods/selector.md
//! @prompt-hash 91f91c61
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// **P423 (S-M)** — Tenta avaliar `<selector>.or(other)` ou
/// `<selector>.and(other)`.
///
/// Retorna `Ok(Some(Selector::Or(...)))` ou `Ok(Some(Selector::And(...)))`
/// se o target avaliar para `Value::Selector` e houver exactamente um
/// argumento posicional que também avalie para `Value::Selector`.
/// Retorna `Ok(None)` se o target não for um selector (deixa o caller
/// continuar com field access normal).
/// Converte `Value::Func` nativo de elemento (heading, figure) ou
/// `Value::Selector` num `Selector` de query. Retorna `None` se o valor não
/// for convertível.
pub(in crate::compiler::eval) fn value_to_query_selector(
    value: &Value,
) -> Option<Selector> {
    match value {
        Value::Selector(s) => Some(s.clone()),
        Value::Func(f) => {
            use crate::compiler::stdlib::{native_figure, native_heading};
            use std::ptr::fn_addr_eq;
            f.native_fn_addr().and_then(|addr| {
                if fn_addr_eq(addr, native_heading as fn(_, _, _, _) -> _) {
                    Some(Selector::Kind(ElementKind::Heading))
                } else if fn_addr_eq(addr, native_figure as fn(_, _, _, _) -> _) {
                    Some(Selector::Kind(ElementKind::Figure))
                } else {
                    None
                }
            })
        }
        _ => None,
    }
}

pub(in crate::compiler::eval) fn eval_selector_or_and<'a>(
    target: &Value,
    method: &str,
    args_node: crate::entities::ast::expr::Args<'a>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Option<Selector>> {
    let Some(base) = value_to_query_selector(target) else {
        return Ok(None);
    };

    let args =
        crate::compiler::eval::call_dispatch::eval_args(args_node, scopes, ctx, engine)?;
    if !args.named.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args_node.span(),
            "unexpected argument",
        )]);
    }
    let mut selectors = vec![base];
    for other in args.items {
        let Some(other_sel) = value_to_query_selector(&other) else {
            return Err(vec![SourceDiagnostic::error(
                args_node.span(),
                format!(
                    "selector.{}() espera um selector, recebeu {}",
                    method,
                    other.type_name()
                ),
            )]);
        };
        selectors.push(other_sel);
    }

    match method {
        "or" => Ok(Some(Selector::Or(EcoVec::from(selectors)))),
        "and" => Ok(Some(Selector::And(EcoVec::from(selectors)))),
        _ => Ok(None),
    }
}

/// **P504** — Intercepta `selector(base).within(ancestor)`.
pub(in crate::compiler::eval) fn eval_selector_within<'a>(
    target: &Value,
    args_node: crate::entities::ast::expr::Args<'a>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Option<Selector>> {
    let Some(base) = value_to_query_selector(target) else {
        return Ok(None);
    };

    let args =
        crate::compiler::eval::call_dispatch::eval_args(args_node, scopes, ctx, engine)?;
    let other = args.items.into_iter().next().ok_or_else(|| {
        vec![SourceDiagnostic::error(
            args_node.span(),
            "selector.within() requer um argumento posicional".to_string(),
        )]
    })?;
    let Some(ancestor) = value_to_query_selector(&other) else {
        return Err(vec![SourceDiagnostic::error(
            args_node.span(),
            format!(
                "selector.within() espera um selector, recebeu {}",
                other.type_name()
            ),
        )]);
    };

    Ok(Some(Selector::Within { base: Box::new(base), ancestor: Box::new(ancestor) }))
}

/// **P1401** — Intercepta os métodos contextuais de selector.
pub(in crate::compiler::eval) fn eval_selector_before_after<'a>(
    target: &Value,
    method: &str,
    args_node: crate::entities::ast::expr::Args<'a>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Option<Selector>> {
    let Some(base) = value_to_query_selector(target) else {
        return Ok(None);
    };
    let args =
        crate::compiler::eval::call_dispatch::eval_args(args_node, scopes, ctx, engine)?;
    crate::compiler::eval::call_dispatch::selector_before_after(base, method, args)
        .map(Some)
}

// ── Dispatcher arms: FieldAccess (Passo 96.2, ADR-0037 Regra 4) ───────────
