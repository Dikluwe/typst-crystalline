//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper/cache.md
//! @prompt-hash 41ca2644
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

/// Cache de faces `ttf-parser` por `slot_idx` dentro de um documento.
///
/// Evita re-parsear a mesma fonte milhares de vezes durante a fase de
/// `split_run_by_font` (P672). O `Face` empresta internamente dos bytes da
/// `Font` owned; a struct vive dentro de `Arc` para que o slice `'static`
/// seja válido enquanto a face existir.
pub(crate) struct FaceCache {
    pub(super) map: HashMap<usize, Option<Arc<CachedFace>>>,
    pub(super) primary_resolution: Option<PrimaryResolution>,
}

impl FaceCache {
    pub(crate) fn new() -> Self {
        Self { map: HashMap::new(), primary_resolution: None }
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
}

impl CachedFace {
    pub(crate) fn new(data: Font) -> Option<Arc<Self>> {
        let slice: &'static [u8] = unsafe {
            std::slice::from_raw_parts(data.as_slice().as_ptr(), data.as_slice().len())
        };
        let face = ttf_parser::Face::parse(slice, 0).ok()?;
        Some(Arc::new(Self { data, face }))
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
