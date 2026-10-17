//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/introspect/outline_entries.md
//! @prompt-hash 0dad9c8f
//! @layer L1
//! @updated 2026-10-17

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

    let outline_locations = intr
        .kind_index
        .get(&ElementKind::Outline)
        .cloned()
        .unwrap_or_default();
    for (outline_index, outline) in outlines.into_iter().enumerate() {
        if outline.target != OutlineTarget::Headings {
            continue;
        }
        let mut entry_locations = Vec::new();
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
            entry_locations.push(entry_location);
        }
        if let Some(anchor) = outline_locations.get(outline_index).copied() {
            intr.counters.observe_locations_after(anchor, &entry_locations);
            intr.state.observe_locations_after(anchor, &entry_locations);
        }
    }
    let counters = &intr.counters;
    if let Some(entries) = intr.kind_index.get_mut(&ElementKind::OutlineEntry) {
        entries.sort_by_key(|location| counters.order_key(*location));
    }
}

#[cfg(test)]
mod v16_walk_order_tests {
    use super::*;
    use crate::entities::{
        counter_update::CounterUpdate, func::Func, state_update::StateUpdate,
    };

    fn selector() -> Selector {
        Selector::NativeElement(Func::native(
            "outline.entry",
            crate::compiler::stdlib::native_outline_entry,
        ))
    }

    #[test]
    fn v16_walk_synthesized_entries_keep_outline_and_direct_entry_order() {
        let tree = Content::sequence(vec![
            Content::outline(),
            Content::heading(1, Content::text("real")),
            Content::outline_entry(1, Content::heading(1, Content::text("direct")), None),
            Content::outline(),
        ]);
        let intr = crate::compiler::introspect::introspect_with_introspector(&tree);
        let entries = intr.query(&selector());
        let bodies: Vec<_> = entries
            .iter()
            .map(|location| {
                let Content::OutlineEntry(entry) =
                    intr.elements[location].content().source_body()
                else {
                    panic!("entry lost")
                };
                entry.body().unwrap().plain_text()
            })
            .collect();
        assert_eq!(bodies, ["real", "direct", "real"]);
        assert_eq!(intr.query(&Selector::Kind(ElementKind::OutlineEntry)), entries);
        let where_selector = Selector::Element {
            function: Func::native(
                "outline.entry",
                crate::compiler::stdlib::native_outline_entry,
            ),
            fields: vec![("level".into(), Value::Int(1))].into(),
        };
        assert_eq!(intr.query(&where_selector), entries);
        assert_eq!(entries.iter().collect::<std::collections::HashSet<_>>().len(), 3);
    }

    #[test]
    fn v16_walk_synthesized_entry_reads_state_and_counter_at_its_outline() {
        let key = CounterKey::Str("probe".into());
        let tree = Content::sequence(vec![
            Content::state("probe", Value::Int(0)),
            Content::state_update("probe", StateUpdate::Set(Box::new(Value::Int(1)))),
            Content::counter_update(key.clone(), CounterUpdate::Set(vec![1])),
            Content::outline(),
            Content::state_update("probe", StateUpdate::Set(Box::new(Value::Int(2)))),
            Content::counter_update(key.clone(), CounterUpdate::Set(vec![2])),
            Content::heading(1, Content::text("h")),
        ]);
        let intr = crate::compiler::introspect::introspect_with_introspector(&tree);
        let entries = intr.query(&selector());
        assert_eq!(entries.len(), 1);
        assert_eq!(intr.state.value_at("probe", entries[0]), Some(&Value::Int(1)));
        assert_eq!(intr.counters.value_at(&key, entries[0]), Some([1].as_slice()));
        assert_eq!(intr.state.final_value("probe"), Some(&Value::Int(2)));
        assert_eq!(intr.counters.value(&key), Some([2].as_slice()));
    }
}
