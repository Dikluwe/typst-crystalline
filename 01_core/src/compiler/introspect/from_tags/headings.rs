//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/introspect/from_tags/headings.md
//! @prompt-hash dd0b3d49
//! @layer L1
//! @updated 2026-10-17

use super::*;
use crate::entities::content::Content;
use crate::entities::counter::CounterKey;
use crate::entities::element_kind::ElementKind;
use crate::entities::selector::Selector;
use crate::entities::style_chain::StyleChain;
use crate::entities::value::{IntrospectedContent, Value};

pub(in crate::compiler::introspect) fn apply_heading_supplements(
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
) -> SourceResult<()> {
    let mut scopes = Scopes::new(None);
    let key = CounterKey::Selector(Selector::Kind(ElementKind::Heading));
    let locations: Vec<_> = intr
        .kind_index
        .get(&ElementKind::Heading)
        .into_iter()
        .flatten()
        .filter(|location| {
            intr.elements
                .get(location)
                .is_some_and(|entry| entry.heading_styles().is_some())
        })
        .copied()
        .collect();
    for location in locations {
        let Some(entry) = intr.elements.get(&location) else { continue };
        let Some(styles) = entry.heading_styles() else { continue };
        let Some(mut fields) = entry.fields().cloned() else { continue };
        let base = entry.content().clone();
        let styles = StyleChain::empty().push_styles(styles);
        let specification = fields.get("supplement").cloned().unwrap_or(Value::None);
        let value = if let Value::Func(callback) = specification {
            let argument = Value::LocatedContent(
                IntrospectedContent::new(base.clone(), Some(fields.clone())),
                location,
            );
            let mut scratch = styles;
            std::mem::swap(&mut *engine.styles, &mut scratch);
            let in_context = ctx.in_context;
            let current_location = ctx.current_location.take();
            ctx.in_context = false;
            let result = apply_func(
                callback,
                Args::positional(vec![argument]),
                &mut scopes,
                ctx,
                engine,
            );
            std::mem::swap(&mut *engine.styles, &mut scratch);
            ctx.in_context = in_context;
            ctx.current_location = current_location;
            result?
        } else {
            specification
        };
        let supplement = crate::compiler::eval::value_to_display_content(value)
            .unwrap_or(Content::Empty);
        if let Some(Value::Label(label)) = fields.get("label") {
            let pattern = fields.get("numbering").and_then(|value| {
                if let Value::Str(pattern) = value {
                    Some(pattern.as_str())
                } else {
                    None
                }
            });
            if let Some(numbers) = intr.counter_values_at(&key, location) {
                let number = pattern
                    .and_then(|pattern| {
                        crate::entities::counter_format::format_counter(numbers, pattern)
                    })
                    .or_else(|| intr.formatted_counter_at(&key, location));
                if let Some(number) = number {
                    intr.resolved_labels.insert(
                        label.clone(),
                        format!("{} {number}", supplement.plain_text()),
                    );
                }
            }
        }
        fields.insert("supplement".into(), Value::Content(supplement));
        intr.elements
            .insert(location, IntrospectedContent::new(base, Some(fields)));
    }
    Ok(())
}
