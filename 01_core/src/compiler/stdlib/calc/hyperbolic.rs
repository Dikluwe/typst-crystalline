//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/calc/hyperbolic.md
//! @prompt-hash a0ddb3c5
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(crate) fn calc_sinh(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    unary_f64("calc.sinh", args, libm::sinh)
}

pub(crate) fn calc_cosh(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    unary_f64("calc.cosh", args, libm::cosh)
}

pub(crate) fn calc_tanh(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    unary_f64("calc.tanh", args, libm::tanh)
}

pub(crate) fn calc_asinh(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    unary_f64("calc.asinh", args, libm::asinh)
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
            trig_op(libm::acosh, x)
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
            trig_op(libm::atanh, x)
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

    type MathFunction = fn(
        &mut EvalContext,
        &Args,
        &dyn crate::contracts::world::World,
        FileId,
    ) -> SourceResult<Value>;

    #[test]
    fn transcendental_hyperbolic_values_and_domains() {
        // Constants measured independently through the pinned vanilla CLI.
        let cases: [(MathFunction, f64, f64); 6] = [
            (calc_sinh, 1.0, 1.1752011936438014),
            (calc_cosh, 1.0, 1.543080634815244),
            (calc_tanh, 1.0, 0.7615941559557649),
            (calc_asinh, 1.0, 0.881373587019543),
            (calc_acosh, 2.0, 1.3169578969248166),
            (calc_atanh, 0.5, 0.5493061443340548),
        ];
        for (function, input, expected) in cases {
            let Value::Float(value) = call(function, vec![Value::Float(input)]).unwrap()
            else {
                panic!("hyperbolic function must return float");
            };
            assert!((value - expected).abs() < 1e-14);
        }
        assert!(call(calc_acosh, vec![Value::Float(0.99)]).is_err());
        for boundary in [-1.0, 1.0] {
            assert!(call(calc_atanh, vec![Value::Float(boundary)]).is_err());
        }
        let Value::Float(zero) = call(calc_tanh, vec![Value::Float(-0.0)]).unwrap()
        else {
            panic!("tanh must return float");
        };
        assert_eq!(1.0 / zero, f64::NEG_INFINITY);
    }
}
