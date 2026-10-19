//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/outline.md
//! @prompt-hash 0c75c442
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
use crate::entities::sides::Sides;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::style::Styles;
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
    let (element, element_location) = match args.items.get(1) {
        Some(Value::Content(element)) => (element.clone(), None),
        Some(Value::LocatedContent(element, location)) => {
            (element.content().clone(), Some(*location))
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "outline.entry(): missing or invalid positional argument `element`",
            )])
        }
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
    let entry = match element_location {
        Some(location) => Content::realized_outline_entry(
            *level as usize,
            element,
            fill,
            None,
            location,
        ),
        None => Content::outline_entry_with_fill_state(
            *level as usize,
            element,
            fill,
            fill_set,
        ),
    };
    Ok(Value::Content(entry))
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

fn outline_entry_receiver(
    args: &Args,
) -> SourceResult<&crate::entities::elements::outline_entry::OutlineEntryElem> {
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

/// Função contextual qualificada `outline.entry.inner(entry)`.
pub fn native_outline_entry_inner(
    ctx: &mut EvalContext,
    args: &Args,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
    scopes: &mut crate::compiler::scopes::Scopes<'_>,
    engine: &mut crate::entities::engine::Engine<'_>,
) -> SourceResult<Value> {
    let entry = outline_entry_receiver(args)?;
    let body = entry
        .body()
        .map_err(|message| vec![SourceDiagnostic::error(args.span, message)])?;
    let page = match native_outline_entry_page(
        ctx,
        args,
        world,
        current_file,
        scopes,
        engine,
    )? {
        Value::Content(page) => page,
        _ => unreachable!("outline.entry.page always returns content"),
    };

    let rtl = matches!(
        engine.styles.custom("text.dir"),
        Some(Value::Dir(crate::entities::dir::Dir::RTL))
    );
    Ok(Value::Content(compose_outline_entry_inner(entry, body, page, rtl)))
}

fn compose_outline_entry_inner(
    entry: &crate::entities::elements::outline_entry::OutlineEntryElem,
    body: Content,
    page: Content,
    rtl: bool,
) -> Content {
    let mut parts = Vec::with_capacity(if entry.fill.is_some() { 8 } else { 6 });
    if rtl {
        parts.push(Content::text("\u{202b}"));
    }
    parts.push(body);
    if rtl {
        parts.push(Content::text("\u{202c}"));
    }
    match &entry.fill {
        Some(fill) => {
            parts.push(Content::Space);
            parts.push(Content::boxed_fractional(fill.clone(), 1.0));
            parts.push(Content::Space);
        }
        None => parts.push(Content::h_space_fraction(1.0, false)),
    }
    parts.push(Content::text("\u{2060}"));
    parts.push(page);
    Content::sequence(parts)
}

fn content_arg(
    value: Option<&Value>,
    name: &str,
    optional: bool,
    span: Span,
) -> SourceResult<Option<Content>> {
    match value {
        Some(Value::Content(content)) => Ok(Some(content.clone())),
        Some(Value::LocatedContent(content, _)) => Ok(Some(content.content().clone())),
        Some(Value::None) if optional => Ok(None),
        Some(value) => Err(vec![SourceDiagnostic::error(
            span,
            if optional {
                format!("expected content or none, found {}", value.type_name())
            } else {
                format!("expected content, found {}", value.type_name())
            },
        )]),
        None => {
            Err(vec![SourceDiagnostic::error(span, format!("missing argument: {name}"))])
        }
    }
}

fn outline_prefix_widths(ctx: &EvalContext, parent_key: i64) -> Vec<Option<f64>> {
    ctx.introspector.outline_prefix_widths(parent_key).to_vec()
}

fn compute_auto_indents(
    widths: &[Option<f64>],
    level: usize,
    prefix_inset: Option<f64>,
    fallback: f64,
) -> (f64, Option<f64>) {
    let index = level.saturating_sub(1);
    let at = |i: usize| widths.get(i).and_then(|value| *value).unwrap_or(fallback);
    let base = (0..index).map(at).sum();
    let hanging = prefix_inset.map(|inset| inset.max(at(index)));
    (base, hanging)
}

fn marked_prefix(prefix: Content) -> Content {
    Content::styled(
        Box::new(prefix),
        Styles::new().push_custom("pdf.marker.tag", Value::Str("label".into())),
    )
}

fn prefix_info(parent_key: i64, level: usize, inset: f64) -> Content {
    Content::styled(
        Box::new(Content::Empty),
        Styles::new().push_custom(
            "outline.prefix-info",
            Value::Array(vec![
                Value::Int(parent_key),
                Value::Int(level as i64),
                Value::Length(crate::entities::layout_types::Length::pt(inset)),
            ]),
        ),
    )
}

struct OutlineIndentPlan {
    prefix_width_pt: Option<f64>,
    base: crate::entities::layout_types::Length,
    hanging_pt: Option<f64>,
    auto: bool,
    parent_key: i64,
    level: usize,
    rtl: bool,
}

fn compose_outline_entry_indented(
    prefix: Option<Content>,
    inner: Content,
    plan: OutlineIndentPlan,
) -> Content {
    let OutlineIndentPlan {
        prefix_width_pt,
        base,
        hanging_pt,
        auto,
        parent_key,
        level,
        rtl,
    } = plan;
    use crate::entities::layout_types::Length;

    let body = match (prefix, prefix_width_pt, hanging_pt) {
        (Some(prefix), Some(prefix_width), Some(hanging)) => {
            let mut parts = Vec::with_capacity(if auto { 5 } else { 4 });
            if auto {
                parts.push(prefix_info(parent_key, level, hanging));
            }
            parts.push(Content::h_space(Length::pt(-hanging), false));
            parts.push(marked_prefix(prefix));
            parts.push(Content::h_space(Length::pt(hanging - prefix_width), false));
            parts.push(inner);
            Content::sequence(parts)
        }
        _ => inner,
    };
    let inset = base + Length::pt(hanging_pt.unwrap_or(0.0));
    let sides = if rtl {
        Sides::new(Length::ZERO, Length::ZERO, inset, Length::ZERO)
    } else {
        Sides::new(inset, Length::ZERO, Length::ZERO, Length::ZERO)
    };
    Content::block(body, None, None, sides, true)
}

/// Função contextual qualificada `outline.entry.indented(entry, prefix, inner, gap:)`.
pub fn native_outline_entry_indented(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
    scopes: &mut crate::compiler::scopes::Scopes<'_>,
    engine: &mut crate::entities::engine::Engine<'_>,
) -> SourceResult<Value> {
    if args.items.len() > 3 || args.named.keys().any(|name| name.as_str() != "gap") {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    let entry = outline_entry_receiver(&Args {
        items: args.items.get(..1).unwrap_or(&[]).to_vec(),
        named: indexmap::IndexMap::default(),
        span: args.span,
        occurrences: None,
    })?
    .clone();
    let prefix = content_arg(args.items.get(1), "prefix", true, args.span)?;
    let inner = content_arg(args.items.get(2), "inner", false, args.span)?
        .expect("inner content is never none");
    let gap = match args.named.get("gap") {
        Some(Value::Length(length)) => *length,
        Some(value) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected length, found {}", value.type_name()),
            )])
        }
        None => crate::entities::layout_types::Length::em(0.5),
    };
    if !ctx.in_context {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "can only be used when context is known",
        )]);
    }
    let Some(indent) = engine.styles.custom("outline.parent.indent").and_then(|value| {
        use crate::entities::elements::outline::OutlineIndent;
        match value {
            Value::Auto => Some(OutlineIndent::Auto),
            Value::Bool(value) => Some(OutlineIndent::Bool(*value)),
            Value::Length(value) => Some(OutlineIndent::Length(*value)),
            Value::Func(value) => Some(OutlineIndent::Function(value.clone())),
            _ => None,
        }
    }) else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "must be called within the context of an outline",
        )]);
    };
    let Some(Value::Int(parent_key)) = engine.styles.custom("outline.parent.key") else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "must be called within the context of an outline",
        )]);
    };
    let parent_key = *parent_key;
    let font_size = engine.styles.size();
    let gap_pt = gap.resolve_pt(font_size);
    let prefix_width_pt = prefix.as_ref().map(|content| {
        crate::compiler::layout::measure_content_real(
            content,
            engine.styles,
            engine.font_metrics,
        )
        .0
    });
    let prefix_inset = prefix_width_pt.map(|width| width + gap_pt);

    use crate::entities::elements::outline::OutlineIndent;
    use crate::entities::layout_types::Length;
    let (base, hanging_pt, auto) = match indent {
        OutlineIndent::Auto | OutlineIndent::Bool(true) => {
            let mut widths = outline_prefix_widths(ctx, parent_key);
            let index = entry.level.saturating_sub(1);
            if widths.len() <= index {
                widths.resize(index + 1, None);
            }
            if let Some(inset) = prefix_inset {
                widths[index] = Some(widths[index].map_or(inset, |old| old.max(inset)));
            }
            let fallback = 1.2 * font_size;
            let (base_pt, hanging) =
                compute_auto_indents(&widths, entry.level, prefix_inset, fallback);
            (Length::pt(base_pt), hanging, true)
        }
        OutlineIndent::Length(amount) => {
            (amount * entry.level.saturating_sub(1) as f64, prefix_inset, false)
        }
        OutlineIndent::Bool(false) => (Length::ZERO, prefix_inset, false),
        OutlineIndent::Function(function) => {
            let call_args =
                Args::positional(vec![Value::Int(entry.level.saturating_sub(1) as i64)]);
            let value = crate::compiler::eval::call_dispatch::apply_func(
                function, call_args, scopes, ctx, engine,
            )?;
            let Value::Length(amount) = value else {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("expected length, found {}", value.type_name()),
                )]);
            };
            (amount, prefix_inset, false)
        }
    };
    let rtl = matches!(
        engine.styles.custom("text.dir"),
        Some(Value::Dir(crate::entities::dir::Dir::RTL))
    );
    let result = Value::Content(compose_outline_entry_indented(
        prefix,
        inner,
        OutlineIndentPlan {
            prefix_width_pt,
            base,
            hanging_pt,
            auto,
            parent_key,
            level: entry.level,
            rtl,
        },
    ));
    ctx.observe_context_read(
        crate::compiler::eval::ContextReadRequest::outline_entry_indented(
            Content::OutlineEntry(std::sync::Arc::new(entry)),
            args.items.get(1).cloned().unwrap_or(Value::None),
            args.items.get(2).cloned().unwrap_or(Value::None),
            gap,
        ),
        args.span,
        Ok(result),
    )
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
    let title_set = args.named.contains_key("title") || !args.items.is_empty();
    // Título: named `title` tem prioridade; fallback para primeiro argumento
    // posicional (vanilla aceita `outline([Título])`).
    let title_named = args.named.get("title").map(|v| match v {
        Value::Content(c) => Ok(Some(c.clone())),
        Value::Str(s) => Ok(Some(Content::text(s.as_str()))),
        Value::None => Ok(None),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "outline(title:): espera content ou string, recebeu {}",
                other.type_name()
            ),
        )]),
    });

    let title_positional = if args.items.is_empty() {
        None
    } else {
        match args.items.first() {
            Some(Value::Content(c)) => Some(Ok(Some(c.clone()))),
            Some(Value::Str(s)) => Some(Ok(Some(Content::text(s.as_str())))),
            Some(Value::None) => Some(Ok(None)),
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

    Ok(Value::Content(Content::Outline(std::sync::Arc::new(
        OutlineElem::with_target_and_title_presence(
            title, title_set, depth, indent, target,
        )
        .with_indent_presence(args.named.contains_key("indent")),
    ))))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::elements::outline_entry::OutlineEntryElem;
    use crate::entities::layout_types::Length;

    #[test]
    fn p1404_inner_default_preserva_morfologia() {
        let fill = Content::repeat(Content::text("."), Some(Length::em(0.15)), true);
        let entry = OutlineEntryElem::new(
            1,
            Content::heading(1, Content::text("H")),
            Some(fill.clone()),
            false,
        );
        let actual = compose_outline_entry_inner(
            &entry,
            Content::text("H"),
            Content::text("1"),
            false,
        );
        let expected = Content::sequence(vec![
            Content::text("H"),
            Content::Space,
            Content::boxed_fractional(fill, 1.0),
            Content::Space,
            Content::text("\u{2060}"),
            Content::text("1"),
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn p1404_inner_none_e_rtl_preservam_controles() {
        let entry = OutlineEntryElem::new(
            1,
            Content::heading(1, Content::strong(Content::text("H"))),
            None,
            true,
        );
        let actual = compose_outline_entry_inner(
            &entry,
            Content::strong(Content::text("H")),
            Content::text("II"),
            true,
        );
        let expected = Content::sequence(vec![
            Content::text("\u{202b}"),
            Content::strong(Content::text("H")),
            Content::text("\u{202c}"),
            Content::h_space_fraction(1.0, false),
            Content::text("\u{2060}"),
            Content::text("II"),
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn p1405_indented_fixo_preserva_content_e_inset_logico() {
        let actual = compose_outline_entry_indented(
            Some(Content::strong(Content::text("P"))),
            Content::emph(Content::text("I")),
            OutlineIndentPlan {
                prefix_width_pt: Some(6.0),
                base: Length::em(2.0),
                hanging_pt: Some(11.5),
                auto: false,
                parent_key: 0,
                level: 2,
                rtl: false,
            },
        );
        let Content::Block(block) = actual else { panic!("esperado Block") };
        assert_eq!(block.inset.left, Length::em(2.0) + Length::pt(11.5));
        assert_eq!(block.inset.right, Length::ZERO);
        let Content::Sequence(parts) = &block.body else { panic!("esperada sequência") };
        assert_eq!(parts.len(), 4);
        assert!(
            matches!(&parts[1], Content::Styled(styled) if matches!(&styled.body, Content::Strong(_)))
        );
        assert_eq!(parts[3], Content::emph(Content::text("I")));
    }

    #[test]
    fn p1405_indented_sem_prefix_mantem_inner_e_rtl() {
        let inner = Content::strong(Content::text("I"));
        let actual = compose_outline_entry_indented(
            None,
            inner.clone(),
            OutlineIndentPlan {
                prefix_width_pt: None,
                base: Length::pt(13.2),
                hanging_pt: None,
                auto: true,
                parent_key: 9,
                level: 2,
                rtl: true,
            },
        );
        let Content::Block(block) = actual else { panic!("esperado Block") };
        assert_eq!(block.inset.left, Length::ZERO);
        assert_eq!(block.inset.right, Length::pt(13.2));
        assert_eq!(block.body, inner);
    }

    #[test]
    fn p1405_auto_soma_niveis_anteriores_e_alinha_maximo_atual() {
        let widths = vec![Some(13.2), Some(26.4)];
        assert_eq!(
            compute_auto_indents(&widths, 2, Some(11.0), 13.2),
            (13.2, Some(26.4))
        );
        assert_eq!(compute_auto_indents(&[], 2, None, 13.2), (13.2, None));
    }
}
