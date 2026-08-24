//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper.md
//! @prompt-hash 5319528a
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
///
/// **P840** — com a tabela de exceções (`fonts.rs::find_exception`, port
/// do vanilla `exceptions.rs`), a família registada no FontBook para
/// `NewCMMath-*.otf` passa a ser `"New Computer Modern Math"` (com
/// espaços) — o nome documentado que o vanilla regista e que a sua cadeia
/// `math::families()` usa (`"new computer modern math"`,
/// `typst-library/src/math/mod.rs:179`). O nome volta por isso à forma
/// com espaços: é essa que resolve no book, como no vanilla.
pub(crate) const DEFAULT_FALLBACK_FONTS_MATH: &[&str] = &[
    "New Computer Modern Math",
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
        // P840 — com a tabela de exceções, a família registada no book é
        // "New Computer Modern Math" (com espaços), o nome que a cadeia
        // `math::families()` do vanilla usa (`math/mod.rs:179`).
        assert_eq!(
            list[0], "New Computer Modern Math",
            "primeiro fallback math deve ser New Computer Modern Math (nome documentado, paridade vanilla)"
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
