//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/gradients/adaptive.md
//! @prompt-hash e654a9eb
//! @layer L3
//! @updated 2026-05-19
//!
//! Adaptive N multispace — perceptual distance + adaptive_n_for_stops (P274).
//!
//! Extraído de `gradients.rs` em P307b.2 (ADR-0100 / diagnóstico
//! P307a §5). Conteúdo bit-exact pré e pós migração.

use typst_core::entities::layout_types::{Color, ColorSpace};
use typst_core::entities::{
    gradient::{Gradient, GradientStop},
    layout_types::Ratio,
};

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

/// P1229 — aproxima Linear/Radial multi-space por stops sRGB.
pub(crate) fn svg_adaptive_stops(gradient: &Gradient) -> Vec<GradientStop> {
    const THRESHOLD: f32 = 0.001;
    const MAX_SUBDIVISIONS: u64 = 64;

    let (stops, anti_alias, clamped_srgb_error) = match gradient {
        Gradient::Linear(value) => (
            &*value.stops,
            value.anti_alias,
            matches!(value.space, ColorSpace::Oklab | ColorSpace::Oklch),
        ),
        Gradient::Radial(value) => (
            &*value.stops,
            value.anti_alias,
            matches!(value.space, ColorSpace::Oklab | ColorSpace::Oklch),
        ),
        Gradient::Conic(value) => {
            return value
                .stops
                .iter()
                .zip(value.effective_offsets())
                .map(|(stop, offset)| {
                    GradientStop::new(stop.color, Ratio(f64::from(offset)))
                })
                .collect();
        }
    };

    if stops.is_empty() {
        return Vec::new();
    }

    let offsets = effective_offsets_f64(stops);

    let sample = |t: f64| sample_gradient_precise_for_svg(gradient, t);

    let mut out = Vec::new();
    for index in 0..stops.len().saturating_sub(1) {
        let t0 = offsets[index];
        let t1 = offsets[index + 1];
        let c0 = stops[index].color;
        let c1 = stops[index + 1].color;
        out.push(GradientStop::new(c0, Ratio(t0)));
        if anti_alias && t1 > t0 {
            let dt = t1 - t0;
            let mut previous = c0;
            let mut next_t = t1;
            let mut next = c1;
            let mut subdivisions = 1_u64;
            let mut position = 1_u64;

            loop {
                let midpoint = t0 + dt * (position as f64 - 0.5) / subdivisions as f64;
                let exact = sample(midpoint);
                let approximate = if clamped_srgb_error {
                    mix_clamped_srgb_half(previous, next)
                } else {
                    previous.mix(next, 0.5, Some(ColorSpace::Srgb))
                };
                let error = if clamped_srgb_error {
                    premultiplied_srgb_error(exact, approximate)
                } else {
                    raw_premultiplied_srgb_error(exact, approximate)
                };
                if subdivisions < MAX_SUBDIVISIONS && error > THRESHOLD {
                    subdivisions *= 2;
                    position = 2 * position - 1;
                } else if position >= subdivisions {
                    break;
                } else {
                    out.push(GradientStop::new(next, Ratio(f64::from(next_t))));
                    previous = next;
                    let shift = position.trailing_zeros();
                    subdivisions >>= shift;
                    position >>= shift;
                    position += 1;
                }

                next_t = t0 + dt * position as f64 / subdivisions as f64;
                next = sample(next_t);
            }
        }
    }
    let last = stops.len() - 1;
    out.push(GradientStop::new(stops[last].color, Ratio(offsets[last])));
    out
}

fn effective_offsets_f64(stops: &[GradientStop]) -> Vec<f64> {
    let count = stops.len();
    if count == 0 {
        return Vec::new();
    }
    if count == 1 {
        return vec![stops[0].offset.map(|ratio| ratio.0).unwrap_or(0.0)];
    }
    let mut source: Vec<Option<f64>> =
        stops.iter().map(|stop| stop.offset.map(|r| r.0)).collect();
    source[0].get_or_insert(0.0);
    source[count - 1].get_or_insert(1.0);
    let mut resolved = vec![0.0; count];
    let mut index = 0;
    while index < count {
        if let Some(value) = source[index] {
            resolved[index] = value;
            index += 1;
            continue;
        }
        let mut end = index;
        while end < count && source[end].is_none() {
            end += 1;
        }
        let previous = resolved[index - 1];
        let next = source[end].expect("last effective offset is explicit");
        let gap = end - index + 1;
        for step in 0..gap {
            resolved[index + step] =
                previous + (next - previous) * (step + 1) as f64 / gap as f64;
        }
        index = end;
    }
    resolved
}

fn sample_gradient_precise_for_svg(gradient: &Gradient, t: f64) -> Color {
    let (stops, space) = match gradient {
        Gradient::Linear(value) => (&*value.stops, value.space),
        Gradient::Radial(value) => (&*value.stops, value.space),
        Gradient::Conic(value) => return value.sample(t as f32),
    };
    if !matches!(
        space,
        ColorSpace::Oklab
            | ColorSpace::LinearRgb
            | ColorSpace::Oklch
            | ColorSpace::Hsl
            | ColorSpace::Hsv
    ) {
        return match gradient {
            Gradient::Linear(value) => value.sample(t as f32),
            Gradient::Radial(value) => value.sample(t as f32),
            Gradient::Conic(_) => unreachable!(),
        };
    }
    if stops.is_empty() {
        return Color::rgb(0, 0, 0);
    }
    if stops.len() == 1 {
        return stops[0].color;
    }
    let t = t.clamp(0.0, 1.0);
    let offsets = effective_offsets_f64(stops);
    let mut right = offsets.partition_point(|offset| *offset < t);
    if right == 0 {
        while right + 1 < offsets.len() && offsets[right + 1] == 0.0 {
            right += 1;
        }
        return stops[right].color;
    }
    if right >= stops.len() {
        return stops[stops.len() - 1].color;
    }
    let left = right - 1;
    let local = (t - offsets[left]) / (offsets[right] - offsets[left]);
    let components = |color: Color| match (space, color.to_space(space)) {
        (ColorSpace::Oklab, Color::Oklab { l, a, b, alpha }) => [l, a, b, alpha],
        (ColorSpace::LinearRgb, Color::LinearRgb { r, g, b, a }) => [r, g, b, a],
        (ColorSpace::Oklch, Color::Oklch { l, c, h, alpha }) => [l, c, h, alpha],
        (ColorSpace::Hsl, Color::Hsl { h, s, l, a }) => [h, s, l, a],
        (ColorSpace::Hsv, Color::Hsv { h, s, v, a }) => [h, s, v, a],
        _ => unreachable!("precise SVG sampler space mismatch"),
    };
    let mut c0 = components(stops[left].color);
    let mut c1 = components(stops[right].color);
    let w0 = (1.0 - local) as f32;
    let w1 = local as f32;
    let total = w0 + w1;
    if let Some(hue_index) = match space {
        ColorSpace::Oklch => Some(2),
        ColorSpace::Hsl | ColorSpace::Hsv => Some(0),
        _ => None,
    } {
        if (c0[hue_index] - c1[hue_index]).abs() > 180.0 {
            if c0[hue_index] < c1[hue_index] {
                c0[hue_index] += 360.0;
            } else {
                c1[hue_index] += 360.0;
            }
        }
    }
    let mixed = [
        (w0 * c0[0] + w1 * c1[0]) / total,
        (w0 * c0[1] + w1 * c1[1]) / total,
        (w0 * c0[2] + w1 * c1[2]) / total,
        (w0 * c0[3] + w1 * c1[3]) / total,
    ];
    match space {
        ColorSpace::Oklab => Color::oklab(mixed[0], mixed[1], mixed[2], mixed[3]),
        ColorSpace::LinearRgb => {
            Color::linear_rgb(mixed[0], mixed[1], mixed[2], mixed[3])
        }
        ColorSpace::Oklch => {
            Color::oklch(mixed[0], mixed[1], mixed[2].rem_euclid(360.0), mixed[3])
        }
        ColorSpace::Hsl => {
            Color::hsl(mixed[0].rem_euclid(360.0), mixed[1], mixed[2], mixed[3])
        }
        ColorSpace::Hsv => {
            Color::hsv(mixed[0].rem_euclid(360.0), mixed[1], mixed[2], mixed[3])
        }
        _ => unreachable!(),
    }
}

fn premultiplied_srgb_error(left: Color, right: Color) -> f32 {
    let (lr, lg, lb) = clamped_premultiplied_srgb(left);
    let (rr, rg, rb) = clamped_premultiplied_srgb(right);
    let dr = lr - rr;
    let dg = lg - rg;
    let db = lb - rb;
    (dr * dr + dg * dg + db * db).sqrt()
}

fn raw_premultiplied_srgb_error(left: Color, right: Color) -> f32 {
    let (lr, lg, lb, la) = left.to_rgba_f32();
    let (rr, rg, rb, ra) = right.to_rgba_f32();
    let dr = lr * la - rr * ra;
    let dg = lg * la - rg * ra;
    let db = lb * la - rb * ra;
    (dr * dr + dg * dg + db * db).sqrt()
}

fn mix_clamped_srgb_half(left: Color, right: Color) -> Color {
    let (lr, lg, lb, la) = left.to_rgba_f32();
    let (rr, rg, rb, ra) = right.to_rgba_f32();
    let half = 0.5_f32;
    let total = half + half;
    Color::srgb_f32(
        (half * lr.clamp(0.0, 1.0) + half * rr.clamp(0.0, 1.0)) / total,
        (half * lg.clamp(0.0, 1.0) + half * rg.clamp(0.0, 1.0)) / total,
        (half * lb.clamp(0.0, 1.0) + half * rb.clamp(0.0, 1.0)) / total,
        (half * la + half * ra) / total,
    )
}

fn clamped_premultiplied_srgb(color: Color) -> (f32, f32, f32) {
    let (r, g, b, alpha) = color.to_rgba_f32();
    (r.clamp(0.0, 1.0) * alpha, g.clamp(0.0, 1.0) * alpha, b.clamp(0.0, 1.0) * alpha)
}

#[cfg(test)]
mod p1257_tests {
    use super::*;
    use typst_core::entities::axes::Axes;
    use typst_core::entities::layout_types::{Angle, Color, ColorSpace, Ratio};

    fn envelope(gradient: &Gradient) -> (usize, f32, f32, f32, f32) {
        let stops = svg_adaptive_stops(gradient);
        let mut pre_max = 0.0_f32;
        let mut post_max = 0.0_f32;
        let mut pre_t = 0.0_f32;
        let mut post_t = 0.0_f32;
        for index in 0..=4096 {
            let t = index as f32 / 4096.0;
            let exact = match gradient {
                Gradient::Linear(value) => value.sample(t),
                Gradient::Radial(value) => value.sample(t),
                Gradient::Conic(value) => value.sample(t),
            }
            .to_rgba_f32();
            let right = stops
                .partition_point(|stop| stop.offset.expect("offset").0 as f32 <= t)
                .min(stops.len() - 1);
            let left = right.saturating_sub(1);
            let lo = &stops[left];
            let hi = &stops[right];
            let a = lo.offset.expect("offset").0 as f32;
            let b = hi.offset.expect("offset").0 as f32;
            let u = if b > a { (t - a) / (b - a) } else { 0.0 };
            let pre_lo = lo.color.to_rgba_f32();
            let pre_hi = hi.color.to_rgba_f32();
            let quantized = |color: Color| {
                let (r, g, b, a) = color.to_srgb();
                (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0)
            };
            let post_lo = quantized(lo.color);
            let post_hi = quantized(hi.color);
            let error = |x: (f32, f32, f32, f32),
                         y0: (f32, f32, f32, f32),
                         y1: (f32, f32, f32, f32)| {
                let channel = |v: (f32, f32, f32, f32), i| [v.0, v.1, v.2][i] * v.3;
                (0..3)
                    .map(|i| {
                        (channel(x, i)
                            - ((1.0 - u) * channel(y0, i) + u * channel(y1, i)))
                        .powi(2)
                    })
                    .sum::<f32>()
                    .sqrt()
            };
            let pre = error(exact, pre_lo, pre_hi);
            let post = error(exact, post_lo, post_hi);
            if pre > pre_max {
                pre_max = pre;
                pre_t = t;
            }
            if post > post_max {
                post_max = post;
                post_t = t;
            }
        }
        (stops.len(), pre_max, post_max, pre_t, post_t)
    }

    fn stops() -> Vec<GradientStop> {
        vec![
            GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
            GradientStop::new(Color::rgb(0, 255, 0), Ratio(0.37)),
            GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
        ]
    }

    fn p1278_expected_polar_sample(space: ColorSpace, t: f64) -> Color {
        let local = t / 0.37_f64;
        let w0 = (1.0 - local) as f32;
        let w1 = local as f32;
        let total = w0 + w1;
        let mix = |left: f32, right: f32| (w0 * left + w1 * right) / total;
        match space {
            ColorSpace::Oklch => Color::oklch(
                mix(0.62, 0.71),
                mix(0.19, 0.13),
                mix(350.0, 370.0).rem_euclid(360.0),
                mix(0.8, 0.6),
            ),
            ColorSpace::Hsl => Color::hsl(
                mix(350.0, 370.0).rem_euclid(360.0),
                mix(0.72, 0.48),
                mix(0.31, 0.68),
                mix(0.8, 0.6),
            ),
            ColorSpace::Hsv => Color::hsv(
                mix(350.0, 370.0).rem_euclid(360.0),
                mix(0.72, 0.48),
                mix(0.31, 0.68),
                mix(0.8, 0.6),
            ),
            _ => unreachable!(),
        }
    }

    #[test]
    fn p1278_sampler_polar_preserva_precisao_e_pesos_vanilla() {
        let t = 0.123_456_789_012_3_f64;
        for space in [ColorSpace::Oklch, ColorSpace::Hsl, ColorSpace::Hsv] {
            let (first, second) = match space {
                ColorSpace::Oklch => (
                    Color::oklch(0.62, 0.19, 350.0, 0.8),
                    Color::oklch(0.71, 0.13, 10.0, 0.6),
                ),
                ColorSpace::Hsl => (
                    Color::hsl(350.0, 0.72, 0.31, 0.8),
                    Color::hsl(10.0, 0.48, 0.68, 0.6),
                ),
                ColorSpace::Hsv => (
                    Color::hsv(350.0, 0.72, 0.31, 0.8),
                    Color::hsv(10.0, 0.48, 0.68, 0.6),
                ),
                _ => unreachable!(),
            };
            let gradient = Gradient::linear_with_space(
                vec![
                    GradientStop::new(first, Ratio(0.0)),
                    GradientStop::new(second, Ratio(0.37)),
                    GradientStop::new(second, Ratio(1.0)),
                ],
                Angle::deg(0.0),
                space,
            );
            assert_eq!(
                sample_gradient_precise_for_svg(&gradient, t),
                p1278_expected_polar_sample(space, t),
                "precisao divergente em {space:?}"
            );
        }
    }

    #[test]
    fn p1257_pre_pos_serializacao_oklab_below_cap() {
        let gradient =
            Gradient::linear_with_space(stops(), Angle::deg(0.0), ColorSpace::Oklab);
        let measured = envelope(&gradient);
        eprintln!(
            "P1257 oklab stops={} pre_max={:.9} post_max={:.9} pre_t={:.9} post_t={:.9}",
            measured.0, measured.1, measured.2, measured.3, measured.4
        );
        assert!(measured.1.is_finite() && measured.2.is_finite());
    }

    #[test]
    fn p1257_pre_pos_serializacao_linear_rgb() {
        let gradient =
            Gradient::linear_with_space(stops(), Angle::deg(0.0), ColorSpace::LinearRgb);
        let measured = envelope(&gradient);
        eprintln!("P1257 linear-rgb stops={} pre_max={:.9} post_max={:.9} pre_t={:.9} post_t={:.9}", measured.0, measured.1, measured.2, measured.3, measured.4);
        assert!(measured.1.is_finite() && measured.2.is_finite());
    }

    fn p1262_named_color(r: u8, g: u8, b: u8, alpha: f32) -> Color {
        Color::srgb_f32(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, alpha)
    }

    fn p1262_stopsets() -> [(&'static str, Vec<GradientStop>, usize); 4] {
        let red = p1262_named_color(255, 65, 54, 1.0);
        let green = p1262_named_color(46, 204, 64, 1.0);
        let blue = p1262_named_color(0, 116, 217, 1.0);
        let alpha_red = p1262_named_color(255, 65, 54, 0.4);
        let alpha_green = p1262_named_color(46, 204, 64, 0.4);
        [
            (
                "base",
                vec![
                    GradientStop::new(red, Ratio(0.0)),
                    GradientStop::new(green, Ratio(0.37)),
                    GradientStop::new(blue, Ratio(1.0)),
                ],
                43,
            ),
            (
                "coincident",
                vec![
                    GradientStop::new(red, Ratio(0.0)),
                    GradientStop::new(green, Ratio(0.0)),
                    GradientStop::new(blue, Ratio(1.0)),
                ],
                19,
            ),
            (
                "alpha-first",
                vec![
                    GradientStop::new(alpha_red, Ratio(0.0)),
                    GradientStop::new(green, Ratio(0.37)),
                    GradientStop::new(blue, Ratio(1.0)),
                ],
                39,
            ),
            (
                "alpha-mid",
                vec![
                    GradientStop::new(red, Ratio(0.0)),
                    GradientStop::new(alpha_green, Ratio(0.37)),
                    GradientStop::new(blue, Ratio(1.0)),
                ],
                37,
            ),
        ]
    }

    fn p1262_full_stopsets() -> Vec<(&'static str, Vec<GradientStop>)> {
        let red = p1262_named_color(255, 65, 54, 1.0);
        let green = p1262_named_color(46, 204, 64, 1.0);
        let blue = p1262_named_color(0, 116, 217, 1.0);
        let alpha_red = p1262_named_color(255, 65, 54, 0.4);
        let alpha_green = p1262_named_color(46, 204, 64, 0.4);
        let alpha_blue = p1262_named_color(0, 116, 217, 0.4);
        vec![
            (
                "base",
                vec![
                    GradientStop::new(red, Ratio(0.0)),
                    GradientStop::new(green, Ratio(0.37)),
                    GradientStop::new(blue, Ratio(1.0)),
                ],
            ),
            (
                "two",
                vec![
                    GradientStop::new(red, Ratio(0.0)),
                    GradientStop::new(blue, Ratio(1.0)),
                ],
            ),
            (
                "coincident",
                vec![
                    GradientStop::new(red, Ratio(0.0)),
                    GradientStop::new(green, Ratio(0.0)),
                    GradientStop::new(blue, Ratio(1.0)),
                ],
            ),
            (
                "alpha-first",
                vec![
                    GradientStop::new(alpha_red, Ratio(0.0)),
                    GradientStop::new(green, Ratio(0.37)),
                    GradientStop::new(blue, Ratio(1.0)),
                ],
            ),
            (
                "alpha-mid",
                vec![
                    GradientStop::new(red, Ratio(0.0)),
                    GradientStop::new(alpha_green, Ratio(0.37)),
                    GradientStop::new(blue, Ratio(1.0)),
                ],
            ),
            (
                "alpha-last",
                vec![
                    GradientStop::new(red, Ratio(0.0)),
                    GradientStop::new(green, Ratio(0.37)),
                    GradientStop::new(alpha_blue, Ratio(1.0)),
                ],
            ),
        ]
    }

    fn p1262_raw_premultiplied_srgb_error(left: Color, right: Color) -> f32 {
        raw_premultiplied_srgb_error(left, right)
    }

    #[test]
    fn p1262_oklab_saturado_recorta_antes_de_premultiplicar() {
        let (_, stops, _) = p1262_stopsets()[0].clone();
        let gradient = Gradient::linear_with_space(
            stops.clone(),
            Angle::deg(0.0),
            ColorSpace::Oklab,
        );
        let exact = sample_gradient_precise_for_svg(&gradient, 0.685);
        let approximate = mix_clamped_srgb_half(stops[1].color, stops[2].color);
        let (er, eg, eb, ea) = exact.to_rgba_f32();
        let (ar, ag, ab, aa) = approximate.to_rgba_f32();
        let expected = {
            let dr = er.clamp(0.0, 1.0) * ea - ar.clamp(0.0, 1.0) * aa;
            let dg = eg.clamp(0.0, 1.0) * ea - ag.clamp(0.0, 1.0) * aa;
            let db = eb.clamp(0.0, 1.0) * ea - ab.clamp(0.0, 1.0) * aa;
            (dr * dr + dg * dg + db * db).sqrt()
        };
        let measured = premultiplied_srgb_error(exact, approximate);
        let raw = p1262_raw_premultiplied_srgb_error(exact, approximate);
        assert_eq!(measured.to_bits(), expected.to_bits());
        assert_ne!(raw.to_bits(), expected.to_bits(), "witness must cross gamut");

        if std::env::var_os("P1262_DUMP").is_some() {
            for (name, stops, vanilla_count) in p1262_stopsets() {
                for (kind, gradient) in [
                    (
                        "linear",
                        Gradient::linear_with_space(
                            stops.clone(),
                            Angle::deg(0.0),
                            ColorSpace::Oklab,
                        ),
                    ),
                    (
                        "radial",
                        Gradient::radial_with_space(
                            stops,
                            Axes::new(Ratio(0.5), Ratio(0.5)),
                            Ratio(0.5),
                            ColorSpace::Oklab,
                        ),
                    ),
                ] {
                    let sampled = svg_adaptive_stops(&gradient);
                    let worst_t = match name {
                        "base" | "alpha-first" => 0.555_419_921_875_f64,
                        "coincident" => 0.292_236_328_125_f64,
                        "alpha-mid" => 0.554_199_218_75_f64,
                        _ => unreachable!(),
                    };
                    let exact = sample_gradient_precise_for_svg(&gradient, worst_t);
                    let (l, a, b, alpha) = match exact {
                        Color::Oklab { l, a, b, alpha } => (l, a, b, alpha),
                        _ => unreachable!("Oklab gradient must sample to Oklab"),
                    };
                    let (raw_r, raw_g, raw_b, raw_a) = exact.to_rgba_f32();
                    let (pre_r, pre_g, pre_b) = clamped_premultiplied_srgb(exact);
                    let (u8_r, u8_g, u8_b, u8_a) = exact.to_srgb();
                    eprintln!(
                        "P1262_STAGE\t{kind}\t{name}\t{worst_t:.17}\t{l:.9}\t{a:.9}\t{b:.9}\t{alpha:.9}\t{raw_r:.9}\t{raw_g:.9}\t{raw_b:.9}\t{raw_a:.9}\t{pre_r:.9}\t{pre_g:.9}\t{pre_b:.9}\t{u8_r}\t{u8_g}\t{u8_b}\t{u8_a}"
                    );
                    eprintln!(
                        "P1262_COUNT\t{kind}\t{name}\t{}\t{vanilla_count}",
                        sampled.len()
                    );
                    for (index, stop) in sampled.iter().enumerate() {
                        let (r, g, b, a) = stop.color.to_srgb();
                        eprintln!(
                            "P1262_STOP\t{kind}\t{name}\t{index}\t{:.17}\t{r}\t{g}\t{b}\t{a}",
                            stop.offset.expect("sampled offset").0
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn p1262_oito_stopsets_saturados_sao_materializados() {
        for (name, stops, expected) in p1262_stopsets() {
            let linear = Gradient::linear_with_space(
                stops.clone(),
                Angle::deg(0.0),
                ColorSpace::Oklab,
            );
            let radial = Gradient::radial_with_space(
                stops,
                Axes::new(Ratio(0.5), Ratio(0.5)),
                Ratio(0.5),
                ColorSpace::Oklab,
            );
            let linear_count = svg_adaptive_stops(&linear).len();
            let radial_count = svg_adaptive_stops(&radial).len();
            assert!(linear_count >= 3, "Linear/Oklab {name}");
            assert!(radial_count >= 3, "Radial/Oklab {name}");
            assert_eq!(linear_count, radial_count, "variant drift in {name}");
            assert!(
                linear_count <= expected,
                "P1262 must not exceed vanilla mechanical cost for {name}"
            );
            for (kind, gradient) in [("linear", linear), ("radial", radial)] {
                let sampled = svg_adaptive_stops(&gradient);
                let mut decisive = 0;
                for pair in sampled.windows(2) {
                    let t0 = pair[0].offset.expect("left offset").0;
                    let t1 = pair[1].offset.expect("right offset").0;
                    if t1 <= t0 {
                        continue;
                    }
                    let midpoint = t0 + (t1 - t0) * 0.5;
                    let exact = sample_gradient_precise_for_svg(&gradient, midpoint);
                    let approximate = mix_clamped_srgb_half(pair[0].color, pair[1].color);
                    let raw = p1262_raw_premultiplied_srgb_error(exact, approximate);
                    let clamped = premultiplied_srgb_error(exact, approximate);
                    if raw > 0.001 && clamped <= 0.001 {
                        decisive += 1;
                        if std::env::var_os("P1262_DUMP").is_some() {
                            let (l, a, b, alpha) = match exact {
                                Color::Oklab { l, a, b, alpha } => (l, a, b, alpha),
                                _ => unreachable!(),
                            };
                            let (er, eg, eb, ea) = exact.to_rgba_f32();
                            let (ar, ag, ab, aa) = approximate.to_rgba_f32();
                            let (ep_r, ep_g, ep_b) = clamped_premultiplied_srgb(exact);
                            let (ap_r, ap_g, ap_b) =
                                clamped_premultiplied_srgb(approximate);
                            eprintln!(
                                "P1262_DECISION\t{kind}\t{name}\t{midpoint:.17}\t{l:.9}\t{a:.9}\t{b:.9}\t{alpha:.9}\t{er:.9}\t{eg:.9}\t{eb:.9}\t{ea:.9}\t{ar:.9}\t{ag:.9}\t{ab:.9}\t{aa:.9}\t{ep_r:.9}\t{ep_g:.9}\t{ep_b:.9}\t{ap_r:.9}\t{ap_g:.9}\t{ap_b:.9}\t{raw:.9}\t{clamped:.9}"
                            );
                        }
                    }
                }
                assert!(
                    decisive > 0,
                    "{kind}/Oklab {name} must expose a raw-vs-clamped decision"
                );
            }
        }
    }

    #[test]
    fn p1262_p1237_full_dump_cobre_vinte_e_quatro_fixtures() {
        let mut variants = 0;
        for (space_name, space) in
            [("oklab", ColorSpace::Oklab), ("linear-rgb", ColorSpace::LinearRgb)]
        {
            for (stopset, stops) in p1262_full_stopsets() {
                for (kind, gradient) in [
                    (
                        "linear",
                        Gradient::linear_with_space(
                            stops.clone(),
                            Angle::deg(0.0),
                            space,
                        ),
                    ),
                    (
                        "radial",
                        Gradient::radial_with_space(
                            stops.clone(),
                            Axes::new(Ratio(0.5), Ratio(0.5)),
                            Ratio(0.5),
                            space,
                        ),
                    ),
                ] {
                    variants += 1;
                    let sampled = svg_adaptive_stops(&gradient);
                    assert!(sampled.len() >= 2, "{kind}/{space_name}/{stopset}");
                    if std::env::var_os("P1262_DUMP").is_some() {
                        eprintln!(
                            "P1262_FULL_COUNT\t{kind}\t{space_name}\t{stopset}\t{}",
                            sampled.len()
                        );
                        for (index, stop) in sampled.iter().enumerate() {
                            let (r, g, b, a) = stop.color.to_srgb();
                            eprintln!(
                                "P1262_FULL_STOP\t{kind}\t{space_name}\t{stopset}\t{index}\t{:.17}\t{r}\t{g}\t{b}\t{a}",
                                stop.offset.expect("sampled offset").0
                            );
                        }
                    }
                }
            }
        }
        assert_eq!(variants, 24);
    }
}
