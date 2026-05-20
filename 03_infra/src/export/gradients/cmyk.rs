//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/gradients/cmyk.md
//! @prompt-hash 08b22cb0
//! @layer L3
//! @updated 2026-05-19
//!
//! CMYK helpers — rgb→cmyk conversão + multispace stops para Linear/Radial CMYK (P270.2).
//!
//! Extraído de `gradients.rs` em P307b.2 (ADR-0100 / diagnóstico
//! P307a §5). Conteúdo bit-exact pré e pós migração.

use typst_core::entities::gradient::{Linear, Radial};
use typst_core::entities::layout_types::Color;

// ── P270.2 — CMYK helpers L3 (Cenário B: Linear+Radial; Conic preserved) ──

/// P270.2 — sRGB → CMYK inverse conversion (fallback precaução).
///
/// Usado quando dispatcher P270 arm Cmyk retorna `Color` não-`Color::Cmyk`
/// (improvável dado que `interpolate_cmyk` retorna `Color::cmyk(...)`,
/// mas precaução defensiva).
pub(crate) fn rgb_to_cmyk(r: f32, g: f32, b: f32) -> (f32, f32, f32, f32) {
    let k = 1.0 - r.max(g).max(b);
    if k >= 1.0 - 1e-6 {
        (0.0, 0.0, 0.0, 1.0)
    } else {
        let denom = 1.0 - k;
        let c = (1.0 - r - k) / denom;
        let m = (1.0 - g - k) / denom;
        let y = (1.0 - b - k) / denom;
        (c.clamp(0.0, 1.0), m.clamp(0.0, 1.0),
         y.clamp(0.0, 1.0), k.clamp(0.0, 1.0))
    }
}

/// P270.2 — Helper amostragem CMYK 4-component para Linear gradient.
///
/// Output: `Vec<(c, m, y, k)>` em [0, 1] CMYK. Bug vanilla #4422
/// resolvido por construção (cristalino emit `/DeviceCMYK` correcto
/// via este pipeline).
///
/// Usado apenas no branch `linear.space == ColorSpace::Cmyk` em
/// `emit_gradient_objects`. Para outras spaces, `multispace_sample_stops`
/// (P270.1) preservado literal.
pub(crate) fn multispace_sample_stops_linear_cmyk(
    linear: &typst_core::entities::gradient::Linear,
    n_samples: usize,
) -> Vec<(f32, f32, f32, f32)> {
    use typst_core::entities::layout_types::Color;
    let n = n_samples.max(2);
    (0..n)
        .map(|i| {
            let t = i as f32 / (n - 1) as f32;
            let c = linear.sample(t);  // P270 dispatcher arm Cmyk
            match c {
                Color::Cmyk { c, m, y, k } => (
                    c.clamp(0.0, 1.0), m.clamp(0.0, 1.0),
                    y.clamp(0.0, 1.0), k.clamp(0.0, 1.0)
                ),
                _ => {
                    // Fallback precaução: convert via sRGB intermediate.
                    let (r, g, b, _) = c.to_rgba_f32();
                    rgb_to_cmyk(r, g, b)
                }
            }
        })
        .collect()
}

/// P270.2 — Helper amostragem CMYK 4-component para Radial gradient.
///
/// Paridade literal `multispace_sample_stops_linear_cmyk` (Linear).
/// Focal_* P269 preservados na sample via `radial.sample(t)` dispatcher.
pub(crate) fn multispace_sample_stops_radial_cmyk(
    radial: &typst_core::entities::gradient::Radial,
    n_samples: usize,
) -> Vec<(f32, f32, f32, f32)> {
    use typst_core::entities::layout_types::Color;
    let n = n_samples.max(2);
    (0..n)
        .map(|i| {
            let t = i as f32 / (n - 1) as f32;
            let c = radial.sample(t);  // P270 dispatcher arm Cmyk
            match c {
                Color::Cmyk { c, m, y, k } => (
                    c.clamp(0.0, 1.0), m.clamp(0.0, 1.0),
                    y.clamp(0.0, 1.0), k.clamp(0.0, 1.0)
                ),
                _ => {
                    let (r, g, b, _) = c.to_rgba_f32();
                    rgb_to_cmyk(r, g, b)
                }
            }
        })
        .collect()
}

