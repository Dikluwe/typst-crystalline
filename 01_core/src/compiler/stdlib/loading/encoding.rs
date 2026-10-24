//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/loading/encoding.md
//! @prompt-hash 946d7c96
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
                if !is_equation_constructor_content(content) {
                    let child = &styled.body;
                    fields.insert("child".into(), Value::Content(child.clone()));
                    fields.insert("styles".into(), Value::Str("styles(..)".into()));
                    return Ok(fields);
                }
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
            Content::Source(..)
            | Content::ShowRecipe(..)
            | Content::Located(..)
            | Content::Realized(..) => {
                unreachable!("execution carriers were normalized above")
            }
            Content::Text(..)
            | Content::Space
            | Content::SpaceFromNewline
            | Content::Parbreak
            | Content::HtmlFrame(..)
            | Content::Par { .. }
            | Content::Heading(..)
            | Content::Title(..)
            | Content::Strong(..)
            | Content::Emph(..)
            | Content::Raw(..)
            | Content::ListItem(..)
            | Content::EnumItem(..)
            | Content::Equation(..)
            | Content::MathSequence(..)
            | Content::Symbol(..)
            | Content::MathIdent(..)
            | Content::MathText(..)
            | Content::MathFrac(..)
            | Content::MathAttach(..)
            | Content::MathPrimes(..)
            | Content::MathRoot(..)
            | Content::MathStretch(..)
            | Content::MathDelimited(..)
            | Content::MathLr(..)
            | Content::MathMid(..)
            | Content::MathOverbrace(..)
            | Content::MathUnderbrace(..)
            | Content::MathUnderbracket(..)
            | Content::MathUnderparen(..)
            | Content::MathUndershell(..)
            | Content::MathOverbracket(..)
            | Content::MathOverparen(..)
            | Content::MathOvershell(..)
            | Content::MathAlignPoint(..)
            | Content::Linebreak(..)
            | Content::MathMatrix(..)
            | Content::MathCases(..)
            | Content::MathAccent(..)
            | Content::MathCancel(..)
            | Content::MathUnderline(..)
            | Content::MathOverline(..)
            | Content::MathVec(..)
            | Content::MathClassOverride(..)
            | Content::MathLimitsOverride(..)
            | Content::MathUnderover(..)
            | Content::MathOp(..)
            | Content::MathStyled(..)
            | Content::Label(..)
            | Content::Ref(..)
            | Content::CounterDisplay(..)
            | Content::CounterUpdate(..)
            | Content::Outline(..)
            | Content::OutlineEntry(..)
            | Content::Figure(..)
            | Content::FigureCaption(..)
            | Content::Image(..)
            | Content::Shape(..)
            | Content::Curve(..)
            | Content::Transform(..)
            | Content::Grid(..)
            | Content::GridHeader(..)
            | Content::GridFooter(..)
            | Content::GridCell(..)
            | Content::SetPage(..)
            | Content::PageRun(..)
            | Content::Align(..)
            | Content::Place(..)
            | Content::Flush(..)
            | Content::Divider(..)
            | Content::Terms(..)
            | Content::TermItem(..)
            | Content::Quote(..)
            | Content::Document { .. }
            | Content::Asset { .. }
            | Content::SmartQuote(..)
            | Content::PdfAttach(..)
            | Content::PdfArtifact(..)
            | Content::Underline(..)
            | Content::Strike(..)
            | Content::Overline(..)
            | Content::SmallCaps { .. }
            | Content::Pad(..)
            | Content::Hide(..)
            | Content::HSpace(..)
            | Content::VSpace(..)
            | Content::Pagebreak(..)
            | Content::Colbreak(..)
            | Content::Stack(..)
            | Content::TableCell(..)
            | Content::Bibliography(..)
            | Content::Cite(..)
            | Content::Footnote(..)
            | Content::FootnoteEntry(..)
            | Content::TableHeader(..)
            | Content::TableFooter(..)
            | Content::GridHLine(..)
            | Content::GridVLine(..)
            | Content::TableHLine(..)
            | Content::TableVLine(..)
            | Content::Table(..)
            | Content::Repeat(..)
            | Content::Columns(..)
            | Content::State(..)
            | Content::StateUpdate(..)
            | Content::StateDisplay(..)
            | Content::CounterDisplayCallback(..)
            | Content::ContextBlock(..)
            | Content::Dynamic(..) => {}
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

/// Recognize only local constructor transport; external styles are a barrier.
fn is_equation_constructor_content(content: &crate::entities::content::Content) -> bool {
    use crate::entities::content::Content;
    match content {
        Content::Equation(_) => true,
        Content::Source(source) => is_equation_constructor_content(&source.body),
        Content::ShowRecipe(recipe) => is_equation_constructor_content(&recipe.body),
        Content::Located(located) => is_equation_constructor_content(&located.body),
        Content::Realized(realized) => is_equation_constructor_content(&realized.visual),
        Content::Styled(styled) => {
            let marker = styled
                .styles
                .delta()
                .custom
                .iter()
                .rev()
                .find(|(key, _)| key.as_str() == "equation.constructor")
                .map(|(_, value)| value);
            matches!(marker, Some(Value::Bool(true)))
                && is_equation_constructor_content(&styled.body)
        }
        Content::Empty
        | Content::Text(..)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Sequence(..)
        | Content::HtmlElem(..)
        | Content::HtmlFrame(..)
        | Content::Par { .. }
        | Content::Heading(..)
        | Content::Title(..)
        | Content::Strong(..)
        | Content::Emph(..)
        | Content::Raw(..)
        | Content::ListItem(..)
        | Content::EnumItem(..)
        | Content::Link(..)
        | Content::MathSequence(..)
        | Content::Symbol(..)
        | Content::MathIdent(..)
        | Content::MathText(..)
        | Content::MathFrac(..)
        | Content::MathAttach(..)
        | Content::MathPrimes(..)
        | Content::MathRoot(..)
        | Content::MathStretch(..)
        | Content::MathDelimited(..)
        | Content::MathLr(..)
        | Content::MathMid(..)
        | Content::MathOverbrace(..)
        | Content::MathUnderbrace(..)
        | Content::MathUnderbracket(..)
        | Content::MathUnderparen(..)
        | Content::MathUndershell(..)
        | Content::MathOverbracket(..)
        | Content::MathOverparen(..)
        | Content::MathOvershell(..)
        | Content::MathAlignPoint(..)
        | Content::Linebreak(..)
        | Content::MathMatrix(..)
        | Content::MathCases(..)
        | Content::MathAccent(..)
        | Content::MathCancel(..)
        | Content::MathUnderline(..)
        | Content::MathOverline(..)
        | Content::MathVec(..)
        | Content::MathClassOverride(..)
        | Content::MathLimitsOverride(..)
        | Content::MathUnderover(..)
        | Content::MathOp(..)
        | Content::MathStyled(..)
        | Content::Label(..)
        | Content::Ref(..)
        | Content::CounterDisplay(..)
        | Content::CounterUpdate(..)
        | Content::Outline(..)
        | Content::OutlineEntry(..)
        | Content::Figure(..)
        | Content::FigureCaption(..)
        | Content::Image(..)
        | Content::Shape(..)
        | Content::Curve(..)
        | Content::Transform(..)
        | Content::Grid(..)
        | Content::GridHeader(..)
        | Content::GridFooter(..)
        | Content::GridCell(..)
        | Content::SetPage(..)
        | Content::PageRun(..)
        | Content::Align(..)
        | Content::Place(..)
        | Content::Flush(..)
        | Content::Divider(..)
        | Content::Terms(..)
        | Content::TermItem(..)
        | Content::Quote(..)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(..)
        | Content::PdfAttach(..)
        | Content::PdfArtifact(..)
        | Content::Underline(..)
        | Content::Strike(..)
        | Content::Overline(..)
        | Content::SmallCaps { .. }
        | Content::Pad(..)
        | Content::Hide(..)
        | Content::HSpace(..)
        | Content::VSpace(..)
        | Content::Pagebreak(..)
        | Content::Colbreak(..)
        | Content::Stack(..)
        | Content::Boxed(..)
        | Content::Block(..)
        | Content::TableCell(..)
        | Content::Bibliography(..)
        | Content::Cite(..)
        | Content::Footnote(..)
        | Content::FootnoteEntry(..)
        | Content::TableHeader(..)
        | Content::TableFooter(..)
        | Content::GridHLine(..)
        | Content::GridVLine(..)
        | Content::TableHLine(..)
        | Content::TableVLine(..)
        | Content::Table(..)
        | Content::Repeat(..)
        | Content::Columns(..)
        | Content::Metadata(..)
        | Content::State(..)
        | Content::StateUpdate(..)
        | Content::StateDisplay(..)
        | Content::CounterDisplayCallback(..)
        | Content::ContextBlock(..)
        | Content::Dynamic(..) => false,
    }
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

#[cfg(test)]
mod v16_e_equation_constructor_tests {
    use super::*;
    use crate::entities::content::{
        Content, LocatedContent, RealizedContent, ShowRecipeContent,
    };
    use crate::entities::elements::equation::EquationElem;
    use crate::entities::location::Location;
    use crate::entities::style::Styles;
    use crate::entities::value::IntrospectedContent;
    use std::sync::Arc;

    fn equation() -> Content {
        Content::Equation(Arc::new(EquationElem::new(Content::text("x"), true)))
    }

    fn marked(body: Content) -> Content {
        Content::styled(
            body,
            Styles::new()
                .push_custom("equation.constructor", Value::Bool(true))
                .push_custom("equation.numbering", Value::None)
                .push_custom("equation.alt", Value::Str("alternative".into())),
        )
    }

    fn carrier(body: Content, kind: usize) -> Content {
        let file = crate::entities::file_id::FileId::from_raw(
            std::num::NonZeroU16::new(1).unwrap(),
        );
        match kind {
            0 => body.with_source_span(Span::from_range(file, 1..2)),
            1 => Content::ShowRecipe(Arc::new(ShowRecipeContent {
                body,
                textual: false,
                context_id_floor: 0,
                rules: Arc::from([]),
                styles: crate::entities::style_chain::StyleChain::default_chain(),
                guards: vec![],
                file,
            })),
            2 => Content::Located(Arc::new(LocatedContent {
                location: Location::from_raw(41),
                body,
            })),
            3 => Content::Realized(Arc::new(RealizedContent {
                original: Content::text("not the visual equation"),
                visual: body,
            })),
            _ => unreachable!("bounded test fixture"),
        }
    }

    fn assert_equation_fields(fields: &Dict) {
        assert_eq!(fields.get("func"), Some(&Value::Str("equation".into())));
        assert_eq!(fields.get("block"), Some(&Value::Bool(true)));
        assert_eq!(fields.get("numbering"), Some(&Value::None));
        assert_eq!(fields.get("alt"), Some(&Value::Str("alternative".into())));
        assert_eq!(fields.get("body"), Some(&Value::Content(Content::text("x"))));
        assert!(!fields.contains_key("child"));
        assert!(!fields.contains_key("styles"));
        assert!(!fields.contains_key("equation.constructor"));
        assert!(!fields.contains_key("supplement"));
    }

    #[test]
    fn v16_e_textual_marked_equation_uses_known_fields_instead_of_style_payload() {
        let fields = textual_content_fields(&Value::Content(marked(equation()))).unwrap();
        assert_equation_fields(&fields);
    }

    #[test]
    fn v16_e_textual_constructor_projects_each_causal_carrier_without_original() {
        for kind in 0..4 {
            for content in
                [marked(carrier(equation(), kind)), carrier(marked(equation()), kind)]
            {
                let fields = textual_content_fields(&Value::Content(content)).unwrap();
                assert_equation_fields(&fields);
            }
        }
    }

    #[test]
    fn v16_e_textual_external_styles_and_invalid_markers_remain_structural() {
        let inner = marked(equation());
        let outer = Content::styled(
            inner.clone(),
            Styles::new().push_custom("equation.numbering", Value::Str("(1)".into())),
        );
        let fields = textual_content_fields(&Value::Content(outer)).unwrap();
        assert_eq!(fields.get("func"), Some(&Value::Str("styled".into())));
        assert_eq!(fields.get("child"), Some(&Value::Content(inner)));
        assert_eq!(fields.get("styles"), Some(&Value::Str("styles(..)".into())));
        assert!(!fields.contains_key("numbering"));
        assert!(!fields.contains_key("body"));
        for child in [
            Content::text("plain"),
            Content::sequence(vec![equation(), Content::text("tail")]),
        ] {
            let fields =
                textual_content_fields(&Value::Content(marked(child.clone()))).unwrap();
            assert_eq!(fields.get("func"), Some(&Value::Str("styled".into())));
            assert_eq!(fields.get("child"), Some(&Value::Content(child)));
            assert_eq!(fields.len(), 3);
        }
    }

    #[test]
    fn v16_e_textual_marked_equation_snapshots_remain_authoritative() {
        let content = marked(equation());
        let mut snapshot = new_dict();
        snapshot.insert("numbering".into(), Value::Str("snapshot".into()));
        for snapshot in [new_dict(), snapshot] {
            let expected = snapshot.clone();
            let fields = textual_content_fields(&Value::LocatedContent(
                IntrospectedContent::new(content.clone(), Some(snapshot)),
                Location::from_raw(42),
            ))
            .unwrap();
            assert_eq!(fields.get("func"), Some(&Value::Str("equation".into())));
            assert_eq!(fields.len(), expected.len() + 1);
            for (key, value) in expected {
                assert_eq!(fields.get(&key), Some(&value));
            }
            assert!(!fields.contains_key("child"));
            assert!(!fields.contains_key("body"));
            assert!(!fields.contains_key("alt"));
        }
        let fields = textual_content_fields(&Value::LocatedContent(
            IntrospectedContent::new(content, None),
            Location::from_raw(43),
        ))
        .unwrap();
        assert_equation_fields(&fields);
    }
}

#[cfg(test)]
mod batch_f_fields_tests {
    use super::*;
    use crate::entities::content::Content;
    #[test]
    fn batch_f_fields_delegate_raw_caption_and_keep_order() {
        let raw = textual_content_fields(&Value::Content(Content::raw(
            "é",
            Some("typ".into()),
            false,
        )))
        .unwrap();
        assert_eq!(
            raw.keys().map(|k| k.as_str()).collect::<Vec<_>>(),
            vec!["func", "text", "block", "lang"]
        );
        assert_eq!(raw.get("text"), Some(&Value::Str("é".into())));
        let caption = textual_content_fields(&Value::Content(Content::figure_caption(
            Content::text("C"),
            None,
            None,
        )))
        .unwrap();
        assert_eq!(
            caption.keys().map(|k| k.as_str()).collect::<Vec<_>>(),
            vec!["func", "body"]
        );
        assert_eq!(caption.get("body"), Some(&Value::Content(Content::text("C"))));
        assert_eq!(
            textual_content_fields(&Value::Int(1)).unwrap_err(),
            "expected content"
        );
    }
    #[test]
    fn batch_f_fields_snapshot_overrides_normalized_raw_without_fallback() {
        let file = FileId::from_raw(std::num::NonZeroU16::new(72).unwrap());
        let raw = Content::raw("original", Some("typ".into()), true)
            .with_source_span(Span::from_range(file, 2..7));
        let mut snapshot = new_dict();
        snapshot.insert("lang".into(), Value::None);
        snapshot.insert("text".into(), Value::Str("captured".into()));
        let fields = textual_content_fields(&Value::LocatedContent(
            crate::entities::value::IntrospectedContent::new(raw, Some(snapshot)),
            crate::entities::location::Location::from_raw(72),
        ))
        .unwrap();
        assert_eq!(
            fields.keys().map(|k| k.as_str()).collect::<Vec<_>>(),
            vec!["func", "lang", "text"]
        );
        assert_eq!(fields.get("text"), Some(&Value::Str("captured".into())));
        assert_eq!(fields.get("lang"), Some(&Value::None));
        assert!(!fields.contains_key("block"));
    }
}
