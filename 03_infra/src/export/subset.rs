//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/font_subset.md
//! @prompt-hash 2de3de7a
//! @layer L3
//! @updated 2026-07-23
//!
//! **P516** — Subsetting TrueType/OpenType/CFF de fontes para embed no PDF.
//! **P874** — Substituição de `oxifont-subset` por `subsetter` (mesma biblioteca
//! do Typst 0.15.0) para produzir CFF CID-keyed válido sob `/CIDFontType0` +
//! `Identity-H`.

use std::collections::{BTreeMap, BTreeSet, HashMap};

/// Resultado de um subset de fonte.
///
/// Contém os bytes da fonte subsetada e o mapa de remapeamento
/// `old_glyph_id → new_glyph_id` necessário para ajustar o operador
/// `TJ` e o ToUnicode CMap no PDF.
pub struct FontSubset {
    /// Bytes SFNT da fonte subsetada.
    pub data: Vec<u8>,
    /// Mapa old → new glyph ID. Inclui sempre a entrada `0 → 0`.
    pub mapping: HashMap<u16, u16>,
}

/// Cria um subset de fonte a partir dos glifos usados.
///
/// `char_to_old_gid` mapeia cada codepoint usado no documento para o
/// glyph ID original na fonte. `additional_gids` contém glyph IDs
/// adicionais que devem ser preservados no subset mas não têm um
/// codepoint único (ex.: glifos de ligature produzidos pelo shaper).
///
/// P874 — Usa `subsetter`, que converte SID-keyed fonts para CID-keyed
/// e garante um mapeamento identidade GID→CID, produzindo subsets válidos
/// tanto para TrueType (`glyf`) como para CFF/OpenType (`CFF `).
/// O subsetter remove a tabela `cmap` de propósito; o mapping é obtido
/// directamente do `GlyphRemapper`.
pub fn subset_font_with_mapping(
    font_data: &[u8],
    char_to_old_gid: &BTreeMap<char, u16>,
    additional_gids: &BTreeSet<u16>,
) -> Option<FontSubset> {
    let mut remapper = subsetter::GlyphRemapper::new();

    // Incluir .notdef (GID 0) e todos os glifos usados no documento.
    // O GlyphRemapper já inclui .notdef por omissão, mas chamamos
    // remap(0) para garantir que o mapping fica explícito.
    remapper.remap(0);
    for &old_gid in char_to_old_gid.values() {
        remapper.remap(old_gid);
    }
    for &old_gid in additional_gids {
        remapper.remap(old_gid);
    }

    let subset_data = subsetter::subset(font_data, 0, &remapper).ok()?;

    // Construir o mapa old → new a partir do remapper.
    let mut mapping = HashMap::new();
    mapping.insert(0, 0);
    for (&_ch, &old_gid) in char_to_old_gid {
        if let Some(new_gid) = remapper.get(old_gid) {
            mapping.insert(old_gid, new_gid);
        }
    }
    for &old_gid in additional_gids {
        if let Some(new_gid) = remapper.get(old_gid) {
            mapping.insert(old_gid, new_gid);
        }
    }

    Some(FontSubset { data: subset_data, mapping })
}

/// Wrapper compatível com a assinatura do Prompt L0 original.
///
/// Mantido para consumidores que apenas precisam dos bytes. O mapa de
/// remapeamento pode ser obtido via [`subset_font_with_mapping`].
pub fn subset_font(font_data: &[u8], used_glyphs: &BTreeSet<u16>) -> Option<Vec<u8>> {
    let char_to_old_gid: BTreeMap<char, u16> = used_glyphs
        .iter()
        .filter(|&&gid| gid != 0)
        .map(|&gid| (char::from_u32(gid as u32).unwrap_or('\u{FFFD}'), gid))
        .collect();
    subset_font_with_mapping(font_data, &char_to_old_gid, &BTreeSet::new())
        .map(|s| s.data)
}

/// Aplica o mapa de remapeamento a um glyph ID original.
pub fn remap_glyph_id(old_id: u16, mapping: &HashMap<u16, u16>) -> u16 {
    mapping.get(&old_id).copied().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_test_font() -> Option<Vec<u8>> {
        let paths = [
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            "/usr/share/fonts/opentype/urw-base35/C059-Roman.otf",
        ];
        for path in &paths {
            if let Ok(bytes) = std::fs::read(path) {
                if ttf_parser::Face::parse(&bytes, 0).is_ok() {
                    return Some(bytes);
                }
            }
        }
        None
    }

    #[test]
    fn subset_font_valid_true_type() {
        let Some(data) = load_test_font() else { return };
        let mut used = BTreeSet::new();
        used.insert(65);
        used.insert(66);
        let subset = subset_font(&data, &used).expect("subset deve funcionar");
        let face =
            ttf_parser::Face::parse(&subset, 0).expect("subset deve ser parseável");
        assert!(face.number_of_glyphs() >= 3); // notdef + A + B
    }

    #[test]
    fn subset_font_empty_keeps_notdef() {
        let Some(data) = load_test_font() else { return };
        let used = BTreeSet::new();
        let subset = subset_font(&data, &used).expect("subset vazio deve funcionar");
        let face =
            ttf_parser::Face::parse(&subset, 0).expect("subset deve ser parseável");
        assert_eq!(face.number_of_glyphs(), 1);
    }

    #[test]
    fn subset_mapping_contains_notdef() {
        let Some(data) = load_test_font() else { return };
        let mut map = BTreeMap::new();
        map.insert('A', 65u16);
        let subset = subset_font_with_mapping(&data, &map, &BTreeSet::new())
            .expect("subset deve funcionar");
        assert_eq!(subset.mapping.get(&0), Some(&0));
        assert!(subset.mapping.contains_key(&65));
    }

    #[test]
    fn p520_subset_mapping_includes_additional_gids() {
        let Some(data) = load_test_font() else { return };
        let mut map = BTreeMap::new();
        map.insert('A', 65u16);
        let mut additional = BTreeSet::new();
        additional.insert(66);
        let subset = subset_font_with_mapping(&data, &map, &additional)
            .expect("subset deve funcionar");
        let face = ttf_parser::Face::parse(&subset.data, 0).expect("subset parseável");
        assert!(face.number_of_glyphs() >= 3, "deve incluir notdef + A + B adicional");
    }

    #[test]
    fn p520_additional_gid_gets_new_gid_mapping() {
        let Some(data) = load_test_font() else { return };
        let mut map = BTreeMap::new();
        map.insert('A', 65u16);
        let mut additional = BTreeSet::new();
        additional.insert(66);
        let subset = subset_font_with_mapping(&data, &map, &additional)
            .expect("subset deve funcionar");
        // O glifo adicional deve ter uma entrada old → new no mapping.
        let new_gid = subset
            .mapping
            .get(&66)
            .copied()
            .expect("B adicional deve ter new_gid");
        assert_ne!(new_gid, 0, "new_gid de B não deve ser .notdef");
    }

    #[test]
    fn remap_glyph_id_missing_returns_notdef() {
        let mapping = HashMap::new();
        assert_eq!(remap_glyph_id(42, &mapping), 0);
    }

    #[test]
    fn remap_glyph_id_known_returns_new() {
        let mut mapping = HashMap::new();
        mapping.insert(65, 1);
        assert_eq!(remap_glyph_id(65, &mapping), 1);
    }

    #[test]
    fn p523_subset_cff_nimbus_sans_produces_valid_cid_subset() {
        let fixture_path =
            concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/fonts/NimbusSans-Regular.otf");
        let font_data = match std::fs::read(fixture_path) {
            Ok(d) => d,
            Err(e) => {
                eprintln!(
                    "SKIP subset_cff_nimbus_sans: fixture não encontrada em {}: {}",
                    fixture_path, e
                );
                return;
            }
        };

        // P524 — glyph IDs derivados do texto real de
        // lab/parity/corpus/p523/test-cff-nimbus.typ.
        let text = "Hello world. The five boxing wizards jump quickly. ffi fl fi AV";
        let face = ttf_parser::Face::parse(&font_data, 0)
            .expect("fonte fixture CFF deve ser parseável");
        let mut map = BTreeMap::new();
        for ch in text.chars() {
            if let Some(gid) = face.glyph_index(ch) {
                map.insert(ch, gid.0);
            }
        }

        let subset = subset_font_with_mapping(&font_data, &map, &BTreeSet::new())
            .expect("P874: subsetting CFF deve funcionar com subsetter");

        let subset_face = ttf_parser::Face::parse(&subset.data, 0)
            .expect("subset CFF deve ser parseável");
        assert!(
            subset_face.tables().cff.is_some(),
            "subset deve preservar tabela CFF"
        );
        assert!(
            subset_face.number_of_glyphs() >= 2,
            "subset CFF deve conter pelo menos notdef + um glifo"
        );

        // Verificar que o mapping contém todos os glifos solicitados.
        for (&ch, &old_gid) in &map {
            assert!(
                subset.mapping.contains_key(&old_gid),
                "glyph {} ({}) deve estar no mapping",
                old_gid,
                ch
            );
        }
    }
}
