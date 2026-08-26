//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/text/shift.md
//! @prompt-hash b6afb733
//! @layer L1
//! @updated 2026-08-13
//!
//! `sub` / `super` — deslocamento de baseline com redução de corpo.
//!
//! Extraído de `stdlib/text.rs` (2026-08-13) conforme ADR-0109 — atomização
//! forma B: a lógica vive no ficheiro da unidade, o hub é só fronteira.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::layout_types::Length;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

// ── Passo 448 — `sub(body)` / `super(body)` ─────────────────────────────────
//
// Paridade vanilla `text/sub.rs::SubElem` e `text/superscript.rs::SuperElem`:
// elementos de texto que deslocam a baseline e reduzem o corpo. Modelo
// minimal: `Content::Styled` com `Style::Subscript`/`Style::Superscript`.

/// `sub(body, size:?)` → content embrulhado em `Content::Styled([Subscript(true)])`.
/// P471: argumento nomeado `size: Length` define tamanho explícito do script.
pub fn native_subscript(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.as_slice() {
        [Value::Content(c)] => c.clone(),
        [Value::Str(s)] => Content::text(s.as_str()),
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("sub() espera content ou string, recebeu {}", other.type_name()),
            )])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "sub() exige body como argumento posicional".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "sub() recebeu {} argumentos posicionais (espera 1)",
                    args.items.len()
                ),
            )])
        }
    };
    let mut size: Option<Length> = None;
    for (key, value) in args.named.iter() {
        match key.as_str() {
            "size" => {
                size = match value {
                    Value::Length(l) => Some(*l),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "sub(size:) espera length, recebeu {}",
                                other.type_name()
                            ),
                        )])
                    }
                };
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("sub() argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }
    Ok(Value::Content(Content::sub_with_size(body, size)))
}

/// `super(body, size:?)` → content embrulhado em `Content::Styled([Superscript(true)])`.
/// P471: argumento nomeado `size: Length` define tamanho explícito do script.
pub fn native_superscript(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.as_slice() {
        [Value::Content(c)] => c.clone(),
        [Value::Str(s)] => Content::text(s.as_str()),
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "super() espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "super() exige body como argumento posicional".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "super() recebeu {} argumentos posicionais (espera 1)",
                    args.items.len()
                ),
            )])
        }
    };
    let mut size: Option<Length> = None;
    for (key, value) in args.named.iter() {
        match key.as_str() {
            "size" => {
                size = match value {
                    Value::Length(l) => Some(*l),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "super(size:) espera length, recebeu {}",
                                other.type_name()
                            ),
                        )])
                    }
                };
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("super() argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }
    Ok(Value::Content(Content::superscript_with_size(body, size)))
}
