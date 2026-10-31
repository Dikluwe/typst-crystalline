//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/sequence/inline_allocation.md
//! @prompt-hash 2a40a279
//! @layer L1
//! @updated 2026-10-21
//!
//! Immutable scope assignments for certified inline fractional layout.

use super::super::{FontMetrics, ImageSizer, Layouter};
use crate::entities::{
    content::Content,
    dir::Dir,
    elements::h_space::Spacing,
    layout_types::TextStyle,
    style::{Style, Styles},
    style_chain::StyleChain,
    value::Value,
};
use std::sync::Arc;
use unicode_script::{Script, UnicodeScript};

#[derive(Debug, Clone)]
pub(in crate::compiler::layout) enum Node {
    Fraction { width: f64 },
    Sequence(Vec<Option<Arc<Node>>>),
    Blocked,
}

enum Draft {
    Fraction(usize),
    Sequence(Vec<Draft>),
    Blocked,
    Independent,
}

enum Occupancy {
    Fixed(f64),
    Weak(f64),
    Fraction(usize, f64, bool),
}

struct Certificate<'a, M> {
    metrics: &'a M,
    left: f64,
    line_left: f64,
    right: f64,
    valid: bool,
    scope_eligible: bool,
    wrap_scope_eligible: bool,
    wrap_eligible: bool,
    material: bool,
    leading_shifted: bool,
    weak_space: bool,
    direction: Option<Dir>,
    boxes: usize,
    occupancy: Vec<Occupancy>,
    widths: Vec<Option<f64>>,
}

impl<M: FontMetrics> Certificate<'_, M> {
    fn fixed(&mut self, width: f64) {
        if !width.is_finite() || width < 0.0 {
            self.valid = false;
        }
        self.occupancy.push(Occupancy::Fixed(width));
    }
    fn weak(&mut self, width: f64) {
        if !width.is_finite() || width < 0.0 {
            self.valid = false;
        }
        self.occupancy.push(Occupancy::Weak(width));
    }
    fn fraction(&mut self, weight: f64, boxed: bool) -> Draft {
        if !weight.is_finite() {
            self.valid = false;
        }
        if weight <= 0.0 {
            self.wrap_eligible = false;
        }
        let index = self.widths.len();
        self.widths.push(None);
        self.occupancy.push(Occupancy::Fraction(index, weight, boxed));
        self.boxes += usize::from(boxed);
        if boxed {
            self.material = true;
            self.weak_space = false;
        }
        Draft::Fraction(index)
    }
    fn line_assignments(
        occupancy: &[Occupancy],
        left: f64,
        right: f64,
    ) -> Option<(Vec<(usize, f64)>, f64)> {
        let fixed: f64 = occupancy
            .iter()
            .filter_map(|v| match v {
                Occupancy::Fixed(width) | Occupancy::Weak(width) => Some(*width),
                Occupancy::Fraction(..) => None,
            })
            .sum();
        let weights: f64 = occupancy
            .iter()
            .filter_map(|v| match v {
                Occupancy::Fraction(_, weight, _) => Some(*weight),
                Occupancy::Fixed(..) | Occupancy::Weak(..) => None,
            })
            .sum();
        let mut free = right - left - fixed;
        if !free.is_finite() || free < 0.0 || !weights.is_finite() {
            return None;
        }
        let share = |weight: f64, remaining: f64| {
            let ratio = weight / weights;
            if ratio.is_finite() {
                (ratio * remaining).max(0.0)
            } else {
                0.0
            }
        };
        let cursor_for = |remaining| {
            occupancy.iter().fold(left, |cursor, v| {
                cursor
                    + match v {
                        Occupancy::Fixed(width) | Occupancy::Weak(width) => *width,
                        Occupancy::Fraction(_, weight, _) => share(*weight, remaining),
                    }
            })
        };
        let mut final_cursor = cursor_for(free);
        // A rounded sum can exceed the edge by its immediate successor.
        // Try one smaller budget, then require the original strict
        // containment proof again. Signed/zero shares retain their old path.
        if final_cursor.is_finite()
            && final_cursor == right.next_up()
            && free > 0.0
            && weights > 0.0
            && occupancy.iter().all(|item| match item {
                Occupancy::Fraction(_, weight, _) => *weight > 0.0,
                Occupancy::Fixed(_) | Occupancy::Weak(_) => true,
            })
        {
            let smaller_free = free.next_down();
            let smaller_cursor = cursor_for(smaller_free);
            if smaller_cursor.is_finite() && smaller_cursor <= right {
                free = smaller_free;
                final_cursor = smaller_cursor;
            }
        }
        if !final_cursor.is_finite() || final_cursor > right {
            return None;
        }
        let assignments = occupancy
            .iter()
            .filter_map(|v| match v {
                Occupancy::Fraction(index, weight, _) => {
                    Some((*index, share(*weight, free)))
                }
                Occupancy::Fixed(..) | Occupancy::Weak(..) => None,
            })
            .collect();
        Some((assignments, final_cursor))
    }

    fn wrapping_assignments(&self) -> Option<Vec<(usize, f64)>> {
        if !self.wrap_eligible {
            return None;
        }
        let mut words = Vec::new();
        let mut start = 0;
        for (index, item) in self.occupancy.iter().enumerate() {
            if matches!(item, Occupancy::Weak(_)) {
                if start == index {
                    return None;
                }
                words.push((start, index));
                start = index + 1;
            }
        }
        if start >= self.occupancy.len() {
            return None;
        }
        words.push((start, self.occupancy.len()));
        let lines = super::super::paragraph_select::select_simple(
            words.len(),
            |first| self.right - if first { self.left } else { self.line_left },
            |line| {
                self.occupancy[words[line.start].0..words[line.end - 1].1]
                    .iter()
                    .map(|item| match item {
                        Occupancy::Fixed(width) | Occupancy::Weak(width) => *width,
                        // Fractional natural width is explicitly zero, never an
                        // estimate for an opaque item's unknown geometry.
                        Occupancy::Fraction(..) => 0.0,
                    })
                    .sum()
            },
        );
        if lines.len() <= 1 {
            return None;
        }
        let mut assignments = Vec::new();
        for line in lines {
            let range = words[line.start].0..words[line.end - 1].1;
            let occupancy = &self.occupancy[range.clone()];
            let left = if line.start == 0 { self.left } else { self.line_left };
            let (local, cursor) = Self::line_assignments(occupancy, left, self.right)?;
            if line.end < words.len() {
                if !occupancy.iter().any(|item| {
                    matches!(item,
                    Occupancy::Fraction(_, weight, true) if *weight > 0.0)
                }) {
                    return None;
                }
                let Occupancy::Weak(gap) = &self.occupancy[range.end] else {
                    return None;
                };
                // Prove the actual next weak dispatch closes this filled line,
                // including finite floating-point addition, without epsilon.
                let next = cursor + *gap;
                if *gap <= 0.0 || !next.is_finite() || next <= self.right {
                    return None;
                }
            }
            assignments.extend(local);
        }
        Some(assignments)
    }
    fn close(&mut self) {
        // Terminal weak spacing disappears from the budget; retained gaps
        // belong to their actual indivisible source words.
        while matches!(self.occupancy.last(), Some(Occupancy::Weak(_))) {
            self.occupancy.pop();
        }
        if self.valid && self.boxes > 0 {
            let assignments =
                Self::line_assignments(&self.occupancy, self.left, self.right)
                    .map(|(assignments, _)| assignments)
                    .or_else(|| self.wrapping_assignments());
            if let Some(assignments) = assignments {
                // Install only after the complete original scope has proved
                // every selected line, including its continuation.
                for (index, width) in assignments {
                    self.widths[index] = Some(width);
                }
            }
        }
        self.left = self.line_left;
        self.occupancy.clear();
        self.boxes = 0;
        self.direction = None;
        self.material = false;
        self.leading_shifted = false;
        self.weak_space = false;
        self.valid = self.scope_eligible;
        self.wrap_eligible = self.wrap_scope_eligible;
    }
    fn observe_direction(&mut self, style: &TextStyle) {
        let direction = style.dir.unwrap_or(Dir::LTR);
        if self.direction.is_some_and(|previous| previous != direction) {
            self.valid = false;
        }
        self.direction = Some(direction);
    }
    fn walk(
        &mut self,
        content: &Content,
        chain: &StyleChain,
        style: &TextStyle,
    ) -> Draft {
        if !safe_style(chain, style) {
            self.valid = false;
        }
        if !wrap_style(chain, style) {
            self.wrap_eligible = false;
        }
        match content {
            Content::Source(v) => self.walk(&v.body, chain, style),
            Content::Located(v) => self.walk(&v.body, chain, style),
            Content::Realized(v) => self.walk(&v.visual, chain, style),
            Content::Link(v) => self.walk(&v.body, chain, style),
            Content::Underline(v) => self.walk(&v.body, chain, style),
            Content::Sequence(parts) => Draft::Sequence(
                parts.iter().map(|part| self.walk(part, chain, style)).collect(),
            ),
            Content::Styled(v) => {
                // Only positively classified lexical fields and channels are
                // transparent; unknown custom scopes remain opaque.
                if !styles_supported(&v.styles, &v.body) {
                    self.valid = false;
                    return Draft::Blocked;
                }
                let next = chain.push_styles(&v.styles);
                let resolved = super::super::text::effective_style_in(
                    &next,
                    &TextStyle::from(&next),
                );
                self.walk(&v.body, &next, &resolved)
            }
            Content::Strong(v) => {
                let next = chain.push_styles(&Styles::from_iter([Style::strong()]));
                self.walk(
                    &v.body,
                    &next,
                    &super::super::text::effective_style_in(
                        &next,
                        &TextStyle::from(&next),
                    ),
                )
            }
            Content::Emph(v) => {
                let next = chain.push_styles(&Styles::from_iter([Style::emph()]));
                self.walk(
                    &v.body,
                    &next,
                    &super::super::text::effective_style_in(
                        &next,
                        &TextStyle::from(&next),
                    ),
                )
            }
            Content::Text(text) => {
                self.observe_direction(style);
                if style.dir == Some(Dir::RTL) && !text.is_ascii() {
                    self.valid = false;
                }
                let literal_words = text.contains(' ')
                    && matches!(style.dir, None | Some(Dir::LTR))
                    && matches!(
                        chain.custom("text.dir"),
                        None | Some(Value::Dir(Dir::LTR))
                    )
                    && text.split(' ').all(|word| {
                        !word.is_empty()
                            && word.chars().all(|ch| {
                                (ch.is_ascii()
                                    && !ch.is_ascii_control()
                                    && !ch.is_ascii_whitespace())
                                    || matches!(
                                        ch.script(),
                                        Script::Latin | Script::Inherited
                                    )
                            })
                    });
                if literal_words {
                    self.wrap_eligible = false;
                    let mut words = text.split(' ').peekable();
                    while let Some(word) = words.next() {
                        self.fixed(
                            super::super::text_measure::word(self.metrics, word, style).0,
                        );
                        if words.peek().is_some() {
                            self.fixed(
                                super::super::text_measure::space(self.metrics, style).0,
                            );
                        }
                    }
                } else {
                    if text.chars().any(char::is_whitespace)
                        || super::super::word_segment::word_needs_line_segmentation(text)
                    {
                        self.valid = false;
                    }
                    self.fixed(
                        super::super::text_measure::word(self.metrics, text, style).0,
                    );
                }
                self.material = true;
                self.weak_space = false;
                Draft::Blocked
            }
            Content::Space | Content::SpaceFromNewline => {
                if self.material && !self.weak_space {
                    self.weak(super::super::text_measure::space(self.metrics, style).0);
                    self.weak_space = true;
                }
                Draft::Blocked
            }
            Content::HSpace(v) => match v.amount {
                Spacing::Absolute(width) => {
                    self.wrap_eligible = false;
                    let width = width.resolve_pt(style.size.0);
                    let leading = self.valid
                        && width.is_finite()
                        && width < 0.0
                        && !v.weak
                        && !self.leading_shifted
                        && !self.material
                        && self.occupancy.is_empty()
                        && matches!(style.dir, None | Some(Dir::LTR))
                        && matches!(
                            chain.custom("text.dir"),
                            None | Some(Value::Dir(Dir::LTR))
                        );
                    if leading {
                        let left = self.left + width;
                        if left.is_finite() {
                            // The original H performs this same addition at dispatch.
                            // Its compensation affects this line, not its continuation.
                            self.observe_direction(style);
                            self.left = left;
                            self.leading_shifted = true;
                        } else {
                            self.valid = false;
                        }
                    } else {
                        self.fixed(width);
                    }
                    Draft::Blocked
                }
                Spacing::Fractional(weight) => self.fraction(weight, false),
            },
            Content::Boxed(v) => {
                self.observe_direction(style);
                if let Some(weight) = v.width_fraction {
                    self.fraction(weight, true)
                } else if let Some(width) = v.width {
                    self.fixed(width.resolve_pt(style.size.0));
                    self.material = true;
                    self.weak_space = false;
                    Draft::Blocked
                } else {
                    self.valid = false;
                    Draft::Blocked
                }
            }
            Content::Empty
            | Content::Metadata(_)
            | Content::State(_)
            | Content::StateUpdate(_)
            | Content::CounterUpdate(_)
            | Content::ContextBlock(_)
            | Content::Document { .. }
            | Content::Asset { .. } => Draft::Blocked,
            Content::SetPage(_) => {
                self.close();
                // Only actual dispatch can install the next page's geometry.
                // A later paragraph boundary cannot revive this stale budget.
                self.scope_eligible = false;
                self.wrap_scope_eligible = false;
                self.valid = false;
                Draft::Independent
            }
            Content::Parbreak
            | Content::Linebreak(_)
            | Content::Par { .. }
            | Content::Block(_)
            | Content::Shape(_)
            | Content::Repeat(_) => {
                self.close();
                Draft::Independent
            }
            Content::ShowRecipe(_)
            | Content::HtmlElem(_)
            | Content::HtmlFrame(_)
            | Content::Heading(_)
            | Content::Title(_)
            | Content::Raw(_)
            | Content::ListItem(_)
            | Content::EnumItem(_)
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
            | Content::Outline(_)
            | Content::OutlineEntry(_)
            | Content::Figure(_)
            | Content::FigureCaption(_)
            | Content::Image(_)
            | Content::Curve(_)
            | Content::Transform(_)
            | Content::Grid(_)
            | Content::GridHeader(_)
            | Content::GridFooter(_)
            | Content::GridCell(_)
            | Content::PageRun(_)
            | Content::Align(_)
            | Content::Place(_)
            | Content::Flush(_)
            | Content::Divider(_)
            | Content::Terms(_)
            | Content::TermItem(_)
            | Content::Quote(_)
            | Content::SmartQuote(_)
            | Content::PdfAttach(_)
            | Content::PdfArtifact(_)
            | Content::Strike(_)
            | Content::Overline(_)
            | Content::SmallCaps { .. }
            | Content::Pad(_)
            | Content::Hide(_)
            | Content::VSpace(_)
            | Content::Pagebreak(_)
            | Content::Colbreak(_)
            | Content::Stack(_)
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
            | Content::Columns(_)
            | Content::StateDisplay(_)
            | Content::CounterDisplayCallback(_)
            | Content::Dynamic(_) => {
                self.valid = false;
                Draft::Blocked
            }
        }
    }
}

// Reconstruct the complete admitted delta, so a new or unsupported field
// cannot silently become an inert channel. The same bounded lexical class
// already has a positive certificate in paragraph preparation.
fn styles_supported(styles: &Styles, body: &Content) -> bool {
    let d = styles.delta();
    let mut permitted = crate::entities::style_chain::StyleDelta::empty();
    permitted.bold = d.bold;
    permitted.bold_from_strong = d.bold_from_strong;
    permitted.italic = d.italic;
    permitted.italic_from_emph = d.italic_from_emph;
    permitted.size = d.size;
    permitted.fill = d.fill.clone();
    permitted.weight = d.weight;
    permitted.lang = d.lang;
    permitted.font = d.font.clone();
    for (key, value) in &d.custom {
        let outline_marker = match key.as_str() {
            "pdf.marker.tag" => {
                matches!(body, Content::Text(_))
                    && matches!(value, Value::Str(tag) if tag.as_str() == "label")
            }
            "outline.prefix-info" => {
                matches!(body, Content::Empty) && inert_prefix_info(value)
            }
            _ => false,
        };
        if !outline_marker
            && !matches!(
                key.as_str(),
                "text.size"
                    | "text.fill"
                    | "text.bold"
                    | "text.italic"
                    | "text.weight"
                    | "text.style"
                    | "text.font"
                    | "text.lang"
                    | "text.dir"
                    | "text.hyphenate"
                    | "text.overhang"
                    | "eval.flow.scope"
                    | "page.width"
                    | "page.height"
                    | "page.margin-top"
                    | "page.margin-right"
                    | "page.margin-bottom"
                    | "page.margin-left"
            )
            && !crate::compiler::text_costs::is_channel(key.as_str())
        {
            return false;
        }
        permitted.custom.push((key.clone(), value.clone()));
    }
    *d == permitted
}

fn inert_prefix_info(value: &Value) -> bool {
    let Value::Array(parts) = value else { return false };
    let [Value::Int(_), Value::Int(level), Value::Length(inset)] = parts.as_slice()
    else {
        return false;
    };
    let absolute = inset.abs.to_pt();
    *level > 0 && inset.em == 0.0 && absolute.is_finite() && absolute >= 0.0
}

fn safe_style(chain: &StyleChain, style: &TextStyle) -> bool {
    matches!(style.dir, None | Some(Dir::LTR | Dir::RTL))
        && matches!(
            chain.custom("text.dir"),
            None | Some(Value::Dir(Dir::LTR | Dir::RTL))
        )
        && style.tracking.is_none()
        && !style.subscript
        && !style.superscript
        && !style.math
        && !matches!(chain.custom("par.justify"), Some(Value::Bool(true)))
        && chain.custom("raw.defaults.active").is_none()
        && chain.custom("raw.block.active").is_none()
        && chain.custom("par.hanging-indent").is_none()
        && chain.custom("par.first-line-indent").is_none()
}

fn wrap_style(chain: &StyleChain, style: &TextStyle) -> bool {
    matches!(style.dir, None | Some(Dir::LTR))
        && matches!(chain.custom("par.justify"), None | Some(Value::Bool(false)))
        && matches!(
            chain.custom("text.hyphenate"),
            None | Some(Value::Auto | Value::Bool(false))
        )
        && match chain.custom("par.linebreaks") {
            None | Some(Value::Auto) => true,
            Some(Value::Str(strategy)) => strategy.as_str() == "simple",
            _ => false,
        }
}

fn finish(draft: Draft, widths: &[Option<f64>]) -> Option<Arc<Node>> {
    Some(Arc::new(match draft {
        Draft::Fraction(index) => match widths[index] {
            Some(width) => Node::Fraction { width },
            None => Node::Blocked,
        },
        Draft::Sequence(children) => Node::Sequence(
            children.into_iter().map(|child| finish(child, widths)).collect(),
        ),
        Draft::Blocked => Node::Blocked,
        Draft::Independent => return None,
    }))
}

pub(super) fn prepare<M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
    parts: &[Content],
) -> Arc<Node> {
    let left = super::super::paragraph_indent::cursor_start(l).0;
    let right = super::super::paragraph_indent::bounds(l).1;
    let style = super::super::text::effective_style(l);
    let scope_eligible =
        !l.smallcaps && left.is_finite() && right.is_finite() && right >= left;
    let valid = scope_eligible
        && !super::super::html_frame::items_have_causal_line(
            &l.regions.current.current_line,
        )
        && l.regions.current.pending_fr.is_empty()
        && l.pending_repeats.is_empty();
    let wrap_scope_eligible = scope_eligible
        && !l.column_mode
        && l.page_columns.is_none()
        && l.regions.cell.is_none()
        && !l.is_sub_frame
        && l.bidi_cursor.is_none()
        && l.floats_pending.is_empty()
        && !l.flow_suffix_checkpoint_pending
        && !l.flow_suffix_transaction_active
        && l.flow_suffix_checkpoint.is_none();
    let mut certificate = Certificate {
        metrics: &l.metrics,
        left,
        line_left: super::super::paragraph_indent::bounds(l).0,
        right,
        valid,
        scope_eligible,
        wrap_scope_eligible,
        wrap_eligible: valid && wrap_scope_eligible,
        material: false,
        leading_shifted: false,
        weak_space: false,
        direction: None,
        boxes: 0,
        occupancy: Vec::new(),
        widths: Vec::new(),
    };
    let draft = Draft::Sequence(
        parts
            .iter()
            .map(|part| certificate.walk(part, &l.chain, &style))
            .collect(),
    );
    certificate.close();
    finish(draft, &certificate.widths)
        .expect("a sequence always owns its assignment scope")
}

pub(in crate::compiler::layout) fn assigned(node: Option<Arc<Node>>) -> Option<f64> {
    match node.as_deref() {
        Some(Node::Fraction { width }) => Some(*width),
        None | Some(Node::Sequence(_)) | Some(Node::Blocked) => None,
    }
}

pub(super) fn independent(content: &Content) -> bool {
    match content {
        Content::Source(v) => independent(&v.body),
        Content::Located(v) => independent(&v.body),
        Content::Realized(v) => independent(&v.visual),
        Content::Styled(v) => independent(&v.body),
        Content::Parbreak
        | Content::Linebreak(_)
        | Content::Par { .. }
        | Content::Block(_)
        | Content::Shape(_)
        | Content::Repeat(_) => true,
        Content::ShowRecipe(_)
        | Content::Empty
        | Content::Text(_)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Sequence(_)
        | Content::HtmlElem(_)
        | Content::HtmlFrame(_)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{
        corners::Corners,
        elements::{boxed::BoxedElem, h_space::HSpaceElem},
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{Length, PageMarginSpec, PageMargins, Pt},
        sides::Sides,
    };
    use comemo::Track;

    struct DeclaredWidths;
    impl FontMetrics for DeclaredWidths {
        fn advance(&self, text: &str, _: Pt, _: &TextStyle) -> Pt {
            Pt(text.chars().count() as f64 * 10.0)
        }
        fn text_width(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
            self.advance(text, size, style)
        }
        fn vertical_metrics(&self, _: Pt, _: &TextStyle) -> (Pt, Pt) {
            (Pt(7.0), Pt(10.0))
        }
        fn cap_height(&self, _: Pt, _: &TextStyle) -> Pt {
            Pt(7.0)
        }
        fn text_edges(&self, _: Pt, _: &TextStyle) -> (Pt, Pt) {
            (Pt(7.0), Pt(-3.0))
        }
    }

    fn boxed(body: Content, width: Option<f64>, weight: Option<f64>) -> Content {
        Content::Boxed(Arc::new(BoxedElem {
            body,
            width: width.map(Length::pt),
            width_fraction: weight,
            height: None,
            inset: Sides::uniform(Length::ZERO),
            baseline: Length::ZERO,
            outset: Sides::uniform(Length::ZERO),
            radius: Corners::uniform(Length::ZERO),
            clip: false,
            fill: None,
            stroke: None,
        }))
    }
    fn fraction(weight: f64) -> Content {
        // Internal whitespace is deliberately uncertifiable if inspected.
        // An exterior fractional assignment does not inspect its body.
        boxed(Content::text("opaque body with whitespace"), None, Some(weight))
    }
    fn scope(parts: &[Content], width: f64) -> Arc<Node> {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            DeclaredWidths,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = width;
        l.page_config.margin = PageMargins::uniform(0.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(0.0);
        l.regions.current.width = width;
        l.regions.current.line_start_x = Pt(0.0);
        l.regions.current.cursor_x = Pt(0.0);
        prepare(&l, parts)
    }
    fn child(node: &Arc<Node>, index: usize) -> Option<Arc<Node>> {
        let Node::Sequence(children) = node.as_ref() else {
            panic!("complete source scope must retain its child routes");
        };
        children[index].clone()
    }

    fn prefix_marker(body: Content, key: &str, value: Value) -> Content {
        Content::styled(body, Styles::new().push_custom(key, value))
    }

    #[test]
    fn readiness_outline_prefix_label_preserves_its_known_fixed_advance() {
        let marker = prefix_marker(
            Content::text("1."),
            "pdf.marker.tag",
            Value::Str("label".into()),
        );
        let plan =
            scope(&[marker, Content::Space, fraction(1.0), Content::text("1")], 100.0);
        assert_eq!(assigned(child(&plan, 2)), Some(60.0));
    }

    #[test]
    fn readiness_outline_prefix_empty_info_keeps_the_original_budget() {
        let marker = prefix_marker(
            Content::Empty,
            "outline.prefix-info",
            Value::Array(vec![
                Value::Int(17),
                Value::Int(1),
                Value::Length(Length::pt(25.0)),
            ]),
        );
        let plan = scope(
            &[marker, Content::text("A"), fraction(1.0), Content::text("B")],
            100.0,
        );
        assert_eq!(assigned(child(&plan, 2)), Some(80.0));
    }

    #[test]
    fn readiness_outline_prefix_leading_compensation_keeps_next_line_origin() {
        let plan = scope(
            &[
                Content::h_space(Length::pt(-10.0), false),
                Content::text("A"),
                fraction(1.0),
                Content::text("B"),
                Content::Parbreak,
                Content::text("A"),
                fraction(1.0),
                Content::text("B"),
            ],
            100.0,
        );
        assert_eq!(assigned(child(&plan, 2)), Some(90.0));
        assert_eq!(
            assigned(child(&plan, 6)),
            Some(80.0),
            "leading compensation leaked into the next scope"
        );
    }

    #[test]
    fn readiness_outline_prefix_unproved_markers_and_signed_advances_stay_opaque() {
        let invalid = [
            prefix_marker(Content::text("1."), "pdf.marker.tag", Value::Int(1)),
            prefix_marker(
                Content::text("1."),
                "pdf.marker.tag",
                Value::Str("other".into()),
            ),
            prefix_marker(fraction(1.0), "pdf.marker.tag", Value::Str("label".into())),
            prefix_marker(
                Content::text("1."),
                "outline.prefix-info",
                Value::Array(vec![
                    Value::Int(17),
                    Value::Int(1),
                    Value::Length(Length::pt(25.0)),
                ]),
            ),
            prefix_marker(
                Content::Empty,
                "outline.prefix-info",
                Value::Array(vec![
                    Value::Int(17),
                    Value::Int(0),
                    Value::Length(Length::pt(25.0)),
                ]),
            ),
            prefix_marker(
                Content::Empty,
                "outline.prefix-info",
                Value::Array(vec![
                    Value::Int(17),
                    Value::Int(1),
                    Value::Length(Length::em(1.0)),
                ]),
            ),
            prefix_marker(
                Content::Empty,
                "outline.prefix-info",
                Value::Array(vec![
                    Value::Int(17),
                    Value::Int(1),
                    Value::Length(Length::pt(f64::NAN)),
                ]),
            ),
            prefix_marker(Content::Empty, "outline.unknown", Value::Bool(true)),
        ];
        for marker in invalid {
            let plan = scope(
                &[marker, Content::text("A"), fraction(1.0), Content::text("B")],
                100.0,
            );
            assert_eq!(assigned(child(&plan, 2)), None);
        }
        for parts in [
            vec![
                Content::text("A"),
                Content::h_space(Length::pt(-10.0), false),
                fraction(1.0),
                Content::text("B"),
            ],
            vec![
                Content::h_space(Length::pt(-10.0), true),
                Content::text("A"),
                fraction(1.0),
                Content::text("B"),
            ],
            vec![
                Content::h_space(Length::pt(-10.0), false),
                Content::h_space(Length::pt(-10.0), false),
                fraction(1.0),
                Content::text("B"),
            ],
            vec![
                Content::h_space(Length::pt(f64::NEG_INFINITY), false),
                Content::text("A"),
                fraction(1.0),
                Content::text("B"),
            ],
        ] {
            let plan = scope(&parts, 100.0);
            assert_eq!(assigned(child(&plan, 2)), None);
        }
        let rtl = Content::styled(
            Content::sequence(vec![
                Content::h_space(Length::pt(-10.0), false),
                Content::text("A"),
                fraction(1.0),
                Content::text("B"),
            ]),
            Styles::new().push_custom("text.dir", Value::Dir(Dir::RTL)),
        );
        let plan = scope(&[rtl], 100.0);
        let inner = child(&plan, 0).expect("original RTL sequence route");
        assert_eq!(
            assigned(child(&inner, 2)),
            None,
            "LTR compensation proof cannot admit RTL"
        );
    }

    #[test]
    fn readiness_outline_prefix_compensation_refuses_later_rtl() {
        let later_rtl = Content::styled(
            Content::sequence(vec![
                Content::text("A"),
                fraction(1.0),
                Content::text("B"),
            ]),
            Styles::new().push_custom("text.dir", Value::Dir(Dir::RTL)),
        );
        let plan = scope(&[Content::h_space(Length::pt(-10.0), false), later_rtl], 100.0);
        let inner = child(&plan, 1).expect("original direction scope");
        assert_eq!(
            assigned(child(&inner, 1)),
            None,
            "a leading LTR origin proof cannot certify later RTL material"
        );
    }

    #[test]
    fn readiness_fraction_inset_observed_positive_budget_has_a_strict_certificate() {
        let occupancy = [
            Occupancy::Fixed(f64::from_bits(0x4033e353f7ced917)),
            Occupancy::Weak(2.75),
            Occupancy::Fraction(0, 1.0, true),
            Occupancy::Weak(2.75),
            Occupancy::Fixed(f64::from_bits(0x401475c28f5c28f6)),
        ];
        let (assignments, cursor) =
            Certificate::<DeclaredWidths>::line_assignments(&occupancy, 33.2, 280.0)
                .expect("observed inset Repeat must receive a contained final share");
        assert_eq!(assignments.len(), 1);
        assert_eq!(assignments[0].0, 0);
        assert!((assignments[0].1 - 216.297).abs() < 1e-10);
        assert!(cursor.is_finite() && cursor <= 280.0);
        assert!((cursor - 280.0).abs() < 1e-10);

        let (normal, cursor) =
            Certificate::<DeclaredWidths>::line_assignments(&occupancy, 20.0, 280.0)
                .expect("original zero-inset certificate stays valid");
        assert!((normal[0].1 - 229.497).abs() < 1e-10);
        assert!(cursor <= 280.0);
        for width in [300.0, f64::INFINITY, f64::NAN] {
            assert!(
                Certificate::<DeclaredWidths>::line_assignments(
                    &[Occupancy::Fixed(width), Occupancy::Fraction(0, 1.0, true)],
                    20.0,
                    280.0,
                )
                .is_none(),
                "uncertified fixed width cannot become a share: {width}"
            );
        }
    }

    #[test]
    fn unknown_width_refuses_without_erasing_a_certified_zero_budget() {
        let unknown =
            scope(&[fraction(1.0), boxed(Content::text("UNKNOWN"), None, None)], 100.0);
        assert!(matches!(child(&unknown, 0).as_deref(), Some(Node::Blocked)));
        assert_eq!(assigned(child(&unknown, 0)), None);

        let exhausted = scope(
            &[Content::text("PREFIX"), fraction(1.0), Content::text("TAIL")],
            100.0,
        );
        assert_eq!(assigned(child(&exhausted, 1)), Some(0.0));
        assert_eq!(assigned(None), None);
        for weight in [0.0, -0.0, -1.0] {
            let signed = scope(&[fraction(weight)], 100.0);
            let expected = if weight == 0.0 { 0.0 } else { 100.0 };
            assert_eq!(assigned(child(&signed, 0)), Some(expected));
        }
        let cancelling = scope(&[fraction(1.0), fraction(-1.0)], 100.0);
        assert_eq!(assigned(child(&cancelling, 0)), Some(0.0));
        assert_eq!(assigned(child(&cancelling, 1)), Some(0.0));
        for weight in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let invalid = scope(&[fraction(weight)], 100.0);
            assert_eq!(assigned(child(&invalid, 0)), None);
        }
    }

    #[test]
    fn terminal_weak_space_is_trimmed_but_a_gap_before_fixed_spacing_is_kept() {
        let terminal = scope(
            &[
                Content::text("A"),
                fraction(1.0),
                Content::text("TAIL"),
                Content::SpaceFromNewline,
            ],
            100.0,
        );
        assert_eq!(assigned(child(&terminal, 1)), Some(50.0));
        let followed = scope(
            &[
                Content::text("A"),
                fraction(1.0),
                Content::text("TAIL"),
                Content::SpaceFromNewline,
                Content::HSpace(Arc::new(HSpaceElem {
                    amount: Spacing::Absolute(Length::pt(10.0)),
                    weak: false,
                    weak_explicit: true,
                })),
            ],
            100.0,
        );
        // A + TAIL = 50pt, retained weak gap = 10pt, explicit H = 10pt.
        assert_eq!(assigned(child(&followed, 1)), Some(30.0));
    }

    #[test]
    fn nested_preparation_uses_its_local_region_without_changing_exterior_shares() {
        let exterior = scope(
            &[
                Content::text("P"),
                fraction(1.0),
                Content::text("M"),
                fraction(2.0),
                Content::text("Q"),
            ],
            150.0,
        );
        assert_eq!(assigned(child(&exterior, 1)), Some(40.0));
        assert_eq!(assigned(child(&exterior, 3)), Some(80.0));
        let local_width = assigned(child(&exterior, 1)).expect("known exterior share");
        let interior =
            scope(&[Content::text("A"), fraction(1.0), Content::text("B")], local_width);
        assert_eq!(assigned(child(&interior, 1)), Some(20.0));
        assert_eq!(assigned(child(&exterior, 1)), Some(40.0));
        assert_eq!(assigned(child(&exterior, 3)), Some(80.0));
    }
}
