//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/paragraph_plan.md
//! @prompt-hash 2e32224c
//! @layer L1
//! @updated 2026-10-08

use super::{
    html_frame, paragraph_emit, paragraph_prepare,
    paragraph_select::{self, Strategy},
    FontMetrics, ImageSizer, Layouter,
};
use crate::entities::{content::Content, value::Value};
fn strategy<M: FontMetrics, S: ImageSizer>(l: &Layouter<M, S>) -> Option<Strategy> {
    match l.chain.custom("par.linebreaks") {
        Some(Value::Str(s)) if s.as_str() == "simple" => Some(Strategy::Simple),
        Some(Value::Str(s)) if s.as_str() == "optimized" => Some(Strategy::Optimized),
        Some(Value::Auto) => {
            Some(if matches!(l.chain.custom("par.justify"), Some(Value::Bool(true))) {
                Strategy::Optimized
            } else {
                Strategy::Simple
            })
        }
        None => {
            Some(if matches!(l.chain.custom("par.justify"), Some(Value::Bool(true))) {
                Strategy::Optimized
            } else {
                Strategy::Simple
            })
        }
        _ => None,
    }
}
fn integral_styled(parts: &[Content]) -> bool {
    let mut material = parts.iter().filter(|part| {
        !matches!(part, Content::Empty | Content::Space | Content::SpaceFromNewline)
    });
    matches!(material.next(), Some(Content::Styled(_))) && material.next().is_none()
}
fn defer_integral_styled(body: &Content) -> bool {
    matches!(body, Content::Styled(_))
        || matches!(body,Content::Sequence(parts) if integral_styled(parts))
}
fn implicit_regional_only<M: FontMetrics, S: ImageSizer>(l: &Layouter<M, S>) -> bool {
    l.chain.custom("par.linebreaks").is_none()
        && !matches!(l.chain.custom("par.justify"), Some(Value::Bool(true)))
        && !matches!(l.chain.custom("text.hyphenate"), Some(Value::Bool(true)))
}
fn single_overlong_word(p: &paragraph_prepare::Prepared, width: f64) -> bool {
    matches!(p.words.as_slice(), [word] if word.before.is_empty() && word.width > width)
}
fn options<M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
) -> Option<(Strategy, f64, f64, bool)> {
    let strategy = strategy(l)?;
    let width = l.regions.current.width
        - l.page_config.margin.right
        - l.regions.current.line_start_x.0;
    if !width.is_finite()
        || width <= 0.0
        || l.column_mode
        || l.page_columns.is_some()
        || l.regions.cell.is_some()
        || !l.floats_pending.is_empty()
        || l.is_sub_frame
        || l.smallcaps
        || html_frame::items_have_material(&l.regions.current.current_line)
        || l.regions.current.cursor_x != l.regions.current.line_start_x
    {
        return None;
    }
    let justify = matches!(l.chain.custom("par.justify"), Some(Value::Bool(true)));
    let font = super::text::effective_style(l).size.0;
    if !font.is_finite() || font <= 0.0 {
        return None;
    }
    Some((strategy, width, font, justify))
}
fn select_product(
    p: paragraph_prepare::Prepared,
    options: (
        Strategy,
        super::paragraph_indent::Shape,
        f64,
        bool,
        crate::compiler::text_costs::LineCosts,
    ),
    preserves_atoms: bool,
) -> std::sync::Arc<super::paragraph_session::Product> {
    let (strategy, shape, font, justify, costs) = options;
    let (lines, hyphenated) = if p.words.iter().any(|w| w.hyphenation.is_some()) {
        let lines = if shape.is_uniform() {
            super::paragraph_hyphen::select(
                &p,
                shape.width(true),
                font,
                justify,
                strategy,
                costs,
            )
        } else {
            super::paragraph_hyphen::select_shaped(
                &p, shape, font, justify, strategy, costs,
            )
        };
        (Vec::new(), Some(lines))
    } else {
        let lines = if shape.is_uniform() {
            paragraph_select::select(
                &p,
                shape.width(true),
                font,
                justify,
                strategy,
                costs,
            )
        } else {
            paragraph_select::select_shaped(&p, shape, font, justify, strategy, costs)
        };
        (lines, None)
    };
    let no_highlight = |p: &paragraph_prepare::Prepared| {
        p.words.iter().all(|w| {
            !w.fragments.is_empty()
                && w.fragments
                    .iter()
                    .all(|f| !f.text.is_empty() && f.style.highlight.is_none())
        })
    };
    let paintable = no_highlight(&p)
        && hyphenated.as_ref().is_none_or(|lines| lines.iter().all(no_highlight));
    std::sync::Arc::new(super::paragraph_session::Product {
        paint_id: std::sync::Arc::new(()),
        paintable,
        preserves_atoms,
        prepared: p,
        lines,
        hyphenated,
    })
}
pub(super) fn try_layout<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    body: &Content,
) -> bool {
    if defer_integral_styled(body) {
        return false;
    }
    let Some((strategy, width, font, justify)) = options(l) else {
        return false;
    };
    let costs = crate::compiler::text_costs::resolve(&l.chain);
    let previous_indent = l.paragraph_indent.clone();
    super::paragraph_indent::start(l);
    let shape = super::paragraph_indent::shape(l, width);
    let request = super::paragraph_session::request(
        l,
        std::slice::from_ref(body),
        (strategy, shape, font, justify, costs),
    );
    l.paragraph_indent = previous_indent;
    let product = if let Some(product) = request.as_ref().and_then(|r| r.lookup()) {
        product
    } else {
        let Some(p) = paragraph_prepare::prepare(l, body) else {
            return false;
        };
        let product = select_product(
            p,
            (strategy, shape, font, justify, costs),
            super::paragraph_session::preserves_atom_boundaries(std::slice::from_ref(
                body,
            )),
        );
        if let Some(request) = request {
            request.store(product.clone());
        }
        product
    };
    super::paragraph_indent::start(l);
    if implicit_regional_only(l)
        && product.hyphenated.as_ref().map_or(product.lines.len(), Vec::len) < 2
        && !single_overlong_word(&product.prepared, width)
    {
        return false;
    }
    paragraph_emit::emit_product(l, &product, width, justify);
    true
}
enum Segment<'a> {
    Planned(std::sync::Arc<super::paragraph_session::Product>),
    Incremental(&'a [Content]),
}
impl Segment<'_> {
    fn product(&self) -> Option<&super::paragraph_session::Product> {
        match self {
            Self::Planned(product) => Some(product),
            Self::Incremental(_) => None,
        }
    }
}
pub(super) fn try_parts<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    parts: &[Content],
) -> bool {
    let Some((strategy, width, font, justify)) = options(l) else { return false };
    super::paragraph_indent::start_scope(l, parts);
    let shape = super::paragraph_indent::shape(l, width);
    if parts
        .split(|part| matches!(part, Content::Parbreak))
        .any(integral_styled)
    {
        return false;
    }
    let costs = crate::compiler::text_costs::resolve(&l.chain);
    let mut prepared = Vec::new();
    let mut first_material = true;
    for segment in parts.split(|part| matches!(part, Content::Parbreak)) {
        if segment.iter().all(|part| {
            matches!(part, Content::Empty | Content::Space | Content::SpaceFromNewline)
        }) {
            prepared.push(None);
            continue;
        }
        let shape = if !first_material && l.pending_bib_escape.is_some() {
            super::paragraph_indent::Shape::uniform(shape.width(false))
        } else {
            shape
        };
        first_material = false;
        let request = super::paragraph_session::request(
            l,
            segment,
            (strategy, shape, font, justify, costs),
        );
        let product = if let Some(product) = request.as_ref().and_then(|r| r.lookup()) {
            product
        } else {
            let Some(p) = paragraph_prepare::prepare_parts(l, segment) else {
                if l.page_running_pass.is_some()
                    || l.capture_html_context_markers
                    || l.pending_bib_escape.is_some()
                    || !super::paragraph_session::preserves_atom_boundaries(segment)
                {
                    return false;
                }
                prepared.push(Some(Segment::Incremental(segment)));
                continue;
            };
            let product = select_product(
                p,
                (strategy, shape, font, justify, costs),
                super::paragraph_session::preserves_atom_boundaries(segment),
            );
            if let Some(request) = request {
                request.store(product.clone());
            }
            product
        };
        prepared.push(Some(Segment::Planned(product)));
    }
    if !prepared.iter().flatten().any(|s| matches!(s, Segment::Planned(_))) {
        return false;
    }
    let mixed = prepared
        .iter()
        .flatten()
        .any(|s| matches!(s, Segment::Incremental(_)));
    let loses_atoms = prepared
        .iter()
        .flatten()
        .filter_map(Segment::product)
        .any(|p| !p.preserves_atoms);
    if mixed && loses_atoms {
        return false;
    }
    if implicit_regional_only(l)
        && prepared.iter().flatten().filter_map(Segment::product).all(|p| {
            p.hyphenated.as_ref().map_or(p.lines.len(), Vec::len) < 2
                && !single_overlong_word(&p.prepared, width)
        })
        && loses_atoms
    {
        return false;
    }
    for (index, segment) in prepared.iter().enumerate() {
        if index > 0 {
            super::parbreak::layout(l);
        }
        let Some(segment) = segment else { continue };
        match segment {
            Segment::Planned(product) => {
                paragraph_emit::emit_product(l, product, width, justify);
            }
            Segment::Incremental(parts) => {
                let previous = l.paragraph_planning_suppressed;
                l.paragraph_planning_suppressed = true;
                super::sequence::layout(l, parts);
                l.paragraph_planning_suppressed = previous;
            }
        }
    }
    true
}

#[cfg(test)]
mod contiguous_overlong_tests {
    use super::*;
    use crate::compiler::layout::FixedMetrics;
    use crate::entities::{
        elements::context_block::ContextBlockElem,
        func::Func,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{PageMarginSpec, PageMargins},
    };
    use comemo::Track;
    use std::sync::Arc;

    #[test]
    fn adjacent_numeric_fragments_use_one_overlong_line() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = 100.0;
        l.page_config.margin = PageMargins::uniform(0.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(0.0);
        l.regions.current.width = 100.0;
        let mut parts = vec![Content::Space];
        parts.extend((0..100).map(|i| Content::text(i.to_string())));
        parts.push(Content::Space);
        assert!(try_parts(&mut l, &parts));
        let doc = l.finish();
        assert_eq!(
            doc.pages[0].plain_text(),
            (0..100).map(|i| i.to_string()).collect::<String>()
        );
    }

    #[test]
    fn contextual_markers_are_neutral_only_in_paginated_layout() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = 100.0;
        l.page_config.margin = PageMargins::uniform(0.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(0.0);
        l.regions.current.width = 100.0;
        let marker = Content::ContextBlock(Arc::new(ContextBlockElem {
            id: 0,
            closure: Func::native("unused", |_, _, _, _| panic!("must not execute")),
        }));
        let mut parts = vec![Content::Space];
        parts.extend((0..100).map(|i| {
            Content::sequence(vec![marker.clone(), Content::text(i.to_string())])
        }));
        parts.push(Content::Space);
        assert!(try_parts(&mut l, &parts));
        let mut html = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        html.capture_html_context_markers = true;
        assert!(!try_parts(&mut html, &parts));
    }
}

#[cfg(test)]
mod retained_route_tests {
    use super::*;
    use crate::entities::{
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{FrameItem, PageMarginSpec, PageMargins, PagedDocument, Pt},
    };
    use comemo::Track;
    fn words(doc: &PagedDocument) -> Vec<(usize, String, u64, u64)> {
        let mut rows = Vec::new();
        for (page, p) in doc.pages.iter().enumerate() {
            for item in &p.items {
                if let FrameItem::Text { pos, text, .. } = item {
                    rows.push((
                        page,
                        text.to_string(),
                        pos.x.0.to_bits(),
                        pos.y.0.to_bits(),
                    ));
                }
            }
        }
        rows
    }
    #[test]
    fn retained_route_single_lines_match_incremental_geometry() {
        let intr = TagIntrospector::empty();
        for width in [180.0, 400.0] {
            for size in [10.0, 18.0] {
                for prefix in [false, true] {
                    let parts: Vec<_> = (0..100)
                        .flat_map(|i| {
                            [
                                Content::text(format!("Paragraph {i} words.")),
                                Content::Parbreak,
                            ]
                        })
                        .collect();
                    let make = || {
                        let mut l = Layouter::new(
                            super::super::FixedMetrics,
                            NullImageSizer,
                            size,
                            (&intr as &dyn Introspector).track(),
                        );
                        l.chain =
                            l.chain.push(crate::entities::style_chain::StyleDelta {
                                size: Some(crate::entities::layout_types::Length::pt(
                                    size,
                                )),
                                ..crate::entities::style_chain::StyleDelta::empty()
                            });
                        l.style =
                            crate::entities::layout_types::TextStyle::from(&l.chain);
                        l.page_config.width = width;
                        l.page_config.height = 200.0;
                        l.page_config.margin = PageMargins::uniform(10.0);
                        l.page_config.margin_spec = PageMarginSpec::uniform(10.0);
                        l.regions.current.width = width;
                        l.regions.current.height = 200.0;
                        l.regions.current.cursor_x = Pt(10.0);
                        l.regions.current.line_start_x = Pt(10.0);
                        l.regions.current.cursor_y = Pt(10.0);
                        if prefix {
                            l.layout_content(&Content::text("Prefix"));
                            l.layout_content(&Content::Parbreak);
                        }
                        l
                    };
                    let mut cold = make();
                    cold.paragraph_planning_suppressed = true;
                    cold.layout_content(&Content::sequence(parts.clone()));
                    let cold = cold.finish();
                    let mut candidate = make();
                    assert!(
                        try_parts(&mut candidate, &parts),
                        "complete single-line plan was discarded"
                    );
                    let candidate = candidate.finish();
                    assert_eq!(candidate.pages.len(), cold.pages.len());
                    assert_eq!(candidate.plain_text(), cold.plain_text());
                    assert_eq!(words(&candidate), words(&cold));
                }
            }
        }
    }
}

#[cfg(test)]
mod paragraph_fallback_tests {
    use super::*;
    use crate::compiler::layout::{
        paragraph_session::{MetricIdentity, ParagraphSession},
        FixedMetrics,
    };
    use crate::entities::{
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{
            FrameItem, PageMarginSpec, PageMargins, PagedDocument, Pt, TextStyle,
        },
        style_chain::StyleDelta,
    };
    use comemo::Track;
    #[derive(Clone)]
    struct Provider {
        session: ParagraphSession,
        identity: MetricIdentity,
    }
    impl FontMetrics for Provider {
        fn paragraph_session(&self) -> Option<ParagraphSession> {
            Some(self.session.clone())
        }
        fn paragraph_identity(&self, _: &str, _: &TextStyle) -> Option<MetricIdentity> {
            Some(self.identity.clone())
        }
        fn advance(&self, t: &str, s: Pt, st: &TextStyle) -> Pt {
            FixedMetrics.advance(t, s, st)
        }
        fn vertical_metrics(&self, s: Pt, st: &TextStyle) -> (Pt, Pt) {
            FixedMetrics.vertical_metrics(s, st)
        }
        fn cap_height(&self, s: Pt, st: &TextStyle) -> Pt {
            FixedMetrics.cap_height(s, st)
        }
        fn text_edges(&self, s: Pt, st: &TextStyle) -> (Pt, Pt) {
            FixedMetrics.text_edges(s, st)
        }
    }
    fn provider() -> Provider {
        Provider {
            session: ParagraphSession::new(),
            identity: MetricIdentity::new(),
        }
    }
    fn configured<'a>(
        p: &Provider,
        intr: &'a dyn Introspector,
        width: f64,
        size: f64,
    ) -> Layouter<'a, Provider, NullImageSizer> {
        let mut l = Layouter::new(p.clone(), NullImageSizer, size, intr.track());
        l.page_config.width = width;
        l.page_config.height = 180.0;
        l.page_config.margin = PageMargins::uniform(10.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(10.0);
        l.regions.current.width = width;
        l.regions.current.height = 180.0;
        l.regions.current.cursor_x = Pt(10.0);
        l.regions.current.line_start_x = Pt(10.0);
        l.regions.current.cursor_y = Pt(10.0);
        let mut d = StyleDelta::empty();
        d.size = Some(crate::entities::layout_types::Length::pt(size));
        l.chain = l.chain.push(d);
        l.style = TextStyle::from(&l.chain);
        l
    }
    fn parts(tail: Content) -> Vec<Content> {
        let mut v = Vec::new();
        for i in 0..12 {
            v.push(Content::text(format!(
                "Paragraph {i} with several words to lay out across lines."
            )));
            v.push(Content::Parbreak);
        }
        v.push(tail);
        v
    }
    fn observed(d: &PagedDocument) -> Vec<(usize, String, u64, u64)> {
        d.pages
            .iter()
            .enumerate()
            .flat_map(|(page, p)| {
                super::super::tests::frame_items_recursive(&p.items)
                    .into_iter()
                    .filter_map(move |i| {
                        if let FrameItem::Text { pos, text, .. } = i {
                            Some((
                                page,
                                text.to_string(),
                                pos.x.0.to_bits(),
                                pos.y.0.to_bits(),
                            ))
                        } else {
                            None
                        }
                    })
            })
            .collect()
    }
    #[test]
    fn paragraph_fallback_keeps_prepared_prefix_with_current_geometry() {
        let p = provider();
        let intr = TagIntrospector::empty();
        for (width, size) in [(400.0, 11.0), (180.0, 11.0), (400.0, 16.0)] {
            let source = parts(Content::text("tail_word"));
            let mut warm = configured(&p, &intr, width, size);
            let _ = try_parts(&mut warm, &source);
            let before = p.session.stats();
            let mut actual = configured(&p, &intr, width, size);
            assert!(
                try_parts(&mut actual, &source),
                "one refused scalar paragraph discarded the complete prepared prefix"
            );
            assert!(
                p.session.stats().0 > before.0,
                "intact paragraph products were not reused"
            );
            let actual = actual.finish();
            let mut cold = configured(&p, &intr, width, size);
            cold.paragraph_planning_suppressed = true;
            super::super::sequence::layout(&mut cold, &source);
            let cold = cold.finish();
            assert_eq!(actual.pages.len(), cold.pages.len());
            assert_eq!(
                actual.pages.iter().map(|p| p.plain_text()).collect::<Vec<_>>(),
                cold.pages.iter().map(|p| p.plain_text()).collect::<Vec<_>>()
            );
            assert_eq!(
                observed(&actual),
                observed(&cold),
                "fallback changed language geometry"
            );
        }
    }
    #[test]
    fn paragraph_fallback_opaque_tail_still_refuses_before_emission() {
        let p = provider();
        let intr = TagIntrospector::empty();
        let mut l = configured(&p, &intr, 400.0, 11.0);
        assert!(!try_parts(&mut l, &parts(Content::Symbol("tail".into()))));
        assert!(l.regions.current.current_line.is_empty());
        assert!(l.pages.is_empty());
    }
    #[test]
    fn paragraph_fallback_adjacent_unprepared_atoms_still_refuse() {
        let p = provider();
        let intr = TagIntrospector::empty();
        let mut l = configured(&p, &intr, 400.0, 11.0);
        let mut source = parts(Content::text("tail_"));
        source.push(Content::text("word"));
        assert!(!try_parts(&mut l, &source));
        assert!(l.regions.current.current_line.is_empty());
        assert!(l.pages.is_empty());
    }
}
