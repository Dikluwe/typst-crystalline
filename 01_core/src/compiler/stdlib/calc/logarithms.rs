//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/calc/logarithms.md
//! @prompt-hash 928602e9
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(crate) fn calc_exp(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    unary_f64("calc.exp", args, f64::exp)
}

pub(crate) fn calc_ln(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let x = coerce_to_f64(v, "calc.ln()")?;
            if x <= 0.0 {
                return err(format!(
                    "calc.ln() valor deve ser estritamente positivo, recebeu {x}"
                ));
            }
            #[allow(clippy::disallowed_methods)]
            let r = f64::ln(x);
            guard_float(r)
        }
        _ => err(format!("calc.ln() requer 1 argumento, recebeu {}", args.items.len())),
    }
}

/// `calc.log(x)` (base 10), `calc.log(x, base)` ou `calc.log(x, base: base)`.
/// P491: suporte ao arg nomeado `base:` (default 10), preservando a forma
/// posicional legada.
pub(crate) fn calc_log(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // P491 — arg nomeado `base` (default 10). Apenas `base` é aceite como named arg.
    let base_from_named = args
        .named
        .get("base")
        .map(|v| coerce_to_f64(v, "calc.log() base"))
        .transpose()?;
    if args.named.len() > 1 || (args.named.len() == 1 && !args.named.contains_key("base"))
    {
        let bad = args
            .named
            .keys()
            .find(|k| k.as_str() != "base")
            .map(|k| k.as_str())
            .unwrap_or("?");
        return err(format!("calc.log() argumento nomeado desconhecido: '{bad}'"));
    }

    let (x, base) = match args.items.as_slice() {
        [v] => {
            (coerce_to_f64(v, "calc.log() valor")?, base_from_named.unwrap_or(10.0_f64))
        }
        [v, b] => {
            if base_from_named.is_some() {
                return err(
                    "calc.log() não pode especificar base tanto posicional como nomeado"
                        .to_string(),
                );
            }
            (coerce_to_f64(v, "calc.log() valor")?, coerce_to_f64(b, "calc.log() base")?)
        }
        _ => {
            return err(format!(
                "calc.log() requer 1 ou 2 argumentos, recebeu {}",
                args.items.len()
            ))
        }
    };
    if x <= 0.0 {
        return err(format!(
            "calc.log() valor deve ser estritamente positivo, recebeu {x}"
        ));
    }
    let base = base_from_named.unwrap_or(base);
    if !base.is_finite() || base <= 0.0 || base == 1.0 {
        return err(format!("calc.log() base inválida: {base}"));
    }
    // P817-F — paridade vanilla (`calc.rs:506-515`): despacho por base
    // exacta. `ln(x)/ln(10)` dava `log(1000) = 2.9999999999999996`; o
    // vanilla despacha base 10 para `libm::log10` (resultado `3`).
    #[allow(clippy::disallowed_methods)]
    let r = if base == std::f64::consts::E {
        f64::ln(x)
    } else if base == 2.0 {
        f64::log2(x)
    } else if base == 10.0 {
        f64::log10(x)
    } else {
        f64::ln(x) / f64::ln(base)
    };
    guard_float(r)
}

/// `calc.log10(x)` — logaritmo base 10.
pub(crate) fn calc_log10(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [v] => {
            let x = coerce_to_f64(v, "calc.log10()")?;
            if x <= 0.0 {
                return err(format!(
                    "calc.log10() valor deve ser estritamente positivo, recebeu {x}"
                ));
            }
            #[allow(clippy::disallowed_methods)]
            guard_float(f64::log10(x))
        }
        _ => {
            err(format!("calc.log10() requer 1 argumento, recebeu {}", args.items.len()))
        }
    }
}

/// `calc.deg(rad)` — converte radianos para graus.
pub(crate) fn calc_deg(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    unary_f64("calc.deg", args, f64::to_degrees)
}

/// `calc.rad(deg)` — converte graus para radianos.
pub(crate) fn calc_rad(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    unary_f64("calc.rad", args, f64::to_radians)
}

#[cfg(test)]
mod tests {
    use super::super::calc_test_support::call;
    use super::*;

    #[test]
    fn preserves_modeled_result() {
        assert_eq!(call(calc_log, vec![Value::Int(100)]).unwrap(), Value::Float(2.0));
    }
}
