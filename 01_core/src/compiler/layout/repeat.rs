//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/repeat.md
//! @prompt-hash 1939248b
//! @layer L1
//! @updated 2026-10-22
//!
//! Repeat owns one planned source piece and a full-width prepared flow frame.

use crate::entities::elements::repeat::RepeatElem;
use crate::entities::layout_types::{FrameItem, HAlign, Point, Pt, TransformMatrix};

use super::{FontMetrics, ImageSizer, Layouter};

/// Legacy inline-range transport retained for existing line-close clients.
#[derive(Debug, Clone)]
pub(super) struct PendingRepeat {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) width: f64,
    pub(super) gap: f64,
    pub(super) justify: bool,
}

/// Geometry produced by one nonexpanding piece, before its width is known.
#[derive(Debug, Clone)]
pub(super) struct PieceAlignment {
    region_width: f64,
    lines: Vec<AlignedLine>,
    pending: Option<(HAlign, f64)>,
    pending_paint: Vec<(usize, usize)>,
    paint: Vec<(usize, usize)>,
}

#[derive(Debug, Clone)]
struct AlignedLine {
    operation: Option<(HAlign, f64)>,
    items: Vec<usize>,
}

impl PieceAlignment {
    pub(super) fn new(region_width: f64) -> Self {
        Self {
            region_width,
            lines: Vec::new(),
            pending: None,
            pending_paint: Vec::new(),
            paint: Vec::new(),
        }
    }

    fn shift(&self, line: usize, width: f64) -> f64 {
        let Some((alignment, original_free)) = self.lines[line].operation else {
            return 0.0;
        };
        let free = original_free + width - self.region_width;
        match alignment {
            HAlign::Center => free / 2.0,
            HAlign::Right | HAlign::End => free,
            HAlign::Left | HAlign::Start => 0.0,
        }
    }

    pub(super) fn settle(
        &self,
        items: &mut [FrameItem],
        decorations: &mut [super::DecoSegment],
        width: f64,
    ) {
        if !width.is_finite() || !self.region_width.is_finite() {
            return;
        }
        for (id, line) in self.lines.iter().enumerate() {
            let delta = self.shift(id, width);
            for &index in &line.items {
                super::helpers::shift_frame_item_x(&mut items[index], delta);
            }
        }
        for &(index, line) in &self.paint {
            super::helpers::shift_frame_item_x(
                &mut items[index],
                self.shift(line, width),
            );
        }
        for segment in decorations {
            if let Some(line) = segment.piece_line.take() {
                let delta = Pt(self.shift(line, width));
                segment.start_x += delta;
                segment.end_x += delta;
            }
        }
    }
}

pub(super) fn piece_line<M: FontMetrics, S: ImageSizer>(
    layouter: &Layouter<M, S>,
) -> Option<usize> {
    layouter.piece_alignment.as_ref().map(|scope| scope.lines.len())
}

pub(super) fn piece_paint<M: FontMetrics, S: ImageSizer>(
    layouter: &Layouter<M, S>,
    index: usize,
) -> bool {
    layouter
        .piece_alignment
        .as_ref()
        .is_some_and(|scope| scope.pending_paint.iter().any(|&(paint, _)| paint == index))
}

pub(super) fn register_piece_paint<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    line: Option<usize>,
) {
    if let (Some(scope), Some(line)) = (layouter.piece_alignment.as_mut(), line) {
        scope
            .pending_paint
            .push((layouter.regions.current.current_line.len(), line));
    }
}

pub(super) fn defer_piece_line<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    alignment: HAlign,
    free: f64,
) -> bool {
    if let Some(scope) = layouter.piece_alignment.as_mut() {
        scope.pending = Some((alignment, free));
        true
    } else {
        false
    }
}

pub(super) fn commit_piece_line<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
) {
    let Some(scope) = layouter.piece_alignment.as_mut() else {
        return;
    };
    let start = layouter.regions.current.current_items.len();
    let count = layouter.regions.current.current_line.len();
    let paint = std::mem::take(&mut scope.pending_paint);
    let items = (0..count)
        .filter(|index| !paint.iter().any(|&(paint, _)| paint == *index))
        .map(|index| start + index)
        .collect();
    scope
        .lines
        .push(AlignedLine { operation: scope.pending.take(), items });
    scope
        .paint
        .extend(paint.into_iter().map(|(index, line)| (start + index, line)));
}

/// Compose one source piece, then repeat its complete visual frame in flow.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &RepeatElem,
) {
    let span = layouter.current_source_span;
    let gap = e.gap.map_or(0.0, |value| value.resolve_pt(layouter.style.size.val()));
    let (origin, fill) = super::block::admit_prepared_frame(layouter);
    let inherited_alignment = layouter.explicit_line_alignment;
    let rtl = layouter.chain.custom("text.dir")
        == Some(&crate::entities::value::Value::Dir(crate::entities::dir::Dir::RTL));
    let alignment =
        inherited_alignment.unwrap_or(if rtl { HAlign::Right } else { HAlign::Left });
    let planned = layouter.planned_child_cursor(
        crate::compiler::introspect::page_running::PlanEdge::RepeatBody,
    );
    let (height, mut piece, decorations, pending_x, pending_y) = layouter
        .layout_repeat_piece(
            &e.body,
            super::sub_frame::SubLayoutRegion {
                origin_x: 0.0,
                width: fill,
                height: None,
                align_rtl: false,
                unconstrained_height: true,
            },
            planned.clone(),
        );
    let baseline = layouter.last_sub_frame_baseline;
    // Flow occupancy is recorded before alignment and excludes overflowing
    // paint. A nonexpanding piece's logical size is bounded by its region.
    let width = layouter.last_sub_frame_flow_width.min(fill);
    if !fill.is_finite() || !height.is_finite() {
        layouter.layout_errors.push(
            crate::entities::source_result::SourceDiagnostic::error(
                span,
                "repeat with no size restrictions",
            ),
        );
        return;
    }
    settle_piece_alignment(layouter, &mut piece, width, height, pending_x, pending_y);
    // Resolve the one source link scope before cloning: each visual copy must
    // retain its complete annotation geometry without duplicating raw scopes.
    super::link_capture::resolve(&mut piece, &layouter.link_capture, &layouter.metrics);

    let count = ((fill + gap) / (width + gap)).floor();
    let remaining = (fill + gap) % (width + gap);
    let adjusted_gap = if e.justify { gap + remaining / (count - 1.0) } else { gap };
    let offset = if count == 1.0 || !e.justify {
        match alignment {
            HAlign::Left => 0.0,
            HAlign::Right => remaining,
            HAlign::Center => remaining / 2.0,
            HAlign::Start => {
                if rtl {
                    remaining
                } else {
                    0.0
                }
            }
            HAlign::End => {
                if rtl {
                    0.0
                } else {
                    remaining
                }
            }
        }
    } else {
        0.0
    };
    let copies = if width > 0.0 { (count as usize).min(1000) } else { 0 };
    let mut items = Vec::with_capacity(copies);
    let mut repeated_decorations = Vec::new();
    let mut x = offset;
    for index in 0..copies {
        if !x.is_finite() {
            break;
        }
        let mut visual = piece.clone();
        if index != 0 {
            strip_source_markers(&mut visual);
        }
        items.push(FrameItem::Group {
            paint_root: false,
            pos: Point { x: Pt(x), y: Pt(0.0) },
            matrix: TransformMatrix::identity(),
            clip_mask: None,
            inner_width: width,
            inner_height: height,
            items: visual,
        });
        repeated_decorations.extend(decorations.iter().map(|segment| {
            super::DecoSegment {
                start_x: segment.start_x + Pt(x),
                end_x: segment.end_x + Pt(x),
                baseline_y: segment.baseline_y,
                piece_line: segment.piece_line,
            }
        }));
        x += width + adjusted_gap;
    }
    if items.is_empty() {
        layouter.discard_empty_repeat_body(planned.as_ref());
    }
    let actual_origin = super::block::commit_prepared_frame(
        layouter, origin, fill, height, baseline, items,
    );
    if let Some(collector) = layouter.decoration_lines_collector.as_mut() {
        collector.extend(repeated_decorations.into_iter().map(|segment| {
            super::DecoSegment {
                start_x: segment.start_x + actual_origin.x,
                end_x: segment.end_x + actual_origin.x,
                baseline_y: segment.baseline_y + actual_origin.y,
                piece_line: segment.piece_line,
            }
        }));
    }
}

/// Local fixup indices belong to the prepared piece, never the parent's list.
fn settle_piece_alignment<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    piece: &mut [FrameItem],
    width: f64,
    height: f64,
    pending_x: Vec<super::PendingAlignXEntry>,
    pending_y: Vec<super::PendingAlignYEntry>,
) {
    let saved_x = std::mem::replace(&mut layouter.pending_align_centering, pending_x);
    let saved_y = std::mem::replace(&mut layouter.pending_align_v_centering, pending_y);
    layouter.apply_pending_align_fixups(
        piece,
        width + layouter.page_config.margin.horizontal(),
    );
    layouter
        .apply_pending_align_v_fixups(piece, height + layouter.page_config.margin.bottom);
    layouter.pending_align_centering = saved_x;
    layouter.pending_align_v_centering = saved_y;
}

/// Render copies carry visuals, while only the actual source piece can own
/// positions and source packet observations.
fn strip_source_markers(items: &mut Vec<FrameItem>) {
    items.retain_mut(|item| {
        if super::html_frame::is_context_marker(item) {
            return false;
        }
        match item {
            FrameItem::Group { items, .. }
            | FrameItem::Semantic { items, .. }
            | FrameItem::TextReplacement { items, .. }
            | FrameItem::Link { items, .. } => strip_source_markers(items),
            FrameItem::Text { .. }
            | FrameItem::TextShaped { .. }
            | FrameItem::TextSeparator { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Line { .. }
            | FrameItem::Shape { .. }
            | FrameItem::Image { .. } => {}
        }
        true
    });
}

/// Expande uma peça dentro de `fill`, preservando a cópia inicial. Retorna o
/// número de `FrameItem`s inseridos para correção de índices externos.
pub(super) fn expand_pending(
    line: &mut Vec<FrameItem>,
    pending: &PendingRepeat,
    fill: f64,
) -> usize {
    if pending.start >= pending.end
        || pending.end > line.len()
        || !fill.is_finite()
        || fill <= 0.0
    {
        return 0;
    }
    let stride = pending.width + pending.gap;
    if !stride.is_finite() || stride <= 0.0 {
        return 0;
    }
    let count = (((fill + pending.gap) / stride).floor() as usize).clamp(1, 1000);
    if count <= 1 {
        return 0;
    }
    let occupied = count as f64 * pending.width + (count - 1) as f64 * pending.gap;
    let residual = (fill - occupied).max(0.0);
    let gap = if pending.justify {
        pending.gap + residual / (count - 1) as f64
    } else {
        pending.gap
    };
    let template = line[pending.start..pending.end].to_vec();
    let template_len = template.len();
    let mut copies = Vec::with_capacity((count - 1).saturating_mul(template_len));
    for copy in 1..count {
        let dx = copy as f64 * (pending.width + gap);
        copies.extend(template.iter().cloned().map(|item| {
            let (x, y) = super::helpers::item_pos(&item);
            super::helpers::translate_frame_item(item, Pt(x + dx), Pt(y))
        }));
    }
    let inserted = copies.len();
    line.splice(pending.end..pending.end, copies);
    inserted
}

#[cfg(test)]
mod nested_inline_repeat_tests {
    use super::*;
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{
            HAlign, Length, PageMarginSpec, PageMargins, PagedDocument, Point, TextStyle,
            TransformMatrix,
        },
        sides::Sides,
        value::Value,
    };
    use comemo::Track;

    struct DeclaredMetrics;
    impl FontMetrics for DeclaredMetrics {
        fn advance(&self, text: &str, _: Pt, _: &TextStyle) -> Pt {
            Pt(text.chars().count() as f64 * 10.0)
        }
        fn text_width(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
            self.advance(text, size, style)
        }
        fn vertical_metrics(&self, _: Pt, _: &TextStyle) -> (Pt, Pt) {
            (Pt(6.0), Pt(8.0))
        }
        fn cap_height(&self, _: Pt, _: &TextStyle) -> Pt {
            Pt(6.0)
        }
        fn text_edges(&self, _: Pt, _: &TextStyle) -> (Pt, Pt) {
            (Pt(6.0), Pt(-2.0))
        }
    }

    fn document(
        content: Content,
        width: f64,
        alignment: Option<HAlign>,
    ) -> PagedDocument {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            DeclaredMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = width;
        l.page_config.height = f64::INFINITY;
        l.page_config.margin = PageMargins::uniform(10.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(10.0);
        l.regions.current.width = width;
        l.regions.current.height = f64::INFINITY;
        l.regions.current.cursor_x = Pt(10.0);
        l.regions.current.line_start_x = Pt(10.0);
        l.regions.current.cursor_y = Pt(10.0);
        l.chain = l.chain.push_custom("par.spacing", Value::Length(Length::pt(12.0)));
        l.explicit_line_alignment = alignment;
        l.layout_content(&content);
        l.finish()
    }
    fn observed(doc: &PagedDocument, text: &str) -> Vec<Point> {
        fn walk(
            items: &[FrameItem],
            path: &mut Vec<(Point, TransformMatrix)>,
            text: &str,
            result: &mut Vec<Point>,
        ) {
            for item in items {
                match item {
                    FrameItem::Text { pos, text: value, .. }
                    | FrameItem::TextShaped { pos, text: value, .. } => {
                        if value.as_str() == text {
                            let mut p = *pos;
                            for (origin, matrix) in path.iter().rev() {
                                let (x, y) = matrix.apply(p.x.0, p.y.0);
                                p = Point { x: origin.x + Pt(x), y: origin.y + Pt(y) };
                            }
                            result.push(p);
                        }
                    }
                    FrameItem::Group { pos, matrix, items, .. } => {
                        path.push((*pos, *matrix));
                        walk(items, path, text, result);
                        path.pop();
                    }
                    FrameItem::Semantic { items, .. }
                    | FrameItem::TextReplacement { items, .. }
                    | FrameItem::Link { items, .. } => walk(items, path, text, result),
                    FrameItem::TextSeparator { .. }
                    | FrameItem::Line { .. }
                    | FrameItem::Glyph { .. }
                    | FrameItem::Image { .. }
                    | FrameItem::Shape { .. } => {}
                }
            }
        }
        let mut result = Vec::new();
        for page in &doc.pages {
            walk(&page.items, &mut Vec::new(), text, &mut result);
        }
        result
    }
    fn repeat(body: Content, gap: f64, justify: bool) -> Content {
        Content::repeat(body, Some(Length::pt(gap)), justify)
    }
    fn close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() < 1e-9, "actual={actual}, expected={expected}");
    }

    #[test]
    fn nested_inline_repeat_owns_full_region_and_flow_boundaries() {
        let doc = document(
            Content::sequence(vec![
                Content::text("BEFORE"),
                repeat(Content::text("X"), 3.0, false),
                Content::text("TAIL"),
            ]),
            100.0,
            None,
        );
        let before = observed(&doc, "BEFORE");
        let copies = observed(&doc, "X");
        let tail = observed(&doc, "TAIL");
        assert_eq!((before.len(), copies.len(), tail.len()), (1, 6, 1));
        close(copies[0].x.0, 10.0);
        close(copies[0].y.0 - before[0].y.0, 20.0);
        close(tail[0].y.0 - copies[0].y.0, 20.0);
        close(tail[0].x.0, 10.0);
    }

    #[test]
    fn nested_inline_repeat_gap_justify_uses_actual_region_residual() {
        for (justify, stride) in [(false, 13.0), (true, 14.0)] {
            let doc = document(repeat(Content::text("X"), 3.0, justify), 100.0, None);
            let copies = observed(&doc, "X");
            assert_eq!(copies.len(), 6);
            for (index, point) in copies.iter().enumerate() {
                close(point.x.0, 10.0 + index as f64 * stride);
            }
        }
    }

    #[test]
    fn nested_inline_repeat_signed_gap_and_zero_copy_are_not_clamped() {
        for (gap, count) in [(-1.0, 8), (-100.0, 0)] {
            let doc = document(repeat(Content::text("X"), gap, false), 100.0, None);
            assert!(doc.layout_errors.is_empty());
            assert_eq!(observed(&doc, "X").len(), count);
        }
    }

    #[test]
    fn nested_inline_repeat_zero_width_piece_emits_no_visual_copy() {
        let zero_width = Content::boxed(
            Content::text("X"),
            Some(Length::ZERO),
            Some(Length::pt(8.0)),
            Sides::uniform(Length::ZERO),
            Length::ZERO,
        );
        let doc = document(repeat(zero_width, 0.0, false), 100.0, None);
        assert!(observed(&doc, "X").is_empty());
        assert!(doc.layout_errors.is_empty());
    }

    #[test]
    fn nested_inline_repeat_multiline_piece_preserves_each_complete_copy() {
        let body = Content::sequence(vec![
            Content::text("U"),
            Content::linebreak(),
            Content::text("V"),
        ]);
        let doc = document(repeat(body, 3.0, false), 100.0, None);
        let upper = observed(&doc, "U");
        let lower = observed(&doc, "V");
        assert_eq!((upper.len(), lower.len()), (6, 6));
        for (upper, lower) in upper.iter().zip(&lower) {
            close(upper.x.0, lower.x.0);
            assert!(lower.y.0 > upper.y.0);
        }
    }

    #[test]
    fn nested_inline_repeat_nonjustified_inherits_horizontal_alignment() {
        let doc =
            document(repeat(Content::text("X"), 3.0, false), 100.0, Some(HAlign::Right));
        let copies = observed(&doc, "X");
        assert_eq!(copies.len(), 6);
        close(copies[0].x.0, 15.0);
        close(copies[5].x.0, 80.0);
    }

    #[test]
    fn nested_inline_repeat_unbounded_region_reports_layout_error() {
        let doc = document(repeat(Content::text("X"), 0.0, true), f64::INFINITY, None);
        assert!(doc
            .layout_errors
            .iter()
            .any(|error| error.message == "repeat with no size restrictions"));
    }

    #[test]
    fn repeat_piece_alignment_nested_fixed_child_center() {
        let child = Content::boxed(
            Content::text("X"),
            Some(Length::pt(25.0)),
            None,
            Sides::uniform(Length::ZERO),
            Length::ZERO,
        );
        let doc = document(repeat(child, 100.0, true), 100.0, Some(HAlign::Center));
        let copies = observed(&doc, "X");
        assert_eq!(copies.len(), 1);
        // Region80; one25pt frame centers by27.5; its10pt text centers by7.5.
        close(copies[0].x.0, 10.0 + (80.0 - 25.0) / 2.0 + (25.0 - 10.0) / 2.0);
    }

    #[test]
    fn repeat_piece_alignment_nested_fixed_child_right() {
        let child = Content::boxed(
            Content::text("X"),
            Some(Length::pt(25.0)),
            None,
            Sides::uniform(Length::ZERO),
            Length::ZERO,
        );
        let doc = document(repeat(child, 100.0, true), 100.0, Some(HAlign::Right));
        let copies = observed(&doc, "X");
        assert_eq!(copies.len(), 1);
        close(copies[0].x.0, 10.0 + (80.0 - 25.0) + (25.0 - 10.0));
    }

    fn unequal_piece() -> Content {
        Content::sequence(vec![
            Content::text("WIDE"),
            Content::linebreak(),
            Content::text("X"),
        ])
    }

    #[test]
    fn repeat_piece_alignment_unequal_lines_center() {
        let doc =
            document(repeat(unequal_piece(), 100.0, true), 100.0, Some(HAlign::Center));
        let wide = observed(&doc, "WIDE");
        let narrow = observed(&doc, "X");
        assert_eq!((wide.len(), narrow.len()), (1, 1));
        close(wide[0].x.0, 10.0 + (80.0 - 40.0) / 2.0);
        close(narrow[0].x.0, wide[0].x.0 + (40.0 - 10.0) / 2.0);
    }

    #[test]
    fn repeat_piece_alignment_unequal_lines_right() {
        let doc =
            document(repeat(unequal_piece(), 100.0, true), 100.0, Some(HAlign::Right));
        let wide = observed(&doc, "WIDE");
        let narrow = observed(&doc, "X");
        assert_eq!((wide.len(), narrow.len()), (1, 1));
        close(wide[0].x.0, 10.0 + 80.0 - 40.0);
        close(narrow[0].x.0, wide[0].x.0 + 40.0 - 10.0);
    }

    #[test]
    fn repeat_piece_alignment_decorated_unequal_lines_center() {
        fn lines(
            items: &[FrameItem],
            matrix: TransformMatrix,
            out: &mut Vec<(f64, f64)>,
        ) {
            for item in items {
                match item {
                    FrameItem::Line { start, end, .. } => {
                        out.push((
                            matrix.apply(start.x.0, start.y.0).0,
                            matrix.apply(end.x.0, end.y.0).0,
                        ));
                    }
                    FrameItem::Group { pos, matrix: local, items, .. } => {
                        lines(
                            items,
                            matrix
                                .concat(&TransformMatrix::translate(pos.x.0, pos.y.0))
                                .concat(local),
                            out,
                        );
                    }
                    FrameItem::Semantic { items, .. }
                    | FrameItem::TextReplacement { items, .. }
                    | FrameItem::Link { items, .. } => lines(items, matrix, out),
                    FrameItem::Text { .. }
                    | FrameItem::TextShaped { .. }
                    | FrameItem::TextSeparator { .. }
                    | FrameItem::Glyph { .. }
                    | FrameItem::Shape { .. }
                    | FrameItem::Image { .. } => {}
                }
            }
        }
        let body = Content::Underline(std::sync::Arc::new(
            crate::entities::elements::underline::UnderlineElem {
                body: unequal_piece(),
                stroke: None,
                offset: None,
                extent: None,
            },
        ));
        let doc = document(repeat(body, 100.0, true), 100.0, Some(HAlign::Center));
        let mut observed_lines = Vec::new();
        for page in &doc.pages {
            lines(&page.items, TransformMatrix::identity(), &mut observed_lines);
        }
        assert_eq!(observed_lines.len(), 2);
        for (actual, expected) in observed_lines.iter().zip([(30.0, 70.0), (45.0, 55.0)])
        {
            close(actual.0, expected.0);
            close(actual.1, expected.1);
        }
        let narrow = observed(&doc, "X");
        assert_eq!(narrow.len(), 1);
        close(narrow[0].x.0, 45.0);
    }
    #[test]
    fn repeat_piece_alignment_flow_source_anchor_is_not_a_text_line() {
        use crate::entities::{
            content::LocatedContent,
            elements::shape::{ShapeElem, ShapeOrigin},
            geometry::ShapeKind,
            locator::Locator,
        };
        let location = Locator::new().next();
        let shape = Content::Shape(std::sync::Arc::new(ShapeElem {
            origin: ShapeOrigin::Geometry,
            kind: ShapeKind::Rect,
            width: Some(Box::new(Value::Length(Length::pt(25.0)))),
            height: Some(Box::new(Value::Length(Length::pt(8.0)))),
            fill: None,
            stroke: None,
            fill_rule: Default::default(),
        }));
        let body = Content::Located(std::sync::Arc::new(LocatedContent {
            location,
            body: shape,
        }));
        let doc = document(repeat(body, 100.0, true), 100.0, Some(HAlign::Center));
        let anchor = doc
            .extracted_positions
            .position_of(location)
            .expect("the actual direct shape location must survive");
        assert_eq!(anchor.page.get(), 1);
        // The actual shape is not a paragraph. Only Repeat's residual applies.
        close(anchor.point.x.0, 10.0 + (80.0 - 25.0) / 2.0);
    }
    fn explicit_left_child(source_owned: bool) -> Content {
        use crate::entities::{
            content::SourceContent, elements::align::AlignElem, layout_types::Align2D,
            span::Span,
        };
        let text = Content::text("X");
        let body = if source_owned {
            Content::Source(std::sync::Arc::new(SourceContent {
                body: text,
                span: Span::detached(),
            }))
        } else {
            text
        };
        Content::boxed(
            Content::Align(std::sync::Arc::new(AlignElem {
                alignment: Align2D { h: Some(HAlign::Left), v: None },
                body,
            })),
            Some(Length::pt(25.0)),
            None,
            Sides::uniform(Length::ZERO),
            Length::ZERO,
        )
    }

    #[test]
    fn repeat_piece_alignment_explicit_bare_child_left_preserves_override() {
        let doc = document(
            repeat(explicit_left_child(false), 100.0, true),
            100.0,
            Some(HAlign::Center),
        );
        let copies = observed(&doc, "X");
        assert_eq!(copies.len(), 1);
        close(copies[0].x.0, 10.0 + (80.0 - 25.0) / 2.0);
    }

    #[test]
    fn repeat_piece_alignment_explicit_source_child_left_preserves_override() {
        let doc = document(
            repeat(explicit_left_child(true), 100.0, true),
            100.0,
            Some(HAlign::Center),
        );
        let copies = observed(&doc, "X");
        assert_eq!(copies.len(), 1);
        close(copies[0].x.0, 10.0 + (80.0 - 25.0) / 2.0);
    }
}
