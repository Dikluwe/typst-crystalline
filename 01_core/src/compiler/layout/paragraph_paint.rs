//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/paragraph_paint.md
//! @prompt-hash cf8d6f09
//! @layer L1
use super::{FontMetrics, ImageSizer, Layouter};
use crate::entities::{
    font_list::FontNamePattern,
    lang::Lang,
    layout_types::{FrameItem, Pt, TextStyle},
    value::Value,
};
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
};
const MAX_ENTRIES: usize = 4096;
const MAX_PAYLOAD: usize = 8 * 1024 * 1024;
const MAX_ENTRY: usize = 256 * 1024;
const MAX_ITEMS: usize = 2048;
#[derive(Default)]
pub(super) struct PaintSession(Mutex<Memo>);
#[derive(Default)]
struct Memo {
    entries: HashMap<Key, Entry>,
    order: VecDeque<Key>,
    charged: usize,
    hits: usize,
    published: usize,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Key {
    identity: usize,
    line: usize,
    numeric: [u64; 4],
}
struct Entry {
    _owner: Arc<()>,
    product: Arc<Paint>,
    charge: usize,
}
struct Paint {
    items: Vec<FrameItem>,
    anchors: Vec<f64>,
    end: Pt,
    baseline: Pt,
}
impl PaintSession {
    #[cfg(test)]
    fn stats(&self) -> (usize, usize, usize) {
        let m = self.0.lock().unwrap();
        (m.hits, m.published, m.entries.len())
    }
}
fn supported_style(style: &TextStyle) -> Option<usize> {
    if style.highlight.is_some()
        || matches!(style.fill, Some(crate::entities::color::Color::Spot(_)))
    {
        return None;
    }
    let mut size = super::paragraph_session::style_payload(style, &mut Vec::new())?;
    if let Some(fonts) = &style.font {
        for family in fonts.as_slice() {
            if matches!(family.name, FontNamePattern::Regex(_)) {
                return None;
            }
            size = size.checked_add(std::mem::size_of_val(family))?.checked_add(
                family
                    .variants
                    .len()
                    .checked_mul(std::mem::size_of::<ecow::EcoString>())?,
            )?;
        }
    }
    Some(size)
}
fn baseline_position(baseline: Pt, style: &TextStyle) -> Pt {
    baseline + Pt(style.baseline_offset.resolve_pt(style.size.val()))
}

fn at_baseline(item: &mut FrameItem, baseline: Pt) {
    match item {
        FrameItem::Text { pos, style, .. }
        | FrameItem::TextSeparator { pos, style, .. } => {
            pos.y = baseline_position(baseline, style);
        }
        FrameItem::TextReplacement { items, .. } => {
            for child in items {
                at_baseline(child, baseline);
            }
        }
        _ => unreachable!(
            "paint product requires individually positioned text or separators"
        ),
    }
}

fn exact_baseline(y: Pt, style: &TextStyle, baseline: Pt) -> bool {
    y.0.is_finite() && y.0.to_bits() == baseline_position(baseline, style).0.to_bits()
}

fn item_charge(item: &FrameItem, depth: usize, baseline: Pt) -> Option<usize> {
    if depth > 1 {
        return None;
    }
    let base = std::mem::size_of::<FrameItem>();
    match item {
        FrameItem::Text { pos, text, style, bidi: None } => {
            if !pos.x.0.is_finite() || !exact_baseline(pos.y, style, baseline) {
                return None;
            }
            base.checked_add(text.len())?.checked_add(supported_style(style)?)
        }
        FrameItem::TextSeparator { pos, advance, style, bidi: None, .. } => {
            if !pos.x.0.is_finite()
                || !exact_baseline(pos.y, style, baseline)
                || !advance.0.is_finite()
            {
                return None;
            }
            base.checked_add(supported_style(style)?)
        }
        FrameItem::TextReplacement { text, items } if items.len() <= MAX_ITEMS => {
            let mut size = base.checked_add(text.len())?;
            for item in items {
                size = size.checked_add(item_charge(item, depth + 1, baseline)?)?;
            }
            Some(size)
        }
        FrameItem::Text { .. }
        | FrameItem::TextSeparator { .. }
        | FrameItem::TextReplacement { .. }
        | FrameItem::TextShaped { .. }
        | FrameItem::Glyph { .. }
        | FrameItem::Image { .. }
        | FrameItem::Shape { .. }
        | FrameItem::Line { .. }
        | FrameItem::Group { .. }
        | FrameItem::Link { .. }
        | FrameItem::Semantic { .. } => None,
    }
}
fn charge(items: &[FrameItem], anchors: &[f64], baseline: Pt) -> Option<usize> {
    if items.is_empty()
        || items.len() > MAX_ITEMS
        || anchors.len() > MAX_ITEMS
        || anchors.iter().any(|v| !v.is_finite())
    {
        return None;
    }
    let mut size =
        std::mem::size_of::<Entry>().checked_add(anchors.len().checked_mul(8)?)?;
    for item in items {
        size = size.checked_add(item_charge(item, 0, baseline)?)?;
    }
    (size <= MAX_ENTRY).then_some(size)
}
/// The caller's immutable product and ordinal identify its exact prepared tail.
/// The first word has already admitted material. Only replay the pure prepared
/// tail, using the current line baseline and its original per-item offset.
pub(super) fn tail<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    identity: &Arc<()>,
    line: usize,
    spacing: [f64; 2],
    language: Option<Lang>,
    emit: impl FnOnce(&mut Layouter<M, S>),
) {
    let baseline = l.regions.current.cursor_y;
    let numeric = [
        l.regions.current.cursor_x.0,
        l.text_horizontal_offset.0,
        spacing[0],
        spacing[1],
    ];
    let session = l.metrics.paragraph_session();
    let eligible = !l.initial_baseline_pending
        && !l.block_chain_active
        && super::paragraph_indent::origin_applied(l)
        && l.pending_bib_prefix.is_none()
        && l.pending_bib_escape.is_none()
        && l.text_bidi.is_none()
        && baseline.0.is_finite()
        && numeric.iter().all(|n| n.is_finite())
        && super::html_frame::items_have_material(&l.regions.current.current_line);
    let Some(session) = session.filter(|_| eligible) else {
        emit(l);
        return;
    };
    let key = Key {
        identity: Arc::as_ptr(identity) as usize,
        line,
        numeric: numeric.map(f64::to_bits),
    };
    let product = {
        let mut m = session.paint.0.lock().unwrap();
        let product = m.entries.get(&key).map(|e| e.product.clone());
        if product.is_some() {
            m.hits += 1;
        }
        product
    };
    if let Some(product) = product {
        let line = &mut l.regions.current.current_line;
        let first = line.len();
        line.extend(product.items.iter().cloned());
        if product.baseline.0.to_bits() != baseline.0.to_bits() {
            for item in &mut line[first..] {
                at_baseline(item, baseline);
            }
        }
        l.regions.current.cursor_x = product.end;
        l.justify_opportunities.extend_from_slice(&product.anchors);
        if l.document_language.is_none() {
            l.document_language = language;
        }
        l.line_optical_overhang =
            Some(!matches!(l.chain.custom("text.overhang"), Some(Value::Bool(false))));
        l.text_tracking_state.record();
        return;
    }
    let first = l.regions.current.current_line.len();
    let anchor = l.justify_opportunities.len();
    emit(l);
    let items = &l.regions.current.current_line[first..];
    let anchors = &l.justify_opportunities[anchor..];
    let Some(charge) = charge(items, anchors, baseline) else {
        return;
    };
    if !l.regions.current.cursor_x.0.is_finite()
        || l.regions.current.cursor_y.0.to_bits() != baseline.0.to_bits()
    {
        return;
    }
    let product = Arc::new(Paint {
        items: items.to_vec(),
        anchors: anchors.to_vec(),
        end: l.regions.current.cursor_x,
        baseline,
    });
    let mut m = session.paint.0.lock().unwrap();
    if m.entries.contains_key(&key) {
        return;
    }
    while m.entries.len() >= MAX_ENTRIES
        || m.charged.checked_add(charge).is_none_or(|n| n > MAX_PAYLOAD)
    {
        let Some(old) = m.order.pop_front() else {
            return;
        };
        if let Some(entry) = m.entries.remove(&old) {
            m.charged -= entry.charge;
        }
    }
    m.charged += charge;
    m.published += 1;
    m.order.push_back(key);
    m.entries
        .insert(key, Entry { _owner: identity.clone(), product, charge });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::layout::{FixedMetrics, paragraph_session::ParagraphSession};
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{Pt, TextStyle},
    };
    use comemo::Track;
    use std::cell::Cell;
    #[derive(Clone)]
    pub(super) struct Metrics(pub(super) Option<ParagraphSession>);
    impl FontMetrics for Metrics {
        fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
            FixedMetrics.advance(text, size, style)
        }
        fn vertical_metrics(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
            FixedMetrics.vertical_metrics(size, style)
        }
        fn cap_height(&self, size: Pt, style: &TextStyle) -> Pt {
            FixedMetrics.cap_height(size, style)
        }
        fn text_edges(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
            FixedMetrics.text_edges(size, style)
        }
        fn paragraph_session(&self) -> Option<ParagraphSession> {
            self.0.clone()
        }
    }
    fn once(
        session: Option<ParagraphSession>,
        id: &Arc<()>,
        x: f64,
        spacing: [f64; 2],
        refuse: bool,
        calls: &Cell<usize>,
    ) -> (String, u64, Option<crate::entities::lang::Lang>) {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            Metrics(session),
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        super::super::paragraph_indent::start(&mut l);
        l.layout_content(&Content::text("prefix"));
        l.regions.current.cursor_x += Pt(x);
        if refuse {
            l.text_bidi = None;
            l.block_chain_active = true;
        }
        let lang = Some("en".parse().unwrap());
        tail(&mut l, id, 0, spacing, lang, |l| {
            calls.set(calls.get() + 1);
            if l.document_language.is_none() {
                l.document_language = lang;
            }
            let style = l.style.clone();
            let width = l.metrics.advance("retained", style.size, &style);
            super::super::word_layout::prepared(l, "retained", width, &style, None);
            l.justify_opportunities.push(l.regions.current.cursor_x.0);
        });
        (
            format!("{:?}", l.regions.current.current_line),
            l.regions.current.cursor_x.0.to_bits(),
            l.document_language,
        )
    }
    #[test]
    fn line_products_skip_actual_cold_emitter_across_fresh_layouters() {
        let session = ParagraphSession::new();
        let id = Arc::new(());
        let calls = Cell::new(0);
        let a = once(Some(session.clone()), &id, 0.0, [0.0, 0.0], false, &calls);
        let b = once(Some(session), &id, 0.0, [0.0, 0.0], false, &calls);
        assert_eq!(a, b);
        assert_eq!(calls.get(), 1, "same pure line tail still rebuilt");
    }
    #[test]
    fn line_products_exact_position_spacing_and_product_key() {
        let session = ParagraphSession::new();
        let a = Arc::new(());
        let b = Arc::new(());
        let calls = Cell::new(0);
        for (id, x, s) in [
            (&a, 0.0, [0.0, 0.0]),
            (&a, 2.0, [0.0, 0.0]),
            (&a, 0.0, [0.1, 0.0]),
            (&b, 0.0, [0.0, 0.0]),
        ] {
            let first = once(Some(session.clone()), id, x, s, false, &calls);
            let second = once(Some(session.clone()), id, x, s, false, &calls);
            assert_eq!(first, second);
        }
        assert_eq!(calls.get(), 4, "exact keys must reuse only their own product");
    }
    #[test]
    fn line_products_refuse_pending_flow_and_absent_session() {
        let session = ParagraphSession::new();
        let id = Arc::new(());
        let calls = Cell::new(0);
        for _ in 0..2 {
            once(Some(session.clone()), &id, 0.0, [0.0, 0.0], true, &calls);
        }
        for _ in 0..2 {
            once(None, &id, 0.0, [0.0, 0.0], false, &calls);
        }
        assert_eq!(calls.get(), 4);
    }
    #[test]
    fn reused_line_replays_exactly_at_the_new_baseline() {
        use crate::entities::layout_types::{Length, Point, TextSeparatorOrigin};

        let identity = Arc::new(());
        let memo = ParagraphSession::new();
        let calls = Cell::new(0);
        let run = |session, y| {
            let intr = TagIntrospector::empty();
            let mut l = Layouter::new(
                Metrics(session),
                NullImageSizer,
                11.0,
                (&intr as &dyn Introspector).track(),
            );
            super::super::paragraph_indent::start(&mut l);
            l.layout_content(&Content::text("prefix"));
            l.regions.current.cursor_y = Pt(y);
            let style = TextStyle {
                baseline_offset: Length::pt(0.1),
                ..l.style.clone()
            };
            tail(&mut l, &identity, 0, [0.0, 0.0], None, |l| {
                calls.set(calls.get() + 1);
                l.regions.current.current_line.push(FrameItem::TextSeparator {
                    pos: Point {
                        x: l.regions.current.cursor_x,
                        y: l.regions.current.cursor_y + Pt(0.1),
                    },
                    advance: Pt(2.0),
                    style: style.clone(),
                    origin: TextSeparatorOrigin::Literal,
                    bidi: None,
                });
                l.regions.current.cursor_x += Pt(2.0);
                l.justify_opportunities.push(l.regions.current.cursor_x.0);
                super::super::word_layout::prepared(
                    l,
                    "frag",
                    Pt(7.0),
                    &style,
                    Some("original"),
                );
            });
            (
                format!("{:?}", l.regions.current.current_line),
                l.regions.current.cursor_x.0.to_bits(),
                l.justify_opportunities,
            )
        };
        let expected = run(None, 0.3);
        let original_y = run(Some(memo.clone()), 29.0);
        assert_ne!(original_y, expected);
        assert_eq!(run(Some(memo), 0.3), expected);
        assert_eq!(calls.get(), 2, "tail must skip the third cold emission");
    }

    #[test]
    fn line_products_independent_sessions_are_cold() {
        let id = Arc::new(());
        let calls = Cell::new(0);
        for _ in 0..2 {
            once(Some(ParagraphSession::new()), &id, 0.0, [0.0, 0.0], false, &calls);
        }
        assert_eq!(calls.get(), 2);
    }
}

#[cfg(test)]
mod pipeline_tests {
    use super::super::{
        FixedMetrics,
        paragraph_session::{MetricIdentity, ParagraphSession},
    };
    use super::*;
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{Length, PageMarginSpec, PageMargins, PagedDocument},
        style_chain::StyleDelta,
    };
    use comemo::Track;
    #[derive(Clone)]
    struct Provider {
        session: Option<ParagraphSession>,
        identity: MetricIdentity,
    }
    impl FontMetrics for Provider {
        fn paragraph_session(&self) -> Option<ParagraphSession> {
            self.session.clone()
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
    fn layout(p: &Provider, width: f64, size: f64, direct: bool) -> PagedDocument {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            p.clone(),
            NullImageSizer,
            size,
            (&intr as &dyn Introspector).track(),
        );
        l.chain = l.chain.push(StyleDelta {
            size: Some(Length::pt(size)),
            ..StyleDelta::empty()
        });
        l.style = TextStyle::from(&l.chain);
        l.page_config.width = width;
        l.page_config.height = 90.0;
        l.page_config.margin = PageMargins::uniform(10.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(10.0);
        l.regions.current.width = width;
        l.regions.current.height = 90.0;
        l.regions.current.cursor_x = Pt(10.0);
        l.regions.current.line_start_x = Pt(10.0);
        l.regions.current.cursor_y = Pt(10.0);
        let body = Content::text(
            "alpha beta gamma delta epsilon zeta eta theta iota kappa lambda mu nu xi omicron pi rho sigma tau upsilon phi chi psi omega",
        );
        assert!(if direct {
            super::super::paragraph_plan::try_layout(&mut l, &body)
        } else {
            super::super::paragraph_plan::try_parts(&mut l, &[body])
        });
        l.finish()
    }
    fn check(direct: bool) {
        let session = ParagraphSession::new();
        let p = Provider {
            session: Some(session.clone()),
            identity: MetricIdentity::new(),
        };
        let cold = Provider { session: None, identity: p.identity.clone() };
        for (width, size) in [(180.0, 11.0), (145.0, 11.0), (180.0, 14.0)] {
            let a = layout(&p, width, size, direct);
            let before = session.paint.stats().0;
            let b = layout(&p, width, size, direct);
            let c = layout(&cold, width, size, direct);
            assert_eq!(format!("{:?}", a.pages), format!("{:?}", b.pages));
            assert_eq!(format!("{:?}", b.pages), format!("{:?}", c.pages));
            assert!(
                session.paint.stats().0 > before,
                "full pipeline did not reuse a completed line tail"
            );
        }
    }
    #[test]
    fn line_products_pipeline_parts_reuses_with_current_pagination() {
        check(false);
    }
    #[test]
    fn line_products_pipeline_direct_reuses_with_current_pagination() {
        check(true);
    }
}

#[cfg(test)]
mod adversarial_tests {
    use super::super::paragraph_session::ParagraphSession;
    use super::tests::Metrics;
    use super::*;
    use crate::entities::{
        bidi_context::{BidiSpan, ParagraphBidiInput, ParagraphBidiSegment},
        content::Content,
        dir::Dir,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{Length, Point},
    };
    use comemo::Track;
    use std::cell::Cell;
    fn ready(
        session: &ParagraphSession,
        run: impl FnOnce(&mut Layouter<Metrics, NullImageSizer>),
    ) {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            Metrics(Some(session.clone())),
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        super::super::paragraph_indent::start(&mut l);
        l.layout_content(&Content::text("prefix"));
        run(&mut l);
    }
    fn emit(l: &mut Layouter<Metrics, NullImageSizer>, calls: &Cell<usize>) {
        calls.set(calls.get() + 1);
        let style = l.style.clone();
        let width = l.metrics.advance("retained", style.size, &style);
        if l.document_language.is_none() {
            l.document_language = Some("en".parse().unwrap());
        }
        super::super::word_layout::prepared(l, "retained", width, &style, None);
        l.justify_opportunities.push(l.regions.current.cursor_x.0);
    }
    #[test]
    fn line_products_all_admission_refusers_bypass_retention() {
        for kind in 0..6 {
            let session = ParagraphSession::new();
            let id = Arc::new(());
            let calls = Cell::new(0);
            for _ in 0..2 {
                ready(&session, |l| {
                    match kind {
                        0 => l.initial_baseline_pending = true,
                        1 => l.block_chain_active = true,
                        2 => {
                            l.paragraph_indent =
                                super::super::paragraph_indent::State::default()
                        }
                        3 => l.regions.current.current_line.clear(),
                        4 => {
                            l.text_bidi = Some(
                                BidiSpan::new(
                                    Arc::new(
                                        ParagraphBidiInput::new(
                                            Dir::LTR,
                                            vec![ParagraphBidiSegment::Text {
                                                text: "retained".into(),
                                                dir: Dir::LTR,
                                            }],
                                        )
                                        .unwrap(),
                                    ),
                                    0,
                                    0..8,
                                )
                                .unwrap(),
                            )
                        }
                        5 => {
                            l.pending_bib_escape =
                                Some(super::super::paragraph_indent::Spec::escape(
                                    2.0,
                                    Dir::LTR,
                                ))
                        }
                        _ => unreachable!(),
                    }
                    tail(l, &id, 0, [0.0, 0.0], Some("en".parse().unwrap()), |l| {
                        emit(l, &calls)
                    });
                });
            }
            assert_eq!(calls.get(), 2, "refuser {kind} admitted a hit");
            assert_eq!(session.paint.stats().2, 0);
        }
    }
    #[test]
    fn line_products_replay_current_tracking_language_and_anchors() {
        let session = ParagraphSession::new();
        let id = Arc::new(());
        let calls = Cell::new(0);
        for language in [None, Some("fr".parse().unwrap())] {
            ready(&session, |l| {
                l.text_tracking_state.enter();
                l.style.tracking = Some(Length::pt(1.0));
                l.document_language = language;
                tail(l, &id, 0, [0.0, 0.0], Some("en".parse().unwrap()), |l| {
                    emit(l, &calls)
                });
                assert_eq!(l.document_language, language.or(Some("en".parse().unwrap())));
                assert_eq!(
                    l.text_tracking_state
                        .boundary(&l.regions.current.current_line, &l.style),
                    Pt(1.0)
                );
                assert_eq!(
                    l.justify_opportunities.last().copied(),
                    Some(l.regions.current.cursor_x.0)
                );
            });
        }
        assert_eq!(calls.get(), 1);
    }
    #[test]
    fn line_products_horizontal_offset_is_exact_input() {
        let session = ParagraphSession::new();
        let id = Arc::new(());
        let calls = Cell::new(0);
        let mut xs = vec![];
        for offset in [0.0, 3.0, 0.0] {
            ready(&session, |l| {
                l.text_horizontal_offset = Pt(offset);
                tail(l, &id, 0, [0.0, 0.0], Some("en".parse().unwrap()), |l| {
                    emit(l, &calls)
                });
                let FrameItem::Text { pos, .. } =
                    l.regions.current.current_line.last().unwrap()
                else {
                    panic!()
                };
                xs.push(pos.x.0);
            });
        }
        assert_eq!(calls.get(), 2);
        assert_eq!(xs[0], xs[2]);
        assert_eq!(xs[1] - xs[0], 3.0);
    }
    #[test]
    fn line_products_oversized_product_stays_cold() {
        let session = ParagraphSession::new();
        let id = Arc::new(());
        let calls = Cell::new(0);
        for _ in 0..2 {
            ready(&session, |l| {
                tail(l, &id, 0, [0.0, 0.0], None, |l| {
                    calls.set(calls.get() + 1);
                    l.regions.current.current_line.push(FrameItem::Text {
                        pos: Point::ZERO,
                        text: "x".repeat(MAX_ENTRY + 1).into(),
                        style: l.style.clone(),
                        bidi: None,
                    });
                })
            });
        }
        assert_eq!(calls.get(), 2);
        assert_eq!(session.paint.stats().2, 0);
    }
    #[test]
    fn line_products_entry_and_payload_bounds_evict_complete_products() {
        let session = ParagraphSession::new();
        let id = Arc::new(());
        let calls = Cell::new(0);
        for x in (0..=MAX_ENTRIES).chain(std::iter::once(0)) {
            ready(&session, |l| {
                l.regions.current.cursor_x += Pt(x as f64);
                tail(l, &id, 0, [0.0, 0.0], Some("en".parse().unwrap()), |l| {
                    emit(l, &calls)
                });
            });
        }
        assert_eq!(calls.get(), MAX_ENTRIES + 2);
        let m = session.paint.0.lock().unwrap();
        assert!(m.entries.len() <= MAX_ENTRIES);
        assert!(m.charged <= MAX_PAYLOAD);
        assert_eq!(m.order.len(), m.entries.len());
    }
    #[test]
    fn line_products_detached_owner_prevents_address_recycling() {
        let session = ParagraphSession::new();
        let id = Arc::new(());
        let weak = Arc::downgrade(&id);
        let calls = Cell::new(0);
        ready(&session, |l| {
            tail(l, &id, 0, [0.0, 0.0], Some("en".parse().unwrap()), |l| emit(l, &calls))
        });
        drop(id);
        assert!(weak.upgrade().is_some());
        drop(session);
        assert!(weak.upgrade().is_none());
    }
}
