//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper/tests/shared_glyphs.md
//! @prompt-hash 609ccf77
//! @layer L3
//! @updated 2026-11-07
use crate::font_metrics::FallbackFontMetrics;
use std::sync::atomic::{AtomicUsize, Ordering};
use typst_core::compiler::layout::FontMetrics;
use typst_core::contracts::world::World;
use typst_core::entities::{
    duration::Duration,
    file_id::FileId,
    font_book::{Coverage, FontBook},
    font_list::FontList,
    font_variations::FontVariations,
    layout_types::{Length, MathSize, Pt, TextStyle},
    source::Source,
    world_types::{Bytes, Datetime, FileResult, Font, Library},
};

const DEJAVU: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/fonts/DejaVuSans.ttf"));
const NOTO: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/fixtures/fonts/NotoSans-Regular.ttf"
));
const UBUNTU: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/fixtures/fonts/UbuntuSans-Variable.ttf"
));
const MATH: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/fixtures/fonts/NewCMMath-Book.otf"
));
const NIMBUS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/fixtures/fonts/NimbusSans-Regular.otf"
));
struct Fixture {
    book: FontBook,
    fonts: Vec<Font>,
    library: Library,
    font_calls: AtomicUsize,
}
impl Fixture {
    fn new() -> Self {
        let data = [DEJAVU, NOTO, UBUNTU, MATH, NIMBUS];
        let mut book = FontBook::new();
        for bytes in data {
            let face =
                ttf_parser::Face::parse(bytes, 0).expect("versioned fixture parses");
            let mut info = crate::fonts::font_info_from_bytes(bytes, 0).unwrap();
            let mut codepoints = Vec::new();
            if let Some(cmap) = face.tables().cmap {
                for subtable in cmap.subtables {
                    if subtable.is_unicode() {
                        subtable.codepoints(|cp| {
                            if subtable.glyph_index(cp).is_some() {
                                codepoints.push(cp);
                            }
                        });
                    }
                }
            }
            assert!(!codepoints.is_empty(), "real Unicode cmap required");
            info.coverage = Coverage::from_codepoints(codepoints);
            book.push(info);
        }
        Self {
            book,
            fonts: data.into_iter().map(|b| Font::from_data(b.to_vec())).collect(),
            library: Library::new(),
            font_calls: AtomicUsize::new(0),
        }
    }
}
impl World for Fixture {
    fn library(&self) -> &Library {
        &self.library
    }
    fn book(&self) -> &FontBook {
        &self.book
    }
    fn main(&self) -> FileId {
        panic!("not called by metrics")
    }
    fn source(&self, _: FileId) -> FileResult<Source> {
        panic!("not called by metrics")
    }
    fn file(&self, _: FileId) -> FileResult<Bytes> {
        panic!("not called by metrics")
    }
    fn font(&self, slot: usize) -> Option<Font> {
        self.font_calls.fetch_add(1, Ordering::Relaxed);
        self.fonts.get(slot).cloned()
    }
    fn today(&self, _: Option<Duration>) -> Option<Datetime> {
        panic!("metrics must not read clock")
    }
    fn candidates_for_char(&self, c: char) -> Vec<usize> {
        self.book
            .infos()
            .iter()
            .enumerate()
            .filter_map(|(i, info)| info.coverage.contains(c as u32).then_some(i))
            .collect()
    }
}
fn style(family: &str) -> TextStyle {
    TextStyle {
        font: Some(FontList::single(family.into())),
        ..TextStyle::regular(Pt(12.0))
    }
}
fn measure(
    m: &dyn FontMetrics,
    shaped: bool,
    text: &str,
    size: Pt,
    s: &TextStyle,
) -> f64 {
    let p = if shaped {
        m.advance_shaped(text, size, s).expect("real fixture shapes")
    } else {
        m.advance(text, size, s)
    };
    assert!(p.0.is_finite() && p.0 >= 0.0);
    p.0
}
fn close(a: f64, b: f64, label: &str) {
    assert!((a - b).abs() < 1e-9, "{label}: {a} != {b}");
}
fn assert_shared_matches_fresh(
    w: &Fixture,
    shaped: bool,
    requests: &[(&str, Pt, TextStyle)],
) -> Vec<f64> {
    let expected: Vec<_> = requests
        .iter()
        .map(|(t, z, s)| measure(&FallbackFontMetrics::new(w), shaped, t, *z, s))
        .collect();
    for reverse in [false, true] {
        let shared = FallbackFontMetrics::new(w);
        let clone = shared.clone();
        let mut indices: Vec<_> = (0..requests.len()).collect();
        if reverse {
            indices.reverse();
        }
        for i in indices.into_iter().cycle().take(requests.len() * 2) {
            let (text, size, s) = &requests[i];
            close(measure(&shared, shaped, text, *size, s), expected[i], "shared/fresh");
            close(measure(&clone, shaped, text, *size, s), expected[i], "clone/fresh");
        }
    }
    expected
}
#[test]
fn explicit_size_real_public_scalar_and_shaped_with_unchanged_authored_style() {
    let w = Fixture::new();
    let s = style("DejaVu Sans");
    for shaped in [false, true] {
        let e = assert_shared_matches_fresh(
            &w,
            shaped,
            &[("abcdefgh", Pt(12.), s.clone()), ("abcdefgh", Pt(24.), s.clone())],
        );
        assert!(e[0] > 1.);
        close(e[1], 2. * e[0], "explicit size scaling");
    }
    assert_eq!(s.size, Pt(12.));
    let face = ttf_parser::Face::parse(DEJAVU, 0).unwrap();
    let expected =
        f64::from(face.glyph_hor_advance(face.glyph_index('H').unwrap()).unwrap()) * 24.
            / f64::from(face.units_per_em());
    close(
        measure(&FallbackFontMetrics::new(&w), false, "H", Pt(24.), &s),
        expected,
        "actual table",
    );
}
#[test]
fn current_public_identity_fields_fresh_shared_both_orders() {
    use typst_core::entities::dir::Dir;
    let w = Fixture::new();
    let base = style("Ubuntu Sans");
    let variants = vec![
        base.clone(),
        TextStyle {
            variations: Some(FontVariations(vec![(*b"wght", 650.)])),
            ..base.clone()
        },
        TextStyle {
            variations: Some(FontVariations(vec![
                (*b"wght", 650.),
                (*b"ital", 1.),
                (*b"opsz", 12.),
                (*b"wdth", 95.),
            ])),
            ..base.clone()
        },
        TextStyle { bold: true, ..base.clone() },
        TextStyle { italic: true, ..base.clone() },
        TextStyle { weight: Some(700), ..base.clone() },
        TextStyle { font: style("DejaVu Sans").font, ..base.clone() },
        TextStyle { lang: Some("sr".parse().unwrap()), ..base.clone() },
        TextStyle { dir: Some(Dir::RTL), ..base.clone() },
        TextStyle { math: true, ..base.clone() },
        TextStyle { superscript: true, ..base.clone() },
        TextStyle {
            superscript: true,
            superscript_native: Some(false),
            ..base.clone()
        },
    ];
    let requests: Vec<_> =
        variants.into_iter().map(|s| ("Weight123", Pt(12.), s)).collect();
    for shaped in [false, true] {
        let e = assert_shared_matches_fresh(&w, shaped, &requests);
        assert!(
            (e[0] - e[1]).abs() > 0.01,
            "variable font must discriminate explicit axes"
        );
        assert!((e[0] - e[6]).abs() > 0.01, "real families must discriminate");
    }
}
#[test]
fn advance_math_textitem_and_ssty_are_distinct_cached_contexts() {
    let w = Fixture::new();
    let s = TextStyle { math: true, ..style("New Computer Modern Math") };
    let mut discriminates_textitem = false;
    let mut discriminates_ssty = false;
    for text in ["𝑥", "𝑓", "𝑧", "x", "1", "2", "123"] {
        let requests = [
            (text, Pt(12.), s.clone()),
            (text, Pt(12.), TextStyle { math_text_item: true, ..s.clone() }),
            (text, Pt(12.), TextStyle { math_size: MathSize::Script, ..s.clone() }),
            (text, Pt(12.), TextStyle { math_size: MathSize::ScriptScript, ..s.clone() }),
        ];
        let e = assert_shared_matches_fresh(&w, false, &requests);
        discriminates_textitem |= (e[0] - e[1]).abs() > 1e-8;
        discriminates_ssty |= (e[0] - e[2]).abs() > 1e-8 || (e[0] - e[3]).abs() > 1e-8;
    }
    assert!(
        discriminates_textitem,
        "fixture must witness actual italic-correction provenance"
    );
    assert!(discriminates_ssty, "fixture must witness actual ssty substitution");
    // Shaped key does not promise these advance-only fields; do not invent that contract.
}
#[test]
fn shaped_tracking_zero_and_signed_tracking_preserve_fresh_measurement() {
    let w = Fixture::new();
    let s = style("DejaVu Sans");
    let requests = [
        ("abcdefgh", Pt(12.), s.clone()),
        ("abcdefgh", Pt(12.), TextStyle { tracking: Some(Length::pt(0.)), ..s.clone() }),
        ("abcdefgh", Pt(12.), TextStyle { tracking: Some(Length::pt(0.2)), ..s.clone() }),
        (
            "abcdefgh",
            Pt(12.),
            TextStyle { tracking: Some(Length::pt(-0.2)), ..s.clone() },
        ),
    ];
    let e = assert_shared_matches_fresh(&w, true, &requests);
    close(e[0], e[1], "zero tracking changes no width");
    // The private CLI tracking control verifies that even Some(0pt) bypasses.
}
#[test]
fn warmed_public_calls_do_not_reload_fonts_and_clone_keeps_face_sharing() {
    let w = Fixture::new();
    let s = style("DejaVu Sans");
    for shaped in [false, true] {
        let m = FallbackFontMetrics::new(&w);
        let first = measure(&m, shaped, "abcdefgh", Pt(12.), &s);
        let calls = w.font_calls.load(Ordering::Relaxed);
        assert!(calls > 0);
        let clone = m.clone();
        for _ in 0..4 {
            close(
                measure(&clone, shaped, "abcdefgh", Pt(12.), &s),
                first,
                "warm public result",
            );
        }
        assert_eq!(
            w.font_calls.load(Ordering::Relaxed),
            calls,
            "warm immutable public query must not reload face"
        );
    }
}

// Independent full emitted-glyph controls: cache sharing occurs within one document.
use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, Point};
fn item(t: &str, s: TextStyle) -> FrameItem {
    FrameItem::Text {
        pos: Point { x: Pt(72.), y: Pt(72.) },
        text: t.into(),
        style: s,
        bidi: None,
    }
}
fn page(items: Vec<FrameItem>) -> Page {
    Page {
        width: 595.,
        height: 842.,
        logical_number: 1,
        numbering: None,
        supplement: typst_core::entities::content::Content::Empty,
        bleed: Default::default(),
        fill: Default::default(),
        background: vec![],
        foreground: vec![],
        items,
    }
}
fn snapshots(w: &Fixture, requests: &[(String, TextStyle)]) -> Vec<String> {
    let d = PagedDocument::new(
        requests.iter().map(|(t, s)| page(vec![item(t, s.clone())])).collect(),
    );
    crate::shaper::shape_document(w, d)
        .pages
        .into_iter()
        .map(|p| {
            // Debug includes full glyph records, char_code, source text, font bytes and style.
            format!("{:?}", p.items)
        })
        .collect()
}
#[test]
fn full_glyph_signatures_fresh_vs_document_cache_in_both_orders() {
    use typst_core::entities::dir::Dir;
    let w = Fixture::new();
    let base = style("DejaVu Sans");
    let mut requests = Vec::new();
    for t in [
        "office affine AV",
        "ffi",
        "a\u{301}",
        "العربية",
        "אבג",
        "abc אבג 123",
        "f\u{ad}fi",
        "\u{200f}\u{202c}",
        " ",
        "",
    ] {
        requests.push((t.to_string(), base.clone()));
    }
    for lang in [None, Some("ru"), Some("sr")] {
        requests.push((
            "б".into(),
            TextStyle {
                lang: lang.map(|x| x.parse().unwrap()),
                ..style("Noto Sans")
            },
        ));
    }
    requests.push(("العربية".into(), TextStyle { dir: Some(Dir::RTL), ..base.clone() }));
    for tracking in [-0.2, 0., 0.2] {
        requests.push((
            "office a\u{301}".into(),
            TextStyle {
                tracking: Some(Length::pt(tracking)),
                ..base.clone()
            },
        ));
    }
    for level in [MathSize::Script, MathSize::ScriptScript] {
        requests.push((
            "123".into(),
            TextStyle {
                math: true,
                math_size: level,
                ..style("New Computer Modern Math")
            },
        ));
    }
    for native in [None, Some(false), Some(true)] {
        requests.push((
            "123".into(),
            TextStyle {
                superscript: true,
                superscript_native: native,
                ..style("Nimbus Sans")
            },
        ));
    }
    requests.push((
        "AV".into(),
        TextStyle {
            variations: Some(FontVariations(vec![
                (*b"wght", 650.),
                (*b"wdth", 95.),
                (*b"ital", 1.),
                (*b"opsz", 12.),
            ])),
            ..style("Ubuntu Sans")
        },
    ));
    requests.push(("fallback".into(), TextStyle { font: None, ..base.clone() }));
    requests.push(("missing".into(), style("Nonexistent Face C05")));
    let expected: Vec<_> = requests
        .iter()
        .map(|r| snapshots(&w, std::slice::from_ref(r)).remove(0))
        .collect();
    for reverse in [false, true] {
        let mut indices: Vec<_> = (0..requests.len()).collect();
        if reverse {
            indices.reverse();
        }
        let indices: Vec<_> =
            indices.into_iter().cycle().take(requests.len() * 3).collect();
        let batch: Vec<_> = indices.iter().map(|&i| requests[i].clone()).collect();
        for (actual, i) in snapshots(&w, &batch).into_iter().zip(indices) {
            assert_eq!(
                actual, expected[i],
                "context {i}: complete output changed on shared cache"
            );
        }
    }
}

#[test]
fn unavailable_font_and_successful_empty_output_remain_distinct() {
    let mut unavailable = Fixture::new();
    unavailable.fonts.clear();
    let request = ("office".to_string(), style("DejaVu Sans"));
    let result = snapshots(&unavailable, std::slice::from_ref(&request));
    assert!(result[0].contains("Text {"), "unavailable bytes must preserve text carrier");
    assert!(!result[0].contains("TextShaped {"));
    let w = Fixture::new();
    // Exercise successful zero-glyph author-control shaping without replacing its
    // policy with missing-font or empty-whitespace fallback.
    let controls = ("\u{200f}\u{202c}".to_string(), style("DejaVu Sans"));
    let empty = (String::new(), style("DejaVu Sans"));
    let e = snapshots(&w, &[controls.clone(), empty.clone()]);
    let shared = snapshots(&w, &[controls.clone(), empty.clone(), controls, empty]);
    assert_eq!(shared[0], e[0]);
    assert_eq!(shared[1], e[1]);
    assert_eq!(shared[2], e[0]);
    assert_eq!(shared[3], e[1]);
}

#[test]
fn actual_native_sups_advance_and_forced_refusal_match_real_font_tables() {
    let provider = crate::font_metrics::FontBookMetrics::from_bytes(NIMBUS).unwrap();
    let face = ttf_parser::Face::parse(NIMBUS, 0).unwrap();
    let rb = rustybuzz::Face::from_slice(NIMBUS, 0).unwrap();
    let mut buffer = rustybuzz::UnicodeBuffer::new();
    buffer.push_str("123");
    let feat = rustybuzz::Feature::new(ttf_parser::Tag::from_bytes(b"sups"), 1, ..);
    let shaped = rustybuzz::shape(&rb, &[feat], buffer);
    assert_eq!(shaped.glyph_infos().len(), 3, "actual whole-run native digit coverage");
    let native_units: u32 = shaped
        .glyph_infos()
        .iter()
        .map(|g| {
            u32::from(
                face.glyph_hor_advance(ttf_parser::GlyphId(g.glyph_id as u16))
                    .unwrap(),
            )
        })
        .sum();
    let ordinary_units: u32 = "123"
        .chars()
        .map(|c| u32::from(face.glyph_hor_advance(face.glyph_index(c).unwrap()).unwrap()))
        .sum();
    assert_ne!(
        native_units, ordinary_units,
        "fixture must discriminate selected native glyphs"
    );
    for size in [11., 17.] {
        let s = TextStyle {
            superscript: true,
            superscript_native: Some(true),
            ..style("Nimbus Sans")
        };
        assert!(provider.text_superscript_metrics("123", Pt(size), &s).3);
        close(
            provider.advance("123", Pt(size), &s).0,
            f64::from(native_units) * size / f64::from(face.units_per_em()),
            "native glyph table advances",
        );
        let refused = TextStyle { superscript_native: Some(false), ..s };
        close(
            provider.advance("123", Pt(size), &refused).0,
            f64::from(ordinary_units) * size / f64::from(face.units_per_em()),
            "forced refusal ordinary table advances",
        );
    }
}

#[test]
fn repeated_document_hits_share_only_payload_and_retained_owners_survive() {
    let w = Fixture::new();
    let base = style("DejaVu Sans");
    let styles =
        [base.clone(), TextStyle { size: Pt(24.), ..base.clone() }, base.clone()];
    let mut inputs = Vec::new();
    for (i, s) in styles.into_iter().enumerate() {
        let mut v = item("office AV", s);
        if let FrameItem::Text { pos, .. } = &mut v {
            pos.x = Pt(30. + i as f64 * 40.);
            pos.y = Pt(20. + i as f64 * 30.);
        }
        inputs.push(v);
    }
    let d = crate::shaper::shape_document(&w, PagedDocument::new(vec![page(inputs)]));
    let runs: Vec<_> = d.pages[0]
        .items
        .iter()
        .filter_map(|x| {
            if let FrameItem::TextShaped { glyphs, pos, style, font, .. } = x {
                Some((glyphs, pos, style, font))
            } else {
                None
            }
        })
        .collect();
    assert_eq!(runs.len(), 3);
    assert!(!runs[0].0.is_empty());
    assert_ne!(runs[0].1, runs[1].1);
    assert_ne!(runs[0].2.size, runs[1].2.size);
    assert_eq!(runs[0].0.as_ptr(), runs[1].0.as_ptr());
    assert_eq!(runs[0].0.as_ptr(), runs[2].0.as_ptr());
    assert_eq!(&runs[0].0[..], &runs[1].0[..]);
    let retained = runs[0].0.clone();
    let expected = runs[0].0.to_vec();
    let retained_font = runs[0].3.clone().expect("real font retained");
    drop(runs);
    drop(d);
    drop(w);
    assert_eq!(&retained[..], expected.as_slice());
    assert_eq!(retained_font.as_slice(), DEJAVU);
}
#[test]
fn equal_glyphs_with_different_shape_keys_are_never_interned() {
    let w = Fixture::new();
    let base = style("DejaVu Sans");
    let requests = [
        (
            String::from("office"),
            TextStyle { lang: Some("ru".parse().unwrap()), ..base.clone() },
        ),
        (String::from("office"), TextStyle { lang: Some("sr".parse().unwrap()), ..base }),
    ];
    let d = crate::shaper::shape_document(
        &w,
        PagedDocument::new(
            requests.iter().map(|(t, s)| page(vec![item(t, s.clone())])).collect(),
        ),
    );
    let glyphs: Vec<_> = d
        .pages
        .iter()
        .map(|p| match &p.items[0] {
            FrameItem::TextShaped { glyphs, .. } => glyphs,
            _ => panic!("real shaped run"),
        })
        .collect();
    assert_eq!(&glyphs[0][..], &glyphs[1][..]);
    assert!(!glyphs[0].is_empty());
    assert_ne!(glyphs[0].as_ptr(), glyphs[1].as_ptr());
}

#[test]
fn exact_axis_bits_feature_context_and_styles_match_fresh_in_ab_ba() {
    let w = Fixture::new();
    let base = style("Ubuntu Sans");
    let math = TextStyle { math: true, ..style("New Computer Modern Math") };
    let mut requests = vec![];
    for value in [0.0_f32, -0.0_f32, 650.0, 650.00006] {
        requests.push((
            "office AV 123".to_string(),
            TextStyle {
                variations: Some(FontVariations(vec![(*b"wght", value)])),
                ..base.clone()
            },
        ));
    }
    for s in [
        TextStyle { bold: true, ..base.clone() },
        TextStyle { italic: true, ..base.clone() },
        TextStyle { weight: Some(700), ..base.clone() },
        TextStyle { math_text_item: true, ..math.clone() },
        TextStyle { math_size: MathSize::Script, ..math.clone() },
        TextStyle { math_size: MathSize::ScriptScript, ..math },
    ] {
        requests.push(("𝑓 123 office".into(), s));
    }
    let expected: Vec<_> = requests
        .iter()
        .map(|r| snapshots(&w, std::slice::from_ref(r)).remove(0))
        .collect();
    for reverse in [false, true] {
        let mut ids: Vec<_> = (0..requests.len()).collect();
        if reverse {
            ids.reverse();
        }
        let ids: Vec<_> = ids.into_iter().cycle().take(requests.len() * 3).collect();
        let batch: Vec<_> = ids.iter().map(|&i| requests[i].clone()).collect();
        for (actual, i) in snapshots(&w, &batch).into_iter().zip(ids) {
            assert_eq!(actual, expected[i], "exact key context {i}");
        }
    }
}
