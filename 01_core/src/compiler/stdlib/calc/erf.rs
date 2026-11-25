//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/calc/erf.md
//! @prompt-hash 457c0cf1
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Função erro de Gauss.
pub(crate) fn calc_erf(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => Ok(Value::Float(libm::erf(coerce_to_f64(v, "calc.erf()")?))),
        _ => err(format!("calc.erf() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

#[cfg(test)]
mod tests {
    use super::super::calc_test_support::call;
    use super::*;

    #[test]
    fn preserves_modeled_result() {
        assert_eq!(call(calc_erf, vec![Value::Float(-0.0)]).unwrap(), Value::Float(-0.0));
    }
}
