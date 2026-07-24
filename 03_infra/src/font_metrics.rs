//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/font_metrics.md
//! @prompt-hash 033fca5c
//! @layer L3
//! @updated 2026-07-16

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use ttf_parser::Face;
use typst_core::contracts::world::World;
use typst_core::engine::layout::{FixedMetrics, FontMetrics};
use typst_core::entities::font_book::FontVariant;
use typst_core::entities::font_list::FontNamePattern;
use typst_core::entities::glyph_variants::{
    GlyphAssembly, GlyphPart, GlyphVariant, GlyphVariants, MathGlyphKern, MathKernRecord,
    MathKernTable,
};
use typst_core::entities::layout_types::{Pt, TextEdge, TextStyle};
use typst_core::entities::math_constants::MathConstants;
use typst_core::entities::world_types::Font;

use crate::fallback_fonts::fallback_font_list_for;
use crate::font_variant::{
    axis_variations_for_font_variant, axis_variations_for_text_style,
    text_style_to_font_variant,
};

/// Extrai variantes verticais de um glifo directamente a partir da face.
fn extract_variants(face: &Face<'_>, c: char) -> GlyphVariants {
    let glyph_id = match face.glyph_index(c) {
        Some(id) => id,
        None => return GlyphVariants::default(),
    };
    let math_table = match face.tables().math {
        Some(m) => m,
        None => return GlyphVariants::default(),
    };
    let variants_table = match math_table.variants {
        Some(v) => v,
        None => return GlyphVariants::default(),
    };
    let construction = match variants_table.vertical_constructions.get(glyph_id) {
        Some(c) => c,
        None => return GlyphVariants::default(),
    };
    GlyphVariants {
        variants: construction
            .variants
            .into_iter()
            .map(|r| GlyphVariant {
                glyph_id: r.variant_glyph.0,
                advance: r.advance_measurement as f64,
            })
            .collect(),
    }
}

/// Extrai a assembly vertical de um glifo directamente a partir da face.
fn extract_assembly(face: &Face<'_>, c: char) -> GlyphAssembly {
    let glyph_id = match face.glyph_index(c) {
        Some(id) => id,
        None => return GlyphAssembly::default(),
    };
    let math_table = match face.tables().math {
        Some(m) => m,
        None => return GlyphAssembly::default(),
    };
    let variants_table = match math_table.variants {
        Some(v) => v,
        None => return GlyphAssembly::default(),
    };
    let construction = match variants_table.vertical_constructions.get(glyph_id) {
        Some(c) => c,
        None => return GlyphAssembly::default(),
    };
    let ttf_assembly = match construction.assembly {
        Some(a) => a,
        None => return GlyphAssembly::default(),
    };
    GlyphAssembly {
        parts: ttf_assembly
            .parts
            .into_iter()
            .map(|p| GlyphPart {
                glyph_id: p.glyph_id.0,
                start_connector: p.start_connector_length,
                end_connector: p.end_connector_length,
                full_advance: p.full_advance,
                is_extender: p.part_flags.extender(),
            })
            .collect(),
    }
}

/// Constrói o dicionário reverso preemptivo: glyph_id → char base.
///
/// Itera sobre os caracteres matemáticos extensíveis conhecidos, extrai
/// as variantes e as peças de assembly da tabela MATH, e guarda o
/// mapeamento inverso. Extensores mapeiam para `|` (barra vertical).
/// Usa `or_insert` para não sobrescrever se a fonte partilhar a peça
/// entre múltiplos caracteres base.
pub(crate) fn build_math_glyph_reverse_map(face: &Face<'_>) -> HashMap<u16, char> {
    const STRETCHY_BASES: &[char] = &['(', ')', '[', ']', '{', '}', '|', '√'];

    let mut map = HashMap::new();
    for &base_char in STRETCHY_BASES {
        for v in extract_variants(face, base_char).variants {
            map.entry(v.glyph_id).or_insert(base_char);
        }
        for part in extract_assembly(face, base_char).parts {
            let mapped = if part.is_extender { '|' } else { base_char };
            map.entry(part.glyph_id).or_insert(mapped);
        }
    }
    map
}

/// Devolve as métricas verticais tipográficas de uma face, preferindo os
/// valores do OS/2 (`sTypoAscender`, `sTypoDescender`, `sTypoLineGap`) quando
/// disponíveis, e caindo para as métricas do `hhea` caso contrário.
///
/// Retorna `(ascender, descender_abs, line_gap)` em unidades de fonte.
fn typo_metrics(face: &Face<'_>) -> (f64, f64, f64) {
    if let Some(os2) = face.tables().os2 {
        let typo_asc = os2.typographic_ascender();
        let typo_desc = os2.typographic_descender();
        if typo_asc != 0 || typo_desc != 0 {
            return (
                typo_asc as f64,
                (typo_desc as f64).abs(),
                os2.typographic_line_gap() as f64,
            );
        }
    }
    (face.ascender() as f64, (face.descender() as f64).abs(), face.line_gap() as f64)
}

/// **P762** — converte um `top-edge`/`bottom-edge` num offset em pontos
/// tipográficos relativos à baseline. Valores positivos são para cima no
/// caso do top; negativos para baixo no caso do bottom.
///
/// **P837** (achado #23 de P831) — `TextEdge::Length`: o edge é o
/// comprimento resolvido a partir da baseline, independente das métricas
/// da face (paridade `FontInstance::edges` do vanilla,
/// `text/font/mod.rs:276-289`: `top = length.at(font_size)`; no
/// cristalino o bottom é negativo-abaixo-da-baseline, logo
/// `bottom = length.resolve_pt(size)` directamente — `bottom-edge: -4pt`
/// → 4pt abaixo da baseline, medido no vanilla 0.15.0).
///
/// `pub(crate)` desde **P845** — o shaper reutiliza para o avanço vertical
/// entre linhas de texto com `\n` interno.
pub(crate) fn edge_offset_pt(
    face: &Face<'_>,
    upem: f64,
    size: Pt,
    edge: Option<&TextEdge>,
    is_top: bool,
) -> Pt {
    let default: &'static str = if is_top { "cap-height" } else { "baseline" };
    let edge = match edge {
        // P837 — Length explícito: resolve no font-size; não depende da face.
        Some(TextEdge::Length(l)) => return Pt(l.resolve_pt(size.val())),
        Some(TextEdge::Metric(m)) => m.as_str(),
        None => default,
    };

    let units = match edge {
        "baseline" => 0.0,
        "x-height" => face.x_height().map(|h| h as f64).unwrap_or_else(|| {
            // fallback: 0.5 em aproximado
            upem * 0.5
        }),
        "cap-height" => {
            let ascender = face
                .typographic_ascender()
                .filter(|&h| h > 0)
                .map(|h| h as f64)
                .unwrap_or_else(|| face.ascender() as f64);
            face.capital_height()
                .filter(|&h| h > 0)
                .map(|h| h as f64)
                .unwrap_or(ascender)
        }
        "ascender" => face
            .typographic_ascender()
            .filter(|&h| h > 0)
            .map(|h| h as f64)
            .unwrap_or_else(|| face.ascender() as f64),
        "descender" => face
            .typographic_descender()
            .filter(|&h| h < 0)
            .map(|h| h as f64)
            .unwrap_or_else(|| face.descender() as f64),
        _ => {
            // Edge desconhecido: comportamento defensivo igual ao default.
            return edge_offset_pt(face, upem, size, None, is_top);
        }
    };

    let pt = size * (units / upem);
    if is_top {
        pt
    } else {
        Pt(-pt.0.abs())
    }
}

/// Métricas de fonte reais via `ttf-parser`.
///
/// `font_size` não armazenado — passado em cada chamada (invariante do trait).
/// Lifetime `'a` ligado aos bytes da fonte.
pub struct FontBookMetrics<'a> {
    face: Face<'a>,
    upem: f64, // units_per_em — tipicamente 1000 ou 2048
    /// Dicionário reverso preemptivo: glyph_id → char base.
    /// Preenchido em `from_bytes`. Usado por `glyph_to_char`.
    glyph_to_unicode: HashMap<u16, char>,
}

impl<'a> FontBookMetrics<'a> {
    /// Constrói métricas a partir de bytes de fonte TrueType/OpenType.
    ///
    /// Retorna `None` se os bytes forem inválidos ou `upem == 0`.
    /// Protecção contra `upem == 0`: fallback para 1000 (não panic).
    pub fn from_bytes(data: &'a [u8]) -> Option<Self> {
        let face = Face::parse(data, 0).ok()?;
        let upem = face.units_per_em();
        let upem = if upem == 0 { 1000.0 } else { upem as f64 };
        let glyph_to_unicode = build_math_glyph_reverse_map(&face);
        Some(Self { face, upem, glyph_to_unicode })
    }
}

impl FontMetrics for FontBookMetrics<'_> {
    fn advance(&self, text: &str, size: Pt, _style: &TextStyle) -> Pt {
        // Fórmula: advance_pt = font_size * (Σ glyph_units / upem)
        let units: f64 = text
            .chars()
            .map(|c| {
                self.face
                    .glyph_index(c)
                    .and_then(|gid| self.face.glyph_hor_advance(gid))
                    .map(|a| a as f64)
                    .unwrap_or(self.upem * 0.6) // fallback para glifos ausentes
            })
            .sum();
        size * (units / self.upem)
    }

    fn vertical_metrics(&self, size: Pt, _style: &TextStyle) -> (Pt, Pt) {
        let (ascender, descender, line_gap) = typo_metrics(&self.face);

        let ascender_pt = size * (ascender / self.upem);
        let line_height_pt = size * ((ascender + descender + line_gap) / self.upem);

        (ascender_pt, line_height_pt)
    }

    /// **P750/P752/P761** — cap-height em pontos, com fallback para o ascender
    /// tipográfico (`sTypoAscender` do OS/2) e, em último caso, para o ascender
    /// da tabela `hhea`. Isto alinha-se com o vanilla, que usa
    /// `typographic_ascender().unwrap_or(ascender())` como fallback de
    /// `capital_height()`.
    fn cap_height(&self, size: Pt, _style: &TextStyle) -> Pt {
        let ascender = self
            .face
            .typographic_ascender()
            .filter(|&h| h > 0)
            .map(|h| h as f64)
            .unwrap_or_else(|| self.face.ascender() as f64);
        let cap = self
            .face
            .capital_height()
            .filter(|&h| h > 0)
            .map(|h| h as f64)
            .unwrap_or(ascender);
        size * (cap / self.upem)
    }

    fn text_edges(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
        let top =
            edge_offset_pt(&self.face, self.upem, size, style.top_edge.as_ref(), true);
        let bottom = edge_offset_pt(
            &self.face,
            self.upem,
            size,
            style.bottom_edge.as_ref(),
            false,
        );
        (top, bottom)
    }

    /// **P813** — limites de tinta do texto: união das bounding boxes
    /// reais dos glyphs (`glyph_index` + `glyph_bounding_box`), escaladas
    /// para pontos. Paridade vanilla (frame math usa bboxes dos glyphs).
    fn text_ink_bounds(&self, text: &str, size: Pt, _style: &TextStyle) -> (Pt, Pt) {
        let mut ascent = 0.0_f64;
        let mut descent = 0.0_f64;
        for c in text.chars() {
            let Some(gid) = self.face.glyph_index(c) else { continue };
            let Some(bbox) = self.face.glyph_bounding_box(gid) else { continue };
            ascent = ascent.max(size.val() * (bbox.y_max as f64 / self.upem));
            descent = descent.max(size.val() * (-(bbox.y_min as f64)) / self.upem);
        }
        (Pt(ascent), Pt(descent))
    }

    fn vertical_glyph_variants(&self, c: char) -> GlyphVariants {
        extract_variants(&self.face, c)
    }

    fn glyph_to_char(&self, glyph_id: u16) -> Option<char> {
        self.glyph_to_unicode.get(&glyph_id).copied()
    }

    fn vertical_glyph_assembly(&self, c: char) -> GlyphAssembly {
        extract_assembly(&self.face, c)
    }

    fn math_constants(&self) -> MathConstants {
        match self.face.tables().math {
            Some(math_table) => match math_table.constants {
                Some(c) => MathConstants {
                    upem: self.upem,
                    fraction_rule_thickness: c.fraction_rule_thickness().value as f64,
                    fraction_num_gap: c.fraction_numerator_gap_min().value as f64,
                    fraction_denom_gap: c.fraction_denominator_gap_min().value as f64,
                    superscript_shift_up: c.superscript_shift_up().value as f64,
                    subscript_shift_down: c.subscript_shift_down().value as f64,
                    radical_vertical_gap: c.radical_vertical_gap().value as f64,
                    radical_rule_thickness: c.radical_rule_thickness().value as f64,
                    axis_height: c.axis_height().value as f64,
                    script_percent_scale_down: c.script_percent_scale_down() as f64
                        / 100.0,
                    script_script_percent_scale_down: c.script_script_percent_scale_down()
                        as f64
                        / 100.0,
                    upper_limit_gap_min: c.upper_limit_gap_min().value as f64,
                    lower_limit_gap_min: c.lower_limit_gap_min().value as f64,
                    math_leading: c.math_leading().value as f64,
                },
                None => MathConstants::fallback(),
            },
            None => MathConstants::fallback(),
        }
    }

    fn math_kern(&self, c: char) -> MathGlyphKern {
        let glyph_id = match self.face.glyph_index(c) {
            Some(id) => id,
            None => return MathGlyphKern::default(),
        };

        let math = match self.face.tables().math {
            Some(m) => m,
            None => return MathGlyphKern::default(),
        };

        let glyph_info = match math.glyph_info {
            Some(gi) => gi,
            None => return MathGlyphKern::default(),
        };

        let kern_infos = match glyph_info.kern_infos {
            Some(k) => k,
            None => return MathGlyphKern::default(),
        };

        let kern_record = match kern_infos.get(glyph_id) {
            Some(r) => r,
            None => return MathGlyphKern::default(),
        };

        // Lê uma tabela Kern do ttf-parser em MathKernTable de L1.
        // A tabela tem `count` alturas e `count+1` valores de kern:
        //   kern[0] aplica-se até height[0], …, kern[count] aplica-se
        //   a todas as alturas acima de height[count-1].
        fn read_kern(kern: Option<ttf_parser::math::Kern>) -> MathKernTable {
            let kern = match kern {
                Some(k) => k,
                None => return MathKernTable::default(),
            };
            let count = kern.count() as usize;
            let mut records = Vec::with_capacity(count + 1);
            for i in 0..count {
                let height = kern.height(i as u16).map(|v| v.value as f64);
                let kv = kern.kern(i as u16).map(|v| v.value as f64).unwrap_or(0.0);
                records
                    .push(MathKernRecord { correction_height: height, kern_value: kv });
            }
            // Último valor de kern (sem correction_height associado)
            if let Some(kv) = kern.kern(count as u16).map(|v| v.value as f64) {
                records.push(MathKernRecord { correction_height: None, kern_value: kv });
            }
            MathKernTable { records }
        }

        MathGlyphKern {
            top_right: read_kern(kern_record.top_right),
            top_left: read_kern(kern_record.top_left),
            bottom_right: read_kern(kern_record.bottom_right),
            bottom_left: read_kern(kern_record.bottom_left),
        }
    }
}

/// Face parseada e bytes correspondentes, alocada em `Arc` para garantir
/// que os bytes não se movem depois de o `Face` ser criado.
///
/// Segue o mesmo padrão do Typst vanilla: o `Face` empresta internamente
/// dos bytes via um slice `'static` obtido com `from_raw_parts`. O campo
/// `data` nunca é movido porque a struct vive dentro de `Arc`.
struct CachedFace {
    // `data` é lido implicitamente pelo `Face`; mantém os bytes vivos.
    #[allow(dead_code)]
    data: Font,
    face: Face<'static>,
}

impl CachedFace {
    /// Parseia uma fonte e devolve a face cacheada.
    fn new(data: Font) -> Option<Arc<Self>> {
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

    fn face(&self) -> &Face<'_> {
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
struct ShapedWidthKey {
    text: String,
    size_bits: u64,
    font_hash: u64,
    bold: bool,
    italic: bool,
    weight: Option<u16>,
    dir: u8,
    lang: Option<typst_core::entities::lang::Lang>,
    axis_hash: u64,
}

/// **P677** — chave para cache de `advance` (caminho rápido não-shaped).
/// Inclui os mesmos campos de estilo que `ShapedWidthKey`; `tracking` é
/// aplicado fora do cache em `text_width`, pelo que não entra na chave.
#[derive(Debug, Hash, Eq, PartialEq)]
struct AdvanceWidthKey {
    text: String,
    size_bits: u64,
    font_hash: u64,
    bold: bool,
    italic: bool,
    weight: Option<u16>,
    dir: u8,
    lang: Option<typst_core::entities::lang::Lang>,
    axis_hash: u64,
}

pub struct FallbackFontMetrics<'a> {
    world: &'a dyn World,
    cache: Arc<Mutex<HashMap<usize, Arc<CachedFace>>>>,
    shaped_width_cache: Arc<Mutex<HashMap<ShapedWidthKey, Pt>>>,
    /// P673 — cache de faces ttf-parser para `shaped_width`, partilhada entre
    /// todas as chamadas de `advance_shaped` no mesmo documento. Evita
    /// re-parsear as mesmas fontes em cada medição de palavra.
    shaper_face_cache: Arc<Mutex<crate::shaper::FaceCache>>,
    /// P677 — cache de larguras `advance` já calculadas. Evita re-medir o
    /// mesmo texto+estilo (especialmente espaços e palavras repetidas) em
    /// cada chamada do layout.
    advance_width_cache: Arc<Mutex<HashMap<AdvanceWidthKey, Pt>>>,
}

/// Candidata a fonte para medição.
#[derive(Clone, Copy)]
struct FontCandidate {
    slot_idx: usize,
    units_per_em: u16,
}

/// Fontes padrão de fallback usadas pelo shaper (P538e/P543).
/// Devem ser consistentes entre `FallbackFontMetrics` e `shaper.rs` para
/// evitar desalinhamento de posicionamento no PDF.
impl<'a> FallbackFontMetrics<'a> {
    /// Constrói métricas de fallback a partir do `World`.
    pub fn new(world: &'a dyn World) -> Self {
        Self {
            world,
            cache: Arc::new(Mutex::new(HashMap::new())),
            shaped_width_cache: Arc::new(Mutex::new(HashMap::new())),
            shaper_face_cache: Arc::new(Mutex::new(crate::shaper::FaceCache::new())),
            advance_width_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// **P591** — constrói uma chave de cache para `advance_shaped`.
    fn shaped_width_key(text: &str, style: &TextStyle) -> Option<ShapedWidthKey> {
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
        let variant = text_style_to_font_variant(style);
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
        })
    }

    /// **P591** — cache lookup/inserção para `advance_shaped`.
    fn cached_shaped_width(
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
    fn advance_width_key(text: &str, style: &TextStyle) -> Option<AdvanceWidthKey> {
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

        let variant = text_style_to_font_variant(style);
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
            size_bits: style.size.0.to_bits(),
            font_hash,
            bold: style.bold,
            italic: style.italic,
            weight: style.weight,
            dir,
            lang: style.lang,
            axis_hash,
        })
    }

    /// **P677** — cache lookup/inserção para `advance`.
    fn cached_advance_width(
        &self,
        text: &str,
        style: &TextStyle,
        compute: impl FnOnce() -> Pt,
    ) -> Pt {
        let key = match Self::advance_width_key(text, style) {
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
    fn cached_face(&self, slot_idx: usize) -> Option<Arc<CachedFace>> {
        if let Some(cached) = self.cache.lock().unwrap().get(&slot_idx).cloned() {
            return Some(cached);
        }
        let font = self.world.font(slot_idx)?;
        let cached = CachedFace::new(font)?;
        self.cache.lock().unwrap().insert(slot_idx, cached.clone());
        Some(cached)
    }

    /// Resolve as fontes primárias declaradas no estilo. Se o estilo não
    /// declarar fonte (ou nenhuma resolver), replica o fallback padrão do
    /// shaper (`DEFAULT_FALLBACK_FONTS`) para manter as métricas alinhadas
    /// com a face que será efectivamente usada no shaping/PDF.
    fn resolve_primary(&self, style: &TextStyle) -> Vec<FontCandidate> {
        let mut primary = Vec::new();
        let variant = text_style_to_font_variant(style);

        // Nome da primeira família para decidir a classe de fallback (P555).
        let first_family: Option<&str> = style
            .font
            .as_ref()
            .and_then(|fl| fl.as_slice().first())
            .and_then(|f| f.name.as_str());

        if let Some(font_list) = &style.font {
            for family in font_list.as_slice() {
                let Some(idx) = self.world.book().select_pattern(&family.name, &variant)
                else {
                    continue;
                };
                let Some(cached) = self.cached_face(idx) else { continue };
                primary.push(FontCandidate {
                    slot_idx: idx,
                    units_per_em: cached.face().units_per_em().max(1) as u16,
                });
            }
        }

        if primary.is_empty() {
            let fallback_list = fallback_font_list_for(first_family.unwrap_or(""));
            for family in fallback_list {
                let pattern = FontNamePattern::Literal(ecow::EcoString::from(*family));
                let Some(idx) = self.world.book().select_pattern(&pattern, &variant)
                else {
                    continue;
                };
                let Some(cached) = self.cached_face(idx) else { continue };
                primary.push(FontCandidate {
                    slot_idx: idx,
                    units_per_em: cached.face().units_per_em().max(1) as u16,
                });
                break;
            }
        }

        primary
    }

    /// Encontra a fonte que cobre `c`: primárias primeiro (a primeira que
    /// cobre); se nenhuma cobrir, recolhe **todas** as fontes do `FontBook`
    /// que cobrem o caractere e escolhe via `FontBook::select_fallback`
    /// (**P838** — scoring de similaridade do vanilla, com `like` = `FontInfo`
    /// da primeira primária). Isto alinha a fonte usada na medição com a
    /// usada no shaping, que aplica o mesmo scoring.
    fn covering(
        &self,
        c: char,
        primary: &[FontCandidate],
        variant: &FontVariant,
    ) -> Option<FontCandidate> {
        for cand in primary {
            let cached = self.cached_face(cand.slot_idx)?;
            if cached.face().glyph_index(c).is_some() {
                return Some(*cand);
            }
        }

        let book = self.world.book();
        let like = primary.first().and_then(|cand| book.infos().get(cand.slot_idx));
        let mut ids = Vec::new();
        // **P880** — usar o filtro de coverage via World::candidates_for_char.
        // SystemWorld calcula coverage lazy; MockWorlds usam o FontBook via default.
        // Isso evita carregar dezenas de faces CJK só para verificar cobertura
        // de caracteres que elas não cobrem.
        for slot_idx in self.world.candidates_for_char(c) {
            if primary.iter().any(|cand| cand.slot_idx == slot_idx) {
                continue;
            }
            // P838 — uma face inválida não aborta o scan (antes: `?`
            // devolvia None para todo o fallback).
            let Some(cached) = self.cached_face(slot_idx) else { continue };
            if cached.face().glyph_index(c).is_some() {
                ids.push(slot_idx);
            }
        }

        let best = book.select_fallback(like, variant, ids)?;
        let cached = self.cached_face(best)?;
        Some(FontCandidate {
            slot_idx: best,
            units_per_em: cached.face().units_per_em().max(1) as u16,
        })
    }
}

impl Clone for FallbackFontMetrics<'_> {
    fn clone(&self) -> Self {
        Self {
            world: self.world,
            cache: self.cache.clone(),
            shaped_width_cache: self.shaped_width_cache.clone(),
            shaper_face_cache: self.shaper_face_cache.clone(),
            advance_width_cache: self.advance_width_cache.clone(),
        }
    }
}

/// Kerning entre dois glifos numa face, consultando as tabelas legacy
/// `kern` e `kerx`. GPOS kerning não é suportado nesta iteração.
fn face_kerning(
    face: &Face<'_>,
    left: ttf_parser::GlyphId,
    right: ttf_parser::GlyphId,
) -> i16 {
    if let Some(kern) = face.tables().kern {
        for subtable in kern.subtables {
            if let Some(v) = subtable.glyphs_kerning(left, right) {
                return v;
            }
        }
    }
    if let Some(kerx) = face.tables().kerx {
        for subtable in kerx.subtables {
            if let Some(v) = subtable.glyphs_kerning(left, right) {
                return v;
            }
        }
    }
    0
}

impl FontMetrics for FallbackFontMetrics<'_> {
    fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
        // **P677** — cache de `advance` por (texto, estilo). Evita re-medir
        // palavras e espaços repetidos no layout de documentos extensos.
        self.cached_advance_width(text, style, || {
            let primary = self.resolve_primary(style);
            // **P772o** — mesmas coordenadas de eixo que já diferenciam a
            // chave de cache (P659, `advance_width_key`) e que o shaper usa
            // para desenhar os glifos (`shaper.rs::shape`,
            // `rb_face.set_variations`). Antes desta correcção, nunca eram
            // aplicadas à face usada para *medir* — `glyph_hor_advance`
            // lia sempre a instância por omissão da fonte variável (ex.:
            // `wght=400`), independentemente do peso pedido. Causa
            // confirmada (instrumentação + comparação com `fontTools`,
            // P772o) do colapso de espaço entre palavras em fontes
            // variáveis de peso alto: o layout posicionava cada palavra
            // usando a largura da instância por omissão (mais estreita),
            // mas o shaper desenhava os glifos já na instância pedida
            // (mais larga) — o erro acumulado ao longo da palavra excedia
            // a largura do espaço.
            let variant = text_style_to_font_variant(style);
            let axis_vars = axis_variations_for_text_style(style);
            let mut total = 0.0;
            let mut prev: Option<(usize, u16)> = None;

            for c in text.chars() {
                let cand = self.covering(c, &primary, &variant);
                let mut slot = None;
                let mut gid = 0u16;

                let char_pt = cand
                    .and_then(|cand| {
                        let cached = self.cached_face(cand.slot_idx)?;
                        let mut face = cached.face().clone();
                        for v in &axis_vars {
                            face.set_variation(v.tag, v.value);
                        }
                        let g = face.glyph_index(c)?;
                        let adv = face.glyph_hor_advance(g)?;
                        slot = Some(cand.slot_idx);
                        gid = g.0;
                        Some(adv as f64 * size.val() / cand.units_per_em as f64)
                    })
                    .unwrap_or(size.val() * 0.6);

                if let (Some(slot_idx), Some((prev_slot, prev_gid))) = (slot, prev) {
                    if prev_slot == slot_idx {
                        if let Some(cached) = self.cached_face(slot_idx) {
                            let face = cached.face();
                            let kern = face_kerning(
                                face,
                                ttf_parser::GlyphId(prev_gid),
                                ttf_parser::GlyphId(gid),
                            );
                            let upem = face.units_per_em().max(1) as f64;
                            total += kern as f64 * size.val() / upem;
                        }
                    }
                }

                total += char_pt;
                prev = slot.map(|s| (s, gid));
            }

            Pt(total)
        })
    }

    /// **P591** — para scripts contextuais (árabe, síriaco, etc.), usa o
    /// shaper para obter a largura real com formas ligadas. Para os restantes
    /// scripts, mantém o caminho rápido `advance`.
    fn advance_shaped(&self, text: &str, _size: Pt, style: &TextStyle) -> Option<Pt> {
        use typst_core::engine::layout::needs_shaped_width;
        if !needs_shaped_width(text) {
            return None;
        }
        let world = self.world;
        let mut face_cache = self.shaper_face_cache.lock().unwrap();
        self.cached_shaped_width(text, style, || {
            crate::shaper::shaped_width(world, text, style, &mut face_cache)
        })
    }

    fn vertical_metrics(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
        // **P760** — usa a mesma face que o estilo resolve (primárias + fallback),
        // em vez da primeira fonte arbitrária do FontBook. Isto alinha o
        // line_height com a fonte efectivamente usada para renderizar.
        // As métricas tipográficas do OS/2 são preferidas, como no Typst vanilla.
        let primary = self.resolve_primary(style);
        if let Some(cand) = primary.first() {
            if let Some(cached) = self.cached_face(cand.slot_idx) {
                let face = cached.face();
                let upem = cand.units_per_em as f64;
                let (ascender, descender, line_gap) = typo_metrics(face);
                return (
                    size * (ascender / upem),
                    size * ((ascender + descender + line_gap) / upem),
                );
            }
        }

        // Fallback final: primeira fonte do book (comportamento anterior).
        let book_len = self.world.book().len();
        for slot_idx in 0..book_len {
            let Some(cached) = self.cached_face(slot_idx) else { continue };
            let face = cached.face();
            let upem = face.units_per_em().max(1) as f64;
            let (ascender, descender, line_gap) = typo_metrics(face);
            return (
                size * (ascender / upem),
                size * ((ascender + descender + line_gap) / upem),
            );
        }
        // Fallback último: proporções fixas se não houver nenhuma fonte.
        (size * 0.8, size * 1.2)
    }

    /// **P750/P752/P761** — cap-height da fonte resolvida para o estilo activo,
    /// com fallback para o ascender tipográfico (`sTypoAscender` do OS/2) e,
    /// em último caso, para o ascender da tabela `hhea`. Isto alinha-se com o
    /// vanilla, que usa `typographic_ascender().unwrap_or(ascender())` como
    /// fallback de `capital_height()`.
    ///
    /// Usa `resolve_primary(style)` para escolher a mesma face que o shaper
    /// usará, em vez da primeira fonte arbitrária do FontBook.
    fn cap_height(&self, size: Pt, style: &TextStyle) -> Pt {
        let primary = self.resolve_primary(style);

        // Se o estilo declarou uma fonte primária, usar a primeira que
        // resolveu. Caso contrário, `resolve_primary` já aplicou a lista de
        // fallback do shaper.
        if let Some(cand) = primary.first() {
            if let Some(cached) = self.cached_face(cand.slot_idx) {
                let face = cached.face();
                let upem = cand.units_per_em as f64;
                let ascender = face
                    .typographic_ascender()
                    .filter(|&h| h > 0)
                    .map(|h| h as f64)
                    .unwrap_or_else(|| face.ascender() as f64);
                let cap = face
                    .capital_height()
                    .filter(|&h| h > 0)
                    .map(|h| h as f64)
                    .unwrap_or(ascender);
                return size * (cap / upem);
            }
        }

        // Fallback final: primeira fonte do book (comportamento anterior).
        let book_len = self.world.book().len();
        for slot_idx in 0..book_len {
            let Some(cached) = self.cached_face(slot_idx) else { continue };
            let face = cached.face();
            let upem = face.units_per_em().max(1) as f64;
            let ascender = face
                .typographic_ascender()
                .filter(|&h| h > 0)
                .map(|h| h as f64)
                .unwrap_or_else(|| face.ascender() as f64);
            let cap = face
                .capital_height()
                .filter(|&h| h > 0)
                .map(|h| h as f64)
                .unwrap_or(ascender);
            return size * (cap / upem);
        }
        // Fallback último: mesma aproximação de FixedMetrics.
        size * 0.7
    }

    fn text_edges(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
        let primary = self.resolve_primary(style);

        if let Some(cand) = primary.first() {
            if let Some(cached) = self.cached_face(cand.slot_idx) {
                let face = cached.face();
                let upem = cand.units_per_em as f64;
                let top =
                    edge_offset_pt(face, upem, size, style.top_edge.as_ref(), true);
                let bottom =
                    edge_offset_pt(face, upem, size, style.bottom_edge.as_ref(), false);
                return (top, bottom);
            }
        }

        // Fallback final: primeira fonte do book.
        let book_len = self.world.book().len();
        for slot_idx in 0..book_len {
            let Some(cached) = self.cached_face(slot_idx) else { continue };
            let face = cached.face();
            let upem = face.units_per_em().max(1) as f64;
            let top = edge_offset_pt(face, upem, size, style.top_edge.as_ref(), true);
            let bottom =
                edge_offset_pt(face, upem, size, style.bottom_edge.as_ref(), false);
            return (top, bottom);
        }

        // Fallback último: métricas fixas.
        FixedMetrics.text_edges(size, style)
    }

    /// **P813** — limites de tinta do texto: união das bounding boxes
    /// reais dos glyphs, resolvendo cada char pela mesma cadeia de
    /// cobertura do shaping (`resolve_primary` + cadeia math P784 +
    /// `covering`) — a tinta de um char math vem da fonte math de
    /// fallback (ex.: NewCMMath), tal como no render. Paridade vanilla
    /// (frame math usa bboxes dos glyphs).
    fn text_ink_bounds(&self, text: &str, size: Pt, style: &TextStyle) -> (Pt, Pt) {
        let mut primary = self.resolve_primary(style);
        let variant = text_style_to_font_variant(style);
        // Espelho do shaper (P784, shaper.rs): com `style.math`, a cadeia
        // de fallback matemático entra como primárias adicionais ANTES do
        // scan global do FontBook em `covering` — sem isto, chars math
        // (ex.: 𝑥/U+1D465) resolviam para uma face arbitrária do book em
        // vez da fonte math usada no render.
        if style.math {
            for family in crate::fallback_fonts::math_fallback_font_list() {
                let pattern = FontNamePattern::Literal(ecow::EcoString::from(*family));
                let Some(idx) = self.world.book().select_pattern(&pattern, &variant)
                else {
                    continue;
                };
                if primary.iter().any(|p| p.slot_idx == idx) {
                    continue;
                }
                let Some(cached) = self.cached_face(idx) else { continue };
                primary.push(FontCandidate {
                    slot_idx: idx,
                    units_per_em: cached.face().units_per_em().max(1) as u16,
                });
            }
        }
        let mut ascent = 0.0_f64;
        let mut descent = 0.0_f64;
        for c in text.chars() {
            let Some(cand) = self.covering(c, &primary, &variant) else { continue };
            let Some(cached) = self.cached_face(cand.slot_idx) else { continue };
            let face = cached.face();
            let Some(gid) = face.glyph_index(c) else { continue };
            let Some(bbox) = face.glyph_bounding_box(gid) else { continue };
            let upem = cand.units_per_em as f64;
            ascent = ascent.max(size.val() * (bbox.y_max as f64 / upem));
            descent = descent.max(size.val() * (-(bbox.y_min as f64)) / upem);
        }
        (Pt(ascent), Pt(descent))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bytes_invalidos_retorna_none() {
        assert!(FontBookMetrics::from_bytes(b"not a font").is_none());
        assert!(FontBookMetrics::from_bytes(b"").is_none());
    }

    /// **P813** — `text_ink_bounds` mede a tinta real dos glyphs (bboxes),
    /// não as métricas globais da fonte: 'H' ≈ cap-height sem descent;
    /// 'g' tem tinta abaixo da baseline.
    #[test]
    fn p813_text_ink_bounds_medem_tinta_real() {
        let data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/NimbusSans-Regular.otf"
        ))
        .expect("fixture NimbusSans-Regular.otf necessária");
        let m = FontBookMetrics::from_bytes(&data).expect("fonte válida");
        let style = TextStyle::default();
        let size = Pt(11.0);

        let (asc_h, desc_h) = m.text_ink_bounds("H", size, &style);
        let cap = m.cap_height(size, &style);
        assert!(
            (asc_h.val() - cap.val()).abs() < 0.5,
            "tinta de 'H' ≈ cap-height: ink={:.3}pt cap={:.3}pt",
            asc_h.val(),
            cap.val()
        );
        assert!(
            desc_h.val() < 0.01,
            "'H' sem tinta abaixo da baseline: {:.3}pt",
            desc_h.val()
        );

        let (_, desc_g) = m.text_ink_bounds("g", size, &style);
        assert!(
            desc_g.val() > 1.0,
            "'g' deve ter tinta abaixo da baseline (descender): {:.3}pt",
            desc_g.val()
        );

        // União de glyphs: "Hg" combina o ascent de 'H' e o descent de 'g'.
        let (asc_hg, desc_hg) = m.text_ink_bounds("Hg", size, &style);
        assert!(asc_hg.val() >= asc_h.val() - 0.001);
        assert!(desc_hg.val() >= desc_g.val() - 0.001);
    }

    /// **P837** (achado #23 de P831) — `text_edges` com `Length` explícito
    /// numa face real: o edge é o comprimento resolvido a partir da
    /// baseline, independente das métricas da fonte (paridade vanilla
    /// `FontInstance::edges`, `text/font/mod.rs:276-289`).
    #[test]
    fn p837_text_edges_length_explicito_face_real() {
        use typst_core::entities::layout_types::{Length, TextEdge};

        let data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/NimbusSans-Regular.otf"
        ))
        .expect("fixture NimbusSans-Regular.otf necessária");
        let m = FontBookMetrics::from_bytes(&data).expect("fonte válida");
        let size = Pt(11.0);

        let mut style = TextStyle::default();
        style.top_edge = Some(TextEdge::Length(Length::pt(18.0)));
        style.bottom_edge = Some(TextEdge::Length(Length::pt(-4.0)));
        let (top, bottom) = m.text_edges(size, &style);
        assert!((top.val() - 18.0).abs() < 1e-9, "top={:?}", top);
        assert!((bottom.val() + 4.0).abs() < 1e-9, "bottom={:?}", bottom);

        // Componente em resolve no font-size: 1.5em a 11pt = 16.5pt.
        let mut style = TextStyle::default();
        style.top_edge = Some(TextEdge::Length(Length::em(1.5)));
        let (top, _) = m.text_edges(size, &style);
        assert!((top.val() - 16.5).abs() < 1e-9, "top={:?}", top);

        // Métrica enumerada continua a vir da face (sem regressão):
        // "baseline" = 0 nos dois edges.
        let mut style = TextStyle::default();
        style.top_edge = Some(TextEdge::Metric("baseline".into()));
        style.bottom_edge = Some(TextEdge::Metric("baseline".into()));
        let (top, bottom) = m.text_edges(size, &style);
        assert_eq!(top.val(), 0.0, "top baseline={:?}", top);
        assert_eq!(bottom.val(), 0.0, "bottom baseline={:?}", bottom);
    }

    #[test]
    #[ignore = "requer tests/fixtures/liberation-sans-regular.ttf"]
    fn proporcionalidade_iiii_vs_wwww() {
        let data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/liberation-sans-regular.ttf"
        ))
        .expect("fixture necessária");

        let m = FontBookMetrics::from_bytes(&data).expect("fonte válida");
        let size = Pt(12.0);
        let style = TextStyle::default();

        let ai = m.advance("iiii", size, &style);
        let aw = m.advance("WWWW", size, &style);

        assert!(
            ai.val() < aw.val(),
            "proporcional: 'iiii' ({:.2}pt) deve ser mais estreito que 'WWWW' ({:.2}pt)\n\
             Diagnóstico: se iiii ≈ 0.07pt → esqueceu size*; se iiii ≈ 700pt → esqueceu /upem",
            ai.val(), aw.val()
        );

        let aa = m.advance("A", size, &style);
        assert!(
            aa.val() > 3.0 && aa.val() < 12.0,
            "'A' em 12pt deve ser 3–12pt, foi {:.2}pt",
            aa.val()
        );
    }

    #[test]
    #[ignore = "requer tests/fixtures/liberation-sans-regular.ttf"]
    fn upem_zero_nao_causa_divisao_por_zero() {
        // Bytes inválidos → None (nunca chega a upem=0 em advance)
        assert!(FontBookMetrics::from_bytes(b"not a font").is_none());
        assert!(FontBookMetrics::from_bytes(b"").is_none());
    }

    #[test]
    fn p548_kerning_aplicado_em_dejavu_sans() {
        use crate::world::SystemWorld;
        use typst_core::contracts::world::World;

        let dir = std::env::temp_dir().join(format!(
            "typst-fontmetrics-test-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("main.typ"), "text").unwrap();
        let Ok(world) = SystemWorld::new(&dir, "main.typ").map(|w| w.with_system_fonts())
        else {
            return;
        };
        if world.book().is_empty() {
            return;
        }

        let metrics = FallbackFontMetrics::new(&world);
        let mut style = TextStyle::default();
        style.font = Some(typst_core::entities::font_list::FontList::single(
            ecow::EcoString::from("DejaVu Sans"),
        ));
        style.size = Pt(12.0);

        let text = metrics.advance("Texto", Pt(12.0), &style);
        let t = metrics.advance("T", Pt(12.0), &style);
        let exto = metrics.advance("exto", Pt(12.0), &style);
        assert!(
            text.val() < t.val() + exto.val(),
            "kerning deve reduzir a largura de 'Texto'"
        );
    }

    #[test]
    fn p591_advance_shaped_arabico_reduz_largura() {
        use crate::world::SystemWorld;
        use typst_core::contracts::world::World;

        let dir = std::env::temp_dir().join(format!(
            "typst-fontmetrics-test-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("main.typ"), "text").unwrap();
        let Ok(world) = SystemWorld::new(&dir, "main.typ").map(|w| w.with_system_fonts())
        else {
            return;
        };
        if world.book().is_empty() {
            return;
        }

        let metrics = FallbackFontMetrics::new(&world);
        let mut style = TextStyle::default();
        style.font = Some(typst_core::entities::font_list::FontList::single(
            ecow::EcoString::from("DejaVu Sans"),
        ));
        style.size = Pt(40.0);
        style.lang = Some("ar".parse().unwrap());
        style.dir = Some(typst_core::entities::dir::Dir::RTL);

        // Para palavras árabes, a largura com shaping deve ser menor que a
        // soma das letras isoladas.
        let plain = metrics.advance("الكتاب", Pt(40.0), &style);
        let shaped = metrics.advance_shaped("الكتاب", Pt(40.0), &style);
        assert!(shaped.is_some(), "advance_shaped deve retornar Some para arabe");
        assert!(
            shaped.unwrap().val() < plain.val() - 10.0,
            "shaped width de 'الكتاب' deve ser significativamente menor que non-shaped; plain={:.4}, shaped={:.4}",
            plain.val(), shaped.unwrap().val()
        );

        // Dígitos latinos não precisam de shaping.
        let digits_plain = metrics.advance("42", Pt(40.0), &style);
        let digits_shaped = metrics.advance_shaped("42", Pt(40.0), &style);
        assert!(
            digits_shaped.is_none()
                || (digits_shaped.unwrap().val() - digits_plain.val()).abs() < 0.1,
            "'42' nao deve sofrer shaping contextual"
        );
    }

    /// **P838** (achado #24 de P831) — o fallback global de `covering` usa o
    /// scoring de similaridade do vanilla (`FontBook::select_fallback`) em vez
    /// da primeira fonte por ordem de índice: para `like` sans-serif
    /// (DejaVu Sans), CJK resolve para "Noto Sans CJK JP" e não para
    /// "Droid Sans Fallback", alinhando a fonte medida com a do shaping.
    #[test]
    fn p838_covering_fallback_scoring_vanilla() {
        use crate::world::SystemWorld;
        use typst_core::contracts::world::World;

        let dir = std::env::temp_dir().join(format!(
            "typst-fontmetrics-test-p838-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("main.typ"), "text").unwrap();
        let Ok(world) = SystemWorld::new(&dir, "main.typ").map(|w| w.with_system_fonts())
        else {
            return;
        };
        let book = world.book();
        if !book.infos().iter().any(|i| i.family == "Noto Sans CJK JP") {
            eprintln!("SKIP: Noto Sans CJK JP não instalada");
            return;
        }
        if !book.infos().iter().any(|i| i.family == "Droid Sans Fallback") {
            eprintln!("SKIP: Droid Sans Fallback não instalada");
            return;
        }

        let metrics = FallbackFontMetrics::new(&world);
        let mut style = TextStyle::default();
        style.font = Some(typst_core::entities::font_list::FontList::single(
            ecow::EcoString::from("DejaVu Sans"),
        ));
        let primary = metrics.resolve_primary(&style);
        assert!(!primary.is_empty(), "DejaVu Sans resolve como primária");

        let cand = metrics
            .covering('日', &primary, &FontVariant::default())
            .expect("alguma fonte do book cobre CJK");
        let chosen = &book.infos()[cand.slot_idx];
        assert_eq!(
            chosen.family, "Noto Sans CJK JP",
            "scoring vanilla escolhe Noto Sans CJK JP, não {:?}",
            chosen.family
        );
        assert_eq!(
            chosen.variant.weight.to_number(),
            400,
            "a face Regular (w400) vence a Bold (w700) por distância de variante, \
             como no vanilla (NotoSansCJKjp-Regular)"
        );
    }

    // P772o — `advance()` deve aplicar as coordenadas de eixo (wght) à face
    // antes de medir, não só à chave de cache (P659). Sem a correcção,
    // `advance("W", ..., weight: 800)` devolvia sempre o valor da instância
    // por omissão (wght=400) da fonte variável — causa confirmada do colapso
    // de espaço entre palavras em peso alto (P772m/P772o).
    #[test]
    fn p772o_advance_aplica_variacao_wght_em_fonte_variavel() {
        use crate::world::SystemWorld;

        let dir = std::env::temp_dir().join(format!(
            "typst-fontmetrics-test-p772o-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("main.typ"), "text").unwrap();
        let font_dir = std::path::PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts"
        ));
        let Ok(world) = SystemWorld::new(&dir, "main.typ")
            .map(|w| w.with_fonts_and_system(&[font_dir]))
        else {
            return;
        };

        let metrics = FallbackFontMetrics::new(&world);
        let mut style_400 = TextStyle::default();
        style_400.font = Some(typst_core::entities::font_list::FontList::single(
            ecow::EcoString::from("Ubuntu Sans"),
        ));
        style_400.size = Pt(11.0);
        style_400.weight = Some(400);

        let mut style_800 = style_400.clone();
        style_800.weight = Some(800);

        let adv_400 = metrics.advance("Weight", Pt(11.0), &style_400);
        let adv_800 = metrics.advance("Weight", Pt(11.0), &style_800);

        // Medido (P772o, via fontTools.varLib.instancer): a soma dos avanços
        // de "Weight" cresce de 3258 para 3448 unidades (upem 1000) entre
        // wght=400 e wght=800 — ~2.09pt a mais em 11pt. Sem a correcção,
        // os dois valores eram idênticos (sempre a instância por omissão).
        assert!(
            adv_800.val() > adv_400.val() + 1.5,
            "advance('Weight', wght=800) deve ser visivelmente maior que \
             wght=400 (fonte variável Ubuntu Sans alarga com o peso); \
             400={:.3}pt 800={:.3}pt",
            adv_400.val(),
            adv_800.val()
        );
    }

    // P659 — a chave de cache de shaped_width deve distinguir variações de eixo
    // OpenType. O mesmo texto, tamanho e peso nominal pode ter larguras shaped
    // diferentes se o eixo efectivo (ex.: wght) divergir.
    #[test]
    fn p659_shaped_width_key_distingue_variacoes_de_eixo() {
        let mut style_a = TextStyle::default();
        style_a.weight = Some(400);

        let mut style_b = TextStyle::default();
        style_b.weight = Some(700);

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
    #[ignore = "requer tests/fixtures/liberation-sans-regular.ttf"]
    fn vertical_metrics_sanidade() {
        let data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/liberation-sans-regular.ttf"
        ))
        .unwrap();
        let m = FontBookMetrics::from_bytes(&data).unwrap();
        let style = TextStyle::default();
        let (asc, lh) = m.vertical_metrics(Pt(12.0), &style);
        assert!(asc.val() > 0.0, "ascender positivo");
        assert!(lh.val() > asc.val(), "line_height > ascender");
        assert!(lh.val() < 24.0, "line_height em 12pt < 24pt");
        // Verificar que métricas escalam com font_size
        let (_, lh24) = m.vertical_metrics(Pt(24.0), &style);
        assert!(
            (lh24.val() - 2.0 * lh.val()).abs() < 0.5,
            "métricas devem escalar com font_size: 24pt ≈ 2× 12pt"
        );
    }

    /// **P880** — `FallbackFontMetrics::covering` deve respeitar o conjunto
    /// devolvido por `World::candidates_for_char` e NUNCA carregar faces cujo
    /// índice não esteja nesse conjunto. Teste de regressão do loop corrigido
    /// em P879/P880.
    #[test]
    fn p880_covering_nao_carrega_faces_fora_de_candidates_for_char() {
        use std::num::NonZeroU16;
        use typst_core::contracts::world::World;
        use typst_core::entities::file_id::FileId;
        use typst_core::entities::font_book::{
            Coverage, FontBook, FontFlags, FontInfo, FontStretch, FontStyle, FontVariant,
            FontWeight,
        };
        use typst_core::entities::source::Source;
        use typst_core::entities::world_types::{
            Bytes, Datetime, FileError, FileResult, Font, Library,
        };

        struct FilteredWorld {
            library: Library,
            book: FontBook,
            fonts: Vec<Option<Font>>,
            allowed: Vec<usize>,
        }

        impl World for FilteredWorld {
            fn library(&self) -> &Library {
                &self.library
            }
            fn book(&self) -> &FontBook {
                &self.book
            }
            fn main(&self) -> FileId {
                FileId::from_raw(NonZeroU16::new(1).unwrap())
            }
            fn source(&self, _: FileId) -> FileResult<Source> {
                Err(FileError::NotFound)
            }
            fn file(&self, _: FileId) -> FileResult<Bytes> {
                Err(FileError::NotFound)
            }
            fn font(&self, idx: usize) -> Option<Font> {
                assert!(
                    self.allowed.contains(&idx),
                    "covering carregou face fora de candidates_for_char: idx={}",
                    idx
                );
                self.fonts.get(idx).cloned().flatten()
            }
            fn today(&self, _: Option<i64>) -> Option<Datetime> {
                None
            }
            fn candidates_for_char(&self, _: char) -> Vec<usize> {
                self.allowed.clone()
            }
        }

        let data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/NimbusSans-Regular.otf"
        ))
        .expect("fixture NimbusSans-Regular.otf necessária");
        let nimbus = Font::from_data(data);

        let mut book = FontBook::new();
        book.push(FontInfo {
            family: "Nimbus Sans".into(),
            variant: FontVariant {
                style: FontStyle::Normal,
                weight: FontWeight::REGULAR,
                stretch: FontStretch::NORMAL,
            },
            flags: FontFlags::default(),
            coverage: Coverage::default(),
        });
        // Duas entradas fictícias que NÃO estão em `allowed`.
        for family in ["Dummy One", "Dummy Two"] {
            book.push(FontInfo {
                family: family.into(),
                variant: FontVariant {
                    style: FontStyle::Normal,
                    weight: FontWeight::REGULAR,
                    stretch: FontStretch::NORMAL,
                },
                flags: FontFlags::default(),
                coverage: Coverage::default(),
            });
        }

        let world = FilteredWorld {
            library: Library::new(),
            book,
            fonts: vec![Some(nimbus), None, None],
            allowed: vec![0],
        };

        let metrics = FallbackFontMetrics::new(&world);
        let mut style = TextStyle::default();
        style.font = Some(typst_core::entities::font_list::FontList::single(
            ecow::EcoString::from("Nimbus Sans"),
        ));
        let primary = metrics.resolve_primary(&style);
        assert!(!primary.is_empty(), "Nimbus Sans deve resolver como primária");

        // '你' (CJK) não é coberto por Nimbus Sans. O fallback global só deve
        // considerar os índices devolvidos por `candidates_for_char` (apenas 0),
        // e portanto não deve carregar os slots 1 e 2.
        let _ = metrics.covering('你', &primary, &FontVariant::default());
        // Se `covering` tentasse carregar slot 1 ou 2, `FilteredWorld::font`
        // daria panic.
    }
}
