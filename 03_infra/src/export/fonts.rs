//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/fonts.md
//! @prompt-hash 14d23b00
//! @layer L3
//! @updated 2026-05-19
//!
//! Helpers do caminho CIDFont + `escape_pdf_string`.
//!
//! Extraído de `export.rs` em P307b.1 (ADR-0100 / diagnóstico
//! P307a §5). Reusado por:
//! - `mod.rs::build_cidfont`/`build_multifont` (collect + cmap + widths).
//! - `stream/text.rs::emit_text_pdf` (escape para caminho Helvetica).
//! - `mod.rs::build_helvetica` (escape para emit principal).
//!
//! Conteúdo bit-exact pré e pós migração — comportamento idêntico.

#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use typst_core::entities::frame_visitor::{walk_frame_items, FrameVisitor};

use ttf_parser::Face;
use typst_core::entities::layout_types::{FrameItem, PagedDocument};
use typst_core::entities::shaped_glyph::ShapedGlyph;

pub(super) fn escape_pdf_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '(' => out.push_str("\\("),
            ')' => out.push_str("\\)"),
            '\\' => out.push_str("\\\\"),
            c if c.is_ascii() && c >= ' ' => out.push(c),
            _ => out.push('?'),
        }
    }
    out
}

// ── Helpers — caminho CIDFont ──────────────────────────────────────────────

/// Coleciona todos os codepoints Unicode distintos usados no documento.
///
/// **P280** — atravessa `FrameItem::Group` recursivamente (classe A,
/// padrão canónico ver L0 `infra/export.md` §"Walkers top-level —
/// invariante arquitectural"). Pré-P280: bug latent — Text dentro
/// de Group não contribuía chars → `text_to_hex_string` retornaria
/// `<0000>` (notdef) quando P280.X-bis-text-emit-em-group lander.
pub(super) fn collect_codepoints(doc: &PagedDocument) -> Vec<char> {
    struct CodepointCollector<'a>(&'a mut std::collections::BTreeSet<char>);
    impl<'a> FrameVisitor for CodepointCollector<'a> {
        fn visit_text(&mut self, item: &FrameItem) {
            if let FrameItem::Text { text, .. } = item {
                for c in text.chars() {
                    self.0.insert(c);
                }
            }
        }
        fn visit_text_shaped(&mut self, item: &FrameItem) {
            if let FrameItem::TextShaped { glyphs, .. } = item {
                for g in glyphs {
                    self.0.insert(g.char_code);
                }
            }
        }
    }
    let mut seen = std::collections::BTreeSet::new();
    let mut visitor = CodepointCollector(&mut seen);
    for page in &doc.pages {
        walk_frame_items(&mut visitor, &page.items);
    }
    seen.into_iter().collect()
}

/// Coleciona todos os codepoints distintos usados apenas em `FrameItem::Text`
/// (não shaped) no documento. Usado para garantir que glyphs de espaços e
/// outros caracteres do caminho fallback são incluídos no subset.
pub(super) fn collect_text_codepoints(doc: &PagedDocument) -> Vec<char> {
    struct TextCodepointCollector<'a>(&'a mut std::collections::BTreeSet<char>);
    impl<'a> FrameVisitor for TextCodepointCollector<'a> {
        fn visit_text(&mut self, item: &FrameItem) {
            if let FrameItem::Text { text, .. } = item {
                for c in text.chars() {
                    self.0.insert(c);
                }
            }
        }
    }
    let mut seen = std::collections::BTreeSet::new();
    let mut visitor = TextCodepointCollector(&mut seen);
    for page in &doc.pages {
        walk_frame_items(&mut visitor, &page.items);
    }
    seen.into_iter().collect()
}

/// Coleciona todos os glyph IDs distintos usados em `FrameItem::Glyph` no documento.
///
/// **P280** — atravessa `FrameItem::Group` recursivamente (classe A,
/// idem `collect_codepoints`). Pré-P280: bug latent — Glyph dentro
/// de Group não contribuía IDs → ToUnicode CMap incompleto para
/// glyphs em Group.
pub(super) fn collect_glyph_ids(doc: &PagedDocument) -> BTreeSet<u16> {
    struct GlyphIdCollector<'a>(&'a mut BTreeSet<u16>);
    impl<'a> FrameVisitor for GlyphIdCollector<'a> {
        fn visit_glyph(&mut self, item: &FrameItem) {
            if let FrameItem::Glyph { glyph_id, .. } = item {
                self.0.insert(*glyph_id);
            }
        }
        fn visit_text_shaped(&mut self, item: &FrameItem) {
            if let FrameItem::TextShaped { glyphs, .. } = item {
                for g in glyphs {
                    self.0.insert(g.glyph_id);
                }
            }
        }
    }
    let mut ids = BTreeSet::new();
    let mut visitor = GlyphIdCollector(&mut ids);
    for page in &doc.pages {
        walk_frame_items(&mut visitor, &page.items);
    }
    ids
}

/// P521 — coleciona todos os pares `(old_gid, hex_utf16be)` dos glifos shaped
/// no documento, usando `cluster_text` para reconstruir o texto completo de
/// cada cluster (incluindo ligatures e RTL).
pub(super) fn collect_shaped_cluster_texts(doc: &PagedDocument) -> Vec<(u16, String)> {
    struct ClusterTextCollector<'a>(&'a mut Vec<(u16, String)>);
    impl<'a> FrameVisitor for ClusterTextCollector<'a> {
        fn visit_text_shaped(&mut self, item: &FrameItem) {
            if let FrameItem::TextShaped { glyphs, text, .. } = item {
                self.0.extend(cluster_text(glyphs, text));
            }
        }
    }
    let mut out = Vec::new();
    let mut visitor = ClusterTextCollector(&mut out);
    for page in &doc.pages {
        walk_frame_items(&mut visitor, &page.items);
    }
    out
}

/// P520 — coleciona os glifos reais produzidos pelo shaper, juntamente com o
/// caractere representativo do cluster a que pertencem.
///
/// Para ligatures (ex.: "fi" → um único glifo), o `char_code` é o primeiro
/// caractere do cluster (o `ShapedGlyph` já o transporta). Isto permite
/// incluir o glifo de ligature no subset e registar um mapeamento ToUnicode
/// parcial sem duplicar a lógica de walk do documento.
///
/// **P558** — glifos mark (`x_advance == 0`) são ignorados: partilham o
/// `char_code` da base, mas o seu `glyph_id` aponta para o acento combinante,
/// não para o carácter acentuado. Incluí-los fazia com que o subsetter
/// associasse o codepoint composto ao glifo do acento (ex.: "ú" → acute).
pub(super) fn collect_shaped_glyph_mappings(doc: &PagedDocument) -> BTreeMap<u16, char> {
    struct GlyphMappingCollector<'a>(&'a mut BTreeMap<u16, char>);
    impl<'a> FrameVisitor for GlyphMappingCollector<'a> {
        fn visit_text_shaped(&mut self, item: &FrameItem) {
            if let FrameItem::TextShaped { glyphs, .. } = item {
                for g in glyphs {
                    if g.x_advance == 0 {
                        continue;
                    }
                    self.0.entry(g.glyph_id).or_insert(g.char_code);
                }
            }
        }
    }
    let mut out = BTreeMap::new();
    let mut visitor = GlyphMappingCollector(&mut out);
    for page in &doc.pages {
        walk_frame_items(&mut visitor, &page.items);
    }
    out
}

/// Para um conjunto de chars, retorna Vec<(char, glyph_id)>.
/// Chars sem glyph na fonte são omitidos.
pub(super) fn map_chars_to_glyphs(face: &Face<'_>, chars: &[char]) -> Vec<(char, u16)> {
    chars
        .iter()
        .filter_map(|&c| face.glyph_index(c).map(|gid| (c, gid.0)))
        .collect()
}

/// Gera o array W do CIDFont: "gid [width] ..." em unidades PDF (1/1000 text space).
///
/// P521 — `mappings` contém pares `(gid, _hex)`; apenas o `gid` é usado para
/// obter a largura da fonte subset.
pub(super) fn widths_array(face: &Face<'_>, mappings: &[(u16, String)]) -> String {
    let upem = face.units_per_em() as f64;
    let mut parts = Vec::new();
    for (gid, _hex) in mappings {
        let adv = face.glyph_hor_advance(ttf_parser::GlyphId(*gid)).unwrap_or(500) as f64;
        let w = (adv / upem * 1000.0).round() as i32;
        parts.push(format!("{gid} [{w}]"));
    }
    parts.join(" ")
}

/// Converte um caractere numa string hex UTF-16BE (4 dígitos por code unit).
///
/// **P809** — caracteres fora do BMP (ex.: math alphanumeric 𝑥 U+1D465)
/// passam a ser codificados como **par de surrogados** UTF-16BE
/// (`D835 DC65`); antes, `c as u16` truncava para 16 bits (U+1D465 →
/// `D465` = 푥, extracção errada no ToUnicode CMap).
pub(super) fn char_to_utf16_hex(c: char) -> String {
    let mut buf = [0u16; 2];
    c.encode_utf16(&mut buf)
        .iter()
        .map(|u| format!("{:04X}", u))
        .collect()
}

/// Gera o stream ToUnicode CMap para o mapeamento `new_gid → hex UTF-16BE`.
///
/// P521 — `mappings` contém pares `(new_gid, hex_string)` onde `hex_string` é
/// uma sequência de codepoints UTF-16BE em hex (ex.: `"00660069"` para "fi").
/// Emite em blocos de ≤ 100 entradas (limite PDF spec).
pub(super) fn to_unicode_cmap(mappings: &[(u16, String)]) -> Vec<u8> {
    let mut s = String::new();
    s.push_str("/CIDInit /ProcSet findresource begin\n");
    s.push_str("12 dict begin\n");
    s.push_str("begincmap\n");
    s.push_str(
        "/CIDSystemInfo << /Registry (Adobe) /Ordering (UCS) /Supplement 0 >> def\n",
    );
    s.push_str("/CMapName /Adobe-Identity-UCS def\n");
    s.push_str("/CMapType 2 def\n");
    s.push_str("1 begincodespacerange\n");
    s.push_str("<0000> <FFFF>\n");
    s.push_str("endcodespacerange\n");

    for chunk in mappings.chunks(100) {
        s.push_str(&format!("{} beginbfchar\n", chunk.len()));
        for (gid, hex) in chunk {
            s.push_str(&format!("<{gid:04X}> <{hex}>\n"));
        }
        s.push_str("endbfchar\n");
    }

    s.push_str("endcmap\n");
    s.push_str("CMapName currentdict /CMap defineresource pop\n");
    s.push_str("end\nend\n");
    s.into_bytes()
}

/// Serializa texto como string hexadecimal de glyph IDs para Identity-H.
/// Chars sem mapeamento usam glyph ID 0 (notdef).
pub(super) fn text_to_hex_string(text: &str, char_to_gid: &HashMap<char, u16>) -> String {
    let mut hex = String::from("<");
    for c in text.chars() {
        let gid = char_to_gid.get(&c).copied().unwrap_or(0);
        hex.push_str(&format!("{gid:04X}"));
    }
    hex.push('>');
    hex
}

/// P521 — Reconstrói o texto Unicode de cada glifo shaped a partir do
/// `cluster` (byte-index na string original) e das fronteiras de cluster.
///
/// Funciona para runs LTR e RTL porque as fronteiras são calculadas a
/// partir do conjunto ordenado de valores de byte, não da posição no
/// vector de glifos. Glifos mark (mesmo `cluster` que a base) partilham a
/// mesma substring, mas apenas a primeira ocorrência de cada cluster leva
/// o hex completo; as restantes ficam com string vazia.
pub(super) fn cluster_text(glyphs: &[ShapedGlyph], text: &str) -> Vec<(u16, String)> {
    if glyphs.is_empty() {
        return Vec::new();
    }

    // Fronteiras de cluster únicas, ordenadas por byte — independente da
    // ordem visual (LTR/RTL) em que os glyphs aparecem no vector.
    let mut boundaries: Vec<usize> = glyphs.iter().map(|g| g.cluster as usize).collect();
    boundaries.push(text.len());
    boundaries.sort_unstable();
    boundaries.dedup();

    let mut result: Vec<(u16, String)> = Vec::with_capacity(glyphs.len());
    let mut seen_clusters: HashSet<u32> = HashSet::new();

    for g in glyphs {
        let start = g.cluster as usize;
        let end = boundaries.iter().find(|&&b| b > start).copied().unwrap_or(text.len());

        let cluster_str = if start < end
            && text.is_char_boundary(start)
            && text.is_char_boundary(end)
        {
            &text[start..end]
        } else {
            ""
        };

        let mut hex: String =
            cluster_str.encode_utf16().map(|u| format!("{:04X}", u)).collect();

        // Mark glyph: mesmo cluster que uma base já vista → sem entrada própria.
        if !seen_clusters.insert(g.cluster) {
            hex.clear();
        }

        result.push((g.glyph_id, hex));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn glyph(gid: u16, cluster: u32) -> ShapedGlyph {
        ShapedGlyph {
            glyph_id: gid,
            x_advance: 500,
            x_offset: 0,
            y_offset: 0,
            cluster,
            char_code: '\0',
        }
    }

    #[test]
    fn p521_cluster_text_ltr_ligature() {
        // "fi" → 1 glyph, cluster=0
        let glyphs = vec![glyph(5042, 0)];
        let result = cluster_text(&glyphs, "fi");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, 5042);
        assert_eq!(result[0].1, "00660069");
    }

    #[test]
    fn p809_char_to_utf16_hex_non_bmp_surrogate_pair() {
        // P809 — chars fora do BMP (math alphanumeric) passam a UTF-16BE com
        // par de surrogados; antes truncava para 16 bits (U+1D465 → "D465").
        assert_eq!(char_to_utf16_hex('\u{1D465}'), "D835DC65"); // 𝑥
        assert_eq!(char_to_utf16_hex('\u{1D6FC}'), "D835DEFC"); // 𝛼
                                                                // BMP inalterado.
        assert_eq!(char_to_utf16_hex('x'), "0078");
        assert_eq!(char_to_utf16_hex('α'), "03B1");
        assert_eq!(char_to_utf16_hex('é'), "00E9");
    }

    #[test]
    fn p521_cluster_text_ltr_simples() {
        // "café" — 'é' é 2 bytes UTF-8, 1 codepoint
        let text = "café";
        let glyphs = vec![
            glyph(1, 0), // c
            glyph(2, 1), // a
            glyph(3, 2), // f
            glyph(4, 3), // é
        ];
        let result = cluster_text(&glyphs, text);
        assert_eq!(result[0].1, "0063"); // c
        assert_eq!(result[1].1, "0061"); // a
        assert_eq!(result[2].1, "0066"); // f
        assert_eq!(result[3].1, "00E9"); // é
    }

    #[test]
    fn p521_cluster_text_mark_glyph_nao_duplica() {
        // Base + combining acute no mesmo cluster.
        let text = "é"; // U+00E9 como base única para simplificar
        let glyphs = vec![
            glyph(1, 0), // base
            glyph(2, 0), // mark
        ];
        let result = cluster_text(&glyphs, text);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].1, "00E9");
        assert_eq!(result[1].1, "");
    }

    #[test]
    fn p521_cluster_text_rtl_nao_entra_panic() {
        // Simula ordem visual RTL: clusters decrescentes ao longo do vector.
        // Cada caractere ocupa 1 byte; ordem visual inverte a lógica.
        let text = "abc";
        let glyphs = vec![
            glyph(1, 2), // visualmente primeiro = logicamente último (c)
            glyph(2, 1), // b
            glyph(3, 0), // visualmente último = logicamente primeiro (a)
        ];
        let result = cluster_text(&glyphs, text);
        assert_eq!(result.len(), 3);
        // Cada glyph representa um cluster diferente; fronteiras por byte
        // ordenado dão substrings correctas na ordem lógica.
        assert_eq!(result[0].1, "0063");
        assert_eq!(result[1].1, "0062");
        assert_eq!(result[2].1, "0061");
    }

    // ── P558 — mark glyphs não devem poluir o mapeamento shaped ───────────────

    #[test]
    fn p558_shaped_glyph_mappings_ignora_mark_glyphs() {
        // Simula "ú" shaped como base 'u' (gid 87, advance>0) + mark acute
        // (gid 706, advance=0). O mapeamento só deve incluir a base.
        let page = typst_core::entities::layout_types::Page {
            width: 595.0,
            height: 842.0,
            numbering: None,
            items: vec![typst_core::entities::layout_types::FrameItem::TextShaped {
                pos: typst_core::entities::layout_types::Point {
                    x: typst_core::entities::layout_types::Pt(0.0),
                    y: typst_core::entities::layout_types::Pt(0.0),
                },
                glyphs: vec![
                    ShapedGlyph {
                        glyph_id: 87,
                        x_advance: 490,
                        x_offset: 0,
                        y_offset: 0,
                        cluster: 5,
                        char_code: 'ú',
                    },
                    ShapedGlyph {
                        glyph_id: 706,
                        x_advance: 0,
                        x_offset: -85,
                        y_offset: 0,
                        cluster: 5,
                        char_code: 'ú',
                    },
                ],
                style: typst_core::entities::layout_types::TextStyle::default(),
                text: ecow::EcoString::from("Conteúdo").into(),
                units_per_em: 1000,
            }],
        };
        let doc = typst_core::entities::layout_types::PagedDocument::new(vec![page]);
        let mappings = collect_shaped_glyph_mappings(&doc);
        assert!(mappings.contains_key(&87), "base 'u' deve estar no mapeamento");
        assert!(
            !mappings.contains_key(&706),
            "mark glyph (x_advance=0) não deve estar no mapeamento"
        );
        assert_eq!(mappings.get(&87).copied(), Some('ú'));
    }

    // ── P568 — collect_text_codepoints extrai codepoints de FrameItem::Text ─────

    #[test]
    fn p568_collect_text_codepoints_inclui_texto_fallback_e_recursao() {
        use typst_core::entities::layout_types::{Page, Point, Pt, TextStyle};
        let style = TextStyle::regular(Pt(12.0));
        let page = Page {
            width: 595.0,
            height: 842.0,
            numbering: None,
            items: vec![
                FrameItem::Text {
                    pos: Point::ZERO,
                    text: "Olá ".into(),
                    style: style.clone(),
                },
                FrameItem::Group {
                    pos: Point::ZERO,
                    matrix: typst_core::entities::layout_types::TransformMatrix::identity(
                    ),
                    clip_mask: None,
                    inner_width: 100.0,
                    inner_height: 20.0,
                    items: vec![FrameItem::Text {
                        pos: Point::ZERO,
                        text: "mundo!".into(),
                        style,
                    }],
                },
            ],
        };
        let doc = PagedDocument::new(vec![page]);
        let codepoints = collect_text_codepoints(&doc);
        let expected: std::collections::BTreeSet<char> = "Olá mundo!".chars().collect();
        let actual: std::collections::BTreeSet<char> = codepoints.into_iter().collect();
        assert_eq!(actual, expected);
    }

    // ── P1043: Pares de independência testcase() para fonts.rs:33 ───────────

    #[test]
    fn p1043_font_name_sanitize_ascii_printable_isolada() {
        // 33 - C1=T, C2=T: c.is_ascii() && c >= ' ' -> out.push(c)
        assert_eq!(escape_pdf_string("Helvetica-Bold 123"), "Helvetica-Bold 123");
    }

    #[test]
    fn p1043_font_name_sanitize_ascii_control_isolada() {
        // 33 - C1=T, C2=F: c.is_ascii() && c < ' ' -> out.push('?')
        assert_eq!(escape_pdf_string("FontName"), "Font?Name?");
    }

    #[test]
    fn p1043_font_name_sanitize_non_ascii_isolada() {
        // 33 - C1=F, C2=_: !c.is_ascii() -> out.push('?')
        assert_eq!(escape_pdf_string("FontÉtoile"), "Font?toile");
    }
}
