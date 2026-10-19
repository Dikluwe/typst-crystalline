//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/text_measure.md
//! @prompt-hash ac631570
//! @layer L1
//! @updated 2026-09-29

use super::metrics::FontMetrics;
use crate::entities::layout_types::{Pt, TextStyle};

/// Default formula behind FontMetrics; callers may override that trait method.
pub(super) fn default_text_width<M: FontMetrics + ?Sized>(
    metrics: &M,
    text: &str,
    size: Pt,
    style: &TextStyle,
) -> Pt {
    let base = metrics
        .advance_shaped(text, size, style)
        .map(|p| p.val())
        .unwrap_or_else(|| metrics.advance(text, size, style).val());
    let tracking_extra = style
        .tracking
        .map(|t| {
            let tracking_pt = t.resolve_pt(size.val());
            let n = text.chars().count();
            tracking_pt * n.saturating_sub(1) as f64
        })
        .unwrap_or(0.0);
    Pt(base + tracking_extra)
}

pub(super) fn word<M: FontMetrics + ?Sized>(
    metrics: &M,
    text: &str,
    style: &TextStyle,
) -> Pt {
    metrics.text_width(text, style.size, style)
}

pub(super) fn space<M: FontMetrics + ?Sized>(metrics: &M, style: &TextStyle) -> Pt {
    metrics.advance(" ", style.size, style)
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Override;
    impl FontMetrics for Override {
        fn advance(&self, text: &str, _: Pt, _: &TextStyle) -> Pt {
            Pt(text.chars().count() as f64)
        }
        fn vertical_metrics(&self, _: Pt, _: &TextStyle) -> (Pt, Pt) {
            panic!("horizontal measurement must not request vertical metrics")
        }
        fn cap_height(&self, _: Pt, _: &TextStyle) -> Pt {
            panic!("horizontal measurement must not request cap height")
        }
        fn text_edges(&self, _: Pt, _: &TextStyle) -> (Pt, Pt) {
            panic!("horizontal measurement must not request text edges")
        }
        fn text_width(&self, _: &str, _: Pt, _: &TextStyle) -> Pt {
            Pt(77.0)
        }
    }
    #[test]
    fn injected_override_remains_authoritative_without_vertical_demand() {
        let style = TextStyle::regular(Pt(12.0));
        assert_eq!(word(&Override, "word", &style), Pt(77.0));
        assert_eq!(space(&Override, &style), Pt(1.0));
        let borrowed: &dyn FontMetrics = &Override;
        assert_eq!(word(borrowed, "word", &style), Pt(77.0));
    }
    #[test]
    fn default_formula_preserves_unicode_pair_tracking_and_fallback() {
        let mut style = TextStyle::regular(Pt(12.0));
        style.tracking = Some(crate::entities::layout_types::Length::pt(2.0));
        assert_eq!(default_text_width(&Override, "éx", style.size, &style), Pt(4.0));
        assert_eq!(default_text_width(&Override, "", style.size, &style), Pt(0.0));
    }
}
