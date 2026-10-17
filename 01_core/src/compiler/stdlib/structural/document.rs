//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/document.md
//! @prompt-hash 14c212ef
//! @layer L1
//! @updated 2026-08-12
//!
//! Nativas de metadados do documento: `document` e `asset`.
//!
//! Extraído de `stdlib/structural.rs` no Passo 1014 conforme ADR-0109
//! (atomização — forma B, free function no arquivo da unidade).

use crate::entities::file_id::FileId;
use ecow::EcoString;

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

// ── Sentinelas e construtores de nós estruturais (Passo 69) ─────────────────

/// Construct a bundle document. Set rules are handled by eval/rules.
pub fn native_document(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    for name in args.named.keys() {
        if !["format", "title", "author", "date", "keywords"].contains(&name.as_str()) {
            return Err(error(format!("unexpected argument: {name}")));
        }
    }
    if args.items.is_empty() {
        return Err(error("missing argument: path"));
    }
    if args.items.len() < 2 {
        return Err(error("missing argument: body"));
    }
    if args.items.len() > 2 {
        return Err(error("unexpected argument"));
    }
    let path = match &args.items[0] {
        Value::Str(path) => normalize_bundle_path(path)?,
        other => {
            return Err(error(format!("expected string, found {}", other.type_name())))
        }
    };
    let body = match &args.items[1] {
        Value::Content(body) => body.clone(),
        other => {
            return Err(error(format!("expected content, found {}", other.type_name())))
        }
    };
    if ctx.target != crate::compiler::eval::EvalTarget::Bundle {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "constructing a document is only supported in the bundle target",
        )
        .with_hint("try exporting with --format bundle")
        .with_hint("use set document to configure document metadata")]);
    }
    let format = match args.named.get("format") {
        None => None,
        Some(Value::Func(func)) => func.name().map(EcoString::from),
        Some(other) => {
            return Err(error(format!("expected function, found {}", other.type_name())))
        }
    };
    let title = match args.named.get("title") {
        Some(Value::Content(title)) => Some(title.clone()),
        Some(other) => {
            return Err(error(format!("expected content, found {}", other.type_name())))
        }
        None => None,
    };
    let author = match args.named.get("author") {
        Some(value) => extract_string_list(value, "author")?,
        None => Vec::new(),
    };
    let date = match args.named.get("date") {
        Some(Value::Datetime(date)) => Some(*date),
        Some(other) => {
            return Err(error(format!("expected datetime, found {}", other.type_name())))
        }
        None => None,
    };
    let keywords = match args.named.get("keywords") {
        Some(value) => extract_string_list(value, "keywords")?,
        None => Vec::new(),
    };
    Ok(Value::Content(Content::document(
        path, body, format, title, author, date, keywords,
    )))
}

/// Construct a bundle asset with byte-exact data.
pub fn native_asset(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if let Some(name) = args.named.keys().next() {
        let message = if name.as_str() == "path" || name.as_str() == "data" {
            format!("the argument {name} is positional")
        } else {
            format!("unexpected argument: {name}")
        };
        return Err(error(message));
    }
    if args.items.is_empty() {
        return Err(error("missing argument: path"));
    }
    if args.items.len() < 2 {
        return Err(error("missing argument: data"));
    }
    if args.items.len() > 2 {
        return Err(error("unexpected argument"));
    }
    let path = match &args.items[0] {
        Value::Str(path) => normalize_bundle_path(path)?,
        other => {
            return Err(error(format!(
                "expected string, found {}",
                diagnostic_type_name(other)
            )))
        }
    };
    let data = match &args.items[1] {
        Value::Str(value) => value.as_bytes().to_vec(),
        Value::Bytes(value) => value.as_slice().to_vec(),
        other => {
            return Err(error(format!(
                "expected string or bytes, found {}",
                diagnostic_type_name(other)
            )))
        }
    };
    Ok(Value::Content(Content::asset(path, data)))
}

fn diagnostic_type_name(value: &Value) -> &str {
    match value {
        Value::Int(_) => "integer",
        Value::Bool(_) => "boolean",
        Value::None
        | Value::Float(_)
        | Value::Str(_)
        | Value::Array(_)
        | Value::Dict(_)
        | Value::Module(_)
        | Value::Datetime(_)
        | Value::Func(_)
        | Value::Content(_)
        | Value::LocatedContent(_, _)
        | Value::Auto
        | Value::Length(_)
        | Value::Relative(_)
        | Value::Ratio(_)
        | Value::Angle(_)
        | Value::Color(_)
        | Value::Stroke(_)
        | Value::Fraction(_)
        | Value::Align(_)
        | Value::Location(_)
        | Value::Gradient(_)
        | Value::Regex(_)
        | Value::Tiling(_)
        | Value::Bytes(_)
        | Value::Decimal(_)
        | Value::Duration(_)
        | Value::Version(_)
        | Value::Selector(_)
        | Value::Symbol(_)
        | Value::Args(_)
        | Value::State(_)
        | Value::Counter(_)
        | Value::Label(_)
        | Value::Dir(_)
        | Value::Path(_)
        | Value::Type(_)
        | Value::SpotColorant(_) => value.type_name(),
    }
}

fn error(message: impl Into<String>) -> Vec<SourceDiagnostic> {
    vec![SourceDiagnostic::error(Span::detached(), message.into())]
}

fn normalize_bundle_path(path: &str) -> SourceResult<EcoString> {
    if path.contains('\\') {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "backslashes are not allowed in bundle paths",
        )
        .with_hint("use forward slashes instead")]);
    }
    let mut components = Vec::new();
    for component in path.split('/') {
        match component {
            "" | "." => {}
            ".." => {
                if components.pop().is_none() {
                    return Err(error(format!(
                        "path {path} would escape the bundle root"
                    )));
                }
            }
            component => components.push(component),
        }
    }
    if components.is_empty() {
        return Err(error("path must have at least one component"));
    }
    Ok(EcoString::from(format!("/{}", components.join("/"))))
}

fn extract_string_list(value: &Value, field: &str) -> SourceResult<Vec<EcoString>> {
    match value {
        Value::Str(s) => Ok(vec![s.clone()]),
        Value::Array(arr) => arr
            .iter()
            .map(|v| match v {
                Value::Str(s) => Ok(s.clone()),
                other => Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "{} deve ser string ou array de strings; recebeu {}",
                        field,
                        other.type_name()
                    ),
                )]),
            })
            .collect::<Result<Vec<_>, _>>(),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "{} deve ser string ou array de strings; recebeu {}",
                field,
                other.type_name()
            ),
        )]),
    }
}

// ── P470 — native_list / native_enum ────────────────────────────────────────

#[cfg(test)]
mod v16_projection_tests {
    use super::*;
    use crate::entities::bytes::Bytes;

    #[test]
    fn v16_value_projection_document_diagnostics_keep_short_string_name() {
        for (value, name) in [
            (Value::Int(3), "integer"),
            (Value::Bool(false), "boolean"),
            (Value::Str("x".into()), "str"),
            (Value::None, "none"),
            (Value::Auto, "auto"),
            (Value::Float(1.5), "float"),
            (Value::Array(vec![]), "array"),
            (Value::Content(Content::Empty), "content"),
            (Value::Bytes(Bytes::new(vec![0, 255])), "bytes"),
        ] {
            assert_eq!(diagnostic_type_name(&value), name);
        }
    }
}
