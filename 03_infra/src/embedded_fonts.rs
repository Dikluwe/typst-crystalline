//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/embedded_fonts.md
//! @prompt-hash b8a5dea9
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

use typst_core::entities::font_book::FontBook;

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
    pub text_slots: Vec<FontSlot>,
    pub text_book: FontBook,
    pub math_code_slots: Vec<FontSlot>,
    pub math_code_book: FontBook,
}

/// Classifica uma família de fonte embutida no grupo texto ou math/code.
///
/// **P783** — tentativa de correcção: assumiu o nome de família OpenType de
/// `NewCMMath-*.otf` como `"New Computer Modern Math"` (com espaços) — **essa
/// medição estava errada**, nunca verificada por leitura directa da tabela
/// `name` do ficheiro (só inferida do nome "bonito" habitual da fonte).
///
/// **P784** — remedição por leitura directa (`fontTools`/`ttf_parser`,
/// nameID 1 `FAMILY`): o valor real, para **todas** as fontes "New Computer
/// Modern *" embutidas via `typst-assets` (`NewCMMath-*.otf`,
/// `NewCM10-*.otf`), é **sem espaços** — `"NewComputerModernMath"` e
/// `"NewComputerModern10"`. `"Libertinus Serif"` (e `"Libertinus Serif
/// Regular"`) mantém espaços — a inconsistência é dos próprios ficheiros de
/// fonte upstream, não de extracção do cristalino. Consequência prática do
/// erro de P783: `resolve_candidates` nunca encontrava nenhuma família "New
/// Computer Modern *" pelo nome (0 candidatos sempre) — tanto aqui como em
/// `fallback_fonts.rs::DEFAULT_FALLBACK_FONTS_MATH` (mesma correcção lá).
/// `NewCM10` ficava classificado (por acidente, via o `else` "math_code")
/// como math/code em vez de texto — bug lateral do mesmo erro de nome,
/// corrigido aqui também (mesma causa-raiz, mesmo ficheiro).
///
/// **P840** — com a tabela de exceções de `fonts.rs::find_exception`
/// (port do vanilla `exceptions.rs`, achados #29/#30 de P831), as famílias
/// registadas no `FontBook` passam a ser os **nomes documentados** (com
/// espaços): `"New Computer Modern"` (NewCM10-*) e `"New Computer Modern
/// Math"` (NewCMMath-*) — os mesmos que o vanilla regista. A classificação
/// compara agora contra esses nomes; os ID1 crus (sem espaços) de P784 já
/// não chegam ao FontBook para estas fontes.
fn embedded_font_group(family: &str) -> &'static str {
    let lower = family.to_lowercase();
    if lower.starts_with("libertinus serif") || lower == "new computer modern" {
        "text"
    } else if lower == "new computer modern math" || lower.starts_with("dejavu sans mono")
    {
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

    EmbeddedFontSets {
        text_slots,
        text_book,
        math_code_slots,
        math_code_book,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p753_embedded_fonts_contain_libertinus_serif() {
        let sets = load_embedded_fonts();
        assert!(
            sets.text_book
                .infos()
                .iter()
                .any(|info| { info.family.eq_ignore_ascii_case("Libertinus Serif") }),
            "fontes de texto embutidas devem incluir Libertinus Serif"
        );
    }

    #[test]
    fn p797_probe_list_all_embedded_families() {
        // Sonda P797: lista todas as famílias embutidas com peso e estilo para
        // confirmar se Bold/Italic do Libertinus Serif estão disponíveis.
        let sets = load_embedded_fonts();
        for info in sets
            .text_book
            .infos()
            .iter()
            .chain(sets.math_code_book.infos().iter())
        {
            eprintln!("FONT: family={:?} variant={:?}", info.family, info.variant);
        }
    }

    #[test]
    fn p797_probe_select_pattern_by_variant() {
        // Sonda P797: verifica que select_pattern retorna índices DISTINTOS
        // para Regular, Bold e Italic do "Libertinus Serif".
        use typst_core::entities::font_book::{
            FontStretch, FontStyle, FontVariant, FontWeight,
        };
        use typst_core::entities::font_list::FontNamePattern;
        let sets = load_embedded_fonts();
        // Usa apenas o text_book (contém Regular, Bold, Italic, BoldItalic de Libertinus)
        let book = &sets.text_book;
        let pattern = FontNamePattern::Literal("Libertinus Serif".into());
        let regular = FontVariant {
            style: FontStyle::Normal,
            weight: FontWeight::REGULAR,
            stretch: FontStretch::NORMAL,
        };
        let bold = FontVariant {
            style: FontStyle::Normal,
            weight: FontWeight::BOLD,
            stretch: FontStretch::NORMAL,
        };
        let italic = FontVariant {
            style: FontStyle::Italic,
            weight: FontWeight::REGULAR,
            stretch: FontStretch::NORMAL,
        };
        let idx_reg = book.select_pattern(&pattern, &regular);
        let idx_bold = book.select_pattern(&pattern, &bold);
        let idx_ital = book.select_pattern(&pattern, &italic);
        eprintln!(
            "P797 select_pattern: regular={:?} bold={:?} italic={:?}",
            idx_reg, idx_bold, idx_ital
        );
        assert!(idx_reg.is_some(), "Regular deve resolver");
        assert!(idx_bold.is_some(), "Bold deve resolver");
        assert!(idx_ital.is_some(), "Italic deve resolver");
        assert_ne!(idx_reg, idx_bold, "Regular e Bold devem ser faces diferentes (P797)");
        assert_ne!(
            idx_reg, idx_ital,
            "Regular e Italic devem ser faces diferentes (P797)"
        );
        assert_ne!(idx_bold, idx_ital, "Bold e Italic devem ser faces diferentes (P797)");
    }

    fn p754_newcm_math_is_not_in_text_group() {
        let sets = load_embedded_fonts();
        // P784 — o ID1 cru (sem espaços) é "NewComputerModernMath".
        // P840 — com a tabela de exceções, a família registada passa a ser
        // o nome documentado "New Computer Modern Math" (com espaços, como
        // no vanilla); as comparações usam agora esse nome.
        let newcm_math_in_text = sets
            .text_book
            .infos()
            .iter()
            .any(|info| info.family.eq_ignore_ascii_case("new computer modern math"));
        assert!(
            !newcm_math_in_text,
            "New Computer Modern Math não deve estar no grupo texto (P754)"
        );
        let newcm_math_in_math_code = sets
            .math_code_book
            .infos()
            .iter()
            .any(|info| info.family.eq_ignore_ascii_case("new computer modern math"));
        assert!(
            newcm_math_in_math_code,
            "New Computer Modern Math deve estar de facto no grupo math_code (P784 — \
             confirma que a classificação não é vacuamente verdadeira)"
        );
    }

    #[test]
    fn p784_newcm10_is_in_text_group() {
        // Débito lateral descoberto pela mesma investigação: `NewCM10`
        // estava a cair no `else` "math_code" por acidente — devia estar
        // em "text" (docstring de `EmbeddedFontSets`: "texto (Libertinus
        // Serif*, NewCM10*)"). P840 — a família registada é agora o nome
        // documentado "New Computer Modern" (tabela de exceções).
        let sets = load_embedded_fonts();
        let newcm10_in_text = sets
            .text_book
            .infos()
            .iter()
            .any(|info| info.family.eq_ignore_ascii_case("new computer modern"));
        assert!(newcm10_in_text, "New Computer Modern deve estar no grupo texto (P784)");
    }

    #[test]
    fn p754_dejavu_sans_mono_is_not_in_text_group() {
        let sets = load_embedded_fonts();
        let dejavu_mono_in_text = sets
            .text_book
            .infos()
            .iter()
            .any(|info| info.family.to_lowercase().starts_with("dejavu sans mono"));
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
