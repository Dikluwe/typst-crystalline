//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/calc/extrema.md
//! @prompt-hash c18d7c18
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(crate) fn calc_min(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    if args.items.is_empty() {
        return err("calc.min() requer pelo menos 1 argumento");
    }
    let mut result = args.items[0].clone();
    for v in &args.items[1..] {
        result = match (&result, v) {
            (Value::Int(a), Value::Int(b)) => Value::Int(*a.min(b)),
            (Value::Float(a), Value::Float(b)) => Value::Float(a.min(*b)),
            (Value::Int(a), Value::Float(b)) => Value::Float((*a as f64).min(*b)),
            (Value::Float(a), Value::Int(b)) => Value::Float(a.min(*b as f64)),
            (_, other) => {
                return err(format!(
                    "calc.min() tipos incompatíveis: {} e {}",
                    result.type_name(),
                    other.type_name()
                ))
            }
        };
    }
    Ok(result)
}

pub(crate) fn calc_max(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    if args.items.is_empty() {
        return err("calc.max() requer pelo menos 1 argumento");
    }
    let mut result = args.items[0].clone();
    for v in &args.items[1..] {
        result = match (&result, v) {
            (Value::Int(a), Value::Int(b)) => Value::Int(*a.max(b)),
            (Value::Float(a), Value::Float(b)) => Value::Float(a.max(*b)),
            (Value::Int(a), Value::Float(b)) => Value::Float((*a as f64).max(*b)),
            (Value::Float(a), Value::Int(b)) => Value::Float(a.max(*b as f64)),
            (_, other) => {
                return err(format!(
                    "calc.max() tipos incompatíveis: {} e {}",
                    result.type_name(),
                    other.type_name()
                ))
            }
        };
    }
    Ok(result)
}

pub(crate) fn calc_clamp(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Int(v), Value::Int(lo), Value::Int(hi)] => {
            Ok(Value::Int((*v).clamp(*lo, *hi)))
        }
        [v, lo, hi] => {
            let vf = coerce_to_f64(v, "calc.clamp() value")?;
            let lof = coerce_to_f64(lo, "calc.clamp() min")?;
            let hif = coerce_to_f64(hi, "calc.clamp() max")?;
            if lof > hif {
                return err(format!("calc.clamp() min ({lof}) > max ({hif})"));
            }
            Ok(Value::Float(vf.clamp(lof, hif)))
        }
        _ => {
            err(format!("calc.clamp() requer 3 argumentos, recebeu {}", args.items.len()))
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
            call(calc_clamp, vec![Value::Int(9), Value::Int(2), Value::Int(5)]).unwrap(),
            Value::Int(5)
        );
    }
}
