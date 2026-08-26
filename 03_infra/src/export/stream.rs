//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/stream.md
//! @prompt-hash 412e3910
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

use super::pdf_defaults;
use typst_core::entities::font_book::FontVariant;
use typst_core::entities::font_list::FontList;
use typst_core::entities::font_variations::FontVariations;
use typst_core::entities::layout_types::{FrameItem, Page, TransformMatrix};

use crate::font_variant::text_style_to_font_variant;

use super::{
    dedup_key_for, escape_pdf_string, group_bbox_from_fields, remap_glyph_id,
    text_to_hex_string, DedupKey, ImageRef, PatternRef, PdfTags, StreamMode,
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
        /// **P941** — glifos bitmap (CBDT) como imagens XObject. Se `Some`,
        /// o run é desenhado como imagens (fonte 100% bitmap, não embutida).
        bitmap: Option<&'a HashMap<u16, super::bitmap_glyphs::BitmapGlyphRef>>,
    },
    /// Multifont Identity-H com selecção `/F{fi+1}` por `(style.font, variant)`.
    Multifont {
        fonts: &'a [((FontList, FontVariant, FontVariations), Vec<u8>)],
        per_font_char_to_gid: &'a [HashMap<char, u16>],
        /// P516 — mapa old → new glyph ID por fonte (vazio se não subsetada).
        per_font_glyph_mapping: &'a [HashMap<u16, u16>],
        /// P520 — largura nominal por old glyph ID, por fonte.
        per_font_glyph_to_nominal: &'a [HashMap<u16, i32>],
        /// **P906** — mapa reverso glyph_id→char por fonte (mesmo `build_
        /// math_glyph_reverse_map` já usado no subsetting DEBT-9/P45,
        /// `builder.rs:988`), agora também partilhado com `emit_glyph_pdf`
        /// para seleccionar `/F{fi+1}` correcto para `FrameItem::Glyph`
        /// (glifos de esticamento matemático sem `Tf` explícito) — antes
        /// hardcodava sempre `/F1`. Ver `export/stream.md` §P906.
        per_font_glyph_reverse: &'a [HashMap<u16, char>],
        /// **P941** — glifos bitmap por fonte; `Some` na entrada se a fonte
        /// é 100% bitmap (não embutida) e o run é desenhado como imagens.
        per_font_bitmap:
            &'a [Option<HashMap<u16, super::bitmap_glyphs::BitmapGlyphRef>>],
    },
}

/// **P281** — agregador de contexto para emit unificado.
pub(crate) struct PageContext<'a> {
    pub ptr_to_idx: &'a HashMap<usize, usize>,
    pub img_refs: &'a [ImageRef],
    pub pat_ptr_to_idx: &'a HashMap<DedupKey, usize>,
    pub pat_refs: &'a [PatternRef],
    pub font_scenario: FontScenario<'a>,
    /// **P956** — modo de emissão de texto (ADR-0126): `Verbose` (envelope
    /// vanilla-espelhado, novo padrão) ou `Compact` (formato Passo 20,
    /// byte-inalterado). Dispatch em `draw_item_top`/`draw_item_local`.
    pub mode: StreamMode,
    /// **P1140.6** — tagging e MCIDs pela identidade de cada envelope.
    pub tags: PdfTags,
    pub semantic_mcids: HashMap<usize, usize>,
    /// **P980/P983** — caminho do oráculo de paridade de operador
    /// (`--oracle-pdf`; `prompts/infra/export/oracle.md`). Quando activo,
    /// itens `style.math` não participam no agrupamento de runs de P979
    /// (o vanilla nunca funde glifos math — `oracle.md` §P983).
    /// `false` no caminho normal.
    pub oracle: bool,
}

impl<'a> PageContext<'a> {
    pub(crate) fn type1(
        ptr_to_idx: &'a HashMap<usize, usize>,
        img_refs: &'a [ImageRef],
        pat_ptr_to_idx: &'a HashMap<DedupKey, usize>,
        pat_refs: &'a [PatternRef],
        mode: StreamMode,
    ) -> Self {
        Self {
            ptr_to_idx,
            img_refs,
            pat_ptr_to_idx,
            pat_refs,
            font_scenario: FontScenario::Type1,
            mode,
            tags: PdfTags::Disabled,
            semantic_mcids: HashMap::new(),
            oracle: false,
        }
    }

    /// **P980/P983** — activa o caminho do oráculo neste contexto.
    pub(crate) fn with_oracle(mut self, oracle: bool) -> Self {
        self.oracle = oracle;
        self
    }

    pub(crate) fn with_pdf_tags(mut self, tags: PdfTags, page: &Page) -> Self {
        self.tags = tags;
        if tags == PdfTags::Enabled {
            fn collect(
                item: &FrameItem,
                next: &mut usize,
                out: &mut HashMap<usize, usize>,
            ) {
                match item {
                    FrameItem::Semantic { kind, items, .. } => {
                        if *kind
                            == typst_core::entities::layout_types::SemanticKind::Formula
                        {
                            out.insert(item as *const FrameItem as usize, *next);
                            *next += 1;
                        }
                        for child in items {
                            collect(child, next, out);
                        }
                    }
                    FrameItem::Group { items, .. } | FrameItem::Link { items, .. } => {
                        for child in items {
                            collect(child, next, out);
                        }
                    }
                    _ => {}
                }
            }
            let mut next = 0;
            for item in &page.items {
                collect(item, &mut next, &mut self.semantic_mcids);
            }
        }
        self
    }

    pub(crate) fn cidfont(
        ptr_to_idx: &'a HashMap<usize, usize>,
        img_refs: &'a [ImageRef],
        pat_ptr_to_idx: &'a HashMap<DedupKey, usize>,
        pat_refs: &'a [PatternRef],
        char_to_gid: &'a HashMap<char, u16>,
        glyph_mapping: &'a HashMap<u16, u16>,
        glyph_to_nominal: &'a HashMap<u16, i32>,
        bitmap: Option<&'a HashMap<u16, super::bitmap_glyphs::BitmapGlyphRef>>,
        mode: StreamMode,
    ) -> Self {
        Self {
            ptr_to_idx,
            img_refs,
            pat_ptr_to_idx,
            pat_refs,
            font_scenario: FontScenario::Cidfont {
                char_to_gid,
                glyph_mapping,
                glyph_to_nominal,
                bitmap,
            },
            mode,
            tags: PdfTags::Disabled,
            semantic_mcids: HashMap::new(),
            oracle: false,
        }
    }

    pub(crate) fn multifont(
        ptr_to_idx: &'a HashMap<usize, usize>,
        img_refs: &'a [ImageRef],
        pat_ptr_to_idx: &'a HashMap<DedupKey, usize>,
        pat_refs: &'a [PatternRef],
        fonts: &'a [((FontList, FontVariant, FontVariations), Vec<u8>)],
        per_font_char_to_gid: &'a [HashMap<char, u16>],
        per_font_glyph_mapping: &'a [HashMap<u16, u16>],
        per_font_glyph_to_nominal: &'a [HashMap<u16, i32>],
        per_font_glyph_reverse: &'a [HashMap<u16, char>],
        per_font_bitmap: &'a [Option<
            HashMap<u16, super::bitmap_glyphs::BitmapGlyphRef>,
        >],
        mode: StreamMode,
    ) -> Self {
        Self {
            ptr_to_idx,
            img_refs,
            pat_ptr_to_idx,
            pat_refs,
            font_scenario: FontScenario::Multifont {
                fonts,
                per_font_char_to_gid,
                per_font_glyph_mapping,
                per_font_glyph_to_nominal,
                per_font_glyph_reverse,
                per_font_bitmap,
            },
            mode,
            tags: PdfTags::Disabled,
            semantic_mcids: HashMap::new(),
            oracle: false,
        }
    }
}

/// P530 — devolve o índice da fonte embutida que corresponde ao
/// `(FontList, FontVariant)` derivado do `TextStyle`.
pub(crate) fn font_index_for_style(
    fonts: &[((FontList, FontVariant, FontVariations), Vec<u8>)],
    style: &typst_core::entities::layout_types::TextStyle,
) -> usize {
    let variant = text_style_to_font_variant(style);
    // P836 — a chave inclui as variações explícitas (derivadas não
    // bastam: dois runs com o mesmo FontVariant podem pedir eixos
    // explícitos distintos).
    let variations = style.variations.clone().unwrap_or_default();
    style
        .font
        .as_ref()
        .and_then(|fl| {
            fonts
                .iter()
                .position(|((stored_fl, stored_variant, stored_vars), _)| {
                    stored_fl == fl
                        && stored_variant == &variant
                        && stored_vars == &variations
                })
        })
        .unwrap_or(0)
}

/// **P281** — emit Text PDF dispatched por `FontScenario`.
///
/// `base_y` é a coordenada Y final em PDF (top-level: `page_height -
/// pos.y`; local: `pos.y.0` directo após Group `cm`).
pub(super) fn emit_text_pdf(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    text: &str,
    style: &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
) {
    let rg = fill_rg_prefix(&style.fill);
    match scenario {
        FontScenario::Type1 => {
            let safe = escape_pdf_string(text);
            if safe.is_empty() {
                return;
            }
            let font_ref = match (style.bold, style.italic) {
                (true, _) => "F2",
                (false, true) => "F3",
                (false, false) => "F1",
            };
            let tracking_pt =
                style.tracking.map(|t| t.resolve_pt(style.size.val())).unwrap_or(0.0);
            let tc_op = if tracking_pt.abs() > f64::EPSILON {
                format!("{:.2} Tc\n", tracking_pt)
            } else {
                String::new()
            };
            let stroke_pt = style.faux_bold_stroke_pt(pdf_defaults::FAUX_BOLD_K);
            let (q_open, q_close, bold_ops) = if stroke_pt > f64::EPSILON {
                ("q\n", "Q\n", format!("2 Tr\n{:.3} w\n", stroke_pt))
            } else {
                ("", "", String::new())
            };
            ops.push_str(&format!(
                "{rg}{q_open}BT\n/{font_ref} {:.1} Tf\n{tc_op}{bold_ops}{:.1} {:.1} Td\n({safe}) Tj\nET\n{q_close}",
                style.size.val(), pos_x, base_y
            ));
        }
        FontScenario::Cidfont { char_to_gid, .. } => {
            if text.is_empty() {
                return;
            }
            let hex_str = text_to_hex_string(text, char_to_gid);
            ops.push_str(&format!(
                "{rg}BT\n/F1 {:.1} Tf\n{:.1} {:.1} Td\n{hex_str} Tj\nET\n",
                style.size.val(),
                pos_x,
                base_y
            ));
        }
        FontScenario::Multifont { fonts, per_font_char_to_gid, .. } => {
            if text.is_empty() {
                return;
            }
            let fi = font_index_for_style(fonts, style);
            let hex_str = text_to_hex_string(text, &per_font_char_to_gid[fi]);
            ops.push_str(&format!(
                "{rg}BT\n/F{} {:.1} Tf\n{:.1} {:.1} Td\n{hex_str} Tj\nET\n",
                fi + 1,
                style.size.val(),
                pos_x,
                base_y
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
/// **P941** — desenha um run de glifos bitmap (CBDT) como imagens XObject.
/// **P956** — inalterado nos dois modos (Verbose e Compact): é desenho de
/// imagem (`q … cm /ImN Do Q`), não emissão de texto — o envelope verbose
/// de ADR-0126 não se aplica.
///
/// Cada glifo é emitido como `q w 0 0 h x y cm /ImN Do Q` na posição calculada
/// a partir do cursor de texto; o cursor avança pelo `x_advance` do shaper,
/// pelo que o espaço na linha é idêntico ao de um glifo de fonte. A fórmula de
/// posicionamento (sinal de `y`) segue krilla `text/glyph/bitmap.rs` e foi
/// verificada visualmente com `mutool draw` (ver `builder.md` §P941).
fn emit_bitmap_glyph_draws(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    glyphs: &[typst_core::entities::layout_types::ShapedGlyph],
    style: &typst_core::entities::layout_types::TextStyle,
    bitmap: &HashMap<u16, super::bitmap_glyphs::BitmapGlyphRef>,
    units_per_em: u16,
) {
    let upm = units_per_em as f64;
    let size = style.size.val();
    let mut cur_x = pos_x;
    for g in glyphs {
        let x_off_pt = g.x_offset as f64 / upm * size;
        if let Some(bmp) = bitmap.get(&g.glyph_id) {
            let scale = size / bmp.pixels_per_em as f64;
            let w_pt = bmp.width as f64 * scale;
            let h_pt = bmp.height as f64 * scale;
            let x_pt = cur_x + x_off_pt + bmp.bearing_x as f64 * scale;
            // bearing_y (RasterGlyphImage.y) é o offset do topo do bitmap
            // acima da linha de base, em pixels; negativo em NotoColorEmoji
            // (-27) porque o bitmap desce ligeiramente abaixo da linha.
            let y_bottom = base_y + bmp.bearing_y as f64 * scale;
            ops.push_str(&format!(
                "q\n{w_pt:.3} 0 0 {h_pt:.3} {x_pt:.3} {y_bottom:.3} cm\n/{} Do\nQ\n",
                bmp.name
            ));
        }
        cur_x += g.x_advance as f64 / upm * size;
    }
}

/// **P956** — entradas do array `TJ` partilhadas pelos modos compact e
/// verbose: delta model P520/P548 (nominal − x_advance), `x_offset` P486 e
/// remap de subsetting P516. Produz os mesmos bytes que o loop inline
/// pré-P956 do caminho compact.
fn push_tj_glyph_entries(
    ops: &mut String,
    glyphs: &[typst_core::entities::layout_types::ShapedGlyph],
    glyph_mapping: &HashMap<u16, u16>,
    glyph_to_nominal: &HashMap<u16, i32>,
    upm: f64,
) {
    for g in glyphs {
        // P486 — x_offset: deslocar glifo sem alterar o avanço do próximo.
        if g.x_offset != 0 {
            let xoff_tu = -(g.x_offset as f64 / upm * 1000.0);
            ops.push_str(&format!("{:.0} ", xoff_tu));
        }
        // P520/P548 — delta model: o CIDFont /W já fornece a largura
        // nominal (hmtx). O TJ deve conter apenas a diferença entre essa
        // largura declarada e o avanço real produzido pelo shaper. No
        // operador PDF TJ, o número é subtraído da coordenada horizontal,
        // logo `nominal - x_advance` é o sinal correcto: positivo para
        // kerning negativo (aproxima), negativo para kerning positivo
        // (afasta).
        let nominal = glyph_to_nominal.get(&g.glyph_id).copied().unwrap_or(g.x_advance);
        let advance_tu = (nominal - g.x_advance) as f64 / upm * 1000.0;
        let new_gid = if glyph_mapping.is_empty() {
            g.glyph_id
        } else {
            remap_glyph_id(g.glyph_id, glyph_mapping)
        };
        ops.push_str(&format!("<{:04X}> {:.0} ", new_gid, advance_tu));
    }
}

/// **P956** — prefixo do envelope verbose (ADR-0126): isola o bloco com
/// `q`, carrega posição + flip Y no `cm` (`Tm` fica sempre
/// `1 0 0 -1 0 0`), e declara a cor de preenchimento por bloco via
/// `/c0 cs … scn` (`/c0` é o colour space sRGB ICCBased dos recursos da
/// página — `builder.md` §P956). `base_y` é exactamente o valor que o
/// compacto passaria ao `Td` (top-level: `page_height − pos.y`; local em
/// Group: `pos.y` directo) — a derivação F·F=I de `stream.md` §P956
/// garante geometria idêntica nos dois modos. Fill `None` → preto `0 0 0`.
/// Precisão do `cm`: 5 casas decimais (convenção P777 das matrizes `cm` de
/// imagem; o vanilla usa 5-8 dígitos — medido em P956: `70.86614`).
fn verbose_block_prefix(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    fill: &Option<typst_core::entities::layout_types::Color>,
) {
    let (r, g, b) = match fill {
        Some(c) => {
            let (r, g, b, _) = c.to_rgba_f32();
            (r, g, b)
        }
        None => (0.0, 0.0, 0.0),
    };
    ops.push_str(&format!(
        "q\n1 0 0 -1 {pos_x:.5} {base_y:.5} cm\n/c0 cs {r:.3} {g:.3} {b:.3} scn\nBT\n"
    ));
}

/// **P956** — operador `Tr` explícito do envelope verbose: `0 Tr` por
/// omissão; faux-bold (P139) → `2 Tr` + `{stroke:.3} w` no mesmo envelope.
fn verbose_tr_ops(style: &typst_core::entities::layout_types::TextStyle) -> String {
    let stroke_pt = style.faux_bold_stroke_pt(pdf_defaults::FAUX_BOLD_K);
    if stroke_pt > f64::EPSILON {
        format!("2 Tr\n{stroke_pt:.3} w\n")
    } else {
        "0 Tr\n".to_string()
    }
}

/// **P956** — `Tc` do envelope verbose, apenas se tracking ≠ 0 (mesma
/// convenção do caminho Type1 compact).
fn push_tc_if_tracking(
    ops: &mut String,
    style: &typst_core::entities::layout_types::TextStyle,
) {
    let tracking_pt =
        style.tracking.map(|t| t.resolve_pt(style.size.val())).unwrap_or(0.0);
    if tracking_pt.abs() > f64::EPSILON {
        ops.push_str(&format!("{tracking_pt:.2} Tc\n"));
    }
}

pub(super) fn emit_shaped_pdf(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    glyphs: &[typst_core::entities::layout_types::ShapedGlyph],
    text: &str,
    style: &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
    units_per_em: u16,
) {
    if glyphs.is_empty() {
        return;
    }
    let upm = units_per_em as f64;
    match scenario {
        FontScenario::Type1 => {
            emit_text_pdf(ops, pos_x, base_y, text, style, scenario);
        }
        FontScenario::Cidfont { glyph_mapping, glyph_to_nominal, bitmap, .. } => {
            if let Some(bmp) = bitmap {
                emit_bitmap_glyph_draws(
                    ops,
                    pos_x,
                    base_y,
                    glyphs,
                    style,
                    bmp,
                    units_per_em,
                );
                return;
            }
            let rg = fill_rg_prefix(&style.fill);
            ops.push_str(&format!(
                "{rg}BT\n/F1 {:.1} Tf\n{:.3} {:.3} Td\n[ ",
                style.size.val(),
                pos_x,
                base_y
            ));
            push_tj_glyph_entries(ops, glyphs, glyph_mapping, glyph_to_nominal, upm);
            ops.push_str("] TJ\nET\n");
        }
        FontScenario::Multifont {
            fonts,
            per_font_glyph_mapping,
            per_font_glyph_to_nominal,
            per_font_bitmap,
            ..
        } => {
            let fi = font_index_for_style(fonts, style);
            if let Some(Some(bmp)) = per_font_bitmap.get(fi) {
                emit_bitmap_glyph_draws(
                    ops,
                    pos_x,
                    base_y,
                    glyphs,
                    style,
                    bmp,
                    units_per_em,
                );
                return;
            }
            let glyph_mapping = &per_font_glyph_mapping[fi];
            let glyph_to_nominal = &per_font_glyph_to_nominal[fi];
            let rg = fill_rg_prefix(&style.fill);
            ops.push_str(&format!(
                "{rg}BT\n/F{} {:.1} Tf\n{:.3} {:.3} Td\n[ ",
                fi + 1,
                style.size.val(),
                pos_x,
                base_y
            ));
            push_tj_glyph_entries(ops, glyphs, glyph_mapping, glyph_to_nominal, upm);
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
pub(super) fn line_rg_prefix(
    color: &Option<typst_core::entities::layout_types::Color>,
) -> String {
    match color {
        None => String::new(),
        Some(c) => {
            let (r, g, b, _) = c.to_rgba_f32();
            format!("{:.3} {:.3} {:.3} RG ", r, g, b)
        }
    }
}

pub(super) fn fill_rg_prefix(
    color: &Option<typst_core::entities::layout_types::Color>,
) -> String {
    match color {
        None => String::new(),
        Some(c) => {
            let (r, g, b, _) = c.to_rgba_f32();
            format!("{:.3} {:.3} {:.3} rg\n", r, g, b)
        }
    }
}

/// Type1 silently ignored (sem TrueType embebida); CIDFont/Multifont
/// emitem `<{:04X}>` Identity-H via `/F1` (math fonts).
///
/// **P906** — `FontScenario::Multifont` tem várias fontes embutidas
/// (`/F1`, `/F2`, ...); antes deste passo, esta função hardcodava `/F1`
/// incondicionalmente, correcto por coincidência só quando o glifo vinha
/// mesmo da primeira fonte embutida. Achado: glifos de esticamento
/// (`FrameItem::Glyph`, usados por `layout_stretchy_delimiter`/
/// `layout_assembly`, eixo vertical e — desde este passo — horizontal)
/// resolvidos numa fonte companion MATH (índice != 0) desenhavam-se com o
/// `glyph_id` certo interpretado pela fonte errada — invisível ou lixo
/// visual. Corrigido reaproveitando `per_font_glyph_reverse` (mesmo `build_
/// math_glyph_reverse_map` já usado no subsetting DEBT-9/P45,
/// `builder.rs:988`) — não precisa de `style`: procura em qual fonte o
/// `glyph_id` está efectivamente presente (glifos de esticamento só
/// existem na fonte de onde vieram, por construção do subsetting).
/// `FontScenario::Cidfont` continua a usar sempre `/F1` (só tem uma fonte
/// candidata — mesmo comportamento de antes, preservado).
///
/// **P906 (2º achado nesta função)** — nem o ramo `Cidfont` nem o
/// `Multifont` aplicavam `remap_glyph_id`/`per_font_glyph_mapping` (P516,
/// já aplicado a `emit_text_pdf`/`emit_shaped_pdf`) ao `glyph_id` recebido
/// — desenhava sempre o índice ORIGINAL (pré-subsetting) da fonte
/// completa, mas a fonte efectivamente embutida no PDF está subsetada
/// (renumerada) sempre que houver `glyph_mapping` não vazio. Resultado:
/// `<XXXX> Tj` referenciava um slot de glifo errado/inexistente na fonte
/// embutida — glifo invisível ou lixo, mesmo depois da selecção de `/Fn`
/// já estar correcta. Corrigido aplicando o mesmo remap já usado no
/// caminho de texto. Ver `export/stream.md` §P906.
pub(super) fn emit_glyph_pdf(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    glyph_id: u16,
    size: typst_core::entities::layout_types::Pt,
    style: &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
) {
    if let Some(c) = style.fill {
        let (r, g, b, _) = c.to_rgba_f32();
        ops.push_str(&format!(
            "{:.3} {:.3} {:.3} rg
",
            r, g, b
        ));
    }
    match scenario {
        FontScenario::Type1 => {
            // Sem fonte TrueType → glyph_id sem significado. Ignored.
        }
        FontScenario::Cidfont { glyph_mapping, .. } => {
            let new_gid = if glyph_mapping.is_empty() {
                glyph_id
            } else {
                crate::export::subset::remap_glyph_id(glyph_id, glyph_mapping)
            };
            ops.push_str(&format!(
                "BT\n/F1 {:.1} Tf\n{:.1} {:.1} Td\n<{:04X}> Tj\nET\n",
                size.val(),
                pos_x,
                base_y,
                new_gid
            ));
        }
        FontScenario::Multifont {
            per_font_glyph_reverse,
            per_font_glyph_mapping,
            ..
        } => {
            let fi = per_font_glyph_reverse
                .iter()
                .position(|m| m.contains_key(&glyph_id))
                .unwrap_or(0);
            let glyph_mapping = &per_font_glyph_mapping[fi];
            let new_gid = if glyph_mapping.is_empty() {
                glyph_id
            } else {
                crate::export::subset::remap_glyph_id(glyph_id, glyph_mapping)
            };
            ops.push_str(&format!(
                "BT\n/F{} {:.1} Tf\n{:.1} {:.1} Td\n<{:04X}> Tj\nET\n",
                fi + 1,
                size.val(),
                pos_x,
                base_y,
                new_gid
            ));
        }
    }
}

// ── P956 — variantes verbose (envelope vanilla-espelhado, ADR-0126) ────────
//
// Por bloco de texto: `q` + `cm 1 0 0 -1 x y` (posição + flip Y) +
// `/c0 cs r g b scn` (cor por bloco) + `BT 0 Tr /F{n} {size} Tf [Tc]
// 1 0 0 -1 0 0 Tm … ET` + `Q`. O `base_y` recebido é o mesmo valor que o
// compacto passa ao `Td` (derivação F·F=I em stream.md §P956) — o que muda
// é o envelope, não a coordenada.

/// **P956** — `emit_text_pdf` verbose: mesmo envelope com `(…) Tj` (Type1)
/// ou `<hex> Tj` (CIDFont/Multifont).
pub(super) fn emit_text_pdf_verbose(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    text: &str,
    style: &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
) {
    match scenario {
        FontScenario::Type1 => {
            let safe = escape_pdf_string(text);
            if safe.is_empty() {
                return;
            }
            let font_ref = match (style.bold, style.italic) {
                (true, _) => "F2",
                (false, true) => "F3",
                (false, false) => "F1",
            };
            verbose_block_prefix(ops, pos_x, base_y, &style.fill);
            ops.push_str(&verbose_tr_ops(style));
            ops.push_str(&format!("/{font_ref} {:.1} Tf\n", style.size.val()));
            push_tc_if_tracking(ops, style);
            ops.push_str(&format!("1 0 0 -1 0 0 Tm\n({safe}) Tj\nET\nQ\n"));
        }
        FontScenario::Cidfont { char_to_gid, .. } => {
            if text.is_empty() {
                return;
            }
            let hex_str = text_to_hex_string(text, char_to_gid);
            verbose_block_prefix(ops, pos_x, base_y, &style.fill);
            ops.push_str(&verbose_tr_ops(style));
            ops.push_str(&format!("/F1 {:.1} Tf\n", style.size.val()));
            push_tc_if_tracking(ops, style);
            ops.push_str(&format!("1 0 0 -1 0 0 Tm\n{hex_str} Tj\nET\nQ\n"));
        }
        FontScenario::Multifont { fonts, per_font_char_to_gid, .. } => {
            if text.is_empty() {
                return;
            }
            let fi = font_index_for_style(fonts, style);
            let hex_str = text_to_hex_string(text, &per_font_char_to_gid[fi]);
            verbose_block_prefix(ops, pos_x, base_y, &style.fill);
            ops.push_str(&verbose_tr_ops(style));
            ops.push_str(&format!("/F{} {:.1} Tf\n", fi + 1, style.size.val()));
            push_tc_if_tracking(ops, style);
            ops.push_str(&format!("1 0 0 -1 0 0 Tm\n{hex_str} Tj\nET\nQ\n"));
        }
    }
}

/// **P956** — `emit_shaped_pdf` verbose: mesmo envelope com o array `TJ`
/// partilhado (`push_tj_glyph_entries` — delta model P520/P548, x_offset
/// P486, remap P516). Glifos bitmap (CBDT, P941) ficam **inalterados nos
/// dois modos** — são desenho de imagem (`/ImN Do`), não texto.
pub(super) fn emit_shaped_pdf_verbose(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    glyphs: &[typst_core::entities::layout_types::ShapedGlyph],
    text: &str,
    style: &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
    units_per_em: u16,
) {
    if glyphs.is_empty() {
        return;
    }
    let upm = units_per_em as f64;
    match scenario {
        FontScenario::Type1 => {
            emit_text_pdf_verbose(ops, pos_x, base_y, text, style, scenario);
        }
        FontScenario::Cidfont { glyph_mapping, glyph_to_nominal, bitmap, .. } => {
            if let Some(bmp) = bitmap {
                emit_bitmap_glyph_draws(
                    ops,
                    pos_x,
                    base_y,
                    glyphs,
                    style,
                    bmp,
                    units_per_em,
                );
                return;
            }
            verbose_block_prefix(ops, pos_x, base_y, &style.fill);
            ops.push_str(&verbose_tr_ops(style));
            ops.push_str(&format!("/F1 {:.1} Tf\n", style.size.val()));
            push_tc_if_tracking(ops, style);
            ops.push_str("1 0 0 -1 0 0 Tm\n[ ");
            push_tj_glyph_entries(ops, glyphs, glyph_mapping, glyph_to_nominal, upm);
            ops.push_str("] TJ\nET\nQ\n");
        }
        FontScenario::Multifont {
            fonts,
            per_font_glyph_mapping,
            per_font_glyph_to_nominal,
            per_font_bitmap,
            ..
        } => {
            let fi = font_index_for_style(fonts, style);
            if let Some(Some(bmp)) = per_font_bitmap.get(fi) {
                emit_bitmap_glyph_draws(
                    ops,
                    pos_x,
                    base_y,
                    glyphs,
                    style,
                    bmp,
                    units_per_em,
                );
                return;
            }
            verbose_block_prefix(ops, pos_x, base_y, &style.fill);
            ops.push_str(&verbose_tr_ops(style));
            ops.push_str(&format!("/F{} {:.1} Tf\n", fi + 1, style.size.val()));
            push_tc_if_tracking(ops, style);
            ops.push_str("1 0 0 -1 0 0 Tm\n[ ");
            push_tj_glyph_entries(
                ops,
                glyphs,
                &per_font_glyph_mapping[fi],
                &per_font_glyph_to_nominal[fi],
                upm,
            );
            ops.push_str("] TJ\nET\nQ\n");
        }
    }
}

/// **P956** — `emit_glyph_pdf` verbose (stretchy): mesmo envelope com
/// `<gid> Tj`. Sem `style` → fill preto e `0 Tr` (como o compact, que
/// também não emite cor neste caminho).
pub(super) fn emit_glyph_pdf_verbose(
    ops: &mut String,
    pos_x: f64,
    base_y: f64,
    glyph_id: u16,
    size: typst_core::entities::layout_types::Pt,
    style: &typst_core::entities::layout_types::TextStyle,
    scenario: &FontScenario,
) {
    match scenario {
        FontScenario::Type1 => {
            // Sem fonte TrueType → glyph_id sem significado. Ignored.
        }
        FontScenario::Cidfont { glyph_mapping, .. } => {
            let new_gid = if glyph_mapping.is_empty() {
                glyph_id
            } else {
                crate::export::subset::remap_glyph_id(glyph_id, glyph_mapping)
            };
            verbose_block_prefix(ops, pos_x, base_y, &style.fill);
            ops.push_str("0 Tr\n");
            ops.push_str(&format!("/F1 {:.1} Tf\n", size.val()));
            ops.push_str(&format!("1 0 0 -1 0 0 Tm\n<{new_gid:04X}> Tj\nET\nQ\n"));
        }
        FontScenario::Multifont {
            per_font_glyph_reverse,
            per_font_glyph_mapping,
            ..
        } => {
            let fi = per_font_glyph_reverse
                .iter()
                .position(|m| m.contains_key(&glyph_id))
                .unwrap_or(0);
            let glyph_mapping = &per_font_glyph_mapping[fi];
            let new_gid = if glyph_mapping.is_empty() {
                glyph_id
            } else {
                crate::export::subset::remap_glyph_id(glyph_id, glyph_mapping)
            };
            verbose_block_prefix(ops, pos_x, base_y, &style.fill);
            ops.push_str("0 Tr\n");
            ops.push_str(&format!("/F{} {:.1} Tf\n", fi + 1, size.val()));
            ops.push_str(&format!("1 0 0 -1 0 0 Tm\n<{new_gid:04X}> Tj\nET\nQ\n"));
        }
    }
}

/// P263 — Emite operadores de stroke colour para um Paint (Solid ou Gradient).
///
/// Para `Paint::Solid(c)`: emit `r g b RG` literal P261 preservado.
/// Para `Paint::Gradient(g)`: emit `/Pattern CS /P{n} SCN` (set colour
/// space pattern + apply pattern).
pub(super) fn emit_stroke_paint(
    ops: &mut String,
    paint: &typst_core::entities::paint::Paint,
    thickness: f64,
    effective_bbox: Option<typst_core::entities::layout_types::Rect>,
    pat_ptr_to_idx: &HashMap<DedupKey, usize>,
    pat_refs: &[PatternRef],
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
                ops.push_str(&format!(
                    "/Pattern CS\n/{} SCN\n{:.2} w\n",
                    r.name, thickness
                ));
            } else {
                // Fallback paranóide — gradient não registado em scan_all_gradients.
                let c = paint.to_color();
                let (r, g, b, _) = c.to_rgba_f32();
                ops.push_str(&format!(
                    "{:.3} {:.3} {:.3} RG\n{:.2} w\n",
                    r, g, b, thickness
                ));
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

/// **P776** — calcula a matriz `cm` para um `FrameItem::Image`, compondo a
/// transformação EXIF (1-8) com a escala e posição de layout.
///
/// Replica as matrizes `cm` do vanilla `exif_transform` em
/// `crates/typst-pdf/src/image.rs`: JPEGs não são recodificados; a orientação
/// é aplicada directamente na matriz PDF. As dimensões `width`/`height` são as
/// dimensões de layout finais (L1 já trocou `width`/`height` para orientações
/// 5-8).
fn image_exif_matrix(
    orientation: u32,
    width: f64,
    height: f64,
    pos_x: f64,
    pdf_y: f64,
) -> TransformMatrix {
    let (a, b, c, d, tx, ty) = match orientation {
        2 => (-width, 0.0, 0.0, height, width, 0.0),
        3 => (-width, 0.0, 0.0, -height, width, height),
        4 => (width, 0.0, 0.0, -height, 0.0, height),
        5 => (0.0, -height, -width, 0.0, width, height),
        6 => (0.0, -height, width, 0.0, 0.0, height),
        7 => (0.0, height, width, 0.0, 0.0, 0.0),
        8 => (0.0, height, -width, 0.0, width, 0.0),
        _ => (width, 0.0, 0.0, height, 0.0, 0.0),
    };

    TransformMatrix { a, b, c, d, tx: tx + pos_x, ty: ty + pdf_y }
}

// **P281** — Stream builder unificado (substitui build_page_stream_type1/
// cidfont/multifont). Despacha Text/Glyph por `ctx.font_scenario`;
// Line/Image/Shape/Group são scenario-independent.
pub(super) fn build_page_stream(page: &Page, ctx: &PageContext) -> Vec<u8> {
    let mut ops = String::new();
    let page_height = page.height;

    if let typst_core::entities::page_canvas::PageFill::Paint(paint) = &page.fill {
        let (r, g, b, _) = paint.to_color().to_rgba_f32();
        ops.push_str(&format!(
            "{r:.5} {g:.5} {b:.5} rg\n0 0 {:.5} {:.5} re f\n",
            page.canvas_width(),
            page.canvas_height(),
        ));
    }
    if !page.bleed.is_zero() {
        ops.push_str(&format!(
            "q\n1 0 0 1 {:.5} {:.5} cm\n",
            page.bleed.left, page.bleed.bottom,
        ));
    }

    // **P979** — agrupamento de runs de texto (modo verbose): itens
    // `TextShaped` consecutivos com o mesmo envelope e a mesma baseline
    // fundem-se num só `BT…ET` (paridade vanilla: um bloco por linha de
    // estilo uniforme — `stream.md` §P979).
    let verbose = matches!(ctx.mode, StreamMode::Verbose);
    emit_page_items(&mut ops, &page.background, page_height, ctx, verbose);
    emit_page_items(&mut ops, &page.items, page_height, ctx, verbose);
    emit_page_items(&mut ops, &page.foreground, page_height, ctx, verbose);

    if !page.bleed.is_zero() {
        ops.push_str("Q\n");
    }
    ops.into_bytes()
}

fn emit_page_items(
    ops: &mut String,
    items: &[FrameItem],
    page_height: f64,
    ctx: &PageContext,
    verbose: bool,
) {
    let mut i = 0;
    while i < items.len() {
        if verbose {
            let run_end = verbose_run_end(items, i, ctx);
            if run_end > i + 1 {
                emit_verbose_text_run(ops, &items[i..run_end], page_height, ctx);
                i = run_end;
                continue;
            }
        }
        *ops = draw_item_top(std::mem::take(ops), &items[i], page_height, ctx);
        i += 1;
    }
}

/// **P979** — dois itens fundem-se no mesmo `BT…ET` se forem ambos
/// `TextShaped` com a mesma baseline e todos os campos que afectam o
/// envelope verbose iguais (fonte, tamanho, cor, Tr/faux-bold, tracking,
//  eixos, direcção, upem). Campos irrelevantes ao envelope (lang, etc.)
/// não comparam — mas nada aqui pode mudar o output: só fundir quando é
/// garantidamente indistinguível.
fn verbose_run_compatible(a: &FrameItem, b: &FrameItem) -> bool {
    let (
        FrameItem::TextShaped {
            pos: pa, style: sa, units_per_em: ua, glyphs: ga, ..
        },
        FrameItem::TextShaped {
            pos: pb, style: sb, units_per_em: ub, glyphs: gb, ..
        },
    ) = (a, b)
    else {
        return false;
    };
    if ga.is_empty() || gb.is_empty() {
        return false;
    }
    if pa.y != pb.y || ua != ub {
        return false;
    }
    sa.size == sb.size
        && sa.fill == sb.fill
        && sa.weight == sb.weight
        && sa.tracking == sb.tracking
        && sa.bold == sb.bold
        && sa.italic == sb.italic
        && sa.font == sb.font
        && sa.variations == sb.variations
        && sa.dir == sb.dir
        && sa.math == sb.math
        && sa.math_size == sb.math_size
}

/// **P979** — fim (exclusivo) do run de texto que começa em `start`.
/// Devolve `start + 1` quando não há fusão possível. Runs só se formam
/// nos cenários com emissão por glifos (Cidfont/Multifont sem bitmap).
fn verbose_run_end(items: &[FrameItem], start: usize, ctx: &PageContext) -> usize {
    // **P1133** — itens `style.math` nunca formam run, também na produção:
    // a fronteira `TJ` inteira quantiza a posição absoluta do átomo e pode
    // alterar o render. O vanilla emite um bloco por fragmento math; prosa
    // continua a fundir por P979. O oracle conserva o mesmo split e aplica
    // depois apenas suas transformações de stream.
    if let FrameItem::TextShaped { style, .. } = &items[start] {
        if style.math {
            return start + 1;
        }
    }
    let supported = match &ctx.font_scenario {
        FontScenario::Cidfont { bitmap, .. } => bitmap.is_none(),
        FontScenario::Multifont { fonts, per_font_bitmap, .. } => {
            if let FrameItem::TextShaped { style, .. } = &items[start] {
                let fi = font_index_for_style(fonts, style);
                per_font_bitmap.get(fi).and_then(|b| b.as_ref()).is_none()
            } else {
                false
            }
        }
        FontScenario::Type1 => false,
    };
    if !supported || !matches!(items[start], FrameItem::TextShaped { .. }) {
        return start + 1;
    }
    let mut end = start + 1;
    while end < items.len() && verbose_run_compatible(&items[end - 1], &items[end]) {
        end += 1;
    }
    end
}

/// **P979** — emissão de um run fundido (≥2 itens): um só envelope, um só
/// array `TJ`; os gaps entre itens são ajustes `TJ` computados das
/// posições (com correcção do arredondamento acumulado — o cursor segue a
/// aritmética exacta do renderer, avanço a avanço).
fn emit_verbose_text_run(
    ops: &mut String,
    run: &[FrameItem],
    page_height: f64,
    ctx: &PageContext,
) {
    let FrameItem::TextShaped { pos: pos0, style, .. } = &run[0] else { return };
    let base_y = page_height - pos0.y.val();
    verbose_block_prefix(ops, pos0.x.val(), base_y, &style.fill);
    ops.push_str(&verbose_tr_ops(style));
    push_tc_if_tracking(ops, style);
    match &ctx.font_scenario {
        FontScenario::Cidfont { glyph_mapping, glyph_to_nominal, .. } => {
            ops.push_str(&format!(
                "/F1 {:.1} Tf
",
                style.size.val()
            ));
            ops.push_str(
                "1 0 0 -1 0 0 Tm
[ ",
            );
            push_run_tj_entries(ops, run, glyph_mapping, glyph_to_nominal);
            ops.push_str(
                "] TJ
ET
Q
",
            );
        }
        FontScenario::Multifont {
            fonts,
            per_font_glyph_mapping,
            per_font_glyph_to_nominal,
            ..
        } => {
            let fi = font_index_for_style(fonts, style);
            ops.push_str(&format!(
                "/F{} {:.1} Tf
",
                fi + 1,
                style.size.val()
            ));
            ops.push_str(
                "1 0 0 -1 0 0 Tm
[ ",
            );
            push_run_tj_entries(
                ops,
                run,
                &per_font_glyph_mapping[fi],
                &per_font_glyph_to_nominal[fi],
            );
            ops.push_str(
                "] TJ
ET
Q
",
            );
        }
        FontScenario::Type1 => unreachable!("runs Type1 não se formam — verbose_run_end"),
    }
}

/// **P979** — entries `TJ` de um run fundido. Dentro de cada item: o
/// delta model de P486/P520/P548 (idem `push_tj_glyph_entries`). Na
/// fronteira entre itens: um ajuste extra que leva o ponto de texto
/// exactamente ao `pos.x` do item seguinte — computado contra o cursor
/// **com o arredondamento aplicado pelo renderer** (cada ajuste TJ é um
/// inteiro em milésimos de em), para que a deriva de arredondamento nunca
/// acumule além de um quantum por fronteira.
fn push_run_tj_entries(
    ops: &mut String,
    run: &[FrameItem],
    glyph_mapping: &HashMap<u16, u16>,
    glyph_to_nominal: &HashMap<u16, i32>,
) {
    let mut cursor: Option<f64> = None;
    for item in run {
        let FrameItem::TextShaped { pos, glyphs, style, units_per_em, .. } = item else {
            continue;
        };
        let size = style.size.val();
        let upm = (*units_per_em).max(1) as f64;
        if let Some(cur) = cursor {
            // Número TJ = milésimos de em SUBTRAÍDOS da posição: para levar
            // o ponto de `cur` a `pos.x`, o número é (cur − pos.x)/size×1000.
            let boundary = ((cur - pos.x.val()) / size * 1000.0).round();
            if boundary != 0.0 {
                ops.push_str(&format!("{boundary:.0} "));
                cursor = Some(cur - boundary / 1000.0 * size);
            }
        } else {
            cursor = Some(pos.x.val());
        }
        for g in glyphs {
            if g.x_offset != 0 {
                let xoff_tu = -(g.x_offset as f64 / upm * 1000.0);
                ops.push_str(&format!("{:.0} ", xoff_tu));
            }
            let nominal =
                glyph_to_nominal.get(&g.glyph_id).copied().unwrap_or(g.x_advance);
            let advance_tu = ((nominal - g.x_advance) as f64 / upm * 1000.0).round();
            let new_gid = if glyph_mapping.is_empty() {
                g.glyph_id
            } else {
                remap_glyph_id(g.glyph_id, glyph_mapping)
            };
            ops.push_str(&format!("<{:04X}> {:.0} ", new_gid, advance_tu));
            // Avanço tal como o renderer o aplica (com o ajuste arredondado).
            let applied = (nominal as f64 / upm - advance_tu / 1000.0) * size;
            cursor = cursor.map(|c| c + applied);
        }
    }
}

/// **P788** — emissão top-level (com flip Y-down→PDF: `page_height - pos.y`)
/// de UM `FrameItem`, extraída de `build_page_stream` para que filhos de
/// `FrameItem::Link` ao nível da página sigam o mesmo caminho. Antes eram
/// desenhados por `draw_item_local` (sem flip — assume matriz de Group) e
/// apareciam no fundo da página (bug medido em P786 A9 e reproduzido com
/// `#link` genérico: filho a y≈754 em vez de y≈68).
///
/// Recebe e devolve `ops` por valor para que o braço `Link` possa chamar
/// recursivamente sem conflitos de borrow.
fn draw_item_top(
    mut ops: String,
    item: &FrameItem,
    page_height: f64,
    ctx: &PageContext,
) -> String {
    use typst_core::entities::geometry::ShapeKind;
    match item {
        // P483 — path primário: glifos com shaping real.
        // **P956** — dispatch por modo: Compact = formato Passo 20
        // byte-inalterado; Verbose = envelope vanilla-espelhado (mesma
        // coordenada `pdf_y` — derivação F·F=I, stream.md §P956).
        FrameItem::TextShaped { pos, glyphs, style, text, units_per_em } => {
            let pdf_y = page_height - pos.y.val();
            match ctx.mode {
                StreamMode::Compact => emit_shaped_pdf(
                    &mut ops,
                    pos.x.val(),
                    pdf_y,
                    glyphs,
                    text.as_str(),
                    style,
                    &ctx.font_scenario,
                    *units_per_em,
                ),
                StreamMode::Verbose => emit_shaped_pdf_verbose(
                    &mut ops,
                    pos.x.val(),
                    pdf_y,
                    glyphs,
                    text.as_str(),
                    style,
                    &ctx.font_scenario,
                    *units_per_em,
                ),
            }
        }
        // P483 — fallback: fonte não carregada, Type1, ou shaping indisponível.
        FrameItem::Text { pos, text, style } => {
            let pdf_y = page_height - pos.y.val();
            match ctx.mode {
                StreamMode::Compact => emit_text_pdf(
                    &mut ops,
                    pos.x.val(),
                    pdf_y,
                    text.as_str(),
                    style,
                    &ctx.font_scenario,
                ),
                StreamMode::Verbose => emit_text_pdf_verbose(
                    &mut ops,
                    pos.x.val(),
                    pdf_y,
                    text.as_str(),
                    style,
                    &ctx.font_scenario,
                ),
            }
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
                "q {}{:.3} w {:.5} {:.5} m {:.5} {:.5} l S Q\n",
                rg, thickness, x1, y1, x2, y2
            ));
        }
        FrameItem::Glyph { pos, glyph_id, size, style, .. } => {
            let pdf_y = page_height - pos.y.val();
            match ctx.mode {
                StreamMode::Compact => emit_glyph_pdf(
                    &mut ops,
                    pos.x.val(),
                    pdf_y,
                    *glyph_id,
                    *size,
                    style,
                    &ctx.font_scenario,
                ),
                StreamMode::Verbose => emit_glyph_pdf_verbose(
                    &mut ops,
                    pos.x.val(),
                    pdf_y,
                    *glyph_id,
                    *size,
                    style,
                    &ctx.font_scenario,
                ),
            }
        }
        FrameItem::Image {
            pos,
            data,
            width,
            height,
            intrinsic_width,
            intrinsic_height,
            clip_rect,
            orientation,
        } => {
            let ptr = Arc::as_ptr(data) as usize;
            if let Some(&idx) = ctx.ptr_to_idx.get(&ptr) {
                // pos.y é o TOPO da imagem → canto inferior esquerdo no espaço PDF.
                let pdf_y = page_height - pos.y.val() - height.val();
                let matrix = image_exif_matrix(
                    *orientation,
                    width.val(),
                    height.val(),
                    pos.x.val(),
                    pdf_y,
                );
                ops.push_str("q\n");
                if let Some(clip) = clip_rect {
                    // P771 — clip_path ao rectângulo target, replicando o vanilla.
                    let clip_pdf_y = page_height - clip.y.val() - clip.h.val();
                    ops.push_str(&format!(
                        "{:.3} {:.3} {:.3} {:.3} re W n\n",
                        clip.x.val(),
                        clip_pdf_y,
                        clip.w.val(),
                        clip.h.val()
                    ));
                }
                // **P777** — precisão aumentada para 5 casas decimais,
                // replicando o vanilla e evitando desvios de 1 px nas
                // bordas em orientações EXIF com flip/rotate.
                ops.push_str(&format!(
                    "{:.5} {:.5} {:.5} {:.5} {:.5} {:.5} cm\n/{} Do\nQ\n",
                    matrix.a,
                    matrix.b,
                    matrix.c,
                    matrix.d,
                    matrix.tx,
                    matrix.ty,
                    ctx.img_refs[idx].name
                ));
            }
        }
        FrameItem::Shape {
            pos,
            kind,
            width,
            height,
            fill,
            stroke,
            parent_bbox_at_emit,
        } => {
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
                emit_stroke_paint(
                    &mut ops,
                    &s.paint,
                    s.thickness,
                    *parent_bbox_at_emit,
                    ctx.pat_ptr_to_idx,
                    ctx.pat_refs,
                );
            }

            // Path — depende do tipo de forma.
            match kind {
                ShapeKind::Rect => {
                    ops.push_str(&format!(
                        "{:.2} {:.2} {:.2} {:.2} re\n",
                        pos.x.val(),
                        pdf_y,
                        width,
                        height
                    ));
                }
                ShapeKind::RoundedRect { radii } => {
                    emit_rounded_rect_ops(
                        &mut ops,
                        pos.x.val(),
                        pdf_y,
                        *width,
                        *height,
                        radii,
                    );
                }
                ShapeKind::Ellipse => {
                    let cx = pos.x.val() + width / 2.0;
                    let cy = pdf_y + height / 2.0;
                    let rx = width / 2.0;
                    let ry = height / 2.0;
                    let ox = rx * pdf_defaults::BEZIER_CIRCLE_KAPPA;
                    let oy = ry * pdf_defaults::BEZIER_CIRCLE_KAPPA;
                    ops.push_str(&format!("{:.3} {:.3} m\n", cx, cy + ry));
                    ops.push_str(&format!(
                        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx + ox,
                        cy + ry,
                        cx + rx,
                        cy + oy,
                        cx + rx,
                        cy
                    ));
                    ops.push_str(&format!(
                        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx + rx,
                        cy - oy,
                        cx + ox,
                        cy - ry,
                        cx,
                        cy - ry
                    ));
                    ops.push_str(&format!(
                        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx - ox,
                        cy - ry,
                        cx - rx,
                        cy - oy,
                        cx - rx,
                        cy
                    ));
                    ops.push_str(&format!(
                        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx - rx,
                        cy + oy,
                        cx - ox,
                        cy + ry,
                        cx,
                        cy + ry
                    ));
                }
                ShapeKind::Line { dx, dy } => {
                    let start_offset_x = if *dx < 0.0 { *width } else { 0.0 };
                    let end_offset_x = if *dx < 0.0 { 0.0 } else { *width };
                    let start_offset_y = if *dy > 0.0 { *height } else { 0.0 };
                    let end_offset_y = if *dy > 0.0 { 0.0 } else { *height };
                    let start_x = pos.x.val() + start_offset_x;
                    let start_y = pdf_y + start_offset_y;
                    let end_x = pos.x.val() + end_offset_x;
                    let end_y = pdf_y + end_offset_y;
                    ops.push_str(&format!("{:.3} {:.3} m\n", start_x, start_y));
                    ops.push_str(&format!("{:.3} {:.3} l\n", end_x, end_y));
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
                                pos.x.val() + p1.x.0,
                                page_height - (pos.y.val() + p1.y.0),
                                pos.x.val() + p2.x.0,
                                page_height - (pos.y.val() + p2.y.0),
                                pos.x.val() + p3.x.0,
                                page_height - (pos.y.val() + p3.y.0),
                            )),
                            PathItem::ClosePath => ops.push_str("h\n"),
                        }
                    }
                }
            }

            match (fill.is_some(), stroke.is_some()) {
                (true, true) => ops.push_str("B\n"),
                (true, false) => ops.push_str("f\n"),
                (false, true) => ops.push_str("S\n"),
                (false, false) => {}
            }

            ops.push_str("Q\n");
        }
        FrameItem::Group {
            pos,
            matrix,
            clip_mask,
            inner_width,
            inner_height,
            items,
        } => {
            let pdf_y = page_height - pos.y.val();

            // O layouter usa Y-down; o PDF usa Y-up.
            // Inverter d para manter a orientação padrão Y-down do layout dentro do Group.
            ops.push_str("q\n");
            ops.push_str(&format!(
                "{:.6} {:.6} {:.6} {:.6} {:.5} {:.5} cm\n",
                matrix.a,
                -matrix.b,
                matrix.c,
                -matrix.d,
                pos.x.val() + matrix.tx,
                pdf_y - matrix.ty,
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
        // **P788** — filhos seguem o caminho top-level COM flip Y (eram
        // `draw_item_local`, sem flip — apareciam no fundo da página).
        FrameItem::Link { items, .. } => {
            for child in items {
                ops = draw_item_top(ops, child, page_height, ctx);
            }
        }
        FrameItem::Semantic { items, .. } => {
            let tagged = ctx.tags == PdfTags::Enabled;
            if tagged {
                let key = item as *const FrameItem as usize;
                let mcid = ctx.semantic_mcids.get(&key).copied().unwrap_or(0);
                ops.push_str(&format!("/Formula << /MCID {mcid} >> BDC\n"));
            }
            for child in items {
                ops = draw_item_top(ops, child, page_height, ctx);
            }
            if tagged {
                ops.push_str("EMC\n");
            }
        }
    }
    ops
}

/// Emite os operadores de path de uma forma no espaço LOCAL de um Group.
///
/// NÃO usa page_height. A matriz `cm` já inverteu o eixo Y.
/// Chamada para emitir clip_mask antes de `W n`.
pub(super) fn emit_shape_path_local(
    ops: &mut String,
    kind: &typst_core::entities::geometry::ShapeKind<
        typst_core::entities::layout_types::Pt,
    >,
    width: f64,
    height: f64,
) {
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
                    PathItem::MoveTo(p) => {
                        ops.push_str(&format!("{:.2} {:.2} m\n", p.x.0, -p.y.0,))
                    }
                    PathItem::LineTo(p) => {
                        ops.push_str(&format!("{:.2} {:.2} l\n", p.x.0, -p.y.0,))
                    }
                    PathItem::CubicTo(p1, p2, p3) => ops.push_str(&format!(
                        "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c\n",
                        p1.x.0, -p1.y.0, p2.x.0, -p2.y.0, p3.x.0, -p3.y.0,
                    )),
                    PathItem::ClosePath => ops.push_str("h\n"),
                }
            }
        }
        _ => {} // neutro: N16[α] — FrameItem não-Shape sem path a emitir no stream PDF (despacho modular fechado)
    }
}

/// **P242 (M9d / M7+5)** — emite operadores PDF para um rectângulo com
/// cantos arredondados via Bezier 4 corners (paridade vanilla
/// `typst-pdf/.../shape.rs::draw_rounded_rect`).
///
/// Coordenadas em sistema PDF (Y crescente para cima). `(x, y)` é o
/// canto inferior-esquerdo; `w` largura; `h` altura positivos. `radii`
/// em `Corners<Pt>` (top_left/top_right/bottom_right/bottom_left
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
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    radii: &typst_core::entities::corners::Corners<
        typst_core::entities::layout_types::Pt,
    >,
) {
    // P1133 — raios já resolvidos em L1. Clamp cada raio a metade da menor dimensão (paridade vanilla evita
    // overflow geométrico).
    let max_r = (w.min(h)) / 2.0;
    let tl = radii.top_left.0.clamp(0.0, max_r);
    let tr = radii.top_right.0.clamp(0.0, max_r);
    let br = radii.bottom_right.0.clamp(0.0, max_r);
    let bl = radii.bottom_left.0.clamp(0.0, max_r);

    // Sentido horário em PDF coords (Y para cima). Sequência:
    // start top-left edge → top edge → top-right corner → right edge →
    // bottom-right corner → bottom edge → bottom-left corner → left edge →
    // top-left corner → close.
    let x_left = x;
    let x_right = x + w;
    let y_top = y + h;
    let y_bottom = y;

    // MoveTo: começa no início da edge top (após canto top-left).
    ops.push_str(&format!("{:.3} {:.3} m\n", x_left + tl, y_top));
    // Linha top edge.
    ops.push_str(&format!("{:.3} {:.3} l\n", x_right - tr, y_top));
    // Cubic top-right corner.
    if tr > 0.0 {
        ops.push_str(&format!(
            "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
            x_right - tr + tr * pdf_defaults::BEZIER_CIRCLE_KAPPA,
            y_top,
            x_right,
            y_top - tr + tr * pdf_defaults::BEZIER_CIRCLE_KAPPA,
            x_right,
            y_top - tr
        ));
    }
    // Linha right edge.
    ops.push_str(&format!("{:.3} {:.3} l\n", x_right, y_bottom + br));
    // Cubic bottom-right corner.
    if br > 0.0 {
        ops.push_str(&format!(
            "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
            x_right,
            y_bottom + br - br * pdf_defaults::BEZIER_CIRCLE_KAPPA,
            x_right - br + br * pdf_defaults::BEZIER_CIRCLE_KAPPA,
            y_bottom,
            x_right - br,
            y_bottom
        ));
    }
    // Linha bottom edge.
    ops.push_str(&format!("{:.3} {:.3} l\n", x_left + bl, y_bottom));
    // Cubic bottom-left corner.
    if bl > 0.0 {
        ops.push_str(&format!(
            "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
            x_left + bl - bl * pdf_defaults::BEZIER_CIRCLE_KAPPA,
            y_bottom,
            x_left,
            y_bottom + bl - bl * pdf_defaults::BEZIER_CIRCLE_KAPPA,
            x_left,
            y_bottom + bl
        ));
    }
    // Linha left edge.
    ops.push_str(&format!("{:.3} {:.3} l\n", x_left, y_top - tl));
    // Cubic top-left corner.
    if tl > 0.0 {
        ops.push_str(&format!(
            "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
            x_left,
            y_top - tl + tl * pdf_defaults::BEZIER_CIRCLE_KAPPA,
            x_left + tl - tl * pdf_defaults::BEZIER_CIRCLE_KAPPA,
            y_top,
            x_left + tl,
            y_top
        ));
    }
    // Fecha o path.
    ops.push_str("h\n");
}

/// **P1120** — abre o envelope de reflexão do texto dentro de um `Group`.
///
/// O `cm` do `Group` (§P1119) é a matriz Typst→PDF **completa**
/// (`[a, -b, c, -d]`, com o flip do eixo Y já composto) — a mesma que o
/// vanilla emite por bloco de glifo. Os helpers de emissão de texto são
/// partilhados com o caminho top-level e assumem que o flip ainda **não**
/// foi aplicado: o verbose emite o seu próprio `cm 1 0 0 -1 x y` + `Tm
/// 1 0 0 -1` (duplo flip = glifo direito), o compacto emite `x y Td` sem
/// flip nenhum. Debaixo do `cm` do `Group`, qualquer um dos dois fica com
/// um flip a mais → **glifos espelhados na vertical** (medido em
/// `.typ/sec_36.typ`, P1120; posições certas, orientação invertida).
///
/// A correcção repõe a pré-condição dos helpers: um `cm` de reflexão
/// (`1 0 0 -1 0 0`) antes do bloco e a coordenada reflectida (`-pos.y`)
/// devolvida por esta função. F∘F = I no eixo do glifo (fica direito) e a
/// reflexão da coordenada recoloca a posição y-down local no sítio certo —
/// derivação em `stream.md` §P1120. Fecha com `group_text_flip_close`.
fn group_text_flip_open(ops: &mut String, pos_y: f64) -> f64 {
    ops.push_str("q\n1 0 0 -1 0 0 cm\n");
    -pos_y
}

/// **P1120** — fecha o envelope aberto por `group_text_flip_open`.
fn group_text_flip_close(ops: &mut String) {
    ops.push_str("Q\n");
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
        FrameItem::Shape {
            pos,
            kind,
            width,
            height,
            fill,
            stroke,
            parent_bbox_at_emit,
        } => {
            let local_y = pos.y.0;
            ops.push_str("q\n");
            if let Some(c) = fill {
                let (r, g, b, _) = c.to_rgba_f32();
                ops.push_str(&format!("{:.3} {:.3} {:.3} rg\n", r, g, b));
            }
            if let Some(s) = stroke {
                let effective_bbox = parent_bbox_at_emit.or(parent_bbox_override);
                emit_stroke_paint(
                    ops,
                    &s.paint,
                    s.thickness,
                    effective_bbox,
                    ctx.pat_ptr_to_idx,
                    ctx.pat_refs,
                );
            }
            match kind {
                ShapeKind::Rect => {
                    ops.push_str(&format!(
                        "{:.2} {:.2} {:.2} {:.2} re\n",
                        pos.x.0, local_y, width, height
                    ));
                }
                ShapeKind::RoundedRect { radii } => {
                    // P242 — Bezier 4 corners em espaço local.
                    emit_rounded_rect_ops(
                        &mut *ops, pos.x.0, local_y, *width, *height, radii,
                    );
                }
                ShapeKind::Ellipse => {
                    let cx = pos.x.0 + width / 2.0;
                    let cy = local_y + height / 2.0;
                    let rx = width / 2.0;
                    let ry = height / 2.0;
                    let ox = rx * pdf_defaults::BEZIER_CIRCLE_KAPPA;
                    let oy = ry * pdf_defaults::BEZIER_CIRCLE_KAPPA;
                    ops.push_str(&format!("{:.3} {:.3} m\n", cx, cy + ry));
                    ops.push_str(&format!(
                        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx + ox,
                        cy + ry,
                        cx + rx,
                        cy + oy,
                        cx + rx,
                        cy
                    ));
                    ops.push_str(&format!(
                        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx + rx,
                        cy - oy,
                        cx + ox,
                        cy - ry,
                        cx,
                        cy - ry
                    ));
                    ops.push_str(&format!(
                        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx - ox,
                        cy - ry,
                        cx - rx,
                        cy - oy,
                        cx - rx,
                        cy
                    ));
                    ops.push_str(&format!(
                        "{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} c\n",
                        cx - rx,
                        cy + oy,
                        cx - ox,
                        cy + ry,
                        cx,
                        cy + ry
                    ));
                }
                ShapeKind::Line { dx, dy } => {
                    let start_offset_x = if *dx < 0.0 { *width } else { 0.0 };
                    let end_offset_x = if *dx < 0.0 { 0.0 } else { *width };
                    let start_offset_y = if *dy > 0.0 { *height } else { 0.0 };
                    let end_offset_y = if *dy > 0.0 { 0.0 } else { *height };
                    ops.push_str(&format!(
                        "{:.3} {:.3} m\n",
                        pos.x.0 + start_offset_x,
                        local_y + start_offset_y
                    ));
                    ops.push_str(&format!(
                        "{:.3} {:.3} l\n",
                        pos.x.0 + end_offset_x,
                        local_y + end_offset_y
                    ));
                }
                ShapeKind::Path(items) => {
                    use typst_core::entities::geometry::PathItem;
                    for item in items {
                        match item {
                            PathItem::MoveTo(p) => ops.push_str(&format!(
                                "{:.2} {:.2} m\n",
                                pos.x.0 + p.x.0,
                                -(local_y + p.y.0),
                            )),
                            PathItem::LineTo(p) => ops.push_str(&format!(
                                "{:.2} {:.2} l\n",
                                pos.x.0 + p.x.0,
                                -(local_y + p.y.0),
                            )),
                            PathItem::CubicTo(p1, p2, p3) => ops.push_str(&format!(
                                "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c\n",
                                pos.x.0 + p1.x.0,
                                -(local_y + p1.y.0),
                                pos.x.0 + p2.x.0,
                                -(local_y + p2.y.0),
                                pos.x.0 + p3.x.0,
                                -(local_y + p3.y.0),
                            )),
                            PathItem::ClosePath => ops.push_str("h\n"),
                        }
                    }
                }
            }
            match (fill.is_some(), stroke.is_some()) {
                (true, true) => ops.push_str("B\n"),
                (true, false) => ops.push_str("f\n"),
                (false, true) => ops.push_str("S\n"),
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
        FrameItem::Image {
            pos,
            data,
            width,
            height,
            intrinsic_width,
            intrinsic_height,
            orientation,
            ..
        } => {
            let ptr = std::sync::Arc::as_ptr(data) as usize;
            if let Some(&idx) = ctx.ptr_to_idx.get(&ptr) {
                // Local emit: pos.x/pos.y são coords locais (após Group cm).
                let matrix = image_exif_matrix(
                    *orientation,
                    width.val(),
                    height.val(),
                    pos.x.0,
                    pos.y.0,
                );
                ops.push_str(&format!(
                    "q\n{:.3} {:.3} {:.3} {:.3} {:.3} {:.3} cm\n/{} Do\nQ\n",
                    matrix.a,
                    matrix.b,
                    matrix.c,
                    matrix.d,
                    matrix.tx,
                    matrix.ty,
                    ctx.img_refs[idx].name
                ));
            }
        }
        // **P281** — Text/Glyph/Line arms real (substituem stubs P278/P279).
        // P483 — path primário TextShaped; Text = fallback.
        // **P1120** — envelope de reflexão: ver `group_text_flip_open`. A
        // coordenada passada aos helpers é `-pos.y` (a reflexão do `cm` de
        // abertura devolve-a à posição y-down local).
        FrameItem::TextShaped { pos, glyphs, style, text, units_per_em } => {
            let y_eff = group_text_flip_open(ops, pos.y.0);
            match ctx.mode {
                StreamMode::Compact => emit_shaped_pdf(
                    ops,
                    pos.x.0,
                    y_eff,
                    glyphs,
                    text.as_str(),
                    style,
                    &ctx.font_scenario,
                    *units_per_em,
                ),
                StreamMode::Verbose => emit_shaped_pdf_verbose(
                    ops,
                    pos.x.0,
                    y_eff,
                    glyphs,
                    text.as_str(),
                    style,
                    &ctx.font_scenario,
                    *units_per_em,
                ),
            }
            group_text_flip_close(ops);
        }
        FrameItem::Text { pos, text, style } => {
            let y_eff = group_text_flip_open(ops, pos.y.0);
            match ctx.mode {
                StreamMode::Compact => emit_text_pdf(
                    ops,
                    pos.x.0,
                    y_eff,
                    text.as_str(),
                    style,
                    &ctx.font_scenario,
                ),
                StreamMode::Verbose => emit_text_pdf_verbose(
                    ops,
                    pos.x.0,
                    y_eff,
                    text.as_str(),
                    style,
                    &ctx.font_scenario,
                ),
            }
            group_text_flip_close(ops);
        }
        FrameItem::Glyph { pos, glyph_id, size, style, .. } => {
            let y_eff = group_text_flip_open(ops, pos.y.0);
            match ctx.mode {
                StreamMode::Compact => emit_glyph_pdf(
                    ops,
                    pos.x.0,
                    y_eff,
                    *glyph_id,
                    *size,
                    style,
                    &ctx.font_scenario,
                ),
                StreamMode::Verbose => emit_glyph_pdf_verbose(
                    ops,
                    pos.x.0,
                    y_eff,
                    *glyph_id,
                    *size,
                    style,
                    &ctx.font_scenario,
                ),
            }
            group_text_flip_close(ops);
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
        FrameItem::Semantic { items, .. } => {
            let tagged = ctx.tags == PdfTags::Enabled;
            if tagged {
                let key = item as *const FrameItem as usize;
                let mcid = ctx.semantic_mcids.get(&key).copied().unwrap_or(0);
                ops.push_str(&format!("/Formula << /MCID {mcid} >> BDC\n"));
            }
            for child in items {
                draw_item_local(ops, child, parent_bbox_override, ctx);
            }
            if tagged {
                ops.push_str("EMC\n");
            }
        }
    }
}

#[cfg(test)]
mod stream_tests {
    use super::*;
    use crate::export::StreamMode;
    use typst_core::entities::corners::Corners;
    use typst_core::entities::layout_types::{Color, Point, Pt, ShapedGlyph, TextStyle};

    fn glyph(glyph_id: u16, x_advance: i32) -> ShapedGlyph {
        ShapedGlyph {
            glyph_id,
            x_advance,
            x_offset: 0,
            y_offset: 0,
            cluster: 0,
            char_code: 'A',
        }
    }

    #[test]
    fn p1133_pdf_rounded_rect_preserva_quatro_cantos_em_pt() {
        let mut ops = String::new();
        emit_rounded_rect_ops(
            &mut ops,
            10.0,
            20.0,
            100.0,
            80.0,
            &Corners::new(Pt(2.0), Pt(4.0), Pt(6.0), Pt(8.0)),
        );
        assert!(ops.contains("12.000 100.000 m"));
        assert!(ops.contains("106.000 100.000 l"));
        assert!(ops.contains("110.000 26.000 l"));
        assert!(ops.contains("18.000 20.000 l"));
    }

    fn glyph_xoff(glyph_id: u16, x_advance: i32, x_offset: i32) -> ShapedGlyph {
        ShapedGlyph {
            glyph_id,
            x_advance,
            x_offset,
            y_offset: 0,
            cluster: 0,
            char_code: 'A',
        }
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
            bitmap: None,
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
            bitmap: None,
        };
        emit_shaped_pdf(&mut ops, 0.0, 0.0, &glyphs, "B", &style, &scenario, 1000);
        assert!(
            ops.contains("<0042> 0"),
            "P485: fallback nominal=x_advance → delta TJ = 0"
        );
    }

    #[test]
    fn p485_emit_shaped_type1_nao_usa_tj() {
        // Type1 faz fallback para emit_text_pdf (string plana, sem TJ)
        let mut ops = String::new();
        let glyphs = vec![glyph(0x0043, 600)];
        let style = TextStyle::default();
        emit_shaped_pdf(
            &mut ops,
            0.0,
            0.0,
            &glyphs,
            "C",
            &style,
            &FontScenario::Type1,
            1000,
        );
        assert!(!ops.contains("TJ"), "P485: Type1 não deve usar TJ");
    }

    #[test]
    fn p485_emit_shaped_vazio_sem_output() {
        let mut ops = String::new();
        emit_shaped_pdf(
            &mut ops,
            0.0,
            0.0,
            &[],
            "x",
            &TextStyle::default(),
            &FontScenario::Type1,
            1000,
        );
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
            bitmap: None,
        };
        emit_shaped_pdf(&mut ops, 0.0, 0.0, &glyphs, "B", &style, &scenario, 1000);
        assert!(ops.contains("<0042>"), "P486: GID presente");
        assert!(ops.contains("<0042> 0"), "P486: delta 0 igual a P485");
        let after_bracket = ops.split("[ ").nth(1).unwrap_or("");
        assert!(
            after_bracket.starts_with("<0042>"),
            "P486: x_offset=0 → sem ajuste antes do GID"
        );
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
            bitmap: None,
        };
        emit_shaped_pdf(&mut ops, 0.0, 0.0, &glyphs, "C", &style, &scenario, 1000);
        let after_bracket = ops.split("[ ").nth(1).unwrap_or("");
        assert!(
            after_bracket.starts_with("50 "),
            "P486: x_offset=-50 → '50 ' antes do GID"
        );
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
            bitmap: None,
        };
        emit_shaped_pdf(&mut ops, 0.0, 0.0, &glyphs, "D", &style, &scenario, 1000);
        let after_bracket = ops.split("[ ").nth(1).unwrap_or("");
        assert!(
            after_bracket.starts_with("-30 "),
            "P486: x_offset=30 → '-30 ' antes do GID"
        );
        assert!(ops.contains("<0044>"), "P486: GID presente");
    }

    // P520/P548 — teste do delta model com largura nominal explícita.
    #[test]
    fn p520_emit_shaped_kerning_delta() {
        // Glifo A: x_advance=599, largura nominal (hmtx)=639, upm=1000.
        // O operador TJ subtrai o delta da coordenada horizontal, logo:
        // delta TJ = (639 - 599) / 1000 * 1000 = +40 (aproxima o próximo glifo).
        let mut ops = String::new();
        let glyphs = vec![ShapedGlyph {
            glyph_id: 0x0041,
            x_advance: 599,
            x_offset: 0,
            y_offset: 0,
            cluster: 0,
            char_code: 'A',
        }];
        let style = TextStyle::default();
        let mut nominal = std::collections::HashMap::new();
        nominal.insert(0x0041, 639);
        let scenario = FontScenario::Cidfont {
            char_to_gid: &std::collections::HashMap::new(),
            glyph_mapping: &std::collections::HashMap::new(),
            glyph_to_nominal: &nominal,
            bitmap: None,
        };
        emit_shaped_pdf(&mut ops, 0.0, 0.0, &glyphs, "A", &style, &scenario, 1000);
        assert!(
            ops.contains("<0041> 40"),
            "P548: kerning negativo → delta positivo (aproxima)"
        );
    }

    // ── P788 — filhos de FrameItem::Link ao nível da página têm flip Y ─────
    // Bug (medido via CLI, P786 A9 + `#link` genérico): os filhos eram
    // desenhados por `draw_item_local` (sem flip — assume matriz de Group)
    // e apareciam no fundo da página (y≈754 em vez de y≈68).
    #[test]
    fn p788_link_top_level_filho_tem_flip_y() {
        use typst_core::entities::layout_types::{
            LinkTarget, Point, Pt, Size, TextStyle,
        };
        let ptr: HashMap<usize, usize> = HashMap::new();
        let imgs: Vec<ImageRef> = vec![];
        let pats: HashMap<DedupKey, usize> = HashMap::new();
        let pat_refs: Vec<PatternRef> = vec![];
        let ctx = PageContext::type1(&ptr, &imgs, &pats, &pat_refs, StreamMode::Compact);
        let child = FrameItem::Text {
            pos: Point { x: Pt(70.0), y: Pt(100.0) },
            text: "Clique".into(),
            style: TextStyle::default(),
        };
        let page = Page {
            width: 595.0,
            height: 800.0,
            numbering: None,
            supplement: typst_core::entities::content::Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items: vec![FrameItem::Link {
                target: LinkTarget::Url("https://example.com".into()),
                items: vec![child],
                pos: Point { x: Pt(70.0), y: Pt(100.0) },
                size: Size { width: Pt(40.0), height: Pt(12.0) },
            }],
        };
        let stream = String::from_utf8(build_page_stream(&page, &ctx)).unwrap();
        // Flip correcto: pdf_y = 800 - 100 = 700. O bug emitia 100 (sem flip).
        assert!(stream.contains("70.0 700.0 Td"), "filho de Link sem flip Y: {stream}");
        assert!(
            !stream.contains("70.0 100.0 Td"),
            "coordenada crua (bug) presente: {stream}"
        );
    }

    // ── P956 — StreamMode: envelope verbose (vanilla-espelhado) vs compact ──
    //
    // Ver `00_nucleo/prompts/infra/export/stream.md` §P956. Por run de texto,
    // o modo verbose emite:
    //   q 1 0 0 -1 {x} {y_eff} cm
    //   /c0 cs {r} {g} {b} scn
    //   BT 0 Tr /F{n} {size} Tf [Tc] 1 0 0 -1 0 0 Tm […] TJ ET
    //   Q
    // (y_eff = page_height − pos.y no top-level; pos.y directo no local).
    // O modo compact é o formato Passo 20, byte-inalterado.

    /// PageContext Type1 com o modo explícito (regra Fase B.3 de P956).
    fn p956_type1_ctx<'a>(
        ptr: &'a HashMap<usize, usize>,
        imgs: &'a [ImageRef],
        pats: &'a HashMap<DedupKey, usize>,
        pat_refs: &'a [PatternRef],
        mode: StreamMode,
    ) -> PageContext<'a> {
        PageContext::type1(ptr, imgs, pats, pat_refs, mode)
    }

    fn p956_page_800(items: Vec<FrameItem>) -> Page {
        Page {
            width: 595.0,
            height: 800.0,
            numbering: None,
            supplement: typst_core::entities::content::Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items,
        }
    }

    fn p956_text_item(x: f64, y: f64, text: &str, style: TextStyle) -> FrameItem {
        FrameItem::Text {
            pos: Point { x: Pt(x), y: Pt(y) },
            text: text.into(),
            style,
        }
    }

    /// Extrai os números de uma linha de operador PDF (ex.: o par `{x} {y}`
    /// de `1 0 0 -1 {x} {y} cm`, ou os componentes de `/c0 cs {r} {g} {b} scn`).
    fn p956_numbers(s: &str) -> Vec<f64> {
        s.split_whitespace().filter_map(|t| t.parse::<f64>().ok()).collect()
    }

    #[test]
    fn p956_stream_mode_default_e_verbose() {
        // mod.md §P956: Verbose é o novo padrão de produção.
        assert_eq!(StreamMode::default(), StreamMode::Verbose);
    }

    #[test]
    fn p956_verbose_text_envelope_vanilla() {
        let ptr: HashMap<usize, usize> = HashMap::new();
        let imgs: Vec<ImageRef> = vec![];
        let pats: HashMap<DedupKey, usize> = HashMap::new();
        let pat_refs: Vec<PatternRef> = vec![];
        let ctx = p956_type1_ctx(&ptr, &imgs, &pats, &pat_refs, StreamMode::Verbose);
        let page = p956_page_800(vec![p956_text_item(
            70.0,
            100.0,
            "Hello",
            TextStyle::regular(Pt(11.0)),
        )]);
        let s = String::from_utf8(build_page_stream(&page, &ctx)).unwrap();
        // pdf_y = 800 − 100 = 700 (mesmo valor que o compacto passa a Td).
        // Presença E ordem dos operadores do envelope (stream.md §P956).
        let tokens = [
            "q\n",
            "1 0 0 -1 70.00000 700.00000 cm\n",
            "/c0 cs ",
            " scn",
            "BT\n",
            "0 Tr",
            "/F1 11.0 Tf",
            "1 0 0 -1 0 0 Tm",
            "(Hello) Tj",
            "ET\n",
            "Q\n",
        ];
        let mut cursor = 0;
        for tok in tokens {
            let rel = s[cursor..].find(tok).unwrap_or_else(|| {
                panic!("token ausente ou fora de ordem: {tok:?} em {s:?}")
            });
            cursor += rel + tok.len();
        }
        // Fill por omissão (style.fill = None) → preto 0 0 0.
        let cs_pos = s.find("/c0 cs ").expect("/c0 cs presente");
        let scn_pos = s[cs_pos..].find(" scn").expect("scn presente") + cs_pos;
        let comps = p956_numbers(&s[cs_pos + "/c0 cs ".len()..scn_pos]);
        assert_eq!(comps, vec![0.0, 0.0, 0.0], "fill default → preto 0 0 0: {s}");
    }

    #[test]
    fn p956_verbose_posicao_final_equivale_compact() {
        // Derivação matricial F·F=I (stream.md §P956): a baseline final é a
        // mesma nos dois modos — o compacto coloca-a no Td, o verbose no cm
        // (Tm é sempre `1 0 0 -1 0 0`).
        let ptr: HashMap<usize, usize> = HashMap::new();
        let imgs: Vec<ImageRef> = vec![];
        let pats: HashMap<DedupKey, usize> = HashMap::new();
        let pat_refs: Vec<PatternRef> = vec![];
        let mk_item =
            || p956_text_item(70.0, 100.0, "Hello", TextStyle::regular(Pt(11.0)));
        let page_c = p956_page_800(vec![mk_item()]);
        let page_v = p956_page_800(vec![mk_item()]);
        let ctx_c = p956_type1_ctx(&ptr, &imgs, &pats, &pat_refs, StreamMode::Compact);
        let ctx_v = p956_type1_ctx(&ptr, &imgs, &pats, &pat_refs, StreamMode::Verbose);
        let compact = String::from_utf8(build_page_stream(&page_c, &ctx_c)).unwrap();
        let verbose = String::from_utf8(build_page_stream(&page_v, &ctx_v)).unwrap();

        let td_op = compact.find(" Td").expect("compacto tem Td");
        let td_start = compact[..td_op].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let td = p956_numbers(&compact[td_start..td_op]);
        assert_eq!(td.len(), 2, "Td tem x y: {compact}");

        let cm_op = verbose.find(" cm\n").expect("verbose tem cm");
        let cm_start = verbose[..cm_op].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let cm = p956_numbers(&verbose[cm_start..cm_op]);
        assert_eq!(cm.len(), 6, "cm tem 6 componentes: {verbose}");
        assert_eq!(&cm[..4], &[1.0, 0.0, 0.0, -1.0], "cm carrega o flip Y");
        assert_eq!(
            &cm[4..],
            &td[..],
            "baseline final idêntica nos dois modos (F·F=I): verbose={verbose} compact={compact}"
        );
    }

    #[test]
    fn p956_verbose_dois_blocos_isolados_q_q() {
        let ptr: HashMap<usize, usize> = HashMap::new();
        let imgs: Vec<ImageRef> = vec![];
        let pats: HashMap<DedupKey, usize> = HashMap::new();
        let pat_refs: Vec<PatternRef> = vec![];
        let ctx = p956_type1_ctx(&ptr, &imgs, &pats, &pat_refs, StreamMode::Verbose);
        let page = p956_page_800(vec![
            p956_text_item(10.0, 20.0, "A", TextStyle::regular(Pt(11.0))),
            p956_text_item(30.0, 40.0, "B", TextStyle::regular(Pt(11.0))),
        ]);
        let s = String::from_utf8(build_page_stream(&page, &ctx)).unwrap();
        // Cada run gera um bloco q…Q independente, com o seu cm próprio.
        assert_eq!(s.matches(" cm\n").count(), 2, "um cm por run: {s}");
        assert_eq!(s.matches("BT\n").count(), 2, "um BT por run: {s}");
        assert_eq!(s.matches("0 Tr").count(), 2, "0 Tr explícito por bloco: {s}");
        assert!(s.contains("1 0 0 -1 10.00000 780.00000 cm\n"), "cm do 1º run: {s}");
        assert!(s.contains("1 0 0 -1 30.00000 760.00000 cm\n"), "cm do 2º run: {s}");
        assert!(s.starts_with("q\n") && s.ends_with("Q\n"), "stream = blocos q…Q: {s}");
        assert!(s.contains("Q\nq\n"), "blocos q…Q independentes e sequenciais: {s}");
    }

    #[test]
    fn p956_verbose_fill_cor_transicao_cs_scn() {
        let ptr: HashMap<usize, usize> = HashMap::new();
        let imgs: Vec<ImageRef> = vec![];
        let pats: HashMap<DedupKey, usize> = HashMap::new();
        let pat_refs: Vec<PatternRef> = vec![];
        let ctx = p956_type1_ctx(&ptr, &imgs, &pats, &pat_refs, StreamMode::Verbose);
        let mut red = TextStyle::regular(Pt(11.0));
        red.fill = Some(Color::rgb(255, 0, 0));
        let page = p956_page_800(vec![
            p956_text_item(10.0, 20.0, "A", red),
            p956_text_item(30.0, 40.0, "B", TextStyle::regular(Pt(11.0))),
        ]);
        let s = String::from_utf8(build_page_stream(&page, &ctx)).unwrap();
        // Dois blocos → dois `/c0 cs … scn`, na ordem dos runs.
        let mut comps_por_bloco = Vec::new();
        let mut rest = s.as_str();
        while let Some(cs) = rest.find("/c0 cs ") {
            let after = &rest[cs + "/c0 cs ".len()..];
            let scn = after.find(" scn").expect("cada cs tem scn");
            comps_por_bloco.push(p956_numbers(&after[..scn]));
            rest = &after[scn..];
        }
        assert_eq!(comps_por_bloco.len(), 2, "dois blocos cs/scn: {s}");
        assert_eq!(comps_por_bloco[0], vec![1.0, 0.0, 0.0], "fill vermelho: {s}");
        assert_eq!(comps_por_bloco[1], vec![0.0, 0.0, 0.0], "sem fill → preto: {s}");
    }

    #[test]
    fn p956_verbose_tr_explicito_e_faux_bold() {
        let ptr: HashMap<usize, usize> = HashMap::new();
        let imgs: Vec<ImageRef> = vec![];
        let pats: HashMap<DedupKey, usize> = HashMap::new();
        let pat_refs: Vec<PatternRef> = vec![];

        // Texto normal → `0 Tr` explícito (sem stroke).
        let ctx = p956_type1_ctx(&ptr, &imgs, &pats, &pat_refs, StreamMode::Verbose);
        let page = p956_page_800(vec![p956_text_item(
            10.0,
            20.0,
            "N",
            TextStyle::regular(Pt(11.0)),
        )]);
        let s = String::from_utf8(build_page_stream(&page, &ctx)).unwrap();
        assert!(s.contains("0 Tr"), "0 Tr explícito em texto normal: {s}");
        assert!(!s.contains("2 Tr"), "sem faux-bold → sem 2 Tr: {s}");

        // Faux-bold (P139): weight 700 @ 11pt → stroke = 0.44 → `2 Tr` + `w`.
        let ctx = p956_type1_ctx(&ptr, &imgs, &pats, &pat_refs, StreamMode::Verbose);
        let mut bold = TextStyle::bold(Pt(11.0));
        bold.weight = Some(700);
        let page = p956_page_800(vec![p956_text_item(10.0, 20.0, "B", bold)]);
        let s = String::from_utf8(build_page_stream(&page, &ctx)).unwrap();
        assert!(s.contains("2 Tr"), "faux-bold → 2 Tr: {s}");
        assert!(s.contains("0.440 w"), "faux-bold → stroke 0.44 w: {s}");
        assert!(!s.contains("0 Tr"), "faux-bold substitui o 0 Tr: {s}");
    }

    #[test]
    fn p956_compact_preserva_formato_passo20() {
        // O modo compact é o formato Passo 20, byte-inalterado (sem q/cm/cs/Tm/Tr).
        let ptr: HashMap<usize, usize> = HashMap::new();
        let imgs: Vec<ImageRef> = vec![];
        let pats: HashMap<DedupKey, usize> = HashMap::new();
        let pat_refs: Vec<PatternRef> = vec![];
        let ctx = p956_type1_ctx(&ptr, &imgs, &pats, &pat_refs, StreamMode::Compact);
        let page = p956_page_800(vec![p956_text_item(
            70.0,
            100.0,
            "Hello",
            TextStyle::regular(Pt(11.0)),
        )]);
        let s = String::from_utf8(build_page_stream(&page, &ctx)).unwrap();
        assert_eq!(
            s, "BT\n/F1 11.0 Tf\n70.0 700.0 Td\n(Hello) Tj\nET\n",
            "compact = formato Passo 20 byte-exact"
        );
    }

    #[test]
    fn p1120_verbose_group_filho_local_reflectido() {
        // **P1120** — caminho local (`FrameItem::Group`): o `cm` do grupo já
        // traz a conversão Typst→PDF completa (flip incluído). O bloco de
        // texto do filho traz o seu próprio flip (`cm 1 0 0 -1` + `Tm
        // 1 0 0 -1`), logo tem de ser precedido pelo `cm` de reflexão e
        // receber `-pos.y` — senão o glifo sai espelhado na vertical (era o
        // caso até P1120, medido em `.typ/sec_36.typ`). O ponto final é o
        // mesmo: F∘F = I no eixo do glifo, `-(-15) = 15` na coordenada.
        let ptr: HashMap<usize, usize> = HashMap::new();
        let imgs: Vec<ImageRef> = vec![];
        let pats: HashMap<DedupKey, usize> = HashMap::new();
        let pat_refs: Vec<PatternRef> = vec![];
        let ctx = p956_type1_ctx(&ptr, &imgs, &pats, &pat_refs, StreamMode::Verbose);
        let child = p956_text_item(10.0, 15.0, "Hi", TextStyle::regular(Pt(11.0)));
        let group = FrameItem::Group {
            pos: Point { x: Pt(50.0), y: Pt(60.0) },
            matrix: TransformMatrix::identity(),
            clip_mask: None,
            inner_width: 100.0,
            inner_height: 50.0,
            items: vec![child],
        };
        let page = Page {
            width: 595.0,
            height: 842.0,
            numbering: None,
            supplement: typst_core::entities::content::Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items: vec![group],
        };
        let s = String::from_utf8(build_page_stream(&page, &ctx)).unwrap();
        assert!(
            s.contains("q\n1 0 0 -1 0 0 cm\n"),
            "filho local: `cm` de reflexão antes do bloco: {s}"
        );
        assert!(
            s.contains("1 0 0 -1 10.00000 -15.00000 cm\n"),
            "filho local: coordenada reflectida (−pos.y) sob o `cm` de reflexão: {s}"
        );
        assert!(
            !s.contains("10.00000 827.00000 cm"),
            "flip de página no caminho local seria bug (842 − 15 = 827): {s}"
        );
        assert!(s.contains("1 0 0 -1 0 0 Tm"), "Tm constante no filho: {s}");
        assert!(s.contains("(Hi) Tj"), "conteúdo do filho preservado: {s}");

        // O produto das três matrizes (grupo → reflexão → bloco) tem de dar
        // um glifo direito (d = +1 relativo ao espaço do grupo) e a posição
        // y-down local: verificação algébrica do envelope emitido.
        let m_tm = [1.0, 0.0, 0.0, -1.0, 0.0, 0.0];
        let m_block = [1.0, 0.0, 0.0, -1.0, 10.0, -15.0];
        let m_reflect = [1.0, 0.0, 0.0, -1.0, 0.0, 0.0];
        // `cm` do grupo: identidade Typst → PDF na página de 842.
        let m_group = [1.0, 0.0, 0.0, -1.0, 50.0, 842.0 - 60.0];
        let mul = |a: [f64; 6], b: [f64; 6]| {
            [
                a[0] * b[0] + a[1] * b[2],
                a[0] * b[1] + a[1] * b[3],
                a[2] * b[0] + a[3] * b[2],
                a[2] * b[1] + a[3] * b[3],
                a[4] * b[0] + a[5] * b[2] + b[4],
                a[4] * b[1] + a[5] * b[3] + b[5],
            ]
        };
        let net = mul(mul(mul(m_tm, m_block), m_reflect), m_group);
        assert!(
            (net[0] - 1.0).abs() < 1e-9 && (net[3] - 1.0).abs() < 1e-9,
            "glifo direito (sem espelho): net = {net:?}"
        );
        assert!(
            (net[4] - 60.0).abs() < 1e-9 && (net[5] - (842.0 - 75.0)).abs() < 1e-9,
            "posição = origem do grupo + local y-down (60, 842−75): net = {net:?}"
        );
    }

    #[test]
    fn p956_verbose_glyph_envelope_cidfont() {
        // emit_glyph_pdf (stretchy): mesmo envelope verbose com `<gid> Tj`.
        let ptr: HashMap<usize, usize> = HashMap::new();
        let imgs: Vec<ImageRef> = vec![];
        let pats: HashMap<DedupKey, usize> = HashMap::new();
        let pat_refs: Vec<PatternRef> = vec![];
        let char_to_gid: HashMap<char, u16> = HashMap::new();
        let glyph_mapping: HashMap<u16, u16> = HashMap::new();
        let glyph_to_nominal: HashMap<u16, i32> = HashMap::new();
        let ctx = PageContext::cidfont(
            &ptr,
            &imgs,
            &pats,
            &pat_refs,
            &char_to_gid,
            &glyph_mapping,
            &glyph_to_nominal,
            None,
            StreamMode::Verbose,
        );
        let item = FrameItem::Glyph {
            pos: Point { x: Pt(10.0), y: Pt(20.0) },
            glyph_id: 42,
            x_advance: Pt(10.0),
            size: Pt(12.0),
            style: TextStyle::regular(Pt(12.0)),
            base_char: 'x',
        };
        let page = p956_page_800(vec![item]);
        let s = String::from_utf8(build_page_stream(&page, &ctx)).unwrap();
        let tokens = [
            "q\n",
            "1 0 0 -1 10.00000 780.00000 cm\n",
            "/c0 cs ",
            "BT\n",
            "0 Tr",
            "/F1 12.0 Tf",
            "1 0 0 -1 0 0 Tm",
            "<002A> Tj",
            "ET\n",
            "Q\n",
        ];
        let mut cursor = 0;
        for tok in tokens {
            let rel = s[cursor..].find(tok).unwrap_or_else(|| {
                panic!("token ausente ou fora de ordem: {tok:?} em {s:?}")
            });
            cursor += rel + tok.len();
        }
    }
}
