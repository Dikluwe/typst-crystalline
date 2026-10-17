//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/font_metrics/basic.md
//! @prompt-hash 87ac0ed4
//! @layer L3
//! @updated 2026-09-26

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
        None => default, // neutro: N16[γ] — TextEdge None resolve para default cap-height/baseline (fallback de métrica)
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

impl FontMetrics for FontBookMetrics<'_> {
    fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
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
