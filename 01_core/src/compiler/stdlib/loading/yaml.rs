//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/loading/yaml.md
//! @prompt-hash b2044f46
//! @layer L1
//! @updated 2026-09-30

use super::*;

// ─────────────────────────────────────────────────────────────────────────────
// YAML — saphyr (mantido; mapa Yaml→Value manual). early_parse=true em
// `load_from_str` → escalares chegam como `Yaml::Value(Scalar)`.
// ─────────────────────────────────────────────────────────────────────────────

pub fn decode_yaml(bytes: &[u8]) -> SourceResult<Value> {
    let s = std::str::from_utf8(bytes)
        .map_err(|_| err("yaml: ficheiro não é UTF-8 válido"))?;
    let docs = <saphyr::Yaml as saphyr::LoadableYamlNode>::load_from_str(s)
        .map_err(|e| err(format!("yaml inválido: {e}")))?;
    match docs.len() {
        0 => Ok(Value::None),
        // documento único → o valor; múltiplos docs (`---`) → Array de docs.
        1 => yaml_to_value(&docs[0]),
        _ => {
            let mut out = Vec::with_capacity(docs.len());
            for d in &docs {
                out.push(yaml_to_value(d)?);
            }
            Ok(Value::Array(out))
        }
    }
}

fn yaml_to_value(y: &saphyr::Yaml) -> SourceResult<Value> {
    use saphyr::{Scalar, Yaml};
    match y {
        Yaml::Value(scalar) => Ok(match scalar {
            Scalar::Null => Value::None,
            Scalar::Boolean(b) => Value::Bool(*b),
            Scalar::Integer(i) => Value::Int(*i),
            Scalar::FloatingPoint(f) => Value::Float(f.into_inner()),
            Scalar::String(s) => Value::Str(s.as_ref().into()),
        }),
        Yaml::Sequence(seq) => {
            let mut out = Vec::with_capacity(seq.len());
            for item in seq {
                out.push(yaml_to_value(item)?);
            }
            Ok(Value::Array(out))
        }
        Yaml::Mapping(map) => {
            let mut d = new_dict();
            for (k, v) in map {
                let key = match yaml_to_value(k)? {
                    Value::Str(s) => s,
                    _ => return Err(err("yaml: chave de mapa não-string sem suporte")),
                };
                d.insert(key, yaml_to_value(v)?);
            }
            Ok(Value::Dict(d))
        }
        // Tagged: ignora a tag (graded ADR-0054), decodifica o interior.
        Yaml::Tagged(_, inner) => yaml_to_value(inner),
        Yaml::Representation(..) => Err(err("yaml: escalar não-resolvido (interno)")),
        Yaml::Alias(_) => Err(err("yaml: alias/anchor sem suporte (graded)")),
        Yaml::BadValue => Err(err("yaml: valor inválido")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_yaml_mapping() {
        let Value::Dict(entries) = decode_yaml(b"a: 1\n").unwrap() else {
            panic!("dictionary expected")
        };
        assert!(matches!(entries.get("a"), Some(Value::Int(1))));
    }
}
