//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/calc/extrema.md
//! @prompt-hash 4b0d899f
//! @layer L1
//! @updated 2026-09-30

use super::*;
use crate::compiler::eval::operators::ordering::compare;
use std::cmp::Ordering;

pub(crate) fn calc_min(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    extremum("calc.min", args, Ordering::Less)
}

pub(crate) fn calc_max(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    extremum("calc.max", args, Ordering::Greater)
}

/// Compara cada valor com o extremo retido pela ordenação da linguagem e
/// devolve o argumento original.
fn extremum(name: &str, args: &Args, goal: Ordering) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    let Some((first, rest)) = args.items.split_first() else {
        return err(format!("{name}() requer pelo menos 1 argumento"));
    };
    let mut result = first;
    for (index, value) in rest.iter().enumerate() {
        let ordering = compare(value, result).map_err(|message| {
            vec![SourceDiagnostic::error(positional_span(args, index + 1), message)]
        })?;
        if ordering == goal {
            result = value;
        }
    }
    Ok(result.clone())
}

pub(crate) fn calc_clamp(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    if args.items.len() != 3 {
        return err(format!(
            "calc.clamp() requer 3 argumentos, recebeu {}",
            args.items.len()
        ));
    }
    let value = DecNum::from_arg(args, 0)?;
    let min = DecNum::from_arg(args, 1)?;
    let max = DecNum::from_arg(args, 2)?;
    if min.is_nan() {
        return bound_error(args, 1, "min must not be NaN");
    }
    if max.is_nan() {
        return bound_error(args, 2, "max must not be NaN");
    }
    let reversed = match min.pair(max) {
        Some(DecNumPair::Int(lo, hi)) => lo > hi,
        Some(DecNumPair::Float(lo, hi)) => lo > hi,
        Some(DecNumPair::Decimal(lo, hi)) => lo > hi,
        None => false,
    };
    if reversed {
        return bound_error(args, 2, "max must be greater than or equal to min");
    }
    match (value, min, max) {
        (DecNum::Int(v), DecNum::Int(lo), DecNum::Int(hi)) => {
            Ok(Value::Int(select(v, lo, hi)))
        }
        (DecNum::Decimal(_), ..)
        | (_, DecNum::Decimal(_), _)
        | (_, _, DecNum::Decimal(_)) => {
            let (Some(v), Some(lo), Some(hi)) =
                (value.decimal(), min.decimal(), max.decimal())
            else {
                return Err(decimal_and_float(args.span));
            };
            Ok(Value::Decimal(select(v, lo, hi)))
        }
        (DecNum::Int(_) | DecNum::Float(_), _, _) => {
            let (Some(v), Some(lo), Some(hi)) = (value.float(), min.float(), max.float())
            else {
                return Err(decimal_and_float(args.span));
            };
            Ok(Value::Float(select(v, lo, hi)))
        }
    }
}

/// Seleção total: um valor não ordenado (NaN) é devolvido.
fn select<T: PartialOrd>(value: T, lo: T, hi: T) -> T {
    if value < lo {
        lo
    } else if value > hi {
        hi
    } else {
        value
    }
}

fn bound_error(args: &Args, positional: usize, message: &str) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(positional_span(args, positional), message)])
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
    #[test]
    fn readiness_math_code_extrema_lengths() {
        use crate::entities::layout_types::Length;
        for (low, high) in
            [(Length::em(1.0), Length::em(2.0)), (Length::pt(1.0), Length::pt(2.0))]
        {
            let low = Value::Length(low);
            let high = Value::Length(high);
            assert_eq!(call(calc_min, vec![high.clone(), low.clone()]).unwrap(), low);
            assert_eq!(call(calc_max, vec![low.clone(), high.clone()]).unwrap(), high);
            assert_eq!(call(calc_max, vec![low.clone()]).unwrap(), low);
            assert_eq!(call(calc_min, vec![low.clone(), low.clone()]).unwrap(), low);
        }
    }

    #[test]
    fn readiness_math_code_extrema_incomparable_controls() {
        use crate::entities::layout_types::Length;
        for (a, b) in [
            (Length::em(1.0), Length::pt(2.0)),
            (Length::pt(1.0) + Length::em(1.0), Length::pt(1.0) + Length::em(1.0)),
        ] {
            assert!(call(calc_min, vec![Value::Length(a), Value::Length(b)]).is_err());
            assert!(call(calc_max, vec![Value::Length(a), Value::Length(b)]).is_err());
        }
        assert_eq!(
            call(calc_min, vec![Value::Int(1), Value::Float(2.5)]).unwrap(),
            Value::Int(1)
        );
        assert_eq!(
            call(calc_clamp, vec![Value::Int(9), Value::Int(2), Value::Int(5)]).unwrap(),
            Value::Int(5)
        );
    }
}
