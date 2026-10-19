//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/calc/power.md
//! @prompt-hash f821d957
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// P817-C/D — paridade vanilla (`calc.rs:108-156`):
/// - `0^0` → Err ("zero to the power of zero is undefined");
/// - expoente `Int` que não cabe em `i32` → Err ("exponent is too large");
/// - expoente `Float` não-normal (inf/subnormal/NaN) → Err;
/// - `(Int, Int≥0)` → `Int` (`checked_pow`; overflow → Err "too large");
/// - `(Int, Int<0)` → `Float` via `powi` (`calc.pow(2, -1) = 0.5`);
/// - `(Decimal, Int)` → `Decimal` via `checked_powi`;
/// - `(Decimal, Float)` → erro dedicado decimal×float + hint (verbatim vanilla);
/// - resto → `Float` via `powf` (`guard_float`, ADR-0101: inf/NaN → Err).
pub(crate) fn calc_pow(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    let [base, exp] = args.items.as_slice() else {
        return err(format!(
            "calc.pow() requer 2 argumentos, recebeu {}",
            args.items.len()
        ));
    };

    let base = power_base(base, "calc.pow() base", "Int, Float ou Decimal")?;
    let exp = match power_base(exp, "calc.pow() expoente", "Int ou Float")? {
        PowerBase::Int(i) => PowerExponent::Int(i),
        PowerBase::Float(f) => PowerExponent::Float(f),
        PowerBase::Decimal(_) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "expected integer or float, found decimal",
            ).with_hint(
                "if loss of precision is acceptable, explicitly cast the decimal to a float with `float(value)`",
            )]);
        }
    };
    let base_zero = match base {
        PowerBase::Int(i) => i == 0,
        PowerBase::Float(f) => f == 0.0,
        PowerBase::Decimal(d) => d.0.is_zero(),
    };
    let exp_zero = match exp {
        PowerExponent::Int(i) => i == 0,
        PowerExponent::Float(f) => f == 0.0,
    };
    if base_zero && exp_zero {
        return err("calc.pow() zero elevado a zero é indefinido");
    }
    match exp {
        PowerExponent::Int(i) => {
            if i32::try_from(i).is_err() {
                return err("calc.pow() expoente demasiado grande");
            }
        }
        PowerExponent::Float(f) => {
            if !f.is_normal() && f != 0.0 {
                return err(
                    "calc.pow() expoente não pode ser infinito, subnormal ou NaN",
                );
            }
        }
    }

    match (base, exp) {
        (PowerBase::Int(a), PowerExponent::Int(b)) if b >= 0 => {
            a.checked_pow(b as u32).map(Value::Int).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    "calc.pow() resultado fora do alcance i64".to_string(),
                )]
            })
        }
        (PowerBase::Int(a), PowerExponent::Int(b)) => {
            #[allow(clippy::disallowed_methods)]
            guard_float((a as f64).powi(b as i32))
        }
        (PowerBase::Decimal(d), PowerExponent::Int(b)) => d.checked_powi(b).map(Value::Decimal).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "calc.pow() resultado fora do alcance decimal".to_string(),
            )]
        }),
        (PowerBase::Decimal(_), PowerExponent::Float(_)) => Err(vec![SourceDiagnostic::error(
            args.span,
            "cannot apply this operation to a decimal and a float".to_string(),
        )
        .with_hint(
            "if loss of precision is acceptable, explicitly cast the decimal to a float with `float(value)`",
        )]),
        (PowerBase::Float(b), PowerExponent::Int(e)) => {
            // Preserve the existing guarded floating path for floating bases.
            #[allow(clippy::disallowed_methods)]
            guard_float(b.powf(e as f64))
        }
        (PowerBase::Float(b), PowerExponent::Float(e)) => {
            #[allow(clippy::disallowed_methods)]
            guard_float(b.powf(e))
        }
        (PowerBase::Int(b), PowerExponent::Float(e)) => {
            #[allow(clippy::disallowed_methods)]
            guard_float((b as f64).powf(e))
        }
    }
}

#[derive(Clone, Copy)]
enum PowerBase {
    Int(i64),
    Float(f64),
    Decimal(crate::entities::decimal::Decimal),
}

#[derive(Clone, Copy)]
enum PowerExponent {
    Int(i64),
    Float(f64),
}

fn power_base(value: &Value, context: &str, expected: &str) -> SourceResult<PowerBase> {
    match value {
        Value::Int(i) => Ok(PowerBase::Int(*i)),
        Value::Float(f) => Ok(PowerBase::Float(*f)),
        Value::Decimal(d) => Ok(PowerBase::Decimal(*d)),
        Value::None
        | Value::Bool(_)
        | Value::Str(_)
        | Value::Array(_)
        | Value::Dict(_)
        | Value::Module(_)
        | Value::Datetime(_)
        | Value::Func(_)
        | Value::Content(_)
        | Value::LocatedContent(..)
        | Value::Auto
        | Value::Length(_)
        | Value::Relative(_)
        | Value::Ratio(_)
        | Value::Angle(_)
        | Value::Color(_)
        | Value::Stroke(_)
        | Value::Fraction(_)
        | Value::Align(_)
        | Value::Location(_)
        | Value::Gradient(_)
        | Value::Regex(_)
        | Value::Tiling(_)
        | Value::Bytes(_)
        | Value::Duration(_)
        | Value::Version(_)
        | Value::Selector(_)
        | Value::Symbol(_)
        | Value::Args(_)
        | Value::State(_)
        | Value::Counter(_)
        | Value::Label(_)
        | Value::Dir(_)
        | Value::Path(_)
        | Value::Type(_)
        | Value::SpotColorant(_) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{context}: esperava {expected}, recebeu {}", value.type_name()),
        )]),
    }
}

pub(crate) fn calc_sqrt(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let f = coerce_to_f64(v, "calc.sqrt()")?;
            if f < 0.0 {
                return err("calc.sqrt() argumento negativo");
            }
            guard_float(f.sqrt())
        }
        _ => err(format!("calc.sqrt() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// Raiz n-ésima: `root(radicand, index)` — **ordem vanilla** (radicando
/// primeiro, índice depois; medido em P817: vanilla `calc.root(27.0, 3) = 3`,
/// `calc.root(-8, 3) = -2`). Preserva sinal para `index` ímpar com radicando
/// negativo; índice negativo equivale a `x^(1/index)` (float).
pub(crate) fn calc_root(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [x, Value::Int(index)] => {
            if *index == 0 {
                return err("calc.root() índice de raiz zero");
            }
            let xf = coerce_to_f64(x, "calc.root() valor")?;
            let inv = 1.0 / (*index as f64);
            let r = if xf < 0.0 {
                if index.rem_euclid(2) == 0 {
                    return err("calc.root() raiz par de número negativo");
                }
                // DEBT-libm: f64::powf banido — partilhado com calc_pow / calc_norm.
                #[allow(clippy::disallowed_methods)]
                let v = (-xf).powf(inv);
                -v
            } else {
                #[allow(clippy::disallowed_methods)]
                let v = xf.powf(inv);
                v
            };
            guard_float(r)
        }
        [_, other] => {
            err(format!("calc.root() índice deve ser Int, recebeu {}", other.type_name()))
        }
        _ => {
            err(format!("calc.root() requer 2 argumentos, recebeu {}", args.items.len()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::calc_test_support::call;
    use super::*;

    #[test]
    fn preserves_modeled_result() {
        assert_eq!(
            call(calc_pow, vec![Value::Int(2), Value::Int(-1)]).unwrap(),
            Value::Float(0.5)
        );
    }
}

#[cfg(test)]
mod v16_power_tests {
    use super::super::calc_test_support::call;
    use super::*;
    use crate::entities::decimal::Decimal;

    #[test]
    fn v16_power_invalid_base_before_large_exponent() {
        let diagnostics =
            call(calc_pow, vec![Value::None, Value::Int(2_147_483_648)]).unwrap_err();
        assert!(diagnostics[0].message.contains("base"));
        assert!(diagnostics[0].message.contains("Decimal"));
    }

    #[test]
    fn v16_power_invalid_base_before_non_normal_exponent() {
        let diagnostics =
            call(calc_pow, vec![Value::None, Value::Float(f64::NAN)]).unwrap_err();
        assert!(diagnostics[0].message.contains("base"));
    }

    #[test]
    fn v16_power_decimal_exponent_has_conversion_hint() {
        let diagnostics =
            call(calc_pow, vec![Value::Int(2), Value::Decimal(Decimal::new(0, 0))])
                .unwrap_err();
        assert_eq!(diagnostics[0].hints, vec!["if loss of precision is acceptable, explicitly cast the decimal to a float with `float(value)`"]);
    }

    #[test]
    fn v16_power_numeric_kinds() {
        for (base, exponent, expected) in [
            (Value::Int(2), Value::Int(3), Value::Int(8)),
            (Value::Int(2), Value::Int(-1), Value::Float(0.5)),
            (Value::Int(2), Value::Float(3.0), Value::Float(8.0)),
            (Value::Float(2.0), Value::Int(3), Value::Float(8.0)),
            (Value::Float(2.0), Value::Float(3.0), Value::Float(8.0)),
            (
                Value::Decimal(Decimal::new(25, 1)),
                Value::Int(3),
                Value::Decimal(Decimal::new(15625, 3)),
            ),
            (
                Value::Decimal(Decimal::new(2, 0)),
                Value::Int(-1),
                Value::Decimal(Decimal::new(5, 1)),
            ),
        ] {
            assert_eq!(call(calc_pow, vec![base, exponent]).unwrap(), expected);
        }
    }

    #[test]
    fn v16_power_zero_and_overflow() {
        for base in [Value::Int(0), Value::Float(0.0), Value::Decimal(Decimal::new(0, 0))]
        {
            for exponent in [Value::Int(0), Value::Float(0.0)] {
                let diagnostics =
                    call(calc_pow, vec![base.clone(), exponent]).unwrap_err();
                assert!(diagnostics[0].message.contains("zero"));
            }
        }
        for exponent in [63, 2_147_483_648] {
            assert!(call(calc_pow, vec![Value::Int(2), Value::Int(exponent)]).is_err());
        }
    }

    #[test]
    fn v16_power_decimal_float_diagnostic() {
        let diagnostics =
            call(calc_pow, vec![Value::Decimal(Decimal::new(2, 0)), Value::Float(2.0)])
                .unwrap_err();
        assert_eq!(
            diagnostics[0].message,
            "cannot apply this operation to a decimal and a float"
        );
        assert_eq!(diagnostics[0].hints.len(), 1);
    }

    #[test]
    fn v16_power_invalid_arguments() {
        assert!(call(calc_pow, vec![Value::Int(2), Value::None]).is_err());
        assert!(call(calc_pow, vec![Value::None, Value::None]).unwrap_err()[0]
            .message
            .contains("base"));
        assert!(call(calc_pow, vec![Value::Int(2)]).is_err());
    }

    #[test]
    fn v16_power_root_and_sqrt_controls() {
        assert_eq!(
            call(calc_root, vec![Value::Int(-8), Value::Int(3)]).unwrap(),
            Value::Float(-2.0)
        );
        assert_eq!(call(calc_sqrt, vec![Value::Int(4)]).unwrap(), Value::Float(2.0));
    }
}
