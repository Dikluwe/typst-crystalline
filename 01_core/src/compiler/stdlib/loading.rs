//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/loading.md
//! @prompt-hash d23f175e
//! @layer L1
//! @updated 2026-06-21
//!
//! Módulo `loading` — carregamento de dados (`read`/`csv`/`json`/`yaml`/`toml`/
//! `cbor`/`xml`). Achado da Lista B do Passo 386; materializado no Passo 387.
//!
//! Arquitetura (ADR-0029 + ADR-0111): o **decode é L1 puro** — `decode_X(bytes,
//! opts) -> SourceResult<Value>`, zero I/O, testável com bytes literais. A
//! **leitura** reusa o L3 já existente `World::read_bytes`. As `native_X`
//! compõem os dois estratos. Paridade é com o `Value` de saída (ADR-0107), não
//! com a mecânica do parser.

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::bytes::Bytes;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

type Dict = IndexMap<EcoString, Value, FxBuildHasher>;

fn err(msg: impl Into<String>) -> Vec<SourceDiagnostic> {
    vec![SourceDiagnostic::error(Span::detached(), msg.into())]
}

fn new_dict() -> Dict {
    IndexMap::with_hasher(FxBuildHasher)
}

/// Adapters privados usam somente as árvores dos formatos autorizados em L1.
/// CBOR conserva a sua classificação própria, inclusive Bytes binário.
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

fn textual_toml_value(value: &Value) -> Result<toml::Value, String> {
    use toml::Value as Toml;
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
fn canonical_numeric_token(mut token: String) -> String {
    if let Some(exponent) = token.find("e+") {
        token.remove(exponent + 1);
    }
    token
}

/// Reescreve somente tokens numéricos; strings JSON, inclusive escapes e
/// texto semelhante a números, são copiadas sem interpretação.
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

fn textual_content_fields(value: &Value) -> Result<Dict, String> {
    use crate::entities::content::Content;
    let (content, snapshot) = match value {
        Value::Content(content) => (content, None),
        Value::LocatedContent(content, _) => (content.content(), content.fields()),
        _ => return Err("expected content".into()),
    };
    let mut fields = new_dict();
    let function = match content {
        Content::Empty => "sequence",
        other => other.elem_name(),
    };
    fields.insert("func".into(), Value::Str(function.into()));
    if let Some(snapshot) = snapshot {
        fields.extend(snapshot.iter().map(|(name, value)| (name.clone(), value.clone())));
    } else {
        // Campos de payload cuja presença é estrutural. O helper legado de
        // métodos só enumera body para essas famílias e não os pode recuperar.
        match content {
            Content::Empty => {
                fields.insert("children".into(), Value::Array(vec![]));
                return Ok(fields);
            }
            Content::Sequence(children) => {
                fields.insert(
                    "children".into(),
                    Value::Array(children.iter().cloned().map(Value::Content).collect()),
                );
                return Ok(fields);
            }
            Content::Metadata(metadata) => {
                fields.insert("value".into(), metadata.value.as_ref().clone());
                return Ok(fields);
            }
            Content::Styled(child, _) => {
                fields.insert("child".into(), Value::Content(child.as_ref().clone()));
                fields.insert("styles".into(), Value::Str("styles(..)".into()));
                return Ok(fields);
            }
            Content::Raw(raw_elem) => {
                fields.insert("text".into(), Value::Str(raw_elem.text.clone()));
                if let Some(language) = &raw_elem.lang {
                    fields.insert("lang".into(), Value::Str(language.clone()));
                }
                // O bool block legado não distingue default de presença.
                return Ok(fields);
            }
            Content::Link(link) => {
                fields.insert("dest".into(), Value::Str(link.url.clone()));
                fields.insert("body".into(), Value::Content(link.body.clone()));
                return Ok(fields);
            }
            Content::Boxed(boxed) => {
                fields.insert("body".into(), Value::Content(boxed.body.clone()));
                return Ok(fields);
            }
            Content::Block(block) => {
                fields.insert("body".into(), Value::Content(block.body.clone()));
                return Ok(fields);
            }
            Content::HtmlElem(element) => {
                fields.insert("tag".into(), Value::Str(element.tag.clone()));
                if let Some(attributes) = &element.attrs {
                    fields.insert(
                        "attrs".into(),
                        Value::Dict(
                            attributes
                                .iter()
                                .map(|(key, value)| {
                                    (key.clone(), Value::Str(value.clone()))
                                })
                                .collect(),
                        ),
                    );
                }
                match &element.body {
                    crate::entities::html::HtmlBody::Unset => {}
                    crate::entities::html::HtmlBody::None => {
                        fields.insert("body".into(), Value::None);
                    }
                    crate::entities::html::HtmlBody::Content(body) => {
                        fields
                            .insert("body".into(), Value::Content(body.as_ref().clone()));
                    }
                }
                return Ok(fields);
            }
            _ => {}
        }
        let projected = crate::compiler::eval::bindings::eval_content_method(
            content,
            "fields",
            Args::positional(vec![]),
            Span::detached(),
        )
        .map_err(|diagnostics| {
            diagnostics
                .into_iter()
                .map(|diagnostic| diagnostic.message)
                .collect::<Vec<_>>()
                .join("; ")
        })?;
        let Value::Dict(projected) = projected else {
            return Err("content fields must be a dictionary".into());
        };
        fields.extend(projected);
    }
    Ok(fields)
}

/// Casts seguem a ordem dos parâmetros do contrato: value, todos os pretty,
/// e então o primeiro argumento remanescente na sequência causal.
fn textual_encoder_args(
    args: &Args,
    toml: bool,
    allow_pretty: bool,
) -> SourceResult<(Value, Span, bool)> {
    let mut occurrences = args.occurrence_sequence();
    let Some(index) = occurrences.iter().position(|argument| argument.name.is_none())
    else {
        if let Some(argument) = occurrences
            .iter()
            .find(|argument| argument.name.as_deref() == Some("value"))
        {
            return Err(vec![SourceDiagnostic::error(
                argument.span,
                "the argument `value` is positional",
            )
            .with_hint("try removing `value:`")]);
        }
        return Err(vec![SourceDiagnostic::error(args.span, "missing argument: value")]);
    };
    let argument = occurrences.remove(index);
    if toml && !matches!(argument.value, Value::Dict(_)) {
        return Err(vec![SourceDiagnostic::error(
            argument.value_span,
            format!("expected dictionary, found {}", vanilla_type_name(&argument.value)),
        )]);
    }
    let mut pretty = true;
    if allow_pretty {
        for occurrence in &occurrences {
            if occurrence.name.as_deref() == Some("pretty") {
                match &occurrence.value {
                    Value::Bool(value) => pretty = *value,
                    value => {
                        return Err(vec![SourceDiagnostic::error(
                            occurrence.value_span,
                            format!(
                                "expected boolean, found {}",
                                vanilla_type_name(value)
                            ),
                        )])
                    }
                }
            }
        }
        occurrences.retain(|occurrence| occurrence.name.as_deref() != Some("pretty"));
    }
    if let Some(occurrence) = occurrences.first() {
        let message = match &occurrence.name {
            Some(name) => format!("unexpected argument: {name}"),
            None => "unexpected argument".into(),
        };
        return Err(vec![SourceDiagnostic::error(occurrence.span, message)]);
    }
    Ok((argument.value, argument.value_span, pretty))
}

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

pub fn native_toml_encode(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let (value, span, pretty) = textual_encoder_args(args, true, true)?;
    let encoded = textual_toml_value(&value).and_then(|value| {
        if pretty { toml::to_string_pretty(&value) } else { toml::to_string(&value) }
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
        Value::Array(values) if !values.is_empty() => {
            for (index, value) in values.iter().enumerate() {
                if index > 0 {
                    output.push_str(&" ".repeat(indent));
                }
                output.push_str("- ");
                write_yaml_value(value, output, indent + 2)?;
            }
        }
        Value::Dict(values) if !values.is_empty() => {
            write_yaml_map(values, output, indent)?
        }
        Value::Content(_) | Value::LocatedContent(_, _) => {
            write_yaml_map(&textual_content_fields(value)?, output, indent)?;
        }
        Value::Str(value) => write_yaml_string(value, output, indent, false),
        Value::Symbol(value) => write_yaml_string(&value.value, output, indent, false),
        _ => {
            match value {
                Value::None => output.push_str("null"),
                Value::Bool(value) => {
                    output.push_str(if *value { "true" } else { "false" })
                }
                Value::Int(value) => output.push_str(&value.to_string()),
                Value::Float(value) if value.is_nan() => output.push_str(".nan"),
                Value::Float(value) if value.is_infinite() => output
                    .push_str(if value.is_sign_negative() { "-.inf" } else { ".inf" }),
                Value::Float(value) => output.push_str(&canonical_numeric_token(
                    serde_json::to_string(value).map_err(|error| error.to_string())?,
                )),
                Value::Array(_) => output.push_str("[]"),
                Value::Dict(_) => output.push_str("{}"),
                other => {
                    write_yaml_string(
                        &crate::compiler::eval::repr_value_for_serialization(other),
                        output,
                        indent,
                        false,
                    );
                    return Ok(());
                }
            }
            output.push('\n');
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

// ─────────────────────────────────────────────────────────────────────────────
// TOML — toml (preserve_order). Requer UTF-8.
// ─────────────────────────────────────────────────────────────────────────────

pub fn decode_toml(bytes: &[u8]) -> SourceResult<Value> {
    let s = std::str::from_utf8(bytes)
        .map_err(|_| err("toml: ficheiro não é UTF-8 válido"))?;
    let v: toml::Value =
        toml::from_str(s).map_err(|e| err(format!("toml inválido: {e}")))?;
    Ok(toml_to_value(v))
}

fn toml_to_value(v: toml::Value) -> Value {
    use toml::Value as T;
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

// ─────────────────────────────────────────────────────────────────────────────
// CBOR — ciborium. byte-strings → graded Err (Value::Bytes ausente, DEBT-62).
// ─────────────────────────────────────────────────────────────────────────────

pub fn decode_cbor(bytes: &[u8]) -> SourceResult<Value> {
    decode_cbor_with_source(bytes, None)
}

/// `path`: caminho da fonte (quando veio de ficheiro) para o sufixo
/// ` in {ficheiro}` da mensagem — paridade vanilla `load_err_in_binary`
/// (`diag.rs`): fonte path ganha o sufixo, fonte bytes fica sem ele.
fn decode_cbor_with_source(bytes: &[u8], path: Option<&str>) -> SourceResult<Value> {
    let v: ciborium::value::Value =
        ciborium::from_reader(bytes).map_err(|e| match path {
            Some(p) => {
                err(format!("failed to parse CBOR ({} in {p})", cbor_error_reason(&e)))
            }
            None => err(format!("failed to parse CBOR ({})", cbor_error_reason(&e))),
        })?;
    cbor_to_value(v)
}

/// Razão do erro no formato do vanilla `format_cbor_error`
/// (`loading/cbor.rs:88-98`): o `Display` do `ciborium::de::Error` delega em
/// `Debug`, por isso a razão extrai-se por variante (P823 — antes o cristalino
/// expunha esse `Debug`, ex.: `Semantic(None, "...")`).
fn cbor_error_reason(e: &ciborium::de::Error<std::io::Error>) -> String {
    use ciborium::de::Error;
    match e {
        Error::Io(e) => format!("IO error: {e}"),
        Error::Syntax(_) => "syntax error".to_string(),
        Error::Semantic(_, s) => s.clone(),
        Error::RecursionLimitExceeded => "recursion limit exceeded".to_string(),
    }
}

fn cbor_to_value(v: ciborium::value::Value) -> SourceResult<Value> {
    use ciborium::value::Value as C;
    match v {
        C::Null => Ok(Value::None),
        C::Bool(b) => Ok(Value::Bool(b)),
        C::Integer(i) => {
            let n: i128 = i.into();
            i64::try_from(n)
                .map(Value::Int)
                .map_err(|_| err("cbor: inteiro fora de i64"))
        }
        C::Float(f) => Ok(Value::Float(f)),
        C::Text(s) => Ok(Value::Str(s.into())),
        // P398: byte-strings CBOR → Value::Bytes (fecha DEBT-62).
        C::Bytes(b) => Ok(Value::Bytes(Bytes::new(b))),

        C::Tag(_, inner) => cbor_to_value(*inner),
        C::Array(a) => {
            let mut out = Vec::with_capacity(a.len());
            for item in a {
                out.push(cbor_to_value(item)?);
            }
            Ok(Value::Array(out))
        }
        C::Map(m) => {
            let mut d = new_dict();
            for (k, val) in m {
                let key = match cbor_to_value(k)? {
                    Value::Str(s) => s,
                    _ => return Err(err("cbor: chave de mapa não-string sem suporte")),
                };
                d.insert(key, cbor_to_value(val)?);
            }
            Ok(Value::Dict(d))
        }
        _ => Err(err("cbor: variante sem suporte")),
    }
}

/// P701 — `cbor.encode(value)`: `Value` → bytes CBOR. Direcção inversa de
/// `cbor_to_value`. Paridade vanilla (`foundations/value.rs:345-366`,
/// `foundations/bytes.rs:364-374`): `None/Bool/Int/Float/Str/Bytes/Array/Dict`
/// mapeiam 1:1; tudo o resto (inclui `Symbol`/`Content`, que o vanilla
/// serializa de forma dedicada) cai no fallback `Text(repr(v))` — ver
/// `loading.md` §3.4 para a divergência documentada.
fn value_to_cbor(v: &Value) -> ciborium::value::Value {
    use ciborium::value::Value as C;
    match v {
        Value::None => C::Null,
        Value::Bool(b) => C::Bool(*b),
        Value::Int(i) => C::Integer((*i).into()),
        Value::Float(f) => C::Float(*f),
        Value::Str(s) => C::Text(s.to_string()),
        Value::Bytes(b) => C::Bytes(b.as_slice().to_vec()),
        Value::Array(a) => C::Array(a.iter().map(value_to_cbor).collect()),
        Value::Dict(d) => C::Map(
            d.iter()
                .map(|(k, val)| (C::Text(k.to_string()), value_to_cbor(val)))
                .collect(),
        ),
        other => C::Text(crate::compiler::eval::repr::repr_value(other)),
    }
}

/// `cbor.encode(value)` — acedido por field access em `cbor` (namespace,
/// P701; registo em `rules/eval/mod.rs`).
pub fn native_cbor_encode(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    reject_named(args, "cbor.encode")?;
    match args.items.as_slice() {
        [v] => {
            let cbor_val = value_to_cbor(v);
            let mut buf = Vec::new();
            ciborium::into_writer(&cbor_val, &mut buf)
                .map_err(|e| err(format!("cbor.encode(): falha ao codificar: {e}")))?;
            Ok(Value::Bytes(Bytes::new(buf)))
        }
        _ => Err(err(format!(
            "cbor.encode() requer 1 argumento, recebeu {}",
            args.items.len()
        ))),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// XML — roxmltree (read-only). Cada elemento → Dict{tag, attrs, children}.
// ─────────────────────────────────────────────────────────────────────────────

pub fn decode_xml(bytes: &[u8]) -> SourceResult<Value> {
    let s = std::str::from_utf8(bytes)
        .map_err(|_| err("xml: ficheiro não é UTF-8 válido"))?;
    let doc =
        roxmltree::Document::parse(s).map_err(|e| err(format!("xml inválido: {e}")))?;
    // Topo: o elemento-raiz como nó único, num Array (paridade `convert_xml`).
    Ok(Value::Array(vec![xml_node_to_value(doc.root_element())]))
}

fn xml_node_to_value(node: roxmltree::Node) -> Value {
    let mut d = new_dict();
    d.insert("tag".into(), Value::Str(node.tag_name().name().into()));

    let mut attrs = new_dict();
    for a in node.attributes() {
        attrs.insert(a.name().into(), Value::Str(a.value().into()));
    }
    d.insert("attrs".into(), Value::Dict(attrs));

    let mut children = Vec::new();
    for child in node.children() {
        if child.is_element() {
            children.push(xml_node_to_value(child));
        } else if child.is_text() {
            if let Some(t) = child.text() {
                children.push(Value::Str(t.into()));
            }
        }
    }
    d.insert("children".into(), Value::Array(children));
    Value::Dict(d)
}

// ─────────────────────────────────────────────────────────────────────────────
// CSV — csv. Array 2D; row-type array (default) ou dictionary.
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
pub enum RowType {
    Array,
    Dictionary,
}

pub fn decode_csv(bytes: &[u8], delimiter: u8, row_type: RowType) -> SourceResult<Value> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(false)
        // P787 — sem `.flexible(true)`: linhas com nº de campos divergente
        // são rejeitadas (paridade vanilla; antes aceites em silêncio, P786 B1).
        .from_reader(bytes);

    // P787 — mapeamento para o formato exacto do vanilla
    // (`loading/csv.rs:138-156`, `format_csv_error`): UnequalLengths →
    // "found {len} instead of {expected_len} fields in line {line}" com a
    // linha do `Position` do próprio erro (não inventada).
    fn map_csv_err(e: csv::Error) -> Vec<SourceDiagnostic> {
        match e.kind() {
            csv::ErrorKind::UnequalLengths { expected_len, len, .. } => {
                let line = e.position().map(|p| p.line()).unwrap_or(0);
                err(format!(
                    "failed to parse CSV (found {len} instead of {expected_len} fields in line {line})"
                ))
            }
            _ => err(format!("failed to parse CSV ({e})")),
        }
    }

    let mut records = reader.records();

    // row-type dictionary: a 1ª linha são as chaves (paridade vanilla).
    let header: Option<Vec<EcoString>> = if row_type == RowType::Dictionary {
        match records.next() {
            Some(r) => {
                let r = r.map_err(map_csv_err)?;
                Some(r.iter().map(EcoString::from).collect())
            }
            None => None,
        }
    } else {
        None
    };

    let mut rows = Vec::new();
    for rec in records {
        let rec = rec.map_err(map_csv_err)?;
        match &header {
            None => {
                rows.push(Value::Array(
                    rec.iter().map(|c| Value::Str(c.into())).collect(),
                ));
            }
            Some(keys) => {
                let mut d = new_dict();
                for (i, cell) in rec.iter().enumerate() {
                    let key = keys
                        .get(i)
                        .cloned()
                        .unwrap_or_else(|| EcoString::from(i.to_string()));
                    d.insert(key, Value::Str(cell.into()));
                }
                rows.push(Value::Dict(d));
            }
        }
    }
    Ok(Value::Array(rows))
}

// ─────────────────────────────────────────────────────────────────────────────
// Funções nativas — compõem L3 (read_bytes) + L1 (decode). ABI per ADR-0044:
// (ctx, args, world, current_file). Ver `figure_image::native_image`.
// ─────────────────────────────────────────────────────────────────────────────

fn arg_path<'a>(args: &'a Args, fname: &str) -> SourceResult<&'a Value> {
    match args.items.first() {
        Some(value @ (Value::Str(_) | Value::Path(_))) => Ok(value),
        Some(other) => Err(err(format!(
            "{fname}() requer string com o caminho, recebeu {}",
            other.type_name()
        ))),
        None => Err(err(format!("{fname}() requer 1 argumento posicional (caminho)"))),
    }
}

fn reject_named(args: &Args, fname: &str) -> SourceResult<()> {
    if let Some(k) = args.named.keys().next() {
        return Err(err(format!("argumento nomeado inesperado em {fname}(): '{k}'")));
    }
    Ok(())
}

fn read_bytes(
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
    path: &Value,
    fname: &str,
) -> SourceResult<(String, std::sync::Arc<Vec<u8>>)> {
    crate::compiler::stdlib::read_path_value(path, world, current_file)
        .map(|(path, bytes)| (path.vpath().get_with_slash().to_string(), bytes))
        .map_err(|msg| err(format!("{fname}(): não foi possível ler: {msg}")))
}

/// P701 — resolve o 1º posicional de `json`/`yaml`/`toml`/`cbor`/`xml` como
/// **caminho (`Str`, lido via `World::read_bytes`) ou dados crus (`Bytes`,
/// usados directamente, sem I/O)**. Paridade vanilla: estas 5 funções usam
/// `DataSource` (`Str | Bytes`) no vanilla — a mesma dualidade aqui.
fn resolve_data(
    args: &Args,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
    fname: &str,
) -> SourceResult<std::sync::Arc<Vec<u8>>> {
    match args.items.first() {
        Some(value @ (Value::Str(_) | Value::Path(_))) => {
            read_bytes(world, current_file, value, fname).map(|(_, bytes)| bytes)
        }
        Some(Value::Bytes(b)) => Ok(std::sync::Arc::new(b.as_slice().to_vec())),
        Some(other) => Err(err(format!(
            "{fname}() requer caminho (str) ou bytes, recebeu {}",
            other.type_name()
        ))),
        None => Err(err(format!(
            "{fname}() requer 1 argumento posicional (caminho ou bytes)"
        ))),
    }
}

/// `read(path, encoding: "utf8" | none)` → `Str` (utf8, default) ou `Bytes`
/// (`encoding: none`). Ficheiro não-UTF8 sem `encoding: none` → **erro**
/// (paridade vanilla medida em P824: `read.rs:24-47` + `diag.rs:797-807`).
/// O comentário que aqui existia ("heurística vanilla: tenta UTF-8; se
/// falhar, retorna bytes opacos") estava **refutado por medição** (P810 §11)
/// e foi removido — o vanilla não tem essa heurística.
pub fn native_read(
    _ctx: &mut EvalContext,
    args: &Args,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
) -> SourceResult<Value> {
    for k in args.named.keys() {
        if k.as_str() != "encoding" {
            return Err(err(format!("argumento nomeado inesperado em read(): '{k}'")));
        }
    }
    let path_value = arg_path(args, "read")?;
    let (path, data) = read_bytes(world, current_file, path_value, "read")?;
    match args.named.get("encoding") {
        None => read_utf8(&data, &path),
        Some(Value::None) => Ok(Value::Bytes(Bytes::new(data.to_vec()))),
        Some(Value::Str(s)) if s.as_str() == "utf8" => read_utf8(&data, &path),
        // Cast do vanilla de `Option<Encoding>`: string fora do domínio →
        // sem sufixo "found"; outro tipo → `, found {tipo}` (medido em P824).
        Some(Value::Str(_)) => Err(err("expected \"utf8\" or none")),
        Some(other) => Err(err(format!(
            "expected \"utf8\" or none, found {}",
            vanilla_type_name(other)
        ))),
    }
}

/// UTF-8 → `Str`; inválido → erro no formato do vanilla
/// (`diag.rs:797-807` + `load_err_in_binary`): posição `:{linha}:{col}` do
/// byte problemático via `LineCol::try_from_byte_pos` (`diag.rs:1027-1039`).
fn read_utf8(data: &[u8], path: &str) -> SourceResult<Value> {
    match std::str::from_utf8(data) {
        Ok(text) => Ok(Value::Str(text.into())),
        Err(e) => {
            let (line, col) = vanilla_line_col(&data[..e.valid_up_to()]);
            Err(err(format!(
                "failed to convert to string (file is not valid UTF-8 in {path}:{line}:{col})"
            )))
        }
    }
}

/// (linha, coluna) 1-based do fim de `prefix`, replicando
/// `LineCol::try_from_byte_pos` + `numbers()`: linha = nº de `'\n'`; coluna =
/// nº de chars desde o último `'\n'` — **1 quando não há `'\n'`** (quirk do
/// vanilla: `unwrap_or(bytes.len())` faz a coluna contar do fim do prefixo).
fn vanilla_line_col(prefix: &[u8]) -> (usize, usize) {
    let line = prefix.iter().filter(|&&b| b == b'\n').count();
    let line_start = prefix
        .iter()
        .rposition(|&b| b == b'\n')
        .map(|i| i + 1)
        .unwrap_or(prefix.len());
    // `prefix` é UTF-8 válido (prefixo até `valid_up_to`).
    let col = std::str::from_utf8(&prefix[line_start..])
        .map(|s| s.chars().count())
        .unwrap_or(0);
    (line + 1, col + 1)
}

macro_rules! native_loader {
    ($fn_name:ident, $name:literal, $decode:expr) => {
        pub fn $fn_name(
            _ctx: &mut EvalContext,
            args: &Args,
            world: &dyn crate::contracts::world::World,
            current_file: FileId,
        ) -> SourceResult<Value> {
            reject_named(args, $name)?;
            let data = resolve_data(args, world, current_file, $name)?;
            $decode(&data[..])
        }
    };
}

native_loader!(native_json, "json", decode_json);
native_loader!(native_yaml, "yaml", decode_yaml);
native_loader!(native_toml, "toml", decode_toml);
native_loader!(native_xml, "xml", decode_xml);

/// `cbor(path | bytes)` — fora da macro `native_loader!` (P823): a mensagem
/// de erro precisa de saber se a fonte foi um caminho, para o sufixo
/// ` in {ficheiro}` do vanilla.
pub fn native_cbor(
    _ctx: &mut EvalContext,
    args: &Args,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
) -> SourceResult<Value> {
    reject_named(args, "cbor")?;
    let path = match args.items.first() {
        Some(Value::Str(s)) => Some(s.to_string()),
        _ => None,
    };
    let data = resolve_data(args, world, current_file, "cbor")?;
    decode_cbor_with_source(&data[..], path.as_deref())
}

/// `csv(path, delimiter: ",", row-type: array)`. Aceita named `delimiter`
/// (Str de 1 char ASCII) e `row-type` (**tipo** `array`/`dictionary` —
/// API vanilla medida em P787; a forma string é rejeitada).
/// Subset graded: sem `escape`/`encoding` cosméticos.
pub fn native_csv(
    _ctx: &mut EvalContext,
    args: &Args,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
) -> SourceResult<Value> {
    for k in args.named.keys() {
        if k.as_str() != "delimiter" && k.as_str() != "row-type" {
            return Err(err(format!("argumento nomeado inesperado em csv(): '{k}'")));
        }
    }
    let path = arg_path(args, "csv")?;

    let delimiter = match args.named.get("delimiter") {
        None => b',',
        Some(Value::Str(s)) => {
            // P787 — casts do vanilla (`loading/csv.rs:103-111`): 1 char ≠ →
            // "expected exactly one character"; não-ASCII → "delimiter must
            // be an ASCII character" (a mensagem anterior culpava o
            // comprimento, que estava certo — P786 D2).
            let mut chars = s.chars();
            match (chars.next(), chars.next()) {
                (Some(c), None) if c.is_ascii() => c as u8,
                (Some(_), None) => {
                    return Err(err("delimiter must be an ASCII character"))
                }
                _ => return Err(err("expected exactly one character")),
            }
        }
        Some(other) => {
            return Err(err(format!(
                "expected string, found {}",
                vanilla_type_name(other)
            )))
        }
    };

    let row_type = match args.named.get("row-type") {
        None => RowType::Array,
        // P787 — API vanilla: `row-type` recebe o TIPO (`dictionary`/`array`
        // como `Value::Type`), não a string (P786 D4 — divergência nos dois
        // sentidos). Tipo errado → "expected `array` or `dictionary`";
        // valor que não é tipo → "expected type, found ...".
        Some(Value::Type(t)) => match t {
            crate::entities::value::Type::Array => RowType::Array,
            crate::entities::value::Type::Dictionary => RowType::Dictionary,
            _ => return Err(err("expected `array` or `dictionary`")),
        },
        Some(other) => {
            return Err(err(format!("expected type, found {}", vanilla_type_name(other))))
        }
    };

    let (_, data) = read_bytes(world, current_file, path, "csv")?;
    decode_csv(&data[..], delimiter, row_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dict_of(pairs: Vec<(&str, Value)>) -> Value {
        let mut d = new_dict();
        for (k, v) in pairs {
            d.insert(k.into(), v);
        }
        Value::Dict(d)
    }

    // ── JSON ────────────────────────────────────────────────────────────────
    #[test]
    fn json_objeto_array_primitivos() {
        let v = decode_json(br#"{"a":1,"b":[true,null,"x"],"c":3.5}"#).unwrap();
        assert_eq!(
            v,
            dict_of(vec![
                ("a", Value::Int(1)),
                (
                    "b",
                    Value::Array(vec![
                        Value::Bool(true),
                        Value::None,
                        Value::Str("x".into())
                    ])
                ),
                ("c", Value::Float(3.5)),
            ])
        );
    }

    #[test]
    fn json_escalar_e_erro() {
        assert_eq!(decode_json(b"3.5").unwrap(), Value::Float(3.5));
        assert_eq!(decode_json(br#""x""#).unwrap(), Value::Str("x".into()));
        assert!(decode_json(b"{bad").is_err());
    }

    #[test]
    fn json_dict_preserva_ordem_de_insercao() {
        let v = decode_json(br#"{"z":1,"a":2,"m":3}"#).unwrap();
        if let Value::Dict(d) = v {
            let keys: Vec<&str> = d.keys().map(|k| k.as_str()).collect();
            assert_eq!(keys, vec!["z", "a", "m"], "ordem de inserção, não alfabética");
        } else {
            panic!("esperava Dict");
        }
    }

    // ── YAML ────────────────────────────────────────────────────────────────
    #[test]
    fn yaml_mapping_e_sequence() {
        let v = decode_yaml(b"a: 1\nb: hello\n").unwrap();
        assert_eq!(
            v,
            dict_of(vec![("a", Value::Int(1)), ("b", Value::Str("hello".into()))])
        );
        let v2 = decode_yaml(b"- 1\n- 2\n").unwrap();
        assert_eq!(v2, Value::Array(vec![Value::Int(1), Value::Int(2)]));
    }

    #[test]
    fn yaml_tipos_e_null() {
        let v = decode_yaml(b"x: true\ny: 3.5\nz: null\n").unwrap();
        assert_eq!(
            v,
            dict_of(vec![
                ("x", Value::Bool(true)),
                ("y", Value::Float(3.5)),
                ("z", Value::None),
            ])
        );
    }

    // ── TOML ────────────────────────────────────────────────────────────────
    #[test]
    fn toml_table_tipos() {
        let v = decode_toml(b"x = 1\ny = \"s\"\nf = 1.5\n").unwrap();
        assert_eq!(
            v,
            dict_of(vec![
                ("x", Value::Int(1)),
                ("y", Value::Str("s".into())),
                ("f", Value::Float(1.5)),
            ])
        );
    }

    #[test]
    fn toml_datetime_graded_str() {
        // graded (ADR-0054 / DEBT-62 contexto): datetime → Str RFC 3339.
        let v = decode_toml(b"d = 1979-05-27\n").unwrap();
        if let Value::Dict(d) = v {
            assert_eq!(d.get("d"), Some(&Value::Str("1979-05-27".into())));
        } else {
            panic!("esperava Dict");
        }
    }

    // ── CBOR ────────────────────────────────────────────────────────────────
    fn cbor_bytes(v: &ciborium::value::Value) -> Vec<u8> {
        let mut out = Vec::new();
        ciborium::into_writer(v, &mut out).unwrap();
        out
    }

    #[test]
    fn cbor_tree() {
        use ciborium::value::Value as C;
        let doc = C::Map(vec![(C::Text("a".into()), C::Integer(1.into()))]);
        let v = decode_cbor(&cbor_bytes(&doc)).unwrap();
        assert_eq!(v, dict_of(vec![("a", Value::Int(1))]));
    }

    #[test]
    fn cbor_byte_string_returns_bytes() {
        use ciborium::value::Value as C;
        let doc = C::Bytes(vec![1, 2, 3]);
        let v = decode_cbor(&cbor_bytes(&doc)).unwrap();
        assert_eq!(v, Value::Bytes(Bytes::new(vec![1, 2, 3])));
    }

    // ── CSV ─────────────────────────────────────────────────────────────────
    #[test]
    fn csv_array_default() {
        let v = decode_csv(b"a,b\n1,2", b',', RowType::Array).unwrap();
        assert_eq!(
            v,
            Value::Array(vec![
                Value::Array(vec![Value::Str("a".into()), Value::Str("b".into())]),
                Value::Array(vec![Value::Str("1".into()), Value::Str("2".into())]),
            ])
        );
    }

    #[test]
    fn csv_dictionary_e_delimiter() {
        let v = decode_csv(b"a;b\n1;2", b';', RowType::Dictionary).unwrap();
        assert_eq!(
            v,
            Value::Array(vec![dict_of(vec![
                ("a", Value::Str("1".into())),
                ("b", Value::Str("2".into())),
            ])])
        );
    }

    // ── P787 — rigor de parsing (mensagens medidas no vanilla 0.15.0) ──────
    #[test]
    fn p787_csv_linha_malformada_rejeitada() {
        // Antes: `flexible(true)` aceitava em silêncio (P786 B1). Vanilla:
        // `failed to parse CSV (found 3 instead of 2 fields in line 2)`.
        let e = decode_csv(b"a,b\n1,2,3\n", b',', RowType::Array).unwrap_err();
        let msg = e.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert!(
            msg.contains("failed to parse CSV (found 3 instead of 2 fields in line 2)"),
            "mensagem inesperada: {msg}"
        );
    }

    #[test]
    fn p787_csv_linha_malformada_dictionary_rejeitada() {
        // Modo dictionary: a validação é contra o nº de chaves do cabeçalho.
        let e = decode_csv(b"a;b\n1;2;3\n", b';', RowType::Dictionary).unwrap_err();
        let msg = e.first().map(|d| d.message.to_string()).unwrap_or_default();
        assert!(
            msg.contains("failed to parse CSV (found 3 instead of 2 fields in line 2)"),
            "mensagem inesperada: {msg}"
        );
    }

    #[test]
    fn p787_csv_valido_nao_regressao() {
        // CSV bem-formado continua a parsear (flexible desligado não quebra o válido).
        let v = decode_csv(b"a,b\n1,2\n3,4\n", b',', RowType::Array).unwrap();
        assert_eq!(
            v,
            Value::Array(vec![
                Value::Array(vec![Value::Str("a".into()), Value::Str("b".into())]),
                Value::Array(vec![Value::Str("1".into()), Value::Str("2".into())]),
                Value::Array(vec![Value::Str("3".into()), Value::Str("4".into())]),
            ])
        );
    }

    // ── XML ─────────────────────────────────────────────────────────────────
    #[test]
    fn xml_nodes() {
        let v = decode_xml(b"<r><c>t</c></r>").unwrap();
        let inner_c = dict_of(vec![
            ("tag", Value::Str("c".into())),
            ("attrs", Value::Dict(new_dict())),
            ("children", Value::Array(vec![Value::Str("t".into())])),
        ]);
        let root = dict_of(vec![
            ("tag", Value::Str("r".into())),
            ("attrs", Value::Dict(new_dict())),
            ("children", Value::Array(vec![inner_c])),
        ]);
        assert_eq!(v, Value::Array(vec![root]));
    }

    #[test]
    fn xml_atributos() {
        let v = decode_xml(br#"<a href="x"/>"#).unwrap();
        if let Value::Array(nodes) = v {
            if let Value::Dict(d) = &nodes[0] {
                assert_eq!(
                    d.get("attrs"),
                    Some(&dict_of(vec![("href", Value::Str("x".into()))]))
                );
            } else {
                panic!("esperava Dict");
            }
        } else {
            panic!("esperava Array");
        }
    }

    // ── read() texto vs binário — P398 ───────────────────────────────────────

    use std::num::NonZeroU16;
    use std::sync::Arc;

    struct MockWorld {
        files: std::collections::HashMap<String, Arc<Vec<u8>>>,
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl Default for MockWorld {
        fn default() -> Self {
            Self {
                files: std::collections::HashMap::new(),
                library: crate::entities::world_types::Library::default(),
                book: crate::entities::font_book::FontBook::default(),
            }
        }
    }
    impl crate::contracts::world::World for MockWorld {
        fn library(&self) -> &crate::entities::world_types::Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(NonZeroU16::new(1).unwrap())
        }
        fn source(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<crate::entities::world_types::Datetime> {
            None
        }
        fn read_bytes(
            &self,
            _current_file: FileId,
            path: &str,
        ) -> Result<Arc<Vec<u8>>, String> {
            self.files
                .get(path)
                .cloned()
                .ok_or_else(|| format!("ficheiro não encontrado: {}", path))
        }
        fn resolve_path(
            &self,
            _current_file: FileId,
            path: &str,
        ) -> Result<crate::entities::path::RootedPath, String> {
            let vpath = crate::entities::path::VirtualPath::new(path)
                .map_err(|e| format!("path inválido: {e:?}"))?;
            Ok(crate::entities::path::RootedPath::new(
                crate::entities::path::VirtualRoot::Project,
                vpath,
            ))
        }
        fn read_path(
            &self,
            path: &crate::entities::path::RootedPath,
        ) -> Result<Arc<Vec<u8>>, String> {
            let key = path.vpath().get_with_slash().trim_start_matches('/');
            self.files
                .get(key)
                .cloned()
                .ok_or_else(|| format!("ficheiro não encontrado: {key}"))
        }
    }

    fn mock_args(path: &str) -> Args {
        Args::positional(vec![Value::Str(path.into())])
    }

    #[test]
    fn read_texto_utf8() {
        let mut world = MockWorld::default();
        world.files.insert("texto.txt".into(), Arc::new(b"hello".to_vec()));
        let v = native_read(
            &mut EvalContext::new(),
            &mock_args("texto.txt"),
            &world,
            FileId::from_raw(NonZeroU16::new(1).unwrap()),
        )
        .unwrap();
        assert_eq!(v, Value::Str("hello".into()));
    }

    #[test]
    fn read_binario_nao_utf8() {
        // P824 — ficheiro não-UTF8 SEM `encoding:` é ERRO (paridade vanilla
        // medida: `failed to convert to string (file is not valid UTF-8 in
        // {ficheiro}:{linha}:{col})`); o fallback silencioso para Bytes foi
        // removido (a "heurística vanilla" do comentário antigo estava
        // refutada por medição — P810 §11).
        let mut world = MockWorld::default();
        world.files.insert(
            "logo.png".into(),
            Arc::new(vec![0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]),
        );
        let e = native_read(
            &mut EvalContext::new(),
            &mock_args("logo.png"),
            &world,
            FileId::from_raw(NonZeroU16::new(1).unwrap()),
        )
        .unwrap_err();
        assert_eq!(
            e[0].message.to_string(),
            "failed to convert to string (file is not valid UTF-8 in /logo.png:1:1)"
        );
    }

    // ── P824 — read(): named `encoding:` (paridade vanilla read.rs:24-47) ───

    fn mock_args_encoding(path: &str, encoding: Value) -> Args {
        let mut args = mock_args(path);
        args.named.insert("encoding".into(), encoding);
        args
    }

    #[test]
    fn p824_read_encoding_utf8_aceite() {
        // `encoding: "utf8"` é língua válida (antes: rejeitado como named
        // inesperado). Equivale ao default.
        let mut world = MockWorld::default();
        world.files.insert("texto.txt".into(), Arc::new(b"hello".to_vec()));
        let v = native_read(
            &mut EvalContext::new(),
            &mock_args_encoding("texto.txt", Value::Str("utf8".into())),
            &world,
            FileId::from_raw(NonZeroU16::new(1).unwrap()),
        )
        .unwrap();
        assert_eq!(v, Value::Str("hello".into()));
    }

    #[test]
    fn p824_read_encoding_none_devolve_bytes() {
        // `encoding: none` → bytes crus, mesmo em ficheiro UTF-8 válido.
        let mut world = MockWorld::default();
        world.files.insert("texto.txt".into(), Arc::new(b"hello".to_vec()));
        let v = native_read(
            &mut EvalContext::new(),
            &mock_args_encoding("texto.txt", Value::None),
            &world,
            FileId::from_raw(NonZeroU16::new(1).unwrap()),
        )
        .unwrap();
        assert_eq!(v, Value::Bytes(Bytes::new(b"hello".to_vec())));
    }

    #[test]
    fn p824_read_encoding_fora_do_dominio_erro_vanilla() {
        // Lista medida no vanilla: só "utf8" ou none. String fora do domínio
        // → `expected "utf8" or none` (sem sufixo "found"); outro tipo →
        // `expected "utf8" or none, found {tipo}`.
        let mut world = MockWorld::default();
        world.files.insert("texto.txt".into(), Arc::new(b"hello".to_vec()));
        let fid = FileId::from_raw(NonZeroU16::new(1).unwrap());
        let e = native_read(
            &mut EvalContext::new(),
            &mock_args_encoding("texto.txt", Value::Str("latin1".into())),
            &world,
            fid,
        )
        .unwrap_err();
        assert_eq!(e[0].message.to_string(), "expected \"utf8\" or none");
        let e = native_read(
            &mut EvalContext::new(),
            &mock_args_encoding("texto.txt", Value::Int(5)),
            &world,
            fid,
        )
        .unwrap_err();
        assert_eq!(e[0].message.to_string(), "expected \"utf8\" or none, found integer");
        let e = native_read(
            &mut EvalContext::new(),
            &mock_args_encoding("texto.txt", Value::Bool(true)),
            &world,
            fid,
        )
        .unwrap_err();
        assert_eq!(e[0].message.to_string(), "expected \"utf8\" or none, found boolean");
    }

    #[test]
    fn p824_read_nao_utf8_linha_col_vanilla() {
        // Posição `line:col` do byte problemático no formato do vanilla
        // (`LineCol::try_from_byte_pos`, diag.rs:1027-1039): com '\n' antes,
        // col = chars desde o último '\n' ("ab\ncd" + 0xE1 → 2:3).
        let mut world = MockWorld::default();
        world
            .files
            .insert("latin1.txt".into(), Arc::new(b"ol\xe1 mundo\n".to_vec()));
        world
            .files
            .insert("multilinha.txt".into(), Arc::new(b"ab\ncd\xe1".to_vec()));
        let fid = FileId::from_raw(NonZeroU16::new(1).unwrap());
        let e =
            native_read(&mut EvalContext::new(), &mock_args("latin1.txt"), &world, fid)
                .unwrap_err();
        assert_eq!(
            e[0].message.to_string(),
            "failed to convert to string (file is not valid UTF-8 in /latin1.txt:1:1)"
        );
        let e = native_read(
            &mut EvalContext::new(),
            &mock_args("multilinha.txt"),
            &world,
            fid,
        )
        .unwrap_err();
        assert_eq!(
            e[0].message.to_string(),
            "failed to convert to string (file is not valid UTF-8 in /multilinha.txt:2:3)"
        );
        // Com `encoding: "utf8"` explícito o erro é o mesmo.
        let e = native_read(
            &mut EvalContext::new(),
            &mock_args_encoding("latin1.txt", Value::Str("utf8".into())),
            &world,
            fid,
        )
        .unwrap_err();
        assert_eq!(
            e[0].message.to_string(),
            "failed to convert to string (file is not valid UTF-8 in /latin1.txt:1:1)"
        );
    }

    #[test]
    fn read_vazio() {
        let mut world = MockWorld::default();
        world.files.insert("empty".into(), Arc::new(Vec::new()));
        let v = native_read(
            &mut EvalContext::new(),
            &mock_args("empty"),
            &world,
            FileId::from_raw(NonZeroU16::new(1).unwrap()),
        )
        .unwrap();
        assert_eq!(v, Value::Str("".into()));
    }

    // ── P823 — erro CBOR no formato do vanilla (format_cbor_error) ──────────

    #[test]
    fn p823_cbor_malformado_mensagem_vanilla() {
        // 0xFF = break byte. Vanilla (`loading/cbor.rs:88-98`):
        // `failed to parse CBOR (invalid type: break, expected non-break)`.
        let e = decode_cbor(&[0xff]).unwrap_err();
        assert_eq!(
            e[0].message.to_string(),
            "failed to parse CBOR (invalid type: break, expected non-break)"
        );
    }

    #[test]
    fn p823_native_cbor_path_acrescenta_ficheiro() {
        // Fonte path: vanilla acrescenta ` in {ficheiro}` dentro dos
        // parênteses (`diag.rs` load_err_in_binary); fonte bytes fica sem
        // sufixo (ambos medidos em P823).
        let mut world = MockWorld::default();
        world.files.insert("invalid.cbor".into(), Arc::new(vec![0xff]));
        let e = native_cbor(
            &mut EvalContext::new(),
            &mock_args("invalid.cbor"),
            &world,
            tfid(),
        )
        .unwrap_err();
        assert_eq!(
            e[0].message.to_string(),
            "failed to parse CBOR (invalid type: break, expected non-break in invalid.cbor)"
        );
    }

    // ── P701 — cbor.encode (Value → CBOR) e path|bytes nos 5 loaders ─────────

    fn tfid() -> FileId {
        FileId::from_raw(NonZeroU16::new(1).unwrap())
    }

    #[test]
    fn cbor_encode_tipos_basicos_ida_e_volta() {
        let v = dict_of(vec![
            ("a", Value::Int(1)),
            ("b", Value::Str("texto".into())),
            ("c", Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)])),
            ("d", Value::None),
            ("e", Value::Bool(true)),
            ("f", Value::Float(1.5)),
        ]);
        let args = Args::positional(vec![v.clone()]);
        let encoded = native_cbor_encode(
            &mut EvalContext::new(),
            &args,
            &MockWorld::default(),
            tfid(),
        )
        .unwrap();
        let bytes = match &encoded {
            Value::Bytes(b) => b.as_slice().to_vec(),
            _ => panic!("esperava Bytes"),
        };
        assert_eq!(
            decode_cbor(&bytes).unwrap(),
            v,
            "ida e volta cbor.encode -> decode_cbor deve preservar o valor"
        );
    }

    #[test]
    fn cbor_encode_bytes_como_byte_string_nao_texto() {
        // Bytes -> CBOR byte-string (não a forma texto de repr()) — paridade
        // vanilla `bytes.rs:370` (`is_human_readable() == false` em ciborium).
        let args = Args::positional(vec![Value::Bytes(Bytes::new(vec![1, 2, 3]))]);
        let encoded = native_cbor_encode(
            &mut EvalContext::new(),
            &args,
            &MockWorld::default(),
            tfid(),
        )
        .unwrap();
        let bytes = match &encoded {
            Value::Bytes(b) => b.as_slice().to_vec(),
            _ => panic!("esperava Bytes"),
        };
        assert_eq!(decode_cbor(&bytes).unwrap(), Value::Bytes(Bytes::new(vec![1, 2, 3])));
    }

    #[test]
    fn cbor_encode_tipo_opaco_cai_em_repr() {
        // Length não está na tabela dedicada (§3.4) -> fallback Text(repr(v)),
        // igual ao "other" do vanilla (value.rs:363).
        let args = Args::positional(vec![Value::Length(
            crate::entities::layout_types::Length::pt(12.0),
        )]);
        let encoded = native_cbor_encode(
            &mut EvalContext::new(),
            &args,
            &MockWorld::default(),
            tfid(),
        )
        .unwrap();
        let bytes = match &encoded {
            Value::Bytes(b) => b.as_slice().to_vec(),
            _ => panic!("esperava Bytes"),
        };
        assert_eq!(
            decode_cbor(&bytes).unwrap(),
            Value::Str(
                crate::compiler::eval::repr::repr_value(&Value::Length(
                    crate::entities::layout_types::Length::pt(12.0)
                ))
                .into()
            )
        );
    }

    #[test]
    fn cbor_encode_requer_exactamente_1_arg() {
        let args = Args::positional(vec![]);
        assert!(native_cbor_encode(
            &mut EvalContext::new(),
            &args,
            &MockWorld::default(),
            tfid()
        )
        .is_err());
    }

    #[test]
    fn native_json_aceita_bytes_alem_de_path() {
        // P701 — resolve_data: Bytes usado directamente, sem tocar em World::read_bytes.
        let args =
            Args::positional(vec![Value::Bytes(Bytes::new(br#"{"a":1}"#.to_vec()))]);
        let v =
            native_json(&mut EvalContext::new(), &args, &MockWorld::default(), tfid())
                .unwrap();
        assert_eq!(v, dict_of(vec![("a", Value::Int(1))]));
    }

    #[test]
    fn native_cbor_aceita_bytes_alem_de_path() {
        use ciborium::value::Value as C;
        let doc = C::Map(vec![(C::Text("a".into()), C::Integer(1.into()))]);
        let args = Args::positional(vec![Value::Bytes(Bytes::new(cbor_bytes(&doc)))]);
        let v =
            native_cbor(&mut EvalContext::new(), &args, &MockWorld::default(), tfid())
                .unwrap();
        assert_eq!(v, dict_of(vec![("a", Value::Int(1))]));
    }

    #[test]
    fn native_json_path_continua_a_funcionar() {
        // Regressão: resolve_data não deve quebrar o caminho já suportado.
        let mut world = MockWorld::default();
        world.files.insert("d.json".into(), Arc::new(br#"{"x":2}"#.to_vec()));
        let v =
            native_json(&mut EvalContext::new(), &mock_args("d.json"), &world, tfid())
                .unwrap();
        assert_eq!(v, dict_of(vec![("x", Value::Int(2))]));
    }

    #[test]
    fn native_json_tipo_invalido_erro() {
        let args = Args::positional(vec![Value::Int(1)]);
        let e =
            native_json(&mut EvalContext::new(), &args, &MockWorld::default(), tfid())
                .unwrap_err();
        assert!(
            e[0].message.contains("caminho") || e[0].message.contains("bytes"),
            "msg: {}",
            e[0].message
        );
    }
}
