//! # CABEÇALHO DE TIPOLOGIA
//! Nível: L1 (Core / Lógica Pura)
//! Módulo: Fonts
//! Responsabilidade: Formatação pura das variantes de fontes.

use typst::text::{FontStretch, FontStyle, FontWeight};

/// Formata a string de detalhes de uma variante de fonte.
pub fn format_font_variant(
    style: FontStyle,
    weight: FontWeight,
    stretch: FontStretch,
    path: &str,
) -> String {
    format!("- Style: {style:?}, Weight: {weight:?}, Stretch: {stretch:?}, Path: {path}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_font_variant_normal() {
        let result = format_font_variant(
            FontStyle::Normal,
            FontWeight::REGULAR,
            FontStretch::NORMAL,
            "/usr/share/fonts/Arial.ttf",
        );
        assert_eq!(
            result,
            "- Style: Normal, Weight: 400, Stretch: FontStretch(1000), Path: /usr/share/fonts/Arial.ttf"
        );
    }

    #[test]
    fn test_format_font_variant_italic_bold() {
        let result = format_font_variant(
            FontStyle::Italic,
            FontWeight::BOLD,
            FontStretch::CONDENSED,
            "embedded",
        );
        assert_eq!(
            result,
            "- Style: Italic, Weight: 700, Stretch: FontStretch(750), Path: embedded"
        );
    }
}
