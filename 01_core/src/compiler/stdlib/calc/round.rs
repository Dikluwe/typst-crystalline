//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/calc/round.md
//! @prompt-hash 50707201
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(crate) fn calc_floor(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(i)] => Ok(Value::Int(*i)),
        [Value::Float(f)] => Ok(Value::Int(f.floor() as i64)),
        // P817-D — paridade vanilla `calc.floor(decimal)` → int.
        [Value::Decimal(d)] => d.floor().to_i64().map(Value::Int).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "calc.floor() resultado fora do alcance i64".to_string(),
            )]
        }),
        [other] => err(format!(
            "calc.floor() requer Int ou Float, recebeu {}",
            other.type_name()
        )),
        _ => {
            err(format!("calc.floor() requer 1 argumento, recebeu {}", args.items.len()))
        }
    }
}

pub(crate) fn calc_ceil(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(i)] => Ok(Value::Int(*i)),
        [Value::Float(f)] => Ok(Value::Int(f.ceil() as i64)),
        // P817-D — paridade vanilla `calc.ceil(decimal)` → int.
        [Value::Decimal(d)] => d.ceil().to_i64().map(Value::Int).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "calc.ceil() resultado fora do alcance i64".to_string(),
            )]
        }),
        [other] => {
            err(format!("calc.ceil() requer Int ou Float, recebeu {}", other.type_name()))
        }
        _ => err(format!("calc.ceil() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_round(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // P491 — arg nomeado `digits` (default 0). Apenas `digits` é aceite como named arg.
    let digits: i64 = match args.named.get("digits") {
        Some(v) => match v {
            Value::Int(i) => *i,
            other => {
                return err(format!(
                    "calc.round() argumento 'digits' requer Int, recebeu {}",
                    other.type_name()
                ))
            }
        },
        None => 0,
    };
    if args.named.len() > 1
        || (args.named.len() == 1 && !args.named.contains_key("digits"))
    {
        let bad = args
            .named
            .keys()
            .find(|k| k.as_str() != "digits")
            .map(|k| k.as_str())
            .unwrap_or("?");
        return err(format!("calc.round() argumento nomeado desconhecido: '{bad}'"));
    }

    match args.items.as_slice() {
        // P817-D — paridade vanilla: `round(Int)` → `Int`. `digits` > 0 é
        // no-op; `digits` < 0 arredonda antes do ponto (away from zero).
        [Value::Int(i)] => {
            let digits = digits.clamp(i64::from(i16::MIN), i64::from(i16::MAX)) as i32;
            if digits >= 0 {
                Ok(Value::Int(*i))
            } else {
                round_int_com_precisao(*i, digits).map(Value::Int).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        "calc.round() resultado fora do alcance i64".to_string(),
                    )]
                })
            }
        }
        // P817-D — paridade vanilla: `round(Float)` → `Float` (não `Int`).
        [Value::Float(f)] => {
            let digits = digits.clamp(i64::from(i16::MIN), i64::from(i16::MAX)) as i32;
            if digits == 0 {
                Ok(Value::Float(f.round()))
            } else if digits >= 15
                || (digits > 0 && f.is_finite() && f.abs() >= 9_007_199_254_740_992.0)
            {
                // Precisão ≥ 15 ou mantissa inteira já preenchida: no-op.
                Ok(Value::Float(*f))
            } else if digits < -308 {
                // Além de qualquer dígito representável → zero com sinal.
                Ok(Value::Float(*f * 0.0))
            } else {
                let factor = libm::exp10(f64::from(digits.abs()));
                guard_float(if digits > 0 {
                    (f * factor).round() / factor
                } else {
                    (f / factor).round() * factor
                })
            }
        }
        // P817-D — paridade vanilla: `round(Decimal)` → `Decimal`
        // (midpoint away from zero).
        [Value::Decimal(d)] => {
            let digits = digits.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32;
            d.round_with_digits(digits).map(Value::Decimal).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    "calc.round() resultado fora do alcance decimal".to_string(),
                )]
            })
        }
        [other] => err(format!(
            "calc.round() requer Int ou Float, recebeu {}",
            other.type_name()
        )),
        _ => {
            err(format!("calc.round() requer 1 argumento, recebeu {}", args.items.len()))
        }
    }
}

/// Port de `round_int_with_precision` (vanilla `typst-utils/round.rs:82`):
/// arredonda um inteiro `digits` casas **antes** do ponto decimal
/// (`digits < 0`), away from zero. `None` em overflow.
fn round_int_com_precisao(value: i64, digits: i32) -> Option<i64> {
    if digits >= 0 {
        return Some(value);
    }
    let n = u32::try_from(digits.checked_neg()?).ok()?;
    let Some(dez) = 10i64.checked_pow(n.checked_sub(1)?) else {
        // Mais dígitos do que qualquer inteiro representável.
        return Some(0);
    };
    let truncado = value / dez;
    if truncado == 0 {
        return Some(0);
    }
    let arredondado = if (truncado % 10).abs() >= 5 {
        truncado.checked_add(truncado.signum() * (10 - (truncado % 10).abs()))?
    } else {
        truncado - (truncado % 10)
    };
    arredondado.checked_mul(dez)
}

// ── P306 — aritmética inteira, combinatória, norma, raiz ────────────────────
//
// Funções puramente aritméticas em `std::*` (`i64::checked_*`,
// `f64::rem_euclid`, etc.). `calc_norm`/`calc_root` usam `f64::powf` —
// DEBT-libm partilhado com `calc_pow` (ADR-0018).

pub(crate) fn calc_trunc(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(i)] => Ok(Value::Int(*i)),
        [Value::Float(f)] => Ok(Value::Int(f.trunc() as i64)),
        // P817-D — paridade vanilla `calc.trunc(decimal)` → int.
        [Value::Decimal(d)] => d.trunc().to_i64().map(Value::Int).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "calc.trunc() resultado fora do alcance i64".to_string(),
            )]
        }),
        [other] => err(format!(
            "calc.trunc() requer Int ou Float, recebeu {}",
            other.type_name()
        )),
        _ => {
            err(format!("calc.trunc() requer 1 argumento, recebeu {}", args.items.len()))
        }
    }
}

pub(crate) fn calc_fract(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        // P817-D — paridade vanilla: `calc.fract(int)` → int `0` (não `0.0`).
        [Value::Int(_)] => Ok(Value::Int(0)),
        [Value::Float(f)] => Ok(Value::Float(f.fract())),
        // P817-D — paridade vanilla `calc.fract(decimal)` → decimal.
        [Value::Decimal(d)] => Ok(Value::Decimal(d.fract())),
        [other] => err(format!(
            "calc.fract() requer Int ou Float, recebeu {}",
            other.type_name()
        )),
        _ => {
            err(format!("calc.fract() requer 1 argumento, recebeu {}", args.items.len()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::calc_test_support::call;
    use super::*;
    use crate::entities::decimal::Decimal;

    fn round(value: Value, digits: i64) -> SourceResult<Value> {
        fn with_digits(
            ctx: &mut EvalContext,
            args: &Args,
            world: &dyn crate::contracts::world::World,
            file: FileId,
        ) -> SourceResult<Value> {
            let mut named = Args::positional(vec![args.items[0].clone()]);
            named.named.insert("digits".into(), args.items[1].clone());
            calc_round(ctx, &named, world, file)
        }
        call(with_digits, vec![value, Value::Int(digits)])
    }

    #[test]
    fn finite_large_values_avoid_intermediate_overflow() {
        for value in [1e308, -1e308] {
            assert_eq!(round(Value::Float(value), 1).unwrap(), Value::Float(value));
        }
        for (value, expected) in [(1e308, 1e308), (1.4e308, 1e308), (-1.4e308, -1e308)] {
            assert_eq!(round(Value::Float(value), -308).unwrap(), Value::Float(expected));
        }
    }

    #[test]
    fn wide_precision_saturates_without_wrapping() {
        for digits in [1i64 << 32, i64::MAX] {
            for value in [
                Value::Int(125),
                Value::Float(1.25),
                Value::Decimal(Decimal::new(125, 2)),
            ] {
                assert_eq!(round(value.clone(), digits).unwrap(), value);
            }
        }
        for digits in [-(1i64 << 32), i64::MIN] {
            for (value, expected) in [
                (Value::Int(125), Value::Int(0)),
                (Value::Float(1.25), Value::Float(0.0)),
                (
                    Value::Decimal(Decimal::new(125, 2)),
                    Value::Decimal(Decimal::new(0, 0)),
                ),
            ] {
                assert_eq!(round(value, digits).unwrap(), expected);
            }
        }
    }

    #[test]
    fn ties_and_signed_zero_remain_observable() {
        for (value, expected) in [(1250.0, 1300.0), (-1250.0, -1300.0)] {
            assert_eq!(round(Value::Float(value), -2).unwrap(), Value::Float(expected));
        }
        for (value, digits) in [(-0.4, 0), (-1e-300, -309), (-1.0, i64::MIN)] {
            let Value::Float(result) = round(Value::Float(value), digits).unwrap() else {
                panic!("round(float) must preserve the language type");
            };
            assert_eq!(result, 0.0);
            assert!(result.is_sign_negative());
        }
    }

    #[test]
    fn existing_nonfinite_and_overflow_policy_is_preserved() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(round(Value::Float(value), 2).is_err());
        }
        for value in [1.6e308, -1.6e308] {
            assert!(round(Value::Float(value), -308).is_err());
        }
        assert!(round(Value::Int(i64::MAX), -1).is_err());
        for digits in [0, 15, i64::MAX] {
            assert_eq!(
                round(Value::Float(f64::INFINITY), digits).unwrap(),
                Value::Float(f64::INFINITY)
            );
        }
        let Value::Float(value) = round(Value::Float(f64::INFINITY), -309).unwrap()
        else {
            panic!("round(float) must preserve the language type");
        };
        assert!(value.is_nan());
    }

    #[test]
    fn preserves_modeled_result() {
        assert_eq!(call(calc_round, vec![Value::Float(1.5)]).unwrap(), Value::Float(2.0));
    }
}
