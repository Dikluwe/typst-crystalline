//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/calc/numeric.md
//! @prompt-hash 77af80a6
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) fn coerce_to_f64(v: &Value, ctx: &str) -> SourceResult<f64> {
    match v {
        Value::Int(i) => Ok(*i as f64),
        Value::Float(f) => Ok(*f),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{ctx}: esperava Int ou Float, recebeu {}", other.type_name()),
        )]),
    }
}

pub(super) fn guard_float(f: f64) -> SourceResult<Value> {
    if f.is_nan() {
        err("resultado não é um número (NaN)")
    } else if f.is_infinite() {
        err("resultado é infinito")
    } else {
        Ok(Value::Float(f))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_modeled_result() {
        assert!(guard_float(f64::NAN).is_err());
        assert_eq!(guard_float(1.0).unwrap(), Value::Float(1.0));
    }
}
