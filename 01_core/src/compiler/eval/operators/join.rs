//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/operators/join.md
//! @prompt-hash 10d7581b
//! @layer L1
//! @updated 2026-08-12
//!
//!
//! `join` — combinação sequencial dos valores de um code block e dos corpos
//! de `for`/`while` (paridade `ops::join`, `foundations/ops.rs:24-45`).
//! A acumulação vive nos consumidores (`eval/mod.rs`, `control_flow.rs`).

// **P1017** — `vanilla_type_name` é a canónica em
// `eval/operators/error_formatting.rs`; a cópia privada que aqui existia foi
// removida.
use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::entities::args::Args;
use crate::entities::bytes::Bytes;
use crate::entities::content::Content;
use crate::entities::span::Span;
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
            Ok(Value::Str(format!("{}{}", a.value, b.value).into()))
        }
        (Value::Str(a), Value::Symbol(b)) => {
            Ok(Value::Str(format!("{}{}", a, b.value).into()))
        }
        (Value::Symbol(a), Value::Str(b)) => {
            Ok(Value::Str(format!("{}{}", a.value, b).into()))
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
                Content::text(b.value.to_string()),
            ])))
        }
        (Value::Symbol(a), Value::Content(b)) => {
            Ok(Value::Content(Content::sequence(vec![
                Content::text(a.value.to_string()),
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
        (Value::Args(a), Value::Args(b)) => {
            let mut left = a.occurrence_sequence();
            let right = b.occurrence_sequence();
            left.retain(|occurrence| {
                !occurrence.name.as_ref().is_some_and(|name| {
                    right.iter().any(|other| other.name.as_ref() == Some(name))
                })
            });
            left.extend(right);
            Ok(Value::Args(Args::from_occurrences(Span::detached(), left)))
        }
        (a, b) => Err(format!(
            "cannot join {} with {}",
            vanilla_type_name(&a),
            vanilla_type_name(&b)
        )),
    }
}
