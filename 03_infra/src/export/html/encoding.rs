//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/html/encoding.md
//! @prompt-hash 44e9a9ab
//! @layer L3
//! @updated 2026-10-06

use super::HtmlSerializationMode;
use typst_core::entities::content::Content;
use typst_core::entities::source_result::SourceDiagnostic;
use typst_core::entities::span::Span;

fn visual_content(mut content: &Content) -> &Content {
    loop {
        content = match content {
            Content::Source(e) => &e.body,
            Content::ShowRecipe(e) => &e.body,
            Content::Located(e) => &e.body,
            Content::Realized(e) => &e.visual,
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
            | Content::Equation(..)
            | Content::MathSequence(..)
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
            | Content::Styled(..)
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
            | Content::Dynamic(..) => return content,
        };
    }
}

fn title_boundary(content: &Content) -> &Content {
    match visual_content(content) {
        Content::Styled(styled) => title_boundary(&styled.body),
        content => content,
    }
}

pub(super) fn raw_text(content: &Content) -> Result<String, SourceDiagnostic> {
    Ok(match visual_content(content) {
        Content::Empty => String::new(),
        Content::Text(text) => text.to_string(),
        Content::Space | Content::SpaceFromNewline => " ".into(),
        Content::Linebreak(_) => "\n".into(),
        Content::Sequence(items) => {
            items.iter().map(raw_text).collect::<Result<Vec<_>, _>>()?.concat()
        }
        Content::Styled(styled) => raw_text(&styled.body)?,
        _ => {
            return Err(SourceDiagnostic::error(
                Span::detached(),
                "HTML raw text element cannot have non-text children",
            ))
        }
    })
}

pub(super) fn title_text_with_context(
    content: &Content,
    chain: &typst_core::entities::style_chain::StyleChain,
    context: &mut typst_core::compiler::lang::quotes::SmartQuoteContext,
) -> Result<String, SourceDiagnostic> {
    Ok(match visual_content(content) {
        Content::Empty => String::new(),
        Content::Text(text) => {
            context.observe_text(text);
            text.as_str().replace('&', "&amp;").replace('<', "&lt;")
        }
        Content::Space | Content::SpaceFromNewline => {
            context.observe_text(" ");
            " ".into()
        }
        Content::Linebreak(_) => {
            context.observe_text("\n");
            "\n".into()
        }
        Content::Sequence(items) => {
            let mut out = String::new();
            let mut after_linebreak = false;
            for item in items.iter() {
                if after_linebreak
                    && matches!(
                        title_boundary(item),
                        Content::Space | Content::SpaceFromNewline
                    )
                {
                    after_linebreak = false;
                    continue;
                }
                out.push_str(&title_text_with_context(item, chain, context)?);
                after_linebreak = matches!(title_boundary(item), Content::Linebreak(_));
            }
            out
        }
        Content::Styled(styled) => title_text_with_context(
            &styled.body,
            &chain.push_styles(&styled.styles),
            context,
        )?,
        Content::SmartQuote(e) => {
            let glyph = typst_core::compiler::lang::quotes::resolve_smart_quote(
                context, e, chain, None,
            )
            .map_err(|e| e.into_iter().next().expect("quote diagnostic"))?;
            glyph.as_str().replace('&', "&amp;").replace('<', "&lt;")
        }
        _ => {
            return Err(SourceDiagnostic::error(
                Span::detached(),
                "HTML raw text element cannot have non-text children",
            ))
        }
    })
}

pub(super) fn escape_text(text: &str, mode: HtmlSerializationMode) -> String {
    let escaped = text.replace('&', "&amp;").replace('<', "&lt;");
    match mode {
        HtmlSerializationMode::Crystalline => {
            escaped.replace('>', "&gt;").replace('"', "&quot;")
        }
        HtmlSerializationMode::Vanilla => escaped,
    }
}

pub(super) fn escape_attribute(text: &str, mode: HtmlSerializationMode) -> String {
    let escaped = text.replace('&', "&amp;").replace('"', "&quot;");
    match mode {
        HtmlSerializationMode::Crystalline => {
            escaped.replace('<', "&lt;").replace('>', "&gt;")
        }
        HtmlSerializationMode::Vanilla => escaped,
    }
}

#[cfg(test)]
mod carrier_tests {
    use super::*;

    use std::sync::Arc;
    use typst_core::entities::content::{
        LocatedContent, RealizedContent, ShowRecipeContent, SourceContent,
    };
    use typst_core::entities::span::Span;
    use typst_core::entities::style_chain::StyleChain;

    fn carriers(body: Content) -> Vec<Content> {
        vec![
            Content::Source(Arc::new(SourceContent {
                body: body.clone(),
                span: Span::detached(),
            })),
            Content::Located(Arc::new(LocatedContent {
                body: body.clone(),
                location: typst_core::entities::locator::Locator::new().next(),
            })),
            Content::Realized(Arc::new(RealizedContent {
                original: Content::text("must-not-render"),
                visual: body.clone(),
            })),
            Content::ShowRecipe(Arc::new(ShowRecipeContent {
                body,
                textual: false,
                context_id_floor: 0,
                rules: Arc::from([]),
                styles: StyleChain::empty(),
                guards: vec![],
                file: typst_core::entities::file_id::FileId::from_raw(
                    std::num::NonZeroU16::new(1).unwrap(),
                ),
            })),
        ]
    }

    #[test]
    fn carrier_text_encoding_preserves_visual_and_nontext_rejection() {
        for wrapped in carriers(Content::text("A & < B")) {
            assert_eq!(raw_text(&wrapped).unwrap(), "A & < B");
            assert_eq!(
                title_text_with_context(
                    &wrapped,
                    &StyleChain::empty(),
                    &mut Default::default()
                )
                .unwrap(),
                "A &amp; &lt; B"
            );
            for nested in carriers(wrapped) {
                assert_eq!(raw_text(&nested).unwrap(), "A & < B");
            }
        }
        for wrapped in carriers(Content::Parbreak) {
            assert!(raw_text(&wrapped).is_err());
            assert!(title_text_with_context(
                &wrapped,
                &StyleChain::empty(),
                &mut Default::default()
            )
            .is_err());
        }
    }
    #[test]
    fn carrier_title_linebreak_absorbs_only_its_next_separator() {
        for linebreak in carriers(Content::linebreak()) {
            for space in carriers(Content::Space) {
                let body = Content::sequence(vec![
                    Content::text("A"),
                    linebreak.clone(),
                    space,
                    Content::Space,
                    Content::text("B"),
                ]);
                assert_eq!(
                    title_text_with_context(
                        &body,
                        &StyleChain::empty(),
                        &mut Default::default()
                    )
                    .unwrap(),
                    "A\n B"
                );
            }
        }
    }
}

#[cfg(test)]
mod batch_g_html_encoding_tests {
    use super::*;
    use std::sync::Arc;
    use typst_core::entities::{
        content::LocatedContent, location::Location, locator::Locator,
    };
    fn located(body: Content, location: Location) -> Content {
        Content::Located(Arc::new(LocatedContent { body, location }))
    }
    #[test]
    fn batch_g_html_textual_guards_do_not_accept_semantic_body_containers() {
        let mut locator = Locator::new();
        for content in [
            Content::label("x", Content::text("TEXT")),
            Content::strong(Content::text("TEXT")),
            Content::heading(1, Content::text("TEXT")),
        ] {
            for wrapped in [
                content.clone(),
                located(content.clone(), locator.next()),
                located(located(content, locator.next()), locator.next()),
            ] {
                assert_eq!(
                    raw_text(&wrapped).unwrap_err().message,
                    "HTML raw text element cannot have non-text children"
                );
                assert_eq!(
                    title_text_with_context(
                        &wrapped,
                        &typst_core::entities::style_chain::StyleChain::default_chain(),
                        &mut Default::default()
                    )
                    .unwrap_err()
                    .message,
                    "HTML raw text element cannot have non-text children"
                );
            }
        }
    }
    #[test]
    fn batch_g_html_title_wrapped_newline_space_origin_and_escaping_stay_exact() {
        let mut locator = Locator::new();
        for separator in [Content::Space, Content::SpaceFromNewline] {
            let body = Content::sequence(vec![
                Content::text("A & <"),
                located(Content::linebreak(), locator.next()),
                located(separator, locator.next()),
                Content::Space,
                Content::text("B"),
            ]);
            assert_eq!(
                title_text_with_context(
                    &body,
                    &typst_core::entities::style_chain::StyleChain::default_chain(),
                    &mut Default::default()
                )
                .unwrap(),
                "A &amp; &lt;\n B"
            );
            assert_eq!(raw_text(&body).unwrap(), "A & <\n  B");
        }
    }
}
