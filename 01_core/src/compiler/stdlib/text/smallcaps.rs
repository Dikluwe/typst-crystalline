//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/text/smallcaps.md
//! @prompt-hash 66b8a68b
//! @layer L1
//! @updated 2026-08-13
//!
//! `smallcaps` — versaletes.
//!
//! Extraído de `stdlib/text.rs` (2026-08-13) conforme ADR-0109 — atomização
//! forma B: a lógica vive no ficheiro da unidade, o hub é só fronteira.

use crate::compiler::eval::EvalContext;
use crate::compiler::stdlib::expect_no_named;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;


// ── Passo 408 — `smallcaps(body)` ───────────────────────────────────────────
//
// Paridade vanilla `text/smallcaps.rs::SmallcapsElem`: elemento de texto que
// transforma o body em small capitals. Consumer real requer shaping OpenType
// (`smcp` / `c2sc`) — DEBT-53 scope-out XL. Neste passo materializa-se o
// variant `Content::SmallCaps` e a stdlib; o consumer em layout é stub
// transparente (output byte-idêntico ao body).
//
// Modelo minimal: só `body` (sem atributos opcionais). Aceita `content` ou
// `string` como argumento posicional único; named args são rejeitados.

/// `smallcaps(body)` → content embrulhado em `Content::SmallCaps`.
pub fn native_smallcaps(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;

    let body = match args.items.as_slice() {
        [Value::Content(c)] => c.clone(),
        [Value::Str(s)] => Content::text(s.as_str()),
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "smallcaps() espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "smallcaps() exige body como argumento posicional".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "smallcaps() recebeu {} argumentos posicionais (espera 1)",
                    args.items.len()
                ),
            )])
        }
    };

    Ok(Value::Content(Content::smallcaps(body)))
}
