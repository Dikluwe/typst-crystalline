//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/introspect/from_tags/equations.md
//! @prompt-hash 7db15ba1
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

/// Substitui a cópia nua guardada por `element_at` pela visão pública e
/// realizada da equação. Isso preserva os valores efetivos usados por query
/// sem acrescentá-los ao `EquationElem` de domínio.
pub fn realize_equation_elements(tags: &[Tag], intr: &mut TagIntrospector) {
    use crate::entities::content::Content;
    use crate::entities::style::Styles;
    use crate::entities::value::Value;

    for tag in tags {
        let Tag::Start(loc, info) = tag else { continue };
        if !matches!(info.payload, ElementPayload::Equation { .. }) {
            continue;
        }
        let Some(base) = intr.elements.get(loc).cloned() else { continue };
        let mut styles = Styles::new()
            .push_custom(
                "equation.numbering",
                intr.equation_numberings.get(loc).cloned().unwrap_or(Value::None),
            )
            .push_custom(
                "equation.number-align",
                Value::Align(intr.equation_number_aligns.get(loc).copied().unwrap_or(
                    crate::entities::layout_types::Align2D {
                        h: Some(crate::entities::layout_types::HAlign::End),
                        v: Some(crate::entities::layout_types::VAlign::Horizon),
                    },
                )),
            )
            .push_custom(
                "equation.supplement",
                Value::Content(
                    intr.equation_supplement_contents
                        .get(loc)
                        .cloned()
                        .unwrap_or(Content::Empty),
                ),
            );
        styles = styles.push_custom(
            "equation.alt",
            intr.equation_alts
                .get(loc)
                .and_then(|alt| alt.clone())
                .map(Value::Str)
                .unwrap_or(Value::None),
        );
        intr.elements.insert(
            *loc,
            crate::entities::value::IntrospectedContent::new(
                Content::styled(Box::new(base.into_content()), styles),
                None,
            ),
        );
    }
}
