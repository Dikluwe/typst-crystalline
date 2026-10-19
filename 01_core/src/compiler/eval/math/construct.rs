//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/math/construct.md
//! @prompt-hash 13ada8d3
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Aplica o binding público do módulo `math` também no caminho de sintaxe.
/// Assim a sintaxe não mantém uma segunda validação nem uma segunda regra de
/// construção para os constructors P1293.
pub(super) fn apply_p1293_math_constructor(
    name: &str,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    let func = match scopes.get("math") {
        Some(Value::Module(module)) => match module.scope().get(name) {
            Some(Value::Func(func)) => func.clone(),
            _ => unreachable!("P1293 binding ausente em math.{name}"),
        },
        _ => unreachable!("módulo math ausente durante avaliação math"),
    };
    match apply_func(func, args, scopes, ctx, engine)? {
        Value::Content(content) => Ok(content),
        _ => unreachable!("constructor math.{name} devolveu valor não-content"),
    }
}
