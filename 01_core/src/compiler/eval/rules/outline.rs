//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/rules/outline.md
//! @prompt-hash e76dd708
//! @layer L1
//! @updated 2026-10-05
//!
//! Comportamento de `eval/rules::outline`.

use super::*;

fn contains_heading_outline(content: &Content) -> bool {
    use crate::entities::elements::outline::OutlineTarget;
    match content {
        Content::Outline(e) => e.target == OutlineTarget::Headings,
        Content::Source(e) => contains_heading_outline(&e.body),
        Content::Located(e) => contains_heading_outline(&e.body),
        Content::Realized(e) => contains_heading_outline(&e.visual),
        Content::ShowRecipe(e) => contains_heading_outline(&e.body),
        Content::Styled(e) => contains_heading_outline(&e.body),
        Content::Sequence(parts) => parts.iter().any(contains_heading_outline),
        Content::Table(e) => {
            let children = if let Some(resolved) = &e.resolved {
                &resolved.cells
            } else {
                &e.children
            };
            children.iter().any(contains_heading_outline)
                || (e.resolved.is_none()
                    && e.header.as_ref().is_some_and(contains_heading_outline))
                || (e.resolved.is_none()
                    && e.footer.as_ref().is_some_and(contains_heading_outline))
                || e.caption.as_ref().is_some_and(contains_heading_outline)
        }
        Content::TableCell(e) => {
            let visual = e
                .standalone
                .as_ref()
                .map(|r| &r.visual)
                .or_else(|| e.resolved.as_ref().map(|r| &r.visual))
                .unwrap_or(&e.body);
            contains_heading_outline(visual)
        }
        other => {
            let mut found = false;
            let traversal = other.map_content(&mut |node| {
                if let Content::Outline(e) = node {
                    found |= e.target == OutlineTarget::Headings;
                } else if let Content::Located(e) = node {
                    if let Content::Outline(outline) = &e.body {
                        found |= outline.target == OutlineTarget::Headings;
                    }
                }
                Ok(None)
            });
            // If the proof cannot traverse a carrier, retain the original
            // transforming path so it can report its own error.
            found || traversal.is_err()
        }
    }
}

/// P1406 — substitui cada outline de headings pela sua sequência realizada
/// depois que a introspecção já conhece entries e páginas. A função permanece
/// em L1: L3 fornece apenas Engine/Introspector e orquestra a fase.
pub fn realize_outlines(
    content: Content,
    introspector: &crate::entities::introspector::TagIntrospector,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if !contains_heading_outline(&content) {
        return Ok(content);
    }
    let mut changed = false;
    let mut realize = |node: &Content| -> SourceResult<Option<Content>> {
        if let Content::Located(e) = node {
            if let Content::Outline(outline) = &e.body {
                if outline.target
                    == crate::entities::elements::outline::OutlineTarget::Headings
                {
                    changed = true;
                    let body =
                        realize_heading_outline(outline, introspector, ctx, engine)?;
                    return Ok(Some(Content::Located(Arc::new(
                        crate::entities::content::LocatedContent {
                            location: e.location,
                            body,
                        },
                    ))));
                }
            }
            return Ok(None);
        }
        let Content::Outline(outline) = node else {
            return Ok(None);
        };
        if outline.target != crate::entities::elements::outline::OutlineTarget::Headings {
            return Ok(None);
        }
        changed = true;
        realize_heading_outline(outline, introspector, ctx, engine).map(Some)
    };
    let transformed = content.map_content(&mut realize)?;
    Ok(if changed { transformed } else { content })
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

#[cfg(test)]
mod scan_tests {
    use super::*;
    use crate::entities::content::{LocatedContent, RealizedContent};
    use crate::entities::elements::outline::OutlineTarget;
    use crate::entities::location::Location;

    #[test]
    fn borrowed_scan_detects_only_visible_heading_outlines() {
        let heading = Content::outline();
        let Content::Outline(elem) = &heading else { unreachable!() };
        let mut other = elem.as_ref().clone();
        other.target = OutlineTarget::Figures;
        let other = Content::Outline(Arc::new(other));
        assert!(!contains_heading_outline(&Content::table(
            vec![],
            vec![],
            vec![other.clone()]
        )));
        assert!(contains_heading_outline(&Content::table(
            vec![],
            vec![],
            vec![heading.clone()]
        )));
        let located = Content::Located(Arc::new(LocatedContent {
            location: Location::from_raw(17),
            body: heading.clone(),
        }));
        assert!(contains_heading_outline(&located));
        let hidden = Content::Realized(Arc::new(RealizedContent {
            original: heading,
            visual: other,
        }));
        assert!(!contains_heading_outline(&hidden));
    }
}
