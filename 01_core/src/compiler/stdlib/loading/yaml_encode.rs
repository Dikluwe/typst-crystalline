//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/loading/yaml_encode.md
//! @prompt-hash 7f33c13c
//! @layer L1
//! @updated 2026-09-30

use super::encoding::{
    canonical_numeric_token, textual_content_fields, textual_encoder_args,
};
use super::*;

pub fn native_yaml_encode(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let (value, span, _) = textual_encoder_args(args, false, false)?;
    let mut output = String::new();
    write_yaml_value(&value, &mut output, 0).map_err(|error| {
        vec![SourceDiagnostic::error(
            span,
            format!("failed to encode value as YAML ({error})"),
        )]
    })?;
    Ok(Value::Str(output.into()))
}

/// YAML usa indentação de sequências sem nível extra em valores de mapas.
/// O writer recebe a primeira linha já posicionada pelo pai.
fn write_yaml_value(
    value: &Value,
    output: &mut String,
    indent: usize,
) -> Result<(), String> {
    match value {
        Value::Array(values) => {
            if values.is_empty() {
                output.push_str("[]\n");
            } else {
                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        output.push_str(&" ".repeat(indent));
                    }
                    output.push_str("- ");
                    write_yaml_value(value, output, indent + 2)?;
                }
            }
        }
        Value::Dict(values) => {
            if values.is_empty() {
                output.push_str("{}\n");
            } else {
                write_yaml_map(values, output, indent)?;
            }
        }
        Value::Content(_) | Value::LocatedContent(_, _) => {
            write_yaml_map(&textual_content_fields(value)?, output, indent)?;
        }
        Value::Str(value) => write_yaml_string(value, output, indent, false),
        Value::Symbol(value) => write_yaml_string(&value.value, output, indent, false),
        Value::None => output.push_str("null\n"),
        Value::Bool(value) => output.push_str(if *value { "true\n" } else { "false\n" }),
        Value::Int(value) => {
            output.push_str(&value.to_string());
            output.push('\n');
        }
        Value::Float(value) => {
            if value.is_nan() {
                output.push_str(".nan");
            } else if value.is_infinite() {
                output.push_str(if value.is_sign_negative() { "-.inf" } else { ".inf" });
            } else {
                output.push_str(&canonical_numeric_token(
                    serde_json::to_string(value).map_err(|error| error.to_string())?,
                ));
            }
            output.push('\n');
        }
        Value::Module(..)
        | Value::Datetime(..)
        | Value::Func(..)
        | Value::Auto
        | Value::Length(..)
        | Value::Relative(..)
        | Value::Ratio(..)
        | Value::Angle(..)
        | Value::Color(..)
        | Value::Stroke(..)
        | Value::Fraction(..)
        | Value::Align(..)
        | Value::Location(..)
        | Value::Gradient(..)
        | Value::Regex(..)
        | Value::Tiling(..)
        | Value::Bytes(..)
        | Value::Decimal(..)
        | Value::Duration(..)
        | Value::Version(..)
        | Value::Selector(..)
        | Value::Args(..)
        | Value::State(..)
        | Value::Counter(..)
        | Value::Label(..)
        | Value::Dir(..)
        | Value::Path(..)
        | Value::Type(..)
        | Value::SpotColorant(..) => {
            write_yaml_string(
                &crate::compiler::eval::repr_value_for_serialization(value),
                output,
                indent,
                false,
            );
        }
    }
    Ok(())
}

fn write_yaml_map(
    values: &Dict,
    output: &mut String,
    indent: usize,
) -> Result<(), String> {
    for (index, (name, value)) in values.iter().enumerate() {
        if index > 0 {
            output.push_str(&" ".repeat(indent));
        }
        write_yaml_string(name, output, indent, true);
        output.push(':');
        let nested_map = matches!(value, Value::Dict(values) if !values.is_empty())
            || matches!(value, Value::Content(_) | Value::LocatedContent(_, _));
        let nested_sequence = matches!(value, Value::Array(values) if !values.is_empty());
        if nested_map || nested_sequence {
            let next_indent = indent + if nested_map { 2 } else { 0 };
            output.push('\n');
            output.push_str(&" ".repeat(next_indent));
            write_yaml_value(value, output, next_indent)?;
        } else {
            output.push(' ');
            write_yaml_value(value, output, indent)?;
        }
    }
    Ok(())
}

fn yaml_needs_typed_quotes(value: &str) -> bool {
    let unsigned = value.strip_prefix(['+', '-']).unwrap_or(value);
    value.is_empty()
        || matches!(
            value,
            "null"
                | "Null"
                | "NULL"
                | "~"
                | "true"
                | "True"
                | "TRUE"
                | "false"
                | "False"
                | "FALSE"
                | ".nan"
                | ".NaN"
                | ".NAN"
                | ".inf"
                | ".Inf"
                | ".INF"
                | "-.inf"
                | "-.Inf"
                | "-.INF"
                | "+.inf"
                | "+.Inf"
                | "+.INF"
        )
        || (!value.trim().is_empty()
            && value.trim() == value
            && value.parse::<f64>().is_ok_and(f64::is_finite))
        || [("0x", 16), ("0o", 8), ("0b", 2)].iter().any(|(prefix, radix)| {
            unsigned.strip_prefix(prefix).is_some_and(|digits| {
                !digits.is_empty() && digits.chars().all(|c| c.is_digit(*radix))
            })
        })
}

fn yaml_token_boundary(character: char) -> bool {
    matches!(
        character,
        ' ' | '\t' | '\r' | '\n' | '\0' | '\u{85}' | '\u{2028}' | '\u{2029}'
    )
}

fn write_yaml_string(value: &str, output: &mut String, indent: usize, key: bool) {
    let special = value.chars().any(|character| matches!(character, '\0'..='\t' | '\u{b}'..='\u{1f}' | '\u{7f}'..='\u{9f}' | '\u{feff}' | '\u{fffe}' | '\u{ffff}'));
    let block = !key
        && value.contains('\n')
        && !special
        && !value.ends_with(' ')
        && !value.contains(" \n");
    if block {
        let trailing = value.bytes().rev().take_while(|byte| *byte == b'\n').count();
        output.push('|');
        if value.starts_with([' ', '\n']) {
            output.push('2');
        }
        if trailing == 0 {
            output.push('-');
        } else if trailing > 1 || value == "\n" {
            output.push('+');
        }
        output.push('\n');
        for line in value.split_terminator('\n') {
            if !line.is_empty() {
                output.push_str(&" ".repeat(indent + 2));
            }
            output.push_str(line);
            output.push('\n');
        }
        return;
    }
    let double = special || value.contains('\n');
    let first = value.chars().next();
    let indicator = first
        .is_some_and(|character| "#,[]{}&*!|>'\"%@`".contains(character))
        || value.starts_with("---")
        || value.starts_with("...")
        || first.is_some_and(|character| "?:-".contains(character))
            && value.chars().nth(1).is_none_or(yaml_token_boundary)
        || value.char_indices().any(|(index, character)| {
            character == ':'
                && value[index + 1..].chars().next().is_none_or(yaml_token_boundary)
                || character == '#'
                    && index > 0
                    && value[..index].chars().next_back().is_some_and(yaml_token_boundary)
        });
    let quote = double
        || yaml_needs_typed_quotes(value)
        || indicator
        || value.starts_with(' ')
        || value.ends_with(' ')
        || value.contains(['\u{2028}', '\u{2029}']);
    if double {
        output.push('"');
        for character in value.chars() {
            match character {
                '\0' => output.push_str("\\0"),
                '\u{7}' => output.push_str("\\a"),
                '\u{8}' => output.push_str("\\b"),
                '\t' => output.push_str("\\t"),
                '\n' => output.push_str("\\n"),
                '\u{b}' => output.push_str("\\v"),
                '\u{c}' => output.push_str("\\f"),
                '\r' => output.push_str("\\r"),
                '\u{1b}' => output.push_str("\\e"),
                '\u{85}' => output.push_str("\\N"),
                '"' => output.push_str("\\\""),
                '\\' => output.push_str("\\\\"),
                '\u{a0}' => output.push_str("\\_"),
                '\u{2028}' => output.push_str("\\L"),
                '\u{2029}' => output.push_str("\\P"),
                '\u{feff}' | '\u{fffe}' | '\u{ffff}' => {
                    output.push_str(&format!("\\u{:04X}", character as u32))
                }
                character if character.is_control() => {
                    output.push_str(&format!("\\x{:02X}", character as u32))
                }
                character => output.push(character),
            }
        }
        output.push('"');
    } else if quote {
        output.push('\'');
        for character in value.chars() {
            output.push(character);
            if character == '\'' {
                output.push('\'');
            }
            if matches!(character, '\u{2028}' | '\u{2029}') {
                output.push_str(&" ".repeat(indent + 2));
            }
        }
        output.push('\'');
    } else {
        output.push_str(value);
    }
    if !key {
        output.push('\n');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_yaml_scalar_string_is_quoted() {
        let mut output = String::new();
        write_yaml_value(&Value::Str("true".into()), &mut output, 0).unwrap();
        assert_ne!(output, "true\n");
        assert!(output.contains("true"));
    }
}

#[cfg(test)]
mod batch_f_yaml_tests {
    use super::*;
    fn encode(value: Value) -> String {
        let mut output = String::new();
        write_yaml_value(&value, &mut output, 0).unwrap();
        output
    }
    #[test]
    fn batch_f_yaml_empty_and_nested_containers_keep_shape() {
        assert_eq!(encode(Value::Array(vec![])), "[]\n");
        assert_eq!(encode(Value::Dict(new_dict())), "{}\n");
        let mut nested = new_dict();
        nested.insert(
            "a".into(),
            Value::Array(vec![
                Value::Array(vec![]),
                Value::Dict(new_dict()),
                Value::Array(vec![Value::Int(1), Value::Int(2)]),
            ]),
        );
        let mut inner = new_dict();
        inner.insert("c".into(), Value::Array(vec![]));
        nested.insert("b".into(), Value::Dict(inner));
        assert_eq!(
            encode(Value::Dict(nested)),
            "a:\n- []\n- {}\n- - 1\n  - 2\nb:\n  c: []\n"
        );
    }
    #[test]
    fn batch_f_yaml_scalar_tokens_and_opaque_repr_keep_newline() {
        for (value, expected) in [
            (Value::None, "null\n"),
            (Value::Bool(true), "true\n"),
            (Value::Bool(false), "false\n"),
            (Value::Int(-2), "-2\n"),
            (Value::Float(-0.0), "-0.0\n"),
            (Value::Float(f64::NAN), ".nan\n"),
            (Value::Float(f64::INFINITY), ".inf\n"),
            (Value::Float(f64::NEG_INFINITY), "-.inf\n"),
            (Value::Auto, "auto\n"),
            (
                Value::Bytes(crate::entities::bytes::Bytes::new(vec![65, 0, 66])),
                "bytes(3)\n",
            ),
        ] {
            assert_eq!(encode(value), expected);
        }
    }
    #[test]
    fn batch_f_yaml_strings_and_snapshot_keep_their_own_routes() {
        for (value, expected) in [
            ("true", "'true'\n"),
            ("é", "é\n"),
            ("a\nb", "|-\n  a\n  b\n"),
            ("a: b", "'a: b'\n"),
        ] {
            assert_eq!(encode(Value::Str(value.into())), expected);
        }
        let value = Value::LocatedContent(
            crate::entities::value::IntrospectedContent::new(
                crate::entities::content::Content::raw("x", None, false),
                Some(new_dict()),
            ),
            crate::entities::location::Location::from_raw(73),
        );
        assert_eq!(encode(value), "func: raw\n");
    }
}
