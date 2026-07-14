//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/embedded_fonts.md
//! @prompt-hash 8c8c9639
//! @layer L3
//! @updated 2026-07-14
//!
//! **P753/P754** — fontes embutidas do vanilla via `typst-assets`.
//! Garante que `Libertinus Serif` e as fontes de math/code do vanilla
//! estejam sempre disponíveis, independentemente das fontes do sistema.
//!
//! **P754** — as fontes embutidas são divididas em dois grupos:
//!   - texto (`Libertinus Serif*`, `NewCM10*`) → início do FontBook;
//!   - math/code (`NewCMMath*`, `DejaVu Sans Mono*`) → fim do FontBook,
//!     depois das fontes do sistema, para não competirem no fallback de
//!     texto normal para scripts não latinos (ex.: devanágari).

use std::path::PathBuf;

use typst_core::entities::font_book::{FontBook, FontInfo};

use crate::fonts::{font_info_from_bytes, FontSlot};

/// Conjuntos de fontes embutidas separados por função (P754).
///
/// `text` contém as fontes de texto propriamente ditas; `math_code` contém as
/// fontes matemáticas e monoespaçadas. A separação permite que o `SystemWorld`
/// coloque as fontes de texto antes das fontes do sistema e as fontes de
/// math/code depois, evitando que `NewCMMath-Regular` (que possui glifos
/// parciais para devanágari) seja escolhida como fallback para texto
/// devanágari.
pub struct EmbeddedFontSets {
    pub text_slots:      Vec<FontSlot>,
    pub text_book:       FontBook,
    pub math_code_slots: Vec<FontSlot>,
    pub math_code_book:  FontBook,
}

/// Classifica uma família de fonte embutida no grupo texto ou math/code.
fn embedded_font_group(family: &str) -> &'static str {
    let lower = family.to_lowercase();
    if lower.starts_with("libertinus serif") || lower.starts_with("newcm10") {
        "text"
    } else if lower.starts_with("newcmmath") || lower.starts_with("dejavu sans mono") {
        "math_code"
    } else {
        // Desconhecido: tratar como math/code por segurança (não competir no
        // fallback de texto).
        "math_code"
    }
}

/// Carrega as fontes embutidas distribuídas com o vanilla Typst.
///
/// Os bytes vêm de `typst_assets::fonts()` e são compilados no binário; não há
/// I/O de disco. Retorna os conjuntos separados para que `SystemWorld` possa
/// compor a ordem correcta no `FontBook`.
pub fn load_embedded_fonts() -> EmbeddedFontSets {
    let mut text_slots = Vec::new();
    let mut text_book = FontBook::new();
    let mut math_code_slots = Vec::new();
    let mut math_code_book = FontBook::new();

    for data in typst_assets::fonts() {
        let data: Vec<u8> = data.to_vec();
        let path = PathBuf::from("<embedded>");
        let slot = FontSlot::new_embedded(path, data.clone());

        if let Some(info) = font_info_from_bytes(&data, 0) {
            match embedded_font_group(&info.family) {
                "text" => {
                    text_book.push(info);
                    text_slots.push(slot);
                }
                _ => {
                    math_code_book.push(info);
                    math_code_slots.push(slot);
                }
            }
        }
    }

    EmbeddedFontSets { text_slots, text_book, math_code_slots, math_code_book }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p753_embedded_fonts_contain_libertinus_serif() {
        let sets = load_embedded_fonts();
        assert!(
            sets.text_book.infos().iter().any(|info| {
                info.family.eq_ignore_ascii_case("Libertinus Serif")
            }),
            "fontes de texto embutidas devem incluir Libertinus Serif"
        );
    }

    #[test]
    fn p754_newcm_math_is_not_in_text_group() {
        let sets = load_embedded_fonts();
        let newcm_math_in_text = sets.text_book.infos().iter().any(|info| {
            info.family.to_lowercase().starts_with("newcmmath")
        });
        assert!(
            !newcm_math_in_text,
            "NewCMMath não deve estar no grupo texto (P754)"
        );
    }

    #[test]
    fn p754_dejavu_sans_mono_is_not_in_text_group() {
        let sets = load_embedded_fonts();
        let dejavu_mono_in_text = sets.text_book.infos().iter().any(|info| {
            info.family.to_lowercase().starts_with("dejavu sans mono")
        });
        assert!(
            !dejavu_mono_in_text,
            "DejaVu Sans Mono não deve estar no grupo texto (P754)"
        );
    }

    #[test]
    fn p753_embedded_slot_loads_without_disk_io() {
        let sets = load_embedded_fonts();
        assert!(
            sets.text_slots.iter().all(|s| s.get().is_some()),
            "todos os slots de texto embutidos devem carregar com sucesso"
        );
        assert!(
            sets.math_code_slots.iter().all(|s| s.get().is_some()),
            "todos os slots de math/code embutidos devem carregar com sucesso"
        );
    }
}
