//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/realized.md
//! @prompt-hash fe4dea7c
//! @layer L1
//! @updated 2026-10-16

use super::{FontMetrics, ImageSizer, Layouter};
use crate::compiler::introspect::extract_payload::extract_payload;
use crate::entities::content::Content;
use crate::entities::location::Location;
use crate::entities::position::Position;

fn needs_fresh_index(content: &Content) -> bool {
    let content = content.source_body();
    match content {
        Content::Outline(_)
        | Content::OutlineEntry(_)
        | Content::Bibliography(_)
        | Content::Ref(_)
        | Content::Dynamic(_)
        | Content::Document { .. }
        | Content::Asset { .. } => true,
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
        | Content::CounterDisplay(_)
        | Content::CounterUpdate(_)
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
        | Content::ContextBlock(_) => false,
    }
}

fn prepared_locations(content: &Content, locations: &mut Vec<Location>) -> bool {
    let mut complete = true;
    let traversal = content.map_content(&mut |node| {
        match node {
            Content::Located(located) => {
                if needs_fresh_index(&located.body)
                    || extract_payload(&located.body).is_none()
                {
                    complete = false;
                } else {
                    locations.push(located.location);
                }
            }
            Content::Realized(realized) => {
                complete &= prepared_locations(&realized.original, locations);
            }
            Content::Source(_)
            | Content::ShowRecipe(_)
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
            | Content::Dynamic(_) => {
                if needs_fresh_index(node) || extract_payload(node).is_some() {
                    complete = false;
                }
            }
        }
        Ok(None)
    });
    complete && traversal.is_ok()
}

pub(super) fn register_origins<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    original: &Content,
) {
    let mut locations = Vec::new();
    if !prepared_locations(original, &mut locations) {
        locations = crate::compiler::introspect::introspect_with_introspector(original)
            .elements
            .keys()
            .copied()
            .collect();
    }
    let page = std::num::NonZeroUsize::new(layouter.pages.len() + 1).unwrap();
    let point = crate::entities::layout_types::Point {
        x: layouter.regions.current.cursor_x,
        y: layouter.regions.current.cursor_y,
    };
    for location in locations {
        layouter.locator.observe(location);
        layouter.record_location_position(location, Position { page, point }, true);
    }
}

fn block_equation_origin(content: &Content) -> bool {
    let content = content.source_body();
    match content {
        Content::Equation(e) => e.block,
        Content::Located(e) => block_equation_origin(&e.body),
        Content::Styled(e) => block_equation_origin(&e.body),
        Content::ShowRecipe(e) => block_equation_origin(&e.body),
        Content::Realized(e) => block_equation_origin(&e.original),
        Content::Source(_)
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
        | Content::Dynamic(_) => false,
    }
}

fn textual_product(content: &Content) -> bool {
    let content = content.source_body();
    match content {
        Content::Text(_)
        | Content::Empty
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Linebreak(_) => true,
        Content::Sequence(parts) => parts.iter().all(textual_product),
        Content::Styled(e) => textual_product(&e.body),
        Content::Strong(e) => textual_product(&e.body),
        Content::Emph(e) => textual_product(&e.body),
        Content::Located(e) => textual_product(&e.body),
        Content::ShowRecipe(e) => textual_product(&e.body),
        Content::Source(_)
        | Content::Realized(_)
        | Content::Parbreak
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

pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    original: &Content,
    visual: &Content,
) {
    let separate = block_equation_origin(original)
        && textual_product(visual)
        && !visual.plain_text().trim().is_empty();
    if separate {
        super::parbreak::layout(layouter);
    }
    if !layouter.register_planned_original() {
        register_origins(layouter, original);
    }
    if separate {
        super::paragraph::layout_with_edge(
            layouter,
            visual,
            Some(crate::compiler::introspect::page_running::PlanEdge::RealizedVisual),
        );
    } else {
        layouter.layout_planned_child(
            crate::compiler::introspect::page_running::PlanEdge::RealizedVisual,
            visual,
        );
    }
}

#[cfg(test)]
mod v16_origins_tests {
    use super::*;
    use crate::entities::content::{LocatedContent, RealizedContent};
    use crate::entities::file_id::FileId;
    use crate::entities::locator::Locator;
    use crate::entities::span::Span;
    use std::num::NonZeroU16;
    use std::sync::Arc;

    fn origin_span() -> Span {
        Span::from_range(FileId::from_raw(NonZeroU16::new(1).unwrap()), 2..5)
    }

    #[test]
    fn v16_origins_located_source_heading_is_a_prepared_occurrence() {
        let location = Locator::new().next();
        let original = Content::Located(Arc::new(LocatedContent {
            location,
            body: Content::heading(1, Content::text("h")).with_source_span(origin_span()),
        }));
        let mut locations = Vec::new();
        assert!(prepared_locations(&original, &mut locations));
        assert_eq!(locations, vec![location]);
    }

    #[test]
    fn v16_origins_source_around_located_does_not_duplicate_location() {
        let location = Locator::new().next();
        let original = Content::Located(Arc::new(LocatedContent {
            location,
            body: Content::heading(1, Content::Empty),
        }))
        .with_source_span(origin_span());
        let mut locations = Vec::new();
        assert!(prepared_locations(&original, &mut locations));
        assert_eq!(locations, vec![location]);
    }

    #[test]
    fn v16_origins_realized_source_original_is_collected_once() {
        let location = Locator::new().next();
        let original = Content::Located(Arc::new(LocatedContent {
            location,
            body: Content::heading(1, Content::text("original"))
                .with_source_span(origin_span()),
        }));
        let realized = Content::Realized(Arc::new(RealizedContent {
            original,
            visual: Content::text("visual"),
        }));
        let mut locations = Vec::new();
        assert!(prepared_locations(&realized, &mut locations));
        assert_eq!(locations, vec![location]);
    }

    #[test]
    fn v16_origins_unprepared_or_special_origins_keep_fallback() {
        for body in [
            Content::text("not a payload"),
            Content::reference("target").with_source_span(origin_span()),
            Content::outline().with_source_span(origin_span()),
        ] {
            let original = Content::Located(Arc::new(LocatedContent {
                location: Locator::new().next(),
                body,
            }));
            let mut locations = Vec::new();
            assert!(!prepared_locations(&original, &mut locations));
            assert!(locations.is_empty());
        }
        let mut locations = Vec::new();
        assert!(!prepared_locations(
            &Content::heading(1, Content::Empty),
            &mut locations
        ));
    }

    #[test]
    fn v16_origins_special_source_classes_still_require_fresh_index() {
        for content in [
            Content::outline(),
            Content::bibliography(vec![], None),
            Content::reference("target"),
        ] {
            assert!(needs_fresh_index(&content));
            assert!(needs_fresh_index(&content.with_source_span(origin_span())));
        }
        for content in
            [Content::Empty, Content::text("plain"), Content::heading(1, Content::Empty)]
        {
            assert!(!needs_fresh_index(&content));
        }
    }

    #[test]
    fn v16_origins_block_equation_certificate_preserves_wrapper_boundary() {
        let equation = Content::equation(Content::Empty, true);
        assert!(block_equation_origin(&equation));
        assert!(!block_equation_origin(&Content::equation(Content::Empty, false)));
        let located = Content::Located(Arc::new(LocatedContent {
            location: Locator::new().next(),
            body: equation.clone(),
        }))
        .with_source_span(origin_span());
        let realized = Content::Realized(Arc::new(RealizedContent {
            original: located,
            visual: Content::text("visual"),
        }));
        assert!(block_equation_origin(&realized));
        assert!(!block_equation_origin(&Content::strong(equation)));
        assert!(!block_equation_origin(&Content::text("plain")));
    }

    #[test]
    fn v16_origins_textual_certificate_keeps_realized_outside_its_scope() {
        for content in [
            Content::Empty,
            Content::Space,
            Content::SpaceFromNewline,
            Content::linebreak(),
            Content::strong(Content::text("bold")),
            Content::emph(Content::text("italic")),
            Content::Sequence(Arc::from(vec![Content::text("a"), Content::Space])),
        ] {
            assert!(textual_product(&content.with_source_span(origin_span())));
        }
        let realized = Content::Realized(Arc::new(RealizedContent {
            original: Content::text("original"),
            visual: Content::text("visual"),
        }));
        assert!(!textual_product(&realized));
        assert!(!textual_product(&Content::heading(1, Content::Empty)));
        assert!(!textual_product(&Content::equation(Content::Empty, true)));
    }
}

#[cfg(test)]
mod v16_walk_prepared_tests {
    use super::*;
    use crate::entities::{
        content::{LocatedContent, RealizedContent},
        locator::Locator,
    };
    use std::sync::Arc;

    fn referenced_entry() -> (Content, Location) {
        let location = Locator::new().next();
        let heading = Content::Located(Arc::new(LocatedContent {
            location,
            body: Content::heading(1, Content::text("reference")),
        }));
        (Content::outline_entry(1, heading, None), location)
    }

    #[test]
    fn v16_walk_entry_reference_does_not_certify_prepared_origin() {
        let (entry, _) = referenced_entry();
        let mut allocator = Locator::new();
        let _ = allocator.next();
        let located = Content::Located(Arc::new(LocatedContent {
            location: allocator.next(),
            body: entry.clone(),
        }));
        let realized = Content::Realized(Arc::new(RealizedContent {
            original: located.clone(),
            visual: Content::text("visual"),
        }));
        for content in [entry, located, realized] {
            assert!(!prepared_locations(&content, &mut Vec::new()));
        }
    }

    #[test]
    fn v16_walk_entry_fallback_indexes_entry_without_its_referenced_heading() {
        let (entry, reference) = referenced_entry();
        let intr = crate::compiler::introspect::introspect_with_introspector(&entry);
        assert_eq!(intr.elements.len(), 1);
        assert!(!intr.elements.contains_key(&reference));
        assert!(intr.elements.values().all(|item| matches!(
            item.content().source_body(),
            Content::OutlineEntry(_)
        )));
    }
}
