//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/gradients/conic.md
//! @prompt-hash f96414bd
//! @layer L3
//! @updated 2026-05-19
//!
//! Conic Type 6 Coons Patch Mesh — bezier + compute_coons_patches + emit streams (P272).
//!
//! Extraído de `gradients.rs` em P307b.2 (ADR-0100 / diagnóstico
//! P307a §5). Conteúdo bit-exact pré e pós migração.

use typst_core::entities::gradient::Conic;
use typst_core::entities::layout_types::Color;

use super::cmyk::rgb_to_cmyk;

pub(crate) fn multispace_sample_stops_conic(
    conic: &typst_core::entities::gradient::Conic,
    n_samples: usize,
) -> Vec<(f32, f32, f32)> {
    let n = n_samples.max(2);
    (0..n)
        .map(|i| {
            let t = i as f32 / (n - 1) as f32;
            let c = conic.sample(t);
            let (r, g, b, _) = c.to_rgba_f32();
            (r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0))
        })
        .collect()
}

// ── P272 — Conic Type 6 Coons Patch Mesh L3 emit estratégia única ──
//
// **ADR-0090 REVOGADO P272**: Type 4 Gouraud descontinuado;
// `emit_conic_gouraud_stream` + `compute_adaptive_n_conic` +
// `oklab_delta_e` (P268+P268.2) removidos.
//
// **ADR-0092 expandida cumulativamente P272 (Cenário A revisado FINAL)**:
// Cristalino tem **estratégia única Coons** para 8/8 spaces — Type 6
// Coons via `emit_conic_coons_stream_rgb` (P272 N=stops*4; 7 spaces
// RGB-family + perceptual) e `emit_conic_coons_stream_cmyk` (P270.4
// N=stops; CMYK).

/// P270.3 — Matemática Bezier cúbico para arc círculo (Stanislaw Adaszewski).
///
/// Returns 2 control points entre start_angle e end_angle. Standard
/// approximation: `offset = radius * (4/3) * tan(angle/4)`. Erro máximo
/// ~0.0003 para quartos (90°); menor para N>4 patches angulares.
///
/// Reference: Stanislaw Adaszewski "Drawing a Circle with Bezier Curves".
pub(crate) fn bezier_control_points_for_arc(
    center: (f32, f32),
    radius: f32,
    start_angle: f32,
    end_angle: f32,
) -> [(f32, f32); 2] {
    let angle_delta = end_angle - start_angle;
    let offset = radius * (4.0 / 3.0) * (angle_delta / 4.0).tan();

    let (sin_s, cos_s) = start_angle.sin_cos();
    let (sin_e, cos_e) = end_angle.sin_cos();

    // Control point 1: rotated 90° from start tangent.
    let cp1 = (
        center.0 + radius * cos_s - offset * sin_s,
        center.1 + radius * sin_s + offset * cos_s,
    );
    // Control point 2: rotated -90° from end tangent.
    let cp2 = (
        center.0 + radius * cos_e + offset * sin_e,
        center.1 + radius * sin_e - offset * cos_e,
    );

    [cp1, cp2]
}

/// P270.3 — Strategy "1 patch per stop" (paridade Typst original blog 2023).
///
/// N stops → N patches angulares. Cada patch cobre 360°/N graus.
pub(crate) fn compute_coons_patches_n_stops(
    conic: &typst_core::entities::gradient::Conic,
) -> usize {
    conic.stops.len()
}

/// P272 — Strategy "N stops * 4" patches angulares (divergência intencional Typst original blog 2023 "1 patch per stop").
///
/// Justificativa: qualidade visual angular superior; corner colors via
/// `Conic::sample(t)` dispatcher P270 (interpolate_in_space per
/// `conic.space`). Cap LOC accommodates extension.
pub(crate) fn compute_coons_patches_n_stops_extended(
    conic: &typst_core::entities::gradient::Conic,
) -> usize {
    conic.stops.len() * 4
}

/// P272 — Emit Coons Patch Mesh stream binary (Type 6) RGB.
///
/// P270.3 helper extension: strategy **N = stops * 4** patches angulares
/// (vs P270.3 "1 patch per stop"). Corner colors via `Conic::sample(t)`
/// dispatcher P270 — dispatches `interpolate_in_space` per `conic.space`
/// (7 spaces RGB-family + perceptual).
///
/// Per patch (flag=0, "new patch"):
/// - 1 byte flag.
/// - 12 control points × 2 coord bytes = 24 bytes.
/// - 4 corner colors × 3 RGB bytes = 12 bytes.
/// - **Total: 37 bytes per patch**.
///
/// Layout 12 control points preserved literal P270.3 (PDF Type 6 Coons;
/// per ISO 32000-1 §7.5.7.4):
/// - P0..P3: top edge (centro → edge_start).
/// - P4..P5: right edge interior (edge_start → edge_end via arc; Bezier
///   control points).
/// - P6: corner edge_end.
/// - P7..P8: bottom edge interior (edge_end → centro).
/// - P9: corner centro baixo.
/// - P10..P11: left edge interior (centro → centro; degenerate).
///
/// **P272 corner colors via `Conic::sample(t)`**:
/// - corner0/corner1 (P0+P3) = conic.sample(t_start).
/// - corner2/corner3 (P6+P9) = conic.sample(t_end).
/// - t_start = i / n_patches; t_end = (i+1) / n_patches.
///
/// Dispatcher P270 `interpolate_in_space` chamado automaticamente em
/// `Conic::sample` per `conic.space` (Oklab/Oklch/sRGB/Luma/LinearRGB/
/// HSL/HSV). Default Oklab preserva qualidade perceptual.
///
/// **ADR-0090 REVOGADO P272**: Type 4 Gouraud descontinuado;
/// estratégia única Coons para 8/8 spaces (ADR-0092 expandida).
pub(crate) fn emit_conic_coons_stream_rgb(
    conic: &typst_core::entities::gradient::Conic,
) -> Vec<u8> {
    let n_patches = compute_coons_patches_n_stops_extended(conic);
    if n_patches == 0 {
        return Vec::new();
    }

    let mut stream = Vec::with_capacity(37 * n_patches);

    let center = (0.5_f32, 0.5_f32);
    let radius = 0.5_f32;

    let push_coord = |stream: &mut Vec<u8>, v: f32| {
        stream.push((v.clamp(0.0, 1.0) * 255.0) as u8);
    };
    let push_color_rgb =
        |stream: &mut Vec<u8>, c: typst_core::entities::layout_types::Color| {
            let (r, g, b, _) = c.to_rgba_f32();
            stream.push((r.clamp(0.0, 1.0) * 255.0) as u8);
            stream.push((g.clamp(0.0, 1.0) * 255.0) as u8);
            stream.push((b.clamp(0.0, 1.0) * 255.0) as u8);
        };

    let angle_offset = conic.angle.to_rad() as f32;
    let n = n_patches as f32;

    for i in 0..n_patches {
        let t_start = (i as f32) / n;
        let t_end = ((i + 1) as f32) / n;

        // Corner colors via Conic::sample (dispatcher P270 interpolate_in_space).
        let color_start = conic.sample(t_start);
        let color_end = conic.sample(t_end);

        let angle_start = angle_offset + t_start * std::f32::consts::TAU;
        let angle_end = angle_offset + t_end * std::f32::consts::TAU;

        let (sin_s, cos_s) = angle_start.sin_cos();
        let (sin_e, cos_e) = angle_end.sin_cos();
        let edge_start = (center.0 + radius * cos_s, center.1 + radius * sin_s);
        let edge_end = (center.0 + radius * cos_e, center.1 + radius * sin_e);

        let cp_arc =
            bezier_control_points_for_arc(center, radius, angle_start, angle_end);

        stream.push(0u8);

        // 12 control points (layout preserved literal P270.3).
        push_coord(&mut stream, center.0);
        push_coord(&mut stream, center.1);
        push_coord(&mut stream, center.0 + (edge_start.0 - center.0) / 3.0);
        push_coord(&mut stream, center.1 + (edge_start.1 - center.1) / 3.0);
        push_coord(&mut stream, center.0 + 2.0 * (edge_start.0 - center.0) / 3.0);
        push_coord(&mut stream, center.1 + 2.0 * (edge_start.1 - center.1) / 3.0);
        push_coord(&mut stream, edge_start.0);
        push_coord(&mut stream, edge_start.1);
        push_coord(&mut stream, cp_arc[0].0);
        push_coord(&mut stream, cp_arc[0].1);
        push_coord(&mut stream, cp_arc[1].0);
        push_coord(&mut stream, cp_arc[1].1);
        push_coord(&mut stream, edge_end.0);
        push_coord(&mut stream, edge_end.1);
        push_coord(&mut stream, edge_end.0 + (center.0 - edge_end.0) / 3.0);
        push_coord(&mut stream, edge_end.1 + (center.1 - edge_end.1) / 3.0);
        push_coord(&mut stream, edge_end.0 + 2.0 * (center.0 - edge_end.0) / 3.0);
        push_coord(&mut stream, edge_end.1 + 2.0 * (center.1 - edge_end.1) / 3.0);
        push_coord(&mut stream, center.0);
        push_coord(&mut stream, center.1);
        push_coord(&mut stream, center.0);
        push_coord(&mut stream, center.1);
        push_coord(&mut stream, center.0);
        push_coord(&mut stream, center.1);

        // 4 corner colors RGB (12 bytes); interpolated via Conic::sample.
        push_color_rgb(&mut stream, color_start); // corner0
        push_color_rgb(&mut stream, color_start); // corner1
        push_color_rgb(&mut stream, color_end); // corner2
        push_color_rgb(&mut stream, color_end); // corner3
    }

    stream
}

/// P270.4 — Emit Coons Patch Mesh stream binary (Type 6) CMYK variant.
///
/// Paridade estrutural `emit_conic_coons_stream` P270.3 RGB. Mudanças:
/// - Corner colors: 4 bytes per corner (vs 3 RGB) — c, m, y, k.
/// - Total bytes per patch: 1 flag + 24 control points + 16 corner CMYK
///   = **41 bytes per patch** (vs 37 RGB).
/// - N stops → 41N bytes total.
///
/// Layout 12 control points preserved literal de `emit_conic_coons_stream`
/// (P270.3 RGB). Apenas corner colors mudam para 4-component CMYK.
///
/// Corner colors via `Color::Cmyk` pattern-match (paridade
/// `multispace_sample_stops_linear_cmyk` P270.2) + `rgb_to_cmyk`
/// fallback precaução (helper P270.2 reused).
///
/// Bug vanilla #4422 resolvido por construção (cristalino emit
/// `/ColorSpace /DeviceCMYK` correcto via Coons patch mesh).
pub(crate) fn emit_conic_coons_stream_cmyk(
    conic: &typst_core::entities::gradient::Conic,
) -> Vec<u8> {
    use typst_core::entities::layout_types::Color;
    let n = compute_coons_patches_n_stops(conic);
    if n == 0 {
        return Vec::new();
    }

    let mut stream = Vec::with_capacity(41 * n);

    let center = (0.5_f32, 0.5_f32);
    let radius = 0.5_f32;

    let push_coord = |stream: &mut Vec<u8>, v: f32| {
        stream.push((v.clamp(0.0, 1.0) * 255.0) as u8);
    };

    // Extract CMYK 4 components per stop (paridade P270.2 pattern).
    let to_cmyk = |c: Color| -> (f32, f32, f32, f32) {
        match c {
            Color::Cmyk { c, m, y, k } => (c, m, y, k),
            _ => {
                let (r, g, b, _) = c.to_rgba_f32();
                rgb_to_cmyk(r, g, b)
            }
        }
    };
    let push_color_cmyk = |stream: &mut Vec<u8>, c: Color| {
        let (cy, mg, yl, kk) = to_cmyk(c);
        stream.push((cy.clamp(0.0, 1.0) * 255.0) as u8);
        stream.push((mg.clamp(0.0, 1.0) * 255.0) as u8);
        stream.push((yl.clamp(0.0, 1.0) * 255.0) as u8);
        stream.push((kk.clamp(0.0, 1.0) * 255.0) as u8);
    };

    let angle_offset = conic.angle.to_rad() as f32;

    for i in 0..n {
        let stop_curr = &conic.stops[i];
        let stop_next = &conic.stops[(i + 1) % n];

        let angle_start = angle_offset + (i as f32) / (n as f32) * std::f32::consts::TAU;
        let angle_end =
            angle_offset + ((i + 1) as f32) / (n as f32) * std::f32::consts::TAU;

        let (sin_s, cos_s) = angle_start.sin_cos();
        let (sin_e, cos_e) = angle_end.sin_cos();
        let edge_start = (center.0 + radius * cos_s, center.1 + radius * sin_s);
        let edge_end = (center.0 + radius * cos_e, center.1 + radius * sin_e);

        let cp_arc =
            bezier_control_points_for_arc(center, radius, angle_start, angle_end);

        stream.push(0u8); // flag = new patch

        // 12 control points × 2 coord bytes (paridade literal P270.3 RGB).
        push_coord(&mut stream, center.0);
        push_coord(&mut stream, center.1);
        push_coord(&mut stream, center.0 + (edge_start.0 - center.0) / 3.0);
        push_coord(&mut stream, center.1 + (edge_start.1 - center.1) / 3.0);
        push_coord(&mut stream, center.0 + 2.0 * (edge_start.0 - center.0) / 3.0);
        push_coord(&mut stream, center.1 + 2.0 * (edge_start.1 - center.1) / 3.0);
        push_coord(&mut stream, edge_start.0);
        push_coord(&mut stream, edge_start.1);
        push_coord(&mut stream, cp_arc[0].0);
        push_coord(&mut stream, cp_arc[0].1);
        push_coord(&mut stream, cp_arc[1].0);
        push_coord(&mut stream, cp_arc[1].1);
        push_coord(&mut stream, edge_end.0);
        push_coord(&mut stream, edge_end.1);
        push_coord(&mut stream, edge_end.0 + (center.0 - edge_end.0) / 3.0);
        push_coord(&mut stream, edge_end.1 + (center.1 - edge_end.1) / 3.0);
        push_coord(&mut stream, edge_end.0 + 2.0 * (center.0 - edge_end.0) / 3.0);
        push_coord(&mut stream, edge_end.1 + 2.0 * (center.1 - edge_end.1) / 3.0);
        push_coord(&mut stream, center.0);
        push_coord(&mut stream, center.1);
        push_coord(&mut stream, center.0);
        push_coord(&mut stream, center.1);
        push_coord(&mut stream, center.0);
        push_coord(&mut stream, center.1);

        // 4 corner colors CMYK (16 bytes vs 12 RGB).
        push_color_cmyk(&mut stream, stop_curr.color); // corner0
        push_color_cmyk(&mut stream, stop_curr.color); // corner1
        push_color_cmyk(&mut stream, stop_next.color); // corner2
        push_color_cmyk(&mut stream, stop_next.color); // corner3
    }

    stream
}
