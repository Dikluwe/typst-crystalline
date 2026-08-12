//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/operators/join.md
//! @prompt-hash 36c4df4e
//! @layer L1
//! @updated 2026-08-12
//!
//!
//! `join` — combinação sequencial dos valores de um code block e dos corpos
//! de `for`/`while` (paridade `ops::join`, `foundations/ops.rs:24-45`).
//! A acumulação vive nos consumidores (`eval/mod.rs`, `control_flow.rs`).

use crate::entities::bytes::Bytes;
use crate::entities::content::Content;
use crate::entities::value::Value;

/// **P728** — `join` de valores produzidos pelas expressões de um code
/// block (paridade vanilla `ops::join`, `foundations/ops.rs:24-45`).
/// `None` é identidade nos dois lados; texto/content/array/dict/args/bytes
/// concatenam ou fazem merge; qualquer outra combinação é erro de tipo
/// (`cannot join X with Y`, medido: `{ 1; 2 }` erra no vanilla).
pub(crate) fn join(lhs: Value, rhs: Value) -> Result<Value, String> {
    match (lhs, rhs) {
        (a, Value::None) => Ok(a),
        (Value::None, b) => Ok(b),
        (Value::Str(a), Value::Str(b)) => Ok(Value::Str(a + b.as_str())),
        (Value::Symbol(a), Value::Symbol(b)) => {
            Ok(Value::Str(format!("{}{}", a.ch, b.ch).into()))
        }
        (Value::Str(a), Value::Symbol(b)) => {
            Ok(Value::Str(format!("{}{}", a, b.ch).into()))
        }
        (Value::Symbol(a), Value::Str(b)) => {
            Ok(Value::Str(format!("{}{}", a.ch, b).into()))
        }
        (Value::Bytes(a), Value::Bytes(b)) => {
            let mut v = a.as_slice().to_vec();
            v.extend_from_slice(b.as_slice());
            Ok(Value::Bytes(Bytes::new(v)))
        }
        (Value::Content(a), Value::Content(b)) => {
            Ok(Value::Content(Content::sequence(vec![a, b])))
        }
        (Value::Content(a), Value::Str(b)) => {
            Ok(Value::Content(Content::sequence(vec![a, Content::text(b)])))
        }
        (Value::Str(a), Value::Content(b)) => {
            Ok(Value::Content(Content::sequence(vec![Content::text(a), b])))
        }
        (Value::Content(a), Value::Symbol(b)) => {
            Ok(Value::Content(Content::sequence(vec![
                a,
                Content::text(b.ch.to_string()),
            ])))
        }
        (Value::Symbol(a), Value::Content(b)) => {
            Ok(Value::Content(Content::sequence(vec![
                Content::text(a.ch.to_string()),
                b,
            ])))
        }
        (Value::Array(mut a), Value::Array(b)) => {
            a.extend(b);
            Ok(Value::Array(a))
        }
        (Value::Dict(mut a), Value::Dict(b)) => {
            a.extend(b);
            Ok(Value::Dict(a))
        }
        (Value::Args(mut a), Value::Args(b)) => {
            a.items.extend(b.items);
            a.named.extend(b.named);
            Ok(Value::Args(a))
        }
        (a, b) => Err(format!(
            "cannot join {} with {}",
            long_type_name(&a),
            long_type_name(&b)
        )),
    }
}

/// **P843 (#60)** — nome longo do tipo nas mensagens de erro (paridade
/// vanilla `Type::long_name`, usada por `mismatch!`): difere do nome curto
/// só em int/str/bool. Medido: `(1, 2).join("-")` no vanilla →
/// "cannot join integer with string".
fn long_type_name(v: &Value) -> &'static str {
    match v.type_name() {
        "int" => "integer",
        "str" => "string",
        "bool" => "boolean",
        other => other,
    }
}
