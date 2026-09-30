//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/loading/toml_encode.md
//! @prompt-hash 60fda64a
//! @layer L1
//! @updated 2026-09-30

use super::encoding::{textual_content_fields, textual_encoder_args};
use super::*;

fn textual_toml_value(value: &Value) -> Result<::toml::Value, String> {
    use ::toml::Value as Toml;
    Ok(match value {
        Value::None => return Err("unsupported None value".into()),
        Value::Bool(value) => Toml::Boolean(*value),
        Value::Int(value) => Toml::Integer(*value),
        Value::Float(value) => Toml::Float(*value),
        Value::Str(value) => Toml::String(value.to_string()),
        Value::Symbol(value) => Toml::String(value.value.to_string()),
        Value::Array(values) => {
            Toml::Array(values.iter().map(textual_toml_value).collect::<Result<_, _>>()?)
        }
        Value::Dict(values) => Toml::Table(
            values
                .iter()
                .filter(|(_, value)| !matches!(value, Value::None))
                .map(|(name, value)| Ok((name.to_string(), textual_toml_value(value)?)))
                .collect::<Result<_, String>>()?,
        ),
        Value::Content(_) | Value::LocatedContent(_, _) => {
            textual_toml_value(&Value::Dict(textual_content_fields(value)?))?
        }
        other => Toml::String(crate::compiler::eval::repr_value_for_serialization(other)),
    })
}

/// A emissão ratificada não inclui `+` no expoente de um token numérico.
fn canonical_toml_strings(encoded: String) -> String {
    let mut output = String::with_capacity(encoded.len());
    let mut characters = encoded.chars().peekable();
    while let Some(quote) = characters.next() {
        output.push(quote);
        if !matches!(quote, '\'' | '"') {
            continue;
        }
        let multiline = characters.clone().take(2).eq([quote, quote]);
        if multiline {
            characters.next();
            characters.next();
            output.push(quote);
            output.push(quote);
        }
        while let Some(character) = characters.next() {
            if quote == '"' && character == '\\' {
                output.push(character);
                if let Some(escaped) = characters.next() {
                    output.push(escaped);
                }
            } else if character == quote {
                if !multiline {
                    output.push(quote);
                    break;
                }
                let mut count = 1;
                while characters.peek() == Some(&quote) {
                    characters.next();
                    count += 1;
                }
                let body_count = if count >= 3 { count - 3 } else { count };
                for _ in 0..body_count {
                    if quote == '"' {
                        output.push('\\');
                    }
                    output.push(quote);
                }
                if count >= 3 {
                    output.push(quote);
                    output.push(quote);
                    output.push(quote);
                    break;
                }
            } else {
                output.push(character);
            }
        }
    }
    output
}

pub fn native_toml_encode(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let (value, span, pretty) = textual_encoder_args(args, true, true)?;
    let encoded = textual_toml_value(&value).and_then(|value| {
        if pretty { ::toml::to_string_pretty(&value) } else { ::toml::to_string(&value) }
            .map(canonical_toml_strings)
            .map_err(|error| error.to_string())
    });
    encoded.map(|value| Value::Str(value.into())).map_err(|error| {
        vec![SourceDiagnostic::error(
            span,
            format!("failed to encode value as TOML ({error})"),
        )]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_in_toml_array_is_rejected() {
        let input = Value::Array(vec![Value::None]);
        assert_eq!(textual_toml_value(&input).unwrap_err(), "unsupported None value");
    }
}
