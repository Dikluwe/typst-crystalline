//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/call_dispatch/collection_methods.md
//! @prompt-hash 5a21ac20
//! @layer L1
//! @updated 2026-10-01

use super::*;

/// Intercepta métodos de coleção e aplicação parcial na ordem vigente.
pub(super) fn try_collection_method(
    call: FuncCallNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    cached_receiver: &mut Option<Value>,
) -> SourceResult<Option<Value>> {
    // **P466** — Métodos de instância para `array`, `dict` e `str`.
    if let Expr::FieldAccess(access) = call.callee() {
        let target = match cached_receiver.take() {
            Some(target) => target,
            None => eval_expr(access.target(), scopes, ctx, engine)?,
        };
        let method = access.field().as_str();
        bindings::check_module_field_feature(
            &target,
            method,
            access.field().span(),
            ctx.features,
        )?;
        let trace_arguments_callback =
            matches!(&target, Value::Args(_)) && matches!(method, "filter" | "map");
        let mut positional = Vec::new();
        let mut named: IndexMap<EcoString, Span, FxBuildHasher> = IndexMap::default();
        for arg in call.args().items() {
            match arg {
                Arg::Pos(expr) => positional.push(expr.span()),
                Arg::Named(named_arg) => {
                    named.insert(named_arg.name().as_str().into(), named_arg.span());
                }
                Arg::Spread(_) => {}
            }
        }
        let call_spans = CollectionCallSpans { call: call.span(), positional, named };
        match try_dispatch_collection_method_with(
            target,
            method,
            |scopes, ctx, engine| eval_args(call.args(), scopes, ctx, engine),
            Some(&call_spans),
            scopes,
            ctx,
            engine,
        ) {
            crate::compiler::stdlib::CollectionDispatch::Handled(result) => {
                return if trace_arguments_callback {
                    trace_call(result, Some(method), call.span(), engine).map(Some)
                } else {
                    result.map(Some)
                };
            }
            crate::compiler::stdlib::CollectionDispatch::Unhandled(target) => {
                *cached_receiver = Some(target);
            }
        }
    }

    Ok(None)
}
