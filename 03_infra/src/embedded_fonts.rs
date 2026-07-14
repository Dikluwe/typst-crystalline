//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/embedded_fonts.md
//! @prompt-hash 78852948
//! @layer L3
//! @updated 2026-07-14
//!
//! **P753** — fontes embutidas do vanilla via `typst-assets`.
//! Garante que `Libertinus Serif` e as fontes de math/code do vanilla
//! estejam sempre disponíveis, independentemente das fontes do sistema.

use std::path::PathBuf;

use typst_core::entities::font_book::FontBook;

use crate::fonts::{font_info_from_bytes, FontSlot};

/// Carrega as fontes embutidas distribuídas com o vanilla Typst.
///
/// Retorna os `FontSlot` descobertos e o `FontBook` populado com os
/// metadados das faces. Os bytes vêm de `typst_assets::fonts()` e são
/// compilados no binário; não há I/O de disco.
pub fn load_embedded_fonts() -> (Vec<FontSlot>, FontBook) {
    let mut slots = Vec::new();
    let mut book = FontBook::new();

    for data in typst_assets::fonts() {
        let data: Vec<u8> = data.to_vec();
        let path = PathBuf::from("<embedded>");
        let slot = FontSlot::new_embedded(path, data.clone());

        if let Some(info) = font_info_from_bytes(&data, 0) {
            book.push(info);
            slots.push(slot);
        }
    }

    (slots, book)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p753_embedded_fonts_contain_libertinus_serif() {
        let (slots, book) = load_embedded_fonts();
        assert!(!slots.is_empty(), "deve haver pelo menos uma fonte embutida");

        let found = book.infos().iter().any(|info| {
            info.family.eq_ignore_ascii_case("Libertinus Serif")
        });
        assert!(found, "fontes embutidas devem incluir Libertinus Serif");
    }

    #[test]
    fn p753_embedded_slot_loads_without_disk_io() {
        let (slots, _) = load_embedded_fonts();
        assert!(
            slots.iter().all(|s| s.get().is_some()),
            "todos os slots embutidos devem carregar com sucesso"
        );
    }
}
