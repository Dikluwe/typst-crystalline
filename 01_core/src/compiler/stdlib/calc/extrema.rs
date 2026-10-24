//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/calc/extrema.md
//! @prompt-hash 2f987f53
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(crate) fn calc_min(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    if args.items.is_empty() {
        return err("calc.min() requer pelo menos 1 argumento");
    }
    let mut result = args.items[0].clone();
    for v in &args.items[1..] {
        result = match (&result, v) {
            (Value::Int(a), Value::Int(b)) => Value::Int(*a.min(b)),
            (Value::Float(a), Value::Float(b)) => Value::Float(a.min(*b)),
            (Value::Int(a), Value::Float(b)) => Value::Float((*a as f64).min(*b)),
            (Value::Float(a), Value::Int(b)) => Value::Float(a.min(*b as f64)),
            (_, other) => {
                return err(format!(
                    "calc.min() tipos incompatíveis: {} e {}",
                    result.type_name(),
                    other.type_name()
                ))
            }
        };
    }
    Ok(result)
}

pub(crate) fn calc_max(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    if args.items.is_empty() {
        return err("calc.max() requer pelo menos 1 argumento");
    }
    let mut result = args.items[0].clone();
    for v in &args.items[1..] {
        result = match (&result, v) {
            (Value::Int(a), Value::Int(b)) => Value::Int(*a.max(b)),
            (Value::Float(a), Value::Float(b)) => Value::Float(a.max(*b)),
            (Value::Int(a), Value::Float(b)) => Value::Float((*a as f64).max(*b)),
            (Value::Float(a), Value::Int(b)) => Value::Float(a.max(*b as f64)),
            (_, other) => {
                return err(format!(
                    "calc.max() tipos incompatíveis: {} e {}",
                    result.type_name(),
                    other.type_name()
                ))
            }
        };
    }
    Ok(result)
}

pub(crate) fn calc_clamp(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(v), Value::Int(lo), Value::Int(hi)] => {
            if lo > hi {
                return clamp_bound_error(
                    args,
                    2,
                    "max must be greater than or equal to min",
                );
            }
            Ok(Value::Int(if v < lo {
                *lo
            } else if v > hi {
                *hi
            } else {
                *v
            }))
        }
        [v, lo, hi] => {
            let vf = coerce_to_f64(v, "calc.clamp() value")?;
            let lof = coerce_to_f64(lo, "calc.clamp() min")?;
            let hif = coerce_to_f64(hi, "calc.clamp() max")?;
            if lof.is_nan() {
                return clamp_bound_error(args, 1, "min must not be NaN");
            }
            if hif.is_nan() {
                return clamp_bound_error(args, 2, "max must not be NaN");
            }
            if lof > hif {
                return clamp_bound_error(
                    args,
                    2,
                    "max must be greater than or equal to min",
                );
            }
            Ok(Value::Float(if vf < lof {
                lof
            } else if vf > hif {
                hif
            } else {
                vf
            }))
        }
        _ => {
            err(format!("calc.clamp() requer 3 argumentos, recebeu {}", args.items.len()))
        }
    }
}

fn clamp_bound_error(
    args: &Args,
    positional: usize,
    message: &str,
) -> SourceResult<Value> {
    let span = args
        .occurrences
        .as_ref()
        .and_then(|items| items.iter().filter(|arg| arg.name.is_none()).nth(positional))
        .map(|arg| arg.value_span)
        .unwrap_or_else(Span::detached);
    Err(vec![SourceDiagnostic::error(span, message)])
}

#[cfg(test)]
mod tests {
    use super::super::calc_test_support::call;
    use super::*;

    #[test]
    fn preserves_modeled_result() {
        assert_eq!(
            call(calc_clamp, vec![Value::Int(9), Value::Int(2), Value::Int(5)]).unwrap(),
            Value::Int(5)
        );
    }

    #[test]
    fn no_panic_clamp_reversed_integer_bounds_are_diagnostics() {
        for (lo, hi) in [(10, 0), (0, -1), (i64::MAX, i64::MIN)] {
            let errors =
                call(calc_clamp, vec![Value::Int(5), Value::Int(lo), Value::Int(hi)])
                    .expect_err("reversed bounds must return a diagnostic");
            assert_eq!(errors.len(), 1);
            assert_eq!(errors[0].message, "max must be greater than or equal to min");
        }
    }

    #[test]
    fn no_panic_clamp_reversed_float_bounds_use_language_diagnostic() {
        let errors = call(
            calc_clamp,
            vec![Value::Float(5.0), Value::Float(10.0), Value::Float(0.0)],
        )
        .expect_err("reversed bounds must return a diagnostic");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].message, "max must be greater than or equal to min");
    }

    #[test]
    fn no_panic_clamp_nan_bounds_are_diagnostics() {
        for (lo, hi, message) in [
            (f64::NAN, 2.0, "min must not be NaN"),
            (0.0, f64::NAN, "max must not be NaN"),
            (f64::NAN, f64::NAN, "min must not be NaN"),
        ] {
            let errors =
                call(calc_clamp, vec![Value::Int(1), Value::Float(lo), Value::Float(hi)])
                    .expect_err("unordered bounds must return a diagnostic");
            assert_eq!(errors.len(), 1);
            assert_eq!(errors[0].message, message);
        }
    }

    #[test]
    fn no_panic_clamp_keeps_nan_value_signed_zero_and_infinite_bounds() {
        let Value::Float(nan) =
            call(calc_clamp, vec![Value::Float(f64::NAN), Value::Int(0), Value::Int(2)])
                .unwrap()
        else {
            panic!("mixed numeric bounds must retain Float result")
        };
        assert!(nan.is_nan());
        let Value::Float(zero) = call(
            calc_clamp,
            vec![Value::Float(-0.0), Value::Float(0.0), Value::Float(1.0)],
        )
        .unwrap() else {
            panic!("floating clamp must retain Float result")
        };
        assert_eq!(zero.to_bits(), (-0.0f64).to_bits());
        assert_eq!(
            call(
                calc_clamp,
                vec![
                    Value::Int(5),
                    Value::Float(f64::NEG_INFINITY),
                    Value::Float(f64::INFINITY)
                ]
            )
            .unwrap(),
            Value::Float(5.0)
        );
        assert_eq!(
            call(
                calc_clamp,
                vec![Value::Int(i64::MIN), Value::Int(i64::MIN), Value::Int(i64::MAX)]
            )
            .unwrap(),
            Value::Int(i64::MIN)
        );
        assert_eq!(
            call(
                calc_clamp,
                vec![Value::Int(i64::MAX), Value::Int(i64::MIN), Value::Int(i64::MAX)]
            )
            .unwrap(),
            Value::Int(i64::MAX)
        );
    }

    #[test]
    fn no_panic_clamp_bound_errors_keep_their_exact_value_origin() {
        use crate::entities::args::ArgOccurrence;
        let file = FileId::from_raw(std::num::NonZeroU16::MIN);
        let min_span = Span::from_range(file, 3..4);
        let max_span = Span::from_range(file, 5..6);
        fn with_origins(
            ctx: &mut EvalContext,
            args: &Args,
            world: &dyn crate::contracts::world::World,
            file: FileId,
        ) -> SourceResult<Value> {
            let spans = [
                Span::from_range(file, 1..2),
                Span::from_range(file, 3..4),
                Span::from_range(file, 5..6),
            ];
            let occurrences = args
                .items
                .iter()
                .cloned()
                .zip(spans)
                .map(|(value, value_span)| ArgOccurrence {
                    name: None,
                    value,
                    span: value_span,
                    value_span,
                })
                .collect();
            calc_clamp(
                ctx,
                &Args::from_occurrences(Span::from_range(file, 0..9), occurrences),
                world,
                file,
            )
        }
        for (lo, hi, selected) in [
            (Value::Int(10), Value::Int(0), max_span),
            (Value::Float(f64::NAN), Value::Int(2), min_span),
            (Value::Int(0), Value::Float(f64::NAN), max_span),
        ] {
            let errors = call(with_origins, vec![Value::Int(1), lo, hi]).unwrap_err();
            assert_eq!(errors.len(), 1);
            assert_eq!(errors[0].span, selected);
            assert!(errors[0].hints.is_empty());
            assert!(errors[0].trace.is_empty());
        }
    }
}
