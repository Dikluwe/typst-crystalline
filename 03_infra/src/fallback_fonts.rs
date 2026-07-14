//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/shaper.md
//! @prompt-hash 3fdedaa0
//! @prompt 00_nucleo/prompts/infra/font_metrics.md
//! @prompt-hash 3fdedaa0
//! @layer L3
//! @updated 2026-07-14
//!
//! **P555** — listas de fallback de fonte divididas por classe visual
//! (serifa vs sem serifa). Quando a fonte primária declarada não resolve,
//! o shaper e as métricas de fallback usam a lista adequada para preservar
//! a classe da fonte pedida.

/// Fontes serif de fallback, preferidas quando a primária é uma serif.
///
/// **P558** — `Liberation Serif` e `DejaVu Serif` são preferidas a `FreeSerif`
/// porque `FreeSerif` descompõe acentos em base + mark e o subsetter CFF do
/// cristalino não reconstrói esses glifos correctamente. `FreeSerif` mantém-se
/// como último recurso para cobertura de símbolos e outros scripts.
pub(crate) const DEFAULT_FALLBACK_FONTS_SERIF: &[&str] = &[
    "Liberation Serif",
    "DejaVu Serif",
    "Bitstream Vera Serif",
    "FreeSerif",
];

/// Fontes sans-serif de fallback, usadas quando a primária é sans-serif
/// ou quando não é possível inferir a classe.
pub(crate) const DEFAULT_FALLBACK_FONTS_SANS: &[&str] = &[
    "DejaVu Sans",
    "Noto Sans",
    "Liberation Sans",
    "FreeSans",
    "Arial",
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
}
