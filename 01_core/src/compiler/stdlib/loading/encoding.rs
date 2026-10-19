//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/loading/encoding.md
//! @prompt-hash 9795dd45
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) fn canonical_numeric_token(mut token: String) -> String {
    if let Some(exponent) = token.find("e+") {
        token.remove(exponent + 1);
    }
    token
}

/// Reescreve somente tokens numéricos; strings JSON, inclusive escapes e
/// texto semelhante a números, são copiadas sem interpretação.
pub(super) fn textual_content_fields(value: &Value) -> Result<Dict, String> {
    use crate::entities::content::Content;
    let (content, snapshot) = match value {
        Value::Content(content) => (content, None),
        Value::LocatedContent(content, _) => (content.content(), content.fields()),
        _ => return Err("expected content".into()),
    };
    // Wrappers de execução e origem não são variantes públicas do conteúdo.
    let mut content = content;
    let content = loop {
        content = match content {
            Content::Source(source) => &source.body,
            Content::ShowRecipe(recipe) => &recipe.body,
            Content::Located(located) => &located.body,
            Content::Realized(realized) => &realized.visual,
            other => break other,
        };
    };
    let mut fields = new_dict();
    let function = match content {
        Content::Empty => "sequence",
        Content::FigureCaption(_) => "caption",
        other => other.elem_name(),
    };
    fields.insert("func".into(), Value::Str(function.into()));
    if let Some(snapshot) = snapshot {
        fields.extend(snapshot.iter().map(|(name, value)| (name.clone(), value.clone())));
    } else {
        // Campos de payload cuja presença é estrutural. O helper legado de
        // métodos só enumera body para essas famílias e não os pode recuperar.
        match content {
            Content::Empty => {
                fields.insert("children".into(), Value::Array(vec![]));
                return Ok(fields);
            }
            Content::Sequence(children) => {
                fields.insert(
                    "children".into(),
                    Value::Array(children.iter().cloned().map(Value::Content).collect()),
                );
                return Ok(fields);
            }
            Content::Metadata(metadata) => {
                fields.insert("value".into(), metadata.value.as_ref().clone());
                return Ok(fields);
            }
            Content::Styled(styled) => {
                let child = &styled.body;
                fields.insert("child".into(), Value::Content(child.clone()));
                fields.insert("styles".into(), Value::Str("styles(..)".into()));
                return Ok(fields);
            }
            Content::Link(link) => {
                fields.insert("dest".into(), Value::Str(link.url.clone()));
                fields.insert("body".into(), Value::Content(link.body.clone()));
                return Ok(fields);
            }
            Content::Boxed(boxed) => {
                fields.insert("body".into(), Value::Content(boxed.body.clone()));
                return Ok(fields);
            }
            Content::Block(block) => {
                fields.insert("body".into(), Value::Content(block.body.clone()));
                return Ok(fields);
            }
            Content::HtmlElem(element) => {
                fields.insert("tag".into(), Value::Str(element.tag.clone()));
                if let Some(attributes) = &element.attrs {
                    fields.insert(
                        "attrs".into(),
                        Value::Dict(
                            attributes
                                .iter()
                                .map(|(key, value)| {
                                    (key.clone(), Value::Str(value.clone()))
                                })
                                .collect(),
                        ),
                    );
                }
                match &element.body {
                    crate::entities::html::HtmlBody::Unset => {}
                    crate::entities::html::HtmlBody::None => {
                        fields.insert("body".into(), Value::None);
                    }
                    crate::entities::html::HtmlBody::Content(body) => {
                        fields
                            .insert("body".into(), Value::Content(body.as_ref().clone()));
                    }
                }
                return Ok(fields);
            }
            _ => {}
        }
        let projected = crate::compiler::eval::bindings::eval_content_method(
            content,
            "fields",
            Args::positional(vec![]),
            Span::detached(),
        )
        .map_err(|diagnostics| {
            diagnostics
                .into_iter()
                .map(|diagnostic| diagnostic.message)
                .collect::<Vec<_>>()
                .join("; ")
        })?;
        let Value::Dict(projected) = projected else {
            return Err("content fields must be a dictionary".into());
        };
        fields.extend(projected);
    }
    Ok(fields)
}

/// Casts seguem a ordem dos parâmetros do contrato: value, todos os pretty,
/// e então o primeiro argumento remanescente na sequência causal.
pub(super) fn textual_encoder_args(
    args: &Args,
    toml: bool,
    allow_pretty: bool,
) -> SourceResult<(Value, Span, bool)> {
    let mut occurrences = args.occurrence_sequence();
    let Some(index) = occurrences.iter().position(|argument| argument.name.is_none())
    else {
        if let Some(argument) = occurrences
            .iter()
            .find(|argument| argument.name.as_deref() == Some("value"))
        {
            return Err(vec![SourceDiagnostic::error(
                argument.span,
                "the argument `value` is positional",
            )
            .with_hint("try removing `value:`")]);
        }
        return Err(vec![SourceDiagnostic::error(args.span, "missing argument: value")]);
    };
    let argument = occurrences.remove(index);
    if toml && !matches!(argument.value, Value::Dict(_)) {
        return Err(vec![SourceDiagnostic::error(
            argument.value_span,
            format!("expected dictionary, found {}", vanilla_type_name(&argument.value)),
        )]);
    }
    let mut pretty = true;
    if allow_pretty {
        for occurrence in &occurrences {
            if occurrence.name.as_deref() == Some("pretty") {
                match &occurrence.value {
                    Value::Bool(value) => pretty = *value,
                    value => {
                        return Err(vec![SourceDiagnostic::error(
                            occurrence.value_span,
                            format!(
                                "expected boolean, found {}",
                                vanilla_type_name(value)
                            ),
                        )])
                    }
                }
            }
        }
        occurrences.retain(|occurrence| occurrence.name.as_deref() != Some("pretty"));
    }
    if let Some(occurrence) = occurrences.first() {
        let message = match &occurrence.name {
            Some(name) => format!("unexpected argument: {name}"),
            None => "unexpected argument".into(),
        };
        return Err(vec![SourceDiagnostic::error(occurrence.span, message)]);
    }
    Ok((argument.value, argument.value_span, pretty))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_projection_has_func_and_structural_fields() {
        let fields = textual_content_fields(&Value::Content(
            crate::entities::content::Content::Empty,
        ))
        .unwrap();
        assert!(
            matches!(fields.get("func"), Some(Value::Str(name)) if name.as_str() == "sequence")
        );
        assert!(
            matches!(fields.get("children"), Some(Value::Array(children)) if children.is_empty())
        );
    }

    fn sourced(
        content: crate::entities::content::Content,
    ) -> crate::entities::content::Content {
        let file = crate::entities::file_id::FileId::from_raw(
            std::num::NonZeroU16::new(1).unwrap(),
        );
        content.with_source_span(Span::from_range(file, 1..2))
    }

    #[test]
    fn content_serialization_source_keeps_empty_and_sequence_fields() {
        use crate::entities::content::Content;
        for (content, count) in [
            (Content::Empty, 0),
            (
                Content::sequence(vec![
                    Content::text("a"),
                    Content::strong(Content::text("b")),
                ]),
                2,
            ),
        ] {
            let fields =
                textual_content_fields(&Value::Content(sourced(content))).unwrap();
            assert_eq!(fields.get("func"), Some(&Value::Str("sequence".into())));
            let Some(Value::Array(children)) = fields.get("children") else {
                panic!("missing children")
            };
            assert_eq!(children.len(), count);
        }
    }

    #[test]
    fn content_serialization_source_keeps_styled_and_raw_fields() {
        use crate::entities::content::Content;
        let content =
            Content::styled(Content::text("x"), crate::entities::style::Styles::new());
        let fields = textual_content_fields(&Value::Content(sourced(content))).unwrap();
        assert!(
            matches!(fields.get("child"),Some(Value::Content(c)) if c.plain_text()=="x")
        );
        assert_eq!(fields.get("styles"), Some(&Value::Str("styles(..)".into())));
        let fields = textual_content_fields(&Value::Content(sourced(Content::raw(
            "é",
            Some("typc".into()),
            false,
        ))))
        .unwrap();
        assert_eq!(fields.get("text"), Some(&Value::Str("é".into())));
        assert_eq!(fields.get("lang"), Some(&Value::Str("typc".into())));
    }

    #[test]
    fn content_serialization_wrappers_keep_visual_without_executing_recipe() {
        use crate::entities::content::{
            Content, LocatedContent, RealizedContent, ShowRecipeContent,
        };
        use std::sync::Arc;
        let content = Content::ShowRecipe(Arc::new(ShowRecipeContent {
            body: Content::Located(Arc::new(LocatedContent {
                location: crate::entities::location::Location::from_raw(17),
                body: Content::Realized(Arc::new(RealizedContent {
                    original: Content::text("old"),
                    visual: sourced(Content::Empty),
                })),
            })),
            textual: false,
            context_id_floor: 0,
            rules: Arc::from([]),
            styles: crate::entities::style_chain::StyleChain::default_chain(),
            guards: vec![],
            file: crate::entities::file_id::FileId::from_raw(
                std::num::NonZeroU16::new(1).unwrap(),
            ),
        }));
        let fields = textual_content_fields(&Value::Content(content)).unwrap();
        assert_eq!(fields.get("func"), Some(&Value::Str("sequence".into())));
        assert!(matches!(fields.get("children"),Some(Value::Array(c)) if c.is_empty()));
    }

    #[test]
    fn content_serialization_snapshot_none_and_empty_are_distinct() {
        use crate::entities::content::Content;
        use crate::entities::value::IntrospectedContent;
        for snapshot in [None, Some(Default::default())] {
            let present = snapshot.is_some();
            let value = Value::LocatedContent(
                IntrospectedContent::new(sourced(Content::Empty), snapshot),
                crate::entities::location::Location::from_raw(19),
            );
            let fields = textual_content_fields(&value).unwrap();
            assert_eq!(fields.get("func"), Some(&Value::Str("sequence".into())));
            assert_eq!(fields.contains_key("children"), !present);
        }
    }
}

#[cfg(test)]
mod caption_shape_tests {
    use super::*;
    #[test]
    fn caption_shape_textual_projection_keeps_body_and_empty_snapshot() {
        use crate::entities::content::Content;
        use crate::entities::value::IntrospectedContent;
        let c = Content::figure_caption(Content::text("C"), None, None);
        let fields = textual_content_fields(&Value::Content(c.clone())).unwrap();
        assert_eq!(fields.get("func"), Some(&Value::Str("caption".into())));
        assert_eq!(fields.get("body"), Some(&Value::Content(Content::text("C"))));
        let mut locator = crate::entities::locator::Locator::new();
        let fields = textual_content_fields(&Value::LocatedContent(
            IntrospectedContent::new(c, Some(new_dict())),
            locator.next(),
        ))
        .unwrap();
        assert_eq!(fields.len(), 1);
        assert_eq!(fields.get("func"), Some(&Value::Str("caption".into())));
    }
}
