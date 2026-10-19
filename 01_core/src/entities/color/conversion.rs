//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/color/conversion.md
//! @prompt-hash 47c58360
//! @layer L1
//! @updated 2026-09-30
//!
//! Conversões entre espaços cromáticos e representação sRGB.

use super::*;

impl Color {
    // ── Conversões ───────────────────────────────────────────

    /// Converte para sRGB byte `(r, g, b, a)` em [0, 255].
    /// Consumer principal: PDF exporter (4 caminhos
    /// `to_rgba_f32` cumulativos).
    /// Converte float em [0.0, 1.0] para u8 em [0, 255] com arredondamento
    /// "round ties to even" (paridade palette crate / Vanilla Typst, Hacker's Delight).
    #[inline]
    pub fn f32_to_u8_ties_even(val: f32) -> u8 {
        let scaled = (val.clamp(0.0, 1.0) * 255.0).min(255.0);
        const C23: u32 = 0x4b00_0000;
        let f = scaled + f32::from_bits(C23);
        (f.to_bits().saturating_sub(C23)) as u8
    }

    pub fn to_srgb(&self) -> (u8, u8, u8, u8) {
        let (r, g, b, a) = self.to_rgba_f32();
        (
            Self::f32_to_u8_ties_even(r),
            Self::f32_to_u8_ties_even(g),
            Self::f32_to_u8_ties_even(b),
            Self::f32_to_u8_ties_even(a),
        )
    }

    /// Converte para sRGB normalizado `(r, g, b, a)` em [0.0, 1.0].
    /// Preservado para compatibilidade hot path PDF exporter
    /// cristalino existente.
    pub fn to_rgba_f32(&self) -> (f32, f32, f32, f32) {
        match self.clone() {
            Self::Srgb { r, g, b, a } => (r, g, b, a),
            Self::Luma { l, a } => (l, l, l, a),
            Self::LinearRgb { r, g, b, a } => {
                // Gamma 2.2 inversa (linear → sRGB).
                (linear_to_srgb(r), linear_to_srgb(g), linear_to_srgb(b), a)
            }
            Self::Oklab { l, a, b, alpha } => {
                let (lin_r, lin_g, lin_b) = oklab_to_linear_rgb(l, a, b);
                (
                    linear_to_srgb(lin_r),
                    linear_to_srgb(lin_g),
                    linear_to_srgb(lin_b),
                    alpha,
                )
            }
            Self::Oklch { l, c, h, alpha } => {
                let (lin_r, lin_g, lin_b) = oklch_to_linear_rgb(l, c, h);
                (
                    linear_to_srgb(lin_r),
                    linear_to_srgb(lin_g),
                    linear_to_srgb(lin_b),
                    alpha,
                )
            }
            Self::Hsl { h, s, l, a } => {
                let (r, g, b) = hsl_to_rgb(h, s, l);
                (r, g, b, a)
            }
            Self::Hsv { h, s, v, a } => {
                let (r, g, b) = hsv_to_rgb(h, s, v);
                (r, g, b, a)
            }
            Self::Cmyk { c, m, y, k } => {
                // CMY → RGB: r=(1-c)(1-k), g=(1-m)(1-k), b=(1-y)(1-k).
                let r = (1.0 - c) * (1.0 - k);
                let g = (1.0 - m) * (1.0 - k);
                let b = (1.0 - y) * (1.0 - k);
                (r, g, b, 1.0)
            }
            Self::Spot(spot) => spot.to_process().to_rgba_f32(),
        }
    }
}

// ── Helpers conversões ──────────────────────────────────────

/// Linear RGB → sRGB (gamma encoding).
pub(super) fn linear_to_srgb(c: f32) -> f32 {
    if c <= 0.0031308 {
        12.92 * c
    } else {
        libm::powf(c, (1.0_f64 / 2.4) as f32) * 1.055 - 0.055
    }
}

/// Oklab → Linear RGB (matriz LMS).
/// Algoritmo: Björn Ottosson <https://bottosson.github.io/posts/oklab/>.
pub(super) fn oklab_to_linear_rgb(l: f32, a: f32, b: f32) -> (f32, f32, f32) {
    let l_ = l + 0.396_337_78 * a + 0.215_803_76 * b;
    let m_ = l - 0.105_561_346 * a - 0.063_854_17 * b;
    let s_ = l - 0.089_484_18 * a - 1.291_485_5 * b;

    let l_3 = l_ * l_ * l_;
    let m_3 = m_ * m_ * m_;
    let s_3 = s_ * s_ * s_;

    let r = 4.076_741_7 * l_3 - 3.307_711_6 * m_3 + 0.230_969_94 * s_3;
    let g = -1.268_438 * l_3 + 2.609_757_4 * m_3 - 0.341_319_38 * s_3;
    let b = -0.0041960863 * l_3 - 0.703_418_6 * m_3 + 1.707_614_7 * s_3;
    (r, g, b)
}

/// Polar conversion uses nonnegative chroma, as in palette's Oklch conversion.
fn oklch_to_linear_rgb(l: f32, c: f32, h: f32) -> (f32, f32, f32) {
    let (sin, cos) = libm::sincosf(h.to_radians());
    let chroma = c.max(0.0);
    oklab_to_linear_rgb(l, cos * chroma, sin * chroma)
}

/// HSL → RGB (h em graus; s, l normalizados).
pub(super) fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let h_prime = (h.rem_euclid(360.0)) / 60.0;
    let x = c * (1.0 - (h_prime.rem_euclid(2.0) - 1.0).abs());
    let m = l - c / 2.0;
    let (r1, g1, b1) = if (0.0..1.0).contains(&h_prime) {
        (c, x, 0.0)
    } else if (1.0..2.0).contains(&h_prime) {
        (x, c, 0.0)
    } else if (2.0..3.0).contains(&h_prime) {
        (0.0, c, x)
    } else if (3.0..4.0).contains(&h_prime) {
        (0.0, x, c)
    } else if (4.0..5.0).contains(&h_prime) {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (r1 + m, g1 + m, b1 + m)
}

/// HSV → RGB (h em graus; s, v normalizados).
pub(super) fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let c = v * s;
    let h_prime = (h.rem_euclid(360.0)) / 60.0;
    let x = c * (1.0 - (h_prime.rem_euclid(2.0) - 1.0).abs());
    let m = v - c;
    let (r1, g1, b1) = if (0.0..1.0).contains(&h_prime) {
        (c, x, 0.0)
    } else if (1.0..2.0).contains(&h_prime) {
        (x, c, 0.0)
    } else if (2.0..3.0).contains(&h_prime) {
        (0.0, c, x)
    } else if (3.0..4.0).contains(&h_prime) {
        (0.0, x, c)
    } else if (4.0..5.0).contains(&h_prime) {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (r1 + m, g1 + m, b1 + m)
}

/// sRGB → linear RGB (inverso de gamma 2.2). Duplicado de `gradient::srgb_to_linear`.
pub(super) fn srgb_to_linear_p476(c: f32) -> f32 {
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

/// linear sRGB → Oklab. Duplicado de `gradient::linear_rgb_to_oklab` (P270).
pub(super) fn linear_rgb_to_oklab_p476(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let l = 0.412_221_46 * r + 0.536_332_55 * g + 0.051_445_995 * b;
    let m = 0.211_903_5 * r + 0.680_699_5 * g + 0.107_396_96 * b;
    let s = 0.088_302_46 * r + 0.281_718_85 * g + 0.629_978_7 * b;
    let l_ = palette_cbrtf_p1253(l);
    let m_ = palette_cbrtf_p1253(m);
    let s_ = palette_cbrtf_p1253(s);
    (
        0.210_454_26 * l_ + 0.793_617_8 * m_ - 0.004_072_047 * s_,
        1.977_998_5 * l_ - 2.428_592_2 * m_ + 0.450_593_7 * s_,
        0.025_904_037 * l_ + 0.782_771_77 * m_ - 0.808_675_77 * s_,
    )
}

/// D65 Luma → Oklab pelo despacho e pelas matrizes do `palette 0.7.6`.
///
/// Luma não é sRGB com três canais duplicados para fins de conversão:
/// `palette` passa por XYZ D65 antes de Oklab (P1271-C2).
pub(super) fn luma_to_oklab_p1271(luma: f32) -> (f32, f32, f32) {
    let linear = srgb_to_linear_p476(luma);
    let x = 0.95047_f32 * linear;
    let y = linear;
    let z = 1.08883_f32 * linear;

    let l = 0.819_022_4_f32 * x + 0.361_906_26_f32 * y + -0.128_873_78_f32 * z;
    let m = 0.032_983_67_f32 * x + 0.929_286_84_f32 * y + 0.036_144_666_f32 * z;
    let s = 0.048_177_2_f32 * x + 0.264_239_52_f32 * y + 0.633_547_84_f32 * z;
    let l_ = palette_cbrtf_p1253(l);
    let m_ = palette_cbrtf_p1253(m);
    let s_ = palette_cbrtf_p1253(s);

    (
        (0.210_454_26_f32 * l_ + 0.793_617_8_f32 * m_ + -0.004_072_047_f32 * s_)
            .clamp(0.0, 1.0),
        1.977_998_5_f32 * l_ + -2.428_592_2_f32 * m_ + 0.450_593_7_f32 * s_,
        0.025_904_037_f32 * l_ + 0.782_771_77_f32 * m_ + -0.808_675_77_f32 * s_,
    )
}

/// Qualquer Color → Oklab (l, a, b, alpha). Base de `mix` e `to_oklch_p476`.
pub(super) fn to_oklab_p476(c: Color) -> (f32, f32, f32, f32) {
    match c {
        Color::Oklab { l, a, b, alpha } => (l, a, b, alpha),
        Color::Luma { l, a } => {
            let (lab_l, lab_a, lab_b) = luma_to_oklab_p1271(l);
            (lab_l, lab_a, lab_b, a)
        }
        Color::Srgb { .. }
        | Color::LinearRgb { .. }
        | Color::Oklch { .. }
        | Color::Hsl { .. }
        | Color::Hsv { .. }
        | Color::Cmyk { .. }
        | Color::Spot(_) => {
            let (r, g, b_c, alpha) = c.to_rgba_f32();
            let (lab_l, lab_a, lab_b) = linear_rgb_to_oklab_p476(
                srgb_to_linear_p476(r),
                srgb_to_linear_p476(g),
                srgb_to_linear_p476(b_c),
            );
            (lab_l, lab_a, lab_b, alpha)
        }
    }
}

/// Qualquer Color → Oklch (l, c, h, alpha). Base de `rotate` e testes.
pub(super) fn to_oklch_p476(c: Color) -> (f32, f32, f32, f32) {
    match c {
        Color::Oklch { l, c, h, alpha } => (l, c, h, alpha),
        Color::Srgb { .. }
        | Color::Luma { .. }
        | Color::LinearRgb { .. }
        | Color::Oklab { .. }
        | Color::Hsl { .. }
        | Color::Hsv { .. }
        | Color::Cmyk { .. }
        | Color::Spot(_) => {
            let (l, a, b, alpha) = to_oklab_p476(c);
            let chroma = (a * a + b * b).sqrt();
            let h = (std::f32::consts::PI + libm::atan2f(-b, -a))
                .to_degrees()
                .rem_euclid(360.0);
            (l, chroma, h, alpha)
        }
    }
}

/// **P742** — sRGB → HSV (hexcone standard = palette
/// `hsv.rs:274-310`, com guarda de negativos e hue `rem_euclid`).
pub(super) fn rgb_to_hsv_p742(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let r = r.max(0.0);
    let g = g.max(0.0);
    let b = b.max(0.0);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let d = max - min;
    if d == 0.0 || max == 0.0 {
        return (0.0, 0.0, max);
    }
    let s = d / max;
    let h = hue_hexcone_p742(r, g, b, max, d);
    (h, s, max)
}

/// **P742** — sRGB → HSL (hexcone standard; mesmo hue de HSV).
pub(super) fn rgb_to_hsl_p742(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let r = r.max(0.0);
    let g = g.max(0.0);
    let b = b.max(0.0);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let d = max - min;
    let l = (max + min) / 2.0;
    if d == 0.0 {
        return (0.0, 0.0, l);
    }
    let s = d / (1.0 - (2.0 * l - 1.0).abs());
    let h = hue_hexcone_p742(r, g, b, max, d);
    (h, s, l)
}

/// Hue hexcone em graus (partilhado por HSV/HSL — paridade palette).
pub(super) fn hue_hexcone_p742(r: f32, g: f32, b: f32, max: f32, d: f32) -> f32 {
    let h = if max == r {
        ((g - b) / d).rem_euclid(6.0)
    } else if max == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    } * 60.0;
    h.rem_euclid(360.0)
}

/// **P742** — sRGB → Luma (palette: luminância linear Rec.709 re-codificada
/// com a curva sRGB).
pub(super) fn srgb_to_luma_p742(r: f32, g: f32, b: f32) -> f32 {
    // P1239: linha Y da matriz sRGB de palette 0.7.6. Os coeficientes
    // abreviados alteram o observável de vermelho de 54.02% para 54.01%.
    let y = 0.2126729 * srgb_to_linear_p476(r)
        + 0.7151522 * srgb_to_linear_p476(g)
        + 0.0721750 * srgb_to_linear_p476(b);
    linear_to_srgb(y)
}

impl Color {
    /// Espaço da cor (mapeamento 1:1 variante → `ColorSpace`; paridade
    /// vanilla `ProcessColor::space`, `color.rs:1535`).
    pub fn space(&self) -> ColorSpace {
        match self {
            Color::Srgb { .. } => ColorSpace::Srgb,
            Color::Luma { .. } => ColorSpace::Luma,
            Color::LinearRgb { .. } => ColorSpace::LinearRgb,
            Color::Oklab { .. } => ColorSpace::Oklab,
            Color::Oklch { .. } => ColorSpace::Oklch,
            Color::Hsl { .. } => ColorSpace::Hsl,
            Color::Hsv { .. } => ColorSpace::Hsv,
            Color::Cmyk { .. } => ColorSpace::Cmyk,
            Color::Spot(spot) => ColorSpace::Spot((*spot.colorant).clone()),
        }
    }

    /// **P742** — conversão para outro espaço. Hub sRGB: `to_rgba_f32` +
    /// conversão para o destino. Cmyk naive (o ICC do vanilla é scope-out
    /// ADR-0083). Mesmo espaço → identidade (paridade vanilla `to_space`).
    /// Converte preservando identidade spot; rejeita colorants incompatíveis.
    pub fn to_space(self, target: ColorSpace) -> Result<Color, ColorSpaceError> {
        if let ColorSpace::Spot(target_colorant) = &target {
            return match self {
                Color::Spot(spot) if spot.colorant.as_ref() == target_colorant => {
                    Ok(Color::Spot(spot))
                }
                Color::Spot(spot) => Err(ColorSpaceError::SpotToOther {
                    source: (*spot.colorant).clone(),
                    target: target_colorant.clone(),
                }),
                Color::Srgb { .. }
                | Color::Luma { .. }
                | Color::LinearRgb { .. }
                | Color::Oklab { .. }
                | Color::Oklch { .. }
                | Color::Hsl { .. }
                | Color::Hsv { .. }
                | Color::Cmyk { .. } => Err(ColorSpaceError::ProcessToSpot {
                    target: target_colorant.clone(),
                }),
            };
        }
        let process = self.to_process();
        if process.space() == target {
            return Ok(process);
        }
        let (r, g, b, a) = process.to_rgba_f32();
        Ok(match target {
            ColorSpace::Oklab => {
                let (l, a, b, alpha) = to_oklab_p476(process);
                Color::oklab(l, a, b, alpha)
            }
            ColorSpace::Oklch => {
                let (l, c, h, alpha) = to_oklch_p476(process);
                Color::oklch(l, c, h, alpha)
            }
            ColorSpace::Srgb => {
                // Upstream preserves the raw transfer from LinearRgb; other
                // conversions use FromColor and clamp the destination channels.
                if matches!(process, Color::LinearRgb { .. }) {
                    Color::srgb_f32(r, g, b, a)
                } else {
                    Color::srgb_f32(
                        r.clamp(0.0, 1.0),
                        g.clamp(0.0, 1.0),
                        b.clamp(0.0, 1.0),
                        a.clamp(0.0, 1.0),
                    )
                }
            }
            ColorSpace::Luma => Color::Luma { l: srgb_to_luma_p742(r, g, b), a },
            ColorSpace::LinearRgb => {
                let (r, g, b) = match process {
                    Color::Oklab { l, a, b, .. } => oklab_to_linear_rgb(l, a, b),
                    Color::Oklch { l, c, h, .. } => oklch_to_linear_rgb(l, c, h),
                    Color::Srgb { .. }
                    | Color::Luma { .. }
                    | Color::LinearRgb { .. }
                    | Color::Hsl { .. }
                    | Color::Hsv { .. }
                    | Color::Cmyk { .. }
                    | Color::Spot(_) => (
                        srgb_to_linear_p476(r),
                        srgb_to_linear_p476(g),
                        srgb_to_linear_p476(b),
                    ),
                };
                Color::linear_rgb(
                    r.clamp(0.0, 1.0),
                    g.clamp(0.0, 1.0),
                    b.clamp(0.0, 1.0),
                    a.clamp(0.0, 1.0),
                )
            }
            ColorSpace::Hsv => {
                let (h, s, v) = rgb_to_hsv_p742(r, g, b);
                Color::hsv(h, s, v, a)
            }
            ColorSpace::Hsl => {
                let (h, s, l) = rgb_to_hsl_p742(r, g, b);
                Color::hsl(h, s, l, a)
            }
            ColorSpace::Cmyk => {
                let k = 1.0 - r.max(g).max(b);
                if k >= 1.0 {
                    Color::cmyk(0.0, 0.0, 0.0, 1.0)
                } else {
                    Color::cmyk(
                        (1.0 - r - k) / (1.0 - k),
                        (1.0 - g - k) / (1.0 - k),
                        (1.0 - b - k) / (1.0 - k),
                        k,
                    )
                }
            }
            ColorSpace::Spot(target) => {
                return Err(ColorSpaceError::ProcessToSpot { target });
            }
        })
    }

    /// Projeção explícita para consumidores que só aceitam processo.
    pub fn to_process(&self) -> Color {
        match self {
            Color::Spot(spot) => spot.to_process(),
            Color::Srgb { .. }
            | Color::Luma { .. }
            | Color::LinearRgb { .. }
            | Color::Oklab { .. }
            | Color::Oklch { .. }
            | Color::Hsl { .. }
            | Color::Hsv { .. }
            | Color::Cmyk { .. } => self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_ci_oklch_to_rgb_observed_colors_and_alpha() {
        // Independent pinned-vanilla observations; no candidate-derived oracle.
        for (hue, alpha, expected) in
            [(30.0, 1.0, "#944b40"), (135.0, 0.5, "#49703280"), (270.0, 1.0, "#4d5f9c")]
        {
            assert_eq!(Color::oklch(0.5, 0.1, hue, alpha).to_hex(), expected);
        }
    }

    #[test]
    fn colors_ci_oklab_polar_quadrants_and_alpha() {
        for (a, b, degrees) in [
            (0.1, 0.1, 45.0),
            (-0.1, 0.1, 135.0),
            (-0.1, -0.1, 225.0),
            (0.1, -0.1, 315.0),
        ] {
            let (l, c, h, alpha) = to_oklch_p476(Color::oklab(0.5, a, b, 0.25));
            assert!((l - 0.5).abs() < 1e-6);
            assert!((c - 0.141_421_36).abs() < 1e-6);
            assert!((h - degrees).abs() < 1e-4);
            assert!((alpha - 0.25).abs() < 1e-6);
        }
    }

    #[test]
    fn bounds_ci_rotate_returns_bounded_rgb_components() {
        let named_red = Color::srgb_f32(1.0, 0.254902, 0.211765, 1.0);
        let (_, _, hue, _) = to_oklch_p476(named_red.clone());
        assert!((hue - 28.442_533).abs() < 1e-6, "hue {hue}");
        let color = named_red.rotate(45.0, None).unwrap();
        let (r, g, b, alpha) = color.to_rgba_f32();
        // Pinned vanilla public components, before conversion to bytes.
        assert!((r - 0.880_720_9).abs() < 1e-7, "red {r}");
        assert!((g - 0.442_628_47).abs() < 1e-7, "green {g}");
        assert_eq!(b, 0.0);
        assert_eq!(alpha, 1.0);
        let (l, c, h, _) = to_oklch_p476(color);
        assert!((l - 0.6663).abs() < 0.0001);
        assert!((c - 0.168_215_22).abs() < 1e-7);
        assert!((h - 53.42).abs() < 0.01);
    }

    #[test]
    fn bounds_ci_linear_target_clamps_without_gamma_roundtrip() {
        let Color::LinearRgb { r, g, b, a } = Color::oklch(0.5, 0.4, 30.0, 0.4)
            .to_space(ColorSpace::LinearRgb)
            .unwrap()
        else {
            panic!("LinearRgb expected");
        };
        assert!((r - 0.983_853_3).abs() < 1e-7, "red {r}");
        assert_eq!(g, 0.0);
        assert_eq!(b, 0.0);
        assert_eq!(a, 0.4);
    }

    #[test]
    fn bounds_ci_negative_chroma_converts_as_achromatic() {
        let color = Color::oklch(0.5, -0.1, 30.0, 1.0);
        assert_eq!(color.to_space(ColorSpace::Srgb).unwrap().to_hex(), "#636363");
    }

    #[test]
    fn bounds_ci_linear_to_srgb_transfer_preserves_raw_range() {
        let color = Color::linear_rgb(2.0, -0.5, 0.5, 0.4)
            .to_space(ColorSpace::Srgb)
            .unwrap();
        let (r, g, b, a) = color.to_rgba_f32();
        assert!(r > 1.0 && g < 0.0 && (0.0..=1.0).contains(&b));
        assert_eq!(a, 0.4);
    }
}
