//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/stream/text/products.md
//! @prompt-hash ccfd41ce
//! @layer L3
use super::super::FontScenario;
use std::{
    collections::{hash_map::DefaultHasher, HashMap, VecDeque},
    hash::{Hash, Hasher},
    sync::{Arc, Mutex, OnceLock},
};
use typst_core::entities::layout_types::{Color, ShapedGlyph, TextStyle};
use typst_core::entities::world_types::Font;

const MAX_ENTRIES: usize = 4096;
const MAX_BYTES: usize = 4 * 1024 * 1024;
const MAX_ENTRY: usize = 256 * 1024;
#[derive(Clone, Copy)]
pub(super) struct Input<'a, 'r> {
    pub position: (f64, f64),
    pub glyphs: &'a [ShapedGlyph],
    pub text: &'a str,
    pub style: &'a TextStyle,
    pub scenario: &'a FontScenario<'r>,
    pub units: u16,
    pub restore: bool,
    pub font: Option<&'a Font>,
    pub zero_alias: bool,
}
#[derive(PartialEq, Eq, Hash)]
struct Facts {
    position: [u64; 2],
    size: u64,
    tracking: Option<[u64; 2]>,
    rgba: [u32; 4],
    render: String,
    resource: usize,
    multifont: bool,
    units: u16,
    restore: bool,
    zero_alias: bool,
}
struct Request<'a> {
    facts: Facts,
    glyphs: &'a [ShapedGlyph],
    text: &'a str,
    mapping: &'a HashMap<u16, u16>,
    widths: &'a HashMap<u16, i32>,
}
impl<'a> Request<'a> {
    fn new(i: Input<'a, '_>) -> Option<Self> {
        if i.glyphs.is_empty()
            || i.glyphs.len() > 4096
            || i.text.len() > 16 * 1024
            || i.units == 0
            || !i.position.0.is_finite()
            || !i.position.1.is_finite()
            || !i.style.size.0.is_finite()
            || i.style.size.0 <= 0.0
            || matches!(i.style.fill, Some(Color::Spot { .. }))
        {
            return None;
        }
        let tracking = match i.style.tracking {
            Some(t) => {
                if !t.abs.to_pt().is_finite() || !t.em.is_finite() {
                    return None;
                }
                Some([t.abs.to_pt().to_bits(), t.em.to_bits()])
            }
            None => None,
        };
        let (resource, multifont, mapping, widths) = match i.scenario {
            FontScenario::Cidfont {
                glyph_mapping,
                glyph_to_nominal,
                bitmap: None,
                ..
            } => (1, false, *glyph_mapping, *glyph_to_nominal),
            FontScenario::Multifont {
                fonts,
                bindings,
                per_font_glyph_mapping,
                per_font_glyph_to_nominal,
                per_font_bitmap,
                ..
            } => {
                let index = crate::export::font_binding::lookup(
                    bindings.as_ref(),
                    fonts,
                    i.style,
                    i.font,
                )?;
                if per_font_bitmap.get(index)?.is_some() {
                    return None;
                }
                (
                    index + 1,
                    true,
                    per_font_glyph_mapping.get(index)?,
                    per_font_glyph_to_nominal.get(index)?,
                )
            }
            FontScenario::Type1 | FontScenario::Cidfont { bitmap: Some(_), .. } => {
                return None
            }
        };
        let (r, g, b, a) = i
            .style
            .fill
            .as_ref()
            .map(Color::to_rgba_f32)
            .unwrap_or((0.0, 0.0, 0.0, 1.0));
        Some(Self {
            facts: Facts {
                position: [i.position.0.to_bits(), i.position.1.to_bits()],
                size: i.style.size.0.to_bits(),
                tracking,
                rgba: [r.to_bits(), g.to_bits(), b.to_bits(), a.to_bits()],
                render: super::shaped_tr_ops(i.style, i.font),
                resource,
                multifont,
                units: i.units,
                restore: i.restore,
                zero_alias: i.zero_alias,
            },
            glyphs: i.glyphs,
            text: i.text,
            mapping,
            widths,
        })
    }
    fn projected(&self, g: &ShapedGlyph) -> (u16, i32) {
        (
            if self.mapping.is_empty() {
                g.glyph_id
            } else {
                self.mapping.get(&g.glyph_id).copied().unwrap_or(0)
            },
            self.widths.get(&g.glyph_id).copied().unwrap_or(g.x_advance),
        )
    }
    fn fingerprint(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.facts.hash(&mut h);
        self.text.hash(&mut h);
        self.glyphs.len().hash(&mut h);
        for g in self.glyphs {
            (
                g.glyph_id,
                g.x_advance,
                g.x_offset,
                g.y_offset,
                g.cluster,
                g.char_code,
                self.projected(g),
            )
                .hash(&mut h);
        }
        h.finish()
    }
    fn matches(&self, key: &Key) -> bool {
        self.facts == key.facts
            && self.text == key.text.as_ref()
            && self.glyphs == key.glyphs.as_ref()
            && key
                .projected
                .iter()
                .copied()
                .eq(self.glyphs.iter().map(|g| self.projected(g)))
    }
    fn charge(&self, output: usize) -> Option<usize> {
        std::mem::size_of::<Entry>()
            .checked_add(std::mem::size_of::<Product>())?
            .checked_add(std::mem::size_of::<u64>())?
            .checked_add(self.glyphs.len().checked_mul(
                std::mem::size_of::<ShapedGlyph>() + std::mem::size_of::<(u16, i32)>(),
            )?)?
            .checked_add(self.text.len())?
            .checked_add(self.facts.render.len())?
            .checked_add(output)
    }
    fn owned(self) -> Key {
        let projected = self.glyphs.iter().map(|g| self.projected(g)).collect();
        Key {
            facts: self.facts,
            glyphs: self.glyphs.into(),
            text: self.text.into(),
            projected,
        }
    }
}
struct Key {
    facts: Facts,
    glyphs: Box<[ShapedGlyph]>,
    text: Box<str>,
    projected: Box<[(u16, i32)]>,
}
struct Product {
    ops: Box<str>,
    deferred: bool,
}
struct Entry {
    key: Key,
    product: Arc<Product>,
    charge: usize,
}
#[derive(Default)]
struct Cache {
    entries: HashMap<u64, Vec<Entry>>,
    order: VecDeque<u64>,
    bytes: usize,
}
impl Cache {
    fn get(&self, hash: u64, request: &Request<'_>) -> Option<Arc<Product>> {
        self.entries
            .get(&hash)?
            .iter()
            .find(|entry| request.matches(&entry.key))
            .map(|entry| entry.product.clone())
    }
    fn insert(&mut self, hash: u64, request: Request<'_>, product: Arc<Product>) {
        let Some(charge) = request.charge(product.ops.len()) else { return };
        if charge > MAX_ENTRY || self.get(hash, &request).is_some() {
            return;
        }
        while self.order.len() >= MAX_ENTRIES || self.bytes > MAX_BYTES - charge {
            let Some(old) = self.order.pop_front() else { return };
            if let Some(bucket) = self.entries.get_mut(&old) {
                let entry = bucket.remove(0);
                self.bytes -= entry.charge;
                if bucket.is_empty() {
                    self.entries.remove(&old);
                }
            }
        }
        self.entries.entry(hash).or_default().push(Entry {
            key: request.owned(),
            product,
            charge,
        });
        self.order.push_back(hash);
        self.bytes += charge;
    }
}
fn cached(
    cache: &Mutex<Cache>,
    ops: &mut String,
    input: Input<'_, '_>,
    compute: impl FnOnce(&mut String) -> bool,
) -> bool {
    cached_indexed(cache, ops, input, compute, |hash| hash)
}
fn cached_indexed(
    cache: &Mutex<Cache>,
    ops: &mut String,
    input: Input<'_, '_>,
    compute: impl FnOnce(&mut String) -> bool,
    index: impl FnOnce(u64) -> u64,
) -> bool {
    let Some(request) = Request::new(input) else { return compute(ops) };
    let hash = index(request.fingerprint());
    {
        let Ok(retained) = cache.lock() else { return compute(ops) };
        if let Some(product) = retained.get(hash, &request) {
            ops.push_str(&product.ops);
            return product.deferred;
        }
    }
    let mut suffix = String::new();
    let deferred = compute(&mut suffix);
    ops.push_str(&suffix);
    if let Ok(mut retained) = cache.lock() {
        retained.insert(
            hash,
            request,
            Arc::new(Product { ops: suffix.into_boxed_str(), deferred }),
        );
    }
    deferred
}
static CACHE: OnceLock<Mutex<Cache>> = OnceLock::new();
pub(super) fn emit(ops: &mut String, input: Input<'_, '_>) -> bool {
    cached(CACHE.get_or_init(|| Mutex::new(Cache::default())), ops, input, |out| {
        super::emit_shaped_pdf_verbose_fresh(out, input)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::Cell, collections::HashMap};
    use typst_core::entities::layout_types::{Color, Length, Pt};
    fn glyph() -> ShapedGlyph {
        ShapedGlyph {
            glyph_id: 4,
            x_advance: 503,
            x_offset: 9,
            y_offset: 2,
            cluster: 0,
            char_code: 'A',
        }
    }
    fn fresh(ops: &mut String, i: Input<'_, '_>) -> bool {
        super::super::emit_shaped_pdf_verbose_fresh(ops, i)
    }

    fn observed(
        cache: &Mutex<Cache>,
        i: Input<'_, '_>,
        calls: &Cell<usize>,
    ) -> (String, bool) {
        let mut expected = "PREFIX\n".to_string();
        let expected_tail = fresh(&mut expected, i);
        let mut actual = "PREFIX\n".to_string();
        let tail = cached(cache, &mut actual, i, |out| {
            calls.set(calls.get() + 1);
            fresh(out, i)
        });
        assert_eq!((actual.as_str(), tail), (expected.as_str(), expected_tail));
        (actual, tail)
    }
    fn input<'a, 'r>(
        glyphs: &'a [ShapedGlyph],
        style: &'a TextStyle,
        scenario: &'a FontScenario<'r>,
    ) -> Input<'a, 'r> {
        Input {
            position: (10.0, 20.0),
            glyphs,
            text: "A",
            style,
            scenario,
            units: 1000,
            restore: true,
            font: None,
            zero_alias: false,
        }
    }
    #[test]
    fn text_products_reuse_actual_complete_suffix_and_deferred_tail() {
        let map = HashMap::new();
        let scenario = FontScenario::Cidfont {
            char_to_gid: &map,
            glyph_mapping: &HashMap::new(),
            glyph_to_nominal: &HashMap::new(),
            bitmap: None,
        };
        let glyphs = vec![glyph(), ShapedGlyph { cluster: 1, ..glyph() }];
        let style = TextStyle::regular(Pt(11.0));
        let cache = Mutex::new(Cache::default());
        let calls = Cell::new(0);
        for restore in [true, false, false, true] {
            let i = Input { restore, ..input(&glyphs, &style, &scenario) };
            let (mut out, tail) = observed(&cache, i, &calls);
            assert_eq!(tail, !restore);
            assert!(out.contains("TJ\nET\n"));
            out.clear();
        }
        assert_eq!(calls.get(), 2, "repeated complete suffix was reformatted");
    }
    #[test]
    fn text_products_reuse_owned_glyph_and_unicode_inputs() {
        let map = HashMap::new();
        let scenario = FontScenario::Cidfont {
            char_to_gid: &map,
            glyph_mapping: &HashMap::new(),
            glyph_to_nominal: &HashMap::new(),
            bitmap: None,
        };
        let style = TextStyle::regular(Pt(11.0));
        let cache = Mutex::new(Cache::default());
        let calls = Cell::new(0);
        for text in ["A café α", "B", "B", "A café α"] {
            let glyphs = vec![glyph()];
            observed(&cache, Input { text, ..input(&glyphs, &style, &scenario) }, &calls);
        }
        assert_eq!(calls.get(), 2, "equal owned inputs were reformatted");
    }
    #[test]
    fn text_products_current_numeric_style_and_glyph_inputs_match_fresh() {
        let map = HashMap::new();
        let scenario = FontScenario::Cidfont {
            char_to_gid: &map,
            glyph_mapping: &HashMap::new(),
            glyph_to_nominal: &HashMap::new(),
            bitmap: None,
        };
        let base = TextStyle::regular(Pt(11.0));
        let cache = Mutex::new(Cache::default());
        let calls = Cell::new(0);
        for k in 0..15 {
            let mut style = base.clone();
            let mut glyphs = vec![glyph(), ShapedGlyph { cluster: 1, ..glyph() }];
            let mut position = (10.0, 20.0);
            let mut units = 1000;
            let mut zero_alias = false;
            match k {
                1 => style.size = Pt(13.125),
                2 => style.tracking = Some(Length::pt(0.2)),
                3 => style.tracking = Some(Length::em(-0.02)),
                4 => style.weight = Some(700),
                5 => style.fill = Some(Color::Srgb { r: 0.3, g: 0.5, b: 0.7, a: 0.4 }),
                6 => position.0 = -0.0,
                7 => position.0 = 0.0,
                8 => position.1 = 21.0,
                9 => units = 2048,
                10 => zero_alias = true,
                11 => glyphs[0].x_advance += 1,
                12 => glyphs[0].glyph_id += 1,
                13 => glyphs[0].x_offset += 1,
                14 => glyphs[1].cluster = 0,
                _ => {}
            }
            observed(
                &cache,
                Input {
                    position,
                    units,
                    zero_alias,
                    ..input(&glyphs, &style, &scenario)
                },
                &calls,
            );
        }
        assert_eq!(calls.get(), 15, "changed causal facts must not reuse an old product");
    }
    #[test]
    fn text_products_current_remapping_and_nominal_answers_match_fresh() {
        let style = TextStyle::regular(Pt(11.0));
        let glyphs = vec![glyph()];
        let chars = HashMap::new();
        let cache = Mutex::new(Cache::default());
        let calls = Cell::new(0);
        for (mapped, nominal) in
            [(None, None), (Some(7), None), (Some(7), Some(600)), (Some(9), Some(600))]
        {
            let mapping = mapped.map(|v| HashMap::from([(4, v)])).unwrap_or_default();
            let widths = nominal.map(|v| HashMap::from([(4, v)])).unwrap_or_default();
            let scenario = FontScenario::Cidfont {
                char_to_gid: &chars,
                glyph_mapping: &mapping,
                glyph_to_nominal: &widths,
                bitmap: None,
            };
            observed(&cache, input(&glyphs, &style, &scenario), &calls);
        }
        assert_eq!(calls.get(), 4);
    }
    #[test]
    fn text_products_actual_font_weight_and_zero_alias_match_fresh() {
        let font = typst_assets::fonts()
            .find_map(|bytes| {
                let face = ttf_parser::Face::parse(bytes, 0).ok()?;
                (face.weight().to_number() == 700 && face.tables().fvar.is_none())
                    .then(|| Font::from_data(bytes.to_vec()))
            })
            .expect("bundled real static 700 face");
        let mut style = TextStyle::regular(Pt(11.0));
        style.weight = Some(700);
        let glyphs = vec![glyph()];
        let map = HashMap::new();
        let scenario = FontScenario::Cidfont {
            char_to_gid: &map,
            glyph_mapping: &HashMap::new(),
            glyph_to_nominal: &HashMap::new(),
            bitmap: None,
        };
        let cache = Mutex::new(Cache::default());
        let calls = Cell::new(0);
        let Font::Vec(owner) = &font else { panic!("test font must own bytes") };
        let owners = Arc::strong_count(owner);
        for font in [None, Some(&font)] {
            for zero_alias in [false, true] {
                let (out, _) = observed(
                    &cache,
                    Input {
                        font,
                        zero_alias,
                        ..input(&glyphs, &style, &scenario)
                    },
                    &calls,
                );
                assert_eq!(out.contains("2 Tr"), font.is_none());
            }
        }
        assert_eq!(calls.get(), 4);
        assert_eq!(Arc::strong_count(owner), owners, "cache must not retain the face");
    }
    #[test]
    fn text_products_unsupported_and_large_inputs_stay_fresh() {
        let style = TextStyle::regular(Pt(11.0));
        let glyphs = vec![glyph()];
        let scenario = FontScenario::Type1;
        let cache = Mutex::new(Cache::default());
        let calls = Cell::new(0);
        for _ in 0..2 {
            observed(&cache, input(&glyphs, &style, &scenario), &calls);
        }
        assert_eq!(calls.get(), 2, "Type1 is outside this product boundary");
        let map = HashMap::new();
        let scenario = FontScenario::Cidfont {
            char_to_gid: &map,
            glyph_mapping: &HashMap::new(),
            glyph_to_nominal: &HashMap::new(),
            bitmap: None,
        };
        let text = "x".repeat(16 * 1024 + 1);
        for _ in 0..2 {
            observed(
                &cache,
                Input { text: &text, ..input(&glyphs, &style, &scenario) },
                &calls,
            );
        }
        assert_eq!(calls.get(), 4, "input bound must preserve ordinary emission");
    }

    #[test]
    fn text_products_current_multifont_resource_index_is_not_frozen() {
        use typst_core::entities::{
            font_book::FontVariant, font_list::FontList, font_variations::FontVariations,
        };
        let make = |name: &str| {
            (
                (
                    FontList::single(name.into()),
                    FontVariant::default(),
                    FontVariations::default(),
                ),
                vec![],
            )
        };
        let first = vec![make("alpha"), make("beta")];
        let second = vec![make("beta"), make("alpha")];
        let mut style = TextStyle::regular(Pt(11.0));
        style.font = Some(FontList::single("alpha".into()));
        let glyphs = vec![glyph()];
        let cache = Mutex::new(Cache::default());
        let calls = Cell::new(0);
        let chars = vec![HashMap::new(), HashMap::new()];
        let maps = vec![HashMap::new(), HashMap::new()];
        let widths = vec![HashMap::new(), HashMap::new()];
        let reverses = vec![HashMap::new(), HashMap::new()];
        let bitmap = vec![None, None];
        for (fonts, expected) in [(&first, "/F1 "), (&second, "/F2 "), (&first, "/F1 ")] {
            let scenario = FontScenario::Multifont {
                fonts,
                bindings: None,
                per_font_char_to_gid: &chars,
                per_font_glyph_mapping: &maps,
                per_font_glyph_to_nominal: &widths,
                per_font_glyph_reverse: &reverses,
                per_font_bitmap: &bitmap,
            };
            let (out, _) = observed(&cache, input(&glyphs, &style, &scenario), &calls);
            assert!(out.contains(expected), "{out}");
        }
        assert_eq!(calls.get(), 2, "equal resolved font binding should reuse");
    }
    #[test]
    fn text_products_bitmap_and_nonfinite_inputs_use_original_path() {
        let style = TextStyle::regular(Pt(11.0));
        let glyphs = vec![glyph()];
        let map = HashMap::new();
        let empty_bitmap = HashMap::new();
        let scenario = FontScenario::Cidfont {
            char_to_gid: &map,
            glyph_mapping: &HashMap::new(),
            glyph_to_nominal: &HashMap::new(),
            bitmap: Some(&empty_bitmap),
        };
        let cache = Mutex::new(Cache::default());
        let calls = Cell::new(0);
        for _ in 0..2 {
            observed(&cache, input(&glyphs, &style, &scenario), &calls);
        }
        let scenario = FontScenario::Cidfont {
            char_to_gid: &map,
            glyph_mapping: &HashMap::new(),
            glyph_to_nominal: &HashMap::new(),
            bitmap: None,
        };
        for _ in 0..2 {
            observed(
                &cache,
                Input {
                    position: (f64::INFINITY, 20.0),
                    ..input(&glyphs, &style, &scenario)
                },
                &calls,
            );
        }
        assert_eq!(calls.get(), 4);
    }
    #[test]
    fn text_products_poisoned_storage_preserves_current_emission() {
        let cache = Mutex::new(Cache::default());
        let _ = std::panic::catch_unwind(|| {
            let _guard = cache.lock().unwrap();
            panic!("poison local test cache")
        });
        let style = TextStyle::regular(Pt(11.0));
        let glyphs = vec![glyph()];
        let map = HashMap::new();
        let scenario = FontScenario::Cidfont {
            char_to_gid: &map,
            glyph_mapping: &HashMap::new(),
            glyph_to_nominal: &HashMap::new(),
            bitmap: None,
        };
        let calls = Cell::new(0);
        for _ in 0..2 {
            observed(&cache, input(&glyphs, &style, &scenario), &calls);
        }
        assert_eq!(calls.get(), 2);
    }

    #[test]
    fn text_products_collisions_require_full_current_facts() {
        let style = TextStyle::regular(Pt(11.0));
        let glyphs = vec![glyph()];
        let chars = HashMap::new();
        let cache = Mutex::new(Cache::default());
        let calls = Cell::new(0);
        for (x, gid) in [(10.0, 7), (20.0, 7), (10.0, 8), (20.0, 7), (10.0, 7)] {
            let mapping = HashMap::from([(4, gid)]);
            let scenario = FontScenario::Cidfont {
                char_to_gid: &chars,
                glyph_mapping: &mapping,
                glyph_to_nominal: &HashMap::new(),
                bitmap: None,
            };
            let i = Input {
                position: (x, 20.0),
                ..input(&glyphs, &style, &scenario)
            };
            let mut expected = String::new();
            let expected_tail = fresh(&mut expected, i);
            let mut actual = String::new();
            let tail = cached_indexed(
                &cache,
                &mut actual,
                i,
                |out| {
                    calls.set(calls.get() + 1);
                    fresh(out, i)
                },
                |_| 7,
            );
            assert_eq!((actual, tail), (expected, expected_tail));
        }
        assert_eq!(calls.get(), 3);
    }
    #[test]
    fn text_products_unused_mapping_changes_preserve_complete_product() {
        let style = TextStyle::regular(Pt(11.0));
        let glyphs = vec![glyph()];
        let chars = HashMap::new();
        let cache = Mutex::new(Cache::default());
        let calls = Cell::new(0);
        for n in 0..3 {
            let mapping = HashMap::from([(4, 7), (19, n)]);
            let widths = HashMap::from([(4, 600), (19, 100 + n as i32)]);
            let scenario = FontScenario::Cidfont {
                char_to_gid: &chars,
                glyph_mapping: &mapping,
                glyph_to_nominal: &widths,
                bitmap: None,
            };
            observed(&cache, input(&glyphs, &style, &scenario), &calls);
        }
        assert_eq!(calls.get(), 1);
    }
    #[test]
    fn text_products_entry_bound_evicts_oldest_without_truncation() {
        let style = TextStyle::regular(Pt(11.0));
        let glyphs = vec![glyph()];
        let chars = HashMap::new();
        let scenario = FontScenario::Cidfont {
            char_to_gid: &chars,
            glyph_mapping: &HashMap::new(),
            glyph_to_nominal: &HashMap::new(),
            bitmap: None,
        };
        let cache = Mutex::new(Cache::default());
        let calls = Cell::new(0);
        for x in 0..4097 {
            observed(
                &cache,
                Input {
                    position: (x as f64, 20.0),
                    ..input(&glyphs, &style, &scenario)
                },
                &calls,
            );
        }
        observed(
            &cache,
            Input {
                position: (4096.0, 20.0),
                ..input(&glyphs, &style, &scenario)
            },
            &calls,
        );
        assert_eq!(calls.get(), 4097);
        observed(
            &cache,
            Input {
                position: (0.0, 20.0),
                ..input(&glyphs, &style, &scenario)
            },
            &calls,
        );
        assert_eq!(calls.get(), 4098);
    }
    #[test]
    fn text_products_output_bound_preserves_full_result_and_recomputes() {
        let style = TextStyle::regular(Pt(11.0));
        let glyphs = vec![glyph()];
        let chars = HashMap::new();
        let scenario = FontScenario::Cidfont {
            char_to_gid: &chars,
            glyph_mapping: &HashMap::new(),
            glyph_to_nominal: &HashMap::new(),
            bitmap: None,
        };
        let cache = Mutex::new(Cache::default());
        let calls = Cell::new(0);
        let suffix = "x".repeat(256 * 1024 + 1);
        for _ in 0..2 {
            let mut out = "PREFIX".to_string();
            let tail =
                cached(&cache, &mut out, input(&glyphs, &style, &scenario), |out| {
                    calls.set(calls.get() + 1);
                    out.push_str(&suffix);
                    true
                });
            assert!(tail);
            assert_eq!(out.len(), 6 + suffix.len());
            assert_eq!(&out[6..], suffix);
        }
        assert_eq!(calls.get(), 2);
    }
    #[test]
    fn text_products_total_payload_bound_and_glyph_bound_fall_back() {
        let style = TextStyle::regular(Pt(11.0));
        let glyphs = vec![glyph(); 1000];
        let chars = HashMap::new();
        let scenario = FontScenario::Cidfont {
            char_to_gid: &chars,
            glyph_mapping: &HashMap::new(),
            glyph_to_nominal: &HashMap::new(),
            bitmap: None,
        };
        let cache = Mutex::new(Cache::default());
        let calls = Cell::new(0);
        let text = "a".repeat(16 * 1024);
        for x in 0..120 {
            observed(
                &cache,
                Input {
                    position: (x as f64, 20.0),
                    text: &text,
                    ..input(&glyphs, &style, &scenario)
                },
                &calls,
            );
        }
        observed(
            &cache,
            Input {
                position: (119.0, 20.0),
                text: &text,
                ..input(&glyphs, &style, &scenario)
            },
            &calls,
        );
        assert_eq!(calls.get(), 120);
        observed(
            &cache,
            Input {
                position: (0.0, 20.0),
                text: &text,
                ..input(&glyphs, &style, &scenario)
            },
            &calls,
        );
        assert_eq!(calls.get(), 121);
        let glyphs = vec![glyph(); 4097];
        for _ in 0..2 {
            observed(&cache, input(&glyphs, &style, &scenario), &calls);
        }
        assert_eq!(calls.get(), 123);
    }
}
