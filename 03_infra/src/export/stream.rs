//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/stream.md
//! @prompt-hash eaf016c2
//! @layer L3
//! @updated 2026-05-19
//!
//! Stream cluster — `PageContext` + `FontScenario` (agregador
//! unificado P281) + `build_page_stream` + emit helpers
//! (`emit_text_pdf`, `emit_glyph_pdf`, `emit_stroke_paint`,
//! `line_rg_prefix`) + shape primitives + `draw_item_local`.
//!
//! Extraído de `export.rs` em P307b.1 (ADR-0100 / diagnóstico
//! P307a §5). Sub-divisão em `stream/{mod,page,text,shape,
//! draw}.rs` é objectivo de P307b.2 (refino opcional).
//!
//! Conteúdo bit-exact pré e pós migração.

#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use std::collections::HashMap;
use std::sync::Arc;

use typst_core::entities::font_list::FontList;
use typst_core::entities::layout_types::{FrameItem, Page};

use super::{
    dedup_key_for, escape_pdf_string, group_bbox_from_fields, remap_glyph_id,
    text_to_hex_string, DedupKey, ImageRef, PatternRef,
};

// ── Helpers — caminho Helvetica ────────────────────────────────────────────

// ── P281 — PageContext + FontScenario (agregador unificado) ───────────────
//
// Unificação dos 3 stream-builders (Type1/CIDFont/Multifont) num
// pipeline central `build_page_stream` despachado por `PageContext`.
// Single source of truth para emit PDF; Text/Glyph dispatch via
// `FontScenario` enum. Sub-padrão "Agregador de contexto em L3"
// N=1 inaugural (análogo conceptual ADR-0044 `Engine<'a>` em L1).
// Decisão: NÃO formalizado em ADR per anti-padrão over-formalização
// P273.17 §0; L0 `infra/export.md` secção "Pipeline unificado (P281)"
// documenta arquitectura.

/// **P281** — variant que distingue o caminho de Text/Glyph emit.
pub(crate) enum FontScenario<'a> {
    /// Helvetica Type1 com WinAnsiEncoding (Latin-1; non-ASCII → `?`).
    /// Glyph silently ignored (sem TrueType embebida).
    Type1,
    /// CIDFont single-font Identity-H (Unicode completo, fonte embebida).
    Cidfont {
        char_to_gid: &'a HashMap<char, u16>,
        /// P516 — mapa old → new glyph ID quando a fonte foi subsetada.
        glyph_mapping: &'a HashMap<u16, u16>,
        /// P520 — largura nominal (hmtx) de cada old glyph ID, em unidades da fonte.
        /// Necessário para calcular o delta do operador `TJ` quando o shaper
        /// aplica kerning (x_advance ≠ largura declarada no `/W`).
        glyph_to_nominal: &'a HashMap<u16, i32>,
    },
    /// Multifont Identity-H com selecção `/F{fi+1}` por `style.font`.
    Multifont {
        fonts:                &'a [(FontList, Vec<u8>)],
        per_font_char_to_gid: &'a [HashMap<char, u16>],
        /// P516 — mapa old → new glyph ID por fonte (vazio se não subsetada).
        per_font_glyph_mapping: &'a [HashMap<u16, u16>],
        /// P520 — largura nominal por old glyph ID, por fonte.
        per_font_glyph_to_nominal: &'a [HashMap<u16, i32>],
    },
}

/// **P281** — agregador de contexto para emit unificado.
pub(crate) struct PageContext<'a> {
    pub ptr_to_idx:     &'a HashMap<usize, usize>,
    pub img_refs:       &'a [ImageRef],
    pub pat_ptr_to_idx: &'a HashMap<DedupKey, usize>,
    pub pat_refs:       &'a [PatternRef],
    pub font_scenario:  FontScenario<'a>,
}

impl<'a> PageContext<'a> {
    pub(crate) fn type1(
        ptr_to_idx:     &'a HashMap<usize, usize>,
        img_refs:       &'a [ImageRef],
        pat_ptr_to_idx: &'a HashMap<DedupKey, usize>,
        pat_refs:       &'a [PatternRef],
    ) -> Self {
        Self {
            ptr_to_idx, img_refs, pat_ptr_to_idx, pat_refs,
            font_scenario: FontScenario::Type1,
        }
    }

    pub(crate) fn cidfont(
        ptr_to_idx:        &'a HashMap<usize, usize>,
        img_refs:          &'a [ImageRef],
        pat_ptr_to_idx:    &'a HashMap<DedupKey, usize>,
        pat_refs:          &'a [PatternRef],
        char_to_gid:       &'a HashMap<char, u16>,
        glyph_mapping:     &'a HashMap<u16, u16>,
        glyph_to_nominal:  &'a HashMap<u16, i32>,
    ) -> Self {
        Self {
            ptr_to_idx, img_refs, pat_ptr_to_idx, pat_refs,
            font_scenario: FontScenario::Cidfont {
                char_to_gid, glyph_mapping, glyph_to_nominal,
            },
        }
    }

    pub(crate) fn multifont(
        ptr_to_idx:                  &'a HashMap<usize, usize>,
        img_refs:                    &'a [ImageRef],
        pat_ptr_to_idx:              &'a HashMap<DedupKey, usize>,
        pat_refs:                    &'a [PatternRef],
        fonts:                       &'a [(FontList, Vec<u8>)],
        per_font_char_to_gid:        &'a [HashMap<char, u16>],
        per_font_glyph_mapping:      &'a [HashMap<u16, u16>],
        per_font_glyph_to_nominal:   &'a [HashMap<u16, i32>],
    ) -> Self {
        Self {
            ptr_to_idx, img_refs, pat_ptr_to_idx, pat_refs,
            font_scenario: FontScenario::Multifont {
                fonts,
                per_font_char_to_gid,
                per_font_glyph_mapping,
                per_font_glyph_to_nominal,
            },
        }
    }
}

/// **P281** — emit Text PDF dispatched por `FontScenario`.
///
/// `base_y` é a coordenada Y final em PDF (top-level: `page_height -
/// pos.y`; local: `pos.y.0` directo após Group `cm`).
pub(super) fn emit_text_pdf(
    ops:      &mut String,
    pos_x:    f64,
    base_y:   f64,
    text:     &str,
    style:    &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
) {
    match scenario {
        FontScenario::Type1 => {
            let safe = escape_pdf_string(text);
            if safe.is_empty() { return; }
            let font_ref = match (style.bold, style.italic) {
                (true,  _)     => "F2",
                (false, true)  => "F3",
                (false, false) => "F1",
            };
            let tracking_pt = style.tracking
                .map(|t| t.resolve_pt(style.size.val()))
                .unwrap_or(0.0);
            let tc_op = if tracking_pt.abs() > f64::EPSILON {
                format!("{:.2} Tc\n", tracking_pt)
            } else {
                String::new()
            };
            const FAUX_BOLD_K: f64 = 0.04;
            let stroke_pt = style.faux_bold_stroke_pt(FAUX_BOLD_K);
            let (q_open, q_close, bold_ops) = if stroke_pt > f64::EPSILON {
                ("q\n", "Q\n", format!("2 Tr\n{:.3} w\n", stroke_pt))
            } else {
                ("", "", String::new())
            };
            ops.push_str(&format!(
                "{q_open}BT\n/{font_ref} {:.1} Tf\n{tc_op}{bold_ops}{:.1} {:.1} Td\n({safe}) Tj\nET\n{q_close}",
                style.size.val(), pos_x, base_y
            ));
        }
        FontScenario::Cidfont { char_to_gid, .. } => {
            if text.is_empty() { return; }
            let hex_str = text_to_hex_string(text, char_to_gid);
            ops.push_str(&format!(
                "BT\n/F1 {:.1} Tf\n{:.1} {:.1} Td\n{hex_str} Tj\nET\n",
                style.size.val(), pos_x, base_y
            ));
        }
        FontScenario::Multifont { fonts, per_font_char_to_gid, .. } => {
            if text.is_empty() { return; }
            let fi = style.font.as_ref()
                .and_then(|fl| fonts.iter().position(|(stored, _)| stored == fl))
                .unwrap_or(0);
            let hex_str = text_to_hex_string(text, &per_font_char_to_gid[fi]);
            ops.push_str(&format!(
                "BT\n/F{} {:.1} Tf\n{:.1} {:.1} Td\n{hex_str} Tj\nET\n",
                fi + 1, style.size.val(), pos_x, base_y
            ));
        }
    }
}

/// **P482/P485** — emit TextShaped PDF via operador `TJ` com avanços explícitos.
///
/// **P485**: cada glifo é emitido individualmente no array `TJ` com o número de
/// deslocamento derivado de `x_advance / units_per_em × 1000` (unidades TJ).
/// Isto garante posicionamento correcto mesmo quando GPOS/kerning altera os avanços
/// relativamente ao `hmtx`. Type1: fallback para `emit_text_pdf` (sem glyph IDs).
pub(super) fn emit_shaped_pdf(
    ops:          &mut String,
    pos_x:        f64,
    base_y:       f64,
    glyphs:       &[typst_core::entities::layout_types::ShapedGlyph],
    text:         &str,
    style:        &typst_core::entities::layout_types::TextStyle,
    scenario:     &FontScenario,
    units_per_em: u16,
) {
    if glyphs.is_empty() { return; }
    let upm = units_per_em as f64;
    match scenario {
        FontScenario::Type1 => {
            emit_text_pdf(ops, pos_x, base_y, text, style, scenario);
        }
        FontScenario::Cidfont { glyph_mapping, glyph_to_nominal, .. } => {
            ops.push_str(&format!(
                "BT\n/F1 {:.1} Tf\n{:.3} {:.3} Td\n[ ",
                style.size.val(), pos_x, base_y
            ));
            for g in glyphs {
                // P486 — x_offset: deslocar glifo sem alterar o avanço do próximo.
                if g.x_offset != 0 {
                    let xoff_tu = -(g.x_offset as f64 / upm * 1000.0);
                    ops.push_str(&format!("{:.0} ", xoff_tu));
                }
                // P520 — delta model: o CIDFont /W já fornece a largura nominal
                // (hmtx). O TJ deve conter apenas a diferença entre essa largura
                // declarada e o avanço real produzido pelo shaper.
                // P520 — delta model: o CIDFont /W já fornece a largura nominal
                // (hmtx). O TJ deve conter apenas a diferença entre essa largura
                // declarada e o avanço real produzido pelo shaper.
                let nominal = glyph_to_nominal.get(&g.glyph_id).copied()
                    .unwrap_or(g.x_advance);
                let advance_tu = (g.x_advance - nominal) as f64 / upm * 1000.0;
                let new_gid = if glyph_mapping.is_empty() {
                    g.glyph_id
                } else {
                    remap_glyph_id(g.glyph_id, glyph_mapping)
                };
                ops.push_str(&format!("<{:04X}> {:.0} ", new_gid, advance_tu));
            }
            ops.push_str("] TJ\nET\n");
        }
        FontScenario::Multifont { fonts, per_font_glyph_mapping, per_font_glyph_to_nominal, .. } => {
            let fi = style.font.as_ref()
                .and_then(|fl| fonts.iter().position(|(stored, _)| stored == fl))
                .unwrap_or(0);
            let glyph_mapping = &per_font_glyph_mapping[fi];
            let glyph_to_nominal = &per_font_glyph_to_nominal[fi];
            ops.push_str(&format!(
                "BT\n/F{} {:.1} Tf\n{:.3} {:.3} Td\n[ ",
                fi + 1, style.size.val(), pos_x, base_y
            ));
            for g in glyphs {
                // P486 — x_offset: deslocar glifo sem alterar o avanço do próximo.
                if g.x_offset != 0 {
                    let xoff_tu = -(g.x_offset as f64 / upm * 1000.0);
                    ops.push_str(&format!("{:.0} ", xoff_tu));
                }
                // P520 — delta model: ver Caso A em P520 / ADR-0108.
                let nominal = glyph_to_nominal.get(&g.glyph_id).copied()
                    .unwrap_or(g.x_advance);
                let advance_tu = (g.x_advance - nominal) as f64 / upm * 1000.0;
                let new_gid = if glyph_mapping.is_empty() {
                    g.glyph_id
                } else {
                    remap_glyph_id(g.glyph_id, glyph_mapping)
                };
                ops.push_str(&format!("<{:04X}> {:.0} ", new_gid, advance_tu));
            }
            ops.push_str("] TJ\nET\n");
        }
    }
}

/// **P281** — emit Glyph PDF dispatched por `FontScenario`.
///
/// P285 — Constrói o prefixo `r g b RG ` para emit de `FrameItem::Line`
/// com cor de stroke. `None` → string vazia (bit-exact pré-P285); `Some(c)`
/// → operador `RG` com componentes sRGB normalizados em [0.0, 1.0] (via
/// `to_rgba_f32`, paridade absoluta com `emit_stroke_paint` Solid path
/// linha 2236-2238). Formato canónico `{:.3} {:.3} {:.3} RG `, trailing
/// space inclusivo para concatenar imediatamente antes de `{:.3} w`.
pub(super) fn line_rg_prefix(color: &Option<typst_core::entities::layout_types::Color>) -> String {
    match color {
        None    => String::new(),
        Some(c) => {
            let (r, g, b, _) = c.to_rgba_f32();
            format!("{:.3} {:.3} {:.3} RG ", r, g, b)
        }
    }
}

/// Type1 silently ignored (sem TrueType embebida); CIDFont/Multifont
/// emitem `<{:04X}>` Identity-H via `/F1` (math fonts).
pub(super) fn emit_glyph_pdf(
    ops:      &mut String,
    pos_x:    f64,
    base_y:   f64,
    glyph_id: u16,
    size:     typst_core::entities::layout_types::Pt,
    scenario: &FontScenario,
) {
    match scenario {
        FontScenario::Type1 => {
            // Sem fonte TrueType → glyph_id sem significado. Ignored.
        }
        FontScenario::Cidfont { .. } | FontScenario::Multifont { .. } => {
            ops.push_str(&format!(
                "BT\n/F1 {:.1} Tf\n{:.1} {:.1} Td\n<{:04X}> Tj\nET\n",
                size.val(), pos_x, base_y, glyph_id
            ));
        }
    }
}

/// P263 — Emite operadores de stroke colour para um Paint (Solid ou Gradient).
///
/// Para `Paint::Solid(c)`: emit `r g b RG` literal P261 preservado.
/// Para `Paint::Gradient(g)`: emit `/Pattern CS /P{n} SCN` (set colour
/// space pattern + apply pattern).
pub(super) fn emit_stroke_paint(
    ops:            &mut String,
    paint:          &typst_core::entities::paint::Paint,
    thickness:      f64,
    effective_bbox: Option<typst_core::entities::layout_types::Rect>,
    pat_ptr_to_idx: &HashMap<DedupKey, usize>,
    pat_refs:       &[PatternRef],
) {
    use typst_core::entities::paint::Paint;
    match paint {
        Paint::Solid(c) => {
            let (r, g, b, _) = c.to_rgba_f32();
            ops.push_str(&format!("{:.3} {:.3} {:.3} RG\n{:.2} w\n", r, g, b, thickness));
        }
        Paint::Gradient(g) => {
            // P273.12 — lookup via DedupKey bbox-aware (substitui ptr-only).
            let key = dedup_key_for(g, effective_bbox);
            if let Some(&idx) = pat_ptr_to_idx.get(&key) {
                let r = &pat_refs[idx];
                ops.push_str(&format!("/Pattern CS\n/{} SCN\n{:.2} w\n", r.name, thickness));
            } else {
                // Fallback paranóide — gradient não registado em scan_all_gradients.
                let c = paint.to_color();
                let (r, g, b, _) = c.to_rgba_f32();
                ops.push_str(&format!("{:.3} {:.3} {:.3} RG\n{:.2} w\n", r, g, b, thickness));
            }
        }
        Paint::Tiling(_) => {
            // P395 — pattern fill de Tiling é scope-out ADR-0054 graded.
            // Fallback para cor representativa (Color body → cor; Image → preto).
            let c = paint.to_color();
            let (r, g, b, _) = c.to_rgba_f32();
            ops.push_str(&format!("{:.3} {:.3} {:.3} RG\n{:.2} w\n", r, g, b, thickness));
        }
    }
}

// **P281** — Stream builder unificado (substitui build_page_stream_type1/
// cidfont/multifont). Despacha Text/Glyph por `ctx.font_scenario`;
// Line/Image/Shape/Group são scenario-independent.
pub(super) fn build_page_stream(page: &Page, ctx: &PageContext) -> Vec<u8> {
    use typst_core::entities::geometry::ShapeKind;
    let mut ops = String::new();
    let page_height = page.height;

    for item in &page.items {
        match item {
            // P483 — path primário: glifos com shaping real.
            FrameItem::TextShaped { pos, glyphs, style, text, units_per_em } => {
                let pdf_y = page_height - pos.y.val();
                emit_shaped_pdf(&mut ops, pos.x.val(), pdf_y, glyphs, text.as_str(),
                                style, &ctx.font_scenario, *units_per_em);
            }
            // P483 — fallback: fonte não carregada, Type1, ou shaping indisponível.
            FrameItem::Text { pos, text, style } => {
                let pdf_y = page_height - pos.y.val();
                emit_text_pdf(&mut ops, pos.x.val(), pdf_y, text.as_str(),
                              style, &ctx.font_scenario);
            }
            FrameItem::Line { start, end, thickness, color } => {
                let x1 = start.x.val();
                let y1 = page_height - start.y.val();
                let x2 = end.x.val();
                let y2 = page_height - end.y.val();
                // P285: `color: Some(c)` injecta `r g b RG ` antes de `w`;
                // `color: None` produz string vazia → bit-exact pré-P285
                // (frac/sqrt/line sem stroke explícito preservam bytes
                // exactos vs baseline P284).
                let rg = line_rg_prefix(color);
                ops.push_str(&format!(
                    "q {}{:.3} w {:.1} {:.1} m {:.1} {:.1} l S Q\n",
                    rg, thickness, x1, y1, x2, y2
                ));
            }
            FrameItem::Glyph { pos, glyph_id, size, .. } => {
                let pdf_y = page_height - pos.y.val();
                emit_glyph_pdf(&mut ops, pos.x.val(), pdf_y, *glyph_id, *size,
                               &ctx.font_scenario);
            }
            FrameItem::Image { pos, data, width, height, .. } => {
                let ptr = Arc::as_ptr(data) as usize;
                if let Some(&idx) = ctx.ptr_to_idx.get(&ptr) {
                    // pos.y é o TOPO da imagem → canto inferior esquerdo no espaço PDF.
                    let pdf_y = page_height - pos.y.val() - height.val();
                    ops.push_str(&format!(
                        "q\n{:.3} 0 0 {:.3} {:.3} {:.3} cm\n/{} Do\nQ\n",
                        width.val(), height.val(), pos.x.val(), pdf_y,
                        ctx.img_refs[idx].name
                    ));
                }
            }
            FrameItem::Shape { pos, kind, width, height, fill, stroke, parent_bbox_at_emit } => {
                // Inverter eixo Y: layout tem Y crescente para baixo; PDF crescente para cima.
                let pdf_y = page_height - pos.y.val() - height;

                ops.push_str("q\n");

                // Cor de preenchimento (rg — RGB para fills).
                if let Some(c) = fill {
                    let (r, g, b, _) = c.to_rgba_f32();
                    ops.push_str(&format!("{:.3} {:.3} {:.3} rg\n", r, g, b));
                }

                // Cor e espessura do contorno (RG + w).
                if let Some(s) = stroke {
                    emit_stroke_paint(&mut ops, &s.paint, s.thickness,
                        *parent_bbox_at_emit, ctx.pat_ptr_to_idx, ctx.pat_refs);
                }

                // Path — depende do tipo de forma.
                match kind {
                    ShapeKind::Rect => {
                        ops.push_str(&format!("{:.2} {:.2} {:.2} {:.2} re\n",
                            pos.x.val(), pdf_y, width, height));
                    }
                    ShapeKind::RoundedRect { radii } => {
                        emit_rounded_rect_ops(&mut ops, pos.x.val(), pdf_y, *width, *height, radii);
                    }
                    ShapeKind::Ellipse => {
                        const KAPPA: f64 = 0.552_284_749_831;
                        let cx = pos.x.val() + width  / 2.0;
                        let cy = pdf_y       + height / 2.0;
                        let rx = width  / 2.0;
                        let ry = height / 2.0;
                        let ox = rx * KAPPA;
                        let oy = ry * KAPPA;
                        ops.push_str(&format!("{:.3} {:.3} m\n", cx, cy + ry));
                        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                            cx + ox, cy + ry, cx + rx, cy + oy, cx + rx, cy));
                        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                            cx + rx, cy - oy, cx + ox, cy - ry, cx, cy - ry));
                        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                            cx - ox, cy - ry, cx - rx, cy - oy, cx - rx, cy));
                        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                            cx - rx, cy + oy, cx - ox, cy + ry, cx, cy + ry));
                    }
                    ShapeKind::Line { dx, dy } => {
                        let start_offset_x = if *dx < 0.0 { *width }  else { 0.0 };
                        let end_offset_x   = if *dx < 0.0 { 0.0 }     else { *width };
                        let start_offset_y = if *dy > 0.0 { *height } else { 0.0 };
                        let end_offset_y   = if *dy > 0.0 { 0.0 }     else { *height };
                        let start_x = pos.x.val() + start_offset_x;
                        let start_y = pdf_y        + start_offset_y;
                        let end_x   = pos.x.val() + end_offset_x;
                        let end_y   = pdf_y        + end_offset_y;
                        ops.push_str(&format!("{:.3} {:.3} m\n", start_x, start_y));
                        ops.push_str(&format!("{:.3} {:.3} l\n", end_x,   end_y));
                    }
                    ShapeKind::Path(items) => {
                        use typst_core::entities::geometry::PathItem;
                        for item in items {
                            match item {
                                PathItem::MoveTo(p) => ops.push_str(&format!(
                                    "{:.2} {:.2} m\n",
                                    pos.x.val() + p.x.0,
                                    page_height - (pos.y.val() + p.y.0),
                                )),
                                PathItem::LineTo(p) => ops.push_str(&format!(
                                    "{:.2} {:.2} l\n",
                                    pos.x.val() + p.x.0,
                                    page_height - (pos.y.val() + p.y.0),
                                )),
                                PathItem::CubicTo(p1, p2, p3) => ops.push_str(&format!(
                                    "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c\n",
                                    pos.x.val() + p1.x.0, page_height - (pos.y.val() + p1.y.0),
                                    pos.x.val() + p2.x.0, page_height - (pos.y.val() + p2.y.0),
                                    pos.x.val() + p3.x.0, page_height - (pos.y.val() + p3.y.0),
                                )),
                                PathItem::ClosePath => ops.push_str("h\n"),
                            }
                        }
                    }
                }

                match (fill.is_some(), stroke.is_some()) {
                    (true,  true)  => ops.push_str("B\n"),
                    (true,  false) => ops.push_str("f\n"),
                    (false, true)  => ops.push_str("S\n"),
                    (false, false) => {}
                }

                ops.push_str("Q\n");
            }
            FrameItem::Group { pos, matrix, clip_mask, inner_width, inner_height, items } => {
                let pdf_y = page_height - pos.y.val();

                // O layouter usa Y-down; o PDF usa Y-up.
                // Os componentes de cisalhamento b e c são invertidos para corrigir a paridade.
                ops.push_str("q\n");
                ops.push_str(&format!(
                    "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} cm\n",
                    matrix.a,
                    -matrix.b,
                    -matrix.c,
                    matrix.d,
                    pos.x.val() + matrix.tx,
                    pdf_y       - matrix.ty,
                ));

                if let Some(mask) = clip_mask {
                    emit_shape_path_local(&mut ops, mask, *inner_width, *inner_height);
                    ops.push_str("W n\n");
                }

                // P273.13 / P278 — Group bbox via helper consolidado.
                let group_bbox = group_bbox_from_fields(*pos, *inner_width, *inner_height);
                for child in items {
                    // P281 — recurse com mesmo ctx (substitui cascade
                    // de 6 params P279).
                    draw_item_local(&mut ops, child, Some(group_bbox), ctx);
                }
                ops.push_str("Q\n");
            }
            // **P425-A7**: FrameItem::Link transportado como Group sem annotation
            // URI por enquanto. Emissão de /Annot requer decisão arquitetural sobre
            // bbox/posição do Link.
            FrameItem::Link { items, .. } => {
                for child in items {
                    draw_item_local(&mut ops, child, None, ctx);
                }
            }
        }
    }

    ops.into_bytes()
}

/// Emite os operadores de path de uma forma no espaço LOCAL de um Group.
///
/// NÃO usa page_height. A matriz `cm` já inverteu o eixo Y.
/// Chamada para emitir clip_mask antes de `W n`.
pub(super) fn emit_shape_path_local(ops: &mut String, kind: &typst_core::entities::geometry::ShapeKind, width: f64, height: f64) {
    use typst_core::entities::geometry::{PathItem, ShapeKind};
    match kind {
        ShapeKind::Rect => {
            ops.push_str(&format!("0.00 {:.2} {:.2} {:.2} re\n", -height, width, height));
        }
        ShapeKind::RoundedRect { radii } => {
            // P242 (M9d / M7+5) — Bezier 4 corners path em espaço local
            // (origem 0,0; Y invertido pela matriz cm). Coords iguais a
            // ShapeKind::Rect: (0, -height, width, height).
            emit_rounded_rect_ops(ops, 0.0, -height, width, height, radii);
        }
        ShapeKind::Path(items) => {
            for item in items {
                match item {
                    PathItem::MoveTo(p) => ops.push_str(&format!(
                        "{:.2} {:.2} m\n", p.x.0, -p.y.0,
                    )),
                    PathItem::LineTo(p) => ops.push_str(&format!(
                        "{:.2} {:.2} l\n", p.x.0, -p.y.0,
                    )),
                    PathItem::CubicTo(p1, p2, p3) => ops.push_str(&format!(
                        "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c\n",
                        p1.x.0, -p1.y.0, p2.x.0, -p2.y.0, p3.x.0, -p3.y.0,
                    )),
                    PathItem::ClosePath => ops.push_str("h\n"),
                }
            }
        }
        _ => {}
    }
}

/// **P242 (M9d / M7+5)** — emite operadores PDF para um rectângulo com
/// cantos arredondados via Bezier 4 corners (paridade vanilla
/// `typst-pdf/.../shape.rs::draw_rounded_rect`).
///
/// Coordenadas em sistema PDF (Y crescente para cima). `(x, y)` é o
/// canto inferior-esquerdo; `w` largura; `h` altura positivos. `radii`
/// em `Corners<Length>` (top_left/top_right/bottom_right/bottom_left
/// sentido horário começando top-left).
///
/// **Bezier kappa = 0.552_284_749_831** (paridade `ShapeKind::Ellipse`
/// neste mesmo ficheiro). Quarto de círculo aproximado com 2 control
/// points por canto.
///
/// **Output**: sequência `m` (move) + `l` (line) + `c` (cubic) + `h`
/// (closePath) — formato compatível com `B`/`S`/`W n` paint operators.
pub(super) fn emit_rounded_rect_ops(
    ops: &mut String,
    x: f64, y: f64, w: f64, h: f64,
    radii: &typst_core::entities::corners::Corners<typst_core::entities::layout_types::Length>,
) {
    const K: f64 = 0.552_284_749_831;
    // Resolver Length → f64 pt (em = 0 para clip_mask; valores absolutos).
    // Clamp cada raio a metade da menor dimensão (paridade vanilla evita
    // overflow geométrico).
    let max_r = (w.min(h)) / 2.0;
    let tl = radii.top_left.abs.0.clamp(0.0, max_r);
    let tr = radii.top_right.abs.0.clamp(0.0, max_r);
    let br = radii.bottom_right.abs.0.clamp(0.0, max_r);
    let bl = radii.bottom_left.abs.0.clamp(0.0, max_r);

    // Sentido horário em PDF coords (Y para cima). Sequência:
    // start top-left edge → top edge → top-right corner → right edge →
    // bottom-right corner → bottom edge → bottom-left corner → left edge →
    // top-left corner → close.
    let x_left   = x;
    let x_right  = x + w;
    let y_top    = y + h;
    let y_bottom = y;

    // MoveTo: começa no início da edge top (após canto top-left).
    ops.push_str(&format!("{:.3} {:.3} m\n", x_left + tl, y_top));
    // Linha top edge.
    ops.push_str(&format!("{:.3} {:.3} l\n", x_right - tr, y_top));
    // Cubic top-right corner.
    if tr > 0.0 {
        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
            x_right - tr + tr * K, y_top,
            x_right,               y_top - tr + tr * K,
            x_right,               y_top - tr));
    }
    // Linha right edge.
    ops.push_str(&format!("{:.3} {:.3} l\n", x_right, y_bottom + br));
    // Cubic bottom-right corner.
    if br > 0.0 {
        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
            x_right,               y_bottom + br - br * K,
            x_right - br + br * K, y_bottom,
            x_right - br,          y_bottom));
    }
    // Linha bottom edge.
    ops.push_str(&format!("{:.3} {:.3} l\n", x_left + bl, y_bottom));
    // Cubic bottom-left corner.
    if bl > 0.0 {
        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
            x_left + bl - bl * K, y_bottom,
            x_left,               y_bottom + bl - bl * K,
            x_left,               y_bottom + bl));
    }
    // Linha left edge.
    ops.push_str(&format!("{:.3} {:.3} l\n", x_left, y_top - tl));
    // Cubic top-left corner.
    if tl > 0.0 {
        ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
            x_left,               y_top - tl + tl * K,
            x_left + tl - tl * K, y_top,
            x_left + tl,          y_top));
    }
    // Fecha o path.
    ops.push_str("h\n");
}

/// Desenha um `FrameItem` em espaço LOCAL (após `cm`).
///
/// Diferença de `draw_item_global`: não subtrai `page_height` — a matriz `cm`
/// já aplicou a transformação e a inversão Y. Os filhos usam `pos.y.0` directamente.
pub(super) fn draw_item_local(
    ops: &mut String,
    item: &FrameItem,
    // P273.13 — parameter threading paralelo P273.10 scan_all_gradients.walk
    // (Decisão 1α Fase A). Inner-wins via .or().
    parent_bbox_override: Option<typst_core::entities::layout_types::Rect>,
    // P281 — substitui cascade P279 (ptr_to_idx, img_refs, pat_ptr_to_idx,
    // pat_refs) por agregador unificado `&PageContext`. Text/Glyph arm
    // dispatch via `ctx.font_scenario`.
    ctx: &PageContext,
) {
    use typst_core::entities::geometry::ShapeKind;
    use typst_core::entities::layout_types::{FrameItem, Pt, Rect};
    match item {
        FrameItem::Shape { pos, kind, width, height, fill, stroke, parent_bbox_at_emit } => {
            let local_y = pos.y.0;
            ops.push_str("q\n");
            if let Some(c) = fill {
                let (r, g, b, _) = c.to_rgba_f32();
                ops.push_str(&format!("{:.3} {:.3} {:.3} rg\n", r, g, b));
            }
            if let Some(s) = stroke {
                let effective_bbox = parent_bbox_at_emit.or(parent_bbox_override);
                emit_stroke_paint(ops, &s.paint, s.thickness,
                    effective_bbox, ctx.pat_ptr_to_idx, ctx.pat_refs);
            }
            match kind {
                ShapeKind::Rect => {
                    ops.push_str(&format!("{:.2} {:.2} {:.2} {:.2} re\n",
                        pos.x.0, local_y, width, height));
                }
                ShapeKind::RoundedRect { radii } => {
                    // P242 — Bezier 4 corners em espaço local.
                    emit_rounded_rect_ops(&mut *ops, pos.x.0, local_y, *width, *height, radii);
                }
                ShapeKind::Ellipse => {
                    const KAPPA: f64 = 0.552_284_749_831;
                    let cx = pos.x.0 + width  / 2.0;
                    let cy = local_y  + height / 2.0;
                    let rx = width  / 2.0;
                    let ry = height / 2.0;
                    let ox = rx * KAPPA;
                    let oy = ry * KAPPA;
                    ops.push_str(&format!("{:.3} {:.3} m\n", cx, cy + ry));
                    ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx + ox, cy + ry, cx + rx, cy + oy, cx + rx, cy));
                    ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx + rx, cy - oy, cx + ox, cy - ry, cx, cy - ry));
                    ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx - ox, cy - ry, cx - rx, cy - oy, cx - rx, cy));
                    ops.push_str(&format!("{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx - rx, cy + oy, cx - ox, cy + ry, cx, cy + ry));
                }
                ShapeKind::Line { dx, dy } => {
                    let start_offset_x = if *dx < 0.0 { *width }  else { 0.0 };
                    let end_offset_x   = if *dx < 0.0 { 0.0 }     else { *width };
                    let start_offset_y = if *dy > 0.0 { *height } else { 0.0 };
                    let end_offset_y   = if *dy > 0.0 { 0.0 }     else { *height };
                    ops.push_str(&format!("{:.3} {:.3} m\n",
                        pos.x.0 + start_offset_x, local_y + start_offset_y));
                    ops.push_str(&format!("{:.3} {:.3} l\n",
                        pos.x.0 + end_offset_x, local_y + end_offset_y));
                }
                ShapeKind::Path(items) => {
                    use typst_core::entities::geometry::PathItem;
                    for item in items {
                        match item {
                            PathItem::MoveTo(p) => ops.push_str(&format!(
                                "{:.2} {:.2} m\n",
                                pos.x.0 + p.x.0, -(local_y + p.y.0),
                            )),
                            PathItem::LineTo(p) => ops.push_str(&format!(
                                "{:.2} {:.2} l\n",
                                pos.x.0 + p.x.0, -(local_y + p.y.0),
                            )),
                            PathItem::CubicTo(p1, p2, p3) => ops.push_str(&format!(
                                "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c\n",
                                pos.x.0 + p1.x.0, -(local_y + p1.y.0),
                                pos.x.0 + p2.x.0, -(local_y + p2.y.0),
                                pos.x.0 + p3.x.0, -(local_y + p3.y.0),
                            )),
                            PathItem::ClosePath => ops.push_str("h\n"),
                        }
                    }
                }
            }
            match (fill.is_some(), stroke.is_some()) {
                (true,  true)  => ops.push_str("B\n"),
                (true,  false) => ops.push_str("f\n"),
                (false, true)  => ops.push_str("S\n"),
                (false, false) => {}
            }
            ops.push_str("Q\n");
        }
        // P273.13 — arm Group; nested Groups via recursão.
        // P281 — cascade único `&PageContext` (substitui 4 params P279).
        FrameItem::Group { pos, inner_width, inner_height, items, .. } => {
            let group_bbox = group_bbox_from_fields(*pos, *inner_width, *inner_height);
            for child in items {
                draw_item_local(ops, child, Some(group_bbox), ctx);
            }
        }
        // P279 — Image em Group emit local.
        FrameItem::Image { pos, data, width, height, .. } => {
            let ptr = std::sync::Arc::as_ptr(data) as usize;
            if let Some(&idx) = ctx.ptr_to_idx.get(&ptr) {
                // Local emit: pos.x/pos.y são coords locais (após Group cm).
                ops.push_str(&format!(
                    "q\n{:.3} 0 0 {:.3} {:.3} {:.3} cm\n/{} Do\nQ\n",
                    width.val(), height.val(), pos.x.0, pos.y.0,
                    ctx.img_refs[idx].name
                ));
            }
        }
        // **P281** — Text/Glyph/Line arms real (substituem stubs P278/P279).
        // P483 — path primário TextShaped; Text = fallback.
        // Local emit: `pos.y.0` directo (matriz `cm` do Group já inverteu Y).
        FrameItem::TextShaped { pos, glyphs, style, text, units_per_em } => {
            emit_shaped_pdf(ops, pos.x.0, pos.y.0, glyphs, text.as_str(),
                            style, &ctx.font_scenario, *units_per_em);
        }
        FrameItem::Text { pos, text, style } => {
            emit_text_pdf(ops, pos.x.0, pos.y.0, text.as_str(),
                          style, &ctx.font_scenario);
        }
        FrameItem::Glyph { pos, glyph_id, size, .. } => {
            emit_glyph_pdf(ops, pos.x.0, pos.y.0, *glyph_id, *size,
                           &ctx.font_scenario);
        }
        FrameItem::Line { start, end, thickness, color } => {
            // Local emit: coords locais (após Group `cm`). Y já invertido
            // pela matriz cm → preserved bit-exact com top-level emit
            // estructura (`q w m l S Q`) mas sem `page_height -` subtract.
            // P285: paridade simétrica com top-level — `color: Some(c)`
            // injecta `r g b RG ` antes de `w`; `color: None` preserva
            // bit-exact (Win arquitectural P281 — helpers unificados
            // garantem que mudança em ambos os emit ocorre simetricamente).
            let rg = line_rg_prefix(color);
            ops.push_str(&format!(
                "q {}{:.3} w {:.1} {:.1} m {:.1} {:.1} l S Q\n",
                rg, thickness, start.x.0, start.y.0, end.x.0, end.y.0
            ));
        }
        // **P425-A7**: transporte recursivo de Link no espaço local do Group.
        FrameItem::Link { items, .. } => {
            for child in items {
                draw_item_local(ops, child, parent_bbox_override, ctx);
            }
        }
    }
}

#[cfg(test)]
mod stream_tests {
    use super::*;
    use typst_core::entities::layout_types::{ShapedGlyph, TextStyle};

    fn glyph(glyph_id: u16, x_advance: i32) -> ShapedGlyph {
        ShapedGlyph { glyph_id, x_advance, x_offset: 0, y_offset: 0, cluster: 0, char_code: 'A' }
    }

    fn glyph_xoff(glyph_id: u16, x_advance: i32, x_offset: i32) -> ShapedGlyph {
        ShapedGlyph { glyph_id, x_advance, x_offset, y_offset: 0, cluster: 0, char_code: 'A' }
    }

    #[test]
    fn p485_emit_shaped_cidfont_usa_tj() {
        let mut ops = String::new();
        let glyphs = vec![glyph(0x0041, 600)];
        let style = TextStyle::default();
        let scenario = FontScenario::Cidfont {
            char_to_gid: &std::collections::HashMap::new(),
            glyph_mapping: &std::collections::HashMap::new(),
            glyph_to_nominal: &std::collections::HashMap::new(),
        };
        emit_shaped_pdf(&mut ops, 72.0, 770.0, &glyphs, "A", &style, &scenario, 1000);
        assert!(ops.contains("TJ"), "P485: CIDFont deve usar TJ, não Tj");
        assert!(!ops.contains("] Tj"), "P485: não deve conter Tj no path CIDFont");
        assert!(ops.contains("<0041>"), "P485: deve conter hex do glyph_id");
    }

    #[test]
    fn p485_emit_shaped_advance_calculado() {
        // P520 — delta model: sem mapa de larguras nominais, o fallback é
        // nominal=x_advance, logo o delta TJ é 0.
        let mut ops = String::new();
        let glyphs = vec![glyph(0x0042, 600)];
        let style = TextStyle::default();
        let scenario = FontScenario::Cidfont {
            char_to_gid: &std::collections::HashMap::new(),
            glyph_mapping: &std::collections::HashMap::new(),
            glyph_to_nominal: &std::collections::HashMap::new(),
        };
        emit_shaped_pdf(&mut ops, 0.0, 0.0, &glyphs, "B", &style, &scenario, 1000);
        assert!(ops.contains("<0042> 0"), "P485: fallback nominal=x_advance → delta TJ = 0");
    }

    #[test]
    fn p485_emit_shaped_type1_nao_usa_tj() {
        // Type1 faz fallback para emit_text_pdf (string plana, sem TJ)
        let mut ops = String::new();
        let glyphs = vec![glyph(0x0043, 600)];
        let style = TextStyle::default();
        emit_shaped_pdf(&mut ops, 0.0, 0.0, &glyphs, "C", &style, &FontScenario::Type1, 1000);
        assert!(!ops.contains("TJ"), "P485: Type1 não deve usar TJ");
    }

    #[test]
    fn p485_emit_shaped_vazio_sem_output() {
        let mut ops = String::new();
        emit_shaped_pdf(&mut ops, 0.0, 0.0, &[], "x", &TextStyle::default(),
                        &FontScenario::Type1, 1000);
        assert!(ops.is_empty(), "P485: glyphs vazios → sem output");
    }

    #[test]
    fn p486_emit_x_offset_zero_equivale_p485() {
        // x_offset=0 → output idêntico ao P485 (sem número antes do GID)
        let mut ops = String::new();
        let glyphs = vec![glyph_xoff(0x0042, 600, 0)];
        let style = TextStyle::default();
        let scenario = FontScenario::Cidfont {
            char_to_gid: &std::collections::HashMap::new(),
            glyph_mapping: &std::collections::HashMap::new(),
            glyph_to_nominal: &std::collections::HashMap::new(),
        };
        emit_shaped_pdf(&mut ops, 0.0, 0.0, &glyphs, "B", &style, &scenario, 1000);
        assert!(ops.contains("<0042>"), "P486: GID presente");
        assert!(ops.contains("<0042> 0"), "P486: delta 0 igual a P485");
        let after_bracket = ops.split("[ ").nth(1).unwrap_or("");
        assert!(after_bracket.starts_with("<0042>"), "P486: x_offset=0 → sem ajuste antes do GID");
    }

    #[test]
    fn p486_emit_x_offset_nonzero_aplica_ajuste() {
        // x_offset=-50, upm=1000 → -((-50)/1000*1000) = 50 → "50 " antes do GID
        let mut ops = String::new();
        let glyphs = vec![glyph_xoff(0x0043, 600, -50)];
        let style = TextStyle::default();
        let scenario = FontScenario::Cidfont {
            char_to_gid: &std::collections::HashMap::new(),
            glyph_mapping: &std::collections::HashMap::new(),
            glyph_to_nominal: &std::collections::HashMap::new(),
        };
        emit_shaped_pdf(&mut ops, 0.0, 0.0, &glyphs, "C", &style, &scenario, 1000);
        let after_bracket = ops.split("[ ").nth(1).unwrap_or("");
        assert!(after_bracket.starts_with("50 "), "P486: x_offset=-50 → '50 ' antes do GID");
        assert!(ops.contains("<0043>"), "P486: GID presente");
    }

    #[test]
    fn p486_emit_x_offset_positivo() {
        // x_offset=30, upm=1000 → -(30/1000*1000) = -30 → "-30 " antes do GID
        let mut ops = String::new();
        let glyphs = vec![glyph_xoff(0x0044, 600, 30)];
        let style = TextStyle::default();
        let scenario = FontScenario::Cidfont {
            char_to_gid: &std::collections::HashMap::new(),
            glyph_mapping: &std::collections::HashMap::new(),
            glyph_to_nominal: &std::collections::HashMap::new(),
        };
        emit_shaped_pdf(&mut ops, 0.0, 0.0, &glyphs, "D", &style, &scenario, 1000);
        let after_bracket = ops.split("[ ").nth(1).unwrap_or("");
        assert!(after_bracket.starts_with("-30 "), "P486: x_offset=30 → '-30 ' antes do GID");
        assert!(ops.contains("<0044>"), "P486: GID presente");
    }

    // P520 — teste do delta model com largura nominal explícita.
    #[test]
    fn p520_emit_shaped_kerning_delta() {
        // Glifo A: x_advance=599, largura nominal (hmtx)=639, upm=1000.
        // delta TJ = (599 - 639) / 1000 * 1000 = -40.
        let mut ops = String::new();
        let glyphs = vec![
            ShapedGlyph { glyph_id: 0x0041, x_advance: 599, x_offset: 0, y_offset: 0, cluster: 0, char_code: 'A' },
        ];
        let style = TextStyle::default();
        let mut nominal = std::collections::HashMap::new();
        nominal.insert(0x0041, 639);
        let scenario = FontScenario::Cidfont {
            char_to_gid: &std::collections::HashMap::new(),
            glyph_mapping: &std::collections::HashMap::new(),
            glyph_to_nominal: &nominal,
        };
        emit_shaped_pdf(&mut ops, 0.0, 0.0, &glyphs, "A", &style, &scenario, 1000);
        assert!(ops.contains("<0041> -40"), "P520: kerning positivo → delta negativo (aproxima)");
    }
}


