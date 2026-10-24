//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/bidi_origin.md
//! @prompt-hash 32762c87
//! @layer L1
use super::{FontMetrics, ImageSizer, Layouter};
use crate::entities::{
    bidi_context::{BidiSpan, ParagraphBidiInput, ParagraphBidiSegment},
    content::Content,
    dir::Dir,
    layout_types::TextSeparatorOrigin,
    style_chain::StyleChain,
    value::Value,
};
use std::sync::Arc;
#[derive(Clone)]
pub(super) struct Cursor {
    input: Arc<ParagraphBidiInput>,
    next: usize,
}
impl Cursor {
    pub(super) fn next(&mut self) -> Option<BidiSpan> {
        let segment = self.next;
        self.next += 1;
        let len = match self.input.segments().get(segment)? {
            ParagraphBidiSegment::Text { text, .. } => text.len(),
            ParagraphBidiSegment::Space { .. } => 1,
            ParagraphBidiSegment::Linebreak => return None,
        };
        BidiSpan::new(self.input.clone(), segment, 0..len)
    }
}

fn collect(
    c: &Content,
    chain: &StyleChain,
    out: &mut Vec<ParagraphBidiSegment>,
) -> Option<()> {
    if chain.tracking().is_some()
        || chain.highlight().is_some()
        || chain.subscript()
        || chain.superscript()
        || chain.custom("text.baseline").is_some()
        || chain.custom("par.line.numbering").is_some()
        || chain.custom("pdf.marker.tag").is_some()
        || chain.custom("outline.link.destination").is_some()
    {
        return None;
    }
    if matches!(chain.custom("text.hyphenate"), Some(Value::Bool(true)))
        || matches!(chain.custom("par.justify"), Some(Value::Bool(true)))
        || matches!(chain.custom("par.linebreaks"),Some(Value::Str(s)) if s.as_str()=="optimized")
        || chain.custom("text.tracking").is_some()
    {
        return None;
    }
    if let Content::Text(text) = c {
        if text.contains(['\n', '\r', '\t', '\u{2028}', '\u{2029}'])
            || text.split(' ').any(super::word_segment::word_needs_line_segmentation)
        {
            return None;
        }
        let dir = chain
            .custom("text.dir")
            .and_then(|v| if let Value::Dir(d) = v { Some(*d) } else { None })
            .unwrap_or(Dir::LTR);
        if !dir.is_horizontal() {
            return None;
        }
        out.push(ParagraphBidiSegment::Text { text: text.clone(), dir });
        return Some(());
    }
    if matches!(c, Content::Empty) {
        return Some(());
    }
    if matches!(c, Content::Space) {
        out.push(ParagraphBidiSegment::Space { origin: TextSeparatorOrigin::Markup });
        return Some(());
    }
    if matches!(c, Content::SpaceFromNewline) {
        out.push(ParagraphBidiSegment::Space { origin: TextSeparatorOrigin::Newline });
        return Some(());
    }
    if let Content::Linebreak(elem) = c {
        if elem.justify {
            return None;
        }
        out.push(ParagraphBidiSegment::Linebreak);
        return Some(());
    }
    if let Content::Source(source) = c {
        return collect(&source.body, chain, out);
    }
    if let Content::Located(located) = c {
        return collect(&located.body, chain, out);
    }
    if let Content::Sequence(parts) = c {
        for p in parts.iter() {
            collect(p, chain, out)?;
        }
        return Some(());
    }
    if let Content::Styled(styled) = c {
        return collect(&styled.body, &chain.push_styles(&styled.styles), out);
    }
    if let Content::Strong(elem) = c {
        return collect(&elem.body, chain, out);
    }
    if let Content::Emph(elem) = c {
        return collect(&elem.body, chain, out);
    }
    None
}
pub(super) fn prepare<M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
    parts: &[Content],
    base: Dir,
) -> Option<Cursor> {
    let certified_cell = l.cell_bidi_allowed && l.regions.cell.is_some();
    if (l.is_sub_frame && !certified_cell)
        || l.column_mode
        || l.page_columns.is_some()
        || (l.regions.cell.is_some() && !certified_cell)
        || l.smallcaps
        || l.decoration_lines_collector.is_some()
        || !l.floats_pending.is_empty()
        || !l.regions.current.width.is_finite()
        || {
            let style = super::text::effective_style(l);
            style.tracking.is_some()
                || style.highlight.is_some()
                || style.subscript
                || style.superscript
                || style.baseline_offset.resolve_pt(style.size.0) != 0.0
        }
    {
        return None;
    }
    let mut segments = Vec::new();
    for c in parts {
        collect(c, &l.chain, &mut segments)?;
    }
    let needed=base==Dir::RTL || segments.iter().any(|s|matches!(s,ParagraphBidiSegment::Text{text,dir} if *dir!=base || text.chars().any(|c|matches!(c,'\u{0590}'..='\u{08ff}'|'\u{200f}'|'\u{202a}'..='\u{202e}'|'\u{2066}'..='\u{2069}'))));
    if !needed {
        return None;
    }
    Some(Cursor {
        input: Arc::new(ParagraphBidiInput::new(base, segments)?),
        next: 0,
    })
}

/// A genuine Sequence/Par keeps its existing complete-flow entry authority.
fn owns_root_scope(content: &Content) -> bool {
    match content {
        Content::Source(source) => owns_root_scope(&source.body),
        Content::Located(located) => owns_root_scope(&located.body),
        Content::Styled(styled) => owns_root_scope(&styled.body),
        Content::Strong(elem) => owns_root_scope(&elem.body),
        Content::Emph(elem) => owns_root_scope(&elem.body),
        Content::Sequence(..) | Content::Par { .. } => false,
        Content::Realized(..)
        | Content::ShowRecipe(..)
        | Content::Empty
        | Content::Text(..)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::HtmlElem(..)
        | Content::HtmlFrame(..)
        | Content::Heading(..)
        | Content::Title(..)
        | Content::Raw(..)
        | Content::ListItem(..)
        | Content::EnumItem(..)
        | Content::Link(..)
        | Content::Equation(..)
        | Content::MathSequence(..)
        | Content::Symbol(..)
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
        | Content::Table(..) => true,
    }
}

/// Certify the complete original singleton root, or guard its whole refusal.
pub(super) fn at_root<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<'_, M, S>,
    content: &Content,
    dispatch: impl FnOnce(&mut Layouter<'_, M, S>, &Content),
) {
    if l.bidi_scope_active || l.bidi_cursor.is_some() || !owns_root_scope(content) {
        dispatch(l, content);
        return;
    }
    let previous_dir = l.active_paragraph_dir;
    let previous_cursor = l.bidi_cursor.take();
    let previous_scope = l.bidi_scope_active;
    if previous_dir.is_none() && l.pending_line_dir.is_none() {
        let parts = std::slice::from_ref(content);
        let direction = super::paragraph::shared_direction(parts, &l.chain);
        let cursor = direction.and_then(|dir| prepare(l, parts, dir));
        if cursor.is_some() {
            l.active_paragraph_dir = direction;
            l.bidi_cursor = cursor;
        }
    }
    // Even an opaque root owns its refusal; descendants cannot recertify it.
    l.bidi_scope_active = true;
    dispatch(l, content);
    if l.bidi_cursor.is_some()
        && super::html_frame::items_have_material(&l.regions.current.current_line)
    {
        if let Some(direction) = l.active_paragraph_dir {
            l.pending_line_dir = Some(direction);
        }
    }
    l.active_paragraph_dir = previous_dir;
    l.bidi_cursor = previous_cursor;
    l.bidi_scope_active = previous_scope;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
    };
    use comemo::Track;
    #[test]
    fn complete_input_keeps_utf8_and_repeated_occurrences() {
        let intr = TagIntrospector::empty();
        let l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            12.,
            (&intr as &dyn Introspector).track(),
        );
        let parts = [Content::text("é aa aa"), Content::Space, Content::text("مرحبا")];
        let mut c = prepare(&l, &parts, Dir::RTL).unwrap();
        let a = c.next().unwrap();
        assert_eq!(a.range(), 0..8);
        assert_eq!(a.subspan(3..5).unwrap().range(), 3..5);
        assert_eq!(a.subspan(6..8).unwrap().range(), 6..8);
        assert_eq!(c.next().unwrap().segment(), 1);
        assert_eq!(c.next().unwrap().segment(), 2);
        assert!(c.next().is_none());
        assert!(prepare(&l, &[Content::text("x"), Content::Parbreak], Dir::RTL).is_none());
    }
    #[test]
    fn checkpoint_and_subframe_conserve_cursor_without_certifying_inner_region() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            12.,
            (&intr as &dyn Introspector).track(),
        );
        l.bidi_cursor = prepare(
            &l,
            &[Content::text("aa"), Content::Space, Content::text("aa")],
            Dir::RTL,
        );
        l.bidi_scope_active = true;
        l.text_bidi = l.bidi_cursor.as_mut().unwrap().next();
        let checkpoint = l.capture_flow_suffix_checkpoint();
        l.bidi_cursor.as_mut().unwrap().next();
        l.text_bidi = None;
        l.bidi_scope_active = false;
        l.restore_flow_suffix_checkpoint(checkpoint);
        assert!(l.bidi_scope_active);
        assert_eq!(l.text_bidi.as_ref().unwrap().segment(), 0);
        let (_, items, _, _, _) = l.layout_sub_frame(
            &Content::text("مرحبا"),
            super::super::sub_frame::SubLayoutRegion {
                origin_x: 0.,
                width: 100.,
                height: None,
                align_rtl: false,
                unconstrained_height: true,
            },
        );
        assert!(items.iter().all(|item| match item {
            crate::entities::layout_types::FrameItem::Text { bidi, .. }
            | crate::entities::layout_types::FrameItem::TextSeparator { bidi, .. }
            | crate::entities::layout_types::FrameItem::TextShaped { bidi, .. } =>
                bidi.is_none(),
            _ => true,
        }));
        assert_eq!(l.bidi_cursor.as_mut().unwrap().next().unwrap().segment(), 1);
        assert_eq!(l.text_bidi.as_ref().unwrap().range(), 0..2);
    }
    #[test]
    fn typed_transforming_styles_refuse_the_entire_certificate() {
        use crate::entities::{
            layout_types::{Color, Length},
            style::{Style, Styles},
        };
        let intr = TagIntrospector::empty();
        let l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            12.,
            (&intr as &dyn Introspector).track(),
        );
        for style in [
            Style::Tracking(Length::pt(1.)),
            Style::Highlight(Some(Color::rgb(255, 0, 0))),
            Style::Superscript(true),
        ] {
            let mut styles = Styles::new();
            styles.push(style);
            let content = Content::styled(Content::text("مرحبا"), styles);
            assert!(prepare(&l, std::slice::from_ref(&content), Dir::RTL).is_none());
        }
    }
    #[test]
    fn styled_envelopes_outside_flat_text_refuse_the_whole_input() {
        use crate::entities::style::Styles;
        let intr = TagIntrospector::empty();
        let l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            12.,
            (&intr as &dyn Introspector).track(),
        );
        for (key, value) in
            [("pdf.marker.tag", "label"), ("outline.link.destination", "target")]
        {
            let c = Content::styled(
                Content::text("مرحبا"),
                Styles::new().push_custom(key, Value::Str(value.into())),
            );
            assert!(prepare(&l, std::slice::from_ref(&c), Dir::RTL).is_none());
        }
    }
}
