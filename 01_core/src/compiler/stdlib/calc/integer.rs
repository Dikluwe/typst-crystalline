//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/calc/integer.md
//! @prompt-hash 269b321b
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
        DecNumPair::Float(a, b) => float_to_int((a / b).floor(), span),
        DecNumPair::Decimal(a, b) => a
            .checked_div(b)
            .and_then(|q| q.floor().to_i64())
            .map(Value::Int)
            .ok_or_else(|| too_large(span)),
    }
}

/// Euclid's algorithm as the language runs it: the remainder is checked and
/// the absolute value is taken last, so `int.min` reports an overflow.
// ref: lab/typst-original/crates/typst-library/src/foundations/calc.rs:748-784
fn gcd_impl(a: i64, b: i64) -> Option<i64> {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        let temp = b;
        b = a.checked_rem(b)?;
        a = temp;
    }
    a.checked_abs()
}

fn lcm_impl(a: i64, b: i64) -> Option<i64> {
    if a == b {
        return a.checked_abs();
    }
    a.checked_div(gcd_impl(a, b)?)?.checked_mul(b)?.checked_abs()
}

pub(crate) fn calc_gcd(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(a), Value::Int(b)] => {
            gcd_impl(*a, *b).map(Value::Int).ok_or_else(|| too_large(args.span))
        }
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
            lcm_impl(*a, *b).map(Value::Int).ok_or_else(|| too_large(args.span))
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
    fn gcd_and_lcm_report_an_overflowing_result() {
        let too_large = |result: SourceResult<Value>| {
            assert_eq!(result.unwrap_err()[0].message.as_str(), "the result is too large");
        };
        too_large(call(calc_gcd, vec![Value::Int(i64::MIN), Value::Int(-1)]));
        too_large(call(calc_gcd, vec![Value::Int(i64::MIN), Value::Int(0)]));
        too_large(call(calc_lcm, vec![Value::Int(i64::MIN), Value::Int(1)]));
        assert_eq!(call(calc_gcd, vec![Value::Int(-12), Value::Int(18)]).unwrap(), Value::Int(6));
        assert_eq!(call(calc_lcm, vec![Value::Int(-4), Value::Int(6)]).unwrap(), Value::Int(12));
        assert_eq!(call(calc_lcm, vec![Value::Int(0), Value::Int(0)]).unwrap(), Value::Int(0));
    }

    #[test]
    fn preserves_modeled_result() {
        assert_eq!(
            call(calc_quo, vec![Value::Int(7), Value::Int(2)]).unwrap(),
            Value::Int(3)
        );
    }
}
