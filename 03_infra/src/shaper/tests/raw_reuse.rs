//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper/tests/raw_reuse.md
//! @prompt-hash 5fda9f0d
//! @layer L3
use super::super::{
    cache::{FaceCache, ShapeCache},
    shaping::{shaped_width, try_shape},
};
use typst_core::contracts::world::World;
use typst_core::entities::{
    dir::Dir,
    duration::Duration,
    file_id::FileId,
    font_book::{Coverage, FontBook},
    font_list::FontList,
    font_variations::FontVariations,
    layout_types::{FrameItem, Length, MathSize, Point, Pt, TextStyle},
    source::Source,
    world_types::{Bytes, Datetime, FileResult, Font, Library},
};
std::thread_local! {static CALLS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
pub(crate) fn record_backend() {
    CALLS.with(|c| c.set(c.get() + 1));
}
fn reset() {
    CALLS.with(|c| c.set(0));
}
fn calls() -> usize {
    CALLS.with(|c| c.get())
}
const FONT: &[u8] = include_bytes!("../../../fixtures/fonts/NotoSans-Regular.ttf");
struct Fixture {
    book: FontBook,
    font: Font,
    lib: Library,
}
impl Fixture {
    fn new() -> Self {
        let mut book = FontBook::new();
        let mut info = crate::fonts::font_info_from_bytes(FONT, 0).unwrap();
        let face = ttf_parser::Face::parse(FONT, 0).unwrap();
        let mut cps = Vec::new();
        for table in face.tables().cmap.unwrap().subtables {
            if table.is_unicode() {
                table.codepoints(|c| {
                    if table.glyph_index(c).is_some() {
                        cps.push(c)
                    }
                });
            }
        }
        info.coverage = Coverage::from_codepoints(cps);
        book.push(info);
        Self {
            book,
            font: Font::from_data(FONT.to_vec()),
            lib: Library::new(),
        }
    }
    fn style(&self) -> TextStyle {
        TextStyle {
            font: Some(FontList::single(self.book.infos()[0].family.clone().into())),
            ..TextStyle::regular(Pt(12.0))
        }
    }
}
impl World for Fixture {
    fn library(&self) -> &Library {
        &self.lib
    }
    fn book(&self) -> &FontBook {
        &self.book
    }
    fn font(&self, i: usize) -> Option<Font> {
        (i == 0).then(|| self.font.clone())
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
    fn today(&self, _: Option<Duration>) -> Option<Datetime> {
        panic!("not used")
    }
}
fn emit(w: &Fixture, t: &str, s: &TextStyle, f: &mut FaceCache) -> Vec<FrameItem> {
    try_shape(w, &Point::ZERO, &t.into(), s, &mut ShapeCache::new(), f).unwrap()
}
fn equal_fresh(w: &Fixture, t: &str, s: &TextStyle, items: &[FrameItem]) {
    assert_eq!(
        format!("{items:?}"),
        format!("{:?}", emit(w, t, s, &mut FaceCache::new()))
    );
}
#[test]
fn width_then_emission_skips_backend_and_matches_fresh() {
    let w = Fixture::new();
    let s = w.style();
    let mut f = FaceCache::new();
    reset();
    let width = shaped_width(&w, "office", &s, &mut f).unwrap();
    assert!(width.0 > 0.0);
    assert_eq!(calls(), 1);
    let result = emit(&w, "office", &s, &mut f);
    assert_eq!(calls(), 1, "measurement must supply raw emission without reshaping");
    equal_fresh(&w, "office", &s, &result);
}
#[test]
fn tracking_and_size_are_applied_after_raw_reuse() {
    let w = Fixture::new();
    let s = w.style();
    let mut f = FaceCache::new();
    shaped_width(&w, "office", &s, &mut f).unwrap();
    for tracking in [0.1, -0.05] {
        let styled = TextStyle {
            tracking: Some(Length::em(tracking)),
            size: Pt(22.0),
            ..s.clone()
        };
        reset();
        let out = emit(&w, "office", &styled, &mut f);
        assert_eq!(calls(), 0, "tracking/scale must not reshape the raw run");
        equal_fresh(&w, "office", &styled, &out);
    }
}
#[test]
fn language_and_math_script_do_not_alias() {
    let w = Fixture::new();
    let s = w.style();
    for changed in [
        TextStyle { lang: Some("en".parse().unwrap()), ..s.clone() },
        TextStyle { math: true, ..s.clone() },
    ] {
        let mut f = FaceCache::new();
        shaped_width(&w, "office", &s, &mut f).unwrap();
        reset();
        let out = emit(&w, "office", &changed, &mut f);
        assert!(calls() > 0);
        equal_fresh(&w, "office", &changed, &out);
    }
}
#[test]
fn distinct_font_owners_and_caches_do_not_alias() {
    let a = Fixture::new();
    let b = Fixture::new();
    let s = a.style();
    let mut f = FaceCache::new();
    shaped_width(&a, "office", &s, &mut f).unwrap();
    reset();
    let out = emit(&b, "office", &s, &mut f);
    assert!(calls() > 0);
    equal_fresh(&b, "office", &s, &out);
    reset();
    emit(&a, "office", &s, &mut FaceCache::new());
    assert!(calls() > 0);
}
#[test]
fn unicode_direction_and_ssty_keep_fresh_results() {
    let w = Fixture::new();
    let s = w.style();
    for (t, style) in [
        ("e\u{301}ffi", s.clone()),
        ("a\u{ad}b", s.clone()),
        ("אבג", TextStyle { dir: Some(Dir::RTL), ..s.clone() }),
        (
            "123",
            TextStyle {
                math: true,
                math_size: MathSize::Script,
                ..s.clone()
            },
        ),
    ] {
        let mut f = FaceCache::new();
        shaped_width(&w, t, &style, &mut f).unwrap();
        let out = emit(&w, t, &style, &mut f);
        equal_fresh(&w, t, &style, &out);
    }
}

#[test]
fn exact_axis_bits_and_feature_policy_do_not_alias() {
    let w = Fixture::new();
    let base = w.style();
    let mut f = FaceCache::new();
    reset();
    for bits in [1137180672, 1137180673] {
        let s = TextStyle {
            variations: Some(FontVariations(vec![(*b"wght", f32::from_bits(bits))])),
            ..base.clone()
        };
        shaped_width(&w, "office", &s, &mut f).unwrap();
    }
    assert_eq!(calls(), 2, "adjacent requested coordinates cannot alias");
    let s = TextStyle {
        math: true,
        math_size: MathSize::Script,
        ..base.clone()
    };
    let out = emit(&w, "123", &s, &mut f);
    equal_fresh(&w, "123", &s, &out);
}

#[test]
fn metrics_clone_supplies_raw_store_to_walker_only() {
    use typst_core::compiler::layout::FontMetrics;
    let w = Fixture::new();
    let style = w.style();
    let metrics = crate::font_metrics::FallbackFontMetrics::new(&w);
    let sibling = metrics.clone();
    metrics.advance_shaped("office", style.size, &style).unwrap();
    let doc = super::doc_with(vec![FrameItem::Text {
        pos: Point::ZERO,
        text: "office".into(),
        style: style.clone(),
        bidi: None,
    }]);
    let registry = crate::bidi_context::Registry::new(&doc);
    let resolved = registry.resolve();
    reset();
    let out = sibling.shape_document_with_context(doc.clone(), &resolved);
    assert_eq!(calls(), 0, "pipeline walker must consume the warmed raw store");
    let fresh = super::super::shape_document(&w, doc);
    assert_eq!(format!("{out:?}"), format!("{fresh:?}"));
    assert!(calls() > 0, "fresh independent cache must still shape");
}
#[test]
fn raw_store_keeps_owner_alive_then_releases_at_compilation_end() {
    let w = Fixture::new();
    let weak = match &w.font {
        Font::Vec(a) => std::sync::Arc::downgrade(a),
        _ => unreachable!(),
    };
    let style = w.style();
    let mut faces = FaceCache::new();
    shaped_width(&w, "office", &style, &mut faces).unwrap();
    let raw = faces.raw_store();
    drop(faces);
    drop(w);
    assert!(weak.upgrade().is_some(), "cached font owner prevents token reuse");
    drop(raw);
    assert!(weak.upgrade().is_none(), "no cross-compilation retention");
}

#[test]
fn failed_parse_is_not_published_and_retry_can_succeed() {
    use super::super::cache::{RawShapeCache, RawShapeInput};
    fn request(font: &Font) -> RawShapeInput<'_> {
        RawShapeInput {
            text: "office",
            slot: 0,
            font,
            axes: &[],
            rtl: false,
            language: None,
            script: None,
            features: &[],
        }
    }
    let mut font = Font::from_data(vec![0; 4]);
    let mut cache = RawShapeCache::default();
    assert!(cache.shape(request(&font)).is_none());
    if let Font::Vec(owner) = &mut font {
        let data =
            std::sync::Arc::get_mut(owner).expect("failed requests retain no font");
        data.clear();
        data.extend_from_slice(FONT);
    }
    assert!(cache.shape(request(&font)).is_some());
    if let Font::Vec(owner) = &mut font {
        assert!(
            std::sync::Arc::get_mut(owner).is_none(),
            "published owner is retained immutable"
        );
    }
}

#[test]
fn empty_success_is_not_a_parse_failure() {
    use super::super::cache::{RawShapeCache, RawShapeInput};
    fn request(font: &Font) -> RawShapeInput<'_> {
        RawShapeInput {
            text: "\u{ad}",
            slot: 0,
            font,
            axes: &[],
            rtl: false,
            language: None,
            script: None,
            features: &[],
        }
    }
    let font = Font::from_data(FONT.to_vec());
    let mut cache = RawShapeCache::default();
    let empty = cache.shape(request(&font)).expect("successful empty shaping");
    assert!(empty.glyph_infos().is_empty());
    let again = cache.shape(request(&font)).expect("cached empty success");
    assert!(std::sync::Arc::ptr_eq(&empty, &again));
    let invalid = Font::from_data(vec![0; 4]);
    assert!(cache.shape(request(&invalid)).is_none());
}

std::thread_local! {static PRIMARY_PROOFS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
pub(crate) fn record_primary_proof() {
    PRIMARY_PROOFS.with(|count| count.set(count.get() + 1));
}
fn proof_count() -> usize {
    PRIMARY_PROOFS.with(|count| count.get())
}
fn reset_proofs() {
    PRIMARY_PROOFS.with(|count| count.set(0));
}

#[test]
fn primary_proof_repeated_width_and_emission_compute_once() {
    let w = Fixture::new();
    let s = w.style();
    let mut f = FaceCache::new();
    reset_proofs();
    let width = shaped_width(&w, "office", &s, &mut f).unwrap();
    let mut outputs = Vec::new();
    for _ in 0..8 {
        outputs.push(emit(&w, "office", &s, &mut f));
    }
    assert_eq!(proof_count(), 1, "repeat occurrences must reuse the completed proof");
    assert_eq!(width, shaped_width(&w, "office", &s, &mut FaceCache::new()).unwrap());
    for output in outputs {
        equal_fresh(&w, "office", &s, &output);
    }
}

#[test]
fn primary_proof_new_face_cache_starts_empty_even_with_shared_raw_store() {
    let w = Fixture::new();
    let s = w.style();
    let mut first = FaceCache::new();
    reset_proofs();
    shaped_width(&w, "office", &s, &mut first).unwrap();
    let mut second = FaceCache::with_raw_store(first.raw_store());
    let out = emit(&w, "office", &s, &mut second);
    assert_eq!(proof_count(), 2, "proofs do not follow the raw-store handle");
    equal_fresh(&w, "office", &s, &out);
}

#[test]
fn primary_proof_exact_policy_inputs_reuse_only_equal_requests() {
    let w = Fixture::new();
    let base = w.style();
    let mut requests = vec![
        ("office", base.clone()),
        ("office!", base.clone()),
        ("office", TextStyle { dir: Some(Dir::LTR), ..base.clone() }),
        ("office", TextStyle { dir: Some(Dir::RTL), ..base.clone() }),
        ("office", TextStyle { lang: Some("en".parse().unwrap()), ..base.clone() }),
        ("office", TextStyle { lang: Some("ru".parse().unwrap()), ..base.clone() }),
    ];
    for bits in [1137180672, 1137180673] {
        requests.push((
            "office",
            TextStyle {
                variations: Some(FontVariations(vec![(*b"wght", f32::from_bits(bits))])),
                ..base.clone()
            },
        ));
    }
    let mut cache = FaceCache::new();
    reset_proofs();
    let mut widths = Vec::new();
    for (text, style) in &requests {
        widths.push(shaped_width(&w, text, style, &mut cache).unwrap());
    }
    assert_eq!(
        proof_count(),
        requests.len(),
        "each distinct raw proof needs computation"
    );
    for ((text, style), expected) in requests.iter().zip(&widths).rev() {
        assert_eq!(shaped_width(&w, text, style, &mut cache).unwrap(), *expected);
    }
    assert_eq!(proof_count(), requests.len(), "ABBA reuse preserves every exact input");
    for ((text, style), expected) in requests.iter().zip(widths) {
        assert_eq!(
            shaped_width(&w, text, style, &mut FaceCache::new()).unwrap(),
            expected
        );
    }
}

#[test]
fn primary_proof_distinct_actual_font_owners_never_alias() {
    let a = Fixture::new();
    let b = Fixture::new();
    let s = a.style();
    let mut cache = FaceCache::new();
    reset_proofs();
    let mut outputs = Vec::new();
    for world in [&a, &b, &b, &a] {
        outputs.push(emit(world, "office", &s, &mut cache));
    }
    assert_eq!(proof_count(), 2, "equal bytes in distinct owners need separate proofs");
    for (world, output) in [&a, &b, &b, &a].into_iter().zip(outputs) {
        equal_fresh(world, "office", &s, &output);
    }
}
