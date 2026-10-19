//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/layout/pad.md
//! @prompt-hash 07b14e0b
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `stdlib/layout::pad`.

use super::*;

/// `pad(body, left: ?, right: ?, top: ?, bottom: ?, x: ?, y: ?, rest: ?)`
/// → `Content::Pad`.
///
/// Resolve a precedência vanilla: específico (`left`/`right`/`top`/`bottom`) >
/// eixo (`x` cobre left+right; `y` cobre top+bottom) > `rest` (cobre os
/// quatro lados). Lados não especificados ficam `None` (P156L refino;
/// per ADR-0064 Caso C — `None` ↔ default vanilla zero, resolvido em
/// momento de uso no Layouter).
///
/// `body` posicional obrigatório (Content ou Str).
/// Padding negativo rejeitado por agora (perfil ADR-0054 graded; vanilla
/// aceita-o mas a semântica em cristalino fica para passo posterior).
pub fn native_pad(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "pad() espera content ou string como primeiro argumento, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "pad() exige body como argumento posicional".to_string(),
            )])
        }
    };

    let sides = extract_sides_lengths(args, "pad")?;

    Ok(Value::Content(Content::pad(body, sides)))
}
