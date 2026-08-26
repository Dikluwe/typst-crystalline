//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/outline.md
//! @prompt-hash 4c3d3673
//! @layer L1
//! @updated 2026-08-13
//!
//! `outline`, `lof`, `lot` — sumários sobre `OutlineElem`.
//!
//! Separado de `structural/sectioning.rs` em 2026-08-13: a fronteira anterior
//! estava sustentada por um cluster de co-mudança que não existia (artefacto
//! de atribuição da ferramenta). Fronteira nova medida no L0 deste nó.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::elements::outline::OutlineElem;
use crate::entities::elements::outline::OutlineTarget;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

/// `outline(title: content?, depth: int?, indent: bool?)` — emite
/// `Content::Outline(OutlineElem { title, depth, indent })`.
///
/// P457: parâmetros settable do vanilla. Defaults: title=None (renderiza
/// "Índice" no layout), depth=3, indent=true.
pub fn native_outline(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // Título: named `title` tem prioridade; fallback para primeiro argumento
    // posicional (vanilla aceita `outline([Título])`).
    let title_named = args.named.get("title").and_then(|v| match v {
        Value::Content(c) => Some(Ok(Some(c.clone()))),
        Value::Str(s) => Some(Ok(Some(Content::text(s.as_str())))),
        Value::None => None,
        other => Some(Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "outline(title:): espera content ou string, recebeu {}",
                other.type_name()
            ),
        )])),
    });

    let title_positional = if args.items.is_empty() {
        None
    } else {
        match args.items.first() {
            Some(Value::Content(c)) => Some(Ok(Some(c.clone()))),
            Some(Value::Str(s)) => Some(Ok(Some(Content::text(s.as_str())))),
            Some(other) => Some(Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "outline(): título espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])),
            None => None,
        }
    };

    let title = match (title_named, title_positional) {
        (Some(Ok(Some(_))), Some(Ok(Some(_)))) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "outline(): não pode usar título posicional e named `title` simultaneamente".to_string(),
            )])
        }
        (Some(Ok(t)), _) => t,
        (Some(Err(e)), _) => return Err(e),
        (None, Some(Ok(t))) => t,
        (None, Some(Err(e))) => return Err(e),
        (None, None) => None,
    };

    let depth = match args.named.get("depth") {
        Some(Value::Int(n)) => {
            let d = *n as usize;
            if d == 0 {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "outline(depth:): depth deve ser >= 1".to_string(),
                )]);
            }
            d
        }
        Some(Value::None) | None => 3,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("outline(depth:): espera int, recebeu {}", other.type_name()),
            )])
        }
    };

    use crate::entities::elements::outline::OutlineIndent;
    let indent = match args.named.get("indent") {
        Some(Value::Bool(b)) => OutlineIndent::Bool(*b),
        Some(Value::Length(l)) => OutlineIndent::Length(*l),
        Some(Value::Func(f)) => OutlineIndent::Function(f.clone()),
        Some(Value::Auto) | Some(Value::None) | None => OutlineIndent::Auto,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "outline(indent:): espera length, function, auto ou bool, recebeu {}",
                    other.type_name()
                ),
            )])
        }
    };

    // **P472** — argumento `target:` opcional: "headings" | "figures" | "tables"
    let target = match args.named.get("target") {
        Some(Value::Str(s)) => match s.as_str() {
            "headings" | "heading" => OutlineTarget::Headings,
            "figures"  | "figure"  => OutlineTarget::Figures,
            "tables"   | "table"   => OutlineTarget::Tables,
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("outline(target:): valor \"{}\" desconhecido; esperado \"headings\", \"figures\" ou \"tables\"", other),
            )]),
        },
        Some(Value::None) | None => OutlineTarget::Headings,
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("outline(target:): espera string, recebeu {}", other.type_name()),
        )]),
    };

    Ok(Value::Content(Content::Outline(std::sync::Arc::new(OutlineElem::with_target(
        title, depth, indent, target,
    )))))
}

/// **P472** — `lof(title:?)` — emite `Content::Outline` com `target = Figures`.
/// Alias de `outline(target: "figures")`. Title opcional.
pub fn native_lof(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let title = match args.named.get("title") {
        Some(Value::Content(c)) => Some(c.clone()),
        Some(Value::Str(s)) => Some(Content::text(s.as_str())),
        Some(Value::None) | None => None,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "lof(title:): espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
    };
    Ok(Value::Content(Content::lof(title)))
}

/// **P472** — `lot(title:?)` — emite `Content::Outline` com `target = Tables`.
/// Alias de `outline(target: "tables")`. Title opcional.
pub fn native_lot(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let title = match args.named.get("title") {
        Some(Value::Content(c)) => Some(c.clone()),
        Some(Value::Str(s)) => Some(Content::text(s.as_str())),
        Some(Value::None) | None => None,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "lot(title:): espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
    };
    Ok(Value::Content(Content::lot(title)))
}
