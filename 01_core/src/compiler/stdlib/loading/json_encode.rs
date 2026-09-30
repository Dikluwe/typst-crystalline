//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/loading/json_encode.md
//! @prompt-hash bda397fc
//! @layer L1
//! @updated 2026-09-30

use super::encoding::{
    canonical_numeric_token, textual_content_fields, textual_encoder_args,
};
use super::*;

fn textual_json_value(value: &Value) -> Result<serde_json::Value, String> {
    use serde_json::Value as Json;
    Ok(match value {
        Value::None => Json::Null,
        Value::Bool(value) => Json::Bool(*value),
        Value::Int(value) => Json::Number((*value).into()),
        Value::Float(value) => serde_json::Number::from_f64(*value)
            .map(Json::Number)
            .unwrap_or(Json::Null),
        Value::Str(value) => Json::String(value.to_string()),
        Value::Symbol(value) => Json::String(value.value.to_string()),
        Value::Array(values) => {
            Json::Array(values.iter().map(textual_json_value).collect::<Result<_, _>>()?)
        }
        Value::Dict(values) => Json::Object(
            values
                .iter()
                .map(|(name, value)| Ok((name.to_string(), textual_json_value(value)?)))
                .collect::<Result<_, String>>()?,
        ),
        Value::Content(_) | Value::LocatedContent(_, _) => {
            textual_json_value(&Value::Dict(textual_content_fields(value)?))?
        }
        other => Json::String(crate::compiler::eval::repr_value_for_serialization(other)),
    })
}

fn canonical_json_numbers(encoded: String) -> String {
    let mut output = String::with_capacity(encoded.len());
    let mut characters = encoded.chars().peekable();
    while let Some(character) = characters.next() {
        if character == '"' {
            output.push(character);
            while let Some(character) = characters.next() {
                output.push(character);
                if character == '\\' {
                    if let Some(escaped) = characters.next() {
                        output.push(escaped);
                    }
                } else if character == '"' {
                    break;
                }
            }
        } else if character.is_ascii_digit() || character == '-' {
            let mut token = String::from(character);
            while characters.peek().is_some_and(|character| {
                character.is_ascii_digit()
                    || matches!(character, '.' | 'e' | 'E' | '+' | '-')
            }) {
                token.push(characters.next().unwrap());
            }
            output.push_str(&canonical_numeric_token(token));
        } else {
            output.push(character);
        }
    }
    output
}

/// Compatibilidade lexical de strings TOML basic multilinha. O backend
/// decide layout e estilo; este passe escapa apenas aspas internas desse
/// estilo, sem alterar strings literais, escapes ou delimitadores.
pub fn native_json_encode(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let (value, span, pretty) = textual_encoder_args(args, false, true)?;
    let encoded = textual_json_value(&value).and_then(|value| {
        if pretty {
            serde_json::to_string_pretty(&value)
        } else {
            serde_json::to_string(&value)
        }
        .map(canonical_json_numbers)
        .map_err(|error| error.to_string())
    });
    encoded.map(|value| Value::Str(value.into())).map_err(|error| {
        vec![SourceDiagnostic::error(
            span,
            format!("failed to encode value as JSON ({error})"),
        )]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonicalizes_numeric_exponent_outside_json_strings() {
        assert_eq!(canonical_json_numbers("[1e+2,\"1e+2\"]".into()), "[1e2,\"1e+2\"]");
    }
}
