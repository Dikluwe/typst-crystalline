//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/gradients/adaptive.md
//! @prompt-hash 4456c00d
//! @layer L3
//! @updated 2026-05-19
//!
//! Adaptive N multispace — perceptual distance + adaptive_n_for_stops (P274).
//!
//! Extraído de `gradients.rs` em P307b.2 (ADR-0100 / diagnóstico
//! P307a §5). Conteúdo bit-exact pré e pós migração.

use typst_core::entities::layout_types::{Color, ColorSpace};

// ── P274 — Adaptive N multispace refino qualitativo ──
//
// Pré-amostragem N=16 fixo (P270.1) substituída por adaptive baseado
// em ΔE Oklab nativo entre stops adjacentes. Aplicável apenas a
// Linear+Radial RGB-family + perceptual (CMYK preserved P270.2;
// Conic preserved P272 Coons).
//
// Fórmula (ADR-0091 §"Anotação cumulativa P274" Opção 1B threshold):
// - N=16 se max_pair_delta_e < 0.05 (low contrast pastel; paridade P270.1).
// - N=32 se 0.05 ≤ max_pair_delta_e < 0.3 (moderate).
// - N=64 se max_pair_delta_e ≥ 0.3 (high contrast; cap N_max).
//
// Distinção vs P268.2 removido P272: helper genérico cross-space
// (`space` param futuro-proofing per ADR-0094 Pattern 2).

/// P274 — Distância perceptual entre duas cores num space dado.
///
/// Métrica: ΔE Oklab (independente do space de entrada; converte cada
/// cor para Oklab e calcula distância euclidiana per Björn Ottosson
/// 2020 + W3C CSS Color 4 §11).
///
/// **Parâmetro `_space` reservado** — não altera a métrica actual
/// (ADR-0094 Pattern 2: antecipa reutilização sem custo). Permite
/// refino futuro com métrica nativa per space.
pub(crate) fn perceptual_distance_in_space(
    c0: typst_core::entities::layout_types::Color,
    c1: typst_core::entities::layout_types::Color,
    _space: typst_core::entities::layout_types::ColorSpace,
) -> f32 {
    let (l0, a0, b0, _) = typst_core::entities::gradient::color_to_oklab_with_alpha(c0);
    let (l1, a1, b1, _) = typst_core::entities::gradient::color_to_oklab_with_alpha(c1);
    let dl = l1 - l0;
    let da = a1 - a0;
    let db = b1 - b0;
    (dl * dl + da * da + db * db).sqrt()
}

/// P274 — Computa N adaptive para pré-amostragem Linear/Radial.
///
/// Fórmula Opção 1B threshold-based (ADR-0091 §"Anotação cumulativa
/// P274" Decisão 1):
/// - N=16 se max_pair_delta_e < 0.05 (low contrast pastel; preserva
///   paridade P270.1 emit literal).
/// - N=32 se 0.05 ≤ max_pair_delta_e < 0.3 (moderate).
/// - N=64 se max_pair_delta_e ≥ 0.3 (high contrast; cap N_max=64).
pub(crate) fn adaptive_n_for_stops(
    stops: &[typst_core::entities::gradient::GradientStop],
    space: typst_core::entities::layout_types::ColorSpace,
) -> usize {
    if stops.len() < 2 {
        return 16;
    }
    let max_delta_e = stops
        .windows(2)
        .map(|pair| perceptual_distance_in_space(pair[0].color, pair[1].color, space))
        .fold(0.0_f32, f32::max);
    if max_delta_e < 0.05 {
        16
    } else if max_delta_e < 0.3 {
        32
    } else {
        64
    }
}
