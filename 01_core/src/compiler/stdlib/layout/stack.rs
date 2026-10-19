//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/layout/stack.md
//! @prompt-hash 64e2aa97
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `stdlib/layout::stack`.

use super::*;

/// Coage `Value::Dir` para `Dir` (`ltr`/`rtl`/`ttb`/`btt`).
/// Rejeita strings, tal como o vanilla (`expected direction, found string`).
pub(crate) fn extract_dir(value: &Value) -> SourceResult<Dir> {
    match value {
        Value::Dir(d) => Ok(*d),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("stack(dir:) espera direction, recebeu {}", other.type_name()),
        )]),
    }
}

/// `stack(dir: ttb, spacing: ?, ..children)` → `Content::Stack`.
///
/// **Atributos** (Fase 2 per ADR-0054 graded; **último sub-passo Fase 2**;
/// atinge target 72% Layout):
/// - `children` variádicos posicionais (Content ou Str).
/// - `dir: direction` (`ltr`/`rtl`/`ttb`/`btt`); default `ttb`.
/// - `spacing: Length`; default `None` (zero).
///
/// Sem atributos vanilla scope-out (vanilla stack tem apenas estes 3).
pub fn native_stack(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let mut dir: Dir = Dir::default(); // TTB
    let mut spacing: Option<Length> = None;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "dir" => dir = extract_dir(value)?,
            "spacing" => {
                let len = extract_length(value).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "stack(spacing:) espera length, recebeu {}",
                            value.type_name()
                        ),
                    )]
                })?;
                if len.abs.0 < 0.0 || len.em < 0.0 {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "stack(spacing:): valor negativo não suportado neste passo (P156I)".to_string(),
                    )]);
                }
                spacing = Some(len);
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("stack(): argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }

    // Children variádicos: iterar args.items, aceitar Content ou Str.
    let mut children: Vec<Content> = Vec::with_capacity(args.items.len());
    for v in args.items.iter() {
        match v {
            Value::Content(c) => children.push(c.clone()),
            Value::Str(s) => children.push(Content::text(s.as_str())),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                        "stack(): children devem ser content ou string, recebeu {}",
                        other.type_name()
                    ),
                )])
            }
        }
    }

    Ok(Value::Content(Content::stack(children, dir, spacing)))
}
