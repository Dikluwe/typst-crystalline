//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/font_metrics/fallback.md
//! @prompt-hash 08d7a559
//! @layer L3
//! @updated 2026-09-27

use super::basic::{edge_offset_pt, typo_metrics};
use super::math::{
    extract_assembly, extract_assembly_horizontal, extract_variants,
    extract_variants_horizontal, is_variation_selector, math_constants_from_face,
    math_kern_from_face, ssty_eligible_text, ssty_level_of, ssty_substitute,
};
use super::FallbackFontMetrics;
use crate::font_variant::{axis_variations_for_text_style, text_style_to_font_variant};
use ttf_parser::Face;
use typst_core::compiler::layout::{FixedMetrics, FontMetrics};
use typst_core::entities::glyph_variants::{GlyphAssembly, GlyphVariants, MathGlyphKern};
use typst_core::entities::layout_types::{Pt, TextStyle};
use typst_core::entities::math_constants::MathConstants;

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
        self.cached_advance_width(text, size, style, || {
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
            // **P1129** — só para texto ssty-elegível (1 carácter ou
            // número puro — `ssty_eligible_text`).
            let ssty_level = ssty_level_of(style, ssty_eligible_text(text));
            let mut total = 0.0;
            let mut prev: Option<(usize, u16)> = None;
            let mut last_base: Option<(usize, u16)> = None;

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
                if !is_variation_selector(c) {
                    last_base = slot.map(|s| (s, gid));
                }
            }

            // **P975** — vanilla `update_glyph`
            // (`lab/typst-original/crates/typst-layout/src/math/fragment/
            // glyph.rs:204-211`): para glifos math singulares não-esticados,
            // `x_advance += italics_correction`. Só `style.math` e só 1
            // carácter (texto math multi-carácter é run, sem o termo).
            // `infra/font_metrics.md` §P975.
            let is_math_glyph = style.math && !style.math_text_item;
            let mut base_chars = text.chars().filter(|c| !is_variation_selector(*c));
            let is_single_base =
                base_chars.next().is_some() && base_chars.next().is_none();
            if is_math_glyph && is_single_base {
                if let Some((slot_idx, gid)) = last_base {
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
    fn advance_shaped(&self, text: &str, size: Pt, style: &TextStyle) -> Option<Pt> {
        let effective_style;
        let style = if size.0.to_bits() == style.size.0.to_bits() {
            style
        } else {
            effective_style = TextStyle { size, ..style.clone() };
            &effective_style
        };
        // **P1104** — Usar o shaper (rustybuzz com GPOS kerning) para medição precisa de extensão de texto
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
                let top = edge_offset_pt(face, upem, size, style.top_edge.as_ref(), true);
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
        let ssty_eligible = ssty_eligible_text(text);
        for c in text.chars() {
            let Some(cand) = self.covering(c, &primary, &variant) else {
                continue;
            };
            let Some(cached) = self.cached_face(cand.slot_idx) else { continue };
            let face = cached.face();
            let Some(mut gid) = face.glyph_index(c) else {
                continue;
            };
            // **P977** — variante ssty (a tinta medida é a do glifo que
            // vai ser desenhado). **P1129** — texto ssty-elegível (1
            // carácter ou número puro), não só glifo atómico.
            if let Some(level) = ssty_level_of(style, ssty_eligible) {
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

    fn char_italics_correction(&self, c: char, size: Pt, style: &TextStyle) -> Pt {
        let variant = text_style_to_font_variant(style);
        let primary = self.resolve_primary_with_math_fallback(style, &variant);
        let Some(cand) = self.covering(c, &primary, &variant) else {
            return Pt(0.0);
        };
        let Some(cached) = self.cached_face(cand.slot_idx) else {
            return Pt(0.0);
        };
        let face = cached.face();
        let Some(mut gid) = face.glyph_index(c) else {
            return Pt(0.0);
        };
        if let Some(level) = ssty_level_of(style, true) {
            if let Some(sub) = ssty_substitute(face, gid, level) {
                gid = sub;
            }
        }
        let Some(value) = face
            .tables()
            .math
            .and_then(|m| m.glyph_info)
            .and_then(|gi| gi.italic_corrections)
            .and_then(|ic| ic.get(gid))
        else {
            return Pt(0.0);
        };
        let upem = cand.units_per_em as f64;
        Pt(size.val() * (value.value as f64 / upem))
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
        if let Some(level) = ssty_level_of(style, true) {
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
    fn text_ink_bounds_signed(
        &self,
        text: &str,
        size: Pt,
        style: &TextStyle,
    ) -> (Pt, Pt) {
        let variant = text_style_to_font_variant(style);
        let primary = self.resolve_primary_with_math_fallback(style, &variant);
        let mut top = f64::NEG_INFINITY;
        let mut bottom = f64::NEG_INFINITY;
        let ssty_eligible = ssty_eligible_text(text);
        for c in text.chars() {
            let Some(cand) = self.covering(c, &primary, &variant) else {
                continue;
            };
            let Some(cached) = self.cached_face(cand.slot_idx) else { continue };
            let face = cached.face();
            let Some(mut gid) = face.glyph_index(c) else {
                continue;
            };
            // **P977** — variante ssty (ver `text_ink_bounds`). **P1129**
            // — texto ssty-elegível (1 carácter ou número puro).
            if let Some(level) = ssty_level_of(style, ssty_eligible) {
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
    use crate::font_metrics::FallbackFontMetrics;
    use typst_core::compiler::layout::FontMetrics;
    use typst_core::entities::layout_types::{Pt, TextStyle};

    /// **P891** — `FallbackFontMetrics::math_kern` deve ler a tabela MATH
    /// real da face que cobre `c` (via `resolve_primary_with_math_fallback`
    /// e `covering`), não devolver sempre o default do trait (kern zero
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
            .expect(
                "fonte embutida NewCMMath com kern não-zero para U+1D437 tem de existir",
            );

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
            fn today(
                &self,
                _: Option<typst_core::entities::duration::Duration>,
            ) -> Option<Datetime> {
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
        let style = TextStyle {
            math: true, // engata DEFAULT_FALLBACK_FONTS_MATH (P890).
            ..TextStyle::default()
        };

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
        let construction = face
            .tables()
            .math
            .unwrap()
            .variants
            .unwrap()
            .vertical_constructions
            .get(gid)
            .unwrap();
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
            fn today(
                &self,
                _: Option<typst_core::entities::duration::Duration>,
            ) -> Option<Datetime> {
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
        let style = TextStyle {
            math: true, // engata DEFAULT_FALLBACK_FONTS_MATH (P890).
            ..TextStyle::default()
        };

        let variants = metrics.vertical_glyph_variants(c, &style);
        let actual: Vec<(u16, f64, f64)> = variants
            .variants
            .iter()
            .map(|v| (v.glyph_id, v.advance, v.hor_advance))
            .collect();
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
            expected
                .iter()
                .any(|(_, _, _, full, _, hor)| (*full as f64 - hor).abs() > 1.0),
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
            fn today(
                &self,
                _: Option<typst_core::entities::duration::Duration>,
            ) -> Option<Datetime> {
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
        let style = TextStyle { math: true, ..TextStyle::default() };

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
        let expected_sup_shift_up_cramped =
            ttf_constants.superscript_shift_up_cramped().value as f64;
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
            fn today(
                &self,
                _: Option<typst_core::entities::duration::Duration>,
            ) -> Option<Datetime> {
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
        let style = TextStyle { math: true, ..TextStyle::default() };
        let variant = text_style_to_font_variant(&style);
        let primary = metrics.resolve_primary_with_math_fallback(&style, &variant);
        let cand = metrics
            .covering('(', &primary, &variant)
            .expect("covering deve resolver a face MATH para '(' em contexto matemático");
        let cached = metrics
            .cached_face(cand.slot_idx)
            .expect("face resolvida deve estar em cache");
        let constants = cached.face().tables().math.unwrap().constants;
        assert!(
            constants.is_some(),
            "face resolvida por covering() deve ter tabela MATH constants"
        );

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
        let style = TextStyle {
            font: Some(typst_core::entities::font_list::FontList::single(
                ecow::EcoString::from("DejaVu Sans"),
            )),
            size: Pt(12.0),
            ..TextStyle::default()
        };

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
        let style = TextStyle {
            font: Some(typst_core::entities::font_list::FontList::single(
                ecow::EcoString::from("DejaVu Sans"),
            )),
            size: Pt(40.0),
            lang: Some("ar".parse().unwrap()),
            dir: Some(typst_core::entities::dir::Dir::RTL),
            ..TextStyle::default()
        };

        // Para palavras árabes, a largura com shaping deve ser menor que a
        // soma das letras isoladas.
        let plain = metrics.advance("الكتاب", Pt(40.0), &style);
        let shaped = metrics.advance_shaped("الكتاب", Pt(40.0), &style);
        assert!(shaped.is_some(), "advance_shaped deve retornar Some para arabe");
        assert!(
            shaped.unwrap().val() < plain.val() - 10.0,
            "shaped width de 'الكتاب' deve ser significativamente menor que non-shaped; plain={:.4}, shaped={:.4}",
            plain.val(),
            shaped.unwrap().val()
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
        let style_400 = TextStyle {
            font: Some(typst_core::entities::font_list::FontList::single(
                ecow::EcoString::from("Ubuntu Sans"),
            )),
            size: Pt(11.0),
            weight: Some(400),
            ..TextStyle::default()
        };

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
}
