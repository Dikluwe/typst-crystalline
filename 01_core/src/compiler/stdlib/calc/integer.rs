//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/calc/integer.md
//! @prompt-hash 5e570819
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

/// Dividendo e divisor não nulo, trazidos a um tipo comum.
fn division_operands(name: &str, args: &Args) -> SourceResult<DecNumPair> {
    expect_no_named(&args.named)?;
    if args.items.len() != 2 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{name}() requer 2 argumentos, recebeu {}", args.items.len()),
        )]);
    }
    let dividend = DecNum::from_arg(args, 0)?;
    let divisor = DecNum::from_arg(args, 1)?;
    if divisor.is_zero() {
        return Err(vec![SourceDiagnostic::error(
            positional_span(args, 1),
            "divisor must not be zero",
        )]);
    }
    dividend.pair(divisor).ok_or_else(|| decimal_and_float(args.span))
}

fn dividend_too_small(span: Span) -> Vec<SourceDiagnostic> {
    vec![SourceDiagnostic::error(span, "dividend too small compared to divisor")]
}

/// Resto truncado: sinal acompanha o dividendo (paridade operador `%`).
pub(crate) fn calc_rem(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let span = args.span;
    match division_operands("calc.rem", args)? {
        // Só `i64::MIN` por -1 transborda, e o resto matemático é zero.
        DecNumPair::Int(a, b) => Ok(Value::Int(a.checked_rem(b).unwrap_or(0))),
        DecNumPair::Float(a, b) => Ok(Value::Float(a % b)),
        DecNumPair::Decimal(a, b) => a
            .checked_rem(b)
            .map(Value::Decimal)
            .ok_or_else(|| dividend_too_small(span)),
    }
}

/// Resto Euclidiano: nunca negativo.
pub(crate) fn calc_rem_euclid(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let span = args.span;
    match division_operands("calc.rem-euclid", args)? {
        // Só `i64::MIN` por -1 transborda, e o resto matemático é zero.
        DecNumPair::Int(a, b) => Ok(Value::Int(a.checked_rem_euclid(b).unwrap_or(0))),
        DecNumPair::Float(a, b) => Ok(Value::Float(a.rem_euclid(b))),
        DecNumPair::Decimal(a, b) => a
            .checked_rem_euclid(b)
            .map(Value::Decimal)
            .ok_or_else(|| dividend_too_small(span)),
    }
}

/// Quociente Euclidiano.
pub(crate) fn calc_div_euclid(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let span = args.span;
    match division_operands("calc.div-euclid", args)? {
        DecNumPair::Int(a, b) => {
            a.checked_div_euclid(b).map(Value::Int).ok_or_else(|| too_large(span))
        }
        DecNumPair::Float(a, b) => Ok(Value::Float(a.div_euclid(b))),
        DecNumPair::Decimal(a, b) => a
            .checked_div_euclid(b)
            .map(Value::Decimal)
            .ok_or_else(|| too_large(span)),
    }
}

/// Quociente **floored**, sempre Int (`calc.quo(-7, 2) = -4`).
pub(crate) fn calc_quo(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let span = args.span;
    match division_operands("calc.quo", args)? {
        DecNumPair::Int(a, b) => {
            let q = a.checked_div(b).ok_or_else(|| too_large(span))?;
            // Arredonda para -∞ quando a fracção é negativa.
            let negative = (a < 0) != (b < 0) && a % b != 0;
            Ok(Value::Int(if negative { q - 1 } else { q }))
        }
        DecNumPair::Float(a, b) => {
            let q = (a / b).floor();
            if q.is_nan() {
                return Err(not_a_real_number(span));
            }
            float_to_int(q, span)
        }
        DecNumPair::Decimal(a, b) => a
            .checked_div(b)
            .and_then(|q| q.floor().to_i64())
            .map(Value::Int)
            .ok_or_else(|| too_large(span)),
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
