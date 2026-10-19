//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/text/smartquote.md
//! @prompt-hash 0c37cd4b
//! @layer L1
//! @updated 2026-10-06
//!
//! `smartquote` — aspa tipográfica resolvida lang-aware pelo layout.
//!
//! Extraído de `stdlib/text.rs` (2026-08-13) conforme ADR-0109 — atomização
//! forma B: a lógica vive no ficheiro da unidade, o hub é só fronteira.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::elements::smartquote::SmartQuoteElem;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

// Preserve explicit fields, including disabled, until the target renders them.

pub fn native_smartquote(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!(
                "smartquote() não aceita argumentos posicionais (recebeu {})",
                args.items.len()
            ),
        )]);
    }

    let mut double = None;
    let mut enabled = None;
    let mut alternative = None;
    let mut quotes = None;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "double" => match value {
                Value::Bool(b) => double = Some(*b),
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "smartquote(double:) espera bool, recebeu {}",
                            other.type_name()
                        ),
                    )])
                }
            },
            "enabled" => match value {
                Value::Bool(b) => enabled = Some(*b),
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "smartquote(enabled:) espera bool, recebeu {}",
                            other.type_name()
                        ),
                    )])
                }
            },
            "alternative" => match value {
                Value::Bool(value) => alternative = Some(*value),
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "smartquote(alternative:) espera bool, recebeu {}",
                            other.type_name()
                        ),
                    )])
                }
            },
            "quotes" => {
                quotes = Some(crate::compiler::lang::quotes::parse_smartquote_quotes(
                    value, args.span,
                )?);
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("smartquote(): argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }

    Ok(Value::Content(Content::SmartQuote(std::sync::Arc::new(SmartQuoteElem {
        double,
        enabled,
        alternative,
        quotes,
    }))))
}
