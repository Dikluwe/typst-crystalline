//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/loading/toml.md
//! @prompt-hash 845eb30c
//! @layer L1
//! @updated 2026-09-30

use super::*;

// ─────────────────────────────────────────────────────────────────────────────
// TOML — toml (preserve_order). Requer UTF-8.
// ─────────────────────────────────────────────────────────────────────────────

pub fn decode_toml(bytes: &[u8]) -> SourceResult<Value> {
    let s = std::str::from_utf8(bytes)
        .map_err(|_| err("toml: ficheiro não é UTF-8 válido"))?;
    let v: ::toml::Value =
        ::toml::from_str(s).map_err(|e| err(format!("toml inválido: {e}")))?;
    Ok(toml_to_value(v))
}

fn toml_to_value(v: ::toml::Value) -> Value {
    use ::toml::Value as T;
    match v {
        T::String(s) => Value::Str(s.into()),
        T::Integer(i) => Value::Int(i),
        T::Float(f) => Value::Float(f),
        T::Boolean(b) => Value::Bool(b),
        // graded (ADR-0054): mapa rico → Datetime deferido; RFC 3339 como Str.
        T::Datetime(dt) => Value::Str(dt.to_string().into()),
        T::Array(a) => Value::Array(a.into_iter().map(toml_to_value).collect()),
        T::Table(t) => {
            let mut d = new_dict();
            for (k, val) in t {
                d.insert(k.into(), toml_to_value(val));
            }
            Value::Dict(d)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_toml_table() {
        let Value::Dict(entries) = decode_toml(b"a = 1\n").unwrap() else {
            panic!("dictionary expected")
        };
        assert!(matches!(entries.get("a"), Some(Value::Int(1))));
    }
}
