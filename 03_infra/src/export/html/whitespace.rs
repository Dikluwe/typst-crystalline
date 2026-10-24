//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/html/whitespace.md
//! @prompt-hash 2c6eba0f
//! @layer L3
//! @updated 2026-10-17

use super::inline::inline_with_context;
use super::tags::{should_group_into_paragraph, BLOCK_TAGS, REPLACED_TAGS};
use super::{HtmlFrameResolver, HtmlSerializationMode};
use typst_core::compiler::lang::quotes::SmartQuoteContext;
use typst_core::entities::content::Content;
use typst_core::entities::source_result::SourceDiagnostic;

const PROTECTED_SPACE: &str = "<span style=\"white-space: pre-wrap\">&#x20;</span>";

#[derive(Clone, Copy)]
enum SpaceSupport {
    Boundary,
    EmptyInline,
    Supportive,
}

// Borrow the rendered branch only. Classification never evaluates recipes.
fn visual_content(mut content: &Content) -> &Content {
    loop {
        content = match content {
            Content::Source(e) => &e.body,
            Content::ShowRecipe(e) => &e.body,
            Content::Located(e) => &e.body,
            Content::Realized(e) => &e.visual,
            Content::Empty
            | Content::Text(_)
            | Content::Space
            | Content::SpaceFromNewline
            | Content::Parbreak
            | Content::Sequence(_)
            | Content::HtmlElem(_)
            | Content::HtmlFrame(_)
            | Content::Par { .. }
            | Content::Heading(_)
            | Content::Title(_)
            | Content::Strong(_)
            | Content::Emph(_)
            | Content::Raw(_)
            | Content::ListItem(_)
            | Content::EnumItem(_)
            | Content::Link(_)
            | Content::Equation(_)
            | Content::MathSequence(_)
            | Content::Symbol(_)
            | Content::MathIdent(_)
            | Content::MathText(_)
            | Content::MathFrac(_)
            | Content::MathAttach(_)
            | Content::MathPrimes(_)
            | Content::MathRoot(_)
            | Content::MathStretch(_)
            | Content::MathDelimited(_)
            | Content::MathLr(_)
            | Content::MathMid(_)
            | Content::MathOverbrace(_)
            | Content::MathUnderbrace(_)
            | Content::MathUnderbracket(_)
            | Content::MathUnderparen(_)
            | Content::MathUndershell(_)
            | Content::MathOverbracket(_)
            | Content::MathOverparen(_)
            | Content::MathOvershell(_)
            | Content::MathAlignPoint(_)
            | Content::Linebreak(_)
            | Content::MathMatrix(_)
            | Content::MathCases(_)
            | Content::MathAccent(_)
            | Content::MathCancel(_)
            | Content::MathUnderline(_)
            | Content::MathOverline(_)
            | Content::MathVec(_)
            | Content::MathClassOverride(_)
            | Content::MathLimitsOverride(_)
            | Content::MathUnderover(_)
            | Content::MathOp(_)
            | Content::MathStyled(_)
            | Content::Label(_)
            | Content::Ref(_)
            | Content::CounterDisplay(_)
            | Content::CounterUpdate(_)
            | Content::Outline(_)
            | Content::OutlineEntry(_)
            | Content::Figure(_)
            | Content::FigureCaption(_)
            | Content::Image(_)
            | Content::Shape(_)
            | Content::Curve(_)
            | Content::Transform(_)
            | Content::Grid(_)
            | Content::GridHeader(_)
            | Content::GridFooter(_)
            | Content::GridCell(_)
            | Content::SetPage(_)
            | Content::PageRun(_)
            | Content::Align(_)
            | Content::Place(_)
            | Content::Flush(_)
            | Content::Styled(_)
            | Content::Divider(_)
            | Content::Terms(_)
            | Content::TermItem(_)
            | Content::Quote(_)
            | Content::Document { .. }
            | Content::Asset { .. }
            | Content::SmartQuote(_)
            | Content::PdfAttach(_)
            | Content::PdfArtifact(_)
            | Content::Underline(_)
            | Content::Strike(_)
            | Content::Overline(_)
            | Content::SmallCaps { .. }
            | Content::Pad(_)
            | Content::Hide(_)
            | Content::HSpace(_)
            | Content::VSpace(_)
            | Content::Pagebreak(_)
            | Content::Colbreak(_)
            | Content::Stack(_)
            | Content::Boxed(_)
            | Content::Block(_)
            | Content::TableCell(_)
            | Content::Bibliography(_)
            | Content::Cite(_)
            | Content::Footnote(_)
            | Content::FootnoteEntry(_)
            | Content::TableHeader(_)
            | Content::TableFooter(_)
            | Content::GridHLine(_)
            | Content::GridVLine(_)
            | Content::TableHLine(_)
            | Content::TableVLine(_)
            | Content::Table(_)
            | Content::Repeat(_)
            | Content::Columns(_)
            | Content::Metadata(_)
            | Content::State(_)
            | Content::StateUpdate(_)
            | Content::StateDisplay(_)
            | Content::CounterDisplayCallback(_)
            | Content::ContextBlock(_)
            | Content::Dynamic(_) => return content,
        };
    }
}

fn classified_content(content: &Content) -> &Content {
    let content = visual_content(content);
    match content {
        Content::Styled(styled) => classified_content(&styled.body),
        Content::Source(_)
        | Content::ShowRecipe(_)
        | Content::Realized(_)
        | Content::Located(_)
        | Content::Empty
        | Content::Text(_)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Sequence(_)
        | Content::HtmlElem(_)
        | Content::HtmlFrame(_)
        | Content::Par { .. }
        | Content::Heading(_)
        | Content::Title(_)
        | Content::Strong(_)
        | Content::Emph(_)
        | Content::Raw(_)
        | Content::ListItem(_)
        | Content::EnumItem(_)
        | Content::Link(_)
        | Content::Equation(_)
        | Content::MathSequence(_)
        | Content::Symbol(_)
        | Content::MathIdent(_)
        | Content::MathText(_)
        | Content::MathFrac(_)
        | Content::MathAttach(_)
        | Content::MathPrimes(_)
        | Content::MathRoot(_)
        | Content::MathStretch(_)
        | Content::MathDelimited(_)
        | Content::MathLr(_)
        | Content::MathMid(_)
        | Content::MathOverbrace(_)
        | Content::MathUnderbrace(_)
        | Content::MathUnderbracket(_)
        | Content::MathUnderparen(_)
        | Content::MathUndershell(_)
        | Content::MathOverbracket(_)
        | Content::MathOverparen(_)
        | Content::MathOvershell(_)
        | Content::MathAlignPoint(_)
        | Content::Linebreak(_)
        | Content::MathMatrix(_)
        | Content::MathCases(_)
        | Content::MathAccent(_)
        | Content::MathCancel(_)
        | Content::MathUnderline(_)
        | Content::MathOverline(_)
        | Content::MathVec(_)
        | Content::MathClassOverride(_)
        | Content::MathLimitsOverride(_)
        | Content::MathUnderover(_)
        | Content::MathOp(_)
        | Content::MathStyled(_)
        | Content::Label(_)
        | Content::Ref(_)
        | Content::CounterDisplay(_)
        | Content::CounterUpdate(_)
        | Content::Outline(_)
        | Content::OutlineEntry(_)
        | Content::Figure(_)
        | Content::FigureCaption(_)
        | Content::Image(_)
        | Content::Shape(_)
        | Content::Curve(_)
        | Content::Transform(_)
        | Content::Grid(_)
        | Content::GridHeader(_)
        | Content::GridFooter(_)
        | Content::GridCell(_)
        | Content::SetPage(_)
        | Content::PageRun(_)
        | Content::Align(_)
        | Content::Place(_)
        | Content::Flush(_)
        | Content::Divider(_)
        | Content::Terms(_)
        | Content::TermItem(_)
        | Content::Quote(_)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(_)
        | Content::PdfAttach(_)
        | Content::PdfArtifact(_)
        | Content::Underline(_)
        | Content::Strike(_)
        | Content::Overline(_)
        | Content::SmallCaps { .. }
        | Content::Pad(_)
        | Content::Hide(_)
        | Content::HSpace(_)
        | Content::VSpace(_)
        | Content::Pagebreak(_)
        | Content::Colbreak(_)
        | Content::Stack(_)
        | Content::Boxed(_)
        | Content::Block(_)
        | Content::TableCell(_)
        | Content::Bibliography(_)
        | Content::Cite(_)
        | Content::Footnote(_)
        | Content::FootnoteEntry(_)
        | Content::TableHeader(_)
        | Content::TableFooter(_)
        | Content::GridHLine(_)
        | Content::GridVLine(_)
        | Content::TableHLine(_)
        | Content::TableVLine(_)
        | Content::Table(_)
        | Content::Repeat(_)
        | Content::Columns(_)
        | Content::Metadata(_)
        | Content::State(_)
        | Content::StateUpdate(_)
        | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_)
        | Content::ContextBlock(_)
        | Content::Dynamic(_) => content,
    }
}

fn is_space(content: &Content) -> bool {
    matches!(classified_content(content), Content::Space | Content::SpaceFromNewline)
}

pub(super) fn inline_html_body_with_context(
    parent_tag: &str,
    content: &Content,
    mode: HtmlSerializationMode,
    resolver: Option<&HtmlFrameResolver<'_>>,
    chain: &typst_core::entities::style_chain::StyleChain,
    context: &mut SmartQuoteContext,
) -> Result<String, SourceDiagnostic> {
    let content = visual_content(content);
    if let Content::Styled(styled) = content {
        return inline_html_body_with_context(
            parent_tag,
            &styled.body,
            mode,
            resolver,
            &chain.push_styles(&styled.styles),
            context,
        );
    }
    let Content::Sequence(items) = content else {
        return inline_with_context(content, mode, resolver, chain, context);
    };
    let first = items.iter().position(|item| !is_space(item));
    let Some(first) = first else { return Ok(String::new()) };
    let last = items.iter().rposition(|item| !is_space(item)).unwrap();
    let mut out = String::new();
    for index in first..=last {
        if is_space(&items[index]) {
            if parent_tag == "ruby"
                && index > first
                && index < last
                && is_ruby_annotation(&items[index - 1])
                && is_ruby_annotation(&items[index + 1])
            {
                continue;
            }
            if index > first && collapses_adjacent_whitespace(&items[index - 1])
                || index < last && collapses_adjacent_whitespace(&items[index + 1])
            {
                continue;
            }
            let space = space_between(items, index);
            if !space.is_empty() {
                context.observe_text(" ");
            }
            out.push_str(space);
        } else {
            out.push_str(&inline_with_context(
                &items[index],
                mode,
                resolver,
                chain,
                context,
            )?);
        }
    }
    Ok(out)
}

fn is_ruby_annotation(content: &Content) -> bool {
    matches!(classified_content(content), Content::HtmlElem(elem) if matches!(elem.tag.as_str(), "rp" | "rt"))
}

fn space_between(items: &[Content], index: usize) -> &'static str {
    match (
        space_support(items[..index].iter().rev()),
        space_support(items[index + 1..].iter()),
    ) {
        (SpaceSupport::Boundary, _) | (_, SpaceSupport::Boundary) => "",
        (SpaceSupport::Supportive, SpaceSupport::Supportive) => " ",
        (SpaceSupport::EmptyInline, SpaceSupport::EmptyInline)
        | (SpaceSupport::EmptyInline, SpaceSupport::Supportive)
        | (SpaceSupport::Supportive, SpaceSupport::EmptyInline) => PROTECTED_SPACE,
    }
}

pub(super) fn top_level_space_between(items: &[Content], index: usize) -> &'static str {
    match (
        paragraph_space_support(items[..index].iter().rev()),
        paragraph_space_support(items[index + 1..].iter()),
    ) {
        (SpaceSupport::Boundary, _) | (_, SpaceSupport::Boundary) => "",
        (SpaceSupport::Supportive, SpaceSupport::Supportive) => " ",
        (SpaceSupport::EmptyInline, SpaceSupport::EmptyInline)
        | (SpaceSupport::EmptyInline, SpaceSupport::Supportive)
        | (SpaceSupport::Supportive, SpaceSupport::EmptyInline) => PROTECTED_SPACE,
    }
}

fn paragraph_space_support<'a>(items: impl Iterator<Item = &'a Content>) -> SpaceSupport {
    let mut found_inline = false;
    for item in items {
        let item = classified_content(item);
        match item {
            Content::Parbreak | Content::Heading(_) | Content::Par { .. } => {
                return if found_inline {
                    SpaceSupport::EmptyInline
                } else {
                    SpaceSupport::Boundary
                };
            }
            Content::HtmlElem(elem)
                if !should_group_into_paragraph(elem.tag.as_str()) =>
            {
                return if found_inline {
                    SpaceSupport::EmptyInline
                } else {
                    SpaceSupport::Boundary
                };
            }
            Content::Source(_)
            | Content::ShowRecipe(_)
            | Content::Realized(_)
            | Content::Located(_)
            | Content::Empty
            | Content::Text(_)
            | Content::Space
            | Content::SpaceFromNewline
            | Content::Sequence(_)
            | Content::HtmlElem(_)
            | Content::HtmlFrame(_)
            | Content::Title(_)
            | Content::Strong(_)
            | Content::Emph(_)
            | Content::Raw(_)
            | Content::ListItem(_)
            | Content::EnumItem(_)
            | Content::Link(_)
            | Content::Equation(_)
            | Content::MathSequence(_)
            | Content::Symbol(_)
            | Content::MathIdent(_)
            | Content::MathText(_)
            | Content::MathFrac(_)
            | Content::MathAttach(_)
            | Content::MathPrimes(_)
            | Content::MathRoot(_)
            | Content::MathStretch(_)
            | Content::MathDelimited(_)
            | Content::MathLr(_)
            | Content::MathMid(_)
            | Content::MathOverbrace(_)
            | Content::MathUnderbrace(_)
            | Content::MathUnderbracket(_)
            | Content::MathUnderparen(_)
            | Content::MathUndershell(_)
            | Content::MathOverbracket(_)
            | Content::MathOverparen(_)
            | Content::MathOvershell(_)
            | Content::MathAlignPoint(_)
            | Content::Linebreak(_)
            | Content::MathMatrix(_)
            | Content::MathCases(_)
            | Content::MathAccent(_)
            | Content::MathCancel(_)
            | Content::MathUnderline(_)
            | Content::MathOverline(_)
            | Content::MathVec(_)
            | Content::MathClassOverride(_)
            | Content::MathLimitsOverride(_)
            | Content::MathUnderover(_)
            | Content::MathOp(_)
            | Content::MathStyled(_)
            | Content::Label(_)
            | Content::Ref(_)
            | Content::CounterDisplay(_)
            | Content::CounterUpdate(_)
            | Content::Outline(_)
            | Content::OutlineEntry(_)
            | Content::Figure(_)
            | Content::FigureCaption(_)
            | Content::Image(_)
            | Content::Shape(_)
            | Content::Curve(_)
            | Content::Transform(_)
            | Content::Grid(_)
            | Content::GridHeader(_)
            | Content::GridFooter(_)
            | Content::GridCell(_)
            | Content::SetPage(_)
            | Content::PageRun(_)
            | Content::Align(_)
            | Content::Place(_)
            | Content::Flush(_)
            | Content::Styled(_)
            | Content::Divider(_)
            | Content::Terms(_)
            | Content::TermItem(_)
            | Content::Quote(_)
            | Content::Document { .. }
            | Content::Asset { .. }
            | Content::SmartQuote(_)
            | Content::PdfAttach(_)
            | Content::PdfArtifact(_)
            | Content::Underline(_)
            | Content::Strike(_)
            | Content::Overline(_)
            | Content::SmallCaps { .. }
            | Content::Pad(_)
            | Content::Hide(_)
            | Content::HSpace(_)
            | Content::VSpace(_)
            | Content::Pagebreak(_)
            | Content::Colbreak(_)
            | Content::Stack(_)
            | Content::Boxed(_)
            | Content::Block(_)
            | Content::TableCell(_)
            | Content::Bibliography(_)
            | Content::Cite(_)
            | Content::Footnote(_)
            | Content::FootnoteEntry(_)
            | Content::TableHeader(_)
            | Content::TableFooter(_)
            | Content::GridHLine(_)
            | Content::GridVLine(_)
            | Content::TableHLine(_)
            | Content::TableVLine(_)
            | Content::Table(_)
            | Content::Repeat(_)
            | Content::Columns(_)
            | Content::Metadata(_)
            | Content::State(_)
            | Content::StateUpdate(_)
            | Content::StateDisplay(_)
            | Content::CounterDisplayCallback(_)
            | Content::ContextBlock(_)
            | Content::Dynamic(_) => {}
        }
        if supports_adjacent_space(item) {
            return SpaceSupport::Supportive;
        }
        if !matches!(item, Content::Space | Content::SpaceFromNewline | Content::Empty) {
            found_inline = true;
        }
    }
    if found_inline {
        SpaceSupport::EmptyInline
    } else {
        SpaceSupport::Boundary
    }
}

fn space_support<'a>(items: impl Iterator<Item = &'a Content>) -> SpaceSupport {
    let mut found_inline = false;
    for item in items {
        let item = classified_content(item);
        if collapses_adjacent_whitespace(item) {
            return if found_inline {
                SpaceSupport::EmptyInline
            } else {
                SpaceSupport::Boundary
            };
        }
        if supports_adjacent_space(item) {
            return SpaceSupport::Supportive;
        }
        if !matches!(item, Content::Space | Content::SpaceFromNewline | Content::Empty) {
            found_inline = true;
        }
    }
    if found_inline {
        SpaceSupport::EmptyInline
    } else {
        SpaceSupport::Boundary
    }
}

fn supports_adjacent_space(content: &Content) -> bool {
    let content = classified_content(content);
    match content {
        Content::Text(text) => !text.is_empty(),
        Content::Sequence(items) => items.iter().any(supports_adjacent_space),
        Content::Strong(elem) => supports_adjacent_space(&elem.body),
        Content::Emph(elem) => supports_adjacent_space(&elem.body),
        Content::Styled(styled) => supports_adjacent_space(&styled.body),
        Content::HtmlElem(elem) => {
            if collapses_adjacent_whitespace(content) {
                false
            } else if REPLACED_TAGS.contains(&elem.tag.as_str()) {
                true
            } else {
                elem.body.content().is_some_and(supports_adjacent_space)
            }
        }
        Content::Empty
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak => false,
        Content::Source(_)
        | Content::ShowRecipe(_)
        | Content::Realized(_)
        | Content::Located(_)
        | Content::HtmlFrame(_)
        | Content::Par { .. }
        | Content::Heading(_)
        | Content::Title(_)
        | Content::Raw(_)
        | Content::ListItem(_)
        | Content::EnumItem(_)
        | Content::Link(_)
        | Content::Equation(_)
        | Content::MathSequence(_)
        | Content::Symbol(_)
        | Content::MathIdent(_)
        | Content::MathText(_)
        | Content::MathFrac(_)
        | Content::MathAttach(_)
        | Content::MathPrimes(_)
        | Content::MathRoot(_)
        | Content::MathStretch(_)
        | Content::MathDelimited(_)
        | Content::MathLr(_)
        | Content::MathMid(_)
        | Content::MathOverbrace(_)
        | Content::MathUnderbrace(_)
        | Content::MathUnderbracket(_)
        | Content::MathUnderparen(_)
        | Content::MathUndershell(_)
        | Content::MathOverbracket(_)
        | Content::MathOverparen(_)
        | Content::MathOvershell(_)
        | Content::MathAlignPoint(_)
        | Content::Linebreak(_)
        | Content::MathMatrix(_)
        | Content::MathCases(_)
        | Content::MathAccent(_)
        | Content::MathCancel(_)
        | Content::MathUnderline(_)
        | Content::MathOverline(_)
        | Content::MathVec(_)
        | Content::MathClassOverride(_)
        | Content::MathLimitsOverride(_)
        | Content::MathUnderover(_)
        | Content::MathOp(_)
        | Content::MathStyled(_)
        | Content::Label(_)
        | Content::Ref(_)
        | Content::CounterDisplay(_)
        | Content::CounterUpdate(_)
        | Content::Outline(_)
        | Content::OutlineEntry(_)
        | Content::Figure(_)
        | Content::FigureCaption(_)
        | Content::Image(_)
        | Content::Shape(_)
        | Content::Curve(_)
        | Content::Transform(_)
        | Content::Grid(_)
        | Content::GridHeader(_)
        | Content::GridFooter(_)
        | Content::GridCell(_)
        | Content::SetPage(_)
        | Content::PageRun(_)
        | Content::Align(_)
        | Content::Place(_)
        | Content::Flush(_)
        | Content::Divider(_)
        | Content::Terms(_)
        | Content::TermItem(_)
        | Content::Quote(_)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(_)
        | Content::PdfAttach(_)
        | Content::PdfArtifact(_)
        | Content::Underline(_)
        | Content::Strike(_)
        | Content::Overline(_)
        | Content::SmallCaps { .. }
        | Content::Pad(_)
        | Content::Hide(_)
        | Content::HSpace(_)
        | Content::VSpace(_)
        | Content::Pagebreak(_)
        | Content::Colbreak(_)
        | Content::Stack(_)
        | Content::Boxed(_)
        | Content::Block(_)
        | Content::TableCell(_)
        | Content::Bibliography(_)
        | Content::Cite(_)
        | Content::Footnote(_)
        | Content::FootnoteEntry(_)
        | Content::TableHeader(_)
        | Content::TableFooter(_)
        | Content::GridHLine(_)
        | Content::GridVLine(_)
        | Content::TableHLine(_)
        | Content::TableVLine(_)
        | Content::Table(_)
        | Content::Repeat(_)
        | Content::Columns(_)
        | Content::Metadata(_)
        | Content::State(_)
        | Content::StateUpdate(_)
        | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_)
        | Content::ContextBlock(_)
        | Content::Dynamic(_) => true,
    }
}

fn collapses_adjacent_whitespace(content: &Content) -> bool {
    matches!(classified_content(content), Content::HtmlElem(elem) if elem.tag == "br" || BLOCK_TAGS.contains(&elem.tag.as_str()))
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

    use typst_core::entities::html::{HtmlBody, HtmlElem};
    fn elem(tag: &str, text: &str) -> Content {
        Content::HtmlElem(Arc::new(HtmlElem::new(
            tag.into(),
            None,
            if tag == "br" {
                HtmlBody::Unset
            } else {
                HtmlBody::Content(Box::new(Content::text(text)))
            },
        )))
    }
    fn render(tag: &str, body: &Content) -> String {
        inline_html_body_with_context(
            tag,
            body,
            HtmlSerializationMode::Vanilla,
            None,
            &StyleChain::empty(),
            &mut SmartQuoteContext::default(),
        )
        .unwrap()
    }
    #[test]
    fn carrier_sequence_edges_and_br_boundaries() {
        let body = Content::sequence(vec![
            Content::Space,
            Content::text("A"),
            Content::Space,
            elem("br", ""),
            Content::Space,
            Content::text("B"),
            Content::Space,
        ]);
        for wrapped in carriers(body) {
            assert_eq!(render("div", &wrapped), "A<br>B");
        }
        for wrapped in carriers(elem("br", "")) {
            let body = Content::sequence(vec![
                Content::text("A"),
                Content::Space,
                wrapped,
                Content::Space,
                Content::text("B"),
            ]);
            assert_eq!(render("div", &body), "A<br>B");
        }
    }
    #[test]
    fn carrier_ruby_and_empty_inline_neighbors() {
        for wrapped in carriers(elem("rt", "T")) {
            assert_eq!(
                render(
                    "ruby",
                    &Content::sequence(vec![elem("rp", "("), Content::Space, wrapped])
                ),
                "<rp>(</rp><rt>T</rt>"
            );
        }
        for wrapped in carriers(elem("span", "")) {
            assert_eq!(
                render(
                    "div",
                    &Content::sequence(vec![Content::text("A"), Content::Space, wrapped])
                ),
                format!("A{PROTECTED_SPACE}<span></span>")
            );
        }
        for wrapped in carriers(Content::Space) {
            assert_eq!(
                render("div", &Content::sequence(vec![wrapped, Content::text("A")])),
                "A"
            );
        }
    }
}

#[cfg(test)]
mod v16_whitespace_tests {
    use super::*;
    use std::sync::Arc;
    use typst_core::entities::html::{HtmlBody, HtmlElem};
    fn element(tag: &str) -> Content {
        Content::HtmlElem(Arc::new(HtmlElem::new(
            tag.into(),
            None,
            HtmlBody::Content(Box::new(Content::Empty)),
        )))
    }
    fn verify(left: Content, row: usize) {
        let expected = [
            ["", "", ""],
            ["", PROTECTED_SPACE, PROTECTED_SPACE],
            ["", PROTECTED_SPACE, " "],
        ];
        for (column, right) in [element("div"), element("span"), Content::text("A")]
            .into_iter()
            .enumerate()
        {
            let input = [left.clone(), Content::Space, right];
            assert_eq!(space_between(&input, 1), expected[row][column]);
            assert_eq!(top_level_space_between(&input, 1), expected[row][column]);
        }
    }
    #[test]
    fn v16_whitespace_html_boundary_row() {
        verify(element("div"), 0);
    }
    #[test]
    fn v16_whitespace_html_empty_inline_row() {
        verify(element("span"), 1);
    }
    #[test]
    fn v16_whitespace_html_supportive_row() {
        verify(Content::text("A"), 2);
    }
}
