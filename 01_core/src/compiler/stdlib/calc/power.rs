//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/calc/power.md
//! @prompt-hash 44bc9ab2
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

    let base_zero = match base {
        Value::Int(i) => *i == 0,
        Value::Float(f) => *f == 0.0,
        Value::Decimal(d) => d.0.is_zero(),
        _ => false, // neutro: N16[β] — tipos não numéricos retornam false no predicado is_zero
    };
    let exp_zero = match exp {
        Value::Int(i) => *i == 0,
        Value::Float(f) => *f == 0.0,
        _ => false, // neutro: N16[β] — tipos não numéricos retornam false no predicado is_zero
    };
    if base_zero && exp_zero {
        return err("calc.pow() zero elevado a zero é indefinido");
    }
    if let Value::Int(i) = exp {
        if i32::try_from(*i).is_err() {
            return err("calc.pow() expoente demasiado grande");
        }
    }
    if let Value::Float(f) = exp {
        if !f.is_normal() && *f != 0.0 {
            return err("calc.pow() expoente não pode ser infinito, subnormal ou NaN");
        }
    }

    match (base, exp) {
        (Value::Int(a), Value::Int(b)) if *b >= 0 => {
            a.checked_pow(*b as u32).map(Value::Int).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    "calc.pow() resultado fora do alcance i64".to_string(),
                )]
            })
        }
        (Value::Int(a), Value::Int(b)) => {
            // Expoente inteiro negativo → Float (paridade vanilla `powi`).
            #[allow(clippy::disallowed_methods)]
            guard_float((*a as f64).powi(*b as i32))
        }
        (Value::Decimal(d), Value::Int(b)) => d.checked_powi(*b).map(Value::Decimal).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "calc.pow() resultado fora do alcance decimal".to_string(),
            )]
        }),
        (Value::Decimal(_), Value::Float(_)) => Err(vec![SourceDiagnostic::error(
            args.span,
            "cannot apply this operation to a decimal and a float".to_string(),
        )
        .with_hint(
            "if loss of precision is acceptable, explicitly cast the decimal to a float with `float(value)`",
        )]),
        _ => {
            let b = coerce_to_f64(base, "calc.pow() base")?;
            let e = coerce_to_f64(exp, "calc.pow() expoente")?;
            // DEBT: migrar para libm::pow quando libm for dependência do workspace (ADR-0018)
            #[allow(clippy::disallowed_methods)]
            guard_float(b.powf(e))
        }
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
