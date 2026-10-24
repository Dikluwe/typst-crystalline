//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/footnote_fragments.md
//! @prompt-hash ca19af50
//! @layer L1
//! @updated 2026-10-24

use super::{DeferredFootnoteFrame, FontMetrics, FootnoteBreak, ImageSizer, Layouter};
use crate::entities::content::Content;

#[derive(Clone, Copy)]
enum BoundaryRole {
    Transparent,
    Spacing,
    Atomic,
    Uncertified,
}

fn certifies_spacing_gap(space: &crate::entities::elements::v_space::VSpaceElem) -> bool {
    if space.weak {
        return false;
    }
    if space.amount.em != 0.0 {
        return false;
    }
    let absolute = space.amount.abs.to_pt();
    if !absolute.is_finite() {
        return false;
    }
    absolute > 0.0
}

fn boundary_role(content: &Content) -> BoundaryRole {
    match content {
        Content::Source(e) => boundary_role(&e.body),
        Content::Located(e) => boundary_role(&e.body),
        Content::Styled(e) => boundary_role(&e.body),
        Content::ShowRecipe(e) => boundary_role(&e.body),
        Content::Realized(e) => boundary_role(&e.visual),
        Content::Strong(e) => boundary_role(&e.body),
        Content::Emph(e) => boundary_role(&e.body),
        Content::Figure(_) | Content::Block(_) => BoundaryRole::Atomic,
        Content::VSpace(e) if certifies_spacing_gap(e) => BoundaryRole::Spacing,
        Content::VSpace(_) => BoundaryRole::Uncertified,
        Content::Sequence(_)
        | Content::Text(_)
        | Content::Empty
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Label(_)
        | Content::Metadata(_)
        | Content::State(_)
        | Content::StateUpdate(_)
        | Content::CounterUpdate(_)
        | Content::HSpace(_)
        | Content::Linebreak(_) => BoundaryRole::Transparent,
        Content::HtmlElem(_)
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
        | Content::Ref(_)
        | Content::CounterDisplay(_)
        | Content::Outline(_)
        | Content::OutlineEntry(_)
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
        | Content::Pagebreak(_)
        | Content::Colbreak(_)
        | Content::Stack(_)
        | Content::Boxed(_)
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
        | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_)
        | Content::ContextBlock(_)
        | Content::Dynamic(_) => BoundaryRole::Uncertified,
    }
}

#[derive(Default)]
struct SourceLines {
    paragraphs: Vec<Vec<bool>>,
    costs: Vec<(bool, bool)>,
    current: Vec<bool>,
    ink: bool,
    cost: Option<(bool, bool)>,
}

impl SourceLines {
    fn observe(&mut self, chain: &crate::entities::style_chain::StyleChain) -> bool {
        let cost = crate::compiler::text_costs::prevention(chain);
        if self.cost.is_some_and(|previous| previous != cost) {
            return false;
        }
        self.cost = Some(cost);
        true
    }

    fn paragraph(&mut self) {
        self.current.push(self.ink);
        self.paragraphs.push(std::mem::take(&mut self.current));
        self.costs.push(self.cost.take().unwrap_or((true, true)));
        self.ink = false;
    }

    fn scan(
        &mut self,
        content: &Content,
        chain: &crate::entities::style_chain::StyleChain,
    ) -> bool {
        match content {
            Content::Source(e) => self.scan(&e.body, chain),
            Content::Located(e) => self.scan(&e.body, chain),
            Content::Styled(e) => self.scan(&e.body, &chain.push_styles(&e.styles)),
            Content::Strong(e) => self.scan(&e.body, chain),
            Content::Emph(e) => self.scan(&e.body, chain),
            Content::Sequence(parts) => parts.iter().all(|part| self.scan(part, chain)),
            Content::Text(text) => {
                if text.contains(['\n', '\r']) || !self.observe(chain) {
                    return false;
                }
                self.ink |= text.chars().any(|c| !c.is_whitespace());
                true
            }
            Content::Linebreak(_) => {
                if !self.observe(chain) {
                    return false;
                }
                self.current.push(self.ink);
                self.ink = false;
                true
            }
            Content::Parbreak => {
                self.paragraph();
                true
            }
            Content::Empty
            | Content::Space
            | Content::SpaceFromNewline
            | Content::Label(_)
            | Content::Metadata(_)
            | Content::State(_)
            | Content::StateUpdate(_)
            | Content::CounterUpdate(_) => true,
            Content::ShowRecipe(_)
            | Content::Realized(_)
            | Content::Figure(_)
            | Content::Block(_)
            | Content::VSpace(_)
            | Content::HSpace(_)
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
            | Content::Ref(_)
            | Content::CounterDisplay(_)
            | Content::Outline(_)
            | Content::OutlineEntry(_)
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
            | Content::Pagebreak(_)
            | Content::Colbreak(_)
            | Content::Stack(_)
            | Content::Boxed(_)
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
            | Content::StateDisplay(_)
            | Content::CounterDisplayCallback(_)
            | Content::ContextBlock(_)
            | Content::Dynamic(_) => false,
        }
    }

    fn certify(mut self, frame: &mut DeferredFootnoteFrame, origin: f64) {
        self.paragraph();
        let count: usize = self.paragraphs.iter().map(Vec::len).sum();
        let origin = if count > 1 { origin } else { 0.0 };
        if self.paragraphs.iter().flatten().any(|ink| !ink)
            || count != frame.breaks.len() + 1
            || !frame.height.is_finite()
            || !origin.is_finite()
            || frame.height + origin <= 0.0
            || frame.breaks.iter().any(|b| {
                !b.height.is_finite()
                    || b.height + origin <= 0.0
                    || b.tail_start.is_none_or(|t| !t.is_finite() || t + origin < 0.0)
            })
        {
            return;
        }
        let mut protected = vec![None; frame.breaks.len()];
        let mut offset = 0;
        for (lines, (widow, orphan)) in self.paragraphs.iter().zip(self.costs) {
            let len = lines.len();
            let orphan = orphan && lines.get(1).copied().unwrap_or(false);
            let widow = widow && len >= 2 && lines[len - 2];
            let mut index = 0;
            while index < len {
                let group = if index == 0 && len == 3 && widow && orphan {
                    3
                } else if (index == 0 && len >= 2 && orphan)
                    || (index >= 2 && index + 2 == len && widow)
                {
                    2
                } else {
                    1
                };
                let start = offset + index;
                let end = start + group;
                let top = if start == 0 {
                    0.0
                } else {
                    frame.breaks[start - 1].tail_start.unwrap()
                };
                let bottom = if end == count {
                    frame.height
                } else {
                    frame.breaks[end - 1].height
                };
                let height = bottom - top + if start == 0 { origin } else { 0.0 };
                if !height.is_finite() || height < 0.0 {
                    return;
                }
                protected[start..end - 1].fill(Some(height));
                index += group;
            }
            offset += len;
        }
        for (cut, height) in frame.breaks.iter_mut().zip(protected) {
            cut.protected_group_height = height;
        }
        frame.height += origin;
        for item in &mut frame.items {
            super::helpers::offset_frame_item(item, 0.0, origin);
        }
        for segment in &mut frame.deco {
            segment.baseline_y.0 += origin;
        }
        for (_, _, _, _, y, _, applied) in &mut frame.orphaned_y {
            *y += origin;
            *applied += origin;
        }
        for cut in &mut frame.breaks {
            cut.height += origin;
            cut.tail_start = cut.tail_start.map(|y| y + origin);
            cut.gap_start_height = cut.gap_start_height.map(|y| y + origin);
        }
        frame.source_lines = true;
    }
}

pub(super) fn first_line_requirement(frame: &DeferredFootnoteFrame) -> f64 {
    frame
        .breaks
        .first()
        .map_or(frame.height, |cut| cut.protected_group_height.unwrap_or(cut.height))
}

pub(super) fn pending_work(
    frames: &[Option<DeferredFootnoteFrame>],
) -> (usize, usize, usize) {
    (
        frames.iter().filter(|f| f.is_none()).count(),
        frames.len(),
        frames.iter().flatten().fold(0usize, |sum, frame| {
            sum.saturating_add(frame.items.len())
                .saturating_add(frame.breaks.len())
                .saturating_add(usize::from(!frame.line_continuation))
        }),
    )
}

fn cut_for(
    frame: &DeferredFootnoteFrame,
    available: f64,
    future_capacity: f64,
) -> Option<&FootnoteBreak> {
    frame.breaks.iter().rev().find(|b| {
        b.protected_group_height.is_none_or(|height| height > future_capacity)
            && (b.height <= available
                || b.gap_start_height.is_some_and(|start| start <= available))
            && b.height < frame.height
            && b.item_count < frame.items.len()
            && b.deco_count <= frame.deco.len()
            && frame.orphaned_x.iter().all(|(path, count, ..)| {
                path[0] >= b.item_count || path[0] + count <= b.item_count
            })
            && frame.orphaned_y.iter().all(|(path, count, ..)| {
                path[0] >= b.item_count || path[0] + count <= b.item_count
            })
    })
}

impl<M: FontMetrics, S: ImageSizer> Layouter<'_, M, S> {
    pub(super) fn admit_footnote_block_frontier(&mut self, content: &Content) {
        if self.footnote_capture.is_none()
            && !self.is_sub_frame
            && !self.column_mode
            && self.page_columns.is_none()
            && self.footnote_reserved_height > 0.0
            && self.footnote_reserved_height
                >= (self.regions.current.height
                    - self.page_config.margin.bottom
                    - (self.regions.current.cursor_y.0 + self.line_inline_descent))
                    .max(0.0)
            && matches!(boundary_role(content), BoundaryRole::Atomic)
        {
            let frontier =
                (self.pages.len(), self.current_column, self.page_running_generation);
            self.flush_line();
            if frontier
                == (self.pages.len(), self.current_column, self.page_running_generation)
            {
                self.new_page();
            }
        }
    }

    pub(super) fn begin_footnote_atomic(&mut self, content: &Content) -> bool {
        if !self
            .footnote_capture
            .as_ref()
            .is_some_and(|c| c.enabled && c.atomic_depth == 0)
        {
            return false;
        }
        match boundary_role(content) {
            BoundaryRole::Transparent | BoundaryRole::Spacing => false,
            BoundaryRole::Uncertified => {
                if let Some(capture) = &mut self.footnote_capture {
                    capture.enabled = false;
                    capture.breaks.clear();
                }
                false
            }
            BoundaryRole::Atomic => {
                self.flush_line();
                self.record_footnote_break();
                if let Some(capture) = &mut self.footnote_capture {
                    capture.atomic_depth += 1;
                }
                true
            }
        }
    }

    pub(super) fn end_footnote_atomic(&mut self, started: bool) {
        if started {
            self.flush_line();
            if let Some(capture) = &mut self.footnote_capture {
                capture.atomic_depth -= 1;
            }
            self.record_footnote_break();
        }
    }

    pub(super) fn certify_footnote_tail_baseline(&mut self) {
        if self.footnote_capture.is_none() {
            return;
        }
        if self.style.superscript
            || self.style.subscript
            || self.style.math
            || super::html_frame::items_have_material(&self.regions.current.current_line)
        {
            return;
        }
        let top = self.regions.current.cursor_y.0
            - self.metrics.text_edges(self.style.size, &self.style).0 .0;
        let Some(capture) = &mut self.footnote_capture else {
            return;
        };
        if !capture.enabled || capture.atomic_depth != 0 {
            return;
        }
        let Some(boundary) = capture.breaks.last_mut() else {
            return;
        };
        let relative = top - capture.origin_y;
        if boundary.tail_start.is_none()
            && relative.is_finite()
            && relative >= boundary.height
        {
            boundary.tail_start = Some(relative);
        }
    }

    pub(super) fn record_footnote_break(&mut self) {
        let Some(capture) = &mut self.footnote_capture else { return };
        if !capture.enabled || capture.atomic_depth != 0 {
            return;
        }
        let end = self
            .flow_placement_end
            .map_or(self.regions.current.cursor_y.0, |(end, _)| end);
        let height = (end - capture.origin_y).max(0.0);
        let item_count = self.regions.current.current_items.len();
        if height > 0.0
            && item_count > 0
            && capture
                .breaks
                .last()
                .is_none_or(|b| b.item_count < item_count && b.height <= height)
        {
            capture.breaks.push(FootnoteBreak {
                protected_group_height: None,
                tail_start: None,
                item_count,
                height,
                gap_start_height: None,
                deco_count: self.decoration_lines_collector.as_ref().map_or(0, Vec::len),
            });
        }
    }

    pub(super) fn record_footnote_spacing(&mut self, content: &Content) {
        if !self
            .footnote_capture
            .as_ref()
            .is_some_and(|c| c.enabled && c.atomic_depth == 0)
            || !matches!(boundary_role(content), BoundaryRole::Spacing)
        {
            return;
        }
        // VSpace's own painter has already consumed its source amount. This
        // empty flush commits only that operation's source position carriers.
        self.flush_line();
        let Some(capture) = &mut self.footnote_capture else { return };
        let Some(previous) = capture.breaks.last() else { return };
        let height = self
            .flow_placement_end
            .map_or(self.regions.current.cursor_y.0, |(end, _)| end)
            - capture.origin_y;
        let gap_start_height = previous.gap_start_height.unwrap_or(previous.height);
        let item_count = self.regions.current.current_items.len();
        if height.is_finite()
            && height >= previous.height
            && item_count >= previous.item_count
        {
            capture.breaks.push(FootnoteBreak {
                protected_group_height: None,
                tail_start: None,
                item_count,
                height,
                gap_start_height: Some(gap_start_height),
                deco_count: self.decoration_lines_collector.as_ref().map_or(0, Vec::len),
            });
        }
    }

    pub(super) fn capture_footnote_frame(
        &mut self,
        prefix: &Content,
        body: &Content,
        planned: Option<super::page_running_requests::Cursor>,
    ) -> DeferredFootnoteFrame {
        let saved_style = self.style.clone();
        let saved_chain = self.chain.clone();
        let outer_size = self
            .footnote_capture
            .as_ref()
            .map(|c| c.outer_size)
            .unwrap_or_else(|| super::text::effective_style(self).size);
        self.style = super::text::effective_style(self);
        // ref: lab/typst-original/crates/typst-library/src/model/footnote.rs:337
        self.style.size = outer_size * 0.85;
        self.style.leading = Some(crate::entities::layout_types::Length::em(0.5));
        self.chain =
            self.chain.push_styles(&crate::entities::style::Styles::from_iter([
                crate::entities::style::Style::Size(
                    crate::entities::layout_types::Length::pt(self.style.size.0),
                ),
                crate::entities::style::Style::Leading(
                    crate::entities::layout_types::Length::em(0.5),
                ),
            ]));
        let mut profile = SourceLines { ink: true, ..SourceLines::default() };
        let certified = profile.scan(body, &self.chain);
        let combined = Content::sequence(vec![prefix.clone(), body.clone()]);
        let margin = self.page_config.margin;
        let width = if self.column_mode && self.page_columns.is_none() {
            self.page_config.width - margin.horizontal()
        } else {
            self.regions.current.width - margin.horizontal()
        };
        let ascender = self.metrics.vertical_metrics(self.style.size, &self.style).0 .0;
        let top = self.metrics.text_edges(self.style.size, &self.style).0 .0;
        let saved_capture = self.footnote_capture.take();
        self.footnote_capture = Some(super::FootnoteCapture {
            outer_size,
            breaks: Vec::new(),
            atomic_depth: 0,
            origin_y: ascender - top,
            enabled: true,
        });
        let (height, mut items, mut deco, orphaned_x, mut orphaned_y) = self
            .layout_sub_frame_footnote(
                &combined,
                super::sub_frame::SubLayoutRegion {
                    origin_x: margin.left,
                    width,
                    height: None,
                    align_rtl: true,
                    unconstrained_height: true,
                },
                prefix,
                planned,
            );
        let origin = ascender - top;
        for item in &mut items {
            super::helpers::offset_frame_item(item, 0.0, -origin);
        }
        for segment in &mut deco {
            segment.baseline_y =
                segment.baseline_y - crate::entities::layout_types::Pt(origin);
        }
        for (_, _, _, _, origin_y, _, applied_y) in &mut orphaned_y {
            *origin_y -= origin;
            *applied_y -= origin;
        }
        self.style = saved_style;
        self.chain = saved_chain;
        let capture = self.footnote_capture.take();
        self.footnote_capture = saved_capture;
        let breaks = capture.filter(|c| c.enabled).map_or_else(Vec::new, |c| {
            c.breaks
                .into_iter()
                .filter(|b| b.height < height && b.item_count < items.len())
                .collect()
        });
        let mut frame = DeferredFootnoteFrame {
            source_lines: false,
            line_continuation: false,
            deferred_region: None,
            outer_size,
            height,
            items,
            deco,
            orphaned_x,
            orphaned_y,
            breaks,
        };
        if certified {
            profile.certify(&mut frame, origin);
        }
        frame
    }

    pub(super) fn refresh_footnote_reservation(&mut self) {
        if self.is_sub_frame || self.column_mode || self.page_columns.is_some() {
            return;
        }
        let margin = self.page_config.margin;
        let bottom = self.regions.current.height - margin.bottom;
        let has_flow = super::html_frame::items_have_causal_line(
            &self.regions.current.current_items,
        ) || super::html_frame::items_have_causal_line(
            &self.regions.current.current_line,
        ) || self.flow_placement_end.is_some();
        let flow_bottom = if has_flow {
            self.regions.current.cursor_y.0 + self.line_inline_descent
        } else {
            margin.top
        };
        let available = (bottom - flow_bottom).max(0.0);
        let mut reserved = self
            .pending_footnote_frames
            .iter()
            .flatten()
            .next()
            .map_or(0.0, |f| f.outer_size.0);
        let future_capacity = self.regions.current.height - margin.vertical();
        for (index, frame) in self.pending_footnote_frames.iter().flatten().enumerate() {
            let previous = reserved;
            reserved += frame.outer_size.0 * 0.5;
            let remaining = (available - reserved).max(0.0);
            if frame.height <= remaining {
                reserved += frame.height;
            } else {
                reserved = if frame.line_continuation
                    || (!has_flow && frame.source_lines)
                    || cut_for(frame, remaining, future_capacity).is_some()
                {
                    available
                } else if index == 0 {
                    0.0
                } else {
                    previous
                };
                break;
            }
        }
        self.footnote_reserved_height = reserved;
    }
}

pub(super) fn split_frame(
    mut frame: DeferredFootnoteFrame,
    available: f64,
    future_capacity: f64,
) -> Result<(DeferredFootnoteFrame, DeferredFootnoteFrame), Box<DeferredFootnoteFrame>> {
    let candidate = cut_for(&frame, available, future_capacity).cloned();
    let Some(cut) = candidate else { return Err(Box::new(frame)) };
    let tail_origin = cut.tail_start.unwrap_or(cut.height);
    let mut tail_items = frame.items.split_off(cut.item_count);
    for item in &mut tail_items {
        super::helpers::offset_frame_item(item, 0.0, -tail_origin);
    }
    let mut head_x = Vec::new();
    let mut tail_x = Vec::new();
    for (mut path, count, align, width, origin, applied) in frame.orphaned_x {
        if path[0] >= cut.item_count {
            path[0] -= cut.item_count;
            tail_x.push((path, count, align, width, origin, applied));
        } else {
            head_x.push((path, count, align, width, origin, applied));
        }
    }
    let mut head_y = Vec::new();
    let mut tail_y = Vec::new();
    for (mut path, count, align, height, origin, dy, applied) in frame.orphaned_y {
        if path[0] >= cut.item_count {
            path[0] -= cut.item_count;
            tail_y.push((
                path,
                count,
                align,
                height,
                origin - tail_origin,
                dy,
                applied - tail_origin,
            ));
        } else {
            head_y.push((path, count, align, height, origin, dy, applied));
        }
    }
    let mut head_deco = frame.deco;
    let mut tail_deco = head_deco.split_off(cut.deco_count);
    for deco in &mut tail_deco {
        deco.baseline_y.0 -= tail_origin;
    }
    let tail_breaks = frame
        .breaks
        .into_iter()
        .filter(|b| b.item_count > cut.item_count && b.height > tail_origin)
        .map(|b| FootnoteBreak {
            protected_group_height: b.protected_group_height,
            tail_start: b.tail_start.map(|origin| origin - tail_origin),
            item_count: b.item_count - cut.item_count,
            height: b.height - tail_origin,
            gap_start_height: b
                .gap_start_height
                .map(|height| (height - tail_origin).max(0.0)),
            deco_count: b.deco_count - cut.deco_count,
        })
        .collect();
    let tail = DeferredFootnoteFrame {
        source_lines: frame.source_lines,
        line_continuation: frame.source_lines || frame.line_continuation,
        deferred_region: frame.deferred_region,
        outer_size: frame.outer_size,
        height: frame.height - tail_origin,
        items: tail_items,
        deco: tail_deco,
        orphaned_x: tail_x,
        orphaned_y: tail_y,
        breaks: tail_breaks,
    };
    let head = DeferredFootnoteFrame {
        source_lines: frame.source_lines,
        line_continuation: frame.line_continuation,
        deferred_region: frame.deferred_region,
        outer_size: frame.outer_size,
        height: cut.height.min(available),
        items: frame.items,
        deco: head_deco,
        orphaned_x: head_x,
        orphaned_y: head_y,
        breaks: Vec::new(),
    };
    Ok((head, tail))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{FrameItem, Pt},
    };
    use comemo::Track;

    fn text_baseline(items: &[FrameItem], needle: &str) -> Option<Pt> {
        items.iter().find_map(|item| match item {
            FrameItem::Text { pos, text, .. }
            | FrameItem::TextShaped { pos, text, .. } => {
                (text.as_str() == needle).then_some(pos.y)
            }
            FrameItem::Group { pos, items, .. } => {
                text_baseline(items, needle).map(|baseline| pos.y + baseline)
            }
            FrameItem::Link { items, .. }
            | FrameItem::TextReplacement { items, .. }
            | FrameItem::Semantic { items, .. } => text_baseline(items, needle),
            FrameItem::TextSeparator { .. }
            | FrameItem::Line { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Image { .. }
            | FrameItem::Shape { .. } => None,
        })
    }

    #[test]
    fn r1_footnote_capture_uses_superscript_entry_marker() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        let captured = l.capture_footnote_frame(
            &Content::sequence(vec![
                Content::superscript(Content::text("1")),
                Content::Space,
            ]),
            &Content::text("BODY"),
            None,
        );
        assert!(text_baseline(&captured.items, "1").is_some(), "{:?}", captured.items);
        assert!(text_baseline(&captured.items, "[1]").is_none());
        assert!(
            text_baseline(&captured.items, "1").unwrap()
                < text_baseline(&captured.items, "BODY").unwrap()
        );
    }

    #[test]
    fn physical_page_note_split_keeps_each_decoration_with_its_actual_line() {
        let intr = TagIntrospector::empty();
        let mut layouter = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        layouter.chain = layouter.chain.push_custom(
            "text.costs.orphan",
            crate::entities::value::Value::Ratio(crate::entities::layout_types::Ratio(
                0.0,
            )),
        );
        layouter.decoration_lines_collector = Some(Vec::new());
        let body = Content::sequence(vec![
            Content::text("ALPHA"),
            Content::linebreak(),
            Content::text("OMEGA"),
        ]);
        let captured = layouter.capture_footnote_frame(
            &Content::sequence(vec![
                Content::superscript(Content::text("1")),
                Content::Space,
            ]),
            &body,
            None,
        );
        let first_baseline = text_baseline(&captured.items, "ALPHA")
            .expect("the actual captured first line must contain ALPHA");
        let second_baseline = text_baseline(&captured.items, "OMEGA")
            .expect("the actual captured second line must contain OMEGA");
        assert!(second_baseline > first_baseline);
        assert_eq!(captured.deco.len(), 2, "{:?}", captured.deco);
        let original_first_decoration = captured.deco[0];
        let original_second_decoration = captured.deco[1];
        let boundary = captured
            .breaks
            .first()
            .expect("committing the first actual line must certify a boundary")
            .height;
        let (head, tail) = split_frame(captured, boundary, 1000.0)
            .expect("the actual first-line boundary must split the two lines");
        assert_eq!(text_baseline(&head.items, "ALPHA"), Some(first_baseline));
        assert!(text_baseline(&head.items, "OMEGA").is_none());
        assert!(text_baseline(&tail.items, "ALPHA").is_none());
        let tail_baseline = text_baseline(&tail.items, "OMEGA").unwrap();
        let tail_style = tail
            .items
            .iter()
            .find_map(|item| match item {
                FrameItem::Text { text, style, .. } if text == "OMEGA" => Some(style),
                _ => None,
            })
            .expect("tail must retain the actual OMEGA style");
        let expected_top =
            super::super::FixedMetrics.text_edges(tail_style.size, tail_style).0;
        assert!((tail_baseline.0 - expected_top.0).abs() < 1e-10);
        assert!(tail_baseline < second_baseline);
        assert_eq!(head.deco.len(), 1, "{:?}", head.deco);
        assert_eq!(tail.deco.len(), 1, "{:?}", tail.deco);
        assert_eq!(head.deco[0].start_x, original_first_decoration.start_x);
        assert_eq!(head.deco[0].end_x, original_first_decoration.end_x);
        assert_eq!(head.deco[0].baseline_y, original_first_decoration.baseline_y);
        assert_eq!(tail.deco[0].start_x, original_second_decoration.start_x);
        assert_eq!(tail.deco[0].end_x, original_second_decoration.end_x);
        let text_translation = second_baseline.0 - tail_baseline.0;
        assert!(
            (tail.deco[0].baseline_y.0
                - (original_second_decoration.baseline_y.0 - text_translation))
                .abs()
                < 1e-10
        );
    }
    fn r9_document(
        lines: usize,
        height: f64,
    ) -> crate::entities::layout_types::PagedDocument {
        r9_document_columns(lines, height, None)
    }

    fn r9_document_columns(
        lines: usize,
        height: f64,
        columns: Option<usize>,
    ) -> crate::entities::layout_types::PagedDocument {
        r9_document_following(lines, height, columns, false)
    }

    fn r9_document_following(
        lines: usize,
        height: f64,
        columns: Option<usize>,
        following: bool,
    ) -> crate::entities::layout_types::PagedDocument {
        use crate::entities::layout_types::{PageMarginSpec, PageMargins};
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = 300.0;
        l.regions.current.width = 300.0;
        l.page_config.height = height;
        l.regions.current.height = height;
        l.page_config.margin = PageMargins::uniform(20.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(20.0);
        l.regions.current.cursor_x = Pt(20.0);
        l.regions.current.line_start_x = Pt(20.0);
        l.regions.current.cursor_y = Pt(20.0);
        l.initial_baseline_pending = true;
        l.line_assumed_ascent = None;
        let mut body = Vec::new();
        for n in 1..=lines {
            if n > 1 {
                body.push(Content::linebreak());
            }
            body.push(Content::text(format!("LINE{n}")));
        }
        let mut whole = vec![
            Content::text("ALPHA"),
            Content::Footnote(std::sync::Arc::new(
                crate::entities::elements::footnote::FootnoteElem {
                    body: Content::sequence(body),
                    numbering: None,
                },
            )),
        ];
        if following {
            whole.push(Content::Parbreak);
            for n in 1..=8 {
                if n > 1 {
                    whole.push(Content::linebreak());
                }
                whole.push(Content::text(format!("TAIL{n}")));
            }
        }
        let body = Content::sequence(whole);
        if let Some(count) = columns {
            l.layout_content(&Content::Columns(std::sync::Arc::new(
                crate::entities::elements::columns::ColumnsElem {
                    count,
                    gutter: None,
                    body,
                    page_columns: true,
                },
            )));
        } else {
            l.layout_content(&body);
        }
        l.finish()
    }

    #[test]
    fn r9_long_note_never_loses_terminal_lines() {
        for count in [9, 12, 20] {
            let doc = r9_document(count, 80.0);
            for n in 1..=count {
                let word = format!("LINE{n}");
                assert_eq!(
                    doc.pages
                        .iter()
                        .filter(|p| text_baseline(&p.items, &word).is_some())
                        .count(),
                    1,
                    "missing or duplicated {word}: {doc:?}"
                );
            }
        }
    }

    #[test]
    fn r9_first_orphan_group_moves_together() {
        let doc = r9_document(5, 80.0);
        assert!(text_baseline(&doc.pages[0].items, "LINE1").is_none(), "{doc:?}");
        assert!(text_baseline(&doc.pages[1].items, "LINE1").is_some(), "{doc:?}");
        assert!(text_baseline(&doc.pages[1].items, "LINE2").is_some(), "{doc:?}");
        assert_eq!(doc.pages.len(), 3, "{doc:?}");
        assert!(text_baseline(&doc.pages[2].items, "LINE5").is_some());
    }
    #[test]
    fn r9_page_column_carrier_admits_before_source_dispatch() {
        let doc = r9_document_columns(2, 80.0, Some(2));
        let carrier_x = super::super::tests::frame_items_recursive(&doc.pages[0].items)
            .into_iter()
            .find_map(|item| match item {
                FrameItem::Text { pos, text, .. }
                | FrameItem::TextShaped { pos, text, .. }
                    if text.as_str() == "ALPHA" =>
                {
                    Some(pos.x.0)
                }
                _ => None,
            })
            .expect("actual carrier retained");
        assert!((carrier_x - 155.2).abs() < 1e-8, "{doc:?}");
        assert_eq!(doc.pages.len(), 2);
        assert!(text_baseline(&doc.pages[1].items, "LINE2").is_some());
    }

    #[test]
    fn r9_page_column_fitting_three_lines_stay_with_carrier() {
        let doc = r9_document_columns(3, 100.0, Some(2));
        assert_eq!(doc.pages.len(), 1, "{doc:?}");
        assert!(text_baseline(&doc.pages[0].items, "LINE3").is_some());
    }
    #[test]
    fn r9_following_prose_waits_for_occupied_note_regions() {
        let doc = r9_document_following(5, 80.0, None, true);
        let note_end = doc
            .pages
            .iter()
            .position(|p| text_baseline(&p.items, "LINE5").is_some())
            .expect("complete note");
        let prose_start = doc
            .pages
            .iter()
            .position(|p| text_baseline(&p.items, "TAIL1").is_some())
            .expect("complete following prose");
        assert!(prose_start > note_end, "{doc:?}");
        for n in 1..=8 {
            assert_eq!(
                doc.pages
                    .iter()
                    .filter(|p| text_baseline(&p.items, &format!("TAIL{n}")).is_some())
                    .count(),
                1
            );
        }
    }

    #[test]
    fn r9_first_source_line_has_one_ascender_to_edge_origin() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        let frame = l.capture_footnote_frame(
            &Content::Empty,
            &Content::sequence(vec![
                Content::text("ALPHA"),
                Content::linebreak(),
                Content::text("OMEGA"),
            ]),
            None,
        );
        let expected = super::super::FixedMetrics
            .text_edges(
                Pt(11.0 * 0.85),
                &crate::entities::layout_types::TextStyle::default(),
            )
            .0;
        assert!(
            (text_baseline(&frame.items, "ALPHA").unwrap().0 - expected.0).abs() < 1e-10,
            "{frame:?}"
        );
        assert!((frame.breaks[0].height - expected.0).abs() < 1e-10, "{frame:?}");
    }
    #[test]
    fn r9_single_source_line_preserves_manual_drain_origin() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        let frame =
            l.capture_footnote_frame(&Content::Empty, &Content::text("ALPHA"), None);
        let cap = super::super::FixedMetrics
            .text_edges(Pt(9.35), &crate::entities::layout_types::TextStyle::default())
            .0;
        assert!(
            (text_baseline(&frame.items, "ALPHA").unwrap().0 - cap.0).abs() < 1e-10,
            "{frame:?}"
        );
        assert!((frame.height - cap.0).abs() < 1e-10, "{frame:?}");
    }

    #[test]
    fn r9_source_position_carrier_does_not_consume_note_capacity() {
        use crate::entities::layout_types::{PageMarginSpec, PageMargins, Point};
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = 300.0;
        l.regions.current.width = 300.0;
        l.page_config.height = 100.0;
        l.regions.current.height = 100.0;
        l.page_config.margin = PageMargins::uniform(20.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(20.0);
        l.regions.current.cursor_x = Pt(20.0);
        l.regions.current.line_start_x = Pt(20.0);
        l.regions.current.cursor_y = Pt(20.0);
        l.initial_baseline_pending = true;
        l.line_assumed_ascent = None;
        let mut body = Vec::new();
        for n in 1..=8 {
            if n > 1 {
                body.push(Content::linebreak());
            }
            body.push(Content::text(format!("LINE{n}")));
        }
        l.layout_content(&Content::sequence(vec![
            Content::text("ALPHA"),
            Content::Footnote(std::sync::Arc::new(
                crate::entities::elements::footnote::FootnoteElem {
                    body: Content::sequence(body),
                    numbering: None,
                },
            )),
        ]));
        l.flush_line();
        let prepared = l.regions.current.cursor_y;
        l.regions
            .current
            .current_items
            .push(super::super::html_frame::context_marker(
                777,
                Point { x: Pt(20.0), y: prepared },
            ));
        let doc = l.finish();
        assert!(text_baseline(&doc.pages[0].items, "LINE3").is_some(), "{doc:?}");
        assert!(doc.pages.iter().any(|p| text_baseline(&p.items, "LINE8").is_some()));
    }
    #[test]
    fn r9_intermediate_prose_keeps_later_notes_stacked() {
        let note = |word| {
            Content::Footnote(std::sync::Arc::new(
                crate::entities::elements::footnote::FootnoteElem {
                    body: Content::text(word),
                    numbering: None,
                },
            ))
        };
        let doc = super::super::layout(&Content::sequence(vec![
            Content::text("ALPHA"),
            note("FIRST"),
            Content::Parbreak,
            Content::text("MIDDLE"),
            Content::Parbreak,
            Content::text("BETA"),
            note("SECOND"),
        ]));
        assert_eq!(doc.pages.len(), 1);
        let first = text_baseline(&doc.pages[0].items, "FIRST").unwrap();
        let second = text_baseline(&doc.pages[0].items, "SECOND").unwrap();
        assert!(second > first, "{doc:?}");
    }
}
