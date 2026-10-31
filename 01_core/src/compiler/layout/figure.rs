//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/figure.md
//! @prompt-hash 47b06284
//! @layer L1
//! @updated 2026-06-26

use crate::compiler::lang::figure_supplement::figure_supplement_for_lang;
use crate::entities::content::Content;
use crate::entities::counter_format::format_counter;
use crate::entities::elements::figure::FigureElem;
use crate::entities::introspector::Introspector;
use crate::entities::value::Value;

use super::{FontMetrics, ImageSizer, Layouter};

#[derive(Clone)]
enum CaptionPrefix {
    Generated(Content),
    Prepared { supplement: Option<Content>, suffix: Content },
}

/// Layout de `figure(...)` (atomização ADR-0109 P378): calcula o prefixo de
/// numeração (gate na chain, número via Introspector) e delega o desenho a
/// `layout_figure`. Content-preserving — era inline no `layout_content`.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &FigureElem,
) {
    let (body, caption) = (&e.body, &e.caption);
    let location = layouter.current_location;
    let enabled = !matches!(layouter.chain.custom("figure.numbering"), Some(Value::None));
    let pattern = match layouter.chain.custom("figure.numbering") {
        Some(Value::Str(pattern)) => Some(pattern.to_string()),
        None => Some("1".to_string()),
        Some(Value::None | Value::Func(_)) => None,
        Some(_) => None,
    };
    let caption_prefix = if let Some(kind) = &e.resolved_kind {
        if enabled && caption.is_some() {
            let key =
                crate::compiler::introspect::figure_counters::figure_counter_key(kind);
            let values = location.and_then(|location| {
                layouter
                    .introspector
                    .counter_values_at(&key, location)
                    .map(ToOwned::to_owned)
            });
            let Some(values) = values else {
                layouter.layout_errors.push(
                    crate::entities::source_result::SourceDiagnostic::error(
                        layouter.current_source_span,
                        "figure counter snapshot could not be verified",
                    ),
                );
                return;
            };
            // Function-numbering visual formatting remains the measured separate debt.
            pattern.map(|pattern| {
                let number = format_counter(&values, &pattern).unwrap_or_default();
                let supplement = e
                    .resolved_supplement
                    .as_ref()
                    .and_then(|value| value.as_ref())
                    .cloned();
                layouter
                    .runtime
                    .figure_page_numbers
                    .push(layouter.current_page_number());
                let suffix = Content::text(format!("{number}: "));
                CaptionPrefix::Prepared { supplement, suffix }
            })
        } else {
            None
        }
    } else if let Some(pattern) = pattern {
        // Explicit compatibility boundary for external synthetic, unprepared Figures.
        let kind_key = match e.kind.as_ref().and_then(|kind| kind.as_ref()) {
            Some(crate::entities::elements::figure::FigureKind::Name(name)) => {
                name.as_str()
            }
            Some(crate::entities::elements::figure::FigureKind::Element(_)) | None => {
                "image"
            }
        };
        let progress = layouter.figure_progress.entry(kind_key.to_string()).or_insert(0);
        let index = *progress;
        *progress += 1;
        let number = layouter
            .introspector
            .figure_number_at_index(kind_key, index)
            .unwrap_or(index + 1);
        layouter
            .runtime
            .figure_page_numbers
            .push(layouter.current_page_number());
        let formatted =
            format_counter(&[number], &pattern).unwrap_or_else(|| number.to_string());
        let supplement =
            figure_supplement_for_lang(kind_key, layouter.chain.lang().as_ref());
        Some(CaptionPrefix::Generated(Content::text(format!(
            "{supplement} {formatted}: "
        ))))
    } else {
        None
    };
    match caption_prefix {
        Some(prefix @ CaptionPrefix::Prepared { .. }) => {
            layout_figure_prefix(layouter, body, caption, Some(prefix))
        }
        Some(CaptionPrefix::Generated(prefix)) => {
            layout_figure(layouter, body, caption, Some(prefix))
        }
        None => layout_figure(layouter, body, caption, None),
    }
}

/// Renderiza uma figura com legenda opcional (Passo 62/75, DEBT-14/15).
///
/// `caption_prefix`: prefixo de numeração pré-calculado pelo layouter
/// (ex: "Figura 1: "), ou `None` se a figura não tiver numeração activa.
/// O cálculo acontece em `layout_content` usando `figure_progress` e
/// `counter_state.figure_numbers` — a introspecção pré-computou os números.
pub(super) fn layout_figure<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    body: &Content,
    caption: &Option<Content>,
    caption_prefix: Option<Content>,
) {
    layout_figure_prefix(
        layouter,
        body,
        caption,
        caption_prefix.map(CaptionPrefix::Generated),
    );
}

fn layout_figure_prefix<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    body: &Content,
    caption: &Option<Content>,
    caption_prefix: Option<CaptionPrefix>,
) {
    use crate::compiler::introspect::page_running::PlanEdge;
    use crate::entities::layout_types::{Align2D, FrameItem, Point, Pt, TransformMatrix};
    if caption.is_none() {
        super::parbreak::layout(layouter);
        layouter.layout_planned_child(PlanEdge::FigureBody, body);
        super::parbreak::layout(layouter);
        return;
    }
    let (origin, available) = super::block::admit_prepared_frame(layouter);
    if !layouter.regions.current.current_line.is_empty()
        && layouter
            .regions
            .current
            .current_line
            .iter()
            .all(super::html_frame::is_context_marker)
        && layouter.regions.current.pending_fr.is_empty()
        && layouter.pending_repeats.is_empty()
    {
        layouter.flush_line();
    }
    let region = || super::sub_frame::SubLayoutRegion {
        origin_x: 0.0,
        width: available,
        height: None,
        align_rtl: false,
        unconstrained_height: true,
    };
    let saved_line_alignment = layouter
        .explicit_line_alignment
        .replace(crate::entities::layout_types::HAlign::Center);
    let body_cursor = layouter.planned_child_cursor(PlanEdge::FigureBody);
    let mut body_piece = layouter.layout_intrinsic_piece(
        std::slice::from_ref(body),
        &[body_cursor],
        region(),
    );
    let mut caption_parts = Vec::new();
    let mut caption_routes = Vec::new();
    if let Some(caption) = caption {
        if let Some(prefix) = caption_prefix {
            match prefix {
                CaptionPrefix::Generated(prefix) => {
                    caption_parts.push(prefix);
                    caption_routes.push(None);
                }
                CaptionPrefix::Prepared { supplement, suffix } => {
                    if let Some(supplement) = supplement {
                        if !supplement.is_empty() {
                            caption_parts.push(supplement);
                            caption_routes.push(layouter.planned_child_cursor(
                                PlanEdge::FigureSelectedSupplement,
                            ));
                            caption_parts.push(Content::text(" "));
                            caption_routes.push(None);
                        }
                    }
                    caption_parts.push(suffix);
                    caption_routes.push(None);
                }
            }
        }
        caption_parts.push(caption.clone());
        caption_routes.push(layouter.planned_child_cursor(PlanEdge::FigureCaption));
    }
    let mut caption_piece =
        layouter.layout_intrinsic_piece(&caption_parts, &caption_routes, region());
    layouter.explicit_line_alignment = saved_line_alignment;
    let width = body_piece.width.max(caption_piece.width);
    // ref: lab/typst-original/crates/typst-library/src/model/figure.rs:303
    let gap = if body_piece.height > 0.0 && caption_piece.height > 0.0 {
        0.65 * layouter.style.size.0
    } else {
        0.0
    };
    let caption_y = body_piece.height + gap;
    let height = caption_y + caption_piece.height;
    let baseline = if body_piece.flow_present {
        body_piece.baseline
    } else {
        caption_piece.baseline.map(|baseline| baseline.map(|y| y + caption_y))
    };
    let body_width = body_piece.width;
    let caption_width = caption_piece.width;
    layouter.settle_intrinsic_piece(&mut body_piece, body_width);
    layouter.settle_intrinsic_piece(&mut caption_piece, caption_width);
    let body_alignment = figure_piece_alignment(body);
    let body_dx = layouter
        .resolve_alignment(
            Align2D { h: Some(body_alignment), v: None },
            body_piece.width,
            0.0,
            width,
            0.0,
            0.0,
            0.0,
        )
        .0;
    let caption_dx = (width - caption_piece.width) / 2.0;
    let defer_center = layouter.block_auto_width;
    let visual_dx = if defer_center { 0.0 } else { (available - width) / 2.0 };
    let mut decorations = Vec::new();
    let mut groups = Vec::new();
    for (piece, x, y) in
        [(body_piece, body_dx, 0.0), (caption_piece, caption_dx, caption_y)]
    {
        for segment in piece.decorations {
            decorations.push(super::DecoSegment {
                start_x: segment.start_x + Pt(x),
                end_x: segment.end_x + Pt(x),
                baseline_y: segment.baseline_y + Pt(y),
                piece_line: segment.piece_line,
            });
        }
        groups.push(FrameItem::Group {
            paint_root: false,
            pos: Point { x: Pt(x), y: Pt(y) },
            matrix: TransformMatrix::identity(),
            clip_mask: None,
            inner_width: piece.width,
            inner_height: piece.height,
            items: piece.items,
        });
    }
    let final_origin = super::block::commit_prepared_frame(
        layouter,
        origin,
        width,
        height,
        baseline,
        vec![FrameItem::Group {
            paint_root: false,
            pos: Point { x: Pt(visual_dx), y: Pt(0.0) },
            matrix: TransformMatrix::identity(),
            clip_mask: None,
            inner_width: width,
            inner_height: height,
            items: groups,
        }],
    );
    if defer_center {
        layouter.pending_align_centering.push((
            vec![layouter.regions.current.current_items.len() - 1],
            1,
            Align2D {
                h: Some(crate::entities::layout_types::HAlign::Center),
                v: None,
            },
            width,
            final_origin.x.0,
            final_origin.x.0,
        ));
    }
    if let Some(collector) = layouter.decoration_lines_collector.as_mut() {
        collector.extend(decorations.into_iter().map(|segment| super::DecoSegment {
            start_x: segment.start_x + final_origin.x + Pt(visual_dx),
            end_x: segment.end_x + final_origin.x + Pt(visual_dx),
            baseline_y: segment.baseline_y + final_origin.y,
            piece_line: segment.piece_line,
        }));
    }
}

fn figure_piece_alignment(
    mut content: &Content,
) -> crate::entities::layout_types::HAlign {
    use crate::entities::layout_types::HAlign;
    loop {
        match content {
            Content::Source(e) => content = &e.body,
            Content::Located(e) => content = &e.body,
            Content::Styled(e) => content = &e.body,
            Content::Realized(e) => content = &e.visual,
            Content::Align(e) => return e.alignment.h.unwrap_or(HAlign::Center),
            Content::ShowRecipe(..)
            | Content::Empty
            | Content::Text(..)
            | Content::Symbol(..)
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
            | Content::Dynamic(..) => return HAlign::Center,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{
        content::{LocatedContent, SourceContent},
        elements::figure::FigureKind,
        func::Func,
        introspector::{FigureCounterSnapshot, TagIntrospector},
        location::Location,
        span::Span,
        style::Styles,
    };
    use comemo::Track;
    use std::sync::Arc;

    fn prepared() -> FigureElem {
        FigureElem {
            body: Content::text("body"),
            caption: Some(Content::text("caption")),
            kind: None,
            resolved_kind: Some(FigureKind::Element(Func::native(
                "image",
                crate::compiler::stdlib::native_image,
            ))),
            supplement: None,
            resolved_supplement: Some(Some(Content::text("Figure"))),
        }
    }

    #[test]
    fn figure_layout_missing_snapshot_keeps_source_span_and_emits_no_fabricated_prefix() {
        let intr = TagIntrospector::empty();
        let span = Span::from_range(
            crate::entities::file_id::FileId::from_raw(
                std::num::NonZeroU16::new(4).unwrap(),
            ),
            3..10,
        );
        let mut layouter = Layouter::new(
            super::super::FixedMetrics,
            super::super::NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        let content = Content::Source(Arc::new(SourceContent {
            span,
            body: Content::Located(Arc::new(LocatedContent {
                location: Location::from_raw(71),
                body: Content::Figure(Arc::new(prepared())),
            })),
        }));
        layouter.layout_content(&content);
        assert_eq!(layouter.layout_errors.len(), 1);
        assert_eq!(
            layouter.layout_errors[0].message,
            "figure counter snapshot could not be verified"
        );
        assert_eq!(layouter.layout_errors[0].span, span);
        assert!(layouter.figure_progress.is_empty());
        let document = layouter.finish();
        assert!(!document.plain_text().contains("body"));
        assert!(!document.plain_text().contains("Figure"));
    }

    #[test]
    fn figure_layout_prepared_location_never_consumes_synthetic_progress() {
        let mut intr = TagIntrospector::empty();
        let location = Location::from_raw(71);
        let mut figure = prepared();
        figure.resolved_supplement = Some(Some(Content::strong(Content::text("Figure"))));
        let key = crate::compiler::introspect::figure_counters::figure_counter_key(
            figure.resolved_kind.as_ref().unwrap(),
        );
        intr.counters.observe_location(location);
        intr.figure_counter_snapshots
            .push(FigureCounterSnapshot { key, history: vec![(location, vec![3])] });
        let mut layouter = Layouter::new(
            super::super::FixedMetrics,
            super::super::NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        layouter.figure_progress.insert("image".into(), 7);
        let content = Content::Located(Arc::new(LocatedContent {
            location,
            body: Content::Figure(Arc::new(figure)),
        }));
        let checkpoint = layouter.capture_flow_suffix_checkpoint();
        layouter.layout_content(&content);
        assert_eq!(layouter.figure_progress.get("image"), Some(&7));
        assert!(layouter.layout_errors.is_empty());
        layouter.restore_flow_suffix_checkpoint(checkpoint);
        assert_eq!(layouter.figure_progress.get("image"), Some(&7));
        layouter.layout_content(&content);
        let document = layouter.finish();
        assert!(document
            .plain_text()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .contains("Figure 3: caption"));
        struct BoldSupplement(bool);
        impl crate::entities::layout_types::FrameVisitor for BoldSupplement {
            fn visit_text(&mut self, item: &crate::entities::layout_types::FrameItem) {
                if let crate::entities::layout_types::FrameItem::Text {
                    text,
                    style,
                    ..
                } = item
                {
                    if text.as_str().contains("Figure") && style.bold {
                        self.0 = true;
                    }
                }
            }
        }
        use crate::entities::layout_types::FrameVisitor;
        let mut visitor = BoldSupplement(false);
        for page in &document.pages {
            visitor.visit_items(&page.items);
        }
        assert!(
            visitor.0,
            "resolved supplement retains Strong morphology in emitted text"
        );
    }

    #[test]
    fn figure_layout_returned_located_styled_occurrence_keeps_exact_location() {
        let mut intr = TagIntrospector::empty();
        let location = Location::from_raw(71);
        let figure = prepared();
        let key = crate::compiler::introspect::figure_counters::figure_counter_key(
            figure.resolved_kind.as_ref().unwrap(),
        );
        intr.counters.observe_location(location);
        intr.figure_counter_snapshots
            .push(FigureCounterSnapshot { key, history: vec![(location, vec![3])] });
        let mut layouter = Layouter::new(
            super::super::FixedMetrics,
            super::super::NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        let content = Content::Located(Arc::new(LocatedContent {
            location,
            body: Content::styled(Content::Figure(Arc::new(figure)), Styles::new()),
        }));
        layouter.layout_content(&content);
        assert_eq!(
            layouter.current_location,
            Some(location),
            "transparent style cannot allocate another Figure identity"
        );
        assert!(layouter.layout_errors.is_empty());
    }
}

#[cfg(test)]
mod paragraph_boundary_tests {
    use super::*;
    use crate::entities::{
        content::LocatedContent,
        elements::figure::FigureKind,
        func::Func,
        introspector::TagIntrospector,
        layout_types::{FrameItem, FrameVisitor},
        location::Location,
    };
    use comemo::Track;
    use std::sync::Arc;

    #[test]
    fn figure_blocks_keep_adjacent_bodies_and_following_text_in_separate_paragraphs() {
        let intr = TagIntrospector::empty();
        let mut layouter = Layouter::new(
            super::super::FixedMetrics,
            super::super::NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        let figure = |text: &str, number: u128| {
            Content::Located(Arc::new(LocatedContent {
                location: Location::from_raw(number),
                body: Content::Figure(Arc::new(FigureElem {
                    body: Content::text(text),
                    caption: None,
                    kind: None,
                    resolved_kind: Some(FigureKind::Element(Func::native(
                        "image",
                        crate::compiler::stdlib::native_image,
                    ))),
                    supplement: None,
                    resolved_supplement: Some(Some(Content::text("Figure"))),
                })),
            }))
        };
        layouter.layout_content(&Content::Sequence(
            vec![figure("ALPHA", 71), figure("BETA", 72), Content::text("TAIL")].into(),
        ));
        assert!(layouter.layout_errors.is_empty());
        let document = layouter.finish();
        struct Lines(Vec<(String, f64)>);
        impl FrameVisitor for Lines {
            fn visit_text(&mut self, item: &FrameItem) {
                if let FrameItem::Text { text, pos, .. } = item {
                    self.0.push((text.to_string(), pos.y.val()));
                }
            }
        }
        let mut lines = Lines(Vec::new());
        for page in &document.pages {
            lines.visit_items(&page.items);
        }
        let y =
            |word: &str| lines.0.iter().find(|(text, _)| text.contains(word)).unwrap().1;
        assert!(
            y("ALPHA") < y("BETA") && y("BETA") < y("TAIL"),
            "each Figure is a block; actual emitted lines: {:?}",
            lines.0
        );
        assert_eq!(
            document.plain_text().split_whitespace().collect::<Vec<_>>(),
            vec!["ALPHA", "BETA", "TAIL"]
        );
    }
}

#[cfg(test)]
mod readiness_frame_geometry {
    use super::*;
    use crate::entities::{
        geometry::ShapeKind,
        introspector::TagIntrospector,
        layout_types::{FrameItem, Length, TransformMatrix},
        style::{Style, Styles},
    };
    use comemo::Track;
    use std::sync::Arc;

    #[derive(Clone, Copy)]
    struct DeclaredFigureMetrics;
    impl FontMetrics for DeclaredFigureMetrics {
        fn advance(
            &self,
            text: &str,
            size: crate::entities::layout_types::Pt,
            style: &crate::entities::layout_types::TextStyle,
        ) -> crate::entities::layout_types::Pt {
            super::super::FixedMetrics.advance(text, size, style)
        }
        fn vertical_metrics(
            &self,
            size: crate::entities::layout_types::Pt,
            style: &crate::entities::layout_types::TextStyle,
        ) -> (crate::entities::layout_types::Pt, crate::entities::layout_types::Pt)
        {
            super::super::FixedMetrics.vertical_metrics(size, style)
        }
        fn cap_height(
            &self,
            size: crate::entities::layout_types::Pt,
            style: &crate::entities::layout_types::TextStyle,
        ) -> crate::entities::layout_types::Pt {
            super::super::FixedMetrics.cap_height(size, style)
        }
        fn text_edges(
            &self,
            size: crate::entities::layout_types::Pt,
            style: &crate::entities::layout_types::TextStyle,
        ) -> (crate::entities::layout_types::Pt, crate::entities::layout_types::Pt)
        {
            super::super::FixedMetrics.text_edges(size, style)
        }
        fn underline_metrics(
            &self,
            _: &str,
            size: crate::entities::layout_types::Pt,
            _: &crate::entities::layout_types::TextStyle,
        ) -> (crate::entities::layout_types::Pt, crate::entities::layout_types::Pt)
        {
            (size * 0.1, size * 0.05)
        }
    }

    #[derive(Default, Debug)]
    struct Observed {
        text: Vec<(String, f64, f64)>,
        shapes: Vec<(f64, f64, f64, f64)>,
        lines: Vec<(f64, f64, f64, f64)>,
    }

    fn collect(items: &[FrameItem], x: f64, y: f64, out: &mut Observed) {
        for item in items {
            match item {
                FrameItem::Text { text, pos, .. }
                | FrameItem::TextShaped { text, pos, .. } => {
                    out.text.push((text.to_string(), x + pos.x.0, y + pos.y.0));
                }
                FrameItem::Shape { pos, width, height, .. } => {
                    out.shapes.push((x + pos.x.0, y + pos.y.0, *width, *height));
                }
                FrameItem::Group { pos, matrix, items, .. } => {
                    assert_eq!(
                        *matrix,
                        TransformMatrix::identity(),
                        "these declared-metric fixtures have no affine content"
                    );
                    collect(items, x + pos.x.0, y + pos.y.0, out);
                }
                FrameItem::Semantic { items, .. }
                | FrameItem::TextReplacement { items, .. }
                | FrameItem::Link { items, .. } => collect(items, x, y, out),
                FrameItem::Line { start, end, thickness, .. } => out.lines.push((
                    x + start.x.0,
                    x + end.x.0,
                    y + start.y.0,
                    *thickness,
                )),
                FrameItem::TextSeparator { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. } => {}
            }
        }
    }

    fn rect() -> Content {
        Content::shape(
            ShapeKind::Rect,
            Some(Box::new(Value::Length(Length::pt(80.0)))),
            Some(Box::new(Value::Length(Length::pt(35.0)))),
            None,
            None,
        )
    }

    fn render(body: Content, caption: &str, leading: f64) -> (Observed, f64, f64) {
        render_full(body, Content::text(caption), leading, false)
    }

    fn render_full(
        body: Content,
        caption: Content,
        leading: f64,
        auto_parent: bool,
    ) -> (Observed, f64, f64) {
        render_wrapped(body, caption, leading, auto_parent, false)
    }

    fn render_wrapped(
        body: Content,
        caption: Content,
        leading: f64,
        auto_parent: bool,
        underline: bool,
    ) -> (Observed, f64, f64) {
        render_protocol(body, caption, leading, auto_parent, underline, false, false)
    }

    fn render_protocol(
        body: Content,
        caption: Content,
        leading: f64,
        auto_parent: bool,
        underline: bool,
        newline: bool,
        joined: bool,
    ) -> (Observed, f64, f64) {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            DeclaredFigureMetrics,
            super::super::NullImageSizer,
            10.0,
            (&intr as &dyn Introspector).track(),
        );
        l.chain = l.chain.push_custom("figure.numbering", Value::None);
        l.chain = l.chain.push_styles(&Styles::from_iter([
            Style::Size(Length::pt(10.0)),
            Style::Leading(Length::em(leading)),
        ]));
        l.style = crate::entities::layout_types::TextStyle::from(&l.chain);
        let top = l.page_config.margin.top;
        let left = l.page_config.margin.left;
        let available = l.available_width();
        let figure = Content::Figure(Arc::new(FigureElem {
            body,
            caption: Some(caption),
            kind: None,
            resolved_kind: None,
            supplement: None,
            resolved_supplement: None,
        }));
        let figure =
            if underline { Content::underline(figure, None, None, None) } else { figure };
        let figure = if auto_parent {
            use crate::entities::{
                corners::Corners, elements::block::BlockElem, sides::Sides,
            };
            Content::Block(Arc::new(BlockElem {
                body: figure,
                width: None,
                height: None,
                inset: Sides::uniform(Length::ZERO),
                breakable: true,
                outset: Sides::uniform(Length::ZERO),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            }))
        } else {
            figure
        };
        if joined {
            let mut parts = vec![figure];
            if newline {
                parts.push(Content::SpaceFromNewline);
            }
            parts.push(Content::text("TAIL"));
            l.layout_content(&Content::Sequence(parts.into()));
        } else {
            l.layout_content(&figure);
            if newline {
                l.layout_content(&Content::SpaceFromNewline);
            }
            l.layout_content(&Content::text("TAIL"));
        }
        assert!(l.layout_errors.is_empty(), "{:?}", l.layout_errors);
        let document = l.finish();
        assert_eq!(document.pages.len(), 1);
        let mut out = Observed::default();
        collect(&document.pages[0].items, 0.0, 0.0, &mut out);
        assert!(out.text.iter().any(|t| t.0.contains("TAIL")), "{out:?}");
        (out, top, left + available / 2.0)
    }

    fn word<'a>(out: &'a Observed, name: &str) -> &'a (String, f64, f64) {
        out.text
            .iter()
            .find(|t| t.0 == name)
            .unwrap_or_else(|| panic!("missing {name}: {out:?}"))
    }

    #[test]
    fn readiness_figure_caption_rect_gap_is_independent_of_paragraph_leading() {
        for leading in [0.65, 2.0] {
            let (out, top, _) = render(rect(), "CAP", leading);
            assert_eq!(out.shapes.len(), 1, "{out:?}");
            assert!(
                (word(&out, "CAP").2 - top - 48.5).abs() < 1e-8,
                "leading={leading}: {out:?}"
            );
            assert!(word(&out, "TAIL").2 > word(&out, "CAP").2, "{out:?}");
        }
    }

    #[test]
    fn readiness_figure_empty_body_has_no_caption_gap_or_empty_line() {
        let (out, top, _) = render(Content::empty(), "CAP", 0.65);
        assert!((word(&out, "CAP").2 - top - 7.0).abs() < 1e-8, "{out:?}");
    }

    #[test]
    fn readiness_figure_text_body_and_caption_keep_their_complete_lines() {
        let (out, top, _) = render(Content::text("BODY"), "CAP", 0.65);
        assert!((word(&out, "BODY").2 - top - 7.0).abs() < 1e-8, "{out:?}");
        assert!((word(&out, "CAP").2 - top - 20.5).abs() < 1e-8, "{out:?}");
    }

    #[test]
    fn readiness_figure_default_centers_body_and_caption() {
        let (out, _, center) = render(rect(), "CAP", 0.65);
        assert!((out.shapes[0].0 - (center - 40.0)).abs() < 1e-8, "{out:?}");
        assert!((word(&out, "CAP").1 - (center - 9.0)).abs() < 1e-8, "{out:?}");
    }

    #[test]
    fn readiness_figure_explicit_body_left_uses_common_intrinsic_caption_width() {
        let body = Content::align(
            crate::entities::layout_types::Align2D::from_string("left"),
            rect(),
        );
        let (out, _, center) = render(body, "A VERY WIDE CAPTION", 0.65);
        assert!((out.shapes[0].0 - (center - 57.0)).abs() < 1e-8, "{out:?}");
        assert!((word(&out, "A").1 - (center - 57.0)).abs() < 1e-8, "{out:?}");
    }
    #[test]
    fn readiness_figure_body_centers_each_unequal_line() {
        let body = Content::Sequence(
            vec![Content::text("BODY LONG"), Content::linebreak(), Content::text("B")]
                .into(),
        );
        let (out, _, center) = render(body, "CAP", 0.65);
        assert!((word(&out, "BODY").1 - (center - 27.0)).abs() < 1e-8, "{out:?}");
        assert!((word(&out, "B").1 - (center - 3.0)).abs() < 1e-8, "{out:?}");
    }

    #[test]
    fn readiness_figure_caption_centers_each_unequal_line() {
        let caption = Content::Sequence(
            vec![Content::text("CAPLONG"), Content::linebreak(), Content::text("C")]
                .into(),
        );
        let (out, _, center) = render_full(rect(), caption, 0.65, false);
        assert!((word(&out, "CAPLONG").1 - (center - 21.0)).abs() < 1e-8, "{out:?}");
        assert!((word(&out, "C").1 - (center - 3.0)).abs() < 1e-8, "{out:?}");
    }

    #[test]
    fn readiness_figure_right_caption_keeps_intrinsic_visible_frame() {
        let caption = Content::align(
            crate::entities::layout_types::Align2D::from_string("right"),
            Content::text("CAP"),
        );
        let (out, _, center) = render_full(rect(), caption, 0.65, false);
        assert!((word(&out, "CAP").1 - (center - 9.0)).abs() < 1e-8, "{out:?}");
        assert!(word(&out, "TAIL").2 > word(&out, "CAP").2, "{out:?}");
    }

    #[test]
    fn readiness_figure_auto_parent_uses_its_actual_intrinsic_width() {
        let (out, _, _) = render_full(rect(), Content::text("CAP"), 0.65, true);
        let left = word(&out, "TAIL").1;
        assert!((out.shapes[0].0 - left).abs() < 1e-8, "{out:?}");
        assert!((word(&out, "CAP").1 - (left + 31.0)).abs() < 1e-8, "{out:?}");
    }
    #[test]
    fn readiness_figure_fixed_child_block_keeps_its_own_center() {
        let body = Content::block(
            Content::text("BODY"),
            Some(Length::pt(80.0)),
            Some(Length::pt(35.0)),
            crate::entities::sides::Sides::uniform(Length::ZERO),
            true,
        );
        let (out, _, center) = render(body, "CAP", 0.65);
        assert!((word(&out, "BODY").1 - (center - 12.0)).abs() < 1e-8, "{out:?}");
    }

    #[test]
    fn readiness_figure_outer_underline_preserves_completed_flow_before_tail() {
        let (out, top, _) =
            render_wrapped(rect(), Content::text("CAP"), 0.65, false, true);
        assert!((word(&out, "CAP").2 - top - 48.5).abs() < 1e-8, "{out:?}");
        assert!(word(&out, "TAIL").2 > word(&out, "CAP").2, "{out:?}");
    }
    #[test]
    fn readiness_figure_fixed_child_backing_has_the_same_center_as_body() {
        let body = Content::block(
            Content::text("BODY"),
            Some(Length::pt(80.0)),
            Some(Length::pt(35.0)),
            crate::entities::sides::Sides::uniform(Length::ZERO),
            true,
        );
        let Content::Block(block) = body else { unreachable!() };
        let mut block = (*block).clone();
        block.fill = Some(crate::entities::layout_types::Color::rgb(0, 0, 0));
        let (out, _, center) = render(Content::Block(Arc::new(block)), "CAP", 0.65);
        assert_eq!(out.shapes.len(), 1, "{out:?}");
        assert!((out.shapes[0].0 - (center - 40.0)).abs() < 1e-8, "{out:?}");
        assert!((word(&out, "BODY").1 - (center - 12.0)).abs() < 1e-8, "{out:?}");
    }

    #[test]
    fn readiness_figure_underline_then_markup_newline_keeps_tail_below_caption() {
        let (out, top, _) =
            render_protocol(rect(), Content::text("CAP"), 0.65, false, true, true, false);
        assert!((word(&out, "CAP").2 - top - 48.5).abs() < 1e-8, "{out:?}");
        assert!(word(&out, "TAIL").2 > word(&out, "CAP").2, "{out:?}");
    }
    #[test]
    fn readiness_figure_joined_decorated_sequence_preserves_tail_paragraph() {
        let (out, top, _) =
            render_protocol(rect(), Content::text("CAP"), 0.65, false, true, true, true);
        assert!((word(&out, "CAP").2 - top - 48.5).abs() < 1e-8, "{out:?}");
        assert_eq!(out.lines.len(), 1, "{out:?}");
        let cap = word(&out, "CAP");
        let line = out.lines[0];
        assert!((line.0 - cap.1).abs() < 1e-8, "{out:?}");
        assert!((line.1 - cap.1 - 18.0).abs() < 1e-8, "{out:?}");
        assert!((line.2 - cap.2 - 1.0).abs() < 1e-8, "{out:?}");
        assert!((line.3 - 0.5).abs() < 1e-8, "{out:?}");
        assert!(word(&out, "TAIL").2 > word(&out, "CAP").2, "{out:?}");
    }

    #[test]
    fn readiness_figure_explicit_left_painted_block_keeps_body_and_backing_together() {
        let body = Content::block(
            Content::text("BODY"),
            Some(Length::pt(80.0)),
            Some(Length::pt(35.0)),
            crate::entities::sides::Sides::uniform(Length::ZERO),
            true,
        );
        let Content::Block(block) = body else { unreachable!() };
        let mut block = (*block).clone();
        block.fill = Some(crate::entities::layout_types::Color::rgb(0, 0, 0));
        let body = Content::align(
            crate::entities::layout_types::Align2D::from_string("left"),
            Content::Block(Arc::new(block)),
        );
        let (out, _, center) = render(body, "A VERY WIDE CAPTION", 0.65);
        assert_eq!(out.shapes.len(), 1, "{out:?}");
        assert!((out.shapes[0].0 - (center - 57.0)).abs() < 1e-8, "{out:?}");
        assert!((word(&out, "BODY").1 - (center - 57.0)).abs() < 1e-8, "{out:?}");
    }
}
