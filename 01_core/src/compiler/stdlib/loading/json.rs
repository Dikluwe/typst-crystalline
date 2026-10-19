//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/loading/json.md
//! @prompt-hash bc836d43
//! @layer L1
//! @updated 2026-09-30

use super::*;

// ─────────────────────────────────────────────────────────────────────────────
// JSON — serde_json (preserve_order: dict ordem-de-inserção)
// ─────────────────────────────────────────────────────────────────────────────

pub fn decode_json(bytes: &[u8]) -> SourceResult<Value> {
    let v: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|e| err(format!("json inválido: {e}")))?;
    Ok(json_to_value(v))
}

fn json_to_value(v: serde_json::Value) -> Value {
    use serde_json::Value as J;
    match v {
        J::Null => Value::None,
        J::Bool(b) => Value::Bool(b),
        J::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Int(i)
            } else {
                // f64 ou inteiro fora de i64 → Float (paridade: número grande vira float)
                Value::Float(n.as_f64().unwrap_or(f64::NAN))
            }
        }
        J::String(s) => Value::Str(s.into()),
        J::Array(a) => Value::Array(a.into_iter().map(json_to_value).collect()),
        J::Object(o) => {
            let mut d = new_dict();
            for (k, val) in o {
                d.insert(k.into(), json_to_value(val));
            }
            Value::Dict(d)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_ordered_json_array_and_null() {
        let Value::Array(items) = decode_json(br#"[1,null,true]"#).unwrap() else {
            panic!("array expected")
        };
        assert!(matches!(
            items.as_slice(),
            [Value::Int(1), Value::None, Value::Bool(true)]
        ));
    }
}
