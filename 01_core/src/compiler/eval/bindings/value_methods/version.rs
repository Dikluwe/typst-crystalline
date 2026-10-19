//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/bindings/value_methods/version.md
//! @prompt-hash 772fdfb7
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// **P796** — Despacha `.at(index)`, o único método de instância de
/// `Value::Version` (`entities/version.md` §8a). Mesmo padrão de
/// `eval_counter_method_value`/`eval_color_method`: recebe os argumentos AST
/// não avaliados, avalia-os aqui.
pub(in crate::compiler::eval) fn eval_version_method_value(
    version: &crate::entities::version::Version,
    method: &str,
    args: crate::entities::ast::expr::Args<'_>,
    call_span: Span,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let mut args =
        crate::compiler::eval::call_dispatch::eval_args(args, scopes, ctx, engine)?;
    args.span = call_span;
    crate::compiler::stdlib::dispatch_version_method(
        version,
        method,
        args,
        ctx,
        engine.world,
        engine.current_file,
    )
}
