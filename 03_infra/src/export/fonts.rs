//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/fonts.md
//! @prompt-hash c7d24b28
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

use ttf_parser::Face;
use typst_core::entities::layout_types::{FrameItem, PagedDocument};
use typst_core::entities::shaped_glyph::ShapedGlyph;

pub(super) fn escape_pdf_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '('  => out.push_str("\\("),
            ')'  => out.push_str("\\)"),
            '\\' => out.push_str("\\\\"),
            c if c.is_ascii() && c >= ' ' => out.push(c),
            _    => out.push('?'),
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
    fn walk(items: &[FrameItem], seen: &mut std::collections::BTreeSet<char>) {
        for item in items {
            match item {
                FrameItem::Text { text, .. } => {
                    for c in text.chars() {
                        seen.insert(c);
                    }
                }
                FrameItem::TextShaped { glyphs, .. } => {
                    for g in glyphs {
                        seen.insert(g.char_code);
                    }
                }
                FrameItem::Group { items: child, .. }
                | FrameItem::Link { items: child, .. } => walk(child, seen),
                _ => {} // Image, Line, Glyph não contribuem com codepoints de texto.
            }
        }
    }
    let mut seen = std::collections::BTreeSet::new();
    for page in &doc.pages {
        walk(&page.items, &mut seen);
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
    fn walk(items: &[FrameItem], ids: &mut BTreeSet<u16>) {
        for item in items {
            match item {
                FrameItem::Glyph { glyph_id, .. } => {
                    ids.insert(*glyph_id);
                }
                FrameItem::TextShaped { glyphs, .. } => {
                    for g in glyphs {
                        ids.insert(g.glyph_id);
                    }
                }
                FrameItem::Group { items: child, .. }
                | FrameItem::Link { items: child, .. } => walk(child, ids),
                _ => {}
            }
        }
    }
    let mut ids = BTreeSet::new();
    for page in &doc.pages {
        walk(&page.items, &mut ids);
    }
    ids
}

/// P520 — coleciona os glifos reais produzidos pelo shaper, juntamente com o
/// caractere representativo do cluster a que pertencem.
///
/// Para ligatures (ex.: "fi" → um único glifo), o `char_code` é o primeiro
/// caractere do cluster (o `ShapedGlyph` já o transporta). Isto permite
/// incluir o glifo de ligature no subset e registar um mapeamento ToUnicode
/// parcial sem duplicar a lógica de walk do documento.
pub(super) fn collect_shaped_glyph_mappings(doc: &PagedDocument) -> BTreeMap<u16, char> {
    fn walk(items: &[FrameItem], out: &mut BTreeMap<u16, char>) {
        for item in items {
            match item {
                FrameItem::TextShaped { glyphs, .. } => {
                    for g in glyphs {
                        // Preferir o primeiro caractere do cluster; se já
                        // existir uma entrada para este glyph_id, manter a
                        // primeira encontrada (ordem de walk é estável).
                        out.entry(g.glyph_id).or_insert(g.char_code);
                    }
                }
                FrameItem::Group { items: child, .. }
                | FrameItem::Link { items: child, .. } => walk(child, out),
                _ => {}
            }
        }
    }
    let mut out = BTreeMap::new();
    for page in &doc.pages {
        walk(&page.items, &mut out);
    }
    out
}

/// Para um conjunto de chars, retorna Vec<(char, glyph_id)>.
/// Chars sem glyph na fonte são omitidos.
pub(super) fn map_chars_to_glyphs(face: &Face<'_>, chars: &[char]) -> Vec<(char, u16)> {
    chars.iter()
        .filter_map(|&c| face.glyph_index(c).map(|gid| (c, gid.0)))
        .collect()
}

/// Gera o array W do CIDFont: "gid [width] ..." em unidades PDF (1/1000 text space).
pub(super) fn widths_array(face: &Face<'_>, mappings: &[(char, u16)]) -> String {
    let upem = face.units_per_em() as f64;
    let mut parts = Vec::new();
    for (_c, gid) in mappings {
        let adv = face.glyph_hor_advance(ttf_parser::GlyphId(*gid))
            .unwrap_or(500) as f64;
        let w = (adv / upem * 1000.0).round() as i32;
        parts.push(format!("{gid} [{w}]"));
    }
    parts.join(" ")
}

/// Gera o stream ToUnicode CMap para o mapeamento glyph_id → char.
/// Emite em blocos de ≤ 100 entradas (limite PDF spec).
pub(super) fn to_unicode_cmap(mappings: &[(char, u16)]) -> Vec<u8> {
    let mut s = String::new();
    s.push_str("/CIDInit /ProcSet findresource begin\n");
    s.push_str("12 dict begin\n");
    s.push_str("begincmap\n");
    s.push_str("/CIDSystemInfo << /Registry (Adobe) /Ordering (UCS) /Supplement 0 >> def\n");
    s.push_str("/CMapName /Adobe-Identity-UCS def\n");
    s.push_str("/CMapType 2 def\n");
    s.push_str("1 begincodespacerange\n");
    s.push_str("<0000> <FFFF>\n");
    s.push_str("endcodespacerange\n");

    for chunk in mappings.chunks(100) {
        s.push_str(&format!("{} beginbfchar\n", chunk.len()));
        for (c, gid) in chunk {
            let cp = *c as u32;
            s.push_str(&format!("<{gid:04X}> <{cp:04X}>\n"));
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
        let end = boundaries
            .iter()
            .find(|&&b| b > start)
            .copied()
            .unwrap_or(text.len());

        let cluster_str = if start < end
            && text.is_char_boundary(start)
            && text.is_char_boundary(end)
        {
            &text[start..end]
        } else {
            ""
        };

        let mut hex: String = cluster_str
            .encode_utf16()
            .map(|u| format!("{:04X}", u))
            .collect();

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
}
