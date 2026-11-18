//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper/cache.md
//! @prompt-hash 9d1f1ddd
//! @layer L3
//! @updated 2026-11-07

use super::font_selection::FontCandidate;
use std::collections::HashMap;
use std::sync::Arc;
use typst_core::contracts::world::World;
use typst_core::entities::layout_types::SharedGlyphs;
use typst_core::entities::world_types::Font;
use typst_core::entities::{font_book::FontVariant, font_list::FontList};

/// Cache de resultados de shaping por documento.
///
/// A chave inclui tudo o que afecta o output de `rustybuzz::shape` para um
/// sub-run: texto, identificador da face, direção, variações de eixo e
/// tracking e idioma explícito opcional. O valor guarda os glifos shaped e a largura total em unidades
/// de fonte.
pub(super) struct ShapeCache {
    pub(super) map: HashMap<ShapeKey, CachedRun>,
}

/// Owned identity of one shaped sub-run, with exact coordinate bits.
#[derive(Debug, PartialEq, Eq, Hash)]
pub(super) struct ShapeKey {
    text: String,
    slot_idx: usize,
    rtl: bool,
    axis_vars: Vec<(u32, u32)>,
    tracking_fu: i32,
    ssty_level: Option<u8>,
    language: Option<String>,
    native_sups: Option<bool>,
}

impl ShapeKey {
    pub(super) fn set_native_sups(&mut self, native_sups: bool) {
        self.native_sups = Some(native_sups);
    }
}

#[derive(Clone)]
pub(super) struct CachedRun {
    pub(super) glyphs: SharedGlyphs,
    pub(super) width: i32,
    pub(super) font: Font,
}

impl ShapeCache {
    pub(super) fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub(super) fn key(
        text: &str,
        slot_idx: usize,
        rtl: bool,
        axis_vars: &[rustybuzz::Variation],
        tracking_fu: i32,
        ssty_level: Option<u8>,
        language: Option<&str>,
    ) -> ShapeKey {
        ShapeKey {
            text: text.to_owned(),
            slot_idx,
            rtl,
            axis_vars: axis_vars
                .iter()
                .map(|v| (u32::from_be_bytes(v.tag.to_bytes()), v.value.to_bits()))
                .collect(),
            tracking_fu,
            ssty_level,
            language: language.map(str::to_owned),
            native_sups: None,
        }
    }
}

/// Last complete ordered primary chain, scoped with the parsed faces.
pub(super) struct PrimaryResolution {
    pub(super) font_list: FontList,
    pub(super) variant: FontVariant,
    pub(super) math: bool,
    pub(super) candidates: Vec<FontCandidate>,
}

/// First-readable query only; not eligible as a complete primary chain.
pub(super) struct FirstResolution {
    pub(super) font_list: FontList,
    pub(super) variant: FontVariant,
    pub(super) candidate: FontCandidate,
}

pub(super) type BidiParagraphs = Arc<Vec<Vec<super::bidi::BidiRun>>>;

/// Positive proof identity; family selection and authored cmap checks precede lookup.
#[derive(PartialEq, Eq, Hash)]
pub(super) struct PrimaryProofKey {
    owner: (u8, usize),
    slot: usize,
    text: String,
    axes: Vec<(u32, u32)>,
    direction: Option<u8>,
    language: Option<String>,
}
impl PrimaryProofKey {
    pub(super) fn new(
        text: &str,
        slot: usize,
        font: &Font,
        axes: &[rustybuzz::Variation],
        direction: Option<typst_core::entities::dir::Dir>,
        language: Option<&str>,
    ) -> Self {
        use typst_core::entities::dir::Dir;
        Self {
            owner: match font {
                Font::Vec(owner) => (0, Arc::as_ptr(owner) as usize),
                Font::Mmap(owner) => (1, Arc::as_ptr(owner) as usize),
            },
            slot,
            text: text.to_owned(),
            axes: axes
                .iter()
                .map(|axis| {
                    (u32::from_be_bytes(axis.tag.to_bytes()), axis.value.to_bits())
                })
                .collect(),
            direction: direction.map(|direction| match direction {
                Dir::LTR => 1,
                Dir::RTL => 2,
                Dir::TTB => 3,
                Dir::BTT => 4,
            }),
            language: language.map(str::to_owned),
        }
    }
}

/// Cache de faces `ttf-parser` por `slot_idx` dentro de um documento.
///
/// Evita re-parsear a mesma fonte milhares de vezes durante a fase de
/// `split_run_by_font` (P672). O `Face` empresta internamente dos bytes da
/// `Font` owned; a struct vive dentro de `Arc` para que o slice `'static`
/// seja válido enquanto a face existir.
pub(crate) struct FaceCache {
    pub(super) products: Option<super::complete_session::ShapeSession>,
    pub(super) map: HashMap<usize, Option<Arc<CachedFace>>>,
    pub(super) primary_resolution: Option<PrimaryResolution>,
    pub(super) first_resolution: Option<FirstResolution>,
    pub(super) primary_proofs: HashMap<PrimaryProofKey, (Font, BidiParagraphs)>,
    raw: RawShapeStore,
}

impl FaceCache {
    pub(crate) fn attach_products(
        &mut self,
        products: &super::complete_session::ShapeSession,
    ) {
        self.products = Some(products.clone());
    }
    pub(crate) fn product_session(
        &self,
    ) -> Option<super::complete_session::ShapeSession> {
        self.products.clone()
    }
    pub(crate) fn new() -> Self {
        Self::with_raw_store(Arc::new(std::sync::Mutex::new(RawShapeCache::default())))
    }

    pub(crate) fn raw_store(&self) -> RawShapeStore {
        Arc::clone(&self.raw)
    }

    pub(super) fn with_raw_store(raw: RawShapeStore) -> Self {
        Self {
            products: None,
            map: HashMap::new(),
            primary_resolution: None,
            first_resolution: None,
            primary_proofs: HashMap::new(),
            raw,
        }
    }

    pub(crate) fn get(
        &mut self,
        world: &dyn World,
        slot_idx: usize,
    ) -> Option<&CachedFace> {
        self.map
            .entry(slot_idx)
            .or_insert_with(|| world.font(slot_idx).and_then(CachedFace::new))
            .as_deref()
    }
}

pub(crate) struct CachedFace {
    #[allow(dead_code)]
    data: Font,
    face: ttf_parser::Face<'static>,
    raw_engine: Option<Box<rustybuzz::Face<'static>>>,
}

impl CachedFace {
    pub(crate) fn new(data: Font) -> Option<Arc<Self>> {
        let slice: &'static [u8] = unsafe {
            std::slice::from_raw_parts(data.as_slice().as_ptr(), data.as_slice().len())
        };
        let face = ttf_parser::Face::parse(slice, 0).ok()?;
        Some(Arc::new(Self { data, face, raw_engine: None }))
    }

    // The existing owned-byte enclosure keeps both parsed views alive. The
    // configured engine is immutable and cannot outlive this CachedFace.
    fn for_raw(data: Font, axes: &[rustybuzz::Variation]) -> Option<Arc<Self>> {
        let mut cached = Self::new(data)?;
        let inner = Arc::get_mut(&mut cached)?;
        let mut engine = rustybuzz::Face::from_face(inner.face.clone());
        engine.set_variations(axes);
        inner.raw_engine = Some(Box::new(engine));
        Some(cached)
    }

    fn raw_engine(&self) -> Option<&rustybuzz::Face<'_>> {
        self.raw_engine.as_deref()
    }

    pub(crate) fn face(&self) -> &ttf_parser::Face<'_> {
        &self.face
    }
}

#[cfg(test)]
mod axis_key_regressions {
    use super::ShapeCache;
    use crate::{fonts::FontSlot, shaper::shape_document, world::SystemWorld};
    use typst_core::{
        contracts::world::World,
        entities::{
            content::Content,
            font_list::FontList,
            font_variations::FontVariations,
            layout_types::{
                FrameItem, Page, PagedDocument, Point, Pt, ShapedGlyph, TextStyle,
            },
        },
    };

    #[test]
    fn axis_key_distinguishes_adjacent_coordinates_and_reuses_identical_inputs() {
        for (a, b) in [(1120998658, 1120998659), (1122114658, 1122114659)] {
            let axis = |bits| {
                [rustybuzz::Variation {
                    tag: rustybuzz::ttf_parser::Tag::from_bytes(b"wght"),
                    value: f32::from_bits(bits),
                }]
            };
            let key_a = ShapeCache::key("A", 0, false, &axis(a), 0, None, None);
            let key_b = ShapeCache::key("A", 0, false, &axis(b), 0, None, None);
            assert_ne!(key_a, key_b);
            assert_eq!(key_a, ShapeCache::key("A", 0, false, &axis(a), 0, None, None));
        }
    }

    #[test]
    fn language_key_distinguishes_explicit_codes_and_absence() {
        let key = |language| ShapeCache::key("б", 0, false, &[], 0, None, language);
        assert_ne!(key(Some("ru")), key(Some("sr")));
        assert_ne!(key(None), key(Some("en")));
        assert_ne!(key(None), key(Some("sr")));
        assert_eq!(key(Some("sr")), key(Some("sr")));
    }

    #[allow(deprecated)]
    fn shaped(
        world: &dyn World,
        text: &str,
        family: &str,
        coordinates: &[f32],
    ) -> Vec<Vec<ShapedGlyph>> {
        let items = coordinates
            .iter()
            .map(|&value| FrameItem::Text {
                pos: Point { x: Pt(0.0), y: Pt(0.0) },
                text: text.into(),
                style: TextStyle {
                    size: Pt(12.0),
                    font: Some(FontList::single(family.into())),
                    variations: Some(FontVariations(vec![(*b"wght", value)])),
                    ..Default::default()
                },
                bidi: None,
            })
            .collect();
        let doc = PagedDocument::new(vec![Page {
            width: 595.0,
            height: 842.0,
            logical_number: 1,
            numbering: None,
            supplement: Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items,
        }]);
        shape_document(world, doc).pages[0]
            .items
            .iter()
            .map(|item| match item {
                FrameItem::TextShaped { glyphs, .. } => glyphs.to_vec(),
                _ => panic!("variable-font fixture did not shape"),
            })
            .collect()
    }

    fn assert_mixed_matches_fresh(
        file: &str,
        text: &str,
        bits: (u32, u32),
        advances: (i32, i32),
    ) {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let world = SystemWorld::for_eval(&root)
            .with_fonts(vec![FontSlot::new(root.join("fixtures/fonts").join(file), 0)]);
        let family = &world.book().infos()[0].family;
        let a = f32::from_bits(bits.0);
        let b = f32::from_bits(bits.1);
        let fresh_a = shaped(&world, text, family, &[a]);
        let fresh_b = shaped(&world, text, family, &[b]);
        assert_eq!(fresh_a.len(), 1);
        assert_eq!(fresh_b.len(), 1);
        assert_eq!(fresh_a[0][0].x_advance, advances.0);
        assert_eq!(fresh_b[0][0].x_advance, advances.1);
        assert_ne!(fresh_a, fresh_b, "fixture must discriminate the two coordinates");
        assert_eq!(
            shaped(&world, text, family, &[a, b]),
            vec![fresh_a[0].clone(), fresh_b[0].clone()]
        );
        assert_eq!(
            shaped(&world, text, family, &[b, a]),
            vec![fresh_b[0].clone(), fresh_a[0].clone()]
        );
        assert_eq!(
            shaped(&world, text, family, &[a, a]),
            vec![fresh_a[0].clone(), fresh_a[0].clone()]
        );
    }

    #[test]
    fn ubuntu_axis_key_mixed_runs_match_fresh_in_both_orders() {
        assert_mixed_matches_fresh(
            "UbuntuSans-Variable.ttf",
            "A",
            (1120998658, 1120998659),
            (617, 618),
        );
    }
    #[test]
    fn cantarell_axis_key_mixed_runs_match_fresh_in_both_orders() {
        assert_mixed_matches_fresh(
            "Cantarell-VF.otf",
            "x",
            (1122114658, 1122114659),
            (446, 447),
        );
    }
}
#[cfg(test)]
mod structured_key_regressions {
    use super::ShapeCache;
    use rustybuzz::{ttf_parser::Tag, Variation};
    use std::collections::HashMap;

    #[derive(Clone, Debug)]
    struct Input {
        text: String,
        slot: usize,
        rtl: bool,
        axes: Vec<(u32, u32)>,
        tracking: i32,
        ssty: Option<u8>,
        language: Option<String>,
        sups: Option<bool>,
    }
    impl Default for Input {
        fn default() -> Self {
            Self {
                text: "б|langNone|sstySome(1) α\0🙂".into(),
                slot: 0,
                rtl: false,
                axes: vec![],
                tracking: 0,
                ssty: None,
                language: None,
                sups: None,
            }
        }
    }
    macro_rules! key {
        ($i:expr) => {{
            let i = $i;
            let axes: Vec<_> = i
                .axes
                .iter()
                .map(|&(tag, bits)| Variation {
                    tag: Tag(tag),
                    value: f32::from_bits(bits),
                })
                .collect();
            let mut k = ShapeCache::key(
                &i.text,
                i.slot,
                i.rtl,
                &axes,
                i.tracking,
                i.ssty,
                i.language.as_deref(),
            );
            if let Some(sups) = i.sups {
                k.set_native_sups(sups);
            }
            k
        }};
    }

    #[test]
    fn full_identity_and_abba_map_reuse() {
        let base = Input::default();
        let mut cases = vec![base.clone()];
        macro_rules! add {
            ($field:ident, $value:expr) => {{
                let mut x = base.clone();
                x.$field = $value;
                cases.push(x);
            }};
        }
        for text in [
            "",
            "б",
            "α",
            "a|0|false||0|sstyNone|langNone",
            "a\0b",
            "a|sups0",
            "a|sups:true",
            "a|sups:false",
            "e\u{301}",
            "é",
        ] {
            add!(text, text.into());
        }
        for slot in [1, usize::MAX] {
            add!(slot, slot);
        }
        add!(rtl, true);
        for tracking in [i32::MIN, -1, 1, i32::MAX] {
            add!(tracking, tracking);
        }
        for ssty in [Some(0), Some(1), Some(2), Some(255)] {
            add!(ssty, ssty);
        }
        for language in ["", "en", "sr", "ru", "EN", "en|sups1", "é"] {
            add!(language, Some(language.into()));
        }
        for sups in [false, true] {
            add!(sups, Some(sups));
        }
        let w = u32::from_be_bytes(*b"wght");
        let d = u32::from_be_bytes(*b"wdth");
        for bits in [
            0, 0x80000000, 1, 1120998658, 1120998659, 0x7f800000, 0xff800000, 0x7fc00001,
            0x7fc00002,
        ] {
            add!(axes, vec![(w, bits)]);
        }
        for axes in [
            vec![(w, 1), (d, 2)],
            vec![(d, 2), (w, 1)],
            vec![(w, 1), (w, 2)],
            vec![(w, 2), (w, 1)],
            vec![(w, 1), (w, 1)],
            vec![(d, 1)],
            vec![(0, 1)],
            vec![(u32::MAX, 1)],
            vec![(u32::MAX - 1, 1)],
            vec![(u32::MAX - 2, 1)],
            vec![(u32::from_be_bytes(*b"|=,\0"), 1)],
        ] {
            add!(axes, axes);
        }
        let mut map = HashMap::new();
        for (n, input) in cases.iter().enumerate() {
            assert!(
                map.insert(key!(input), n).is_none(),
                "identity collision for {input:?}"
            );
            assert_eq!(map.get(&key!(input)), Some(&n), "identical owned key must reuse");
        }
        for a in 0..cases.len() {
            for b in 0..cases.len() {
                if a == b {
                    continue;
                }
                let mut m = HashMap::new();
                for index in [a, b, b, a, a, b] {
                    let k = key!(&cases[index]);
                    match m.insert(k, index) {
                        None => {}
                        Some(prior) => assert_eq!(prior, index, "ABBA contamination"),
                    }
                }
                assert_eq!(m.len(), 2);
            }
        }
    }

    #[test]
    fn native_sups_setter_replaces_effective_value() {
        {
            let input = Input::default();
            let mut key = key!(&input);
            key.set_native_sups(false);
            key.set_native_sups(true);
            let mut yes = input.clone();
            yes.sups = Some(true);
            assert_eq!(key, key!(&yes));
            key.set_native_sups(false);
            let mut no = input;
            no.sups = Some(false);
            assert_eq!(key, key!(&no));
        }
    }
}

/// Closed raw engine request: caller policies stay distinct; no opaque buffers enter.
pub(super) struct RawShapeInput<'a> {
    pub text: &'a str,
    pub slot: usize,
    pub font: &'a Font,
    pub axes: &'a [rustybuzz::Variation],
    pub rtl: bool,
    pub language: Option<&'a str>,
    pub script: Option<rustybuzz::Script>,
    pub features: &'a [rustybuzz::Feature],
}
#[derive(PartialEq, Eq, Hash)]
struct RawShapeKey {
    owner: (u8, usize),
    slot: usize,
    text: String,
    axes: Vec<(u32, u32)>,
    rtl: bool,
    language: Option<String>,
    script: Option<u32>,
    features: Vec<(u32, u32, u32, u32)>,
}
pub(crate) type RawShapeStore = Arc<std::sync::Mutex<RawShapeCache>>;
#[derive(Default)]
pub(crate) struct RawShapeCache {
    last_face: Option<LastRawFace>,
    #[cfg(test)]
    face_builds: usize,
    // The owning Font prevents address-token reuse for the whole entry lifetime.
    map: HashMap<RawShapeKey, (Font, Arc<rustybuzz::GlyphBuffer>)>,
}
// One configured engine, bounded independently of the raw-result map.
struct LastRawFace {
    owner: (u8, usize),
    axes: Vec<(u32, u32)>,
    face: Arc<CachedFace>,
}
impl RawShapeCache {
    pub(super) fn shape(
        &mut self,
        input: RawShapeInput<'_>,
    ) -> Option<Arc<rustybuzz::GlyphBuffer>> {
        let owner = match input.font {
            Font::Vec(a) => (0, Arc::as_ptr(a) as usize),
            Font::Mmap(a) => (1, Arc::as_ptr(a) as usize),
        };
        let key = RawShapeKey {
            owner,
            slot: input.slot,
            text: input.text.to_owned(),
            axes: input
                .axes
                .iter()
                .map(|a| (u32::from_be_bytes(a.tag.to_bytes()), a.value.to_bits()))
                .collect(),
            rtl: input.rtl,
            language: input.language.map(str::to_owned),
            script: input.script.map(|s| u32::from_be_bytes(s.tag().to_bytes())),
            features: input
                .features
                .iter()
                .map(|f| (u32::from_be_bytes(f.tag.to_bytes()), f.value, f.start, f.end))
                .collect(),
        };
        if let Some((_, run)) = self.map.get(&key) {
            return Some(Arc::clone(run));
        }
        let matches = self
            .last_face
            .as_ref()
            .is_some_and(|entry| entry.owner == owner && entry.axes == key.axes);
        if !matches {
            let face = CachedFace::for_raw(input.font.clone(), input.axes)?;
            #[cfg(test)]
            {
                self.face_builds += 1;
            }
            self.last_face = Some(LastRawFace { owner, axes: key.axes.clone(), face });
        }
        let face = self.last_face.as_ref()?.face.raw_engine()?;
        let mut buffer = rustybuzz::UnicodeBuffer::new();
        buffer.push_str(input.text);
        super::text_identity::configure_authored_ignorables_buffer(
            &mut buffer,
            input.text,
        );
        buffer.set_direction(if input.rtl {
            rustybuzz::Direction::RightToLeft
        } else {
            rustybuzz::Direction::LeftToRight
        });
        if let Some(language) = input.language {
            buffer.set_language(language.parse().expect("Lang is nonempty"));
        }
        if let Some(script) = input.script {
            buffer.set_script(script);
        }
        #[cfg(test)]
        super::tests::raw_reuse::record_backend();
        let run = Arc::new(rustybuzz::shape(face, input.features, buffer));
        self.map.insert(key, (input.font.clone(), Arc::clone(&run)));
        Some(run)
    }
}

#[cfg(test)]
mod configured_engine_tests {
    use super::*;
    fn font(file: &str) -> Font {
        Font::from_data(
            std::fs::read(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("fixtures/fonts")
                    .join(file),
            )
            .unwrap(),
        )
    }
    fn run(cache: &mut RawShapeCache, input: RawShapeInput<'_>) -> Option<String> {
        cache
            .shape(input)
            .map(|x| format!("{:?}{:?}", x.glyph_infos(), x.glyph_positions()))
    }
    #[test]
    fn different_texts_reuse_one_configured_engine() {
        let f = font("NotoSans-Regular.ttf");
        let mut cache = RawShapeCache::default();
        for text in ["Alpha", "Beta", "Gamma", "Alpha"] {
            let a = run(
                &mut cache,
                RawShapeInput {
                    font: &f,
                    slot: 0,
                    text,
                    axes: &[],
                    rtl: false,
                    language: None,
                    script: None,
                    features: &[],
                },
            );
            let b = run(
                &mut RawShapeCache::default(),
                RawShapeInput {
                    font: &f,
                    slot: 0,
                    text,
                    axes: &[],
                    rtl: false,
                    language: None,
                    script: None,
                    features: &[],
                },
            );
            assert!(a.is_some());
            assert_eq!(a, b);
        }
        assert_eq!(
            cache.face_builds, 1,
            "one immutable engine for one exact owner/axis request"
        );
    }
    #[test]
    fn owner_axes_and_failure_preserve_fresh_raw_glyphs() {
        let fonts = [
            font("UbuntuSans-Variable.ttf"),
            font("NimbusSans-Regular.otf"),
            font("UbuntuSans-Variable.ttf"),
        ];
        let mut cache = RawShapeCache::default();
        let w = rustybuzz::ttf_parser::Tag::from_bytes(b"wght");
        let axes: Vec<Vec<_>> = vec![
            vec![],
            vec![rustybuzz::Variation { tag: w, value: f32::from_bits(1120998658) }],
            vec![rustybuzz::Variation { tag: w, value: f32::from_bits(1120998659) }],
            vec![rustybuzz::Variation { tag: w, value: 900.0 }],
            vec![
                rustybuzz::Variation { tag: w, value: 400.0 },
                rustybuzz::Variation { tag: w, value: 900.0 },
            ],
            vec![
                rustybuzz::Variation { tag: w, value: 900.0 },
                rustybuzz::Variation { tag: w, value: 400.0 },
            ],
        ];
        let mut distinct = std::collections::HashSet::new();
        for &i in &[0, 1, 2, 0] {
            for a in axes.iter().chain(axes.iter().rev()) {
                for text in ["A", "WWW", "éffi", "б"] {
                    let got = run(
                        &mut cache,
                        RawShapeInput {
                            font: &fonts[i],
                            slot: 0,
                            text,
                            axes: a,
                            rtl: false,
                            language: Some("en"),
                            script: Some(rustybuzz::script::LATIN),
                            features: &[],
                        },
                    );
                    let fresh = run(
                        &mut RawShapeCache::default(),
                        RawShapeInput {
                            font: &fonts[i],
                            slot: 0,
                            text,
                            axes: a,
                            rtl: false,
                            language: Some("en"),
                            script: Some(rustybuzz::script::LATIN),
                            features: &[],
                        },
                    );
                    assert!(got.is_some());
                    assert_eq!(got, fresh);
                    if i == 0 && text == "A" {
                        distinct.insert(got.unwrap());
                    }
                }
            }
        }
        assert!(distinct.len() > 1, "variable font must discriminate axes");
        let invalid = Font::from_data(vec![0, 1, 2]);
        assert!(run(
            &mut cache,
            RawShapeInput {
                font: &invalid,
                slot: 0,
                text: "invalid",
                axes: &[],
                rtl: false,
                language: None,
                script: None,
                features: &[]
            }
        )
        .is_none());
        assert!(run(
            &mut cache,
            RawShapeInput {
                font: &invalid,
                slot: 0,
                text: "invalid",
                axes: &[],
                rtl: false,
                language: None,
                script: None,
                features: &[]
            }
        )
        .is_none());
        assert_eq!(
            run(
                &mut cache,
                RawShapeInput {
                    font: &fonts[0],
                    slot: 0,
                    text: "recovered",
                    axes: &[],
                    rtl: false,
                    language: None,
                    script: None,
                    features: &[]
                }
            ),
            run(
                &mut RawShapeCache::default(),
                RawShapeInput {
                    font: &fonts[0],
                    slot: 0,
                    text: "recovered",
                    axes: &[],
                    rtl: false,
                    language: None,
                    script: None,
                    features: &[]
                }
            )
        );
    }
    #[test]
    fn request_policies_do_not_mutate_retained_face() {
        let f = font("NotoSans-Regular.ttf");
        let mut cache = RawShapeCache::default();
        let feature = [rustybuzz::Feature::new(
            rustybuzz::ttf_parser::Tag::from_bytes(b"liga"),
            0,
            ..,
        )];
        for slot in [0, 7, 0] {
            for rtl in [false, true, false] {
                for lang in [None, Some("en"), Some("sr")] {
                    for script in [
                        None,
                        Some(rustybuzz::script::LATIN),
                        Some(rustybuzz::script::ARABIC),
                    ] {
                        for features in [&[][..], &feature[..]] {
                            assert_eq!(
                                run(
                                    &mut cache,
                                    RawShapeInput {
                                        font: &f,
                                        slot,
                                        text: "ffi б A\u{ad}B",
                                        axes: &[],
                                        rtl,
                                        language: lang,
                                        script,
                                        features
                                    }
                                ),
                                run(
                                    &mut RawShapeCache::default(),
                                    RawShapeInput {
                                        font: &f,
                                        slot,
                                        text: "ffi б A\u{ad}B",
                                        axes: &[],
                                        rtl,
                                        language: lang,
                                        script,
                                        features
                                    }
                                )
                            );
                        }
                    }
                }
            }
        }
        assert_eq!(cache.face_builds, 1, "buffer policy is not mutable face state");
    }

    #[test]
    fn distinct_owners_with_equal_axes_force_fresh_face_on_raw_miss() {
        let a = font("UbuntuSans-Variable.ttf");
        let b = font("NimbusSans-Regular.otf");
        let mut cache = RawShapeCache::default();
        let first = run(
            &mut cache,
            RawShapeInput {
                font: &a,
                slot: 0,
                text: "AV",
                axes: &[],
                rtl: false,
                language: None,
                script: None,
                features: &[],
            },
        )
        .unwrap();
        let second = run(
            &mut cache,
            RawShapeInput {
                font: &b,
                slot: 0,
                text: "AV",
                axes: &[],
                rtl: false,
                language: None,
                script: None,
                features: &[],
            },
        )
        .unwrap();
        let expected = run(
            &mut RawShapeCache::default(),
            RawShapeInput {
                font: &b,
                slot: 0,
                text: "AV",
                axes: &[],
                rtl: false,
                language: None,
                script: None,
                features: &[],
            },
        )
        .unwrap();
        assert_ne!(first, expected, "font pair must discriminate the face owner");
        assert_eq!(
            second, expected,
            "same slot/axes cannot authorize another owner's face"
        );
        assert_eq!(cache.face_builds, 2);
        assert_eq!(
            run(
                &mut cache,
                RawShapeInput {
                    font: &a,
                    slot: 0,
                    text: "fresh text",
                    axes: &[],
                    rtl: false,
                    language: None,
                    script: None,
                    features: &[]
                }
            ),
            run(
                &mut RawShapeCache::default(),
                RawShapeInput {
                    font: &a,
                    slot: 0,
                    text: "fresh text",
                    axes: &[],
                    rtl: false,
                    language: None,
                    script: None,
                    features: &[]
                }
            )
        );
        assert_eq!(cache.face_builds, 3, "return to A on a raw miss replaces B");
    }
}
