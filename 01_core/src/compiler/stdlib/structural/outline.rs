//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/outline.md
//! @prompt-hash 3391036f
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
use crate::entities::introspector::Introspector;
use crate::entities::numbering::Numbering;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

/// `outline.entry(level, element, fill:?)` — construtor público P1402.
pub fn native_outline_entry(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let Some(Value::Int(level)) = args.items.first() else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "outline.entry(): missing or invalid positional argument `level`",
        )]);
    };
    if *level < 1 {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "outline.entry(): level must be at least 1",
        )]);
    }
    let Some(Value::Content(element)) = args.items.get(1) else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "outline.entry(): missing or invalid positional argument `element`",
        )]);
    };
    if args.items.len() != 2 {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    if args.named.keys().any(|name| name.as_str() != "fill") {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    let (fill, fill_set) = match args.named.get("fill") {
        Some(Value::Content(fill)) => (Some(fill.clone()), true),
        Some(Value::None) => (None, true),
        Some(_) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "outline.entry(fill:): expected content or none",
            )])
        }
        None => (
            Some(Content::repeat(
                Content::text("."),
                Some(crate::entities::layout_types::Length::em(0.15)),
                true,
            )),
            false,
        ),
    };
    Ok(Value::Content(Content::outline_entry_with_fill_state(
        *level as usize,
        element.clone(),
        fill,
        fill_set,
    )))
}

/// Função qualificada `outline.entry.body(entry)`.
pub fn native_outline_entry_body(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let Some(first) = args.items.first() else {
        return Err(vec![SourceDiagnostic::error(args.span, "missing argument: self")]);
    };
    if args.items.len() != 1 || !args.named.is_empty() {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    let content = match first {
        Value::Content(content) => content,
        Value::LocatedContent(content, _) => content.content(),
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "expected outline entry",
            )])
        }
    };
    let Content::OutlineEntry(entry) = content else {
        return Err(vec![SourceDiagnostic::error(args.span, "expected outline entry")]);
    };
    entry
        .body()
        .map(Value::Content)
        .map_err(|message| vec![SourceDiagnostic::error(args.span, message)])
}

fn outline_entry_receiver<'a>(
    args: &'a Args,
) -> SourceResult<&'a crate::entities::elements::outline_entry::OutlineEntryElem> {
    let Some(first) = args.items.first() else {
        return Err(vec![SourceDiagnostic::error(args.span, "missing argument: self")]);
    };
    if args.items.len() != 1 || !args.named.is_empty() {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    let content = match first {
        Value::Content(content) => content,
        Value::LocatedContent(content, _) => content.content(),
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "expected outline entry",
            )])
        }
    };
    let Content::OutlineEntry(entry) = content else {
        return Err(vec![SourceDiagnostic::error(args.span, "expected outline entry")]);
    };
    Ok(entry)
}

/// Função qualificada `outline.entry.prefix(entry)`.
pub fn native_outline_entry_prefix(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    Ok(outline_entry_receiver(args)?
        .prefix()
        .map(Value::Content)
        .unwrap_or(Value::None))
}

/// Função contextual qualificada `outline.entry.page(entry)`.
pub fn native_outline_entry_page(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
    scopes: &mut crate::compiler::scopes::Scopes<'_>,
    engine: &mut crate::entities::engine::Engine<'_>,
) -> SourceResult<Value> {
    let entry = outline_entry_receiver(args)?;
    let Some(location) = entry.element_location() else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("{} must have a location", entry.element.elem_name()),
        )
        .with_hint("try customizing `outline.entry` with a show rule")]);
    };
    let result = realize_outline_entry_page(location, args.span, scopes, ctx, engine)?;
    ctx.observe_context_read(
        crate::compiler::eval::ContextReadRequest::OutlineEntryPage { location },
        args.span,
        Ok(result),
    )
}

pub(crate) fn realize_outline_entry_page(
    location: crate::entities::location::Location,
    span: Span,
    scopes: &mut crate::compiler::scopes::Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut crate::entities::engine::Engine<'_>,
) -> SourceResult<Value> {
    let page = ctx.introspector.page(location).map(|page| page.get()).unwrap_or(1);
    let numbering = ctx
        .introspector
        .page_numbering(location)
        .cloned()
        .unwrap_or_else(|| Numbering::Pattern("1".into()));
    let content = crate::compiler::stdlib::numbering::realize_numbering(
        &numbering,
        &[page],
        span,
        scopes,
        ctx,
        engine,
    )?;
    Ok(Value::Content(content))
}

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
