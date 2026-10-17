//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/call_dispatch/selector_intercepts.md
//! @prompt-hash b8744dd3
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) fn is_selector_logical(method: &str) -> bool {
    matches!(method, "or" | "and")
}

pub(super) fn try_selector_call(
    call: FuncCallNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    cached_receiver: &mut Option<Value>,
) -> SourceResult<Option<Value>> {
    // **P423 (S-M)** — Intercepção de `selector.or(other)` e
    // `selector.and(other)` antes de avaliar o callee genérico. O target e o
    // argumento devem avaliar para `Value::Selector`.
    if let Expr::FieldAccess(access) = call.callee() {
        let method = access.field().as_str();
        if is_selector_logical(method) {
            if let Some(selector) = bindings::eval_selector_or_and(
                super::call::resolve_receiver(
                    access,
                    cached_receiver,
                    scopes,
                    ctx,
                    engine,
                )?,
                method,
                call.args(),
                scopes,
                ctx,
                engine,
            )? {
                return Ok(Some(Value::Selector(selector)));
            }
        }
    }

    // **P504** — Intercepção de `selector.within(ancestor)`.
    if let Expr::FieldAccess(access) = call.callee() {
        if access.field().as_str() == "within" {
            if let Some(selector) = bindings::eval_selector_within(
                super::call::resolve_receiver(
                    access,
                    cached_receiver,
                    scopes,
                    ctx,
                    engine,
                )?,
                call.args(),
                scopes,
                ctx,
                engine,
            )? {
                return Ok(Some(Value::Selector(selector)));
            }
        }
    }

    // **P1401** — Intercepção de seletores contextuais.
    if let Expr::FieldAccess(access) = call.callee() {
        let method = access.field().as_str();
        if method == "before" || method == "after" {
            if let Some(value) = bindings::eval_selector_before_after(
                super::call::resolve_receiver(
                    access,
                    cached_receiver,
                    scopes,
                    ctx,
                    engine,
                )?,
                method,
                call.args(),
                scopes,
                ctx,
                engine,
            )? {
                return Ok(Some(Value::Selector(value)));
            }
        }
    }

    Ok(None)
}
