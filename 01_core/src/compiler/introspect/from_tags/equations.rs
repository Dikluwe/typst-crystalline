//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/introspect/from_tags/equations.md
//! @prompt-hash 57c83d8d
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Materializa o número visível de cada equação depois que o contador
/// convergiu. O layouter consome apenas `Content` e permanece puro.
pub fn apply_equation_numberings(
    tags: &[Tag],
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
) -> SourceResult<()> {
    use crate::entities::content::Content;
    use crate::entities::counter::CounterKey;
    use crate::entities::element_kind::ElementKind;
    use crate::entities::selector::Selector;
    use crate::entities::source_result::SourceDiagnostic;
    use crate::entities::span::Span;
    use crate::entities::value::Value;

    let key = CounterKey::Selector(Selector::Kind(ElementKind::Equation));
    let mut scopes = Scopes::new(None);
    for tag in tags {
        let Tag::Start(loc, info) = tag else { continue };
        let ElementPayload::Equation { block, numbering_active, .. } = &info.payload
        else {
            continue;
        };
        if !*block || !*numbering_active {
            continue;
        }
        let n = intr
            .counters
            .value_at(&key, *loc)
            .and_then(|v| v.last())
            .copied()
            .unwrap_or(0);
        let content =
            if let Some(callback) = intr.equation_numbering_callbacks.get(loc).cloned() {
                let args = Args::positional(vec![Value::Int(n as i64)]);
                match apply_func(callback, args, &mut scopes, ctx, engine)? {
                    Value::Content(content) => content,
                    Value::Str(text) => Content::text(text.as_str()),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            Span::detached(),
                            format!(
                                "expected content or string, found {}",
                                other.type_name()
                            ),
                        )]);
                    }
                }
            } else if let Some(pattern) = intr.equation_numbering_pattern.get(loc) {
                let text = crate::entities::counter_format::format_counter(&[n], pattern)
                    .unwrap_or_else(|| n.to_string());
                Content::text(&text)
            } else {
                continue;
            };
        intr.equation_numbering_contents.insert(*loc, content);
    }
    Ok(())
}

/// Materializa o suplemento próprio de cada equação com os styles capturados
/// no alvo. O layout consome somente o `Content` resultante.
pub fn apply_equation_supplements(
    tags: &[Tag],
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
) -> SourceResult<()> {
    use crate::compiler::eval::value_to_display_content;
    use crate::compiler::lang::equation_supplement::equation_supplement_for_lang;
    use crate::entities::content::Content;
    use crate::entities::value::Value;

    let mut scopes = Scopes::new(None);
    for tag in tags {
        let Tag::Start(loc, info) = tag else { continue };
        if !matches!(info.payload, ElementPayload::Equation { .. }) {
            continue;
        }
        let specification =
            intr.equation_supplements.get(loc).cloned().unwrap_or(Value::Auto);
        let content = match specification {
            Value::Auto => {
                let lang =
                    intr.equation_supplement_langs.get(loc).and_then(|v| v.as_ref());
                Content::text(equation_supplement_for_lang(lang))
            }
            Value::None => Content::Empty,
            Value::Func(callback) => {
                let base = intr.element_at(*loc).cloned().unwrap_or(Content::Empty);
                let result = apply_func(
                    callback,
                    Args::positional(vec![Value::Content(base)]),
                    &mut scopes,
                    ctx,
                    engine,
                )?;
                value_to_display_content(result).unwrap_or(Content::Empty)
            }
            other => value_to_display_content(other).unwrap_or(Content::Empty),
        };
        intr.equation_supplement_contents.insert(*loc, content);
    }
    Ok(())
}

/// Publish realized occurrence fields without changing the semantic Equation.
pub fn realize_equation_elements(tags: &[Tag], intr: &mut TagIntrospector) {
    use crate::entities::content::Content;
    use crate::entities::value::Value;

    for tag in tags {
        let Tag::Start(loc, info) = tag else { continue };
        if !matches!(info.payload, ElementPayload::Equation { .. }) {
            continue;
        }
        let Some(base) = intr.elements.get(loc).cloned() else { continue };
        if base.fields().is_some() {
            continue;
        }
        let (Some(block), Some(body)) =
            (base.content().get_field("block"), base.content().get_field("body"))
        else {
            continue;
        };
        let mut fields = indexmap::IndexMap::default();
        fields.insert("block".into(), block);
        fields.insert(
            "numbering".into(),
            intr.equation_numberings.get(loc).cloned().unwrap_or(Value::None),
        );
        fields.insert(
            "number-align".into(),
            Value::Align(intr.equation_number_aligns.get(loc).copied().unwrap_or(
                crate::entities::layout_types::Align2D {
                    h: Some(crate::entities::layout_types::HAlign::End),
                    v: Some(crate::entities::layout_types::VAlign::Horizon),
                },
            )),
        );
        fields.insert(
            "supplement".into(),
            Value::Content(
                intr.equation_supplement_contents
                    .get(loc)
                    .cloned()
                    .unwrap_or(Content::Empty),
            ),
        );
        fields.insert(
            "alt".into(),
            intr.equation_alts
                .get(loc)
                .and_then(|alt| alt.clone())
                .map(Value::Str)
                .unwrap_or(Value::None),
        );
        fields.insert("body".into(), body);
        if let Some(label) = &info.label {
            fields.insert("label".into(), Value::Label(label.clone()));
        }
        intr.elements.insert(
            *loc,
            crate::entities::value::IntrospectedContent::new(
                base.into_content(),
                Some(fields),
            ),
        );
    }
}

#[cfg(test)]
mod v16_e_occurrence_tests {
    use super::*;
    use crate::entities::content::Content;
    use crate::entities::element_info::ElementInfo;
    use crate::entities::label::Label;
    use crate::entities::location::Location;
    use crate::entities::value::{IntrospectedContent, Value};

    fn equation_tag(location: Location, label: Option<Label>) -> Tag {
        let payload = crate::compiler::introspect::extract_payload::extract_payload(
            &Content::equation(Content::text("x"), true),
        )
        .expect("equation payload");
        Tag::Start(location, Box::new(ElementInfo { payload, label }))
    }

    #[test]
    fn v16_e_realized_equation_publishes_semantic_snapshot_per_occurrence() {
        let mut intr = TagIntrospector::default();
        let mut tags = Vec::new();
        let mut old_entries = Vec::new();
        for (raw, text) in [(1, "first"), (2, "second")] {
            let location = Location::from_raw(raw);
            let base = IntrospectedContent::new(
                Content::equation(Content::text(text), true),
                None,
            );
            old_entries.push(base.clone());
            intr.elements.insert(location, base);
            intr.equation_numberings.insert(location, Value::Str(text.into()));
            intr.equation_alts.insert(location, Some(text.into()));
            intr.equation_supplement_contents
                .insert(location, Content::text("Equation"));
            tags.push(equation_tag(location, Some(Label(text.into()))));
        }
        realize_equation_elements(&tags, &mut intr);
        for (raw, text) in [(1, "first"), (2, "second")] {
            let entry = &intr.elements[&Location::from_raw(raw)];
            assert_eq!(entry.content().elem_name(), "equation");
            let fields = entry.fields().expect("realized occurrence snapshot");
            assert_eq!(
                fields.keys().map(|key| key.as_str()).collect::<Vec<_>>(),
                [
                    "block",
                    "numbering",
                    "number-align",
                    "supplement",
                    "alt",
                    "body",
                    "label"
                ],
            );
            assert_eq!(fields.get("block"), Some(&Value::Bool(true)));
            assert_eq!(fields.get("numbering"), Some(&Value::Str(text.into())));
            assert_eq!(fields.get("alt"), Some(&Value::Str(text.into())));
            assert_eq!(fields.get("body"), Some(&Value::Content(Content::text(text))));
            assert_eq!(
                fields.get("supplement"),
                Some(&Value::Content(Content::text("Equation")))
            );
            assert_eq!(fields.get("label"), Some(&Value::Label(Label(text.into()))));
            assert_eq!(entry.content(), old_entries[(raw - 1) as usize].content());
        }
        assert!(old_entries.iter().all(|entry| entry.fields().is_none()));
    }

    #[test]
    fn v16_e_realized_equation_retains_effective_defaults_without_label() {
        let location = Location::from_raw(3);
        let mut intr = TagIntrospector::default();
        intr.elements.insert(
            location,
            IntrospectedContent::new(Content::equation(Content::text("x"), true), None),
        );
        intr.equation_supplement_contents
            .insert(location, Content::text("Equation"));
        realize_equation_elements(&[equation_tag(location, None)], &mut intr);
        let entry = &intr.elements[&location];
        let fields = entry.fields().expect("realized defaults snapshot");
        assert_eq!(
            fields.keys().map(|key| key.as_str()).collect::<Vec<_>>(),
            ["block", "numbering", "number-align", "supplement", "alt", "body"]
        );
        assert_eq!(fields.get("numbering"), Some(&Value::None));
        assert_eq!(fields.get("alt"), Some(&Value::None));
        assert_eq!(
            fields.get("number-align"),
            Some(&Value::Align(crate::entities::layout_types::Align2D {
                h: Some(crate::entities::layout_types::HAlign::End),
                v: Some(crate::entities::layout_types::VAlign::Horizon),
            },))
        );
        assert_eq!(
            fields.get("supplement"),
            Some(&Value::Content(Content::text("Equation")))
        );
    }

    #[test]
    fn v16_e_realized_equation_preserves_present_snapshots_and_missing_entries() {
        let location = Location::from_raw(4);
        for known in [false, true] {
            let mut fields = indexmap::IndexMap::default();
            if known {
                fields.insert("numbering".into(), Value::None);
                fields.insert("label".into(), Value::Label(Label("kept".into())));
            }
            let entry = IntrospectedContent::new(
                Content::equation(Content::text("x"), false),
                Some(fields.clone()),
            );
            let old = entry.clone();
            let mut intr = TagIntrospector::default();
            intr.elements.insert(location, entry);
            intr.equation_numberings
                .insert(location, Value::Str("ignored".into()));
            realize_equation_elements(
                &[equation_tag(location, Some(Label("replacement".into())))],
                &mut intr,
            );
            assert_eq!(intr.elements[&location].fields(), Some(&fields));
            assert_eq!(intr.elements[&location].content(), old.content());
            assert_eq!(old.fields(), Some(&fields));
        }
        let mut empty = TagIntrospector::default();
        realize_equation_elements(&[equation_tag(location, None)], &mut empty);
        assert!(empty.elements.is_empty());
        let unrelated = IntrospectedContent::new(Content::text("kept"), None);
        empty.elements.insert(location, unrelated.clone());
        let other_tag = Tag::Start(
            location,
            Box::new(ElementInfo::new(ElementPayload::Heading {
                depth: 1,
                body_hash: 0,
                counter_update: crate::entities::counter_update::CounterUpdate::step(),
                numbering_active: false,
            })),
        );
        realize_equation_elements(&[other_tag], &mut empty);
        assert_eq!(empty.elements[&location], unrelated);
    }
}
