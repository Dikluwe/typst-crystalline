//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/introspect/outline_entries.md
//! @prompt-hash 70048e28
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// P1402 — projeta as entradas públicas depois que o walk conhece todos os
/// headings. A Location da entry é própria; o campo `element` retém a Location
/// causal do heading.
pub(super) fn materialize_outline_entries(
    locator: &mut Locator,
    intr: &mut TagIntrospector,
) {
    use crate::entities::elements::outline::OutlineTarget;
    use crate::entities::layout_types::Length;

    let outlines = intr.outline_elements.clone();
    let headings = intr.headings_for_toc.clone();
    let heading_locations = intr.outlined_heading_locations.clone();

    for outline in outlines {
        if outline.target != OutlineTarget::Headings {
            continue;
        }
        for ((_label, number, _body, level), source_location) in
            headings.iter().zip(&heading_locations)
        {
            if *level > outline.depth {
                continue;
            }
            let Some(source) = intr.elements.get(source_location).cloned() else {
                continue;
            };
            let element = source.content().clone();
            let fill = Content::repeat(Content::text("."), Some(Length::em(0.15)), true);
            let prefix = number.as_ref().map(|number| Content::text(number.as_str()));
            let entry = Content::realized_outline_entry(
                *level,
                element,
                Some(fill.clone()),
                prefix,
                *source_location,
            );
            let mut fields = indexmap::IndexMap::default();
            fields.insert("level".into(), Value::Int(*level as i64));
            fields.insert(
                "element".into(),
                Value::LocatedContent(source, *source_location),
            );
            fields.insert("fill".into(), Value::Content(fill));
            let entry_location = locator.next();
            intr.elements.insert(
                entry_location,
                crate::entities::value::IntrospectedContent::new(entry, Some(fields)),
            );
            intr.kind_index
                .entry(ElementKind::OutlineEntry)
                .or_default()
                .push(entry_location);
        }
    }
}
