//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/font_metrics.md
//! @prompt-hash 24c29899
//! @layer L3
//! @updated 2026-07-24

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use ttf_parser::Face;
use typst_core::contracts::world::World;
use typst_core::compiler::layout::{FixedMetrics, FontMetrics};
use typst_core::entities::font_book::FontVariant;
use typst_core::entities::font_list::{FontList, FontNamePattern};
use typst_core::entities::font_variations::FontVariations;
use typst_core::entities::glyph_variants::{
    GlyphAssembly, GlyphPart, GlyphVariant, GlyphVariants, MathGlyphKern, MathKernRecord,
    MathKernTable,
};
use typst_core::entities::layout_types::{MathSize, Pt, TextEdge, TextStyle};
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
                // **P917** — avanço horizontal NATIVO do glifo (hmtx),
                // nunca `advance_measurement` (eixo de esticamento, aqui
                // altura). Ver `entities/glyph_variants.md` §P917.
                hor_advance: face.glyph_hor_advance(r.variant_glyph).unwrap_or(0) as f64,
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
        // **P945** — `minConnectorOverlap` da tabela MATH, lido do mesmo
        // sítio onde o vanilla o lê (`glyph.rs:542-548`). Ver
        // `infra/font_metrics.md` §P945.
        min_overlap: variants_table.min_connector_overlap,
        parts: ttf_assembly
            .parts
            .into_iter()
            .map(|p| GlyphPart {
                glyph_id: p.glyph_id.0,
                start_connector: p.start_connector_length,
                end_connector: p.end_connector_length,
                full_advance: p.full_advance,
                is_extender: p.part_flags.extender(),
                // **P917** — avanço horizontal nativo da peça (hmtx), nunca
                // `full_advance` (eixo de empilhamento). Ver `entities/
                // glyph_variants.md` §P917.
                hor_advance: face.glyph_hor_advance(p.glyph_id).unwrap_or(0) as f64,
            })
            .collect(),
    }
}

/// **P906** — extrai variantes horizontais de um glifo directamente a partir
/// da face. Espelha `extract_variants` acima — única diferença:
/// `variants_table.horizontal_constructions` em vez de `.vertical_
/// constructions` (campo simétrico da mesma struct `ttf_parser`) — ver
/// `infra/font_metrics.md` §P906.
fn extract_variants_horizontal(face: &Face<'_>, c: char) -> GlyphVariants {
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
    let construction = match variants_table.horizontal_constructions.get(glyph_id) {
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
                // **P917** — uniformidade com o extractor vertical: mesmo
                // para construções horizontais (onde `advance` e
                // `hor_advance` tendem a coincidir), a fonte de verdade
                // para posicionamento é sempre o hmtx nativo, nunca a
                // medida do eixo de esticamento.
                hor_advance: face.glyph_hor_advance(r.variant_glyph).unwrap_or(0) as f64,
            })
            .collect(),
    }
}

/// **P906** — extrai a assembly horizontal de um glifo directamente a partir
/// da face. Espelha `extract_assembly` acima, lendo `.horizontal_
/// constructions` em vez de `.vertical_constructions` — ver
/// `infra/font_metrics.md` §P906.
fn extract_assembly_horizontal(face: &Face<'_>, c: char) -> GlyphAssembly {
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
    let construction = match variants_table.horizontal_constructions.get(glyph_id) {
        Some(c) => c,
        None => return GlyphAssembly::default(),
    };
    let ttf_assembly = match construction.assembly {
        Some(a) => a,
        None => return GlyphAssembly::default(),
    };
    GlyphAssembly {
        // **P945** — mesmo `minConnectorOverlap` do extractor vertical
        // (campo da tabela `variants`, partilhado pelos dois eixos).
        min_overlap: variants_table.min_connector_overlap,
        parts: ttf_assembly
            .parts
            .into_iter()
            .map(|p| GlyphPart {
                glyph_id: p.glyph_id.0,
                start_connector: p.start_connector_length,
                end_connector: p.end_connector_length,
                full_advance: p.full_advance,
                is_extender: p.part_flags.extender(),
                // **P917** — mesma uniformidade de `extract_variants_horizontal` acima.
                hor_advance: face.glyph_hor_advance(p.glyph_id).unwrap_or(0) as f64,
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
    const STRETCHY_BASES: &[char] = &[
        '(', ')', '[', ']', '{', '}', '|', '‖', '∥', '⌊', '⌋', '⌈', '⌉', '⟨', '⟩', '〈', '〉', '√', '/', '\\',
        '↑', '↓', '↕', '⇑', '⇓', '⇕',
        // **P952** — operadores grandes (`MathClass::Large`) cujas variantes
        // de Display (`layout_large_operator_display`, `_comum.md` §P952)
        // precisam de entrar no subset/ToUnicode como qualquer outra
        // variante — sem isto, o `∑`/`∫` display emitidos como
        // `FrameItem::Glyph` caíam fora do subset embutido (glifo invisível
        // no PDF real, achado da revalidação end-to-end de P952).
        '∑', '∏', '∐', '⋃', '⋂', '⨄', '⨅', '⨆', '∫', '∬', '∭', '∮', '∯', '∰',
        '⨁', '⨂', '⨀', '⋀', '⋁',
    ];

    // **P906** — chars extensíveis no eixo horizontal (chaves/colchetes de
    // underbrace/overbrace/underbracket/overbracket + acentos largos
    // hat/tilde). Sem isto, os glyph_ids das suas variantes/assembly nunca
    // entravam neste mapa — nem no subsetting (P45/DEBT-9, `builder.rs:988`
    // consome exactamente este mapa) nem na selecção de `/Fn` em
    // `emit_glyph_pdf` (que passou a reaproveitá-lo neste mesmo passo) —
    // glifos de esticamento horizontal ficavam fora do subset embutido ou
    // referenciados pela fonte errada. Extensor mapeia para `_` (linha
    // horizontal), análogo ao `|` já usado para extensores verticais.
    const STRETCHY_BASES_HORIZONTAL: &[char] =
        &['⏟', '⏞', '⎵', '⎴', '⏝', '⏜', '\u{0302}', '\u{0303}'];

    // **P946** — mapa inverso da cmap da face (glyph_id → char), um passe:
    // peças de assembly com codepoint próprio (ex.: `⎧`/`⎨`/`⎩`/`⎪`,
    // `⎛`/`⎜`/`⎝`) passam a mapear para esse codepoint real — extracção
    // semanticamente mais rica (decisão do dono, divergência deliberada:
    // o vanilla agrupa a assembly num único char base no ToUnicode —
    // medição registada em `infra/font_metrics.md` §P946). Glifos não
    // codificados (variantes `.vN`, peças sem codepoint próprio, ex.:
    // `braceleft.ex`) caem no fallback `base_char`/`|`/`_` (comportamento
    // anterior).
    let mut cmap_reverse: HashMap<u16, char> = HashMap::new();
    if let Some(cmap) = face.tables().cmap {
        for subtable in cmap.subtables {
            subtable.codepoints(|cp| {
                let Some(ch) = char::from_u32(cp) else { return };
                let Some(gid) = subtable.glyph_index(cp) else { return };
                cmap_reverse.entry(gid.0).or_insert(ch);
            });
        }
    }
    let real_or = |glyph_id: u16, fallback: char| -> char {
        cmap_reverse.get(&glyph_id).copied().unwrap_or(fallback)
    };

    let mut map = HashMap::new();
    for &base_char in STRETCHY_BASES {
        for v in extract_variants(face, base_char).variants {
            map.entry(v.glyph_id).or_insert_with(|| real_or(v.glyph_id, base_char));
        }
        for part in extract_assembly(face, base_char).parts {
            let mapped = if part.is_extender { '|' } else { base_char };
            map.entry(part.glyph_id).or_insert_with(|| real_or(part.glyph_id, mapped));
        }
    }
    for &base_char in STRETCHY_BASES_HORIZONTAL {
        for v in extract_variants_horizontal(face, base_char).variants {
            map.entry(v.glyph_id).or_insert_with(|| real_or(v.glyph_id, base_char));
        }
        for part in extract_assembly_horizontal(face, base_char).parts {
            let mapped = if part.is_extender { '_' } else { base_char };
            map.entry(part.glyph_id).or_insert_with(|| real_or(part.glyph_id, mapped));
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

/// **P977** — nível ssty do estilo em contexto math: `Some(1)` para
/// `MathSize::Script`, `Some(2)` para `ScriptScript`, `None` nos restantes
/// casos (fora de math, Display, Text). Vanilla
/// `text/mod.rs:1457-1460`. Ver `infra/font_metrics.md` §P977.
fn ssty_level_of(style: &TextStyle) -> Option<u8> {
    if !style.math {
        return None;
    }
    match style.math_size {
        MathSize::Script => Some(1),
        MathSize::ScriptScript => Some(2),
        _ => None,
    }
}

/// **P977** — variante ssty (`.st`/`.sts`) de um glifo, lida da GSUB da
/// face (AlternateSubst da feature `ssty`: `alternates[level-1]`).
/// `None` se a face não tiver a feature/o glifo não for coberto — o
/// chamador fica com o glifo base nesse caso.
fn ssty_substitute(
    face: &ttf_parser::Face,
    gid: ttf_parser::GlyphId,
    level: u8,
) -> Option<ttf_parser::GlyphId> {
    let gsub = face.tables().gsub?;
    let feature = gsub.features.find(ttf_parser::Tag::from_bytes(b"ssty"))?;
    for lookup_index in feature.lookup_indices {
        let Some(lookup) = gsub.lookups.get(lookup_index) else { continue };
        for subtable in lookup
            .subtables
            .into_iter::<ttf_parser::gsub::SubstitutionSubtable>()
        {
            let ttf_parser::gsub::SubstitutionSubtable::Alternate(alt) = subtable
            else {
                continue;
            };
            let Some(cov_idx) = alt.coverage.get(gid) else { continue };
            let Some(set) = alt.alternate_sets.get(cov_idx) else { continue };
            if let Some(sub) = set.alternates.get(u16::from(level) - 1) {
                return Some(sub);
            }
        }
    }
    None
}

impl FontMetrics for FontBookMetrics<'_> {
    fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
        // Fórmula: advance_pt = font_size * (Σ glyph_units / upem)
        // **P977** — variante ssty por carácter (mesma regra de
        // `FallbackFontMetrics::advance`).
        let ssty_level = ssty_level_of(style);
        let mut units: f64 = text
            .chars()
            .map(|c| {
                self.face
                    .glyph_index(c)
                    .map(|gid| match ssty_level {
                        Some(level) => ssty_substitute(&self.face, gid, level).unwrap_or(gid),
                        None => gid,
                    })
                    .and_then(|gid| self.face.glyph_hor_advance(gid))
                    .map(|a| a as f64)
                    .unwrap_or(self.upem * 0.6) // fallback para glifos ausentes
            })
            .sum();
        // **P975** — termo de italics correction para glifo math singular
        // (mesma regra de `FallbackFontMetrics::advance`;
        // `infra/font_metrics.md` §P975). **P977** — lido do glifo já
        // substituído por ssty, se aplicável.
        if style.math && text.chars().count() == 1 {
            let c = text.chars().next().unwrap();
            if let Some(gid) = self.face.glyph_index(c) {
                let gid = match ssty_level_of(style) {
                    Some(level) => ssty_substitute(&self.face, gid, level).unwrap_or(gid),
                    None => gid,
                };
                if let Some(value) = self
                    .face
                    .tables()
                    .math
                    .and_then(|m| m.glyph_info)
                    .and_then(|gi| gi.italic_corrections)
                    .and_then(|ic| ic.get(gid))
                {
                    units += value.value as f64;
                }
            }
        }
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

    /// **P952b** — limites de tinta de um glifo por `glyph_id` nesta face
    /// (FontBookMetrics: face única fixa), da bounding box real.
    fn glyph_ink_bounds(&self, glyph_id: u16, size: Pt, _style: &TextStyle) -> (Pt, Pt) {
        let Some(bbox) = self.face.glyph_bounding_box(ttf_parser::GlyphId(glyph_id))
        else {
            return (self.cap_height(size, _style), Pt(0.0));
        };
        (
            Pt(size.val() * (bbox.y_max as f64 / self.upem)),
            Pt(size.val() * (-(bbox.y_min as f64)) / self.upem),
        )
    }

    /// **P971** — italics correction por `glyph_id`, lida de
    /// `MathItalicsCorrectionInfo` da face única (`infra/font_metrics.md`
    /// §P971). Sem tabela MATH ou sem entrada na coverage ⇒ `Pt(0.0)`.
    fn italics_correction(&self, glyph_id: u16, size: Pt, _style: &TextStyle) -> Pt {
        let Some(value) = self
            .face
            .tables()
            .math
            .and_then(|m| m.glyph_info)
            .and_then(|gi| gi.italic_corrections)
            .and_then(|ic| ic.get(ttf_parser::GlyphId(glyph_id)))
        else {
            return Pt(0.0);
        };
        Pt(size.val() * (value.value as f64 / self.upem))
    }

    /// **P988-B** — `TopAccentAttachment` da face única
    /// (`infra/font_metrics.md` §P988-B). Na cobertura ⇒ valor da tabela;
    /// fora ⇒ fallback do vanilla `(advance + IC)/2`
    /// (`fragment/glyph.rs:222-224`); char sem glifo na face ⇒ `None`.
    fn top_accent_attach(&self, c: char, size: Pt, _style: &TextStyle) -> Option<Pt> {
        let gid = self.face.glyph_index(c)?;
        let attach = self
            .face
            .tables()
            .math
            .and_then(|m| m.glyph_info)
            .and_then(|gi| gi.top_accent_attachments)
            .and_then(|ta| ta.get(gid))
            .map(|v| v.value as f64);
        let value = match attach {
            Some(v) => v,
            None => {
                let advance = self.face.glyph_hor_advance(gid)? as f64;
                let ic = self
                    .face
                    .tables()
                    .math
                    .and_then(|m| m.glyph_info)
                    .and_then(|gi| gi.italic_corrections)
                    .and_then(|ic| ic.get(gid))
                    .map(|v| v.value as f64)
                    .unwrap_or(0.0);
                (advance + ic) / 2.0
            }
        };
        Some(Pt(size.val() * (value / self.upem)))
    }

    /// **P922** — limites de tinta do texto com sinal: `(top, bottom)` em
    /// pontos. `top` = distância do topo da tinta à baseline (positivo para
    /// cima); `bottom` = distância do fundo da tinta à baseline (positivo
    /// para baixo). Para combining marks acima da baseline, `bottom` é
    /// negativo. Não força `max(0.0, ...)` — preserva o sinal.
    fn text_ink_bounds_signed(&self, text: &str, size: Pt, _style: &TextStyle) -> (Pt, Pt) {
        let mut top = f64::NEG_INFINITY;
        let mut bottom = f64::NEG_INFINITY;
        for c in text.chars() {
            let Some(gid) = self.face.glyph_index(c) else { continue };
            let Some(bbox) = self.face.glyph_bounding_box(gid) else { continue };
            top = top.max(size.val() * (bbox.y_max as f64 / self.upem));
            // **P989** — `bottom` é a distância do fundo da tinta à
            // baseline, POSITIVO PARA BAIXO (`−y_min·s`, convenção do trait
            // e do `descent()` do vanilla) — era `+y_min·s` (sinal
            // invertido para tinta que flutua acima da baseline, ex.
            // combining marks). Ver `infra/font_metrics.md` §P989.
            bottom = bottom.max(size.val() * (-(bbox.y_min as f64)) / self.upem);
        }
        if top == f64::NEG_INFINITY {
            top = size.val() * 0.7;
        }
        if bottom == f64::NEG_INFINITY {
            bottom = 0.0;
        }
        (Pt(top), Pt(bottom))
    }

    fn vertical_glyph_variants(&self, c: char, _style: &TextStyle) -> GlyphVariants {
        extract_variants(&self.face, c)
    }

    fn glyph_to_char(&self, glyph_id: u16) -> Option<char> {
        self.glyph_to_unicode.get(&glyph_id).copied()
    }

    fn vertical_glyph_assembly(&self, c: char, _style: &TextStyle) -> GlyphAssembly {
        extract_assembly(&self.face, c)
    }

    fn horizontal_glyph_variants(&self, c: char, _style: &TextStyle) -> GlyphVariants {
        extract_variants_horizontal(&self.face, c)
    }

    fn horizontal_glyph_assembly(&self, c: char, _style: &TextStyle) -> GlyphAssembly {
        extract_assembly_horizontal(&self.face, c)
    }

    fn math_constants(&self, _style: &TextStyle) -> MathConstants {
        math_constants_from_face(&self.face, self.upem)
    }

    fn math_kern(&self, c: char, _style: &TextStyle) -> MathGlyphKern {
        math_kern_from_face(&self.face, c)
    }
}

/// Lê as constantes da tabela MATH de `face`, ou `MathConstants::fallback()`
/// se a face não tiver tabela MATH/constants.
///
/// **P893** — extraída de `FontBookMetrics::math_constants` (mesmo padrão de
/// `math_kern_from_face`, P891) para ser partilhada com
/// `FallbackFontMetrics::math_constants`, que resolve a face candidata (via
/// `resolve_primary_with_math_fallback`, primeira com tabela MATH) antes de
/// chamar esta função, em vez de ter uma única face fixa.
fn math_constants_from_face(face: &Face<'_>, upem: f64) -> MathConstants {
    match face.tables().math {
        Some(math_table) => match math_table.constants {
            Some(c) => MathConstants {
                upem,
                fraction_rule_thickness: c.fraction_rule_thickness().value as f64,
                fraction_num_gap: c.fraction_numerator_gap_min().value as f64,
                fraction_denom_gap: c.fraction_denominator_gap_min().value as f64,
                // P920 — mesmo mecanismo dos outros campos; métodos já
                // existentes em ttf_parser 0.25 (confirmado antes de propor
                // o campo, `ttf-parser-0.25.1/src/tables/math.rs:378,390`).
                fraction_numerator_shift_up: c.fraction_numerator_shift_up().value as f64,
                fraction_denominator_shift_down: c.fraction_denominator_shift_down().value
                    as f64,
                // P990 — variantes Display (mesmo mecanismo; métodos de
                // ttf_parser 0.25, `tables/math.rs`).
                fraction_numerator_display_style_shift_up: c
                    .fraction_numerator_display_style_shift_up()
                    .value as f64,
                fraction_denominator_display_style_shift_down: c
                    .fraction_denominator_display_style_shift_down()
                    .value as f64,
                fraction_num_display_style_gap_min: c
                    .fraction_num_display_style_gap_min()
                    .value as f64,
                fraction_denom_display_style_gap_min: c
                    .fraction_denom_display_style_gap_min()
                    .value as f64,
                superscript_shift_up: c.superscript_shift_up().value as f64,
                // P915 — mesmo mecanismo dos outros 14 campos; método já
                // existia em ttf_parser 0.25, só nunca tinha sido lido.
                superscript_shift_up_cramped: c.superscript_shift_up_cramped().value as f64,
                subscript_shift_down: c.subscript_shift_down().value as f64,
                superscript_bottom_min: c.superscript_bottom_min().value as f64,
                superscript_bottom_max_with_subscript: c
                    .superscript_bottom_max_with_subscript()
                    .value as f64,
                superscript_baseline_drop_max: c.superscript_baseline_drop_max().value as f64,
                sub_superscript_gap_min: c.sub_superscript_gap_min().value as f64,
                subscript_top_max: c.subscript_top_max().value as f64,
                subscript_baseline_drop_min: c.subscript_baseline_drop_min().value as f64,
                radical_vertical_gap: c.radical_vertical_gap().value as f64,
                // P974 — MathValueRecord, mesmo padrão dos vizinhos
                // (`infra/font_metrics.md` §P974).
                radical_display_style_vertical_gap: c
                    .radical_display_style_vertical_gap()
                    .value as f64,
                radical_rule_thickness: c.radical_rule_thickness().value as f64,
                // P970 — os três primeiros são `MathValueRecord` (`.value`),
                // mesmo padrão dos vizinhos; o percentual é `i16` cru em
                // ttf-parser 0.25 (`radical_degree_bottom_raise_percent()`),
                // lido ÷100 como `script_percent_scale_down`.
                // (`infra/font_metrics.md` §P970.)
                radical_kern_before_degree: c.radical_kern_before_degree().value as f64,
                radical_kern_after_degree: c.radical_kern_after_degree().value as f64,
                radical_degree_bottom_raise_percent: c
                    .radical_degree_bottom_raise_percent()
                    as f64
                    / 100.0,
                radical_extra_ascender: c.radical_extra_ascender().value as f64,
                axis_height: c.axis_height().value as f64,
                script_percent_scale_down: c.script_percent_scale_down() as f64 / 100.0,
                script_script_percent_scale_down: c.script_script_percent_scale_down() as f64
                    / 100.0,
                upper_limit_gap_min: c.upper_limit_gap_min().value as f64,
                lower_limit_gap_min: c.lower_limit_gap_min().value as f64,
                // P959 — `MathValueRecord.value` (i16), mesmo padrão dos
                // vizinhos (`infra/font_metrics.md` §P959).
                upper_limit_baseline_rise_min: c.upper_limit_baseline_rise_min().value as f64,
                lower_limit_baseline_drop_min: c.lower_limit_baseline_drop_min().value as f64,
                math_leading: c.math_leading().value as f64,
                // P922 — métodos já existem em ttf_parser 0.25.
                accent_base_height: c.accent_base_height().value as f64,
                flattened_accent_base_height: c.flattened_accent_base_height().value as f64,
                // P952 — DisplayOperatorMinHeight. **Desvio do L0** (forma,
                // não substância): o L0 diz `c.display_operator_min_height()
                // .value`, mas em ttf_parser 0.25 este método devolve `u16`
                // directamente (não `MathLeadingValue` — é um dos poucos
                // campos u16 crus da tabela, `ttf-parser-0.25.1/src/tables/
                // math.rs:196`; o vanilla lê-o da mesma forma,
                // `typst-library/src/text/font/metrics.rs:228`).
                display_operator_min_height: c.display_operator_min_height() as f64,
            },
            None => MathConstants::fallback(),
        },
        None => MathConstants::fallback(),
    }
}

/// Lê o kern matemático por quadrante para `c` na tabela MATH de `face`.
///
/// **P891** — extraída de `FontBookMetrics::math_kern` para ser partilhada
/// com `FallbackFontMetrics::math_kern`, que resolve a face candidata (via
/// `resolve_primary_with_math_fallback` + `covering`) antes de chamar esta
/// função, em vez de ter uma única face fixa.
fn math_kern_from_face(face: &Face<'_>, c: char) -> MathGlyphKern {
    let Some(glyph_id) = face.glyph_index(c) else { return MathGlyphKern::default() };
    let Some(math) = face.tables().math else { return MathGlyphKern::default() };
    let Some(glyph_info) = math.glyph_info else { return MathGlyphKern::default() };
    let Some(kern_infos) = glyph_info.kern_infos else { return MathGlyphKern::default() };
    let Some(kern_record) = kern_infos.get(glyph_id) else { return MathGlyphKern::default() };

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
            records.push(MathKernRecord { correction_height: height, kern_value: kv });
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
    /// **P975** — o termo de italics correction depende de `style.math`;
    /// sem este campo, uma medição em prosa e outra em math com o mesmo
    /// texto/estilo colidiam na cache.
    math: bool,
    /// **P977** — o advance muda por nível MathSize (variante ssty).
    math_size: u8,
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
            math: style.math,
            math_size: style.math_size as u8,
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

    /// **P890** — como `resolve_primary`, mas injecta a cadeia de fallback
    /// matemático (`math_fallback_font_list()`) como primárias adicionais
    /// quando `style.math` é verdadeiro — mesma condição e cadeia que
    /// `shaper.rs::try_shape`/`shaped_width` (P783/P784) já aplicam antes de
    /// chamar `covering`/`covering_run`. Extraída para uso partilhado entre
    /// **todos** os métodos de `FontMetrics` que resolvem cobertura
    /// carácter-a-carácter (`advance`, `text_ink_bounds`) — antes de P890,
    /// só `text_ink_bounds` tinha esta injecção; `advance` chamava
    /// `resolve_primary` puro e, por correr primeiro em
    /// `layout_equation_measured`, era sempre quem disparava o scan caro de
    /// `World::candidates_for_char` para glifos matemáticos (letras
    /// itálicas de variável, letras gregas) só cobertos pela fonte MATH, não
    /// pela fonte de corpo — mesmo quando a fonte certa já está na cadeia
    /// dedicada, na primeira posição (`typst-passo-890-relatorio.md`).
    fn resolve_primary_with_math_fallback(
        &self,
        style: &TextStyle,
        variant: &FontVariant,
    ) -> Vec<FontCandidate> {
        let mut primary = self.resolve_primary(style);
        if style.math {
            for family in crate::fallback_fonts::math_fallback_font_list() {
                let pattern = FontNamePattern::Literal(ecow::EcoString::from(*family));
                let Some(idx) = self.world.book().select_pattern(&pattern, variant) else {
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
        // Primeiro passe (P912): dar prioridade a qualquer face em `primary` que
        // possua a tabela OpenType MATH e cubra `c`. Em contexto matemático, isto
        // garante que glifos comuns como `(`, `)`, `{` resolvam para a fonte MATH
        // (ex: New Computer Modern Math) em vez de pararem na primeira fonte de
        // corpo sem tabela MATH.
        for cand in primary {
            if let Some(cached) = self.cached_face(cand.slot_idx) {
                let face = cached.face();
                if face.tables().math.is_some() && face.glyph_index(c).is_some() {
                    return Some(*cand);
                }
            }
        }

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
        //
        // **P942** — a coverage exacta (P937/P938, runs de codepoints) já é
        // definitiva: o vanilla confia nela em `select_fallback` (book.rs:106-114)
        // e não re-verifica `glyph_index`. A versão anterior carregava a face de
        // cada candidata que cobria o caractere (dezenas de `.ttc` de CJK,
        // ~10-20 MB cada) só para re-verificar `glyph_index` — redundante e
        // medido como o custo dominante do fallback de layout (~480 ms em
        // `utf8-cjk`). Carrega-se agora só a face da fonte vencedora.
        for slot_idx in self.world.candidates_for_char(c) {
            if primary.iter().any(|cand| cand.slot_idx == slot_idx) {
                continue;
            }
            ids.push(slot_idx);
        }

        let best = book.select_fallback(like, variant, ids)?;
        let cached = self.cached_face(best)?;
        Some(FontCandidate {
            slot_idx: best,
            units_per_em: cached.face().units_per_em().max(1) as u16,
        })
    }

    /// **P906** — resolve o combo `(FontList, FontVariant, FontVariations)`
    /// que efectivamente cobre `c`, mesmo mecanismo de `covering` +
    /// `resolve_primary_with_math_fallback` (inclui a cadeia de fallback
    /// matemático quando `style.math`), mas devolve a IDENTIDADE da fonte
    /// (via `FontInfo` no `slot_idx` resolvido) em vez de dados de glifo.
    ///
    /// Usado por `pipeline::collect_fonts_in_items` para que `FrameItem::
    /// Glyph` (glifos de esticamento matemático, sem `style.font` já
    /// apontado à fonte certa como `FrameItem::TextShaped` tem após
    /// `shaper.rs`) entre correctamente na selecção Cidfont-vs-Multifont —
    /// achado: antes deste passo, `collect_fonts_in_items` ignorava
    /// `FrameItem::Glyph` por completo (braço `FrameItem::Glyph { .. } =>
    /// {}`), fazendo equações cujo ÚNICO conteúdo a precisar da fonte MATH
    /// fosse um glifo de esticamento (`underbracket`/`underbrace`/etc. sem
    /// texto itálico à volta) escolherem só a fonte de corpo como candidata
    /// Cidfont única — os glyph_ids de esticamento, correctos na fonte MATH,
    /// caíam fora do subset embutido (glifo `.notdef`). Ver `pipeline.md`
    /// §P906.
    pub(crate) fn resolve_font_combo(
        &self,
        c: char,
        style: &TextStyle,
    ) -> Option<(FontList, FontVariant, FontVariations)> {
        let variant = text_style_to_font_variant(style);
        let primary = self.resolve_primary_with_math_fallback(style, &variant);
        let cand = self.covering(c, &primary, &variant)?;
        let info = self.world.book().infos().get(cand.slot_idx)?;
        Some((
            FontList::single(ecow::EcoString::from(info.family.as_str())),
            info.variant,
            FontVariations::default(),
        ))
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
            let variant = text_style_to_font_variant(style);
            // P890 — injecção de fallback matemático (P784) via helper
            // partilhado com `text_ink_bounds`. Antes deste passo, `advance`
            // chamava `resolve_primary` puro (sem a cadeia math) e, por
            // correr primeiro em `layout_equation_measured`
            // (`math/layout/mod.rs`), era sempre quem primeiro tentava
            // cobrir um glifo matemático (letra itálica de variável, letra
            // grega) — sem `New Computer Modern Math` em `primary`, caía no
            // scan caro de `World::candidates_for_char` (computa cobertura
            // de TODO o FontBook na primeira chamada) mesmo quando a fonte
            // certa já estava na cadeia dedicada, só não tinha sido
            // consultada (`typst-passo-890-relatorio.md`).
            let primary = self.resolve_primary_with_math_fallback(style, &variant);
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
            let axis_vars = axis_variations_for_text_style(style);
            // **P977** — variante ssty para glifos math de script.
            let ssty_level = ssty_level_of(style);
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
                        let mut g = face.glyph_index(c)?;
                        if let Some(level) = ssty_level {
                            if let Some(sub) = ssty_substitute(&face, g, level) {
                                g = sub;
                            }
                        }
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

            // **P975** — vanilla `update_glyph`
            // (`lab/typst-original/crates/typst-layout/src/math/fragment/
            // glyph.rs:204-211`): para glifos math singulares não-esticados,
            // `x_advance += italics_correction`. Só `style.math` e só 1
            // carácter (texto math multi-carácter é run, sem o termo).
            // `infra/font_metrics.md` §P975.
            if style.math && text.chars().count() == 1 {
                if let Some((slot_idx, gid)) = prev {
                    if let Some(cached) = self.cached_face(slot_idx) {
                        let face = cached.face();
                        if let Some(value) = face
                            .tables()
                            .math
                            .and_then(|m| m.glyph_info)
                            .and_then(|gi| gi.italic_corrections)
                            .and_then(|ic| ic.get(ttf_parser::GlyphId(gid)))
                        {
                            let upem = face.units_per_em().max(1) as f64;
                            total += value.value as f64 * size.val() / upem;
                        }
                    }
                }
            }

            Pt(total)
        })
    }

    /// **P591** — para scripts contextuais (árabe, síriaco, etc.), usa o
    /// shaper para obter a largura real com formas ligadas. Para os restantes
    /// scripts, mantém o caminho rápido `advance`.
    fn advance_shaped(&self, text: &str, _size: Pt, style: &TextStyle) -> Option<Pt> {
        use typst_core::compiler::layout::needs_shaped_width;
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
        let variant = text_style_to_font_variant(style);
        // P890 — injecção de fallback matemático (P784) via helper
        // partilhado com `advance` (era duplicada aqui antes deste passo).
        let primary = self.resolve_primary_with_math_fallback(style, &variant);
        let mut ascent = 0.0_f64;
        let mut descent = 0.0_f64;
        for c in text.chars() {
            let Some(cand) = self.covering(c, &primary, &variant) else { continue };
            let Some(cached) = self.cached_face(cand.slot_idx) else { continue };
            let face = cached.face();
            let Some(mut gid) = face.glyph_index(c) else { continue };
            // **P977** — variante ssty (a tinta medida é a do glifo que
            // vai ser desenhado).
            if let Some(level) = ssty_level_of(style) {
                if let Some(sub) = ssty_substitute(face, gid, level) {
                    gid = sub;
                }
            }
            let Some(bbox) = face.glyph_bounding_box(gid) else { continue };
            let upem = cand.units_per_em as f64;
            ascent = ascent.max(size.val() * (bbox.y_max as f64 / upem));
            descent = descent.max(size.val() * (-(bbox.y_min as f64)) / upem);
        }
        (Pt(ascent), Pt(descent))
    }

    /// **P952b** — limites de tinta de um glifo por `glyph_id`, resolvendo a
    /// face MATH activa da cadeia de fallback (a mesma que fornece as
    /// variantes — `math_constants`, P893), e lendo a bounding box real.
    fn glyph_ink_bounds(&self, glyph_id: u16, size: Pt, style: &TextStyle) -> (Pt, Pt) {
        let variant = text_style_to_font_variant(style);
        let primary = self.resolve_primary_with_math_fallback(style, &variant);
        for cand in &primary {
            let Some(cached) = self.cached_face(cand.slot_idx) else { continue };
            let face = cached.face();
            if face.tables().math.is_none() {
                continue;
            }
            let Some(bbox) = face.glyph_bounding_box(ttf_parser::GlyphId(glyph_id))
            else {
                continue;
            };
            let upem = cand.units_per_em as f64;
            return (
                Pt(size.val() * (bbox.y_max as f64 / upem)),
                Pt(size.val() * (-(bbox.y_min as f64)) / upem),
            );
        }
        (self.cap_height(size, style), Pt(0.0))
    }

    /// **P971** — italics correction por `glyph_id`, resolvendo a face MATH
    /// activa da cadeia de fallback (a mesma que fornece as variantes —
    /// P893). Sem entrada na coverage ⇒ `Pt(0.0)` (default do trait).
    fn italics_correction(&self, glyph_id: u16, size: Pt, style: &TextStyle) -> Pt {
        let variant = text_style_to_font_variant(style);
        let primary = self.resolve_primary_with_math_fallback(style, &variant);
        for cand in &primary {
            let Some(cached) = self.cached_face(cand.slot_idx) else { continue };
            let face = cached.face();
            let Some(value) = face
                .tables()
                .math
                .and_then(|m| m.glyph_info)
                .and_then(|gi| gi.italic_corrections)
                .and_then(|ic| ic.get(ttf_parser::GlyphId(glyph_id)))
            else {
                continue;
            };
            let upem = cand.units_per_em as f64;
            return Pt(size.val() * (value.value as f64 / upem));
        }
        Pt(0.0)
    }

    /// **P988-B** — `TopAccentAttachment` com a mesma resolução de face de
    /// `text_ink_bounds` (`resolve_primary_with_math_fallback` +
    /// `covering`). Na cobertura ⇒ tabela; fora ⇒ fallback do vanilla
    /// `(advance + IC)/2`; char sem glifo em nenhuma face ⇒ `None`.
    /// **P977** — variante ssty aplicada como em `text_ink_bounds`.
    fn top_accent_attach(&self, c: char, size: Pt, style: &TextStyle) -> Option<Pt> {
        let variant = text_style_to_font_variant(style);
        let primary = self.resolve_primary_with_math_fallback(style, &variant);
        let cand = self.covering(c, &primary, &variant)?;
        let cached = self.cached_face(cand.slot_idx)?;
        let face = cached.face();
        let mut gid = face.glyph_index(c)?;
        if let Some(level) = ssty_level_of(style) {
            if let Some(sub) = ssty_substitute(face, gid, level) {
                gid = sub;
            }
        }
        let upem = cand.units_per_em as f64;
        let attach = face
            .tables()
            .math
            .and_then(|m| m.glyph_info)
            .and_then(|gi| gi.top_accent_attachments)
            .and_then(|ta| ta.get(gid))
            .map(|v| v.value as f64);
        let value = match attach {
            Some(v) => v,
            None => {
                let advance = face.glyph_hor_advance(gid)? as f64;
                let ic = face
                    .tables()
                    .math
                    .and_then(|m| m.glyph_info)
                    .and_then(|gi| gi.italic_corrections)
                    .and_then(|ic| ic.get(gid))
                    .map(|v| v.value as f64)
                    .unwrap_or(0.0);
                (advance + ic) / 2.0
            }
        };
        Some(Pt(size.val() * (value / upem)))
    }

    /// **P922** — limites de tinta com sinal, com a mesma resolução de face
    /// (`resolve_primary_with_math_fallback` + `covering`) de `text_ink_bounds`.
    /// Devolve `(top, bottom)` em pontos, sem forçar `max(0.0, ...)`.
    fn text_ink_bounds_signed(&self, text: &str, size: Pt, style: &TextStyle) -> (Pt, Pt) {
        let variant = text_style_to_font_variant(style);
        let primary = self.resolve_primary_with_math_fallback(style, &variant);
        let mut top = f64::NEG_INFINITY;
        let mut bottom = f64::NEG_INFINITY;
        for c in text.chars() {
            let Some(cand) = self.covering(c, &primary, &variant) else { continue };
            let Some(cached) = self.cached_face(cand.slot_idx) else { continue };
            let face = cached.face();
            let Some(mut gid) = face.glyph_index(c) else { continue };
            // **P977** — variante ssty (ver `text_ink_bounds`).
            if let Some(level) = ssty_level_of(style) {
                if let Some(sub) = ssty_substitute(face, gid, level) {
                    gid = sub;
                }
            }
            let Some(bbox) = face.glyph_bounding_box(gid) else { continue };
            let upem = cand.units_per_em as f64;
            top = top.max(size.val() * (bbox.y_max as f64 / upem));
            // **P989** — `bottom` positivo para baixo (`−y_min/upem`),
            // convenção do trait — era `+y_min` (sinal invertido). Ver
            // `infra/font_metrics.md` §P989.
            bottom = bottom.max(size.val() * (-(bbox.y_min as f64)) / upem);
        }
        if top == f64::NEG_INFINITY {
            top = self.cap_height(size, style).val();
        }
        if bottom == f64::NEG_INFINITY {
            bottom = 0.0;
        }
        (Pt(top), Pt(bottom))
    }

    /// **P891** — resolve a face que cobre `c` (mesmo mecanismo de
    /// `text_ink_bounds`: `resolve_primary_with_math_fallback` + `covering`)
    /// e lê o kern matemático real dessa face via `math_kern_from_face`.
    /// Antes deste passo, `FallbackFontMetrics` não sobrepunha este método —
    /// herdava o default do trait (kern zero incondicional), causa
    /// confirmada do gap indevido antes de expoentes (`i^2` → `i  ²`,
    /// achado de P889/P891).
    /// **P893** — `math_constants` não recebe `char` (propriedade por-fonte,
    /// não por-glifo, ao contrário de `math_kern` acima) — em vez de
    /// `covering(c, ...)`, resolve a PRIMEIRA face de `primary` com tabela
    /// MATH presente (`face.tables().math.is_some()`). `primary.first()`
    /// sozinho seria errado: no caso comum (sem `#set text(font:)`
    /// explícito), a primeira candidata é a fonte de corpo genérica
    /// (`Libertinus Serif`, sem tabela MATH) — só a partir daí é que a
    /// cadeia `math_fallback_font_list()` (P890) acrescenta `New Computer
    /// Modern Math`. Sem candidato com tabela MATH: `MathConstants::
    /// fallback()` (mesmo comportamento de antes deste passo). Antes deste
    /// passo, `FallbackFontMetrics` — a ÚNICA implementação usada no
    /// pipeline real — não sobrepunha este método: herdava o default do
    /// trait (`MathConstants::fallback()`, valores de STIX Two Math)
    /// incondicionalmente, mesmo com uma fonte MATH real disponível —
    /// achado de P893, medido contra `NewCMMath-Regular.otf`:
    /// `axis_height` errado por 2× (500 vs 250 real), `subscript_shift_down`
    /// +90% (130 vs 247), `upper_limit_gap_min` +100% (100 vs 200). Ver
    /// `infra/font_metrics.md` §P893.
    fn math_constants(&self, style: &TextStyle) -> MathConstants {
        let variant = text_style_to_font_variant(style);
        let primary = self.resolve_primary_with_math_fallback(style, &variant);
        for cand in &primary {
            let Some(cached) = self.cached_face(cand.slot_idx) else { continue };
            let face = cached.face();
            if face.tables().math.and_then(|m| m.constants).is_some() {
                return math_constants_from_face(face, cand.units_per_em as f64);
            }
        }
        MathConstants::fallback()
    }

    fn math_kern(&self, c: char, style: &TextStyle) -> MathGlyphKern {
        let variant = text_style_to_font_variant(style);
        let primary = self.resolve_primary_with_math_fallback(style, &variant);
        let Some(cand) = self.covering(c, &primary, &variant) else {
            return MathGlyphKern::default();
        };
        let Some(cached) = self.cached_face(cand.slot_idx) else {
            return MathGlyphKern::default();
        };
        math_kern_from_face(cached.face(), c)
    }

    /// **P906** — mesmo mecanismo de `math_kern` acima (resolve a face que
    /// cobre `c` via `resolve_primary_with_math_fallback` + `covering`, lê a
    /// tabela MATH dessa face). Antes deste passo, `FallbackFontMetrics` não
    /// sobrepunha `vertical_glyph_variants`/`vertical_glyph_assembly` —
    /// herdava o default do trait (vazio incondicional), gap já confirmado e
    /// deliberadamente adiado em P891/P893 (`infra/font_metrics.md`, nota
    /// "fora de âmbito"). Corrigido agora porque bloqueava a confirmação
    /// visual do próprio P906: como `FallbackFontMetrics` é a ÚNICA
    /// implementação usada no pipeline real (`03_infra/src/pipeline.rs:126`),
    /// TODO o esticamento de glifo (parênteses, chaves, sqrt — não só o
    /// mecanismo horizontal novo) estava silenciosamente inactivo em PDFs
    /// reais, apesar de os testes unitários (que instanciam `FontBookMetrics`
    /// ou stubs directamente) sempre terem passado. Ver
    /// `infra/font_metrics.md` §P906.
    fn vertical_glyph_variants(&self, c: char, style: &TextStyle) -> GlyphVariants {
        let variant = text_style_to_font_variant(style);
        let primary = self.resolve_primary_with_math_fallback(style, &variant);
        let Some(cand) = self.covering(c, &primary, &variant) else {
            return GlyphVariants::default();
        };
        let Some(cached) = self.cached_face(cand.slot_idx) else {
            return GlyphVariants::default();
        };
        extract_variants(cached.face(), c)
    }

    /// **P906** — ver `vertical_glyph_variants` acima.
    fn vertical_glyph_assembly(&self, c: char, style: &TextStyle) -> GlyphAssembly {
        let variant = text_style_to_font_variant(style);
        let primary = self.resolve_primary_with_math_fallback(style, &variant);
        let Some(cand) = self.covering(c, &primary, &variant) else {
            return GlyphAssembly::default();
        };
        let Some(cached) = self.cached_face(cand.slot_idx) else {
            return GlyphAssembly::default();
        };
        extract_assembly(cached.face(), c)
    }

    /// **P906** — variantes horizontais, mesmo mecanismo. Ver
    /// `infra/font_metrics.md` §P906.
    fn horizontal_glyph_variants(&self, c: char, style: &TextStyle) -> GlyphVariants {
        let variant = text_style_to_font_variant(style);
        let primary = self.resolve_primary_with_math_fallback(style, &variant);
        let Some(cand) = self.covering(c, &primary, &variant) else {
            return GlyphVariants::default();
        };
        let Some(cached) = self.cached_face(cand.slot_idx) else {
            return GlyphVariants::default();
        };
        extract_variants_horizontal(cached.face(), c)
    }

    /// **P906** — assembly horizontal, mesmo mecanismo.
    fn horizontal_glyph_assembly(&self, c: char, style: &TextStyle) -> GlyphAssembly {
        let variant = text_style_to_font_variant(style);
        let primary = self.resolve_primary_with_math_fallback(style, &variant);
        let Some(cand) = self.covering(c, &primary, &variant) else {
            return GlyphAssembly::default();
        };
        let Some(cached) = self.cached_face(cand.slot_idx) else {
            return GlyphAssembly::default();
        };
        extract_assembly_horizontal(cached.face(), c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **P891** — `FallbackFontMetrics::math_kern` deve ler a tabela MATH
    /// real da face que cobre `c` (via `resolve_primary_with_math_fallback`
    /// + `covering`), não devolver sempre o default do trait (kern zero
    /// incondicional — o bug confirmado em `typst-passo-891-relatorio.md`
    /// para `i^2`, onde a base `𝑖` nunca aproximava o expoente).
    ///
    /// Usa a fonte `NewCMMath` real embutida via `typst_assets::fonts()`
    /// (a mesma usada em produção pela cadeia `DEFAULT_FALLBACK_FONTS_MATH`)
    /// — localizada por cobertura (tem tabela MATH + kern não-trivial para
    /// `𝐷`, U+1D437), não por nome (as 3 variantes embutidas Regular/Bold/
    /// Book não expõem nome PostScript decodificável nas plataformas que
    /// `ttf_parser` lê por omissão). O valor esperado é calculado
    /// directamente via `ttf_parser` no próprio teste — não hardcoded —
    /// para não ficar frágil a uma actualização do asset.
    #[test]
    fn p891_fallback_font_metrics_math_kern_le_tabela_math_real() {
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

        let c = '\u{1D437}'; // 𝐷 — D itálico matemático (mesma classe de 𝑖).

        let math_font_data = typst_assets::fonts()
            .find(|data| {
                let Ok(face) = ttf_parser::Face::parse(data, 0) else { return false };
                let Some(gid) = face.glyph_index(c) else { return false };
                let Some(math) = face.tables().math else { return false };
                let Some(gi) = math.glyph_info else { return false };
                let Some(kerns) = gi.kern_infos else { return false };
                let Some(rec) = kerns.get(gid) else { return false };
                rec.bottom_right
                    .as_ref()
                    .and_then(|k| k.kern(k.count()))
                    .is_some_and(|v| v.value != 0)
            })
            .expect("fonte embutida NewCMMath com kern não-zero para U+1D437 tem de existir");

        // Ground truth via ttf_parser directo, independente da implementação
        // sob teste.
        let expected_bottom_right = {
            let face = ttf_parser::Face::parse(math_font_data, 0).unwrap();
            let gid = face.glyph_index(c).unwrap();
            let rec = face
                .tables()
                .math
                .unwrap()
                .glyph_info
                .unwrap()
                .kern_infos
                .unwrap()
                .get(gid)
                .unwrap();
            rec.bottom_right
                .as_ref()
                .and_then(|k| k.kern(k.count()))
                .map(|v| v.value as f64)
                .unwrap()
        };
        assert_ne!(expected_bottom_right, 0.0);

        struct StubWorld {
            library: Library,
            book: FontBook,
            fonts: Vec<Option<Font>>,
        }
        impl World for StubWorld {
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
                self.fonts.get(idx).cloned().flatten()
            }
            fn today(&self, _: Option<i64>) -> Option<Datetime> {
                None
            }
            fn candidates_for_char(&self, _: char) -> Vec<usize> {
                vec![]
            }
        }

        let mut book = FontBook::new();
        book.push(FontInfo {
            family: "New Computer Modern Math".into(),
            variant: FontVariant {
                style: FontStyle::Normal,
                weight: FontWeight::REGULAR,
                stretch: FontStretch::NORMAL,
            },
            flags: FontFlags::default(),
            coverage: Coverage::default(),
        });
        let world = StubWorld {
            library: Library::new(),
            book,
            fonts: vec![Some(Font::from_data(math_font_data.to_vec()))],
        };

        let metrics = FallbackFontMetrics::new(&world);
        let mut style = TextStyle::default();
        style.math = true; // engata DEFAULT_FALLBACK_FONTS_MATH (P890).

        let kern = metrics.math_kern(c, &style);
        assert_eq!(
            kern.bottom_right.records.last().map(|r| r.kern_value),
            Some(expected_bottom_right),
            "bottom_right kern de 'D' deve bater com a tabela MATH real da fonte"
        );
    }

    /// **P912/P917** — `FallbackFontMetrics::vertical_glyph_variants` deve
    /// ler a tabela MATH real da face que cobre `c` (mesmo mecanismo de
    /// `math_kern`, P891) — gap de cobertura explicitamente pedido pelo L0
    /// de P912 ("testes com métricas reais, não `FixedMetrics`") mas nunca
    /// entregue: confirmado ausente durante o diagnóstico de P917 (só um
    /// teste de fonte real existia em todo o crate, `p891_...math_kern...`
    /// acima). Sem este teste, o desvio `advance` (eixo de esticamento) vs
    /// `hor_advance` (avanço nativo) encontrado por P917 teria continuado
    /// indetectável pela suíte automática — só foi encontrado por
    /// instrumentação manual (`eprintln!`, revertida). Ground truth
    /// calculado directamente via `ttf_parser` no próprio teste, não
    /// hardcoded (mesma disciplina de P891).
    #[test]
    fn p912_fallback_font_metrics_vertical_glyph_variants_le_tabela_math_real() {
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

        let c = '(';

        let math_font_data = typst_assets::fonts()
            .find(|data| {
                let Ok(face) = ttf_parser::Face::parse(data, 0) else { return false };
                let Some(gid) = face.glyph_index(c) else { return false };
                let Some(math) = face.tables().math else { return false };
                let Some(variants) = math.variants else { return false };
                variants
                    .vertical_constructions
                    .get(gid)
                    .is_some_and(|c| !c.variants.is_empty())
            })
            .expect("fonte embutida com variantes verticais de '(' tem de existir");

        // Ground truth via ttf_parser directo, independente da implementação
        // sob teste — inclui `hor_advance` (hmtx), o campo que P917 provou
        // estar a ser confundido com `advance` (eixo de esticamento).
        let face = ttf_parser::Face::parse(math_font_data, 0).unwrap();
        let gid = face.glyph_index(c).unwrap();
        let construction =
            face.tables().math.unwrap().variants.unwrap().vertical_constructions.get(gid).unwrap();
        let expected: Vec<(u16, f64, f64)> = construction
            .variants
            .into_iter()
            .map(|v| {
                let hor = face.glyph_hor_advance(v.variant_glyph).unwrap_or(0) as f64;
                (v.variant_glyph.0, v.advance_measurement as f64, hor)
            })
            .collect();
        assert!(!expected.is_empty());
        // P917 — a fixture só serve de prova da distinção advance/hor_advance
        // se pelo menos uma variante divergir claramente entre os dois eixos.
        assert!(
            expected.iter().any(|(_, adv, hor)| (adv - hor).abs() > adv * 0.1),
            "fixture não prova a distinção P917: precisa de pelo menos uma variante onde \
             advance e hor_advance divirjam claramente; expected={:?}",
            expected
        );

        struct StubWorld {
            library: Library,
            book: FontBook,
            fonts: Vec<Option<Font>>,
        }
        impl World for StubWorld {
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
                self.fonts.get(idx).cloned().flatten()
            }
            fn today(&self, _: Option<i64>) -> Option<Datetime> {
                None
            }
            fn candidates_for_char(&self, _: char) -> Vec<usize> {
                vec![]
            }
        }

        let mut book = FontBook::new();
        book.push(FontInfo {
            family: "New Computer Modern Math".into(),
            variant: FontVariant {
                style: FontStyle::Normal,
                weight: FontWeight::REGULAR,
                stretch: FontStretch::NORMAL,
            },
            flags: FontFlags::default(),
            coverage: Coverage::default(),
        });
        let world = StubWorld {
            library: Library::new(),
            book,
            fonts: vec![Some(Font::from_data(math_font_data.to_vec()))],
        };

        let metrics = FallbackFontMetrics::new(&world);
        let mut style = TextStyle::default();
        style.math = true; // engata DEFAULT_FALLBACK_FONTS_MATH (P890).

        let variants = metrics.vertical_glyph_variants(c, &style);
        let actual: Vec<(u16, f64, f64)> =
            variants.variants.iter().map(|v| (v.glyph_id, v.advance, v.hor_advance)).collect();
        assert_eq!(
            actual, expected,
            "variantes verticais de '(' devem bater byte-a-byte com a tabela MATH real \
             (glyph_id, advance E hor_advance)"
        );
    }

    /// **P913/P917** — mesma disciplina do teste acima
    /// (`p912_..._vertical_glyph_variants_...`), para
    /// `vertical_glyph_assembly`: gap de cobertura de fonte real nunca
    /// fechado para o mecanismo de montagem por partes, confirmado ausente
    /// durante P917. Prova que `hor_advance` de cada `GlyphPart` (usado
    /// para `x_advance`/largura em `layout_assembly`, P917) bate com o
    /// avanço nativo real do glifo da peça, não com `full_advance` (eixo de
    /// empilhamento).
    #[test]
    fn p913_fallback_font_metrics_vertical_glyph_assembly_le_tabela_math_real() {
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

        let c = '(';

        let math_font_data = typst_assets::fonts()
            .find(|data| {
                let Ok(face) = ttf_parser::Face::parse(data, 0) else { return false };
                let Some(gid) = face.glyph_index(c) else { return false };
                let Some(math) = face.tables().math else { return false };
                let Some(variants) = math.variants else { return false };
                variants
                    .vertical_constructions
                    .get(gid)
                    .and_then(|c| c.assembly)
                    .is_some_and(|a| !a.parts.is_empty())
            })
            .expect("fonte embutida com assembly vertical de '(' tem de existir");

        let face = ttf_parser::Face::parse(math_font_data, 0).unwrap();
        let gid = face.glyph_index(c).unwrap();
        let ttf_assembly = face
            .tables()
            .math
            .unwrap()
            .variants
            .unwrap()
            .vertical_constructions
            .get(gid)
            .unwrap()
            .assembly
            .unwrap();
        let expected: Vec<(u16, u16, u16, u16, bool, f64)> = ttf_assembly
            .parts
            .into_iter()
            .map(|p| {
                let hor = face.glyph_hor_advance(p.glyph_id).unwrap_or(0) as f64;
                (
                    p.glyph_id.0,
                    p.start_connector_length,
                    p.end_connector_length,
                    p.full_advance,
                    p.part_flags.extender(),
                    hor,
                )
            })
            .collect();
        assert!(!expected.is_empty());
        assert!(
            expected.iter().any(|(_, _, _, full, _, hor)| (*full as f64 - hor).abs() > 1.0),
            "fixture não prova a distinção P917 para assembly: precisa de pelo menos uma peça \
             onde full_advance e hor_advance divirjam; expected={:?}",
            expected
        );

        struct StubWorld {
            library: Library,
            book: FontBook,
            fonts: Vec<Option<Font>>,
        }
        impl World for StubWorld {
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
                self.fonts.get(idx).cloned().flatten()
            }
            fn today(&self, _: Option<i64>) -> Option<Datetime> {
                None
            }
            fn candidates_for_char(&self, _: char) -> Vec<usize> {
                vec![]
            }
        }

        let mut book = FontBook::new();
        book.push(FontInfo {
            family: "New Computer Modern Math".into(),
            variant: FontVariant {
                style: FontStyle::Normal,
                weight: FontWeight::REGULAR,
                stretch: FontStretch::NORMAL,
            },
            flags: FontFlags::default(),
            coverage: Coverage::default(),
        });
        let world = StubWorld {
            library: Library::new(),
            book,
            fonts: vec![Some(Font::from_data(math_font_data.to_vec()))],
        };

        let metrics = FallbackFontMetrics::new(&world);
        let mut style = TextStyle::default();
        style.math = true;

        let assembly = metrics.vertical_glyph_assembly(c, &style);
        let actual: Vec<(u16, u16, u16, u16, bool, f64)> = assembly
            .parts
            .iter()
            .map(|p| {
                (
                    p.glyph_id,
                    p.start_connector,
                    p.end_connector,
                    p.full_advance,
                    p.is_extender,
                    p.hor_advance,
                )
            })
            .collect();
        assert_eq!(
            actual, expected,
            "peças do assembly vertical de '(' devem bater byte-a-byte com a tabela MATH real"
        );
    }

    /// **P914/P917** — `FallbackFontMetrics::math_constants` deve ler as
    /// constantes reais da tabela MATH (usadas por `compute_script_shifts`/
    /// `attach.rs` — `superscript_shift_up`, `subscript_shift_down`, etc.)
    /// — gap de cobertura de fonte real nunca fechado para este método,
    /// confirmado ausente durante P917 (nenhum teste, em todo o crate,
    /// chamava `math_constants()` numa face real antes deste). P916 já
    /// tinha revisto `attach.rs` cepticamente sem achados (lógica correcta
    /// para o que `StubHorizontalMetrics` consegue exercitar) — este teste
    /// fecha a lacuna que essa revisão não cobria: se os valores chegam
    /// certos a partir da fonte real, não só se a fórmula está certa dado
    /// um valor sintético qualquer.
    #[test]
    fn p914_fallback_font_metrics_math_constants_le_tabela_math_real() {
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

        let math_font_data = typst_assets::fonts()
            .find(|data| {
                let Ok(face) = ttf_parser::Face::parse(data, 0) else { return false };
                face.tables().math.and_then(|m| m.constants).is_some()
            })
            .expect("fonte embutida com tabela MATH constants tem de existir");

        let face = ttf_parser::Face::parse(math_font_data, 0).unwrap();
        let ttf_constants = face.tables().math.unwrap().constants.unwrap();
        let expected_sup_shift_up = ttf_constants.superscript_shift_up().value as f64;
        let expected_sup_shift_up_cramped = ttf_constants.superscript_shift_up_cramped().value as f64;
        let expected_sub_shift_down = ttf_constants.subscript_shift_down().value as f64;
        let expected_axis_height = ttf_constants.axis_height().value as f64;
        assert_ne!(expected_sup_shift_up, 0.0);
        assert_ne!(expected_sub_shift_down, 0.0);

        struct StubWorld {
            library: Library,
            book: FontBook,
            fonts: Vec<Option<Font>>,
        }
        impl World for StubWorld {
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
                self.fonts.get(idx).cloned().flatten()
            }
            fn today(&self, _: Option<i64>) -> Option<Datetime> {
                None
            }
            fn candidates_for_char(&self, _: char) -> Vec<usize> {
                vec![]
            }
        }

        let mut book = FontBook::new();
        book.push(FontInfo {
            family: "New Computer Modern Math".into(),
            variant: FontVariant {
                style: FontStyle::Normal,
                weight: FontWeight::REGULAR,
                stretch: FontStretch::NORMAL,
            },
            flags: FontFlags::default(),
            coverage: Coverage::default(),
        });
        let world = StubWorld {
            library: Library::new(),
            book,
            fonts: vec![Some(Font::from_data(math_font_data.to_vec()))],
        };

        let metrics = FallbackFontMetrics::new(&world);
        let mut style = TextStyle::default();
        style.math = true;
        let variant = text_style_to_font_variant(&style);
        let primary = metrics.resolve_primary_with_math_fallback(&style, &variant);
        let cand = metrics
            .covering('(', &primary, &variant)
            .expect("covering deve resolver a face MATH para '(' em contexto matemático");
        let cached = metrics.cached_face(cand.slot_idx).expect("face resolvida deve estar em cache");
        let constants = cached.face().tables().math.unwrap().constants;
        assert!(constants.is_some(), "face resolvida por covering() deve ter tabela MATH constants");

        // Chama o método sob teste via o mesmo mecanismo que `attach.rs`
        // usa (`self.metrics.math_constants(style)` em `MathLayouter::new`,
        // P893 — sem `char`, só `style`).
        let actual = metrics.math_constants(&style);
        assert_eq!(
            actual.superscript_shift_up, expected_sup_shift_up,
            "superscript_shift_up deve bater com a tabela MATH real"
        );
        // **P915** — mesmo teste, campo novo: fecha o gap de cobertura de
        // fonte real para superscript_shift_up_cramped, confirmado ausente
        // durante o diagnóstico de P915 (nenhum teste, em todo o crate,
        // lia este campo específico antes deste).
        assert_eq!(
            actual.superscript_shift_up_cramped, expected_sup_shift_up_cramped,
            "superscript_shift_up_cramped deve bater com a tabela MATH real"
        );
        assert_eq!(
            actual.subscript_shift_down, expected_sub_shift_down,
            "subscript_shift_down deve bater com a tabela MATH real"
        );
        assert_eq!(
            actual.axis_height, expected_axis_height,
            "axis_height deve bater com a tabela MATH real"
        );
    }

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

    /// **P989** — `text_ink_bounds_signed` com a convenção do trait:
    /// `bottom` POSITIVO PARA BAIXO (`−y_min·s`). O combining dot (U+0307)
    /// tem a tinta toda ACIMA da baseline (NewCMMath-Book: bbox y
    /// 571..677du) → `bottom` NEGATIVO. Antes de P989 a L3 devolvia o
    /// `y_min` cru (+6.28pt) — sinal invertido (a posição do acento
    /// cancelava o termo, logo só se manifestou em acento aninhado,
    /// `accent.md` §P989).
    #[test]
    fn p989_text_ink_bounds_signed_bottom_negativo_para_tinta_flutuante() {
        let data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/NewCMMath-Book.otf"
        ))
        .expect("fixture NewCMMath-Book.otf necessária");
        let m = FontBookMetrics::from_bytes(&data).expect("fonte válida");
        let style = TextStyle::default();
        let size = Pt(11.0);

        let (top, bottom) = m.text_ink_bounds_signed("\u{0307}", size, &style);
        // 677du/1000 × 11pt ≈ 7.45pt acima; 571du → bottom ≈ −6.28pt.
        assert!(
            (top.val() - 7.447).abs() < 0.05,
            "topo da tinta do ponto combinante ≈ 7.45pt: {:.4}",
            top.val()
        );
        assert!(
            (bottom.val() - (-6.281)).abs() < 0.05,
            "bottom com sinal deve ser NEGATIVO para tinta flutuante (−6.28pt): {:.4}",
            bottom.val()
        );

        // Guarda da convenção oposta: 'g' (descender) → bottom POSITIVO.
        let (_, bottom_g) = m.text_ink_bounds_signed("g", size, &style);
        assert!(
            bottom_g.val() > 0.0,
            "bottom de 'g' (tinta abaixo da baseline) deve ser positivo: {:.4}",
            bottom_g.val()
        );
    }

    /// **P988-B** — `top_accent_attach` com a fonte real: x itálico
    /// matemático (U+1D44E) = 287du e hat combinante (U+0302) = 250du da
    /// tabela `MathTopAccentAttachment` de NewCMMath-Book (fontTools);
    /// um char fora da cobertura cai no fallback `(advance + IC)/2` do
    /// vanilla. Gate ADR-0127 aprovado pelo dono em 2026-08-06.
    #[test]
    fn p988b_top_accent_attach_fonte_real() {
        let data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/NewCMMath-Book.otf"
        ))
        .expect("fixture NewCMMath-Book.otf necessária");
        let m = FontBookMetrics::from_bytes(&data).expect("fonte válida");
        let style = TextStyle::default();
        let size = Pt(11.0);

        let x_it = m
            .top_accent_attach('\u{1D44E}', size, &style)
            .expect("x itálico existe na face");
        assert!(
            (x_it.val() - 287.0 * 11.0 / 1000.0).abs() < 0.01,
            "TopAccentAttach de 𝑥 = 287du ≈ 3.157pt: {:.4}",
            x_it.val()
        );
        let hat = m
            .top_accent_attach('\u{0302}', size, &style)
            .expect("hat combinante existe na face");
        assert!(
            (hat.val() - 250.0 * 11.0 / 1000.0).abs() < 0.01,
            "TopAccentAttach de ̂ = 250du = 2.75pt: {:.4}",
            hat.val()
        );
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
    fn proporcionalidade_iiii_vs_wwww() {
        let data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/NimbusSans-Regular.otf"
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
    fn vertical_metrics_sanidade() {
        let data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/NimbusSans-Regular.otf"
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

    /// **P890** — `advance()` deve encontrar um carácter coberto só pela
    /// cadeia de fallback matemático (`math_fallback_font_list()`, primeira
    /// entrada "New Computer Modern Math") **sem** chamar
    /// `World::candidates_for_char` — o scan caro só é aceitável quando a
    /// cadeia dedicada genuinamente não cobre o carácter, não quando cobre
    /// mas não foi consultada (a causa exacta confirmada por instrumentação
    /// em `typst-passo-890-relatorio.md`). Antes da correcção, `advance()`
    /// chamava só `resolve_primary` (sem a cadeia math) e este teste falha.
    #[test]
    fn p890_advance_math_nao_chama_candidates_for_char_quando_math_fallback_cobre() {
        use std::num::NonZeroU16;
        use std::sync::atomic::{AtomicUsize, Ordering};
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

        struct SpyWorld {
            library: Library,
            book: FontBook,
            fonts: Vec<Option<Font>>,
            candidates_for_char_calls: AtomicUsize,
        }

        impl World for SpyWorld {
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
                self.fonts.get(idx).cloned().flatten()
            }
            fn today(&self, _: Option<i64>) -> Option<Datetime> {
                None
            }
            fn candidates_for_char(&self, _: char) -> Vec<usize> {
                // O scan caro em si (custo de P880/candidates_for_char) não é
                // o que este teste verifica — o que importa é que NÃO seja
                // chamado neste cenário. Regista a chamada em vez de dar
                // panic para a asserção poder reportar a contagem exacta.
                self.candidates_for_char_calls.fetch_add(1, Ordering::SeqCst);
                vec![]
            }
        }

        // Nimbus Sans (fonte de corpo declarada) NÃO tem 'Ə' (U+018F).
        let nimbus_data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/NimbusSans-Regular.otf"
        ))
        .expect("fixture NimbusSans-Regular.otf necessária");
        // Cantarell TEM 'Ə' (U+018F) — usada aqui a fazer de stand-in para
        // "New Computer Modern Math" (não precisamos da fonte MATH real de
        // produção só para provar que a cadeia é consultada; qualquer fonte
        // registada sob esse nome exacto serve para o mecanismo).
        let cantarell_data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/Cantarell-VF.otf"
        ))
        .expect("fixture Cantarell-VF.otf necessária");

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
        book.push(FontInfo {
            family: "New Computer Modern Math".into(),
            variant: FontVariant {
                style: FontStyle::Normal,
                weight: FontWeight::REGULAR,
                stretch: FontStretch::NORMAL,
            },
            flags: FontFlags::default(),
            coverage: Coverage::default(),
        });

        let world = SpyWorld {
            library: Library::new(),
            book,
            fonts: vec![Some(Font::from_data(nimbus_data)), Some(Font::from_data(cantarell_data))],
            candidates_for_char_calls: AtomicUsize::new(0),
        };

        let metrics = FallbackFontMetrics::new(&world);
        let mut style = TextStyle::default();
        style.font = Some(typst_core::entities::font_list::FontList::single(
            ecow::EcoString::from("Nimbus Sans"),
        ));
        style.math = true;

        let _ = metrics.advance("\u{018F}", Pt(11.0), &style);

        assert_eq!(
            world.candidates_for_char_calls.load(Ordering::SeqCst),
            0,
            "advance() não deveria ter caído no scan caro de candidates_for_char — \
             'Ə' (U+018F) já está coberto pela cadeia de fallback matemático \
             ('New Computer Modern Math'), só faltava consultá-la antes do scan global"
        );
    }

    // P912 — `covering()` deve preferir a fonte com tabela MATH quando `style.math` é
    // verdadeiro, para que delimitadores como `(`, `)` resolvam para a fonte MATH.
    #[test]
    fn p912_covering_prefere_fonte_math_para_glifos_comuns() {
        use crate::world::SystemWorld;

        let dir = std::env::temp_dir().join(format!(
            "typst-fontmetrics-test-p912-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("main.typ"), "text").unwrap();
        let Ok(world) = SystemWorld::new(&dir, "main.typ").map(|w| w.with_system_fonts()) else {
            return;
        };
        let book = world.book();
        if !book.infos().iter().any(|i| i.family.contains("Math")) {
            eprintln!("SKIP: Nenhuma fonte MATH instalada no sistema");
            return;
        }

        let metrics = FallbackFontMetrics::new(&world);
        let mut style = TextStyle::default();
        style.font = Some(typst_core::entities::font_list::FontList::single(
            ecow::EcoString::from("DejaVu Sans"),
        ));
        style.math = true;

        let variant = text_style_to_font_variant(&style);
        let primary = metrics.resolve_primary_with_math_fallback(&style, &variant);
        let cand = metrics
            .covering('(', &primary, &variant)
            .expect("alguma fonte cobre '('");

        let chosen = metrics.cached_face(cand.slot_idx).expect("cached face");
        assert!(
            chosen.face().tables().math.is_some(),
            "covering('(') com style.math=true deve escolher uma face com tabela OpenType MATH"
        );
    }

    /// **P946** — `build_math_glyph_reverse_map` prefere o **codepoint real**
    /// da cmap da face para peças de assembly (ex.: `⎩`/`⎪`/`⎨`/`⎧` para a
    /// chave) — decisão do dono: extracção semanticamente mais rica, como
    /// divergência deliberada do vanilla (que agrupa a assembly num único
    /// char base no ToUnicode — medição em `infra/font_metrics.md` §P946).
    /// Glifos **não codificados** (variantes `.vN`, peças sem codepoint
    /// próprio como `braceleft.ex`) mantêm o fallback anterior
    /// (`base_char`/`|`/`_`).
    ///
    /// Usa a fonte `NewCMMath` real embutida (mesmo padrão de P891:
    /// localizada por cobertura, não por nome) — a cmap dela cobre
    /// U+23A7–U+23AA/U+239B–U+23A0 (verificado via fontTools em P946).
    #[test]
    fn p946_reverse_map_assembly_usa_codepoints_reais_da_cmap() {
        let data = typst_assets::fonts()
            .find(|data| {
                let Ok(face) = ttf_parser::Face::parse(data, 0) else { return false };
                face.tables().math.and_then(|m| m.variants).is_some()
                    && face.glyph_index('\u{23A9}').is_some()
                    && face.glyph_index('\u{239C}').is_some()
            })
            .expect("fonte embutida NewCMMath com variants + U+23A9/U+239C tem de existir");
        let face = ttf_parser::Face::parse(data, 0).unwrap();
        let map = build_math_glyph_reverse_map(&face);

        // Assembly de '{' (ordem da fonte: fundo → topo):
        // [⎩ U+23A9, braceleft.ex (sem codepoint próprio → fallback '|'),
        //  ⎨ U+23A8, braceleft.ex ('|'), ⎧ U+23A7].
        let assembly = extract_assembly(&face, '{');
        assert!(!assembly.is_empty(), "NewCMMath tem assembly para '{{'");
        let chars: Vec<char> =
            assembly.parts.iter().map(|p| map[&p.glyph_id]).collect();
        assert_eq!(
            chars,
            vec!['\u{23A9}', '|', '\u{23A8}', '|', '\u{23A7}'],
            "peças com codepoint na cmap mapeiam para o codepoint real; \
             extensores sem codepoint próprio (braceleft.ex) caem no fallback '|'"
        );

        // Assembly de '(': [⎝ U+239D, ⎜ U+239C, ⎛ U+239B].
        let paren = extract_assembly(&face, '(');
        assert!(!paren.is_empty(), "NewCMMath tem assembly para '('");
        let paren_chars: Vec<char> =
            paren.parts.iter().map(|p| map[&p.glyph_id]).collect();
        assert_eq!(paren_chars, vec!['\u{239D}', '\u{239C}', '\u{239B}']);

        // Variantes (.vN) não são codificadas na cmap → fallback base_char.
        let variants = extract_variants(&face, '(');
        assert!(!variants.is_empty(), "NewCMMath tem variantes para '('");
        for v in &variants.variants {
            assert_eq!(
                map[&v.glyph_id], '(',
                "variante sem codepoint na cmap deve cair no base_char"
            );
        }

        // **P952** — variantes de operadores grandes (Display) também entram
        // no mapa (subset/ToUnicode), com fallback ao base_char.
        let sum_variants = extract_variants(&face, '∑');
        assert!(!sum_variants.is_empty(), "NewCMMath tem variantes para '∑'");
        for v in &sum_variants.variants {
            assert_eq!(
                map[&v.glyph_id], '∑',
                "variante de operador grande sem codepoint deve cair no base_char"
            );
        }
    }
}
