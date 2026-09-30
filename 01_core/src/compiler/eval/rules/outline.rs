//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/rules/outline.md
//! @prompt-hash 60632d9f
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `eval/rules::outline`.

use super::*;

/// P1406 — substitui cada outline de headings pela sua sequência realizada
/// depois que a introspecção já conhece entries e páginas. A função permanece
/// em L1: L3 fornece apenas Engine/Introspector e orquestra a fase.
pub fn realize_outlines(
    content: Content,
    introspector: &crate::entities::introspector::TagIntrospector,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    let mut realize = |node: &Content| -> SourceResult<Option<Content>> {
        let Content::Outline(outline) = node else {
            return Ok(None);
        };
        if outline.target != crate::entities::elements::outline::OutlineTarget::Headings {
            return Ok(None);
        }
        realize_heading_outline(outline, introspector, ctx, engine).map(Some)
    };
    content.map_content(&mut realize)
}

pub(crate) fn realize_heading_outline(
    outline: &crate::entities::elements::outline::OutlineElem,
    introspector: &crate::entities::introspector::TagIntrospector,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    use crate::entities::layout_types::Length;

    let previous_styles = engine.styles.clone();
    let previous_rules = Arc::clone(&*engine.show_rules);
    let previous_context = ctx.in_context;
    let previous_intr = std::mem::replace(&mut ctx.introspector, introspector.clone());

    let parent_styles = Styles::new()
        .push_custom(
            "outline.parent.indent",
            match &outline.indent {
                crate::entities::elements::outline::OutlineIndent::Auto => Value::Auto,
                crate::entities::elements::outline::OutlineIndent::Bool(value) => {
                    Value::Bool(*value)
                }
                crate::entities::elements::outline::OutlineIndent::Length(value) => {
                    Value::Length(*value)
                }
                crate::entities::elements::outline::OutlineIndent::Function(value) => {
                    Value::Func(value.clone())
                }
            },
        )
        .push_custom("outline.parent.key", Value::Int(outline.parent_key()));
    *engine.styles = StyleChain::default_chain()
        .push_styles(outline.captured_styles())
        .push_styles(&parent_styles);
    *engine.show_rules = Arc::from(outline.show_rules().to_vec());
    ctx.in_context = true;

    let result = (|| {
        let mut parts = Vec::new();
        if !outline.title_was_set() {
            let title = crate::compiler::lang::outline_title::outline_title_for_lang(
                engine.styles.lang().as_ref(),
            );
            parts.push(Content::heading(1, Content::text(title)));
        } else if let Some(title) = &outline.title {
            parts.push(Content::heading(1, title.clone()));
        }

        for (((label, number, _body, level), source_location), source) in introspector
            .headings_for_toc
            .iter()
            .zip(&introspector.outlined_heading_locations)
            .filter_map(|(heading, location)| {
                introspector
                    .elements
                    .get(location)
                    .map(|source| ((heading, location), source))
            })
        {
            if *level > outline.depth {
                continue;
            }
            let fill = match engine.styles.custom("outline.entry.fill") {
                Some(Value::Content(content)) => Some(content.clone()),
                Some(Value::None) => None,
                _ => Some(Content::repeat(
                    Content::text("."),
                    Some(Length::em(0.15)),
                    true,
                )),
            };
            let prefix = number.as_ref().map(|value| Content::text(value.as_str()));
            let entry = Content::realized_outline_entry(
                *level,
                source.content().clone(),
                fill,
                prefix,
                *source_location,
            );
            let relevant: Vec<ShowRule> = outline
                .show_rules()
                .iter()
                .filter(|rule| {
                    selector_matching::is_node_rule(&rule.selector)
                        && selector_matching::selector_matches(&entry, &rule.selector)
                })
                .cloned()
                .collect();
            let has_transform = relevant
                .iter()
                .any(|rule| !matches!(rule.transform, Transformation::Style(_)));
            let realized = if has_transform {
                apply_show_rules(entry, &relevant, ctx, engine)?
            } else {
                let Content::OutlineEntry(entry_elem) = &entry else { unreachable!() };
                let mut scopes = Scopes::new(None);
                let receiver = Value::Content(entry.clone());
                let args = Args::positional(vec![receiver.clone()]);
                let inner = match crate::compiler::stdlib::native_outline_entry_inner(
                    ctx,
                    &args,
                    engine.world,
                    engine.current_file,
                    &mut scopes,
                    engine,
                )? {
                    Value::Content(content) => content,
                    _ => unreachable!("outline.entry.inner always returns content"),
                };
                let prefix =
                    entry_elem.prefix().map(Value::Content).unwrap_or(Value::None);
                let args =
                    Args::positional(vec![receiver, prefix, Value::Content(inner)]);
                let body = match crate::compiler::stdlib::native_outline_entry_indented(
                    ctx,
                    &args,
                    engine.world,
                    engine.current_file,
                    &mut scopes,
                    engine,
                )? {
                    Value::Content(content) => content,
                    _ => unreachable!("outline.entry.indented always returns content"),
                };
                let linked = Content::styled(
                    Box::new(body),
                    Styles::new().push_custom(
                        "outline.link.destination",
                        Value::Str(label.0.clone().into()),
                    ),
                );
                let mut set_chain = StyleChain::empty();
                for rule in &relevant {
                    if let Transformation::Style(styles) = &rule.transform {
                        set_chain = set_chain.push(styles.delta().clone());
                    }
                }
                let delta = set_chain.collapse();
                if delta.is_empty() {
                    linked
                } else {
                    Content::styled(Box::new(linked), Styles::from_delta(delta))
                }
            };
            parts.push(Content::styled(Box::new(realized), parent_styles.clone()));
        }
        Ok(Content::Outline(Arc::new(
            outline.with_realized_body(Content::sequence(parts)),
        )))
    })();

    *engine.styles = previous_styles;
    *engine.show_rules = previous_rules;
    ctx.in_context = previous_context;
    ctx.introspector = previous_intr;
    result
}
