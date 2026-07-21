//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/gradients/linear.md
//! @prompt-hash 02428077
//! @layer L3
//! @updated 2026-05-19
//!
//! Linear gradients — coords + multispace sample stops (P263).
//!
//! Extraído de `gradients.rs` em P307b.2 (ADR-0100 / diagnóstico
//! P307a §5). Conteúdo bit-exact pré e pós migração.

use typst_core::entities::gradient::Linear;

pub(crate) fn compute_axial_coords(
    angle_rad: f64,
    x0: f64,
    y0: f64,
    w: f64,
    h: f64,
) -> (f64, f64, f64, f64) {
    let cx = x0 + w / 2.0;
    let cy = y0 + h / 2.0;
    let dx = angle_rad.cos();
    let dy = angle_rad.sin();
    let hx = (w / 2.0) * dx;
    let hy = (h / 2.0) * dy;
    (cx - hx, cy - hy, cx + hx, cy + hy)
}

/// P263 + P270.1 — Amostra N stops intermédios em sRGB normalizado.
///
/// **P263** materializou pipeline Oklab hardcoded.
/// **P270.1** renomeia para reflectir consciência multi-space —
/// `Linear::sample(t)` despacha via `interpolate_in_space` dispatcher
/// (P270; ADR-0091) per `linear.space`. Body literal preserved: o
/// helper continua a chamar `linear.sample(t)` + `to_rgba_f32()`;
/// só o nome muda. Default `linear.space = ColorSpace::Oklab`
/// preserva bytes P263 bit-exact (arm Oklab dispatcher chama
/// `interpolate_oklab` P262 literal).
///
/// 7 spaces materializados L3 emit via este helper: Oklab/Oklch/sRGB/
/// Luma/LinearRGB/HSL/HSV. CMYK preservado via pipeline natural
/// CMYK→sRGB sub-óptimo até P270.2 (`/DeviceCMYK` directo).
///
/// Output: `Vec<(r, g, b)>` em [0, 1] sRGB normalizado.
pub(crate) fn multispace_sample_stops(
    linear: &typst_core::entities::gradient::Linear,
    n_samples: usize,
) -> Vec<(f32, f32, f32)> {
    let n = n_samples.max(2);
    (0..n)
        .map(|i| {
            let t = i as f32 / (n - 1) as f32;
            let c = linear.sample(t);
            let (r, g, b, _) = c.to_rgba_f32();
            (r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0))
        })
        .collect()
}
