//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/layout.md
//! @prompt-hash d09c8333
//! @layer L3
//! @updated 2026-03-28

use typst_core::entities::content::Content;
use typst_core::entities::layout_types::PagedDocument;

use crate::font_metrics::FontBookMetrics;
use crate::image_sizer::ImageSizeImageSizer;

/// Layout com métricas de fonte reais.
///
/// Usa `FontBookMetrics` se os bytes de fonte forem válidos.
/// Fallback para `FixedMetrics` (L1) se inválidos — não panic.
pub fn layout_with_font(
    content: &Content,
    font_data: &[u8],
    font_size: f64,
) -> PagedDocument {
    if let Some(metrics) = FontBookMetrics::from_bytes(font_data) {
        use typst_core::compiler::layout::FontMetrics;
        use typst_core::entities::introspector::TagIntrospector;

        // This legacy helper cannot realize `Func` callbacks because it has
        // no Engine/World. Still enter through the callback-aware boundary so
        // a pending request fails closed instead of leaking provisional pages
        // to its L4/exporter callers.
        let metrics_ref: &dyn FontMetrics = &metrics;
        typst_core::compiler::layout::layout_with_introspector_and_metrics(
            content,
            TagIntrospector::empty(),
            metrics_ref,
            ImageSizeImageSizer,
            font_size,
        )
        .expect("layout_with_font cannot resolve math callbacks")
    } else {
        // P190I (M6 fechado): layout() já não recebe state.
        typst_core::compiler::layout::layout(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use typst_core::entities::content::Content;
    use typst_core::entities::elements::math_cancel::{
        MathCancelAngle, MathCancelElem, MathCancelExplicit,
    };
    use typst_core::entities::func::Func;
    use typst_core::entities::layout_types::{Angle, Length};
    use typst_core::entities::rel::Rel;
    use typst_core::entities::span::Span;
    use typst_core::entities::value::Value;

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
    fn bytes_validos_produzem_documento() {
        let data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/NimbusSans-Regular.otf"
        ))
        .unwrap();
        let doc = layout_with_font(&Content::text("Hello world"), &data, 12.0);
        assert!(!doc.pages.is_empty());
        assert!(doc.plain_text().contains("Hello") || doc.plain_text().contains("world"));
    }

    #[test]
    fn p1291_layout_with_font_callback_nao_exporta_documento_provisorio() {
        let data = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/NewCMMath-Book.otf"
        ))
        .expect("fixture math valida deve existir");
        let angle = Func::native("p1291-angle-zero", |_ctx, _args, _world, _file| {
            Ok(Value::Angle(Angle::deg(0.0)))
        });
        let cancel = Content::MathCancel(Arc::new(MathCancelElem {
            body: Content::MathIdent("x".into()),
            length: Rel::<Length>::from_percent(100.0) + Length::em(0.3),
            inverted: false,
            cross: false,
            angle: MathCancelAngle::Func(angle),
            stroke: None,
            background: false,
            span: Span::detached(),
            explicit: MathCancelExplicit::default(),
        }));
        let content = Content::equation(cancel, false);

        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            layout_with_font(&content, &data, 12.0)
        }));

        if let Ok(document) = outcome {
            panic!(
                "layout_with_font devolveu PagedDocument provisório com {} página(s)",
                document.pages.len()
            );
        }
    }
}
