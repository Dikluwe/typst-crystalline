//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/layout.md
//! @prompt-hash 4f7a4e44
//! @layer L3
//! @updated 2026-03-28

use typst_core::{
    entities::{content::Content, layout_types::PagedDocument},
    rules::layout::Layouter,
};

use crate::font_metrics::FontBookMetrics;

/// Layout com métricas de fonte reais.
///
/// Usa `FontBookMetrics` se os bytes de fonte forem válidos.
/// Fallback para `FixedMetrics` (L1) se inválidos — não panic.
pub fn layout_with_font(
    content:   &Content,
    font_data: &[u8],
    font_size: f64,
) -> PagedDocument {
    if let Some(metrics) = FontBookMetrics::from_bytes(font_data) {
        let mut l = Layouter::new(metrics, font_size);
        l.layout_content(content);
        l.finish()
    } else {
        typst_core::rules::layout::layout(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::entities::content::Content;

    #[test]
    fn bytes_invalidos_nao_panic() {
        let doc = layout_with_font(&Content::text("Hello"), b"invalid", 12.0);
        // Fallback para FixedMetrics — deve retornar documento válido
        assert!(!doc.pages.is_empty());
        assert!(doc.plain_text().contains("Hello"));
    }

    #[test]
    fn bytes_vazios_nao_panic() {
        let doc = layout_with_font(&Content::text("Test"), b"", 12.0);
        assert!(!doc.pages.is_empty());
    }

    #[test]
    #[ignore = "requer tests/fixtures/liberation-sans-regular.ttf"]
    fn bytes_validos_produzem_documento() {
        let data = std::fs::read(
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/liberation-sans-regular.ttf")
        ).unwrap();
        let doc = layout_with_font(&Content::text("Hello world"), &data, 12.0);
        assert!(!doc.pages.is_empty());
        assert!(doc.plain_text().contains("Hello") || doc.plain_text().contains("world"));
    }
}
