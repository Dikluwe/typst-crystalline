//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper.md
//! @prompt-hash 3fdedaa0
//! @prompt 00_nucleo/prompts/infra/font_metrics.md
//! @prompt-hash 167c29d3
//! @layer L3
//! @updated 2026-07-18
//!
//! **P555** — listas de fallback de fonte divididas por classe visual
//! (serifa vs sem serifa). Quando a fonte primária declarada não resolve,
//! o shaper e as métricas de fallback usam a lista adequada para preservar
//! a classe da fonte pedida.
//!
//! **P783** — lista de fallback específica de modo matemático, espelhando
//! a cadeia do vanilla (`math::families()`): New Computer Modern Math →
//! Libertinus Serif → fontes de emoji. Usada quando a fonte primária
//! tem tabela MATH OpenType e não cobre um glifo.

/// Fontes serif de fallback, preferidas quando a primária é uma serif.
///
/// **P558** — `Liberation Serif` e `DejaVu Serif` são preferidas a `FreeSerif`
/// porque `FreeSerif` descompõe acentos em base + mark e o subsetter CFF do
/// cristalino não reconstrói esses glifos correctamente. `FreeSerif` mantém-se
/// como último recurso para cobertura de símbolos e outros scripts.
pub(crate) const DEFAULT_FALLBACK_FONTS_SERIF: &[&str] =
    &["Liberation Serif", "DejaVu Serif", "Bitstream Vera Serif", "FreeSerif"];

/// Fontes sans-serif de fallback, usadas quando a primária é sans-serif
/// ou quando não é possível inferir a classe.
pub(crate) const DEFAULT_FALLBACK_FONTS_SANS: &[&str] =
    &["DejaVu Sans", "Noto Sans", "Liberation Sans", "FreeSans", "Arial"];

/// Fontes de fallback específicas de modo matemático.
///
/// **P783** — paridade com a cadeia do vanilla (`math::families()` em
/// `typst-library/src/math/mod.rs`): quando a fonte matemática primária
/// não cobre um glifo, tentam-se estas fontes antes das genéricas.
/// A ordem é idêntica à do vanilla: New Computer Modern Math →
/// Libertinus Serif → fontes de emoji de cobertura ampla.
///
/// **P784** — o primeiro nome estava errado: `"New Computer Modern Math"`
/// (com espaços) nunca resolvia nenhum candidato — confirmado por
/// verificação visual real (glifo `⨿`/U+2A3F rendia com o glifo errado,
/// `uni27F8`, embutido de uma fonte de sistema aleatória, `MathJax_Main-
/// Regular`, apanhada pelo fallback global em vez da cadeia math). Causa:
/// o nome de família real do ficheiro embutido (`NewCMMath-*.otf`, tabela
/// `name` nameID 1, confirmado por `fontTools`/`ttf_parser`) é
/// `"NewComputerModernMath"`, **sem espaços** — inconsistência do próprio
/// ficheiro de fonte upstream (`Libertinus Serif` tem espaços; `New
/// Computer Modern *` não tem). Ver `embedded_fonts.rs::
/// embedded_font_group` para a mesma correcção do lado da classificação.
pub(crate) const DEFAULT_FALLBACK_FONTS_MATH: &[&str] = &[
    "NewComputerModernMath",
    "Libertinus Serif",
    "Twitter Color Emoji",
    "Noto Color Emoji",
    "Apple Color Emoji",
    "Segoe UI Emoji",
];

/// Heurística simples de classe a partir do nome da família.
/// - Nome contendo "serif" (case-insensitive) → serif.
/// - Nome contendo "sans" (case-insensitive) → sans.
/// - Qualquer outro caso → sans (preserva comportamento anterior).
pub(crate) fn fallback_font_list_for(name: &str) -> &'static [&'static str] {
    let lower = name.to_lowercase();
    if lower.contains("serif") {
        DEFAULT_FALLBACK_FONTS_SERIF
    } else {
        DEFAULT_FALLBACK_FONTS_SANS
    }
}

/// Lista de fallback para modo matemático.
///
/// **P783** — usar quando se sabe que a fonte primária é uma fonte matemática
/// (i.e., tem tabela MATH OpenType). Retorna `DEFAULT_FALLBACK_FONTS_MATH`
/// em vez da lista genérica serif/sans, para que glifos matemáticos ausentes
/// na primária sejam buscados em fontes com cobertura matemática adequada.
pub(crate) fn math_fallback_font_list() -> &'static [&'static str] {
    DEFAULT_FALLBACK_FONTS_MATH
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p555_fallback_font_list_serif_for_freeserif() {
        assert_eq!(fallback_font_list_for("FreeSerif"), DEFAULT_FALLBACK_FONTS_SERIF);
    }

    #[test]
    fn p555_fallback_font_list_serif_for_dejavu_serif() {
        assert_eq!(fallback_font_list_for("DejaVu Serif"), DEFAULT_FALLBACK_FONTS_SERIF);
    }

    #[test]
    fn p555_fallback_font_list_sans_for_dejavu_sans() {
        assert_eq!(fallback_font_list_for("DejaVu Sans"), DEFAULT_FALLBACK_FONTS_SANS);
    }

    #[test]
    fn p555_fallback_font_list_default_for_unknown() {
        assert_eq!(fallback_font_list_for("SomeCustomFont"), DEFAULT_FALLBACK_FONTS_SANS);
    }

    // ── P783 — lista de fallback matemático ─────────────────────────────────

    #[test]
    fn p783_math_fallback_list_starts_with_new_computer_modern() {
        let list = math_fallback_font_list();
        assert!(!list.is_empty(), "lista de fallback math não pode ser vazia");
        // P784 — nome real da fonte embutida é sem espaços
        // ("NewComputerModernMath", confirmado por leitura da tabela
        // `name`, nameID 1) — "New Computer Modern Math" (com espaços)
        // nunca resolvia nenhum candidato.
        assert_eq!(
            list[0], "NewComputerModernMath",
            "primeiro fallback math deve ser NewComputerModernMath (nome real, paridade vanilla)"
        );
    }

    #[test]
    fn p783_math_fallback_list_contains_libertinus_serif() {
        let list = math_fallback_font_list();
        assert!(
            list.contains(&"Libertinus Serif"),
            "lista math deve conter Libertinus Serif como fallback de segunda ordem"
        );
    }
}
