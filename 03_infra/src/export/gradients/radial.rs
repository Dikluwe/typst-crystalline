//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/gradients/radial.md
//! @prompt-hash 88610ac6
//! @layer L3
//! @updated 2026-05-19
//!
//! Radial gradients — coords + multispace sample stops (P264-P265).
//!
//! Extraído de `gradients.rs` em P307b.2 (ADR-0100 / diagnóstico
//! P307a §5). Conteúdo bit-exact pré e pós migração.

use typst_core::entities::axes::Axes;
use typst_core::entities::gradient::Radial;
use typst_core::entities::layout_types::Ratio;

pub(crate) fn compute_radial_coords(
    center: typst_core::entities::axes::Axes<typst_core::entities::layout_types::Ratio>,
    radius: typst_core::entities::layout_types::Ratio,
    focal_center: typst_core::entities::axes::Axes<typst_core::entities::layout_types::Ratio>,
    focal_radius: typst_core::entities::layout_types::Ratio,
    w: f64,
    h: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    let cx = center.x.0 * w;
    let cy = center.y.0 * h;
    let r = radius.0 * w.min(h);
    // P269 — focal real (defaults focal_center=center, focal_radius=0
    // produzem (cx, cy, 0.0, cx, cy, r) idêntico P265).
    let fx = focal_center.x.0 * w;
    let fy = focal_center.y.0 * h;
    let fr = focal_radius.0 * w.min(h);
    (fx, fy, fr, cx, cy, r)
}

/// P265 + P270.1 — Amostra N stops intermédios em sRGB normalizado.
///
/// **P265** materializou pipeline Oklab hardcoded.
/// **P270.1** renomeia para reflectir consciência multi-space —
/// `Radial::sample(t)` despacha via `interpolate_in_space`
/// (P270; ADR-0091) per `radial.space`. Body literal preserved.
/// Default Oklab preserva bytes P265 bit-exact. 7 spaces
/// materializados; CMYK via pipeline natural até P270.2.
/// Paridade literal `multispace_sample_stops` (Linear).
pub(crate) fn multispace_sample_stops_radial(
    radial: &typst_core::entities::gradient::Radial,
    n_samples: usize,
) -> Vec<(f32, f32, f32)> {
    let n = n_samples.max(2);
    (0..n)
        .map(|i| {
            let t = i as f32 / (n - 1) as f32;
            let c = radial.sample(t);
            let (r, g, b, _) = c.to_rgba_f32();
            (r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0))
        })
        .collect()
}
