//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/gradient/interpolation.md
//! @prompt-hash 1286a18f
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Interpolação linear em Oklab.
///
/// Converte ambas as cores para representação Oklab (via
/// to_rgba_f32 → linear path); interpola linearmente os componentes
/// L, a, b; retorna `Color::oklab` com alpha interpolado.
pub(super) fn interpolate_oklab(c0: Color, c1: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    let (l0, a0, b0, alpha0) = color_to_oklab_with_alpha(c0);
    let (l1, a1, b1, alpha1) = color_to_oklab_with_alpha(c1);
    let w0 = (1.0_f64 - t as f64) as f32;
    let w1 = t;
    let total = w0 + w1;
    Color::oklab(
        (w0 * l0 + w1 * l1) / total,
        (w0 * a0 + w1 * a1) / total,
        (w0 * b0 + w1 * b1) / total,
        (w0 * alpha0 + w1 * alpha1) / total,
    )
}

/// Converte qualquer Color para Oklab (L, a, b, alpha).
///
/// Para cores já em Oklab retorna campos directos; para outras
/// converte via sRGB → linear → Oklab (caminho inverso).
///
/// **P268.2**: promovido a `pub` para acessibilidade cross-crate
/// (L3 `oklab_delta_e` reutilização literal — ver anotação ADR-0089
/// §"Anotação cumulativa P268.2" + `diagnostico-adaptive-n-passo-268-2.md`
/// §A.1). Function body preservada literal P262.
/// **P272**: `oklab_delta_e` removed (ADR-0090 REVOGADO; adaptive N
/// não aplicável a Coons). `color_to_oklab_with_alpha` permanece
/// `pub` — usada por outros helpers L1/L3 (Oklab interpolation arms
/// L1 + multispace helpers L3).
pub fn color_to_oklab_with_alpha(c: Color) -> (f32, f32, f32, f32) {
    match c {
        Color::Oklab { l, a, b, alpha } => (l, a, b, alpha),
        _ => {
            // Converter via sRGB → linear → Oklab.
            let (r, g, b, alpha) = c.to_rgba_f32();
            let lin_r = srgb_to_linear(r);
            let lin_g = srgb_to_linear(g);
            let lin_b = srgb_to_linear(b);
            let (lab_l, lab_a, lab_b) = linear_rgb_to_oklab(lin_r, lin_g, lin_b);
            (lab_l, lab_a, lab_b, alpha)
        }
    }
}

/// Gamma 2.2 inversa (sRGB → linear).
pub(super) fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        (1.0_f64 / 12.92) as f32 * c
    } else {
        libm::powf(
            libm::fmaf(c, (1.0_f64 / 1.055) as f32, (0.055_f64 / 1.055) as f32),
            2.4,
        )
    }
}

/// Cube root bit-compatible with the `libm::cbrtf` used by palette 0.7.6.
pub(super) fn palette_cbrtf_p1253(x: f32) -> f32 {
    libm::cbrtf(x)
}

/// linear sRGB → Oklab (paridade ICC; constantes da publicação Björn Ottosson 2020).
pub(super) fn linear_rgb_to_oklab(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let l = 0.412_221_46 * r + 0.536_332_55 * g + 0.051_445_995 * b;
    let m = 0.211_903_5 * r + 0.680_699_5 * g + 0.107_396_96 * b;
    let s = 0.088_302_46 * r + 0.281_718_85 * g + 0.629_978_7 * b;

    let l_ = palette_cbrtf_p1253(l);
    let m_ = palette_cbrtf_p1253(m);
    let s_ = palette_cbrtf_p1253(s);

    let l_lab = 0.210_454_26 * l_ + 0.793_617_8 * m_ - 0.004_072_047 * s_;
    let a_lab = 1.977_998_5 * l_ - 2.428_592_2 * m_ + 0.450_593_7 * s_;
    let b_lab = 0.025_904_037 * l_ + 0.782_771_77 * m_ - 0.808_675_77 * s_;

    (l_lab, a_lab, b_lab)
}

// ── P270 — Multi-space interpolation helpers ────────────────────

/// Dispatcher central: interpola `c0` ↔ `c1` no espaço `space` em `t`.
///
/// **P270**: arm `Oklab` chama `interpolate_oklab` P262 literal
/// (preserva bytes P262/P264/P267 bit-exact). Demais 7 arms chamam
/// helpers per-space.
pub(super) fn interpolate_in_space(
    c0: Color,
    c1: Color,
    t: f32,
    space: ColorSpace,
) -> Color {
    let t = t.clamp(0.0, 1.0);
    match space {
        ColorSpace::Spot(colorant) => match (c0, c1) {
            (Color::Spot(a), Color::Spot(b)) => {
                if a.colorant == b.colorant && a.colorant.as_ref() == &colorant {
                    Color::Spot(crate::entities::color::SpotColor {
                        colorant: a.colorant,
                        tint: Ratio(
                            a.tint.get() + (b.tint.get() - a.tint.get()) * t as f64,
                        ),
                    })
                } else {
                    interpolate_oklab(
                        Color::Spot(a).to_process(),
                        Color::Spot(b).to_process(),
                        t,
                    )
                }
            }
            (a, b) => interpolate_oklab(a.to_process(), b.to_process(), t),
        },
        ColorSpace::Oklab => interpolate_oklab(c0, c1, t),
        ColorSpace::Oklch => interpolate_oklch(c0, c1, t),
        ColorSpace::Srgb => interpolate_srgb(c0, c1, t),
        ColorSpace::Luma => interpolate_luma(c0, c1, t),
        ColorSpace::LinearRgb => interpolate_linear_rgb(c0, c1, t),
        ColorSpace::Hsl => interpolate_hsl(c0, c1, t),
        ColorSpace::Hsv => interpolate_hsv(c0, c1, t),
        ColorSpace::Cmyk => interpolate_cmyk(c0, c1, t),
    }
}

/// Hue interpolation com wrap shorter (CSS standard; vanilla paridade
/// literal portada de `mix_iter` linha 1126-1136).
pub(super) fn interpolate_hue_shorter(h0: f32, h1: f32, t: f32) -> f32 {
    let diff = h1 - h0;
    let wrapped_h1 = if diff.abs() > 180.0 {
        if diff > 0.0 {
            h1 - 360.0
        } else {
            h1 + 360.0
        }
    } else {
        h1
    };
    (h0 + (wrapped_h1 - h0) * t).rem_euclid(360.0)
}

/// Lerp linear simples.
#[inline]
pub(super) fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// sRGB → linear (gamma 2.2 inversa). Espelho privado de `srgb_to_linear`
/// já existente, mantido aqui por encapsulamento — helper P270.
#[inline]
pub(super) fn srgb_to_linear_local(c: f32) -> f32 {
    if c <= 0.04045 {
        (1.0_f64 / 12.92) as f32 * c
    } else {
        libm::powf(
            libm::fmaf(c, (1.0_f64 / 1.055) as f32, (0.055_f64 / 1.055) as f32),
            2.4,
        )
    }
}

/// sRGB componentes (extracted; lossless para `Color::Srgb`).
pub(super) fn to_srgb_components(c: Color) -> (f32, f32, f32, f32) {
    c.to_rgba_f32() // todas variants convertem para sRGB nativo
}

/// Linear RGB componentes (extracted; via sRGB → linear inverso).
pub(super) fn to_linear_rgb_components(c: Color) -> (f32, f32, f32, f32) {
    match c {
        Color::LinearRgb { r, g, b, a } => (r, g, b, a),
        _ => {
            let (r, g, b, a) = c.to_rgba_f32();
            (srgb_to_linear_local(r), srgb_to_linear_local(g), srgb_to_linear_local(b), a)
        }
    }
}

/// Luma componente (Rec 709 luminance) + alpha.
pub(super) fn to_luma_components(c: Color) -> (f32, f32) {
    match c {
        Color::Luma { l, a } => (l, a),
        _ => {
            let (r, g, b, a) = c.to_rgba_f32();
            // Rec 709 luminance (paridade cristalino `Color::Luma {l, l, l, a}` to_rgba).
            let l = 0.2126 * r + 0.7152 * g + 0.0722 * b;
            (l, a)
        }
    }
}

/// Oklch componentes (l, c, h, alpha) — via Oklab cartesiano → polar.
pub(super) fn to_oklch_components(c: Color) -> (f32, f32, f32, f32) {
    match c {
        Color::Oklch { l, c, h, alpha } => (l, c, h, alpha),
        _ => {
            let (l, a, b, alpha) = color_to_oklab_with_alpha(c);
            let chroma = (a * a + b * b).sqrt();
            let h = (std::f32::consts::PI + libm::atan2f(-b, -a))
                .to_degrees()
                .rem_euclid(360.0);
            (l, chroma, h, alpha)
        }
    }
}

/// HSL componentes (h, s, l, alpha).
pub(super) fn to_hsl_components(c: Color) -> (f32, f32, f32, f32) {
    match c {
        Color::Hsl { h, s, l, a } => (h, s, l, a),
        _ => {
            let (r, g, b, a) = c.to_rgba_f32();
            let max = r.max(g).max(b);
            let min = r.min(g).min(b);
            let delta = max - min;
            let l = (max + min) * 0.5;
            let s = if delta == 0.0 {
                0.0
            } else {
                delta / (1.0 - (2.0 * l - 1.0).abs()).max(1e-6)
            };
            let h = if delta == 0.0 {
                0.0
            } else if max == r {
                60.0 * (((g - b) / delta).rem_euclid(6.0))
            } else if max == g {
                60.0 * ((b - r) / delta + 2.0)
            } else {
                60.0 * ((r - g) / delta + 4.0)
            };
            (h.rem_euclid(360.0), s.clamp(0.0, 1.0), l.clamp(0.0, 1.0), a)
        }
    }
}

/// HSV componentes (h, s, v, alpha).
pub(super) fn to_hsv_components(c: Color) -> (f32, f32, f32, f32) {
    match c {
        Color::Hsv { h, s, v, a } => (h, s, v, a),
        _ => {
            let (r, g, b, a) = c.to_rgba_f32();
            let max = r.max(g).max(b);
            let min = r.min(g).min(b);
            let delta = max - min;
            let v = max;
            let s = if max == 0.0 { 0.0 } else { delta / max };
            let h = if delta == 0.0 {
                0.0
            } else if max == r {
                60.0 * (((g - b) / delta).rem_euclid(6.0))
            } else if max == g {
                60.0 * ((b - r) / delta + 2.0)
            } else {
                60.0 * ((r - g) / delta + 4.0)
            };
            (h.rem_euclid(360.0), s.clamp(0.0, 1.0), v.clamp(0.0, 1.0), a)
        }
    }
}

/// CMYK componentes (c, m, y, k).
pub(super) fn to_cmyk_components(c: Color) -> (f32, f32, f32, f32) {
    match c {
        Color::Cmyk { c, m, y, k } => (c, m, y, k),
        _ => {
            let (r, g, b, _) = c.to_rgba_f32();
            let k = 1.0 - r.max(g).max(b);
            if k >= 1.0 - 1e-6 {
                (0.0, 0.0, 0.0, 1.0)
            } else {
                let denom = 1.0 - k;
                let cc = (1.0 - r - k) / denom;
                let mm = (1.0 - g - k) / denom;
                let yy = (1.0 - b - k) / denom;
                (
                    cc.clamp(0.0, 1.0),
                    mm.clamp(0.0, 1.0),
                    yy.clamp(0.0, 1.0),
                    k.clamp(0.0, 1.0),
                )
            }
        }
    }
}

/// Interpolação sRGB componentwise.
pub(super) fn interpolate_srgb(c0: Color, c1: Color, t: f32) -> Color {
    let (r0, g0, b0, a0) = to_srgb_components(c0);
    let (r1, g1, b1, a1) = to_srgb_components(c1);
    Color::srgb_f32(lerp(r0, r1, t), lerp(g0, g1, t), lerp(b0, b1, t), lerp(a0, a1, t))
}

/// Interpolação linear RGB componentwise.
pub(super) fn interpolate_linear_rgb(c0: Color, c1: Color, t: f32) -> Color {
    let (r0, g0, b0, a0) = to_linear_rgb_components(c0);
    let (r1, g1, b1, a1) = to_linear_rgb_components(c1);
    Color::linear_rgb(lerp(r0, r1, t), lerp(g0, g1, t), lerp(b0, b1, t), lerp(a0, a1, t))
}

/// Interpolação Luma (grayscale; alpha preservado).
pub(super) fn interpolate_luma(c0: Color, c1: Color, t: f32) -> Color {
    let (l0, a0) = to_luma_components(c0);
    let (l1, a1) = to_luma_components(c1);
    // Color::luma só aceita l (alpha=1.0 implícito); construct via Color::Luma directo.
    Color::Luma { l: lerp(l0, l1, t), a: lerp(a0, a1, t) }
}

/// Interpolação Oklch (polar; hue-wrap shorter).
pub(super) fn interpolate_oklch(c0: Color, c1: Color, t: f32) -> Color {
    let (l0, ch0, h0, a0) = to_oklch_components(c0);
    let (l1, ch1, h1, a1) = to_oklch_components(c1);
    Color::oklch(
        lerp(l0, l1, t),
        lerp(ch0, ch1, t),
        interpolate_hue_shorter(h0, h1, t),
        lerp(a0, a1, t),
    )
}

/// Interpolação HSL (polar; hue-wrap shorter).
pub(super) fn interpolate_hsl(c0: Color, c1: Color, t: f32) -> Color {
    let (h0, s0, l0, a0) = to_hsl_components(c0);
    let (h1, s1, l1, a1) = to_hsl_components(c1);
    Color::hsl(
        interpolate_hue_shorter(h0, h1, t),
        lerp(s0, s1, t),
        lerp(l0, l1, t),
        lerp(a0, a1, t),
    )
}

/// Interpolação HSV (polar; hue-wrap shorter).
pub(super) fn interpolate_hsv(c0: Color, c1: Color, t: f32) -> Color {
    let (h0, s0, v0, a0) = to_hsv_components(c0);
    let (h1, s1, v1, a1) = to_hsv_components(c1);
    Color::hsv(
        interpolate_hue_shorter(h0, h1, t),
        lerp(s0, s1, t),
        lerp(v0, v1, t),
        lerp(a0, a1, t),
    )
}

/// Interpolação CMYK componentwise.
pub(super) fn interpolate_cmyk(c0: Color, c1: Color, t: f32) -> Color {
    let (c0_, m0, y0, k0) = to_cmyk_components(c0);
    let (c1_, m1, y1, k1) = to_cmyk_components(c1);
    Color::cmyk(lerp(c0_, c1_, t), lerp(m0, m1, t), lerp(y0, y1, t), lerp(k0, k1, t))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_ci_gradient_polar_quadrants() {
        for (a, b, degrees) in [
            (0.1, 0.1, 45.0),
            (-0.1, 0.1, 135.0),
            (-0.1, -0.1, 225.0),
            (0.1, -0.1, 315.0),
        ] {
            let (l, c, h, alpha) = to_oklch_components(Color::oklab(0.5, a, b, 0.25));
            assert!((l - 0.5).abs() < 1e-6);
            assert!((c - 0.141_421_36).abs() < 1e-6);
            assert!((h - degrees).abs() < 1e-4);
            assert!((alpha - 0.25).abs() < 1e-6);
        }
    }

    #[test]
    fn colors_ci_gradient_shorter_hue_and_alpha() {
        let result = interpolate_oklch(
            Color::oklch(0.5, 0.1, 350.0, 0.2),
            Color::oklch(0.5, 0.1, 10.0, 0.8),
            0.5,
        );
        let Color::Oklch { l, c, h, alpha } = result else {
            panic!("Oklch result required");
        };
        assert!((l - 0.5).abs() < 1e-6);
        assert!((c - 0.1).abs() < 1e-6);
        assert!(h.abs() < 1e-4 || (h - 360.0).abs() < 1e-4);
        assert!((alpha - 0.5).abs() < 1e-6);
    }

    #[test]
    fn bounds_ci_gradient_preserves_measured_hue() {
        let (_, _, hue, _) =
            to_oklch_components(Color::srgb_f32(1.0, 0.254902, 0.211765, 1.0));
        assert!((hue - 28.442_533).abs() < 1e-6, "hue {hue}");
    }
}
