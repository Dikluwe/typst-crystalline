//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/layout/pagebreak.md
//! @prompt-hash a9660685
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `stdlib/layout::pagebreak`.

use super::*;

/// `pagebreak(weak: false, to: ?)` → `Content::Pagebreak`.
///
/// Sem argumentos posicionais. `weak` armazenado mas comportamento de
/// collapse adiado (perfil ADR-0054 graded; consistente com P156D).
/// `to` aceita string `"even"` ou `"odd"`; ausente → `None` (sem
/// ajuste de paridade).
pub fn native_pagebreak(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "pagebreak() não aceita argumentos posicionais".to_string(),
        )]);
    }

    let mut weak: bool = false;
    let mut to: Option<Parity> = None;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "weak" => match value {
                Value::Bool(b) => weak = *b,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "pagebreak(weak:) espera bool, recebeu {}",
                            other.type_name()
                        ),
                    )])
                }
            },
            "to" => to = Some(extract_parity(value)?),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("pagebreak(): argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }

    Ok(Value::Content(Content::pagebreak_with_weak_presence(
        weak,
        args.named.contains_key("weak"),
        to,
    )))
}
