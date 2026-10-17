//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/stream.md
//! @prompt-hash 0e4af887
//! @layer L3
//! @updated 2026-09-28
//!
//! Shared PDF emission context and stable static reexports for PdfBuilder.
#![allow(deprecated)]

use super::{
    escape_pdf_string, group_bbox_from_fields, pdf_defaults, remap_glyph_id,
    spot_bbox_for_shape, text_to_hex_string, DedupKey, ImageRef, PatternRef, PdfTags,
    StreamMode,
};
use crate::export::FontKey;
use std::collections::HashMap;
use typst_core::entities::layout_types::{FrameItem, Page};

mod page;
pub(super) use self::page::build_page_stream;
mod text;
pub(crate) use self::text::font_index_for_style;
pub(super) use self::text::{
    emit_glyph_pdf, emit_glyph_pdf_marked, emit_glyph_pdf_verbose,
    emit_glyph_pdf_verbose_marked, emit_shaped_pdf, emit_shaped_pdf_deferred,
    emit_shaped_pdf_verbose, emit_shaped_pdf_verbose_deferred, emit_text_pdf,
    emit_text_pdf_deferred, emit_text_pdf_verbose, emit_text_pdf_verbose_deferred,
};
use self::text::{emit_verbose_text_run, verbose_run_end};
mod shape;
use self::shape::{
    emit_fill_paint, line_cap_prefix, paint_effective_bbox, spot_fill_operator,
};
pub(super) use self::shape::{
    emit_rounded_rect_ops, emit_shape_path_local, emit_stroke_paint, fill_rg_prefix,
    line_rg_prefix, opacity_name, opacity_operator, solid_opacity, spot_resource_name,
};
mod draw;
mod separator;
mod text_replacement;
use self::draw::draw_item_top;

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
        bitmap: Option<&'a HashMap<u16, crate::export::bitmap_glyphs::BitmapGlyphRef>>,
    },
    /// Multifont Identity-H com selecção `/F{fi+1}` por `(style.font, variant)`.
    Multifont {
        fonts: &'a [FontKey],
        bindings: Option<crate::export::font_binding::LookupCache>,
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
            &'a [Option<HashMap<u16, crate::export::bitmap_glyphs::BitmapGlyphRef>>],
    },
}

/// **P281** — agregador de contexto para emit unificado.
pub(crate) struct PageContext<'a> {
    pub ptr_to_idx: &'a HashMap<usize, usize>,
    pub img_refs: &'a [ImageRef],
    pub pat_ptr_to_idx: &'a HashMap<DedupKey, usize>,
    pub pat_refs: &'a [PatternRef],
    pub tiling_names: Option<&'a HashMap<super::pdf_tiling::Key, String>>,
    pub font_scenario: FontScenario<'a>,
    /// **P956** — modo de emissão de texto (ADR-0126): `Verbose` (envelope
    /// vanilla-espelhado, novo padrão) ou `Compact` (formato Passo 20,
    /// byte-inalterado). Dispatch em `draw_item_top`/`draw_item_local`.
    pub mode: StreamMode,
    /// **P1140.6** — tagging e MCIDs pela identidade de cada envelope.
    pub tags: PdfTags,
    pub semantic_mcids: HashMap<usize, usize>,
    /// P1481: marca folhas reais somente no perfil PDF/UA-1.
    pub ua1: bool,
    /// **P980/P983** — caminho do oráculo de paridade de operador
    /// (`--oracle-pdf`; `prompts/infra/export/oracle.md`). Quando activo,
    /// itens `style.math` não participam no agrupamento de runs de P979
    /// (o vanilla nunca funde glifos math — `oracle.md` §P983).
    /// `false` no caminho normal.
    pub oracle: bool,
}

impl<'a> PageContext<'a> {
    pub(crate) fn with_font_scenario(
        ptr_to_idx: &'a HashMap<usize, usize>,
        img_refs: &'a [ImageRef],
        pat_ptr_to_idx: &'a HashMap<DedupKey, usize>,
        pat_refs: &'a [PatternRef],
        font_scenario: FontScenario<'a>,
        mode: StreamMode,
    ) -> Self {
        Self {
            ptr_to_idx,
            img_refs,
            pat_ptr_to_idx,
            pat_refs,
            tiling_names: None,
            font_scenario,
            mode,
            tags: PdfTags::Disabled,
            semantic_mcids: HashMap::new(),
            ua1: false,
            oracle: false,
        }
    }

    pub(crate) fn type1(
        ptr_to_idx: &'a HashMap<usize, usize>,
        img_refs: &'a [ImageRef],
        pat_ptr_to_idx: &'a HashMap<DedupKey, usize>,
        pat_refs: &'a [PatternRef],
        mode: StreamMode,
    ) -> Self {
        Self::with_font_scenario(
            ptr_to_idx,
            img_refs,
            pat_ptr_to_idx,
            pat_refs,
            FontScenario::Type1,
            mode,
        )
    }

    /// **P980/P983** — activa o caminho do oráculo neste contexto.
    pub(crate) fn with_oracle(mut self, oracle: bool) -> Self {
        self.oracle = oracle;
        self
    }

    pub(crate) fn with_pdf_tags(mut self, tags: PdfTags, page: &Page, ua1: bool) -> Self {
        self.tags = tags;
        self.ua1 = ua1;
        if tags == PdfTags::Enabled {
            fn collect(
                item: &FrameItem,
                next: &mut usize,
                out: &mut HashMap<usize, usize>,
                inside_artifact: bool,
                ua1: bool,
            ) {
                match item {
                    FrameItem::Semantic { kind, items, .. } => {
                        if ua1 && matches!(kind,
                            typst_core::entities::layout_types::SemanticKind::ParbreakBoundary
                            | typst_core::entities::layout_types::SemanticKind::ExplicitLinebreakBoundary
                        ) { return; }
                        let is_artifact = matches!(
                            kind,
                            typst_core::entities::layout_types::SemanticKind::Artifact(_)
                        );
                        let owns_mcid = matches!(
                            kind,
                            typst_core::entities::layout_types::SemanticKind::Formula
                                | typst_core::entities::layout_types::SemanticKind::Label
                                | typst_core::entities::layout_types::SemanticKind::TableHeaderCell { .. }
                                | typst_core::entities::layout_types::SemanticKind::TableDataCell { .. }
                        );
                        if !inside_artifact && owns_mcid {
                            out.insert(item as *const FrameItem as usize, *next);
                            *next += 1;
                            if ua1 {
                                return;
                            }
                        }
                        for child in items {
                            collect(
                                child,
                                next,
                                out,
                                inside_artifact || is_artifact,
                                ua1,
                            );
                        }
                    }
                    FrameItem::TextReplacement { items, .. }
                    | FrameItem::Group { items, .. }
                    | FrameItem::Link { items, .. } => {
                        for child in items {
                            collect(child, next, out, inside_artifact, ua1);
                        }
                    }
                    FrameItem::TextSeparator { .. }
                    | FrameItem::Text { .. }
                    | FrameItem::TextShaped { .. }
                    | FrameItem::Glyph { .. }
                    | FrameItem::Image { .. } => {
                        if ua1 && !inside_artifact {
                            out.insert(item as *const FrameItem as usize, *next);
                            *next += 1;
                        }
                    }
                    FrameItem::Line { .. } | FrameItem::Shape { .. } => {}
                }
            }
            let mut next = 0;
            for item in &page.items {
                collect(item, &mut next, &mut self.semantic_mcids, false, ua1);
            }
        }
        self
    }
}

pub(super) fn build_cell_stream(items: &[FrameItem], ctx: &PageContext) -> Vec<u8> {
    let mut ops = String::new();
    for item in items {
        draw::draw_item_local(&mut ops, item, None, ctx);
    }
    ops.into_bytes()
}

#[cfg(test)]
mod stream_tests {
    use super::*;
    use crate::export::StreamMode;
    use typst_core::entities::layout_types::TransformMatrix;
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
        emit_shaped_pdf(
            &mut ops, 72.0, 770.0, &glyphs, "A", &style, &scenario, 1000, None,
        );
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
        emit_shaped_pdf(&mut ops, 0.0, 0.0, &glyphs, "B", &style, &scenario, 1000, None);
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
            None,
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
            None,
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
        emit_shaped_pdf(&mut ops, 0.0, 0.0, &glyphs, "B", &style, &scenario, 1000, None);
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
        emit_shaped_pdf(&mut ops, 0.0, 0.0, &glyphs, "C", &style, &scenario, 1000, None);
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
        emit_shaped_pdf(&mut ops, 0.0, 0.0, &glyphs, "D", &style, &scenario, 1000, None);
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
        emit_shaped_pdf(&mut ops, 0.0, 0.0, &glyphs, "A", &style, &scenario, 1000, None);
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
            bidi: None,
        };
        let page = Page {
            width: 595.0,
            height: 800.0,
            numbering: None,
            logical_number: 1,
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
            logical_number: 1,
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
            bidi: None,
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
            paint_root: false,
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
            logical_number: 1,
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
        let ctx = PageContext::with_font_scenario(
            &ptr,
            &imgs,
            &pats,
            &pat_refs,
            FontScenario::Cidfont {
                char_to_gid: &char_to_gid,
                glyph_mapping: &glyph_mapping,
                glyph_to_nominal: &glyph_to_nominal,
                bitmap: None,
            },
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
