//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/break_selection.md
//! @prompt-hash ad0302fd
//! @layer L1
//! @updated 2026-09-28

use super::{hyphenation, metrics::FontMetrics, text_measure};
use crate::entities::layout_types::{Pt, TextStyle};

#[derive(Debug, PartialEq)]
pub(super) enum Decision {
    Keep,
    Wrap,
    Hyphenate { prefix: String, replacement: String, width: Pt, rest: String },
}

/// Compare coordinates in one frame; an overlong first word must make progress.
pub(super) fn should_wrap(
    width: Pt,
    cursor_x: f64,
    right_edge: f64,
    threshold_x: f64,
) -> bool {
    cursor_x + width.0 > right_edge && cursor_x > threshold_x
}

#[expect(
    clippy::too_many_arguments,
    reason = "C016: Seletor puro isolado do Layouter por decisão explícita do L0: recebe palavra/largura medida, métricas/estilo, cursor/borda/origem reais e política de hifenização"
)]
pub(super) fn word<M: FontMetrics + ?Sized>(
    metrics: &M,
    style: &TextStyle,
    text: &str,
    width: Pt,
    cursor_x: f64,
    right_edge: f64,
    threshold_x: f64,
    hyphenate: bool,
) -> Decision {
    if !should_wrap(width, cursor_x, right_edge, threshold_x) {
        return Decision::Keep;
    }
    if !hyphenate {
        return Decision::Wrap;
    }
    if let Some(lang) = style.lang {
        let break_points = hyphenation::hyphenate(text, &lang);
        if !break_points.is_empty() {
            let available = right_edge - cursor_x;
            for &point in break_points.iter().rev() {
                let prefix: String = text.chars().take(point).collect();
                let prefix_with_hyphen = format!("{}-", prefix);
                let measured = text_measure::word(metrics, &prefix_with_hyphen, style);
                if measured.0 <= available {
                    return Decision::Hyphenate {
                        prefix: prefix_with_hyphen,
                        replacement: format!("{prefix}\u{ad}"),
                        width: measured,
                        rest: text.chars().skip(point).collect(),
                    };
                }
            }
        }
    }
    Decision::Wrap
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::layout::FixedMetrics;
    #[test]
    fn selection_preserves_exact_fit_first_word_and_unconstrained_progress() {
        assert!(!should_wrap(Pt(10.0), 5.0, 15.0, 0.0));
        assert!(should_wrap(Pt(10.0), 5.0, 14.0, 0.0));
        assert!(!should_wrap(Pt(100.0), 0.0, 14.0, 0.0));
        assert!(!should_wrap(Pt(100.0), 5.0, f64::INFINITY, 0.0));
        let style = TextStyle::regular(Pt(12.0));
        assert_eq!(
            word(&FixedMetrics, &style, "abc", Pt(10.0), 5.0, 14.0, 0.0, true),
            Decision::Wrap
        );
    }
    #[test]
    fn selected_hyphen_preserves_unicode_content_and_fits_remaining_region() {
        let mut style = TextStyle::regular(Pt(12.0));
        style.lang = Some("pt".parse().unwrap());
        let text = "representação";
        let decision =
            word(&FixedMetrics, &style, text, Pt(100.0), 10.0, 70.0, 0.0, true);
        let Decision::Hyphenate { prefix, width, rest, .. } = decision else {
            panic!("expected a language-defined break")
        };
        assert!(width.0 <= 60.0);
        assert_eq!(format!("{}{}", prefix.strip_suffix('-').unwrap(), rest), text);
        assert!(!rest.is_empty());
    }
}
