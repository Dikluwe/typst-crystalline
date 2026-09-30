//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/calc/integer.md
//! @prompt-hash f26103e4
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(crate) fn calc_even(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(n)] => Ok(Value::Bool(n % 2 == 0)),
        [other] => err(format!("calc.even() requer Int, recebeu {}", other.type_name())),
        _ => err(format!("calc.even() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_odd(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(n)] => Ok(Value::Bool(n % 2 != 0)),
        [other] => err(format!("calc.odd() requer Int, recebeu {}", other.type_name())),
        _ => err(format!("calc.odd() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// Resto truncado: sinal acompanha o dividendo (paridade operador `%`).
pub(crate) fn calc_rem(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(a), Value::Int(b)] => {
            if *b == 0 {
                return err("calc.rem() divisão por zero");
            }
            // `checked_rem` evita panic em `i64::MIN % -1` (overflow).
            a.checked_rem(*b).map(Value::Int).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    "calc.rem() resto fora do alcance i64".to_string(),
                )]
            })
        }
        [a, b] => {
            let af = coerce_to_f64(a, "calc.rem() dividendo")?;
            let bf = coerce_to_f64(b, "calc.rem() divisor")?;
            if bf == 0.0 {
                return err("calc.rem() divisão por zero");
            }
            Ok(Value::Float(af % bf))
        }
        _ => err(format!("calc.rem() requer 2 argumentos, recebeu {}", args.items.len())),
    }
}

/// Resto Euclidiano: sempre ≥ 0 para divisor > 0.
pub(crate) fn calc_rem_euclid(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(a), Value::Int(b)] => {
            if *b == 0 {
                return err("calc.rem-euclid() divisão por zero");
            }
            a.checked_rem_euclid(*b).map(Value::Int).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    "calc.rem-euclid() resto fora do alcance i64".to_string(),
                )]
            })
        }
        [a, b] => {
            let af = coerce_to_f64(a, "calc.rem-euclid() dividendo")?;
            let bf = coerce_to_f64(b, "calc.rem-euclid() divisor")?;
            if bf == 0.0 {
                return err("calc.rem-euclid() divisão por zero");
            }
            Ok(Value::Float(af.rem_euclid(bf)))
        }
        _ => err(format!(
            "calc.rem-euclid() requer 2 argumentos, recebeu {}",
            args.items.len()
        )),
    }
}

/// Quociente Euclidiano: arredonda para -∞ quando divisor > 0.
pub(crate) fn calc_div_euclid(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(a), Value::Int(b)] => {
            if *b == 0 {
                return err("calc.div-euclid() divisão por zero");
            }
            a.checked_div_euclid(*b).map(Value::Int).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    "calc.div-euclid() quociente fora do alcance i64".to_string(),
                )]
            })
        }
        [a, b] => {
            let af = coerce_to_f64(a, "calc.div-euclid() dividendo")?;
            let bf = coerce_to_f64(b, "calc.div-euclid() divisor")?;
            if bf == 0.0 {
                return err("calc.div-euclid() divisão por zero");
            }
            Ok(Value::Float(af.div_euclid(bf)))
        }
        _ => err(format!(
            "calc.div-euclid() requer 2 argumentos, recebeu {}",
            args.items.len()
        )),
    }
}

/// Quociente **floored** em Int (paridade vanilla `calc.quo(-7, 2) = -4`).
///
/// P817-B — o cristalino truncava em direcção a zero (`-3`); o vanilla
/// arredonda para -∞ (`calc.rs:1140-1177`: divisão com ajuste quando os
/// sinais diferem e há resto, `floor` no caminho float). O resultado é
/// sempre `int` (vanilla: `SourceResult<i64>`).
pub(crate) fn calc_quo(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(a), Value::Int(b)] => {
            if *b == 0 {
                return err("calc.quo() divisão por zero");
            }
            let q = a.checked_div(*b).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    "calc.quo() quociente fora do alcance i64".to_string(),
                )]
            })?;
            // Arredonda para -∞ quando a fracção é negativa (paridade vanilla).
            let floored = if (*a < 0) != (*b < 0) && a % b != 0 { q - 1 } else { q };
            Ok(Value::Int(floored))
        }
        [a, b] => {
            let af = coerce_to_f64(a, "calc.quo() dividendo")?;
            let bf = coerce_to_f64(b, "calc.quo() divisor")?;
            if bf == 0.0 {
                return err("calc.quo() divisão por zero");
            }
            let q = (af / bf).floor();
            if !q.is_finite()
                || q < -9_223_372_036_854_775_808.0
                || q >= 9_223_372_036_854_775_808.0
            {
                return err("calc.quo() resultado fora do alcance i64");
            }
            Ok(Value::Int(q as i64))
        }
        _ => err(format!("calc.quo() requer 2 argumentos, recebeu {}", args.items.len())),
    }
}

/// Algoritmo de Euclides iterativo sobre valores absolutos.
fn gcd_impl(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub(crate) fn calc_gcd(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(a), Value::Int(b)] => Ok(Value::Int(gcd_impl(*a, *b))),
        [_, _] => err("calc.gcd() requer 2 inteiros"),
        _ => err(format!("calc.gcd() requer 2 argumentos, recebeu {}", args.items.len())),
    }
}

pub(crate) fn calc_lcm(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(a), Value::Int(b)] => {
            if *a == 0 || *b == 0 {
                return Ok(Value::Int(0));
            }
            let g = gcd_impl(*a, *b);
            // Dividir antes de multiplicar evita overflow intermédio.
            let aa = a.checked_abs().ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    "calc.lcm() valor fora do alcance i64".to_string(),
                )]
            })?;
            let bb = b.checked_abs().ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    "calc.lcm() valor fora do alcance i64".to_string(),
                )]
            })?;
            (aa / g).checked_mul(bb).map(Value::Int).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    "calc.lcm() resultado fora do alcance i64".to_string(),
                )]
            })
        }
        [_, _] => err("calc.lcm() requer 2 inteiros"),
        _ => err(format!("calc.lcm() requer 2 argumentos, recebeu {}", args.items.len())),
    }
}

#[cfg(test)]
mod tests {
    use super::super::calc_test_support::call;
    use super::*;

    #[test]
    fn preserves_modeled_result() {
        assert_eq!(
            call(calc_quo, vec![Value::Int(7), Value::Int(2)]).unwrap(),
            Value::Int(3)
        );
    }
}
