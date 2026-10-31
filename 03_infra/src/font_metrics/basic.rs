//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/font_metrics/basic.md
//! @prompt-hash 2348de80
//! @layer L3
//! @updated 2026-10-24

use super::math::{
    extract_assembly, extract_assembly_horizontal, extract_variants,
    extract_variants_horizontal, is_variation_selector, math_constants_from_face,
    math_kern_from_face, ssty_eligible_text, ssty_level_of, ssty_substitute,
};
use super::FontBookMetrics;
use ttf_parser::Face;
use typst_core::compiler::layout::FontMetrics;
use typst_core::entities::glyph_variants::{GlyphAssembly, GlyphVariants, MathGlyphKern};
use typst_core::entities::layout_types::{Pt, TextEdge, TextStyle};
use typst_core::entities::math_constants::MathConstants;

/// Devolve as métricas verticais tipográficas de uma face, preferindo os
/// valores do OS/2 (`sTypoAscender`, `sTypoDescender`, `sTypoLineGap`) quando
/// disponíveis, e caindo para as métricas do `hhea` caso contrário.
///
/// Retorna `(ascender, descender_abs, line_gap)` em unidades de fonte.
pub(super) fn typo_metrics(face: &Face<'_>) -> (f64, f64, f64) {
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
        Option::None => default,
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

/// Shared face/table conversion; no font selection or cached-face mutation.
pub(super) fn underline_in_face(face: &Face<'_>, size: Pt) -> (Pt, Pt) {
    underline_in_tables(
        face.underline_metrics(),
        face.strikeout_metrics(),
        f64::from(face.units_per_em().max(1)),
        size,
    )
}

fn underline_in_tables(
    underline: Option<ttf_parser::LineMetrics>,
    strikeout: Option<ttf_parser::LineMetrics>,
    upem: f64,
    size: Pt,
) -> (Pt, Pt) {
    let offset =
        underline.map_or(size * 0.2, |line| size * (-f64::from(line.position) / upem));
    let thickness = underline
        .or(strikeout)
        .map_or(size * 0.06, |line| size * (f64::from(line.thickness).max(0.0) / upem));
    (offset, thickness)
}

/// Resolve text script geometry from the selected, already varied face.
pub(crate) fn superscript_in_face(
    face: &Face<'_>,
    text: &str,
    size: Pt,
) -> (Pt, Pt, Pt, bool) {
    let native = !text.is_empty()
        && face.tables().gsub.is_some_and(|gsub| {
            gsub.features.find(ttf_parser::Tag::from_bytes(b"sups")).is_some_and(
                |feature| {
                    text.chars().all(|c| {
                        face.glyph_index(c).is_some_and(|gid| {
                            feature
                                .lookup_indices
                                .into_iter()
                                .filter_map(|i| gsub.lookups.get(i))
                                .flat_map(|lookup| {
                                    lookup
                                    .subtables
                                    .into_iter::<ttf_parser::gsub::SubstitutionSubtable>()
                                })
                                .any(|subtable| subtable.coverage().contains(gid))
                        })
                    })
                },
            )
        });
    if native {
        return (size, Pt(0.0), Pt(0.0), true);
    }
    synthetic_superscript_in_face(face, size)
}

fn synthetic_superscript_in_face(face: &Face<'_>, size: Pt) -> (Pt, Pt, Pt, bool) {
    let Some(metrics) = face.superscript_metrics() else {
        return (size * 0.6, Pt(0.0), size * -0.5, false);
    };
    let scale = size.0 / f64::from(face.units_per_em().max(1));
    (
        Pt(f64::from(metrics.y_size) * scale),
        Pt(f64::from(metrics.x_offset) * scale),
        Pt(-f64::from(metrics.y_offset) * scale),
        false,
    )
}

pub(super) fn resolved_superscript_style(
    face: &Face<'_>,
    text: &str,
    size: Pt,
    style: &TextStyle,
) -> (TextStyle, Pt) {
    let metrics = if style.superscript_native == Some(false) {
        synthetic_superscript_in_face(face, size)
    } else {
        superscript_in_face(face, text, size)
    };
    let (paint_size, x, y, native) = metrics;
    let mut actual = style.clone();
    actual.size = if native {
        paint_size
    } else {
        style
            .superscript_size
            .map(|l| Pt(l.resolve_pt(size.0)))
            .unwrap_or(paint_size)
    };
    if size.0.is_finite() && size.0 != 0.0 {
        actual.tracking = style.tracking.map(|length| {
            typst_core::entities::layout_types::Length::pt(
                length.resolve_pt(size.0) * actual.size.0 / size.0,
            )
        });
    }
    actual.baseline_offset = typst_core::entities::layout_types::Length::pt(y.0);
    actual.superscript_native = Some(native);
    let upem = f64::from(face.units_per_em().max(1));
    let top = edge_offset_pt(face, upem, size, style.top_edge.as_ref(), true);
    let bottom = edge_offset_pt(face, upem, size, style.bottom_edge.as_ref(), false);
    actual.top_edge =
        Some(TextEdge::Length(typst_core::entities::layout_types::Length::pt(top.0)));
    actual.bottom_edge =
        Some(TextEdge::Length(typst_core::entities::layout_types::Length::pt(bottom.0)));
    (actual, x)
}

impl FontMetrics for FontBookMetrics<'_> {
    fn text_trailing_overhang(
        &self,
        text: &str,
        size: Pt,
        style: &TextStyle,
        ratios: &[(char, f64)],
    ) -> Option<(Pt, usize)> {
        let mut face = self.face.clone();
        for variation in crate::font_variant::axis_variations_for_text_style(style) {
            face.set_variation(variation.tag, variation.value);
        }
        let style = TextStyle { size, ..style.clone() };
        crate::shaper::terminal_in_face(
            &rustybuzz::Face::from_face(face),
            text,
            &style,
            ratios,
        )
    }
    fn glyph_overhang(
        &self,
        font: &typst_core::entities::world_types::Font,
        glyph_id: u16,
        advance: Pt,
        ratios: &[(char, f64)],
    ) -> Option<Pt> {
        let face = Face::parse(font.as_slice(), 0).ok()?;
        crate::shaper::glyph_overhang_in_face(&face, glyph_id, advance, ratios)
    }

    fn text_superscript_runs(
        &self,
        text: &str,
        size: Pt,
        style: &TextStyle,
    ) -> Vec<(std::ops::Range<usize>, TextStyle, Pt)> {
        let mut face = self.face.clone();
        for variation in crate::font_variant::axis_variations_for_text_style(style) {
            face.set_variation(variation.tag, variation.value);
        }
        let (actual, x) = resolved_superscript_style(&face, text, size, style);
        vec![(0..text.len(), actual, x)]
    }

    fn text_superscript_metrics(
        &self,
        text: &str,
        size: Pt,
        style: &TextStyle,
    ) -> (Pt, Pt, Pt, bool) {
        let mut face = self.face.clone();
        for variation in crate::font_variant::axis_variations_for_text_style(style) {
            face.set_variation(variation.tag, variation.value);
        }
        superscript_in_face(&face, text, size)
    }

    fn underline_metrics(&self, _: &str, size: Pt, style: &TextStyle) -> (Pt, Pt) {
        let mut face = self.face.clone();
        for variation in crate::font_variant::axis_variations_for_text_style(style) {
            face.set_variation(variation.tag, variation.value);
        }
        underline_in_face(&face, size)
    }

    fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
        if style.superscript
            && !style.subscript
            && style.superscript_native != Some(false)
        {
            let mut face = self.face.clone();
            for variation in crate::font_variant::axis_variations_for_text_style(style) {
                face.set_variation(variation.tag, variation.value);
            }
            if superscript_in_face(&face, text, size).3 {
                let face = rustybuzz::Face::from_face(face);
                let mut buffer = rustybuzz::UnicodeBuffer::new();
                buffer.push_str(text);
                let feature =
                    rustybuzz::Feature::new(ttf_parser::Tag::from_bytes(b"sups"), 1, ..);
                let shaped = rustybuzz::shape(&face, &[feature], buffer);
                let units: i32 =
                    shaped.glyph_positions().iter().map(|p| p.x_advance).sum();
                return size * (f64::from(units) / self.upem);
            }
        }

        // Fórmula: advance_pt = font_size * (Σ glyph_units / upem)
        // **P977** — variante ssty por carácter (mesma regra de
        // `FallbackFontMetrics::advance`). **P1129** — só para texto
        // ssty-elegível (`ssty_eligible_text`): 1 carácter ou número
        // puro (dígito a dígito, `NumberItem` no vanilla) — texto
        // multi-carácter não numérico não força o script `math` no
        // vanilla, logo `ssty` nunca se aplica.
        let ssty_level = ssty_level_of(style, ssty_eligible_text(text));
        let mut units: f64 = text
            .chars()
            .filter(|c| !is_variation_selector(*c))
            .map(|c| {
                self.face
                    .glyph_index(c)
                    .map(|gid| match ssty_level {
                        Some(level) => {
                            ssty_substitute(&self.face, gid, level).unwrap_or(gid)
                        }
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
        let mut base_chars = text.chars().filter(|c| !is_variation_selector(*c));
        let base_char = base_chars.next();
        let is_single_base = base_char.is_some() && base_chars.next().is_none();
        if style.math && !style.math_text_item && is_single_base {
            let c = base_char.unwrap();
            if let Some(gid) = self.face.glyph_index(c) {
                let gid = match ssty_level_of(style, true) {
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

    fn char_italics_correction(&self, c: char, size: Pt, _style: &TextStyle) -> Pt {
        let Some(gid) = self.face.glyph_index(c) else {
            return Pt(0.0);
        };
        let Some(value) = self
            .face
            .tables()
            .math
            .and_then(|m| m.glyph_info)
            .and_then(|gi| gi.italic_corrections)
            .and_then(|ic| ic.get(gid))
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
    fn text_ink_bounds_signed(
        &self,
        text: &str,
        size: Pt,
        _style: &TextStyle,
    ) -> (Pt, Pt) {
        let mut top = f64::NEG_INFINITY;
        let mut bottom = f64::NEG_INFINITY;
        for c in text.chars() {
            let Some(gid) = self.face.glyph_index(c) else {
                continue;
            };
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

#[cfg(test)]
mod tests {
    use crate::font_metrics::FontBookMetrics;
    use typst_core::compiler::layout::FontMetrics;
    use typst_core::entities::layout_types::{Pt, TextStyle};

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

        let style = TextStyle {
            top_edge: Some(TextEdge::Length(Length::pt(18.0))),
            bottom_edge: Some(TextEdge::Length(Length::pt(-4.0))),
            ..TextStyle::default()
        };
        let (top, bottom) = m.text_edges(size, &style);
        assert!((top.val() - 18.0).abs() < 1e-9, "top={:?}", top);
        assert!((bottom.val() + 4.0).abs() < 1e-9, "bottom={:?}", bottom);

        // Componente em resolve no font-size: 1.5em a 11pt = 16.5pt.
        let style = TextStyle {
            top_edge: Some(TextEdge::Length(Length::em(1.5))),
            ..TextStyle::default()
        };
        let (top, _) = m.text_edges(size, &style);
        assert!((top.val() - 16.5).abs() < 1e-9, "top={:?}", top);

        // Métrica enumerada continua a vir da face (sem regressão):
        // "baseline" = 0 nos dois edges.
        let style = TextStyle {
            top_edge: Some(TextEdge::Metric("baseline".into())),
            bottom_edge: Some(TextEdge::Metric("baseline".into())),
            ..TextStyle::default()
        };
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
            ai.val(),
            aw.val()
        );

        let aa = m.advance("A", size, &style);
        assert!(
            aa.val() > 3.0 && aa.val() < 12.0,
            "'A' em 12pt deve ser 3–12pt, foi {:.2}pt",
            aa.val()
        );
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

    #[test]
    fn r3_geometry_variation_selectors_have_no_scalar_advance() {
        let data = include_bytes!("../../fixtures/fonts/NewCMMath-Book.otf");
        let metrics = FontBookMetrics::from_bytes(data).unwrap();
        let size = Pt(12.0);
        for math in [false, true] {
            let style = TextStyle { math, ..TextStyle::regular(size) };
            let base = metrics.advance("𝑥", size, &style);
            for selector in ['\u{fe00}', '\u{fe0f}', '\u{e0100}', '\u{e01ef}'] {
                assert_eq!(metrics.advance(&format!("𝑥{selector}"), size, &style), base);
                assert_eq!(metrics.advance(&selector.to_string(), size, &style), Pt(0.0));
            }
        }
    }

    #[test]
    fn p1293_fontbook_textitem_math_suprime_somente_ic() {
        use typst_core::compiler::layout::FontMetrics;

        let c = '\u{1D453}'; // 𝑓 — possui IC não-zero na fonte MATH embutida.
        let data = typst_assets::fonts()
            .find(|data| {
                let Ok(face) = ttf_parser::Face::parse(data, 0) else { return false };
                let Some(gid) = face.glyph_index(c) else { return false };
                face.tables()
                    .math
                    .and_then(|m| m.glyph_info)
                    .and_then(|gi| gi.italic_corrections)
                    .and_then(|ic| ic.get(gid))
                    .is_some_and(|value| value.value != 0)
            })
            .expect("fonte MATH embutida com IC não-zero para 𝑓");
        let metrics = FontBookMetrics::from_bytes(data).expect("fonte válida");
        let size = Pt(11.0);
        let glyph_style = TextStyle {
            math: true,
            math_text_item: false,
            ..TextStyle::regular(size)
        };
        let textitem_style = TextStyle { math_text_item: true, ..glyph_style.clone() };

        let glyph = metrics.advance(&c.to_string(), size, &glyph_style);
        let textitem = metrics.advance(&c.to_string(), size, &textitem_style);
        let ic = metrics.char_italics_correction(c, size, &glyph_style);

        assert!(ic.val() > 0.0, "fixture precisa de IC discriminatória");
        assert!((glyph.val() - textitem.val() - ic.val()).abs() < 1e-9);
        assert!(glyph > textitem);
    }
}

#[cfg(test)]
mod remaining_underline_tests {
    use super::*;
    #[test]
    fn remaining_underline_missing_tables_preserve_signed_positions_and_fallback_priority(
    ) {
        assert_eq!(underline_in_tables(None, None, 1000.0, Pt(10.0)), (Pt(2.0), Pt(0.6)));
        let strike = ttf_parser::LineMetrics { position: 400, thickness: 30 };
        assert_eq!(
            underline_in_tables(None, Some(strike), 1000.0, Pt(10.0)),
            (Pt(2.0), Pt(0.3))
        );
        let underline = ttf_parser::LineMetrics { position: 50, thickness: 20 };
        assert_eq!(
            underline_in_tables(Some(underline), Some(strike), 1000.0, Pt(10.0)),
            (Pt(-0.5), Pt(0.2))
        );
        let zero = ttf_parser::LineMetrics { position: 0, thickness: -10 };
        assert_eq!(
            underline_in_tables(Some(zero), Some(strike), 1000.0, Pt(10.0)),
            (Pt(0.0), Pt(0.0))
        );
    }
    #[test]
    fn remaining_underline_real_faces_use_tables_and_the_explicit_requested_size() {
        for data in [
            include_bytes!("../../fixtures/fonts/DejaVuSans.ttf").as_slice(),
            include_bytes!("../../fixtures/fonts/NimbusSans-Regular.otf").as_slice(),
        ] {
            let face = Face::parse(data, 0).expect("declared portable font");
            let table = face.underline_metrics().expect("actual underline table");
            let upem = face.units_per_em() as f64;
            let provider = FontBookMetrics::from_bytes(data).expect("valid provider");
            let style = TextStyle { size: Pt(99.0), ..TextStyle::default() };
            for size in [11.0, 17.0] {
                let (offset, thickness) =
                    provider.underline_metrics("LEFT RIGHT", Pt(size), &style);
                assert!(
                    (offset.0 - (-f64::from(table.position) * size / upem)).abs() < 1e-8
                );
                assert!(
                    (thickness.0 - (f64::from(table.thickness) * size / upem)).abs()
                        < 1e-8
                );
            }
        }
    }
}

#[cfg(test)]
mod r8_superscript_tests {
    use super::*;
    #[test]
    fn r8_real_face_superscript_uses_os2_and_explicit_size() {
        let bytes = include_bytes!("../../fixtures/fonts/DejaVuSans.ttf");
        let face = Face::parse(bytes, 0).unwrap();
        let table = face.superscript_metrics().unwrap();
        let provider = FontBookMetrics::from_bytes(bytes).unwrap();
        let style = TextStyle { size: Pt(99.0), ..TextStyle::default() };
        for size in [11.0, 17.0] {
            let result = provider.text_superscript_metrics("12", Pt(size), &style);
            let scale = size / f64::from(face.units_per_em());
            assert_eq!(
                result,
                (
                    Pt(f64::from(table.y_size) * scale),
                    Pt(f64::from(table.x_offset) * scale),
                    Pt(-f64::from(table.y_offset) * scale),
                    false
                )
            );
        }
    }
}

#[cfg(test)]
mod r8_native_superscript_tests {
    use super::*;
    #[test]
    fn r8_native_superscript_requires_complete_glyph_coverage() {
        let bytes = include_bytes!("../../fixtures/fonts/NimbusSans-Regular.otf");
        let face = Face::parse(bytes, 0).unwrap();
        let gsub = face.tables().gsub.unwrap();
        let feature = gsub.features.find(ttf_parser::Tag::from_bytes(b"sups")).unwrap();
        let covered = |c| {
            face.glyph_index(c).is_some_and(|gid| {
                feature
                    .lookup_indices
                    .into_iter()
                    .filter_map(|i| gsub.lookups.get(i))
                    .flat_map(|l| {
                        l.subtables.into_iter::<ttf_parser::gsub::SubstitutionSubtable>()
                    })
                    .any(|s| s.coverage().contains(gid))
            })
        };
        let native = ('0'..='9').find(|&c| covered(c)).expect("actual covered digit");
        let refused =
            ('A'..='Z').find(|&c| !covered(c)).expect("actual uncovered letter");
        let provider = FontBookMetrics::from_bytes(bytes).unwrap();
        let style = TextStyle::default();
        for size in [11.0, 17.0] {
            assert_eq!(
                provider.text_superscript_metrics(&native.to_string(), Pt(size), &style),
                (Pt(size), Pt(0.0), Pt(0.0), true)
            );
            for text in [format!("{native}{refused}"), format!("{native}\u{10ffff}")] {
                assert!(!provider.text_superscript_metrics(&text, Pt(size), &style).3);
            }
        }
    }
}

#[cfg(test)]
mod r8_native_advance_tests {
    use super::*;
    #[test]
    fn r8_native_superscript_advance_matches_shaped_glyphs() {
        let bytes = include_bytes!("../../fixtures/fonts/NimbusSans-Regular.otf");
        let face = rustybuzz::Face::from_slice(bytes, 0).unwrap();
        let provider = FontBookMetrics::from_bytes(bytes).unwrap();
        let style = TextStyle {
            superscript: true,
            size: Pt(11.0),
            ..TextStyle::default()
        };
        let mut buf = rustybuzz::UnicodeBuffer::new();
        buf.push_str("123");
        let feature =
            rustybuzz::Feature::new(ttf_parser::Tag::from_bytes(b"sups"), 1, ..);
        let output = rustybuzz::shape(&face, &[feature], buf);
        let units: i32 = output.glyph_positions().iter().map(|p| p.x_advance).sum();
        let expected = units as f64 * 11.0 / f64::from(face.units_per_em());
        assert!((provider.advance("123", Pt(11.0), &style).0 - expected).abs() < 1e-8);
    }
}

#[cfg(test)]
mod r8_native_carrier_tests {
    use super::*;
    #[test]
    fn r8_forced_synthetic_fragment_keeps_normal_advance() {
        let bytes = include_bytes!("../../fixtures/fonts/NimbusSans-Regular.otf");
        let provider = FontBookMetrics::from_bytes(bytes).unwrap();
        let normal = TextStyle::default();
        let synthetic = TextStyle {
            superscript: true,
            superscript_native: Some(false),
            ..normal.clone()
        };
        assert_eq!(
            provider.advance("123", Pt(11.0), &synthetic),
            provider.advance("123", Pt(11.0), &normal)
        );
        assert_eq!(normal.superscript_native, None);
    }
}

#[cfg(test)]
mod r10_resolved_tracking_tests {
    use super::*;
    use typst_core::entities::layout_types::Length;
    #[test]
    fn r10_real_face_runs_scale_tracking_with_actual_script_paint_size() {
        let bytes = include_bytes!("../../fixtures/fonts/NimbusSans-Regular.otf");
        let face = Face::parse(bytes, 0).unwrap();
        let table = face.superscript_metrics().unwrap();
        let provider = FontBookMetrics::from_bytes(bytes).unwrap();
        for size in [11.0, 17.0] {
            let native = TextStyle { superscript: true, ..TextStyle::default() };
            assert_eq!(
                provider.text_superscript_runs("12", Pt(size), &native)[0]
                    .1
                    .superscript_native,
                Some(true)
            );
            for explicit in [None, Some(Length::pt(5.0))] {
                let style = TextStyle {
                    superscript_native: Some(false),
                    superscript_size: explicit,
                    tracking: Some(Length::pt(0.2)),
                    ..native.clone()
                };
                let result = provider.text_superscript_runs("12", Pt(size), &style);
                let actual = &result[0].1;
                let paint = explicit.map(|l| l.resolve_pt(size)).unwrap_or(
                    f64::from(table.y_size) / f64::from(face.units_per_em()) * size,
                );
                assert!((actual.size.0 - paint).abs() < 1e-10);
                assert!(
                    (actual.baseline_offset.resolve_pt(paint)
                        + f64::from(table.y_offset) / f64::from(face.units_per_em())
                            * size)
                        .abs()
                        < 1e-10
                );
                assert_eq!(actual.superscript_native, Some(false));
                assert!(
                    (actual.tracking.unwrap().resolve_pt(paint) - 0.2 * paint / size)
                        .abs()
                        < 1e-10
                );
            }
        }
    }
}

#[cfg(test)]
mod readiness_optical_tests {
    use super::*;
    use typst_core::entities::world_types::Font;

    fn fixture() -> &'static [u8] {
        typst_assets::fonts()
            .find(|data| {
                let Ok(face) = Face::parse(data, 0) else { return false };
                face.weight().to_number() == 400
                    && face.tables().fvar.is_none()
                    && face.glyph_index('A').is_some()
                    && face.glyph_index('.').is_some()
            })
            .expect("embedded static Latin/dot face")
    }

    #[test]
    fn readiness_optical_actual_face_text_and_glyph_metrics() {
        let bytes = fixture();
        let face = Face::parse(bytes, 0).unwrap();
        let dot = face.glyph_index('.').unwrap();
        let m = FontBookMetrics::from_bytes(bytes).unwrap();
        let style = TextStyle::regular(Pt(999.0));
        for size in [11.0, 17.0] {
            let expected = f64::from(face.glyph_hor_advance(dot).unwrap()) * size
                / f64::from(face.units_per_em())
                * 0.8;
            let (actual, count) = m
                .text_trailing_overhang("A.", Pt(size), &style, &[('.', 0.8)])
                .expect("certified actual terminal glyph");
            assert!((actual.0 - expected).abs() < 1e-10);
            assert_eq!(count, 2);
        }
        let font = Font::from_data(bytes.to_vec());
        assert_eq!(
            m.glyph_overhang(&font, dot.0, Pt(3.0), &[('.', 0.8)]),
            Some(Pt(3.0 * 0.8))
        );
        let a = face.glyph_index('A').unwrap();
        assert_eq!(m.glyph_overhang(&font, a.0, Pt(3.0), &[('.', 0.8)]), Some(Pt(0.0)));
        assert!(m
            .glyph_overhang(&Font::from_data(vec![]), dot.0, Pt(3.0), &[('.', 0.8)])
            .is_none());
        assert!(m.glyph_overhang(&font, dot.0, Pt(3.0), &[('.', f64::NAN)]).is_none());
    }

    #[test]
    fn readiness_optical_supplied_font_is_not_replaced_by_provider_face() {
        let own = fixture();
        let own_face = Face::parse(own, 0).unwrap();
        let own_dot = own_face.glyph_index('.').unwrap();
        let other = typst_assets::fonts()
            .find(|data| {
                let Ok(face) = Face::parse(data, 0) else { return false };
                face.glyph_index('.').is_some_and(|gid| gid != own_dot)
            })
            .expect("independent embedded dot GID");
        let other_face = Face::parse(other, 0).unwrap();
        let gid = other_face.glyph_index('.').unwrap();
        let m = FontBookMetrics::from_bytes(own).unwrap();
        assert_eq!(
            m.glyph_overhang(
                &Font::from_data(other.to_vec()),
                gid.0,
                Pt(3.0),
                &[('.', 0.8)]
            ),
            Some(Pt(3.0 * 0.8))
        );
    }
}
