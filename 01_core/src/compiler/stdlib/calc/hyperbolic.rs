//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/calc/hyperbolic.md
//! @prompt-hash 3c9a40d7
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(crate) fn calc_sinh(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    unary_f64("calc.sinh", args, f64::sinh)
}

pub(crate) fn calc_cosh(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    unary_f64("calc.cosh", args, f64::cosh)
}

pub(crate) fn calc_tanh(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    unary_f64("calc.tanh", args, f64::tanh)
}

pub(crate) fn calc_asinh(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    unary_f64("calc.asinh", args, f64::asinh)
}

pub(crate) fn calc_acosh(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let x = coerce_to_f64(v, "calc.acosh()")?;
            if x < 1.0 {
                return err(format!("calc.acosh() valor deve ser >= 1, recebeu {x}"));
            }
            trig_op(f64::acosh, x)
        }
        _ => {
            err(format!("calc.acosh() requer 1 argumento, recebeu {}", args.items.len()))
        }
    }
}

pub(crate) fn calc_atanh(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let x = coerce_to_f64(v, "calc.atanh()")?;
            if x <= -1.0 || x >= 1.0 {
                return err(format!(
                    "calc.atanh() valor deve estar em (-1, 1), recebeu {x}"
                ));
            }
            trig_op(f64::atanh, x)
        }
        _ => {
            err(format!("calc.atanh() requer 1 argumento, recebeu {}", args.items.len()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::calc_test_support::call;
    use super::*;

    #[test]
    fn preserves_modeled_result() {
        assert_eq!(call(calc_cosh, vec![Value::Int(0)]).unwrap(), Value::Float(1.0));
    }
}
