//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/font_metrics/caches.md
//! @prompt-hash 1c6869df
//! @layer L3
//! @updated 2026-09-27

use super::FallbackFontMetrics;
use crate::font_variant::axis_variations_for_text_style;
use std::sync::Arc;
use ttf_parser::Face;
use typst_core::entities::layout_types::{Pt, TextStyle};
use typst_core::entities::world_types::Font;

/// Face parseada e bytes correspondentes, alocada em `Arc` para garantir
/// que os bytes não se movem depois de o `Face` ser criado.
///
/// Segue o mesmo padrão do Typst vanilla: o `Face` empresta internamente
/// dos bytes via um slice `'static` obtido com `from_raw_parts`. O campo
/// `data` nunca é movido porque a struct vive dentro de `Arc`.
pub(super) struct CachedFace {
    // `data` é lido implicitamente pelo `Face`; mantém os bytes vivos.
    #[allow(dead_code)]
    data: Font,
    face: Face<'static>,
}

impl CachedFace {
    /// Parseia uma fonte e devolve a face cacheada.
    pub(super) fn new(data: Font) -> Option<Arc<Self>> {
        // Safety: `data` é owned e não será movido (a struct fica em Arc no
        // heap). O slice `'static` é apenas um artefacto para satisfazer o
        // lifetime do `Face`; nunca escapa como `'static` para fora deste
        // módulo.
        let slice: &'static [u8] = unsafe {
            std::slice::from_raw_parts(data.as_slice().as_ptr(), data.as_slice().len())
        };
        let face = Face::parse(slice, 0).ok()?;
        Some(Arc::new(Self { data, face }))
    }

    pub(super) fn face(&self) -> &Face<'_> {
        &self.face
    }
}

/// **P544** — Métricas de fonte com fallback multi-script.
///
/// Dado um `World`, resolve a fonte real (primárias do `TextStyle` + fallback
/// global do `FontBook`) para medir cada caractere com a face correcta,
/// em vez de usar uma largura fixa monoespaçada.
/// **P591** — chave para cache de `advance_shaped`. Inclui os campos do
/// `TextStyle` que afectam a largura shaped; campos puramente visuais
/// (fill, highlight, etc.) são omitidos porque não alteram métricas.
/// **P659** — adicionado `axis_hash` para incluir variações de eixo OpenType
/// (weight, width, italic slant, etc.) na chave, evitando colisões entre
/// estilos com o mesmo texto/tamanho mas eixos diferentes.
#[derive(Debug, Hash, Eq, PartialEq)]
pub(super) struct ShapedWidthKey {
    text: String,
    size_bits: u64,
    font_hash: u64,
    bold: bool,
    italic: bool,
    weight: Option<u16>,
    dir: u8,
    lang: Option<typst_core::entities::lang::Lang>,
    axis_hash: u64,
    math: bool,
}

/// **P677** — chave para cache de `advance` (caminho rápido não-shaped).
/// Inclui os mesmos campos de estilo que `ShapedWidthKey`; `tracking` é
/// aplicado fora do cache em `text_width`, pelo que não entra na chave.
#[derive(Debug, Hash, Eq, PartialEq)]
pub(super) struct AdvanceWidthKey {
    text: String,
    size_bits: u64,
    font_hash: u64,
    bold: bool,
    italic: bool,
    weight: Option<u16>,
    dir: u8,
    lang: Option<typst_core::entities::lang::Lang>,
    axis_hash: u64,
    /// **P975** — o termo de italics correction depende de `style.math`;
    /// sem este campo, uma medição em prosa e outra em math com o mesmo
    /// texto/estilo colidiam na cache.
    math: bool,
    /// **P1293** — em math, `TextItem` e `GlyphFragment` partilham fonte e
    /// tamanho, mas somente o segundo recebe italics correction.
    math_text_item: bool,
    /// **P977** — o advance muda por nível MathSize (variante ssty).
    math_size: u8,
}

impl FallbackFontMetrics<'_> {
    /// **P591** — constrói uma chave de cache para `advance_shaped`.
    pub(super) fn shaped_width_key(
        text: &str,
        style: &TextStyle,
    ) -> Option<ShapedWidthKey> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use typst_core::entities::dir::Dir;

        let dir = style
            .dir
            .map(|d| match d {
                Dir::LTR => 1u8,
                Dir::RTL => 2,
                Dir::TTB => 3,
                Dir::BTT => 4,
            })
            .unwrap_or(0);

        let font_hash = style
            .font
            .as_ref()
            .map(|fl| {
                let mut h = DefaultHasher::new();
                fl.hash(&mut h);
                h.finish()
            })
            .unwrap_or(0);

        // P659 — incluir variações de eixo OpenType na chave. O mesmo texto,
        // tamanho e peso nominal pode ter larguras diferentes se os eixos
        // (wdth, wght real, etc.) divergirem.
        let axis_vars = axis_variations_for_text_style(style);
        let axis_hash = {
            let mut h = DefaultHasher::new();
            for v in &axis_vars {
                v.tag.hash(&mut h);
                v.value.to_bits().hash(&mut h);
            }
            h.finish()
        };

        Some(ShapedWidthKey {
            text: text.to_string(),
            size_bits: style.size.0.to_bits(),
            font_hash,
            bold: style.bold,
            italic: style.italic,
            weight: style.weight,
            dir,
            lang: style.lang,
            axis_hash,
            math: style.math,
        })
    }

    /// **P591** — cache lookup/inserção para `advance_shaped`.
    pub(super) fn cached_shaped_width(
        &self,
        text: &str,
        style: &TextStyle,
        compute: impl FnOnce() -> Option<Pt>,
    ) -> Option<Pt> {
        // Não cachear quando tracking ou outros ajustes de largura estão
        // activos, para evitar valores incorrectos.
        if style.tracking.is_some() {
            return compute();
        }

        let key = Self::shaped_width_key(text, style)?;
        {
            let cache = self.shaped_width_cache.lock().unwrap();
            if let Some(&value) = cache.get(&key) {
                return Some(value);
            }
        }

        let value = compute()?;
        self.shaped_width_cache.lock().unwrap().insert(key, value);
        Some(value)
    }

    /// **P677** — constrói uma chave de cache para `advance`.
    pub(super) fn advance_width_key(
        text: &str,
        size: Pt,
        style: &TextStyle,
    ) -> Option<AdvanceWidthKey> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use typst_core::entities::dir::Dir;

        let dir = style
            .dir
            .map(|d| match d {
                Dir::LTR => 1u8,
                Dir::RTL => 2,
                Dir::TTB => 3,
                Dir::BTT => 4,
            })
            .unwrap_or(0);

        let font_hash = style
            .font
            .as_ref()
            .map(|fl| {
                let mut h = DefaultHasher::new();
                fl.hash(&mut h);
                h.finish()
            })
            .unwrap_or(0);

        let axis_vars = axis_variations_for_text_style(style);
        let axis_hash = {
            let mut h = DefaultHasher::new();
            for v in &axis_vars {
                v.tag.hash(&mut h);
                v.value.to_bits().hash(&mut h);
            }
            h.finish()
        };

        Some(AdvanceWidthKey {
            text: text.to_string(),
            size_bits: size.0.to_bits(),
            font_hash,
            bold: style.bold,
            italic: style.italic,
            weight: style.weight,
            dir,
            lang: style.lang,
            axis_hash,
            math: style.math,
            math_text_item: style.math_text_item,
            math_size: style.math_size as u8,
        })
    }

    /// **P677** — cache lookup/inserção para `advance`.
    pub(super) fn cached_advance_width(
        &self,
        text: &str,
        size: Pt,
        style: &TextStyle,
        compute: impl FnOnce() -> Pt,
    ) -> Pt {
        let key = match Self::advance_width_key(text, size, style) {
            Some(k) => k,
            None => return compute(),
        };
        {
            let cache = self.advance_width_cache.lock().unwrap();
            if let Some(&value) = cache.get(&key) {
                return value;
            }
        }

        let value = compute();
        self.advance_width_cache.lock().unwrap().insert(key, value);
        value
    }

    /// Devolve a face cacheada para `slot_idx`, criando-a se necessário.
    pub(super) fn cached_face(&self, slot_idx: usize) -> Option<Arc<CachedFace>> {
        if let Some(cached) = self.cache.lock().unwrap().get(&slot_idx).cloned() {
            return Some(cached);
        }
        let font = self.world.font(slot_idx)?;
        let cached = CachedFace::new(font)?;
        self.cache.lock().unwrap().insert(slot_idx, cached.clone());
        Some(cached)
    }
}

#[cfg(test)]
mod tests {
    use super::FallbackFontMetrics;
    use typst_core::entities::layout_types::{Pt, TextStyle};

    // P659 — a chave de cache de shaped_width deve distinguir variações de eixo
    // OpenType. O mesmo texto, tamanho e peso nominal pode ter larguras shaped
    // diferentes se o eixo efectivo (ex.: wght) divergir.
    #[test]
    fn p659_shaped_width_key_distingue_variacoes_de_eixo() {
        let style_a = TextStyle { weight: Some(400), ..TextStyle::default() };

        let style_b = TextStyle { weight: Some(700), ..TextStyle::default() };

        let key_a = FallbackFontMetrics::shaped_width_key("abc", &style_a).unwrap();
        let key_b = FallbackFontMetrics::shaped_width_key("abc", &style_b).unwrap();

        assert_ne!(
            key_a.axis_hash, key_b.axis_hash,
            "pesos 400 e 700 devem produzir axis_hash diferentes"
        );
        assert_ne!(key_a, key_b, "ShapedWidthKey de 400 e 700 deve ser distinta");

        // Dois estilos com o mesmo peso devem colidir em axis_hash.
        let key_a2 = FallbackFontMetrics::shaped_width_key("abc", &style_a).unwrap();
        assert_eq!(key_a.axis_hash, key_a2.axis_hash);
    }

    #[test]
    fn p1293_advance_width_key_distingue_proveniencia_textitem() {
        let glyph_style = TextStyle {
            math: true,
            math_text_item: false,
            ..TextStyle::regular(Pt(11.0))
        };
        let textitem_style = TextStyle { math_text_item: true, ..glyph_style.clone() };

        let glyph =
            FallbackFontMetrics::advance_width_key("x", glyph_style.size, &glyph_style)
                .unwrap();
        let textitem = FallbackFontMetrics::advance_width_key(
            "x",
            textitem_style.size,
            &textitem_style,
        )
        .unwrap();
        assert_ne!(glyph, textitem, "a cache não pode colidir entre glifo e TextItem");
    }
}

#[cfg(test)]
mod infra_review_regressions {
    use super::*;
    use typst_core::{
        compiler::layout::FontMetrics,
        contracts::world::World,
        entities::{
            duration::Duration,
            file_id::FileId,
            font_book::{FontBook, FontVariant},
            font_list::FontList,
            source::Source,
            world_types::{Bytes, Datetime, FileResult, Library},
        },
    };
    struct TestWorld {
        book: FontBook,
        fonts: Vec<Font>,
        library: Library,
    }
    impl World for TestWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            panic!("not exercised")
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            panic!("not exercised")
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            panic!("not exercised")
        }
        fn font(&self, i: usize) -> Option<Font> {
            self.fonts.get(i).cloned()
        }
        fn today(&self, _: Option<Duration>) -> Option<Datetime> {
            None
        }
    }
    fn bytes(family: &str) -> Vec<u8> {
        typst_assets::fonts()
            .find(|data| {
                crate::fonts::font_info_from_bytes(data, 0).is_some_and(|info| {
                    info.family == family && info.variant == FontVariant::default()
                })
            })
            .expect("mandatory embedded font")
            .to_vec()
    }
    #[test]
    fn infra_review_math_shaped_width_matches_fresh() {
        let data = [
            bytes("Libertinus Serif"),
            bytes("DejaVu Sans Mono"),
            bytes("New Computer Modern Math"),
        ];
        let faces: Vec<_> =
            data.iter().map(|d| ttf_parser::Face::parse(d, 0).unwrap()).collect();
        let c = (0x2200..=0x2bff)
            .filter_map(char::from_u32)
            .find(|&c| {
                if faces[0].glyph_index(c).is_some() {
                    return false;
                }
                let (Some(a), Some(b)) =
                    (faces[1].glyph_index(c), faces[2].glyph_index(c))
                else {
                    return false;
                };
                faces[1].glyph_hor_advance(a).unwrap() as f64
                    / faces[1].units_per_em() as f64
                    != faces[2].glyph_hor_advance(b).unwrap() as f64
                        / faces[2].units_per_em() as f64
            })
            .expect("distinct math/nonmath fallback advances");
        let mut book = FontBook::new();
        let flags = crate::fonts::font_info_from_bytes(&data[0], 0).unwrap().flags;
        for (i, d) in data.iter().enumerate() {
            let mut info = crate::fonts::font_info_from_bytes(d, 0).unwrap();
            info.family =
                ["Primary", "Primary Backup", "New Computer Modern Math"][i].into();
            info.flags = flags;
            info.coverage = crate::fonts::extract_coverage(&faces[i]);
            book.push(info);
        }
        assert_eq!(
            book.select_fallback(Some(&book.infos()[0]), &FontVariant::default(), [1, 2]),
            Some(1)
        );
        let w = TestWorld {
            book,
            fonts: data.into_iter().map(Font::from_data).collect(),
            library: Library::new(),
        };
        let a = TextStyle {
            font: Some(FontList::single("Primary".into())),
            size: Pt(12.0),
            ..Default::default()
        };
        let b = TextStyle { math: true, ..a.clone() };
        let text = c.to_string();
        let fresh_a = FallbackFontMetrics::new(&w)
            .advance_shaped(&text, a.size, &a)
            .unwrap();
        let fresh_b = FallbackFontMetrics::new(&w)
            .advance_shaped(&text, b.size, &b)
            .unwrap();
        assert_ne!(fresh_a, fresh_b, "must discriminate");
        for styles in [[&a, &b], [&b, &a]] {
            let shared = FallbackFontMetrics::new(&w);
            for style in styles {
                let fresh = if style.math { fresh_b } else { fresh_a };
                assert_eq!(shared.advance_shaped(&text, style.size, style), Some(fresh));
            }
        }
    }
    #[test]
    fn infra_review_explicit_size_matches_fresh() {
        let data = bytes("Libertinus Serif");
        let mut book = FontBook::new();
        book.push(crate::fonts::font_info_from_bytes(&data, 0).unwrap());
        let w = TestWorld {
            book,
            fonts: vec![Font::from_data(data)],
            library: Library::new(),
        };
        let style = TextStyle {
            font: Some(FontList::single("Libertinus Serif".into())),
            ..TextStyle::regular(Pt(12.0))
        };
        for shaped in [false, true] {
            let measure = |metrics: &FallbackFontMetrics<'_>, size: Pt| {
                if shaped {
                    metrics.advance_shaped("AV", size, &style).unwrap()
                } else {
                    metrics.advance("AV", size, &style)
                }
            };
            let small = measure(&FallbackFontMetrics::new(&w), Pt(12.0));
            let large = measure(&FallbackFontMetrics::new(&w), Pt(24.0));
            assert_eq!(
                large,
                small * 2.0,
                "explicit size must govern scaling; shaped={shaped}"
            );
            for sizes in [[Pt(12.0), Pt(24.0)], [Pt(24.0), Pt(12.0)]] {
                let shared = FallbackFontMetrics::new(&w);
                for size in sizes {
                    assert_eq!(
                        measure(&shared, size),
                        if size == Pt(12.0) { small } else { large },
                        "cached width must match fresh; shaped={shaped}"
                    );
                }
            }
        }
        assert_eq!(style.size, Pt(12.0));
    }
}
