//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/calc/round.md
//! @prompt-hash 10025b9b
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
    let digits: i32 = match args.named.get("digits") {
        Some(v) => match v {
            Value::Int(i) => *i as i32,
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
            if digits == 0 {
                Ok(Value::Float(f.round()))
            } else if digits >= 15 {
                // Paridade vanilla `round_with_precision`: precisão ≥ 15 é no-op.
                Ok(Value::Float(*f))
            } else if digits < -308 {
                // Além de qualquer dígito representável → zero com sinal.
                Ok(Value::Float(*f * 0.0))
            } else {
                let factor = 10f64.powi(digits);
                #[allow(clippy::disallowed_methods)]
                guard_float((f * factor).round() / factor)
            }
        }
        // P817-D — paridade vanilla: `round(Decimal)` → `Decimal`
        // (midpoint away from zero).
        [Value::Decimal(d)] => {
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

    #[test]
    fn preserves_modeled_result() {
        assert_eq!(call(calc_round, vec![Value::Float(1.5)]).unwrap(), Value::Float(2.0));
    }
}
