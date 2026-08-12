//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/markup.md
//! @prompt-hash 98e70664
//! @layer L1
//! @updated 2026-08-12
//!
//! Nativas de markup inline: `strong`, `emph`, `raw`, `link`.
//!
//! Extraído de `stdlib/structural.rs` no Passo 1014 conforme ADR-0109
//! (atomização — forma B, free function no arquivo da unidade).

use crate::entities::file_id::FileId;
use ecow::EcoString;

use crate::compiler::stdlib::expect_no_named;

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

// ── Sentinelas e construtores de nós estruturais (Passo 69) ─────────────────

/// `strong(body)` — emite `Content::Styled([Bold(true)], body)`
/// (Passo 101) ou serve como selector em show rules.
pub fn native_strong(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "strong() espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => Content::Empty,
    };
    Ok(Value::Content(Content::strong(body)))
}

/// `emph(body)` — emite `Content::Styled([Italic(true)], body)`
/// (Passo 101) ou serve como selector em show rules.
pub fn native_emph(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("emph() espera content ou string, recebeu {}", other.type_name()),
            )])
        }
        None => Content::Empty,
    };
    Ok(Value::Content(Content::emph(body)))
}

/// `raw(text, lang:?, block:?)` — cria `Content::Raw` ou serve como selector em show rules.
/// Aceita apenas string — não faz sentido semântico aceitar Content aqui.
/// P502: `lang` e `block` named opcionais; syntax highlighting real continua scope-out.
pub fn native_raw(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let text: EcoString = match args.items.first() {
        Some(Value::Str(s)) => s.clone(),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("raw() espera string, recebeu {}", other.type_name()),
            )])
        }
        None => EcoString::default(),
    };

    for (key, _value) in args.named.iter() {
        match key.as_str() {
            "lang" | "block" => {}
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("raw(): argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }

    let lang = match args.named.get("lang") {
        Some(Value::Str(s)) => Some(s.clone()),
        Some(Value::None) | None => None,
        Some(v) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("raw(lang:): espera string, recebeu {}", v.type_name()),
            )]);
        }
    };

    let block = match args.named.get("block") {
        Some(Value::Bool(b)) => *b,
        Some(Value::None) | None => false,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("raw(block:): espera bool, recebeu {}", other.type_name()),
            )])
        }
    };

    Ok(Value::Content(Content::raw(text, lang, block)))
}

// ── `heading()` — função nativa + selector para show rules (P451) ───────────

/// `link(url, body)` — hiperligação (P422).
///
/// - 1º arg posicional: URL (`Str`).
/// - 2º arg posicional opcional: body (`Content`). Se omitido, o body é o
///   próprio URL como texto.
/// - Named args não suportados.
pub fn native_link(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    crate::compiler::stdlib::expect_no_named(&args.named)?;
    let url = match args.items.first() {
        Some(Value::Str(s)) if !s.is_empty() => s.clone(),
        Some(Value::Str(_)) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "link() URL não pode ser vazia".to_string(),
            )])
        }
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("link() espera URL como string, recebeu {}", other.type_name()),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "link() exige URL como argumento posicional".to_string(),
            )])
        }
    };

    let body = match args.items.get(1) {
        Some(Value::Content(c)) => c.clone(),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("link() espera body como content, recebeu {}", other.type_name()),
            )])
        }
        None => Content::text(url.as_str()),
    };

    Ok(Value::Content(Content::link(url, body)))
}

