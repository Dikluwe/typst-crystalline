//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/call_dispatch/collection_methods.md
//! @prompt-hash 867097e3
//! @layer L1
//! @updated 2026-10-01

use super::*;

/// Intercepta métodos de coleção e aplicação parcial na ordem vigente.
pub(super) fn try_collection_or_with(
    call: FuncCallNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Option<Value>> {
    // **P466** — Métodos de instância para `array`, `dict` e `str`.
    if let Expr::FieldAccess(access) = call.callee() {
        let target = eval_expr(access.target(), scopes, ctx, engine)?;
        let method = access.field().as_str();
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
        let args = eval_args(call.args(), scopes, ctx, engine)?;
        if let Some(result) = try_dispatch_collection_method(
            target,
            method,
            args,
            Some(&call_spans),
            scopes,
            ctx,
            engine,
        ) {
            return if trace_arguments_callback {
                trace_call(result, Some(method), call.span(), engine).map(Some)
            } else {
                result.map(Some)
            };
        }
    }

    // **P702** — `f.with(...)`: aplicação parcial de argumentos, disponível
    // em qualquer `Value::Func` (nativa com/sem namespace, closure, elemento,
    // plugin, ou já parcialmente aplicada). Mesmo padrão de intercepção das
    // secções acima; se o alvo não for `Value::Func`, não intercepta — cai
    // no field access genérico, sem mudança de comportamento para
    // não-funções (ver `entities/func.md` §"Variante `With`").
    if let Expr::FieldAccess(access) = call.callee() {
        if access.field().as_str() == "with" {
            let target = eval_expr(access.target(), scopes, ctx, engine)?;
            if let Value::Func(f) = target {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                return Ok(Some(Value::Func(f.with(args))));
            }
        }
    }

    Ok(None)
}
