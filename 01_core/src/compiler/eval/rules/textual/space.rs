//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/rules/textual/space.md
//! @prompt-hash 7bdac878
//! @layer L1
//! @updated 2026-10-17
//!
//! Normalização de espaços no fluxo textual realizado.

use crate::entities::content::Content;

fn space_origin(content: &Content) -> Option<bool> {
    match content {
        Content::Realized(e) => space_origin(&e.visual),
        Content::Source(e) => space_origin(&e.body),
        Content::Located(e) => space_origin(&e.body),
        Content::Space => Some(false),
        Content::SpaceFromNewline => Some(true),
        Content::Styled(styled) => space_origin(&styled.body),
        Content::ShowRecipe(_)
        | Content::Empty
        | Content::Text(_)
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
        | Content::Dynamic(_) => None,
    }
}

fn space_with_origin(original: &Content, had_newline: bool) -> Content {
    match original {
        Content::Source(e) => {
            space_with_origin(&e.body, had_newline).with_source_span(e.span)
        }
        Content::Styled(styled) => Content::styled(
            space_with_origin(&styled.body, had_newline),
            styled.styles.clone(),
        ),
        Content::ShowRecipe(_)
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
        | Content::Dynamic(_)
            if had_newline =>
        {
            Content::SpaceFromNewline
        }
        Content::ShowRecipe(_)
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
        | Content::Dynamic(_) => Content::Space,
    }
}

fn previous_visible(parts: &[Content], before: usize) -> Option<usize> {
    (0..before).rev().find(|&index| !parts[index].is_empty())
}

fn next_visible(parts: &[Content], from: usize) -> Option<usize> {
    (from..parts.len()).find(|&index| !parts[index].is_empty())
}

/// Texto de um wrapper de estilo pode informar o descarte do espaço exterior,
/// embora o wrapper continue a interromper matching literal/regex.
fn supportive_text(content: &Content) -> String {
    match content {
        Content::Realized(e) => supportive_text(&e.visual),
        Content::Located(e) => supportive_text(&e.body),
        Content::Source(e) => supportive_text(&e.body),
        Content::ShowRecipe(e) => supportive_text(&e.body),
        Content::Text(text) => text.to_string(),
        Content::Styled(..)
        | Content::Strong(_)
        | Content::Emph(_)
        | Content::Underline(_)
        | Content::Strike(_)
        | Content::Overline(_)
        | Content::SmallCaps { .. } => content.plain_text(),
        Content::Empty
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Sequence(_)
        | Content::HtmlElem(_)
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
        | Content::Dynamic(_) => String::new(),
    }
}

// A canonical occurrence already has the representation produced by space_with_origin.
fn canonical_space(content: &Content, had_newline: bool) -> bool {
    match content {
        Content::Space => !had_newline,
        Content::SpaceFromNewline => had_newline,
        Content::Styled(e) => canonical_space(&e.body, had_newline),
        Content::Source(e) => {
            !e.span.is_detached()
                && e.body.source_span().is_detached()
                && canonical_space(&e.body, had_newline)
        }
        Content::ShowRecipe(_)
        | Content::Realized(_)
        | Content::Located(_)
        | Content::Empty
        | Content::Text(_)
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
        | Content::Dynamic(_) => false,
    }
}

// Classify a boundary through already supported visual carriers without rewriting it.
fn space_boundary_content(mut content: &Content) -> &Content {
    loop {
        content = match content {
            Content::Source(e) => &e.body,
            Content::Located(e) => &e.body,
            Content::Realized(e) => &e.visual,
            Content::Styled(e) => &e.body,
            Content::ShowRecipe(_)
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
            | Content::Dynamic(_) => return content,
        };
    }
}

struct SpaceRun {
    end: usize,
    had_newline: bool,
    discard: bool,
    repeated: bool,
}

// Both consumers inspect the current state, including earlier discarded groups.
fn next_space_run(parts: &[Content], mut start: usize) -> Option<(usize, SpaceRun)> {
    while start < parts.len() {
        let Some(mut had_newline) = space_origin(&parts[start]) else {
            start += 1;
            continue;
        };
        let mut end = start + 1;
        let mut repeated = false;
        while end < parts.len() {
            match space_origin(&parts[end]) {
                Some(origin) => {
                    had_newline |= origin;
                    repeated = true;
                }
                None if matches!(space_boundary_content(&parts[end]), Content::Empty) => {
                }
                None => break,
            }
            end += 1;
        }
        let before = previous_visible(parts, start);
        let after = next_visible(parts, end);
        let destructive = before.is_some_and(|index| {
            matches!(space_boundary_content(&parts[index]), Content::Linebreak(_))
        }) || after.is_some_and(|index| {
            matches!(space_boundary_content(&parts[index]), Content::Linebreak(_))
        });
        // An isolated newline must retain its origin until insertion.
        let discard = destructive
            || (had_newline
                && before.is_some()
                && after.is_some()
                && discard_space_between(
                    &supportive_text(&parts[before.unwrap()]),
                    &supportive_text(&parts[after.unwrap()]),
                ));
        return Some((start, SpaceRun { end, had_newline, discard, repeated }));
    }
    None
}

fn apply_space_run(parts: &mut [Content], start: usize, run: &SpaceRun) {
    parts[start] = if run.discard {
        Content::Empty
    } else {
        space_with_origin(&parts[start], run.had_newline)
    };
    for part in &mut parts[start + 1..run.end] {
        if space_origin(part).is_some() {
            *part = Content::Empty;
        }
    }
}

pub(crate) fn normalize_markup_spaces(parts: &mut [Content]) {
    let mut from = 0;
    while let Some((start, run)) = next_space_run(parts, from) {
        from = run.end;
        apply_space_run(parts, start, &run);
    }
}

pub(crate) fn normalize_markup_spaces_if_changed(
    parts: &[Content],
) -> Option<Vec<Content>> {
    let mut output: Option<Vec<Content>> = None;
    let mut from = 0;
    while let Some((start, run)) =
        next_space_run(output.as_deref().unwrap_or(parts), from)
    {
        from = run.end;
        if run.discard
            || run.repeated
            || !canonical_space(
                &output.as_deref().unwrap_or(parts)[start],
                run.had_newline,
            )
        {
            apply_space_run(output.get_or_insert_with(|| parts.to_vec()), start, &run);
        }
    }
    output
}

/// Predicado Unicode puro equivalente à superfície do pin vanilla:
/// Han/Bopomofo/Hiragana/Katakana/Yi e pontuação ideográfica não recebem
/// espaços inferidos; coreano, aspas ambíguas e emoji conservam-nos.
fn discard_space_between(left: &str, right: &str) -> bool {
    use icu_properties::props::{EastAsianWidth, Emoji, Script};
    use icu_properties::{CodePointMapDataBorrowed, CodePointSetDataBorrowed};

    fn spacingless(c: char) -> bool {
        const SCRIPTS: CodePointMapDataBorrowed<Script> = CodePointMapDataBorrowed::new();
        const WIDTHS: CodePointMapDataBorrowed<EastAsianWidth> =
            CodePointMapDataBorrowed::new();
        const EMOJI: CodePointSetDataBorrowed = CodePointSetDataBorrowed::new::<Emoji>();

        match SCRIPTS.get(c) {
            Script::Han
            | Script::Bopomofo
            | Script::Hiragana
            | Script::Katakana
            | Script::Yi => true,
            Script::Inherited if matches!(c, '\u{302A}'..='\u{302D}' | '\u{3099}'..='\u{309A}') => {
                true
            }
            Script::Common => match WIDTHS.get(c) {
                EastAsianWidth::Fullwidth => true,
                EastAsianWidth::Halfwidth if c != '₩' => true,
                EastAsianWidth::Wide if !EMOJI.contains(c) => true,
                EastAsianWidth::Ambiguous if matches!(c, '㉈'..='㉏') => true,
                _ => false,
            },
            _ => false,
        }
    }

    left.chars().next_back().is_some_and(spacingless)
        || right.chars().next().is_some_and(spacingless)
}

#[cfg(test)]
mod p1494_space_tests {
    use super::{discard_space_between, normalize_markup_spaces};
    use crate::entities::content::Content;
    use crate::entities::style::Styles;

    #[test]
    fn pin_unicode_spacing_controls() {
        for (left, right, discard) in [
            ("水", "果", true),
            ("换行", "newline", true),
            ("newline", "换行", true),
            ("쉼표，", "줄", true),
            ("줄", "바꿈", false),
            ("new", "line", false),
            ("“你好”", "“你好”", false),
            ("🎃", "🎃", false),
            ("A", "（B", true),
            ("A", "₩", false),
        ] {
            assert_eq!(discard_space_between(left, right), discard, "{left:?}/{right:?}");
        }
    }

    #[test]
    fn styled_space_preserva_origem_ate_observar_vizinhos() {
        let styled =
            Content::styled(Box::new(Content::SpaceFromNewline), Styles::default());
        let mut isolated = [styled.clone()];
        normalize_markup_spaces(&mut isolated);
        assert!(matches!(
            &isolated[0],
            Content::Styled(styled) if matches!(&styled.body, Content::SpaceFromNewline)
        ));

        let mut between_han = [Content::text("水"), styled, Content::text("果")];
        normalize_markup_spaces(&mut between_han);
        assert!(matches!(&between_han[1], Content::Empty));
    }
}

#[cfg(test)]
mod normalization_change_tests {
    use super::*;
    use crate::entities::content::{LocatedContent, RealizedContent, SourceContent};
    use crate::entities::location::Location;
    use crate::entities::span::Span;
    use crate::entities::style::Styles;
    use std::sync::Arc;

    #[test]
    fn canonical_spaces_and_empty_slots_need_no_output() {
        for space in [
            Content::Space,
            Content::SpaceFromNewline,
            Content::styled(Box::new(Content::Space), Styles::default()),
        ] {
            assert!(normalize_markup_spaces_if_changed(&[space.clone(), Content::Empty])
                .is_none());
            assert!(normalize_markup_spaces_if_changed(&[
                Content::text("left"),
                space,
                Content::text("right")
            ])
            .is_none());
        }
        assert!(normalize_markup_spaces_if_changed(&[
            Content::text("水"),
            Content::Space,
            Content::text("果")
        ])
        .is_none());
        assert!(normalize_markup_spaces_if_changed(&[
            Content::text("줄"),
            Content::SpaceFromNewline,
            Content::text("바꿈")
        ])
        .is_none());
    }

    #[test]
    fn changed_groups_keep_unicode_origin_and_style() {
        let input = [
            Content::text("left"),
            Content::styled(Box::new(Content::Space), Styles::default()),
            Content::Empty,
            Content::SpaceFromNewline,
            Content::text("right"),
        ];
        let output = normalize_markup_spaces_if_changed(&input).unwrap();
        assert!(
            matches!(&output[1], Content::Styled(e) if matches!(e.body, Content::SpaceFromNewline))
        );
        assert!(matches!(&output[2], Content::Empty));
        assert!(matches!(&output[3], Content::Empty));
        let han = normalize_markup_spaces_if_changed(&[
            Content::text("水"),
            Content::SpaceFromNewline,
            Content::text("果"),
        ])
        .unwrap();
        assert!(matches!(&han[1], Content::Empty));
    }

    #[test]
    fn space_carriers_keep_previous_normalization() {
        let file = crate::entities::file_id::FileId::from_raw(
            std::num::NonZeroU16::new(1).unwrap(),
        );
        let span = Span::from_range(file, 1..2);
        let sourced = Content::Space.with_source_span(span);
        assert!(
            normalize_markup_spaces_if_changed(std::slice::from_ref(&sourced)).is_none()
        );
        for carrier in [
            Content::Source(Arc::new(SourceContent {
                body: Content::Space,
                span: Span::detached(),
            })),
            Content::Located(Arc::new(LocatedContent {
                location: Location::from_raw(9),
                body: Content::Space,
            })),
            Content::Realized(Arc::new(RealizedContent {
                original: Content::text("origin"),
                visual: Content::Space,
            })),
        ] {
            assert!(matches!(
                &normalize_markup_spaces_if_changed(&[carrier]).unwrap()[0],
                Content::Space
            ));
        }
        let nested = Content::Source(Arc::new(SourceContent {
            body: sourced,
            span: Span::from_range(file, 3..4),
        }));
        let output = normalize_markup_spaces_if_changed(&[nested]).unwrap();
        assert_eq!(output[0].source_span(), span);
        assert!(
            matches!(&output[0], Content::Source(e) if matches!(e.body, Content::Space))
        );
    }

    #[test]
    fn destructive_groups_observe_previously_normalized_neighbors() {
        let input = [
            Content::text("left"),
            Content::Space,
            Content::linebreak(),
            Content::SpaceFromNewline,
            Content::text("right"),
        ];
        let output = normalize_markup_spaces_if_changed(&input).unwrap();
        assert!(matches!(&output[1], Content::Empty));
        assert!(matches!(&output[2], Content::Linebreak(_)));
        assert!(matches!(&output[3], Content::Empty));
        let mut inplace = input.clone();
        normalize_markup_spaces(&mut inplace);
        assert_eq!(inplace.as_slice(), output.as_slice());
        assert!(normalize_markup_spaces_if_changed(&output).is_none());
    }
}

#[cfg(test)]
mod v16_whitespace_tests {
    use super::*;
    use crate::entities::content::{LocatedContent, RealizedContent, SourceContent};
    use crate::entities::{
        file_id::FileId, location::Location, span::Span, style::Styles,
    };
    use std::sync::Arc;

    fn span(at: usize) -> Span {
        Span::from_range(
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
            at..at + 1,
        )
    }
    fn carriers(body: Content) -> Vec<Content> {
        vec![
            body.clone().with_source_span(span(1)),
            Content::Located(Arc::new(LocatedContent {
                body: body.clone(),
                location: Location::from_raw(9),
            })),
            Content::Realized(Arc::new(RealizedContent {
                original: Content::text("must-not-observe"),
                visual: body.clone(),
            })),
            Content::styled(Box::new(body), Styles::default()),
        ]
    }
    fn view(content: &Content) -> String {
        match content.source_body() {
            Content::Located(e) => view(&e.body),
            Content::Realized(e) => view(&e.visual),
            Content::Styled(e) => view(&e.body),
            Content::Empty => "_".into(),
            Content::Space => "S".into(),
            Content::SpaceFromNewline => "N".into(),
            Content::Linebreak(_) => "L".into(),
            Content::Parbreak => "P".into(),
            Content::Text(t) => format!("T:{t}"),
            Content::MathIdent(t) => format!("M:{t}"),
            other => panic!("unsupported observation {other:?}"),
        }
    }
    fn assert_run(input: Vec<Content>, expected: &[&str]) {
        let mut inplace = input.clone();
        normalize_markup_spaces(&mut inplace);
        assert_eq!(inplace.iter().map(view).collect::<Vec<_>>(), expected);
        let lazy = normalize_markup_spaces_if_changed(&input).unwrap_or(input);
        assert_eq!(lazy.iter().map(view).collect::<Vec<_>>(), expected);
        assert!(normalize_markup_spaces_if_changed(&inplace).is_none());
        assert!(normalize_markup_spaces_if_changed(&lazy).is_none());
    }
    fn right_break(middle: Content) {
        assert_run(
            vec![Content::text("a"), Content::Space, middle, Content::text("b")],
            &["T:a", "_", "L", "T:b"],
        );
    }
    #[test]
    fn v16_whitespace_break_right() {
        for middle in carriers(Content::linebreak()) {
            right_break(middle);
        }
    }
    #[test]
    fn v16_whitespace_break_left() {
        for middle in carriers(Content::linebreak()) {
            assert_run(
                vec![
                    Content::text("a"),
                    middle,
                    Content::SpaceFromNewline,
                    Content::text("b"),
                ],
                &["T:a", "L", "_", "T:b"],
            );
        }
    }
    #[test]
    fn v16_whitespace_empty_run() {
        for middle in carriers(Content::Empty) {
            assert_run(
                vec![
                    Content::text("a"),
                    Content::Space,
                    middle,
                    Content::SpaceFromNewline,
                    Content::text("b"),
                ],
                &["T:a", "N", "_", "_", "T:b"],
            );
        }
    }
    #[test]
    fn v16_whitespace_empty_run_reverse() {
        for middle in carriers(Content::Empty) {
            assert_run(
                vec![
                    Content::text("a"),
                    Content::SpaceFromNewline,
                    middle,
                    Content::Space,
                    Content::text("b"),
                ],
                &["T:a", "N", "_", "_", "T:b"],
            );
        }
    }
    #[test]
    fn v16_whitespace_explicit_run() {
        for middle in carriers(Content::Empty) {
            assert_run(
                vec![
                    Content::text("a"),
                    Content::Space,
                    middle,
                    Content::Space,
                    Content::text("b"),
                ],
                &["T:a", "S", "_", "_", "T:b"],
            );
        }
    }
    #[test]
    fn v16_whitespace_nested_linebreak() {
        for inner in carriers(Content::linebreak().with_source_span(span(1))) {
            right_break(Content::Source(Arc::new(SourceContent {
                body: inner,
                span: span(3),
            })));
        }
    }
    #[test]
    fn v16_whitespace_paragraph_boundary() {
        for middle in carriers(Content::Parbreak) {
            assert_run(
                vec![
                    Content::text("a"),
                    Content::Space,
                    middle,
                    Content::SpaceFromNewline,
                    Content::text("b"),
                ],
                &["T:a", "S", "P", "N", "T:b"],
            );
        }
    }
    #[test]
    fn v16_whitespace_material_text() {
        for middle in carriers(Content::text("x")) {
            assert_run(
                vec![
                    Content::text("a"),
                    Content::Space,
                    middle,
                    Content::SpaceFromNewline,
                    Content::text("b"),
                ],
                &["T:a", "S", "T:x", "N", "T:b"],
            );
        }
    }
    #[test]
    fn v16_whitespace_nontext_math_boundary() {
        for middle in carriers(Content::MathIdent("x".into())) {
            assert_run(
                vec![
                    Content::text("a"),
                    Content::Space,
                    middle,
                    Content::SpaceFromNewline,
                    Content::text("b"),
                ],
                &["T:a", "S", "M:x", "N", "T:b"],
            );
        }
    }
    #[test]
    fn v16_whitespace_canonical_origin_and_style() {
        let original = Content::styled(Box::new(Content::Space), Styles::default())
            .with_source_span(span(1));
        let input = vec![Content::text("a"), original, Content::text("b")];
        assert!(normalize_markup_spaces_if_changed(&input).is_none());
        let mut output = input.clone();
        normalize_markup_spaces(&mut output);
        assert_eq!(output[1].source_span(), span(1));
        assert!(
            matches!(output[1].source_body(), Content::Styled(e) if matches!(e.body, Content::Space))
        );
        assert_run(input, &["T:a", "S", "T:b"]);
    }
    #[test]
    fn v16_whitespace_realized_uses_visual() {
        let text = Content::Realized(Arc::new(RealizedContent {
            original: Content::linebreak(),
            visual: Content::text("x"),
        }));
        assert_run(
            vec![
                Content::text("a"),
                Content::Space,
                text,
                Content::SpaceFromNewline,
                Content::text("b"),
            ],
            &["T:a", "S", "T:x", "N", "T:b"],
        );
        let line = Content::Realized(Arc::new(RealizedContent {
            original: Content::text("x"),
            visual: Content::linebreak(),
        }));
        assert_run(
            vec![
                Content::text("a"),
                Content::Space,
                line,
                Content::SpaceFromNewline,
                Content::text("b"),
            ],
            &["T:a", "_", "L", "_", "T:b"],
        );
    }
}
