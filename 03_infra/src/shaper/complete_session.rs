//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper/complete_session.md
//! @prompt-hash e194edd8
//! @layer L3
use ecow::EcoString;
use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
    sync::{Arc, Mutex},
};
use typst_core::contracts::world::World;
use typst_core::entities::{
    bidi_context::BidiSpan,
    font_list::{FontList, FontNamePattern},
    layout_types::{FrameItem, Point, Pt, SharedGlyphs, TextEdge, TextStyle},
    world_types::Font,
};
const MAX_ENTRIES: usize = 4096;
const MAX_PAYLOAD: usize = 8 * 1024 * 1024;
const MAX_INPUT: usize = 16 * 1024;
const MAX_KEY: usize = 64 * 1024;
const MAX_RUNS: usize = 2048;
const MAX_FONT: usize = 16 * 1024 * 1024;
const MAX_OWNERS: usize = 4;
/// Explicit caller-owned retention; contains no World or compilation-local face.
#[derive(Clone, Default)]
pub struct ShapeSession {
    memo: Arc<Mutex<Memo>>,
}
impl ShapeSession {
    pub fn new() -> Self {
        Self::default()
    }
    /// Cumulative hits, completed publications, retained products.
    pub fn stats(&self) -> (usize, usize, usize) {
        let m = self.memo.lock().unwrap();
        (m.hits, m.published, m.order.len())
    }
}
#[derive(Default)]
struct Memo {
    epoch: Option<Arc<()>>,
    entries: HashMap<EcoString, Vec<Entry>>,
    order: VecDeque<(EcoString, u64)>,
    serial: u64,
    charged: usize,
    owners: HashMap<usize, usize>,
    hits: usize,
    published: usize,
}
struct Entry {
    id: u64,
    key: Key,
    product: Arc<Product>,
    charge: usize,
}
struct Key {
    style: TextStyle,
    bits: Vec<u64>,
    kind: Kind,
}
#[derive(PartialEq, Eq)]
enum Kind {
    Width,
    Emission { position: [u64; 2], context: Context },
}
#[derive(PartialEq, Eq)]
enum Context {
    Absent,
    Unresolved,
    Resolved(Vec<(Range<usize>, bool)>),
}
impl Key {
    fn same(&self, other: &Self) -> bool {
        self.kind == other.kind && self.bits == other.bits && self.style == other.style
    }
}
pub(super) enum Product {
    Width(Pt),
    Emission(Vec<Run>),
}
pub(super) struct Run {
    pos: Point,
    glyphs: SharedGlyphs,
    text: EcoString,
    units_per_em: u16,
    font: Font,
    family: Option<FontList>,
    range: Option<Range<usize>>,
}
impl Product {
    pub(super) fn emitted(
        items: &[FrameItem],
        style: &TextStyle,
        bidi: Option<&BidiSpan>,
    ) -> Option<Self> {
        if items.len() > MAX_RUNS {
            return None;
        }
        let mut runs = Vec::with_capacity(items.len());
        for item in items {
            let FrameItem::TextShaped {
                pos,
                glyphs,
                style: actual,
                text,
                units_per_em,
                font: Some(font),
                bidi: span,
            } = item
            else {
                return None;
            };
            if !matches!(font, Font::Vec(_)) || font.as_slice().len() > MAX_FONT {
                return None;
            }
            let mut expected = style.clone();
            expected.font = actual.font.clone();
            if *actual != expected {
                return None;
            }
            let range = if let Some(span) = span {
                let source = bidi?;
                if !Arc::ptr_eq(source.input(), span.input())
                    || source.segment() != span.segment()
                {
                    return None;
                }
                let r = span.range();
                let original = source.range();
                if r.start < original.start || r.end > original.end {
                    return None;
                }
                Some(r.start - original.start..r.end - original.start)
            } else {
                None
            };
            runs.push(Run {
                pos: *pos,
                glyphs: glyphs.clone(),
                text: text.clone(),
                units_per_em: *units_per_em,
                font: font.clone(),
                family: actual.font.clone(),
                range,
            });
        }
        Some(Self::Emission(runs))
    }
    pub(super) fn materialize(
        &self,
        style: &TextStyle,
        bidi: Option<&BidiSpan>,
    ) -> Option<Vec<FrameItem>> {
        let Self::Emission(runs) = self else {
            return None;
        };
        runs.iter()
            .map(|run| {
                let span = match &run.range {
                    Some(r) => Some(bidi?.subspan(r.clone())?),
                    None => None,
                };
                let mut actual = style.clone();
                actual.font = run.family.clone();
                Some(FrameItem::TextShaped {
                    pos: run.pos,
                    glyphs: run.glyphs.clone(),
                    style: actual,
                    text: run.text.clone(),
                    units_per_em: run.units_per_em,
                    font: Some(run.font.clone()),
                    bidi: span,
                })
            })
            .collect()
    }
    fn charge(&self) -> Option<usize> {
        let mut size = std::mem::size_of::<Self>();
        if let Self::Emission(runs) = self {
            for r in runs {
                size = size
                    .checked_add(std::mem::size_of::<Run>())?
                    .checked_add(r.text.len())?
                    .checked_add(
                        r.glyphs.len().checked_mul(std::mem::size_of::<
                            typst_core::entities::layout_types::ShapedGlyph,
                        >())?,
                    )?;
                if let Some(family) = &r.family {
                    size = size.checked_add(font_list_charge(family)?)?;
                }
            }
        }
        Some(size)
    }
    fn owners(&self, owners: &mut Vec<usize>) {
        if let Self::Emission(runs) = self {
            for r in runs {
                if let Font::Vec(owner) = &r.font {
                    let id = Arc::as_ptr(owner) as usize;
                    if !owners.contains(&id) {
                        owners.push(id);
                    }
                }
            }
        }
    }
}
pub(super) struct Request {
    session: ShapeSession,
    epoch: Arc<()>,
    text: EcoString,
    key: Key,
    charge: usize,
}
fn same_epoch(world: &dyn World, epoch: &Arc<()>) -> bool {
    world.font_query_epoch().is_some_and(|now| Arc::ptr_eq(&now, epoch))
}
impl Request {
    pub(super) fn width(
        session: Option<&ShapeSession>,
        world: &dyn World,
        text: &str,
        style: &TextStyle,
    ) -> Option<Self> {
        Self::new(session?, world, text, style, Kind::Width)
    }
    pub(super) fn emission(
        session: Option<&ShapeSession>,
        world: &dyn World,
        text: &str,
        style: &TextStyle,
        pos: &Point,
        bidi: Option<&BidiSpan>,
        resolved: Option<&crate::bidi_context::Resolved<'_>>,
    ) -> Option<Self> {
        let session = session?;
        let context = match (bidi, resolved) {
            (Some(b), Some(r)) => Context::Resolved(r.runs(b, text)?),
            (Some(_), None) => Context::Unresolved,
            (None, _) => Context::Absent,
        };
        if !pos.x.0.is_finite() || !pos.y.0.is_finite() {
            return None;
        }
        Self::new(
            session,
            world,
            text,
            style,
            Kind::Emission {
                position: [pos.x.0.to_bits(), pos.y.0.to_bits()],
                context,
            },
        )
    }
    fn new(
        session: &ShapeSession,
        world: &dyn World,
        text: &str,
        style: &TextStyle,
        kind: Kind,
    ) -> Option<Self> {
        if text.is_empty() || text.len() > MAX_INPUT {
            return None;
        }
        let epoch = world.font_query_epoch()?;
        let (bits, style_charge) = style_identity(style)?;
        let context_charge = match &kind {
            Kind::Emission { context: Context::Resolved(runs), .. } => {
                runs.len().checked_mul(std::mem::size_of::<(Range<usize>, bool)>())?
            }
            _ => 0,
        };
        let charge = style_charge
            .checked_add(text.len())?
            .checked_add(bits.len().checked_mul(8)?)?
            .checked_add(context_charge)?
            .checked_add(std::mem::size_of::<Entry>())?;
        if charge > MAX_KEY {
            return None;
        }
        Some(Self {
            session: session.clone(),
            epoch,
            text: text.into(),
            key: Key { style: style.clone(), bits, kind },
            charge,
        })
    }
    pub(super) fn lookup(&self, world: &dyn World) -> Option<Arc<Product>> {
        if !same_epoch(world, &self.epoch) {
            return None;
        }
        let product = {
            let mut m = self.session.memo.lock().unwrap();
            if !m.epoch.as_ref().is_some_and(|e| Arc::ptr_eq(e, &self.epoch)) {
                return None;
            }
            let product = m
                .entries
                .get(&self.text)?
                .iter()
                .find(|e| e.key.same(&self.key))?
                .product
                .clone();
            m.hits += 1;
            product
        };
        same_epoch(world, &self.epoch).then_some(product)
    }
    pub(super) fn publish(self, world: &dyn World, product: Product) {
        if !same_epoch(world, &self.epoch) {
            return;
        }
        let Some(charge) = product.charge().and_then(|n| n.checked_add(self.charge))
        else {
            return;
        };
        if charge > MAX_PAYLOAD {
            return;
        }
        let mut own = Vec::new();
        product.owners(&mut own);
        if own.len() > MAX_OWNERS {
            return;
        }
        let mut m = self.session.memo.lock().unwrap();
        if !m.epoch.as_ref().is_some_and(|e| Arc::ptr_eq(e, &self.epoch)) {
            m.entries.clear();
            m.order.clear();
            m.charged = 0;
            m.owners.clear();
            m.epoch = Some(self.epoch.clone());
        }
        if m.entries
            .get(&self.text)
            .is_some_and(|es| es.iter().any(|e| e.key.same(&self.key)))
        {
            return;
        }
        loop {
            let owners = m.owners.len()
                + own.iter().filter(|id| !m.owners.contains_key(id)).count();
            if m.order.len() < MAX_ENTRIES
                && m.charged.checked_add(charge).is_some_and(|n| n <= MAX_PAYLOAD)
                && owners <= MAX_OWNERS
            {
                break;
            }
            let Some((text, id)) = m.order.pop_front() else {
                return;
            };
            let entries = m.entries.get_mut(&text).unwrap();
            let i = entries.iter().position(|e| e.id == id).unwrap();
            let old = entries.remove(i);
            let empty = entries.is_empty();
            m.charged -= old.charge;
            let mut removed = Vec::new();
            old.product.owners(&mut removed);
            for id in removed {
                let count = m.owners.get_mut(&id).unwrap();
                *count -= 1;
                if *count == 0 {
                    m.owners.remove(&id);
                }
            }
            if empty {
                m.entries.remove(&text);
            }
        }
        for owner in own {
            *m.owners.entry(owner).or_default() += 1;
        }
        let id = m.serial;
        m.serial = m.serial.wrapping_add(1);
        m.charged += charge;
        m.published += 1;
        m.order.push_back((self.text.clone(), id));
        m.entries.entry(self.text).or_default().push(Entry {
            id,
            key: self.key,
            product: Arc::new(product),
            charge,
        });
    }
}
fn font_list_charge(fonts: &FontList) -> Option<usize> {
    let mut size = 0usize;
    for family in fonts.as_slice() {
        let name = match &family.name {
            FontNamePattern::Literal(n) => n.as_str(),
            FontNamePattern::Regex(_) => return None,
        };
        size = size
            .checked_add(std::mem::size_of_val(family))?
            .checked_add(name.len())?;
        for value in family
            .variants
            .iter()
            .chain(family.variant.iter())
            .chain(family.weight.iter())
            .chain(family.style.iter())
        {
            size = size.checked_add(value.len())?;
        }
    }
    Some(size)
}
fn style_identity(style: &TextStyle) -> Option<(Vec<u64>, usize)> {
    let mut bits = vec![style.size.0.to_bits()];
    let mut size = std::mem::size_of::<TextStyle>();
    for length in [
        Some(style.baseline_offset),
        style.tracking,
        style.leading,
        style.highlight_radius,
        style.highlight_extent,
        style.subscript_size,
        style.superscript_size,
    ]
    .into_iter()
    .flatten()
    {
        bits.extend([length.abs.to_pt().to_bits(), length.em.to_bits()]);
    }
    for edge in [style.top_edge.as_ref(), style.bottom_edge.as_ref()]
        .into_iter()
        .flatten()
    {
        match edge {
            TextEdge::Length(l) => bits.extend([l.abs.to_pt().to_bits(), l.em.to_bits()]),
            TextEdge::Metric(n) => size = size.checked_add(n.len())?,
        }
    }
    // Complex spot-color owners are outside this bounded key; their output stays cold.
    for color in style.fill.iter().chain(style.highlight.iter()) {
        if matches!(color, typst_core::entities::color::Color::Spot(_)) {
            return None;
        }
    }
    if let Some(fonts) = &style.font {
        size = size.checked_add(font_list_charge(fonts)?)?;
    }
    if let Some(vars) = &style.variations {
        size = size.checked_add(vars.0.len().checked_mul(8)?)?;
        bits.extend(vars.0.iter().map(|(_, v)| u64::from(v.to_bits())));
    }
    Some((bits, size))
}

#[cfg(test)]
mod tests {
    use super::ShapeSession;
    use crate::shaper::{
        cache::{FaceCache, ShapeCache},
        shaping::{shaped_width, try_shape_context},
    };
    use std::sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    };
    use typst_core::{
        contracts::world::World,
        entities::{
            bidi_context::{BidiSpan, ParagraphBidiInput, ParagraphBidiSegment},
            content::Content,
            dir::Dir,
            file_id::FileId,
            font_book::{FontBook, FontInfo, FontVariant},
            font_list::{FontList, FontNamePattern},
            font_variations::FontVariations,
            layout_types::{FrameItem, Page, PagedDocument, Point, Pt, TextStyle},
            source::Source,
            world_types::{Bytes, FileResult, Font, Library},
        },
    };
    struct Fixture {
        book: FontBook,
        font: Font,
        library: Library,
        epoch: Arc<()>,
        valid: AtomicBool,
        revoke: AtomicBool,
        queries: AtomicUsize,
    }
    impl Fixture {
        fn new() -> Self {
            let bytes = include_bytes!("../../fixtures/fonts/NotoSans-Regular.ttf");
            let mut info = crate::fonts::font_info_from_bytes(bytes, 0).unwrap();
            info.coverage = crate::fonts::extract_coverage(
                &ttf_parser::Face::parse(bytes, 0).unwrap(),
            );
            let mut book = FontBook::new();
            book.push(info);
            Self {
                book,
                font: Font::from_data(bytes.to_vec()),
                library: Library::new(),
                epoch: Arc::new(()),
                valid: AtomicBool::new(true),
                revoke: AtomicBool::new(false),
                queries: AtomicUsize::new(0),
            }
        }
        fn style(&self) -> TextStyle {
            TextStyle {
                font: Some(FontList::single(self.book.infos()[0].family.clone().into())),
                ..TextStyle::regular(Pt(12.0))
            }
        }
        fn reset(&self) {
            self.queries.store(0, Ordering::SeqCst);
        }
        fn count(&self) -> usize {
            self.queries.load(Ordering::SeqCst)
        }
    }
    impl World for Fixture {
        fn font_query_epoch(&self) -> Option<Arc<()>> {
            self.valid.load(Ordering::SeqCst).then(|| self.epoch.clone())
        }
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            self.queries.fetch_add(1, Ordering::SeqCst);
            &self.book
        }
        fn select_font(&self, p: &FontNamePattern, v: &FontVariant) -> Option<usize> {
            self.queries.fetch_add(1, Ordering::SeqCst);
            self.book.select_pattern(p, v)
        }
        fn font_info(&self, i: usize) -> Option<&FontInfo> {
            self.queries.fetch_add(1, Ordering::SeqCst);
            self.book.infos().get(i)
        }
        fn font(&self, i: usize) -> Option<Font> {
            self.queries.fetch_add(1, Ordering::SeqCst);
            if self.revoke.swap(false, Ordering::SeqCst) {
                self.valid.store(false, Ordering::SeqCst);
            }
            (i == 0).then(|| self.font.clone())
        }
        fn today(
            &self,
            _: Option<typst_core::entities::duration::Duration>,
        ) -> Option<typst_core::entities::world_types::Datetime> {
            None
        }
        fn main(&self) -> FileId {
            panic!("not used")
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            panic!("not used")
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            panic!("not used")
        }
    }
    fn faces(s: &ShapeSession) -> FaceCache {
        let mut f = FaceCache::new();
        f.products = Some(s.clone());
        f
    }
    fn width(w: &Fixture, s: &ShapeSession, text: &str, style: &TextStyle) -> Pt {
        shaped_width(w, text, style, &mut faces(s)).unwrap()
    }
    fn emit(
        w: &Fixture,
        s: &ShapeSession,
        text: &str,
        style: &TextStyle,
        pos: Point,
        bidi: Option<&BidiSpan>,
        resolved: Option<&crate::bidi_context::Resolved<'_>>,
    ) -> Vec<FrameItem> {
        try_shape_context(
            w,
            &pos,
            &text.into(),
            style,
            &mut ShapeCache::new(),
            &mut faces(s),
            bidi,
            resolved,
        )
        .unwrap()
    }
    fn signature(items: &[FrameItem]) -> Vec<String> {
        items
            .iter()
            .map(|i| match i {
                FrameItem::TextShaped {
                    pos, text, style, glyphs, units_per_em, ..
                } => format!("{pos:?}/{text:?}/{style:?}/{glyphs:?}/{units_per_em}"),
                _ => panic!("unexpected output"),
            })
            .collect()
    }
    #[test]
    fn shaping_products_width_survives_fresh_faces_and_preserves_current_value() {
        let w = Fixture::new();
        let s = ShapeSession::default();
        let st = w.style();
        let a = width(&w, &s, "office", &st);
        assert!(w.count() > 0);
        w.reset();
        assert_eq!(width(&w, &s, "office", &st), a);
        assert_eq!(
            w.count(),
            0,
            "complete width must survive fresh per-compilation faces"
        );
        assert_eq!(width(&w, &ShapeSession::default(), "office", &st), a);
        assert!(w.count() > 0);
    }
    #[test]
    fn shaping_products_emission_survives_fresh_faces_without_selection_replay() {
        let w = Fixture::new();
        let s = ShapeSession::default();
        let st = w.style();
        let p = Point { x: Pt(20.25), y: Pt(40.125) };
        let a = emit(&w, &s, "office", &st, p, None, None);
        assert!(!a.is_empty());
        w.reset();
        let b = emit(&w, &s, "office", &st, p, None, None);
        assert_eq!(signature(&a), signature(&b));
        assert_eq!(
            w.count(),
            0,
            "complete emission must bypass selection and reconstruction"
        );
    }
    #[test]
    fn shaping_products_changed_epoch_absence_and_position_match_fresh() {
        let w = Fixture::new();
        let s = ShapeSession::default();
        let st = w.style();
        emit(&w, &s, "office", &st, Point::ZERO, None, None);
        for p in [Point { x: Pt(1.0), y: Pt(0.0) }, Point { x: Pt(0.0), y: Pt(2.0) }] {
            w.reset();
            let a = emit(&w, &s, "office", &st, p, None, None);
            assert!(w.count() > 0);
            let b = emit(&w, &ShapeSession::default(), "office", &st, p, None, None);
            assert_eq!(signature(&a), signature(&b));
        }
        let other = Fixture::new();
        other.reset();
        width(&other, &s, "office", &st);
        assert!(other.count() > 0);
        w.valid.store(false, Ordering::SeqCst);
        w.reset();
        emit(&w, &s, "office", &st, Point::ZERO, None, None);
        assert!(w.count() > 0);
    }
    #[test]
    fn shaping_products_style_and_exact_axes_do_not_contaminate_width_or_emission() {
        let w = Fixture::new();
        let s = ShapeSession::default();
        let base = w.style();
        width(&w, &s, "office", &base);
        emit(&w, &s, "office", &base, Point::ZERO, None, None);
        for st in [
            TextStyle { size: Pt(24.0), ..base.clone() },
            TextStyle { lang: Some("sr".parse().unwrap()), ..base.clone() },
            TextStyle { dir: Some(Dir::RTL), ..base.clone() },
            TextStyle {
                variations: Some(FontVariations(vec![(
                    *b"wght",
                    f32::from_bits(1137180672),
                )])),
                ..base.clone()
            },
            TextStyle {
                variations: Some(FontVariations(vec![(
                    *b"wght",
                    f32::from_bits(1137180673),
                )])),
                ..base.clone()
            },
        ] {
            w.reset();
            let a = width(&w, &s, "office", &st);
            assert!(w.count() > 0);
            assert_eq!(a, width(&w, &ShapeSession::default(), "office", &st));
            let a = emit(&w, &s, "office", &st, Point::ZERO, None, None);
            let b = emit(
                &w,
                &ShapeSession::default(),
                "office",
                &st,
                Point::ZERO,
                None,
                None,
            );
            assert_eq!(signature(&a), signature(&b));
        }
    }
    #[test]
    fn shaping_products_revocation_during_computation_never_publishes() {
        let w = Fixture::new();
        let s = ShapeSession::default();
        let st = w.style();
        w.revoke.store(true, Ordering::SeqCst);
        width(&w, &s, "office", &st);
        assert!(!w.valid.load(Ordering::SeqCst));
        w.valid.store(true, Ordering::SeqCst);
        w.reset();
        width(&w, &s, "office", &st);
        assert!(w.count() > 0, "expired result must be recomputed");
        w.reset();
        width(&w, &s, "office", &st);
        assert_eq!(w.count(), 0, "complete retry may now be retained");
    }
    fn span(text: &str) -> BidiSpan {
        BidiSpan::new(
            Arc::new(
                ParagraphBidiInput::new(
                    Dir::LTR,
                    vec![ParagraphBidiSegment::Text { text: text.into(), dir: Dir::LTR }],
                )
                .unwrap(),
            ),
            0,
            0..text.len(),
        )
        .unwrap()
    }
    fn doc(span: BidiSpan, style: TextStyle) -> PagedDocument {
        let text = match &span.input().segments()[span.segment()] {
            ParagraphBidiSegment::Text { text, .. } => text[span.range()].into(),
            _ => panic!("fixture needs text"),
        };
        PagedDocument::new(vec![Page {
            width: 200.0,
            height: 100.0,
            logical_number: 1,
            numbering: None,
            supplement: Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items: vec![FrameItem::Text {
                pos: Point::ZERO,
                text,
                style,
                bidi: Some(span),
            }],
        }])
    }
    #[test]
    fn shaping_products_current_bidi_owner_is_rebound_on_completed_hit() {
        let w = Fixture::new();
        let s = ShapeSession::default();
        let st = w.style();
        let a = span("office");
        let da = doc(a.clone(), st.clone());
        let ra = crate::bidi_context::Registry::new(&da);
        let va = ra.resolve();
        let x = emit(&w, &s, "office", &st, Point::ZERO, Some(&a), Some(&va));
        let b = span("office");
        assert!(!Arc::ptr_eq(a.input(), b.input()));
        let db = doc(b.clone(), st.clone());
        let rb = crate::bidi_context::Registry::new(&db);
        let vb = rb.resolve();
        w.reset();
        let y = emit(&w, &s, "office", &st, Point::ZERO, Some(&b), Some(&vb));
        assert_eq!(signature(&x), signature(&y));
        for i in &y {
            if let FrameItem::TextShaped { bidi: Some(current), .. } = i {
                assert!(Arc::ptr_eq(current.input(), b.input()));
            } else {
                panic!("missing current bidi");
            }
        }
        assert_eq!(
            w.count(),
            0,
            "equal resolved runs may reuse while retaining current causal owner"
        );
    }
    #[test]
    fn shaping_products_changed_resolved_direction_matches_fresh() {
        let w = Fixture::new();
        let s = ShapeSession::default();
        let st = w.style();
        let make = |dir| {
            BidiSpan::new(
                Arc::new(
                    ParagraphBidiInput::new(
                        dir,
                        vec![ParagraphBidiSegment::Text { text: "(".into(), dir }],
                    )
                    .unwrap(),
                ),
                0,
                0..1,
            )
            .unwrap()
        };
        let left = make(Dir::LTR);
        let dl = doc(left.clone(), st.clone());
        let rl = crate::bidi_context::Registry::new(&dl);
        let vl = rl.resolve();
        let a = emit(&w, &s, "(", &st, Point::ZERO, Some(&left), Some(&vl));
        let right = make(Dir::RTL);
        let dr = doc(right.clone(), st.clone());
        let rr = crate::bidi_context::Registry::new(&dr);
        let vr = rr.resolve();
        w.reset();
        let b = emit(&w, &s, "(", &st, Point::ZERO, Some(&right), Some(&vr));
        assert!(w.count() > 0, "different resolved direction cannot hit");
        assert_ne!(
            signature(&a),
            signature(&b),
            "direction witness must distinguish the actual output"
        );
        let cold = emit(
            &w,
            &ShapeSession::default(),
            "(",
            &st,
            Point::ZERO,
            Some(&right),
            Some(&vr),
        );
        assert_eq!(signature(&b), signature(&cold));
        w.reset();
        let again = emit(&w, &s, "(", &st, Point::ZERO, Some(&right), Some(&vr));
        assert_eq!(signature(&b), signature(&again));
        assert_eq!(w.count(), 0);
    }
    #[test]
    fn shaping_products_capacity_and_payload_evict_old_complete_products() {
        use super::{Product, Request, MAX_ENTRIES, MAX_INPUT, MAX_PAYLOAD};
        let w = Fixture::new();
        let s = ShapeSession::default();
        let st = w.style();
        for i in 0..=MAX_ENTRIES {
            Request::width(Some(&s), &w, &format!("word-{i}"), &st)
                .unwrap()
                .publish(&w, Product::Width(Pt(i as f64)));
        }
        assert!(s.stats().2 <= MAX_ENTRIES);
        assert!(Request::width(Some(&s), &w, "word-0", &st)
            .unwrap()
            .lookup(&w)
            .is_none());
        assert!(Request::width(Some(&s), &w, &format!("word-{MAX_ENTRIES}"), &st)
            .unwrap()
            .lookup(&w)
            .is_some());
        let long = "x".repeat(MAX_INPUT - 16);
        for i in 0..1024 {
            Request::width(Some(&s), &w, &format!("{long}{i}"), &st)
                .unwrap()
                .publish(&w, Product::Width(Pt(1.0)));
        }
        assert!(s.memo.lock().unwrap().charged <= MAX_PAYLOAD);
        assert!(Request::width(Some(&s), &w, &"x".repeat(MAX_INPUT + 1), &st).is_none());
        assert!(Request::width(Some(&s), &w, "word-4096", &st)
            .unwrap()
            .lookup(&w)
            .is_none());
    }
    #[test]
    fn shaping_products_own_fonts_until_session_drop_and_sessions_are_independent() {
        let w = Fixture::new();
        let st = w.style();
        let s = ShapeSession::default();
        let weak = match &w.font {
            Font::Vec(f) => Arc::downgrade(f),
            Font::Mmap(_) => panic!("fixture"),
        };
        emit(&w, &s, "office", &st, Point::ZERO, None, None);
        w.reset();
        emit(&w, &ShapeSession::default(), "office", &st, Point::ZERO, None, None);
        assert!(w.count() > 0);
        drop(w);
        assert!(
            weak.upgrade().is_some(),
            "completed product must keep glyph source alive"
        );
        drop(s);
        assert!(
            weak.upgrade().is_none(),
            "caller-owned session must release retained source"
        );
    }
    #[test]
    fn shaping_products_empty_success_stays_distinct_from_uncached_failure() {
        use super::{Product, Request};
        let w = Fixture::new();
        let st = w.style();
        let s = ShapeSession::default();
        let request =
            Request::emission(Some(&s), &w, "\u{2060}", &st, &Point::ZERO, None, None)
                .unwrap();
        request.publish(&w, Product::emitted(&[], &st, None).unwrap());
        let request =
            Request::emission(Some(&s), &w, "\u{2060}", &st, &Point::ZERO, None, None)
                .unwrap();
        assert!(request.lookup(&w).unwrap().materialize(&st, None).unwrap().is_empty());
        let before = s.stats();
        assert!(try_shape_context(
            &w,
            &Point::ZERO,
            &" ".into(),
            &st,
            &mut ShapeCache::new(),
            &mut faces(&s),
            None,
            None
        )
        .is_none());
        assert_eq!(
            s.stats(),
            before,
            "whitespace fallback must not become an empty success"
        );
    }
}
