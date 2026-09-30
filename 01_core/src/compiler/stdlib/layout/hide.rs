//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/layout/hide.md
//! @prompt-hash 0d1cd9aa
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `stdlib/layout::hide`.

use super::*;

/// `hide(body)` → `Content::Hide`. Sem argumentos nomeados.
pub fn native_hide(
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
                args.span,
                format!("hide() espera content ou string, recebeu {}", other.type_name()),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "hide() exige body como argumento posicional".to_string(),
            )])
        }
    };
    Ok(Value::Content(Content::hide(body)))
}
