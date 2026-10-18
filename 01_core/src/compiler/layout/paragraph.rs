//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/paragraph.md
//! @prompt-hash c1faa075
//! @layer L1
//! @updated 2026-10-17

use super::{html_frame, parbreak, FontMetrics, ImageSizer, Layouter};
use crate::entities::{content::Content, layout_types::Pt, value::Value};

/// Inspect a completed visual without changing the content that is emitted.
pub(super) fn prepared_text_body(content: &Content) -> &Content {
    match content {
        Content::Source(e) => prepared_text_body(&e.body),
        Content::Located(e) => prepared_text_body(&e.body),
        Content::Realized(e) => prepared_text_body(&e.visual),
        Content::ShowRecipe(..)
        | Content::Empty
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
        | Content::Repeat(..)
        | Content::Columns(..)
        | Content::Metadata(..)
        | Content::State(..)
        | Content::StateUpdate(..)
        | Content::StateDisplay(..)
        | Content::CounterDisplayCallback(..)
        | Content::ContextBlock(..)
        | Content::Dynamic(..)
        | Content::TableCell(..)
        | Content::Table(..) => content,
    }
}

/// Resolve only a complete textual segment, preserving shared lexical scope
/// rather than taking a consensus of individually resolved run values.
pub(super) fn shared_direction(
    parts: &[Content],
    chain: &crate::entities::style_chain::StyleChain,
) -> Option<crate::entities::dir::Dir> {
    shared_chain(parts, chain).map(|chain| chain_direction(&chain))
}

pub(super) fn shared_chain(
    parts: &[Content],
    chain: &crate::entities::style_chain::StyleChain,
) -> Option<crate::entities::style_chain::StyleChain> {
    fn certified(c: &Content) -> bool {
        let c = prepared_text_body(c);
        if matches!(c, Content::Empty) {
            return true;
        }
        if matches!(c, Content::Text(_)) {
            return true;
        }
        if matches!(c, Content::Space) {
            return true;
        }
        if matches!(c, Content::SpaceFromNewline) {
            return true;
        }
        if matches!(c, Content::Linebreak(_)) {
            return true;
        }
        if let Content::Sequence(p) = c {
            return p.iter().all(certified);
        }
        if let Content::Styled(s) = c {
            return certified(&s.body);
        }
        if let Content::Strong(s) = c {
            return certified(&s.body);
        }
        if let Content::Emph(s) = c {
            return certified(&s.body);
        }
        // A rejected carrier still goes through ordinary layout unchanged.
        false
    }
    if !parts.iter().all(certified) {
        return None;
    }
    let mut substantive = parts.iter().map(prepared_text_body).filter(|p| {
        !matches!(p, Content::Empty)
            && !matches!(p, Content::Space)
            && !matches!(p, Content::SpaceFromNewline)
    });
    let first = substantive.next();
    if substantive.next().is_none() {
        if let Some(first) = first {
            if let Content::Styled(s) = first {
                return shared_chain(
                    std::slice::from_ref(&s.body),
                    &chain.push_styles(&s.styles),
                );
            }
            if let Content::Sequence(p) = first {
                return shared_chain(p, chain);
            }
            if let Content::Strong(s) = first {
                return shared_chain(std::slice::from_ref(&s.body), chain);
            }
            if let Content::Emph(s) = first {
                return shared_chain(std::slice::from_ref(&s.body), chain);
            }
        }
    }
    Some(chain.clone())
}

pub(super) fn spacing(chain: &crate::entities::style_chain::StyleChain) -> f64 {
    spacing_for_size(chain, crate::entities::layout_types::TextStyle::from(chain).size.0)
}

pub(super) fn spacing_for_size(
    chain: &crate::entities::style_chain::StyleChain,
    size: f64,
) -> f64 {
    match chain.custom("par.spacing") {
        Some(Value::Length(length)) => length.resolve_pt(size),
        _ => size * super::vanilla_defaults::PAR_SPACING,
    }
}

/// Close only a complete textual segment whose lexical scope is known.
pub(super) fn close_segment<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    parts: &[Content],
) {
    let Some(chain) = shared_chain(parts, &l.chain) else { return };
    close_in_chain(l, chain, !parts.is_empty());
}

pub(super) fn close_in_chain<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    chain: crate::entities::style_chain::StyleChain,
    nonempty: bool,
) {
    let outer_spacing = spacing(&l.chain);
    if !html_frame::items_have_causal_line(&l.regions.current.current_line) {
        if nonempty && !l.initial_baseline_pending && !l.block_chain_active {
            if let Some(end) = l.flow_placement_y() {
                l.prev_line_baseline = end;
                l.prev_block_equation_descent = 0.0;
                l.prev_block_below_pending = spacing(&chain).max(outer_spacing);
                l.block_chain_active = true;
                l.prev_margin_is_parbreak = true;
            }
        }
        return;
    }
    let previous_chain = std::mem::replace(&mut l.chain, chain);
    let previous_style = std::mem::replace(
        &mut l.style,
        crate::entities::layout_types::TextStyle::from(&l.chain),
    );
    parbreak::layout(l);
    l.chain = previous_chain;
    l.style = previous_style;
    l.prev_block_below_pending = l.prev_block_below_pending.max(outer_spacing);
}

/// Pure textual measurement owns no effects or state in the emitter.
pub(super) fn measure_segment<M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
    parts: &[Content],
    width: f64,
    single_line: bool,
) -> Option<(f64, f64)> {
    use comemo::Track;
    let chain = shared_chain(parts, &l.chain)?;
    if parts.is_empty() || !width.is_finite() || width <= 0.0 {
        return None;
    }
    let intr = crate::entities::introspector::TagIntrospector::empty();
    let metrics: &dyn FontMetrics = &l.metrics;
    let mut isolated = Layouter::new(
        metrics,
        crate::entities::image_sizer::NullImageSizer,
        l.style.size.0,
        (&intr as &dyn crate::entities::introspector::Introspector).track(),
    );
    isolated.chain = l.chain.clone();
    isolated.style = l.style.clone();
    let content = Content::Sequence(parts.to_vec().into());
    let (height, items, _, _, _) = isolated.layout_sub_frame_at_geometric_origin(
        &content,
        super::sub_frame::SubLayoutRegion {
            origin_x: 0.0,
            width,
            height: None,
            align_rtl: false,
            unconstrained_height: true,
        },
    );
    let height = if single_line {
        let mut baseline = None;
        text_frame_bottom(&items, &l.metrics, &mut baseline, true)?
    } else {
        height
    };
    (height.is_finite() && height >= 0.0).then(|| (height, spacing(&chain)))
}

/// Isolated causal measurement of a certified multiline prefix.
pub(super) fn measure_multiline_prefix<M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
    parts: &[Content],
    width: f64,
) -> Option<(f64, f64)> {
    use crate::entities::layout_types::{PageMarginSpec, PageMargins};
    use comemo::Track;
    let chain = shared_chain(parts, &l.chain)?;
    if !width.is_finite() || width <= 0.0 || l.bidi_cursor.is_some() {
        return None;
    }
    let intr = crate::entities::introspector::TagIntrospector::empty();
    let metrics: &dyn FontMetrics = &l.metrics;
    let mut isolated = Layouter::new(
        metrics,
        crate::entities::image_sizer::NullImageSizer,
        l.style.size.0,
        (&intr as &dyn crate::entities::introspector::Introspector).track(),
    );
    isolated.chain = l.chain.clone();
    isolated.style = l.style.clone();
    isolated.text_segment_chain = Some(chain.clone());
    isolated.geometric_text_lines = true;
    super::paragraph_prepare::prepare_parts(&mut isolated, parts)?;
    isolated.is_sub_frame = true;
    isolated.is_height_unconstrained = true;
    isolated.page_config.width = width;
    isolated.page_config.height = f64::INFINITY;
    isolated.page_config.margin = PageMargins::uniform(0.0);
    isolated.page_config.margin_spec = PageMarginSpec::uniform(0.0);
    isolated.regions.current.width = width;
    isolated.regions.current.height = f64::INFINITY;
    isolated.regions.current.cursor_x = Pt(0.0);
    isolated.regions.current.line_start_x = Pt(0.0);
    isolated.regions.current.cursor_y = Pt(0.0);
    isolated.layout_content(&Content::Sequence(parts.to_vec().into()));
    isolated.flush_line();
    let items = &isolated.regions.current.current_items;
    let height = text_frame_bottom(items, &l.metrics, &mut None, false)?;
    if text_frame_bottom(items, &l.metrics, &mut None, true).is_some() {
        return None;
    }
    (height.is_finite() && height > 0.0).then(|| (height, spacing(&chain)))
}

fn text_frame_bottom<M: FontMetrics>(
    items: &[crate::entities::layout_types::FrameItem],
    metrics: &M,
    baseline: &mut Option<f64>,
    single_line: bool,
) -> Option<f64> {
    use crate::entities::layout_types::FrameItem;
    let mut bottom = 0.0_f64;
    for item in items {
        let end = match item {
            FrameItem::Text { pos, style, .. }
            | FrameItem::TextShaped { pos, style, .. } => {
                if single_line && baseline.is_some_and(|y| (y - pos.y.0).abs() > 1e-8) {
                    return None;
                }
                baseline.get_or_insert(pos.y.0);
                pos.y.0 - metrics.text_edges(style.size, style).1 .0
            }
            FrameItem::TextSeparator { .. } => continue,
            FrameItem::Semantic { items, .. }
            | FrameItem::TextReplacement { items, .. } => {
                text_frame_bottom(items, metrics, baseline, single_line)?
            }
            FrameItem::Glyph { .. }
            | FrameItem::Image { .. }
            | FrameItem::Shape { .. }
            | FrameItem::Group { .. }
            | FrameItem::Link { .. }
            | FrameItem::Line { .. } => return None,
        };
        if !end.is_finite() {
            return None;
        }
        bottom = bottom.max(end);
    }
    Some(bottom)
}

/// Reserve the first actual fragment, keeping ordinary widow/orphan admission.
pub(super) fn measure_first_fragment<M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
    parts: &[Content],
    width: f64,
    height: f64,
) -> Option<f64> {
    use crate::entities::layout_types::{PageMarginSpec, PageMargins};
    use comemo::Track;
    shared_chain(parts, &l.chain)?;
    if !height.is_finite() || height <= 0.0 {
        return None;
    }
    let intr = crate::entities::introspector::TagIntrospector::empty();
    let metrics: &dyn FontMetrics = &l.metrics;
    let mut isolated = Layouter::new(
        metrics,
        crate::entities::image_sizer::NullImageSizer,
        l.style.size.0,
        (&intr as &dyn crate::entities::introspector::Introspector).track(),
    );
    isolated.chain = l.chain.clone();
    isolated.style = l.style.clone();
    isolated.page_config.width = width;
    isolated.page_config.height = height;
    isolated.page_config.margin = PageMargins::uniform(0.0);
    isolated.page_config.margin_spec = PageMarginSpec::uniform(0.0);
    isolated.regions.current.width = width;
    isolated.regions.current.height = height;
    isolated.regions.current.cursor_x = Pt(0.0);
    isolated.regions.current.line_start_x = Pt(0.0);
    isolated.regions.current.cursor_y = Pt(0.0);
    isolated.layout_content(&Content::Sequence(parts.to_vec().into()));
    let doc = isolated.finish();
    text_frame_bottom(&doc.pages.first()?.items, &l.metrics, &mut None, false)
}

fn chain_direction(
    chain: &crate::entities::style_chain::StyleChain,
) -> crate::entities::dir::Dir {
    chain
        .custom("text.dir")
        .and_then(|v| match v {
            Value::Dir(d) => Some(*d),
            _ => None,
        })
        .unwrap_or(crate::entities::dir::Dir::LTR)
}

pub(super) fn line_direction<M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
) -> Option<crate::entities::dir::Dir> {
    l.active_paragraph_dir.or(l.pending_line_dir).or_else(|| {
        l.regions.current.current_line.iter().find_map(|item| match item {
            crate::entities::layout_types::FrameItem::Text { style, .. } => style.dir,
            crate::entities::layout_types::FrameItem::TextShaped { style, .. } => {
                style.dir
            }
            crate::entities::layout_types::FrameItem::TextSeparator { .. } => None,
            crate::entities::layout_types::FrameItem::Glyph { .. } => None,
            crate::entities::layout_types::FrameItem::Image { .. } => None,
            crate::entities::layout_types::FrameItem::Shape { .. } => None,
            crate::entities::layout_types::FrameItem::Group { .. } => None,
            crate::entities::layout_types::FrameItem::Link { .. } => None,
            crate::entities::layout_types::FrameItem::Line { .. } => None,
            crate::entities::layout_types::FrameItem::TextReplacement { .. } => None,
            crate::entities::layout_types::FrameItem::Semantic { .. } => None,
        })
    })
}

enum ParagraphBody<'a> {
    Empty,
    Space,
    Text(&'a str),
    Sequence(&'a [Content]),
    Styled(&'a crate::entities::content::StyledContent),
    Par,
    Opaque,
}

/// One exhaustive boundary owns the admission policy shared by these predicates.
fn paragraph_body(content: &Content) -> ParagraphBody<'_> {
    match content {
        Content::Source(e) => paragraph_body(&e.body),
        Content::Empty => ParagraphBody::Empty,
        Content::Space | Content::SpaceFromNewline => ParagraphBody::Space,
        Content::Text(text) => ParagraphBody::Text(text.as_str()),
        Content::Sequence(parts) => ParagraphBody::Sequence(parts),
        Content::Styled(styled) => ParagraphBody::Styled(styled),
        Content::Par { .. } => ParagraphBody::Par,
        Content::ShowRecipe(..)
        | Content::Realized(..)
        | Content::Located(..)
        | Content::Parbreak
        | Content::HtmlElem(..)
        | Content::HtmlFrame(..)
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
        | Content::Dynamic(..) => ParagraphBody::Opaque,
    }
}

pub(super) fn is_paragraph(content: &Content) -> bool {
    match paragraph_body(content) {
        ParagraphBody::Par => true,
        ParagraphBody::Styled(styled) => is_paragraph(&styled.body),
        ParagraphBody::Empty
        | ParagraphBody::Space
        | ParagraphBody::Text(_)
        | ParagraphBody::Sequence(_)
        | ParagraphBody::Opaque => false,
    }
}

fn is_empty_body(content: &Content) -> bool {
    match paragraph_body(content) {
        ParagraphBody::Empty | ParagraphBody::Space => true,
        ParagraphBody::Text(text) => text.is_empty(),
        ParagraphBody::Sequence(parts) => parts.iter().all(is_empty_body),
        ParagraphBody::Styled(styled) => is_empty_body(&styled.body),
        ParagraphBody::Par | ParagraphBody::Opaque => false,
    }
}

fn empty_text_style(
    content: &Content,
    chain: &crate::entities::style_chain::StyleChain,
) -> Option<crate::entities::style_chain::StyleChain> {
    match paragraph_body(content) {
        ParagraphBody::Text(text) => text.is_empty().then(|| chain.clone()),
        ParagraphBody::Sequence(parts) => {
            parts.iter().rev().find_map(|part| empty_text_style(part, chain))
        }
        ParagraphBody::Styled(styled) => {
            empty_text_style(&styled.body, &chain.push_styles(&styled.styles))
        }
        ParagraphBody::Empty
        | ParagraphBody::Space
        | ParagraphBody::Par
        | ParagraphBody::Opaque => None,
    }
}

pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    body: &Content,
) {
    let had_previous_text =
        html_frame::items_have_material(&layouter.regions.current.current_line)
            || layouter.terminal_paragraph_line;
    parbreak::layout(layouter);
    let previous_indent = std::mem::take(&mut layouter.paragraph_indent);
    super::paragraph_indent::start_scope(layouter, std::slice::from_ref(body));
    let previous_dir = layouter.active_paragraph_dir;
    let previous_bidi = layouter.bidi_cursor.take();
    let previous_scope = layouter.bidi_scope_active;
    layouter.bidi_scope_active = true;
    layouter.active_paragraph_dir = Some(chain_direction(&layouter.chain));
    layouter.bidi_cursor = super::bidi_origin::prepare(
        layouter,
        std::slice::from_ref(body),
        chain_direction(&layouter.chain),
    );
    let empty_body = is_empty_body(body);
    if !empty_body
        && (layouter.bidi_cursor.is_some()
            || !super::paragraph_plan::try_layout(layouter, body))
    {
        layouter.layout_content(body);
    }
    if empty_body {
        let size = layouter.style.size.val();
        let spacing = match layouter.chain.custom("par.spacing") {
            Some(Value::Length(length)) => length.resolve_pt(size),
            _ => size * super::vanilla_defaults::PAR_SPACING,
        };
        let virtual_height =
            empty_text_style(body, &layouter.chain).map_or(0.0, |chain| {
                let style = crate::entities::layout_types::TextStyle::from(&chain);
                let (top, bottom) = layouter.metrics.text_edges(style.size, &style);
                let offset = style.baseline_offset.resolve_pt(style.size.val());
                (top.0 - offset).max(0.0) + (-bottom.0 + offset).max(0.0)
            });
        let first_empty =
            layouter.initial_baseline_pending && !layouter.block_chain_active;
        let above = if layouter.block_chain_active && !layouter.prev_margin_is_parbreak {
            layouter.prev_block_below_pending
        } else {
            layouter.prev_block_below_pending.max(spacing)
        };
        let end = if first_empty {
            layouter.regions.current.cursor_y.0 + virtual_height
        } else if had_previous_text || layouter.block_chain_active {
            layouter.prev_line_baseline
                + layouter.prev_block_equation_descent
                + above
                + virtual_height
        } else {
            layouter.regions.current.cursor_y.0 + spacing + virtual_height
        };
        // A zero-height paragraph is a causal unit; only its lower margin is pending.
        layouter.regions.current.cursor_y = Pt(end);
        layouter.prev_line_baseline = end;
        layouter.prev_block_equation_descent = 0.0;
        layouter.last_block_descent_y =
            Some(layouter.last_block_descent_y.map_or(end, |bottom| bottom.max(end)));
        layouter.record_flow_placement_end(end);
        layouter.prev_block_below_pending = spacing;
        layouter.initial_baseline_pending = false;
        layouter.empty_paragraph_weak_spacing = 0.0;
        layouter.block_chain_active = true;
        layouter.prev_margin_is_parbreak = true;
        layouter.last_flush_advance = 0.0;
        layouter.terminal_paragraph_line = false;
    } else if html_frame::items_have_material(&layouter.regions.current.current_line) {
        parbreak::layout(layouter);
        layouter.terminal_paragraph_line = true;
    }
    super::paragraph_indent::end(layouter);
    layouter.paragraph_indent = previous_indent;
    layouter.active_paragraph_dir = previous_dir;
    layouter.bidi_cursor = previous_bidi;
    layouter.bidi_scope_active = previous_scope;
}

#[cfg(test)]
mod tests {
    #![allow(deprecated)]
    use super::super::{FixedMetrics, Layouter};
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{FrameItem, PageMarginSpec, PageMargins, Pt, TextStyle},
        style::{Style, Styles},
        value::Value,
    };
    use comemo::Track;

    #[test]
    fn paragraph_anchor_survives_inline_direction_overrides() {
        use crate::entities::{dir::Dir, elements::linebreak::LinebreakElem};
        for (outer, inner) in [(Dir::RTL, Dir::LTR), (Dir::LTR, Dir::RTL)] {
            let intr = TagIntrospector::empty();
            let mut l = make(&intr, false);
            l.chain = l.chain.push_custom("text.dir", Value::Dir(outer));
            l.style = TextStyle::from(&l.chain);
            let br = || {
                Content::Linebreak(std::sync::Arc::new(LinebreakElem {
                    justify: false,
                    justify_explicit: false,
                }))
            };
            let styled = Content::styled(
                Content::text("B"),
                Styles::new().push_custom("text.dir", Value::Dir(inner)),
            );
            l.layout_content(&Content::sequence(vec![
                Content::text("A"),
                br(),
                styled,
                br(),
                Content::text("C"),
            ]));
            let doc = l.finish();
            let xs: Vec<_> = doc
                .pages
                .iter()
                .flat_map(|p| &p.items)
                .filter_map(|item| match item {
                    FrameItem::Text { pos, text, style, .. } if text.as_str() == "B" => {
                        assert_eq!(style.dir, Some(inner));
                        Some(pos.x.0)
                    }
                    _ => None,
                })
                .collect();
            assert_eq!(xs.len(), 1);
            assert!(
                (xs[0] - if outer == Dir::RTL { 62.8 } else { 0.0 }).abs() < 1e-8,
                "all paragraph lines must keep the paragraph anchor: {xs:?}"
            );
        }
    }

    #[test]
    fn checkpoint_restores_paragraph_anchor_for_pending_text() {
        use crate::entities::dir::Dir;
        let intr = TagIntrospector::empty();
        for active in [false, true] {
            let mut l = make(&intr, false);
            if active {
                l.active_paragraph_dir = Some(Dir::RTL);
            } else {
                l.pending_line_dir = Some(Dir::RTL);
            }
            let checkpoint = l.capture_flow_suffix_checkpoint();
            l.active_paragraph_dir = Some(Dir::LTR);
            l.pending_line_dir = Some(Dir::LTR);
            l.restore_flow_suffix_checkpoint(checkpoint);
            l.layout_content(&Content::text("B"));
            let p = positions(&l.finish());
            assert!((p[0].1 - 62.8).abs() < 1e-8);
        }
    }

    #[test]
    fn subframe_has_its_own_direction_and_restores_parent_anchor() {
        use crate::entities::dir::Dir;
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.active_paragraph_dir = Some(Dir::RTL);
        l.pending_line_dir = Some(Dir::RTL);
        let (_, items, _, _, _) = l.layout_sub_frame(
            &Content::text("Inner"),
            super::super::sub_frame::SubLayoutRegion {
                origin_x: 0.0,
                width: 70.0,
                height: None,
                align_rtl: false,
                unconstrained_height: true,
            },
        );
        let mut p = vec![];
        texts(&items, &mut p);
        assert!((p[0].1).abs() < 1e-8, "inner LTR text must use the subframe anchor");
        l.layout_content(&Content::text("B"));
        let outer = positions(&l.finish());
        assert!(
            (outer[0].1 - 62.8).abs() < 1e-8,
            "parent RTL anchor must survive the child region"
        );
    }

    #[test]
    fn source_bidi_spans_survive_wraps_translation_and_paragraph_boundaries() {
        use crate::entities::{bidi_context::ParagraphBidiSegment, dir::Dir};
        for source in [
            "#set text(dir: rtl)\nABC aa aa مرحبا",
            "#set text(dir: ltr)\n123 456 مرحبا",
            "#set text(dir: rtl)\n#text(dir: ltr)[ABC 123] مرحبا\n\nعالم",
        ] {
            let intr = TagIntrospector::empty();
            let mut l = make(&intr, false);
            l.layout_content(&audit_content(source));
            let doc = l.finish();
            let mut count = 0;
            for page in &doc.pages {
                for item in &page.items {
                    if let FrameItem::Text { text, bidi: Some(span), .. } = item {
                        let ParagraphBidiSegment::Text { text: original, .. } =
                            &span.input().segments()[span.segment()]
                        else {
                            panic!("text span must refer to text");
                        };
                        assert_eq!(original.get(span.range()).unwrap(), text.as_str());
                        count += 1;
                        assert!(matches!(span.input().base_dir(), Dir::RTL | Dir::LTR));
                    }
                }
            }
            assert!(count >= 3, "missing complete paragraph context for {source}");
        }
    }

    pub(super) fn audit_content(src: &str) -> Content {
        use crate::{
            compiler::eval::eval_for_test,
            contracts::world::World,
            entities::{
                file_id::FileId,
                font_book::FontBook,
                source::Source,
                world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
            },
        };
        use std::num::NonZeroU16;

        struct MockWorld {
            library: Library,
            book: FontBook,
            source: Source,
        }

        impl MockWorld {
            fn new(text: &str) -> Self {
                let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
                Self {
                    library: Library::new(),
                    book: FontBook::new(),
                    source: Source::new(id, text.to_string()),
                }
            }
        }

        impl World for MockWorld {
            fn library(&self) -> &Library {
                &self.library
            }
            fn book(&self) -> &FontBook {
                &self.book
            }
            fn main(&self) -> FileId {
                self.source.id()
            }
            fn source(&self, _: FileId) -> FileResult<Source> {
                Ok(self.source.clone())
            }
            fn file(&self, _: FileId) -> FileResult<Bytes> {
                Err(FileError::NotFound)
            }
            fn font(&self, _: usize) -> Option<Font> {
                None
            }
            fn today(
                &self,
                _: Option<crate::entities::duration::Duration>,
            ) -> Option<Datetime> {
                None
            }
        }

        let world = MockWorld::new(src);
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        content.clone()
    }

    #[test]
    fn actual_set_scope_retains_anchor_after_the_leading_newline() {
        let intr = TagIntrospector::empty();
        for source in [
            "#set text(dir: rtl)\nمرحبا\\\n#text(dir: ltr)[B]\\\nعالم",
            "#set text(dir: rtl)\n#text(dir: ltr)[B]",
            "#set page(width: 70pt, margin: 0pt)\n#set text(dir: rtl)\nمرحبا\\\n#text(dir: ltr)[B]",
        ] {
            let mut l = make(&intr, false);
            l.layout_content(&audit_content(source));
            let p = positions(&l.finish());
            let b = p.iter().find(|p| p.0 == "B").unwrap();
            let expected = if source.contains("مرحبا") { 62.8 } else { 0.0 };
            assert!(
                (b.1 - expected).abs() < 1e-8,
                "source scopes must determine the paragraph anchor: {b:?}"
            );
        }
    }

    pub(super) fn make(
        intr: &dyn Introspector,
        justify: bool,
    ) -> Layouter<'_, FixedMetrics, NullImageSizer> {
        let mut l = Layouter::new(FixedMetrics, NullImageSizer, 12.0, intr.track());
        l.chain = l.chain.push_styles(&Styles::from_iter([Style::Size(
            crate::entities::layout_types::Length::pt(12.0),
        )]));
        l.chain = l
            .chain
            .push_custom("par.justify", Value::Bool(justify))
            .push_custom("text.hyphenate", Value::Bool(false));
        l.style = TextStyle::from(&l.chain);
        l.page_config.width = 70.0;
        l.page_config.height = 400.0;
        l.page_config.margin = PageMargins::uniform(0.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(0.0);
        l.regions.current.width = 70.0;
        l.regions.current.height = 400.0;
        l.regions.current.cursor_x = Pt(0.0);
        l.regions.current.line_start_x = Pt(0.0);
        l.regions.current.cursor_y = Pt(0.0);
        l
    }
    fn texts(items: &[FrameItem], out: &mut Vec<(String, f64, f64)>) {
        for item in items {
            match item {
                FrameItem::Text { text, pos, .. } if !text.is_empty() => {
                    out.push((text.to_string(), pos.x.0, pos.y.0))
                }
                FrameItem::TextReplacement { items, .. }
                | FrameItem::Semantic { items, .. }
                | FrameItem::Group { items, .. }
                | FrameItem::Link { items, .. } => texts(items, out),
                FrameItem::TextSeparator { .. }
                | FrameItem::Text { .. }
                | FrameItem::TextShaped { .. }
                | FrameItem::Line { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. }
                | FrameItem::Shape { .. } => {}
            }
        }
    }
    pub(super) fn positions(
        doc: &crate::entities::layout_types::PagedDocument,
    ) -> Vec<(String, f64, f64)> {
        let mut out = Vec::new();
        for p in &doc.pages {
            texts(&p.items, &mut out)
        }
        out
    }
    #[test]
    fn composition_justifies_only_material_continuation() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, true);
        l.layout_content(&Content::text("aa bb ccccccc"));
        let p = positions(&l.finish());
        let a = p.iter().find(|(s, _, _)| s == "aa").unwrap();
        let b = p.iter().find(|(s, _, _)| s == "bb").unwrap();
        let last = p.iter().find(|(s, _, _)| s == "ccccccc").unwrap();
        assert!((a.1 - 0.0).abs() < 1e-8);
        assert!((b.1 + 14.4 - 70.0).abs() < 1e-8);
        assert_eq!(a.2, b.2);
        assert_eq!(last.1, 0.0);
        assert!(last.1 + 50.4 < 70.0);
        assert!(last.2 > b.2);
    }
    #[test]
    fn composition_final_literal_spaces_keep_natural_width() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, true);
        l.layout_content(&Content::text("aa bb                 "));
        let p = positions(&l.finish());
        assert_eq!(p.len(), 2);
        assert!((p[1].1 - 21.6).abs() < 1e-8);
        assert_eq!(p[0].2, p[1].2);
    }
    #[test]
    fn composition_native_par_has_before_and_after_boundaries() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.layout_content(&Content::text("A"));
        l.layout_content(&Content::par(Content::text("B")));
        l.layout_content(&Content::text("C"));
        let p = positions(&l.finish());
        assert_eq!(p.len(), 3);
        for item in &p {
            assert_eq!(item.1, 0.0)
        }
        assert!((p[1].2 - p[0].2 - 22.8).abs() < 1e-8);
        assert!((p[2].2 - p[1].2 - 22.8).abs() < 1e-8);
    }
    #[test]
    fn composition_empty_paragraph_reserves_spacing_without_ink() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.layout_content(&Content::par(Content::Empty));
        l.layout_content(&Content::text("A"));
        let p = positions(&l.finish());
        assert_eq!(p.len(), 1);
        assert!((p[0].2 - 22.8).abs() < 1e-8);
    }
    #[test]
    fn composition_terminal_paragraph_auto_height_is_causal() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.page_config.height = f64::INFINITY;
        l.regions.current.height = f64::INFINITY;
        l.layout_content(&Content::par(Content::text("A")));
        let doc = l.finish();
        assert_eq!(doc.pages.len(), 1);
        assert!((doc.pages[0].height - 8.4).abs() < 1e-8);
    }
    #[test]
    fn composition_empty_terminal_auto_height_keeps_explicit_spacing() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.page_config.height = f64::INFINITY;
        l.regions.current.height = f64::INFINITY;
        l.layout_content(&Content::text("A"));
        l.layout_content(&Content::par(Content::Empty));
        let doc = l.finish();
        assert_eq!(doc.pages.len(), 1);
        assert!((doc.pages[0].height - 22.8).abs() < 1e-8);
    }
    #[test]
    fn composition_rollback_restores_terminal_paragraph_geometry_and_prefix() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, true);
        l.page_config.height = f64::INFINITY;
        l.regions.current.height = f64::INFINITY;
        l.layout_content(&Content::par(Content::text("A")));
        assert!(!super::super::html_frame::items_have_material(
            &l.regions.current.current_line
        ));
        let checkpoint = l.capture_flow_suffix_checkpoint();
        l.layout_content(&Content::text("trial words overflow more"));
        l.restore_flow_suffix_checkpoint(checkpoint);
        let doc = l.finish();
        assert_eq!(
            positions(&doc).iter().map(|p| p.0.as_str()).collect::<Vec<_>>(),
            vec!["A"]
        );
        assert_eq!(doc.pages.len(), 1);
        assert!((doc.pages[0].height - 8.4).abs() < 1e-8);
    }
    #[test]
    fn composition_empty_styled_terminal_auto_height_keeps_explicit_spacing() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.page_config.height = f64::INFINITY;
        l.regions.current.height = f64::INFINITY;
        l.layout_content(&Content::text("A"));
        l.layout_content(&Content::styled(
            Content::par(Content::Empty),
            Styles::new().push_custom(
                "par.leading",
                Value::Length(crate::entities::layout_types::Length::pt(2.0)),
            ),
        ));
        let doc = l.finish();
        assert!((doc.pages[0].height - 22.8).abs() < 1e-8);
    }

    #[test]
    fn composition_empty_only_auto_height_discards_weak_spacing() {
        for (body, height) in [(Content::Empty, 0.0), (Content::text(""), 8.4)] {
            let intr = TagIntrospector::empty();
            let mut l = make(&intr, false);
            l.page_config.height = f64::INFINITY;
            l.regions.current.height = f64::INFINITY;
            l.layout_content(&Content::par(body));
            let doc = l.finish();
            assert!(
                (doc.pages[0].height - height).abs() < 1e-8,
                "{:?}",
                doc.pages[0].height
            );
        }
    }
    #[test]
    fn composition_empty_text_reserves_virtual_top_edge() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.layout_content(&Content::par(Content::text("")));
        l.layout_content(&Content::text("A"));
        let p = positions(&l.finish());
        assert!((p[0].2 - 31.2).abs() < 1e-8);
    }
}

#[cfg(test)]
mod global_selection_tests {
    use super::tests::{make, positions};
    use crate::entities::{
        content::Content, introspector::TagIntrospector, value::Value,
    };
    fn lines(strategy: &str) -> Vec<String> {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.regions.current.width = 124.0;
        l.page_config.width = 124.0;
        l.chain = l.chain.push_custom("par.linebreaks", Value::Str(strategy.into()));
        l.layout_content(&Content::par(Content::text("a bbbbb cc ddddddd eeeee ff g hhhhhhh iiiiiiiii jjjjjjjjj kkkkkk llllll m nnn ooo")));
        let words = positions(&l.finish());
        let mut lines: Vec<(f64, Vec<String>)> = Vec::new();
        for (word, _, y) in words {
            if lines.last().is_none_or(|line| line.0 != y) {
                lines.push((y, Vec::new()));
            }
            lines.last_mut().unwrap().1.push(word);
        }
        lines.into_iter().map(|(_, w)| w.join(" ")).collect()
    }
    #[test]
    fn global_selection_strategy_changes_breaks_under_identical_metrics() {
        let simple = lines("simple");
        let optimized = lines("optimized");
        assert_eq!(simple.join(" "), optimized.join(" "));
        assert_ne!(simple, optimized, "linebreaks must not be discarded");
    }
}

#[cfg(test)]
mod configured_scope_tests {
    use super::tests::{make, positions};
    use crate::entities::{
        content::Content, introspector::TagIntrospector, style::Styles, value::Value,
    };
    fn run(outer: bool) -> Vec<(String, f64, f64)> {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.regions.current.width = 124.0;
        l.page_config.width = 124.0;
        let text=Content::text("a bbbbb cc ddddddd eeeee ff g hhhhhhh iiiiiiiii jjjjjjjjj kkkkkk llllll m nnn ooo");
        let styled = Content::styled(
            text,
            Styles::new().push_custom("par.linebreaks", Value::Str("optimized".into())),
        );
        let content = if outer {
            Content::sequence(vec![Content::Empty, styled, Content::SpaceFromNewline])
        } else {
            styled
        };
        l.layout_content(&content);
        assert!(!l.paragraph_planning_suppressed);
        positions(&l.finish())
    }
    #[test]
    fn global_selection_terminal_scope_after_configuration_is_complete() {
        assert_eq!(run(true), run(false));
    }
}

#[cfg(test)]
mod guard_checkpoint_tests {
    use super::tests::make;
    use crate::entities::{content::Content, introspector::TagIntrospector};
    #[test]
    fn paragraph_planning_guard_survives_rejected_suffix() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.layout_content(&Content::par(Content::text("prefix")));
        l.paragraph_planning_suppressed = true;
        let saved = l.capture_flow_suffix_checkpoint();
        l.paragraph_planning_suppressed = false;
        l.layout_content(&Content::text("suffix"));
        l.restore_flow_suffix_checkpoint(saved);
        assert!(l.paragraph_planning_suppressed);
        assert_eq!(
            super::tests::positions(&l.finish())
                .into_iter()
                .map(|p| p.0)
                .collect::<Vec<_>>(),
            vec!["prefix"]
        );
    }
}

#[cfg(test)]
mod multi_paragraph_selection_tests {
    use super::tests::{make, positions};
    use crate::entities::{
        content::Content, introspector::TagIntrospector, value::Value,
    };
    fn lines(explicit: bool) -> Vec<String> {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.regions.current.width = 124.0;
        l.page_config.width = 124.0;
        l.chain = l.chain.push_custom("par.linebreaks", Value::Str("optimized".into()));
        let body=Content::text("a bbbbb cc ddddddd eeeee ff g hhhhhhh iiiiiiiii jjjjjjjjj kkkkkk llllll m nnn ooo");
        let content = if explicit {
            Content::sequence(vec![Content::par(body.clone()), Content::par(body)])
        } else {
            Content::sequence(vec![body.clone(), Content::Parbreak, body])
        };
        l.layout_content(&content);
        let words = positions(&l.finish());
        let mut lines: Vec<(f64, Vec<String>)> = Vec::new();
        for (w, _, y) in words {
            if lines.last().is_none_or(|l| l.0 != y) {
                lines.push((y, Vec::new()));
            }
            lines.last_mut().unwrap().1.push(w);
        }
        lines.into_iter().map(|(_, w)| w.join(" ")).collect()
    }
    #[test]
    fn global_selection_prepares_each_complete_paragraph_before_emitting() {
        assert_eq!(lines(false), lines(true));
    }
}

#[cfg(test)]
mod global_hyphenation_origin_tests {
    use super::tests::{make, positions};
    use crate::entities::{
        content::Content, introspector::TagIntrospector, value::Value,
    };
    #[test]
    fn global_hyphenation_splits_overlong_word_at_line_origin() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.chain = l
            .chain
            .push_custom("text.hyphenate", Value::Bool(true))
            .push_custom("par.linebreaks", Value::Str("simple".into()));
        l.layout_content(&Content::par(Content::text("representation")));
        let positions = positions(&l.finish());
        assert!(positions.iter().any(|(text, _, _)| text.contains('-')));
        assert_eq!(
            positions
                .iter()
                .map(|(t, _, _)| t.replace('-', ""))
                .collect::<String>(),
            "representation"
        );
        assert!(positions.last().unwrap().2 > positions[0].2);
    }
}

#[cfg(test)]
mod v16_paragraph_tests {
    use super::tests::{make, positions};
    use super::*;
    use crate::entities::{
        content::SourceContent,
        introspector::TagIntrospector,
        layout_types::PagedDocument,
        span::Span,
        style::{Style, Styles},
    };
    use std::sync::Arc;

    fn source(body: Content) -> Content {
        Content::Source(Arc::new(SourceContent { body, span: Span::detached() }))
    }

    fn render(parts: Vec<Content>) -> PagedDocument {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.page_config.height = f64::INFINITY;
        l.regions.current.height = f64::INFINITY;
        for part in parts {
            l.layout_content(&part);
        }
        l.finish()
    }

    fn same_geometry(actual: &PagedDocument, expected: &PagedDocument) {
        assert_eq!(actual.pages.len(), expected.pages.len());
        for (a, e) in actual.pages.iter().zip(&expected.pages) {
            assert!(
                (a.height - e.height).abs() < 1e-8,
                "height {} versus {}",
                a.height,
                e.height
            );
        }
        let a = positions(actual);
        let e = positions(expected);
        assert_eq!(a.len(), e.len());
        for (a, e) in a.iter().zip(e.iter()) {
            assert_eq!(a.0, e.0);
            assert!((a.1 - e.1).abs() < 1e-8);
            assert!((a.2 - e.2).abs() < 1e-8, "baseline {} versus {}", a.2, e.2);
        }
    }

    #[test]
    fn v16_paragraph_source_empty_before_text_keeps_spacing() {
        let doc = render(vec![Content::par(source(Content::Empty)), Content::text("A")]);
        assert!((positions(&doc)[0].2 - 22.8).abs() < 1e-8);
    }

    #[test]
    fn v16_paragraph_source_empty_after_text_keeps_auto_height() {
        let doc = render(vec![Content::text("A"), Content::par(source(Content::Empty))]);
        assert!((doc.pages[0].height - 22.8).abs() < 1e-8);
    }

    #[test]
    fn v16_paragraph_source_space_sequence_keeps_spacing() {
        let body = Content::Sequence(Arc::from([
            source(Content::Space),
            source(Content::SpaceFromNewline),
        ]));
        let doc = render(vec![Content::par(body), Content::text("A")]);
        assert!((positions(&doc)[0].2 - 22.8).abs() < 1e-8);
    }

    #[test]
    fn v16_paragraph_nested_source_empty_text_keeps_virtual_top_once() {
        let body =
            source(source(Content::styled(source(Content::text("")), Styles::new())));
        let actual = render(vec![Content::par(body), Content::text("A")]);
        assert!((positions(&actual)[0].2 - 31.2).abs() < 1e-8);
    }

    #[test]
    fn v16_paragraph_source_empty_only_collapses_weak_spacing() {
        let doc = render(vec![Content::par(source(Content::Empty))]);
        assert!((doc.pages[0].height).abs() < 1e-8);
        assert!(positions(&doc).is_empty());
    }

    #[test]
    fn v16_paragraph_source_empty_text_only_keeps_top_edge() {
        let doc = render(vec![Content::par(source(Content::text("")))]);
        assert!((doc.pages[0].height - 8.4).abs() < 1e-8);
        assert!(positions(&doc).is_empty());
    }

    #[test]
    fn v16_paragraph_source_styled_boundary_closes_before_size() {
        let par = Content::par(Content::text("B"));
        let styles = Styles::from_iter([Style::Size(
            crate::entities::layout_types::Length::pt(18.0),
        )]);
        let expected = render(vec![
            Content::text("A"),
            Content::styled(par.clone(), styles.clone()),
            Content::text("C"),
        ]);
        let actual = render(vec![
            Content::text("A"),
            Content::styled(source(par), styles),
            Content::text("C"),
        ]);
        same_geometry(&actual, &expected);
    }

    #[test]
    fn v16_paragraph_source_styled_empty_boundary_restores_scope() {
        let par = Content::par(Content::Empty);
        let styles = Styles::from_iter([Style::Size(
            crate::entities::layout_types::Length::pt(18.0),
        )]);
        let expected = render(vec![
            Content::text("A"),
            Content::styled(par.clone(), styles.clone()),
            Content::text("C"),
        ]);
        let actual = render(vec![
            Content::text("A"),
            Content::styled(source(par), styles),
            Content::text("C"),
        ]);
        same_geometry(&actual, &expected);
    }

    #[test]
    fn v16_paragraph_source_material_and_inline_styles_remain_material() {
        let body = Content::styled(
            Content::text("B"),
            Styles::from_iter([Style::Size(crate::entities::layout_types::Length::pt(
                18.0,
            ))]),
        );
        let expected = render(vec![
            Content::text("A"),
            Content::par(body.clone()),
            Content::text("C"),
        ]);
        let actual = render(vec![
            Content::text("A"),
            Content::par(source(body)),
            Content::text("C"),
        ]);
        same_geometry(&actual, &expected);
    }

    fn language(source: &str) -> PagedDocument {
        render(vec![super::tests::audit_content(source)])
    }

    #[test]
    fn v16_paragraph_empty_sizes_collapse_both_sides() {
        for (size, delta) in [(8, 37.2), (18, 51.6)] {
            let doc = language(&format!("A#text(size:{size}pt)[#par[]]C"));
            let p = positions(&doc);
            assert_eq!(p.iter().map(|p| p.0.as_str()).collect::<Vec<_>>(), ["A", "C"]);
            assert!((p[1].2 - p[0].2 - delta).abs() < 1e-8, "size {size}: {p:?}");
        }
    }

    #[test]
    fn v16_paragraph_empty_initial_sizes_collapse_with_following_text() {
        for (size, baseline) in [(8, 22.8), (18, 30.0)] {
            let doc = language(&format!("#text(size:{size}pt)[#par[]]C"));
            assert!((positions(&doc)[0].2 - baseline).abs() < 1e-8);
        }
    }

    #[test]
    fn v16_paragraph_two_empty_nodes_keep_internal_gap_without_ink() {
        for text in [
            "#text(size:8pt)[#par[]]#text(size:18pt)[#par[]]",
            "#text(size:18pt)[#par[]]#text(size:8pt)[#par[]]",
        ] {
            let doc = language(text);
            assert!(positions(&doc).is_empty());
            assert!((doc.pages[0].height - 21.6).abs() < 1e-8);
        }
    }

    #[test]
    fn v16_paragraph_two_empty_text_nodes_keep_virtual_tops() {
        let doc = language(
            r#"#text(size:8pt)[#par[#text("")]]#text(size:18pt)[#par[#text("")]]"#,
        );
        assert!(positions(&doc).is_empty());
        assert!((doc.pages[0].height - (5.6 + 21.6 + 12.6)).abs() < 1e-8);
    }

    #[test]
    fn v16_paragraph_spacing_constructor_controls() {
        for (text, delta) in [
            ("A#par(spacing:30pt)[]C", 68.4),
            ("A#text(size:18pt)[#par(spacing:2em)[]]C", 80.4),
        ] {
            let doc = language(text);
            let p = positions(&doc);
            assert!((p[1].2 - p[0].2 - delta).abs() < 1e-8, "{p:?}");
        }
    }

    #[test]
    fn v16_paragraph_empty_terminals_reserve_only_above_gap() {
        for (text, height) in [
            ("A#text(size:8pt)[#par[]]", 22.8),
            ("A#text(size:18pt)[#par[]]", 30.0),
            ("A#par(spacing:30pt)[]", 38.4),
        ] {
            let doc = language(text);
            assert!((doc.pages[0].height - height).abs() < 1e-8);
        }
    }

    #[test]
    fn v16_paragraph_empty_between_box_and_text_preserves_geometry() {
        let source = language("#box(width:8pt,height:7pt)[]#par[]C");
        assert!(positions(&source)[0].2 > 28.8);
        assert_eq!(positions(&source)[0].0, "C");
    }

    #[test]
    fn v16_paragraph_empty_gap_zero_negative_length() {
        let expected = language("A#par[]C");
        for text in ["A#par(spacing:0pt)[]C", "A#par(spacing:-5pt)[]C"] {
            same_geometry(&language(text), &expected);
        }
    }

    #[test]
    fn v16_paragraph_empty_text_uses_local_font_size() {
        for (size, baseline) in [(8, 28.4), (18, 35.4)] {
            let doc = language(&format!(r#"#par[#text(size:{size}pt, "")]C"#));
            assert!((positions(&doc)[0].2 - baseline).abs() < 1e-8);
        }
    }

    #[test]
    fn v16_paragraph_empty_text_uses_local_top_edge() {
        for (edge, baseline) in [(0, 22.8), (-5, 22.8), (20, 42.8)] {
            let doc = language(&format!(r#"#par[#text(top-edge:{edge}pt, "")]C"#));
            assert!((positions(&doc)[0].2 - baseline).abs() < 1e-8);
        }
    }

    #[test]
    fn v16_paragraph_empty_text_keeps_local_descent() {
        let doc = language(r#"#par[#text(bottom-edge:-10pt, "")]C"#);
        assert!((positions(&doc)[0].2 - 41.2).abs() < 1e-8);
        let only = language(r#"#par[#text(bottom-edge:-10pt, "")]"#);
        assert!((only.pages[0].height - 18.4).abs() < 1e-8);
    }

    #[test]
    fn v16_paragraph_empty_text_uses_last_fallback_style() {
        let doc =
            language(r#"#par[#text(top-edge:20pt, "")#text(bottom-edge:-10pt, "")]C"#);
        assert!((positions(&doc)[0].2 - 41.2).abs() < 1e-8);
        let reverse = language(r#"#par[#text(size:18pt, "")#text(size:8pt, "")]C"#);
        assert!((positions(&reverse)[0].2 - 28.4).abs() < 1e-8);
    }
}
