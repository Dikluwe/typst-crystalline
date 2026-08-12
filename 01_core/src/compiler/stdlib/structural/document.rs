//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/document.md
//! @prompt-hash bec77d19
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

/// `document(title:?, author:?, date:?, keywords:?)` — metadata pura do documento.
pub fn native_document(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    const ALLOWED: &[&str] = &["title", "author", "date", "keywords"];
    for key in args.named.keys() {
        if !ALLOWED.contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("document() não aceita o argumento nomeado '{}'", key),
            )]);
        }
    }

    let title = match args.named.get("title") {
        Some(Value::Content(c)) => Some(c.clone()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("title deve ser content, recebeu {}", other.type_name()),
            )]);
        }
        None => None,
    };

    let author = match args.named.get("author") {
        Some(v) => extract_string_list(v, "author")?,
        None => Vec::new(),
    };

    let date = match args.named.get("date") {
        Some(Value::Datetime(d)) => Some(*d),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("date deve ser datetime, recebeu {}", other.type_name()),
            )]);
        }
        None => None,
    };

    let keywords = match args.named.get("keywords") {
        Some(v) => extract_string_list(v, "keywords")?,
        None => Vec::new(),
    };

    Ok(Value::Content(Content::document(title, author, date, keywords)))
}

/// `asset(path, kind:?)` — placeholder de resource externo (extensão cristalina).
pub fn native_asset(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    const ALLOWED: &[&str] = &["path", "kind"];
    for key in args.named.keys() {
        if !ALLOWED.contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("asset() não aceita o argumento nomeado '{}'", key),
            )]);
        }
    }

    let path = if let Some(Value::Str(s)) = args.named.get("path") {
        s.clone()
    } else if let Some(Value::Str(s)) = args.items.first() {
        s.clone()
    } else {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "asset() espera path como string posicional ou named".to_string(),
        )]);
    };

    let kind = if let Some(Value::Str(s)) = args.named.get("kind") {
        Some(s.clone())
    } else {
        infer_asset_kind(&path)
    };

    Ok(Value::Content(Content::asset(path, kind)))
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
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e),
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

fn infer_asset_kind(path: &EcoString) -> Option<EcoString> {
    let ext = path.rsplit('.').next()?;
    Some(match ext.to_lowercase().as_str() {
        "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" => EcoString::from("image"),
        "ttf" | "otf" | "woff" | "woff2" => EcoString::from("font"),
        "wasm" => EcoString::from("wasm"),
        "txt" | "csv" | "json" | "yaml" | "yml" | "toml" | "xml" => {
            EcoString::from("data")
        }
        _ => return None,
    })
}

// ── P470 — native_list / native_enum ────────────────────────────────────────

