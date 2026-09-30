//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/calc/combinatorics.md
//! @prompt-hash 409193c3
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(crate) fn calc_fact(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(n)] => {
            if *n < 0 {
                return err("calc.fact() factorial de número negativo");
            }
            let mut acc: i64 = 1;
            for i in 1..=*n {
                acc = acc.checked_mul(i).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        "calc.fact() resultado fora do alcance i64".to_string(),
                    )]
                })?;
            }
            Ok(Value::Int(acc))
        }
        [other] => err(format!("calc.fact() requer Int, recebeu {}", other.type_name())),
        _ => err(format!("calc.fact() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// Arranjos: `P(n, k) = n * (n-1) * ... * (n-k+1)`.
pub(crate) fn calc_perm(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(n), Value::Int(k)] => {
            if *n < 0 || *k < 0 {
                return err("calc.perm() argumento negativo");
            }
            if *k > *n {
                return Ok(Value::Int(0));
            }
            let mut acc: i64 = 1;
            for i in 0..*k {
                acc = acc.checked_mul(n - i).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        "calc.perm() resultado fora do alcance i64".to_string(),
                    )]
                })?;
            }
            Ok(Value::Int(acc))
        }
        [_, _] => err("calc.perm() requer 2 inteiros"),
        _ => {
            err(format!("calc.perm() requer 2 argumentos, recebeu {}", args.items.len()))
        }
    }
}

/// Combinações: algoritmo iterativo com divisão exacta a cada passo.
///
/// `C(n, k) = ∏_{i=0..k} (n - i) / (i + 1)`. A divisão exacta a cada
/// iteração é garantida pela propriedade combinatória: o produto parcial
/// após `i` passos é divisível por `(i + 1)`.
pub(crate) fn calc_binom(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(n), Value::Int(k)] => {
            if *n < 0 || *k < 0 {
                return err("calc.binom() argumento negativo");
            }
            if *k > *n {
                return Ok(Value::Int(0));
            }
            // Simetria: C(n, k) = C(n, n-k). Reduz iterações.
            let k_eff = (*k).min(*n - *k);
            let mut acc: i64 = 1;
            for i in 0..k_eff {
                acc = acc.checked_mul(n - i).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        "calc.binom() resultado fora do alcance i64".to_string(),
                    )]
                })?;
                acc /= i + 1; // divisão exacta garantida pela propriedade binomial.
            }
            Ok(Value::Int(acc))
        }
        [_, _] => err("calc.binom() requer 2 inteiros"),
        _ => {
            err(format!("calc.binom() requer 2 argumentos, recebeu {}", args.items.len()))
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
            call(calc_binom, vec![Value::Int(5), Value::Int(2)]).unwrap(),
            Value::Int(10)
        );
    }
}
