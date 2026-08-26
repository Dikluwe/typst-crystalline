//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/text/smartquote.md
//! @prompt-hash 5a495c14
//! @layer L1
//! @updated 2026-08-13
//!
//! `smartquote` — aspa tipográfica resolvida lang-aware pelo layout.
//!
//! Extraído de `stdlib/text.rs` (2026-08-13) conforme ADR-0109 — atomização
//! forma B: a lógica vive no ficheiro da unidade, o hub é só fronteira.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

// ── Passo 287 — função `#smartquote(double, enabled, alternative)` ──────────
//
// Paridade vanilla `text/smartquote.rs::SmartQuoteElem`. Cristalino:
// - `double: bool = true`  — aspas duplas (default).
// - `enabled: bool = true` — smart-quotes activas. Quando `false`, emite
//                            o glyph ASCII literal (`"`/`'`) sem alternância
//                            lang-aware (paridade vanilla `set smartquote(enabled: false)`).
// - `alternative: bool`    — scope-out per ADR-0054 graded (rejeitado com erro
//                            educacional; passo dedicado futuro condicional).
//
// Diagnóstico P287 §A.2: variant `Content::smartquote(double)` é leaf
// (não rico) — 1 campo bool required. Quando `enabled = false`, função emite
// `Content::Text` directo (não passa pelo variant; estado open/close do
// Layouter preservado intacto).

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

    let mut double: bool = true; // vanilla default
    let mut enabled: bool = true; // vanilla default

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "double" => match value {
                Value::Bool(b) => double = *b,
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
                Value::Bool(b) => enabled = *b,
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
            // Scope-out per diagnóstico P287 §A.2: `alternative` rejeitada
            // com erro educacional (ADR-0054 graded); passo dedicado futuro.
            "alternative" | "quotes" => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                        "smartquote({}:) não suportado neste passo \
                         (P287 §A.2 scope-out / ADR-0054 graded)",
                        key,
                    ),
                )]);
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("smartquote(): argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }

    if !enabled {
        // Paridade vanilla `enabled: false` — emite glyph ASCII literal
        // sem alternância lang-aware. Não passa pelo variant SmartQuote
        // (consumer Layouter não vê este caso).
        let glyph: &str = if double { "\"" } else { "'" };
        return Ok(Value::Content(Content::text(glyph)));
    }

    Ok(Value::Content(Content::smartquote(double)))
}
