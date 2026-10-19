//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/calc/norm.md
//! @prompt-hash e74702f7
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Norma p de um vector: `(Σ |x_i|^p)^(1/p)`. `p` é named arg, default `2.0`.
pub(crate) fn calc_norm(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // `p` é o único named arg aceite.
    let p = match args.named.get("p") {
        Some(Value::Float(f)) => *f,
        Some(Value::Int(i)) => *i as f64,
        Some(other) => {
            return err(format!(
                "calc.norm() argumento 'p' requer Int ou Float, recebeu {}",
                other.type_name(),
            ))
        }
        None => 2.0,
    };
    if args.named.len() > 1 || (args.named.len() == 1 && !args.named.contains_key("p")) {
        // Detecta outros named args para erro claro.
        let bad = args
            .named
            .keys()
            .find(|k| k.as_str() != "p")
            .map(|k| k.as_str())
            .unwrap_or("?");
        return err(format!("calc.norm() argumento nomeado desconhecido: '{bad}'"));
    }

    if args.items.is_empty() {
        return Ok(Value::Float(0.0));
    }
    let mut sum: f64 = 0.0;
    for v in &args.items {
        let x = coerce_to_f64(v, "calc.norm() valor")?;
        // DEBT-libm: f64::powf banido — partilhado com calc_pow / calc_root.
        #[allow(clippy::disallowed_methods)]
        let term = x.abs().powf(p);
        sum += term;
    }
    #[allow(clippy::disallowed_methods)]
    let r = sum.powf(1.0 / p);
    guard_float(r)
}

#[cfg(test)]
mod tests {
    use super::super::calc_test_support::call;
    use super::*;

    #[test]
    fn preserves_modeled_result() {
        assert_eq!(
            call(calc_norm, vec![Value::Int(3), Value::Int(4)]).unwrap(),
            Value::Float(5.0)
        );
    }
}
