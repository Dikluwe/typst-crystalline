//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/footnote.md
//! @prompt-hash 7d0175d4
//! @layer L1
//! @updated 2026-10-24
//!
//! Atomização (ADR-0109, P381): o layout de `Footnote` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.
//! Estado dedicado (`pending_footnote_bodies`) acedido por descendência de
//! módulo — sem extracção.
//! **P1020** — marcador inline formatado por `format_pattern` e envolvido em
//! `Content::superscript`, sem colchetes literais.
//!
//! **P1016** — o número deixou de vir de `Layouter::footnote_counter` (campo
//! local, removido) e passa a vir do introspector, como no P461 para `Table`.
//! É o que torna `counter(footnote)` visível à língua.

use crate::entities::content::Content;
use crate::entities::counter::CounterKey;
use crate::entities::counter_format::format_counter;
use crate::entities::element_kind::ElementKind;
use crate::entities::elements::footnote::FootnoteElem;
use crate::entities::introspector::Introspector;
use crate::entities::selector::Selector;

use super::{FontMetrics, ImageSizer, Layouter};

#[derive(Debug, Clone)]
pub(super) struct EntrySettings {
    pub(super) indent: f64,
    pub(super) clearance: f64,
    pub(super) gap: f64,
    pub(super) separator: Option<Content>,
    pub(super) separator_height: Option<f64>,
    pub(super) chain: crate::entities::style_chain::StyleChain,
    pub(super) style: crate::entities::layout_types::TextStyle,
}

// Only the common source prefix establishes flow settings. Local note/call
// wrappers retain their ordinary styles without replacing this shared scope.
pub(super) fn prepare_entry_scope<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    content: &Content,
) {
    if l.footnote_entry_scope.is_some() || l.footnote_capture.is_some() {
        return;
    }
    fn weak(c: &Content) -> bool {
        matches!(
            c.source_body(),
            Content::Empty
                | Content::Space
                | Content::SpaceFromNewline
                | Content::Parbreak
                | Content::SetPage(_)
        )
    }
    let mut source = content;
    let mut chain = l.chain.clone();
    loop {
        if let Content::Source(e) = source {
            source = &e.body;
            continue;
        }
        if let Content::Located(e) = source {
            source = &e.body;
            continue;
        }
        if let Content::Styled(e) = source {
            chain = chain.push_styles(&e.styles);
            source = &e.body;
            continue;
        }
        if let Content::Sequence(parts) = source {
            let mut material = parts
                .iter()
                .take_while(|c| !matches!(c.source_body(), Content::Pagebreak(_)))
                .filter(|c| !weak(c));
            if let Some(first) = material.next() {
                if material.next().is_none() {
                    source = first;
                    continue;
                }
            } else {
                return;
            }
        }
        if weak(source) {
            return;
        }
        break;
    }
    l.footnote_entry_scope = Some(chain);
}

fn pure_separator(content: &Content) -> bool {
    if admission_text(content) || matches!(content, Content::Shape(_)) {
        return true;
    }
    if let Content::Styled(e) = content {
        return pure_separator(&e.body);
    }
    if let Content::Source(e) = content {
        return pure_separator(&e.body);
    }
    if let Content::Located(e) = content {
        return pure_separator(&e.body);
    }
    if let Content::Sequence(parts) = content {
        return parts.iter().all(pure_separator);
    }
    false
}

pub(super) fn entry_settings<M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
) -> EntrySettings {
    use crate::entities::{layout_types::TextStyle, value::Value};
    let chain = l.footnote_entry_scope.as_ref().unwrap_or(&l.chain).clone();
    let mut style = TextStyle::from(&chain);
    let outer = l.footnote_capture.as_ref().map_or(style.size.0, |c| c.outer_size.0);
    style.size = crate::entities::layout_types::Pt(outer);
    let length = |key, default, size| match chain.custom(key) {
        Some(Value::Length(length)) => length.resolve_pt(size),
        _ => default,
    };
    let separator = match chain.custom("footnote.entry.separator") {
        Some(Value::None) => Some(Content::Empty),
        Some(Value::Content(content)) => Some(content.clone()),
        Some(Value::Str(text)) => Some(Content::text(text.as_str())),
        _ => None,
    };
    let separator_height = if let Some(content) = &separator {
        if pure_separator(content) {
            let metrics: &dyn FontMetrics = &l.metrics;
            let mut probe = Layouter::new(
                metrics,
                crate::entities::image_sizer::NullImageSizer,
                outer,
                l.introspector,
            );
            probe.chain = chain.clone();
            probe.style = style.clone();
            probe.footnote_entry_scope = Some(chain.clone());
            probe.page_config = l.page_config.clone();
            let width = l.regions.current.width - l.page_config.margin.horizontal();
            let (height, ..) = probe.layout_sub_frame_at_geometric_origin(
                content,
                super::sub_frame::SubLayoutRegion {
                    origin_x: l.page_config.margin.left,
                    width,
                    height: None,
                    align_rtl: true,
                    unconstrained_height: true,
                },
            );
            Some(height)
        } else {
            None
        }
    } else {
        Some(0.0)
    };
    EntrySettings {
        indent: length("footnote.entry.indent", outer * 0.85, outer * 0.85),
        clearance: length("footnote.entry.clearance", outer, outer),
        gap: length("footnote.entry.gap", outer * 0.5, outer),
        separator,
        separator_height,
        chain,
        style,
    }
}

/// Layout de `footnote(...)`: emite o marcador `N` em superscript inline e
/// difere o body para o rodapé via `pending_footnote_bodies`
/// (flush em new_page/finish).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &FootnoteElem,
) {
    // **P1016** — número via introspector (counter `Selector(Kind(Footnote))`),
    // fonte única. `current_location` foi actualizado no topo de
    // `layout_content` porque Footnote é locatable desde este passo.
    let footnote_key = CounterKey::Selector(Selector::Kind(ElementKind::Footnote));
    let n = layouter
        .current_location
        .and_then(|loc| layouter.introspector.flat_counter_at(&footnote_key, loc))
        .unwrap_or(1);
    // **P1020** — marcador formatado pelo pattern `e.numbering` (default "1")
    // e envolvido em superscript, em vez do literal `[N]`.
    let pattern = e.numbering.as_deref().unwrap_or("1");
    let marker_text = format_counter(&[n], pattern).unwrap_or_else(|| n.to_string());
    let marker = Content::superscript(Content::text(marker_text));
    layouter.layout_content(&marker);
    let outer = layouter.footnote_capture.as_ref().map_or_else(
        || super::text::effective_style(layouter).size.0,
        |capture| capture.outer_size.0,
    );
    let prefix = entry_prefix(marker, outer, entry_settings(layouter).indent);
    let planned = layouter.planned_child_cursor(
        crate::compiler::introspect::page_running::PlanEdge::FootnoteBody,
    );
    let planned = layouter.defer_planned_cursor(planned);
    layouter
        .pending_footnote_plans
        .resize(layouter.pending_footnote_bodies.len(), None);
    layouter.pending_footnote_bodies.push((
        n as u32,
        Box::new(e.body.clone()),
        prefix.clone(),
    ));
    layouter.pending_footnote_plans.push(planned.clone());
    let index = layouter.pending_footnote_bodies.len() - 1;
    layouter.pending_footnote_frames.resize(index + 1, None);
    let frame = layouter.capture_footnote_frame(&prefix, &e.body, planned);
    layouter.pending_footnote_frames[index] = Some(frame);
    layouter.refresh_footnote_reservation();
}

fn entry_prefix(marker: Content, outer: f64, indent: f64) -> Content {
    let entry_size = outer * 0.85;
    let spacing = |amount| {
        Content::HSpace(std::sync::Arc::new(
            crate::entities::elements::h_space::HSpaceElem {
                amount: crate::entities::elements::h_space::Spacing::Absolute(
                    crate::entities::layout_types::Length::pt(amount),
                ),
                weak: false,
                weak_explicit: false,
            },
        ))
    };
    Content::sequence(vec![spacing(indent), marker, spacing(entry_size * 0.05)])
}

/// A deliberately finite, effect-free note body certificate for admission.
fn admission_text(content: &Content) -> bool {
    if matches!(
        content,
        Content::Text(_)
            | Content::Empty
            | Content::Space
            | Content::SpaceFromNewline
            | Content::Linebreak(_)
    ) {
        return true;
    }
    if let Content::Source(e) = content {
        return admission_text(&e.body);
    }
    if let Content::Located(e) = content {
        return admission_text(&e.body);
    }
    if let Content::Sequence(parts) = content {
        return parts.iter().all(admission_text);
    }
    false
}

fn unwrapped(
    mut content: &Content,
    mut location: Option<crate::entities::location::Location>,
) -> (&Content, Option<crate::entities::location::Location>) {
    loop {
        if let Content::Source(e) = content {
            content = &e.body;
        } else if let Content::Located(e) = content {
            location = Some(e.location);
            content = &e.body;
        } else {
            return (content, location);
        }
    }
}

fn prose_lines(
    content: &Content,
    metrics: &dyn FontMetrics,
    style: &crate::entities::layout_types::TextStyle,
    width: f64,
    state: &mut (usize, f64, bool),
) -> bool {
    match content {
        Content::Source(e) => prose_lines(&e.body, metrics, style, width, state),
        Content::Located(e) => prose_lines(&e.body, metrics, style, width, state),
        Content::Sequence(parts) => {
            parts.iter().all(|p| prose_lines(p, metrics, style, width, state))
        }
        Content::Text(text) => {
            if !text.is_ascii() || text.chars().any(char::is_control) {
                return false;
            }
            state.1 += metrics.text_width(text, style.size, style).0;
            state.2 |= text.chars().any(|c| !c.is_whitespace());
            state.1.is_finite() && state.1 <= width
        }
        Content::Space | Content::SpaceFromNewline => {
            state.1 += metrics.text_width(" ", style.size, style).0;
            state.1.is_finite() && state.1 <= width
        }
        Content::Empty => true,
        Content::Linebreak(_) => {
            if !state.2 {
                return false;
            }
            state.0 += 1;
            state.1 = 0.0;
            state.2 = false;
            true
        }
        Content::Styled(_)
        | Content::ShowRecipe(_)
        | Content::Realized(_)
        | Content::Strong(_)
        | Content::Emph(_)
        | Content::Figure(_)
        | Content::Block(_)
        | Content::VSpace(_)
        | Content::Parbreak
        | Content::Label(_)
        | Content::Metadata(_)
        | Content::State(_)
        | Content::StateUpdate(_)
        | Content::CounterUpdate(_)
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

/// Admit the original following paragraph before any of its source children.
pub(super) fn preflight_following_prose<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    parts: &[Content],
) {
    if l.is_sub_frame
        || l.column_mode
        || l.page_columns.is_some()
        || l.regions.cell.is_some()
        || !l.floats_pending.is_empty()
        || l.flow_suffix_transaction_active
        || super::html_frame::items_have_causal_line(&l.regions.current.current_line)
        || !l
            .pending_footnote_frames
            .first()
            .and_then(Option::as_ref)
            .is_some_and(|f| f.source_lines)
        || !l.page_config.height.is_finite()
    {
        return;
    }
    let style = super::text::effective_style(l);
    let width = l.regions.current.width - l.page_config.margin.horizontal();
    if !width.is_finite()
        || width <= 0.0
        || style.size.0 <= 0.0
        || !style.size.0.is_finite()
        || style.tracking.is_some()
        || style.math
        || style.superscript
        || style.subscript
        || style.dir == Some(crate::entities::dir::Dir::RTL)
    {
        return;
    }
    let mut lines = (1, 0.0, false);
    if !parts
        .iter()
        .all(|p| prose_lines(p, &l.metrics, &style, width, &mut lines))
        || !lines.2
    {
        return;
    }
    let (widow, orphan) = crate::compiler::text_costs::prevention(&l.chain);
    let group = if lines.0 == 3 && widow && orphan {
        3
    } else if lines.0 >= 2 && orphan {
        2
    } else {
        1
    };
    let (top, bottom) = l.metrics.text_edges(style.size, &style);
    let descent = (-bottom.0).max(0.0);
    let leading = style
        .leading
        .map(|v| v.resolve_pt(style.size.0))
        .unwrap_or(style.size.0 * super::vanilla_defaults::PAR_LEADING);
    let mut need = (group - 1) as f64 * (top.0 + descent + leading) + descent;
    let fresh = l.regions.current.height - l.page_config.margin.vertical();
    if !leading.is_finite()
        || leading < 0.0
        || top.0 <= 0.0
        || !descent.is_finite()
        || !need.is_finite()
        || !top.0.is_finite()
        || !fresh.is_finite()
        || fresh <= 0.0
    {
        return;
    }
    if top.0 + need > fresh {
        need = descent;
    }
    let mut stalled = false;
    loop {
        if !l.pending_footnote_bodies.is_empty()
            && !l
                .pending_footnote_frames
                .first()
                .and_then(Option::as_ref)
                .is_some_and(|f| f.source_lines)
        {
            return;
        }
        let before = super::footnote_fragments::pending_work(&l.pending_footnote_frames);
        let page = l.pages.len();
        l.ensure_text_baseline();
        if l.pages.len() != page {
            let after =
                super::footnote_fragments::pending_work(&l.pending_footnote_frames);
            stalled = !l.pending_footnote_bodies.is_empty() && after >= before;
            continue;
        }
        if l.regions.current.cursor_y.0 + need <= l.page_bottom_limit() {
            return;
        }
        if l.current_page_is_empty() && l.pending_footnote_bodies.is_empty() {
            return;
        }
        l.advance_footnote_region(stalled);
        let after = super::footnote_fragments::pending_work(&l.pending_footnote_frames);
        stalled = !l.pending_footnote_bodies.is_empty() && after >= before;
    }
}

/// Admit a certified carrier before its original source child is dispatched.
pub(super) fn preflight_carrier<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    parts: &[Content],
) {
    if l.is_sub_frame
        || ((l.column_mode || l.page_columns.is_some()) && !l.page_column_notes)
        || l.bidi_cursor.is_some()
        || l.regions.cell.is_some()
        || !l.pending_footnote_bodies.is_empty()
        || !l.floats_pending.is_empty()
        || !l.regions.current.current_line.is_empty()
        || !l.page_config.height.is_finite()
        || !l.regions.current.width.is_finite()
        || l.flow_suffix_transaction_active
    {
        return;
    }
    let style = super::text::effective_style(l);
    if style.tracking.is_some()
        || style.superscript
        || style.subscript
        || style.math
        || style.dir == Some(crate::entities::dir::Dir::RTL)
    {
        return;
    }
    let available_width = l.regions.current.width - l.page_config.margin.horizontal();
    let mut width = 0.0;
    let mut has_text = false;
    for part in parts {
        let (part, location) = unwrapped(part, None);
        if let Content::Text(text) = part {
            if text.chars().any(char::is_control) {
                return;
            }
            width += l.metrics.text_width(text, style.size, &style).0;
            has_text |= !text.is_empty();
            continue;
        }
        if matches!(part, Content::Empty) {
            continue;
        }
        if matches!(part, Content::Space | Content::SpaceFromNewline) {
            if has_text {
                width += l.metrics.text_width(" ", style.size, &style).0;
            }
            continue;
        }
        let Content::Footnote(note) = part else {
            return;
        };
        if !has_text || !admission_text(&note.body) {
            return;
        }
        let key = CounterKey::Selector(Selector::Kind(ElementKind::Footnote));
        let n = location
            .and_then(|loc| l.introspector.flat_counter_at(&key, loc))
            .unwrap_or(1);
        let marker_text = format_counter(&[n], note.numbering.as_deref().unwrap_or("1"))
            .unwrap_or_else(|| n.to_string());
        let mut script = style.clone();
        script.superscript = true;
        script.size = l
            .metrics
            .text_superscript_metrics(&marker_text, style.size, &script)
            .0;
        width += l.metrics.text_width(&marker_text, script.size, &script).0;
        if !width.is_finite() || width > available_width {
            return;
        }
        let marker = Content::superscript(Content::text(marker_text));
        let entry = entry_settings(l);
        let Some(separator_height) = entry.separator_height else { return };
        let prefix = entry_prefix(marker, style.size.0, entry.indent);
        let height = {
            let metrics: &dyn FontMetrics = &l.metrics;
            let mut probe = Layouter::new(
                metrics,
                crate::entities::image_sizer::NullImageSizer,
                style.size.0,
                l.introspector,
            );
            probe.chain = l.chain.clone();
            probe.footnote_entry_scope = l.footnote_entry_scope.clone();
            probe.style = style.clone();
            probe.page_config = l.page_config.clone();
            probe.regions.current.width = l.regions.current.width;
            let frame = probe.capture_footnote_frame(&prefix, &note.body, None);
            if l.page_column_notes {
                if !frame.source_lines {
                    return;
                }
                super::footnote_fragments::first_line_requirement(&frame)
            } else {
                frame.height
            }
        };
        let overhead = entry.clearance + entry.gap + separator_height;
        let top = l.metrics.text_edges(style.size, &style).0 .0;
        let initial = if l.initial_baseline_pending { top } else { 0.0 };
        let mut required = initial + overhead + height;
        let mut remaining = l.regions.current.height
            - l.page_config.margin.bottom
            - l.regions.current.cursor_y.0;
        let fresh = l.regions.current.height - l.page_config.margin.vertical();
        if !required.is_finite() {
            return;
        }
        if l.page_column_notes {
            while required > remaining {
                if top + overhead + height > fresh
                    && l.page_columns.is_none_or(|count| l.current_column + 1 >= count)
                {
                    break;
                }
                l.new_page();
                let initial = if l.initial_baseline_pending { top } else { 0.0 };
                required = initial + overhead + height;
                remaining = l.regions.current.height
                    - l.page_config.margin.bottom
                    - l.regions.current.cursor_y.0;
            }
        } else if required > remaining && top + overhead + height <= fresh {
            l.new_page();
        }
        return;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{
        image_sizer::NullImageSizer, introspector::TagIntrospector,
        layout_types::FrameItem,
    };
    use comemo::Track;

    fn texts(items: &[FrameItem], out: &mut Vec<String>) {
        for item in items {
            match item {
                FrameItem::Text { text, .. } | FrameItem::TextShaped { text, .. } => {
                    out.push(text.to_string())
                }
                FrameItem::Group { items, .. }
                | FrameItem::Link { items, .. }
                | FrameItem::Semantic { items, .. }
                | FrameItem::TextReplacement { items, .. } => texts(items, out),
                FrameItem::TextSeparator { .. }
                | FrameItem::Line { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. }
                | FrameItem::Shape { .. } => {}
            }
        }
    }

    fn check(pattern: Option<&str>, marker: &str, rollback: bool) {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        layout(
            &mut l,
            &FootnoteElem {
                body: Content::text("LITERAL[2]END"),
                numbering: pattern.map(Into::into),
            },
        );
        if rollback {
            let checkpoint = l.capture_flow_suffix_checkpoint();
            l.flush_pending_footnote_bodies(None);
            l.restore_flow_suffix_checkpoint(checkpoint);
        }
        let doc = l.finish();
        let mut out = Vec::new();
        for page in &doc.pages {
            texts(&page.items, &mut out);
        }
        assert_eq!(out.iter().filter(|s| s.as_str() == marker).count(), 2, "{out:?}");
        assert_eq!(
            out.iter().filter(|s| s.contains("LITERAL[2]END")).count(),
            1,
            "{out:?}"
        );
        if marker != "[1]" {
            assert!(!out.iter().any(|s| s.contains("[1]")), "{out:?}");
        }
    }

    #[test]
    fn r1_footnote_default_entry_matches_call() {
        check(None, "1", false);
    }
    #[test]
    fn r1_footnote_custom_entry_matches_call() {
        check(Some("a"), "a", false);
    }
    #[test]
    fn r1_footnote_intentional_brackets_remain() {
        check(Some("[1]"), "[1]", false);
    }
    #[test]
    fn r1_footnote_custom_marker_survives_destructive_flush_rollback() {
        check(Some("(i)"), "(i)", true);
    }
}

#[cfg(test)]
mod r8_entry_tests {
    use super::*;
    use crate::entities::{
        image_sizer::NullImageSizer,
        introspector::TagIntrospector,
        layout_types::{FrameItem, PageMargins},
    };
    use comemo::Track;
    fn collect(items: &[FrameItem], out: &mut Vec<(String, f64, f64)>) {
        for item in items {
            match item {
                FrameItem::Text { text, pos, style, .. }
                | FrameItem::TextShaped { text, pos, style, .. } => {
                    out.push((text.to_string(), pos.x.0, style.size.0))
                }
                FrameItem::Group { items, .. }
                | FrameItem::Link { items, .. }
                | FrameItem::Semantic { items, .. }
                | FrameItem::TextReplacement { items, .. } => collect(items, out),
                FrameItem::TextSeparator { .. }
                | FrameItem::Line { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. }
                | FrameItem::Shape { .. } => {}
            }
        }
    }
    #[test]
    fn r8_entry_body_uses_reduced_size_and_indented_prefix() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.margin = PageMargins::uniform(20.0);
        l.page_config.margin_spec =
            crate::entities::layout_types::PageMarginSpec::uniform(20.0);
        layout(&mut l, &FootnoteElem { body: Content::text("NOTE"), numbering: None });
        let doc = l.finish();
        let mut rows = Vec::new();
        for p in doc.pages {
            collect(&p.items, &mut rows);
        }
        let body = rows.iter().find(|r| r.0 == "NOTE").unwrap();
        assert!((body.2 - 9.35).abs() < 1e-8, "{rows:?}");
        let entry = rows.iter().rfind(|r| r.0 == "1").unwrap();
        assert!((entry.1 - 29.35).abs() < 1e-8, "{rows:?}");
    }
    #[test]
    fn r8_nested_note_body_is_emitted_once_at_root_entry_size() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        let nested = Content::Footnote(std::sync::Arc::new(FootnoteElem {
            body: Content::text("INNER"),
            numbering: None,
        }));
        layout(
            &mut l,
            &FootnoteElem {
                body: Content::sequence(vec![
                    Content::text("FIRST"),
                    nested,
                    Content::text("END"),
                ]),
                numbering: None,
            },
        );
        let doc = l.finish();
        let mut rows = Vec::new();
        for p in doc.pages {
            collect(&p.items, &mut rows);
        }
        let inner: Vec<_> = rows.iter().filter(|r| r.0 == "INNER").collect();
        assert_eq!(inner.len(), 1, "{rows:?}");
        assert!((inner[0].2 - 9.35).abs() < 1e-8, "{rows:?}");
    }
}

#[cfg(test)]
mod r8_spill_tests {
    use super::*;
    use crate::entities::{
        elements::v_space::VSpaceElem,
        image_sizer::NullImageSizer,
        introspector::TagIntrospector,
        layout_types::{FrameItem, Length, PageMargins},
    };
    use comemo::Track;
    #[test]
    fn r8_explicit_note_spacing_preserves_end_on_second_page() {
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
        l.page_config.margin_spec =
            crate::entities::layout_types::PageMarginSpec::uniform(20.0);
        l.regions.current.cursor_x = crate::entities::layout_types::Pt(20.0);
        l.regions.current.line_start_x = crate::entities::layout_types::Pt(20.0);
        l.regions.current.cursor_y = crate::entities::layout_types::Pt(20.0);
        l.initial_baseline_pending = true;
        l.line_assumed_ascent = None;
        l.layout_content(&Content::text("ALPHA"));
        let body = Content::sequence(vec![
            Content::text("START"),
            Content::VSpace(std::sync::Arc::new(VSpaceElem {
                amount: Length::pt(120.0),
                weak: false,
                weak_explicit: false,
            })),
            Content::text("END"),
        ]);
        layout(&mut l, &FootnoteElem { body, numbering: None });
        let captured = l.pending_footnote_frames.clone();
        let doc = l.finish();
        let end = doc
            .pages
            .iter()
            .enumerate()
            .flat_map(|(p, page)| {
                super::super::tests::frame_items_recursive(&page.items)
                    .into_iter()
                    .map(move |item| (p, item))
            })
            .find_map(|(p, item)| match item {
                FrameItem::Text { pos, text, .. } if text.as_str() == "END" => {
                    Some((p, pos.y.0))
                }
                _ => None,
            })
            .expect("END retained");
        assert_eq!(end.0, 1, "{doc:?}");
        assert!(
            end.1 <= 80.0,
            "END outside page: {end:?}; captured={captured:?}; {doc:?}"
        );
        let separator = doc.pages[1]
            .items
            .iter()
            .find_map(|item| match item {
                FrameItem::Line { start, .. } => Some(start.y.0),
                _ => None,
            })
            .unwrap();
        let size = crate::entities::layout_types::Pt(9.35);
        let top = super::super::FixedMetrics
            .text_edges(size, &crate::entities::layout_types::TextStyle::regular(size))
            .0
             .0;
        assert!(
            (separator - (80.0 - 5.5 - top)).abs() < 1e-8,
            "continuation separator={separator}, top={top}"
        );
    }
}

#[cfg(test)]
mod r8_empty_admission_tests {
    use super::*;
    use crate::entities::{
        image_sizer::NullImageSizer,
        introspector::TagIntrospector,
        layout_types::{PageMarginSpec, PageMargins, Pt},
    };
    use comemo::Track;
    #[test]
    fn r8_empty_sibling_has_no_width_in_carrier_admission() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = 76.97;
        l.regions.current.width = 76.97;
        l.page_config.height = 100.0;
        l.regions.current.height = 100.0;
        l.page_config.margin = PageMargins::uniform(20.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(20.0);
        l.regions.current.cursor_x = Pt(20.0);
        l.regions.current.line_start_x = Pt(20.0);
        l.regions.current.cursor_y = Pt(65.0);
        l.initial_baseline_pending = true;
        l.line_assumed_ascent = None;
        l.page_has_unpainted_material = true;
        let parts = [
            Content::text("ALPHA"),
            Content::Empty,
            Content::Footnote(std::sync::Arc::new(FootnoteElem {
                body: Content::text("NOTE"),
                numbering: None,
            })),
        ];
        preflight_carrier(&mut l, &parts);
        assert_eq!(l.pages.len(), 1, "Empty must not consume a separating space");
    }
}

#[cfg(test)]
mod r12_entry_tests {
    use super::*;
    use crate::entities::{
        image_sizer::NullImageSizer,
        introspector::TagIntrospector,
        layout_types::{FrameItem, Length, PageMarginSpec, PageMargins, PagedDocument},
        style::Styles,
        value::Value,
    };
    use comemo::Track;

    fn settings(values: &[(&str, Value)], body: Content) -> Content {
        let styles = values.iter().fold(Styles::new(), |styles, (name, value)| {
            styles.push_custom(format!("footnote.entry.{name}"), value.clone())
        });
        Content::styled(body, styles)
    }

    fn notes() -> Content {
        Content::sequence(vec![
            Content::text("Alpha"),
            Content::footnote(Content::text("FIRST")),
            Content::text(" Beta"),
            Content::footnote(Content::text("SECOND")),
        ])
    }

    fn render(content: &Content, height: f64) -> PagedDocument {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = 300.0;
        l.page_config.height = height;
        l.page_config.margin = PageMargins::uniform(20.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(20.0);
        l.regions.current.width = 300.0;
        l.regions.current.height = height;
        l.layout_content(content);
        l.finish()
    }

    fn rows(doc: &PagedDocument) -> Vec<(String, f64, f64)> {
        fn walk(items: &[FrameItem], out: &mut Vec<(String, f64, f64)>) {
            for item in items {
                match item {
                    FrameItem::Text { text, pos, .. }
                    | FrameItem::TextShaped { text, pos, .. } => {
                        out.push((text.to_string(), pos.x.0, pos.y.0));
                    }
                    FrameItem::Group { items, .. }
                    | FrameItem::Link { items, .. }
                    | FrameItem::Semantic { items, .. }
                    | FrameItem::TextReplacement { items, .. } => walk(items, out),
                    FrameItem::TextSeparator { .. }
                    | FrameItem::Line { .. }
                    | FrameItem::Glyph { .. }
                    | FrameItem::Image { .. }
                    | FrameItem::Shape { .. } => {}
                }
            }
        }
        let mut out = Vec::new();
        for page in &doc.pages {
            walk(&page.items, &mut out);
        }
        out
    }

    fn position(doc: &PagedDocument, word: &str) -> (f64, f64) {
        let found = rows(doc).into_iter().filter(|r| r.0 == word).collect::<Vec<_>>();
        assert_eq!(found.len(), 1, "{word}: {found:?}");
        (found[0].1, found[0].2)
    }

    #[test]
    fn r12_indent_resolves_em_at_reduced_entry_size() {
        let before = render(&notes(), 180.0);
        let after = render(
            &settings(&[("indent", Value::Length(Length::em(2.0)))], notes()),
            180.0,
        );
        for word in ["FIRST", "SECOND"] {
            assert!(
                (position(&after, word).0 - position(&before, word).0 - 9.35).abs()
                    < 1e-8
            );
        }
    }

    #[test]
    fn r12_gap_changes_entry_separation_by_the_declared_outer_length() {
        let before = render(&notes(), 180.0);
        let after = render(
            &settings(&[("gap", Value::Length(Length::pt(12.0)))], notes()),
            180.0,
        );
        let distance =
            |doc: &PagedDocument| position(doc, "SECOND").1 - position(doc, "FIRST").1;
        assert!((distance(&after) - distance(&before) - 6.5).abs() < 1e-8);
        assert_eq!(position(&after, "SECOND"), position(&before, "SECOND"));
    }

    #[test]
    fn r12_none_separator_removes_only_the_default_rule() {
        let before = render(&notes(), 180.0);
        let after = render(&settings(&[("separator", Value::None)], notes()), 180.0);
        let lines = |doc: &PagedDocument| {
            doc.pages
                .iter()
                .flat_map(|p| &p.items)
                .filter(|i| matches!(i, FrameItem::Line { .. }))
                .count()
        };
        assert_eq!(lines(&before), 1);
        assert_eq!(lines(&after), 0);
        assert_eq!(rows(&after), rows(&before));
    }

    #[test]
    fn r12_original_text_separator_is_emitted_once_in_outer_style() {
        let doc = render(
            &settings(&[("separator", Value::Content(Content::text("SEP")))], notes()),
            180.0,
        );
        let sep = position(&doc, "SEP");
        assert!(sep.1 < position(&doc, "FIRST").1);
        assert_eq!(sep.0, 20.0);
    }

    #[test]
    fn r12_local_override_does_not_replace_shared_entry_scope() {
        let body = Content::sequence(vec![
            Content::text("Alpha"),
            Content::footnote(Content::text("FIRST")),
            settings(
                &[("indent", Value::Length(Length::pt(0.0)))],
                Content::footnote(Content::text("SECOND")),
            ),
        ]);
        let before = render(&notes(), 180.0);
        let doc =
            render(&settings(&[("indent", Value::Length(Length::em(2.0)))], body), 180.0);
        assert!(
            (position(&doc, "FIRST").0 - position(&before, "FIRST").0 - 9.35).abs()
                < 1e-8
        );
        assert_eq!(position(&doc, "FIRST").0, position(&doc, "SECOND").0);
    }

    #[test]
    fn r12_impossible_clearance_preserves_progress_and_complete_entry() {
        let body = Content::sequence(vec![
            Content::text("Alpha"),
            Content::footnote(Content::text("FIRST")),
        ]);
        let doc = render(
            &settings(&[("clearance", Value::Length(Length::pt(30.0)))], body),
            80.0,
        );
        assert_eq!(doc.pages.len(), 3, "{doc:?}");
        position(&doc, "FIRST");
    }
}
