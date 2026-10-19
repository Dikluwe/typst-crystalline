//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/gradient/radial.md
//! @prompt-hash fdb066a8
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Radial gradient — paridade vanilla `RadialGradient` subset
/// (per ADR-0088 P264).
///
/// **3 campos materializados**: stops + center + radius.
/// Scope-outs (per ADR-0088): focal_center, focal_radius,
/// space (Oklab fixo), relative (bbox-local), anti_alias (true).
///
/// **PDF render Radial fallback Solid** (first_stop_color) até
/// P265 dedicado materializar `/ShadingType 3` real.
#[derive(Debug, Clone, PartialEq)]
pub struct Radial {
    pub stops: Arc<[GradientStop]>,
    pub center: crate::entities::axes::Axes<Ratio>,
    pub radius: Ratio,
    /// P269 — focal_center activado per ADR-0088 §"Anotação cumulativa P269".
    /// Default via construtor `Gradient::radial(...)` = `center`.
    pub focal_center: crate::entities::axes::Axes<Ratio>,
    /// P269 — focal_radius activado per ADR-0088 §"Anotação cumulativa P269".
    /// Default via construtor `Gradient::radial(...)` = `Ratio(0.0)`.
    pub focal_radius: Ratio,
    /// P270 — ColorSpace runtime activado per ADR-0091 EM VIGOR.
    /// Default via construtor `Gradient::radial(...)` = `ColorSpace::Oklab`.
    pub space: ColorSpace,
    /// P273 — `RelativeTo` cross-variant runtime field. `None` = Auto =
    /// `Self_` default; preserva P264/P265/P269 bit-exact.
    pub relative: Option<RelativeTo>,
    /// P1145 — constructors normais usam true; `gradient.sharp` usa false.
    pub anti_alias: bool,
}

impl Radial {
    /// Auto-spacing paridade `Linear::effective_offsets` (P262).
    pub fn effective_offsets(&self) -> Vec<f32> {
        let n = self.stops.len();
        if n == 0 {
            return Vec::new();
        }
        if n == 1 {
            return vec![self.stops[0].offset.map(|r| r.0 as f32).unwrap_or(0.0)];
        }

        let mut offs: Vec<Option<f32>> =
            self.stops.iter().map(|s| s.offset.map(|r| r.0 as f32)).collect();

        if offs[0].is_none() {
            offs[0] = Some(0.0);
        }
        if offs[n - 1].is_none() {
            offs[n - 1] = Some(1.0);
        }

        let mut result = vec![0.0_f32; n];
        let mut i = 0;
        while i < n {
            if let Some(v) = offs[i] {
                result[i] = v;
                i += 1;
                continue;
            }
            let mut j = i;
            while j < n && offs[j].is_none() {
                j += 1;
            }
            let prev = result[i - 1];
            let next = offs[j].unwrap();
            let gap = j - i + 1;
            for k in 0..gap {
                result[i + k] = prev + (next - prev) * ((k + 1) as f32) / (gap as f32);
            }
            i = j;
        }
        result
    }

    pub(crate) fn effective_offsets_precise(&self) -> Vec<f64> {
        effective_offsets_precise_for(&self.stops)
    }

    /// Amostragem 1D em t ∈ [0, 1] (paridade `Linear::sample`).
    ///
    /// **P270**: interpola no `self.space` via dispatcher
    /// `interpolate_in_space`. Default `space: Oklab` preserva P264/P269
    /// behavior bit-exact.
    pub fn sample(&self, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        let offs = self.effective_offsets();
        let n = self.stops.len();
        if n == 0 {
            return Color::rgb(0, 0, 0);
        }
        if n == 1 {
            return self.stops[0].color.clone();
        }

        for i in 0..(n - 1) {
            let o0 = offs[i];
            let o1 = offs[i + 1];
            if t >= o0 && t <= o1 {
                let local_t = if o1 > o0 { (t - o0) / (o1 - o0) } else { 0.0 };
                return interpolate_in_space(
                    self.stops[i].color.clone(),
                    self.stops[i + 1].color.clone(),
                    local_t,
                    self.space.clone(),
                );
            }
        }
        if t <= offs[0] {
            self.stops[0].color.clone()
        } else {
            self.stops[n - 1].color.clone()
        }
    }

    /// Sampling da superfície de linguagem sem estreitar `Ratio(f64)` antes
    /// da formação dos pesos do mixing (P1271-C4).
    pub(crate) fn sample_precise(&self, t: f64) -> Color {
        let t = t.clamp(0.0, 1.0);
        let n = self.stops.len();
        if n == 0 {
            return Color::rgb(0, 0, 0);
        }
        if n == 1 {
            return self.stops[0].color.clone();
        }

        let offsets = self.effective_offsets_precise();
        for i in 0..(n - 1) {
            let (o0, o1) = (offsets[i], offsets[i + 1]);
            if t >= o0 && t <= o1 {
                let local = if o1 > o0 { (t - o0) / (o1 - o0) } else { 0.0 };
                let (w0, w1) = ((1.0 - local) as f32, local as f32);
                let total = w0 + w1;
                return match self.space {
                    ColorSpace::Oklab => {
                        let (l0, a0, b0, alpha0) =
                            color_to_oklab_with_alpha(self.stops[i].color.clone());
                        let (l1, a1, b1, alpha1) =
                            color_to_oklab_with_alpha(self.stops[i + 1].color.clone());
                        Color::oklab(
                            (w0 * l0 + w1 * l1) / total,
                            (w0 * a0 + w1 * a1) / total,
                            (w0 * b0 + w1 * b1) / total,
                            (w0 * alpha0 + w1 * alpha1) / total,
                        )
                    }
                    ColorSpace::LinearRgb => {
                        let (r0, g0, b0, a0) =
                            to_linear_rgb_components(self.stops[i].color.clone());
                        let (r1, g1, b1, a1) =
                            to_linear_rgb_components(self.stops[i + 1].color.clone());
                        Color::linear_rgb(
                            (w0 * r0 + w1 * r1) / total,
                            (w0 * g0 + w1 * g1) / total,
                            (w0 * b0 + w1 * b1) / total,
                            (w0 * a0 + w1 * a1) / total,
                        )
                    }
                    ColorSpace::Srgb
                    | ColorSpace::Luma
                    | ColorSpace::Oklch
                    | ColorSpace::Hsl
                    | ColorSpace::Hsv
                    | ColorSpace::Cmyk
                    | ColorSpace::Spot(_) => interpolate_in_space(
                        self.stops[i].color.clone(),
                        self.stops[i + 1].color.clone(),
                        local as f32,
                        self.space.clone(),
                    ),
                };
            }
        }
        if t <= offsets[0] {
            self.stops[0].color.clone()
        } else {
            self.stops[n - 1].color.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stops() -> Vec<GradientStop> {
        vec![
            GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
            GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
        ]
    }

    #[test]
    fn preserves_variant_behavior() {
        let gradient = Gradient::radial(
            stops(),
            crate::entities::axes::Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
        );
        let Gradient::Radial(radial) = gradient else { panic!("expected radial") };
        assert_eq!(radial.effective_offsets(), vec![0.0, 1.0]);
    }

    fn precise_radial(space: ColorSpace, stops: Vec<GradientStop>) -> Radial {
        Radial {
            stops: stops.into(),
            center: crate::entities::axes::Axes::new(Ratio(0.5), Ratio(0.5)),
            radius: Ratio(0.5),
            focal_center: crate::entities::axes::Axes::new(Ratio(0.5), Ratio(0.5)),
            focal_radius: Ratio(0.0),
            space,
            relative: None,
            anti_alias: true,
        }
    }

    fn precise_pair(space: ColorSpace, first: Color, last: Color) -> Radial {
        precise_radial(
            space,
            vec![
                GradientStop::new(first, Ratio(0.0)),
                GradientStop::new(last, Ratio(1.0)),
            ],
        )
    }

    fn assert_components(actual: [f32; 4], expected: [f32; 4]) {
        for (actual, expected) in actual.into_iter().zip(expected) {
            assert!((actual - expected).abs() < 1e-6, "{actual} != {expected}");
        }
    }

    #[test]
    fn sample_precise_specialized_spaces_keep_components_and_alpha() {
        // Native-space dyadic fixtures give independent, exact quarter weights.
        let lab = precise_pair(
            ColorSpace::Oklab,
            Color::oklab(0.25, -0.25, 0.5, 0.25),
            Color::oklab(0.75, 0.25, -0.5, 0.75),
        );
        let Color::Oklab { l, a, b, alpha } = lab.sample_precise(0.25) else {
            panic!("expected Oklab")
        };
        assert_components([l, a, b, alpha], [0.375, -0.125, 0.25, 0.375]);

        let rgb = precise_pair(
            ColorSpace::LinearRgb,
            Color::linear_rgb(0.0, 0.25, 1.0, 0.25),
            Color::linear_rgb(1.0, 0.75, 0.0, 0.75),
        );
        let Color::LinearRgb { r, g, b, a } = rgb.sample_precise(0.25) else {
            panic!("expected linear RGB")
        };
        assert_components([r, g, b, a], [0.25, 0.375, 0.75, 0.375]);
    }

    #[test]
    fn sample_precise_delegated_process_spaces_keep_components_and_shorter_hue() {
        let rgb = precise_pair(
            ColorSpace::Srgb,
            Color::srgb_f32(0.0, 0.25, 1.0, 0.25),
            Color::srgb_f32(1.0, 0.75, 0.0, 0.75),
        );
        let Color::Srgb { r, g, b, a } = rgb.sample_precise(0.25) else {
            panic!("expected sRGB")
        };
        assert_components([r, g, b, a], [0.25, 0.375, 0.75, 0.375]);

        let gray = precise_pair(
            ColorSpace::Luma,
            Color::Luma { l: 0.25, a: 0.25 },
            Color::Luma { l: 0.75, a: 0.75 },
        );
        let Color::Luma { l, a } = gray.sample_precise(0.25) else {
            panic!("expected luma")
        };
        assert_components([l, a, 0.0, 0.0], [0.375, 0.375, 0.0, 0.0]);

        // 350 -> 10 crosses zero along the shorter arc: one quarter is 355.
        let lch = precise_pair(
            ColorSpace::Oklch,
            Color::oklch(0.25, 0.125, 350.0, 0.25),
            Color::oklch(0.75, 0.375, 10.0, 0.75),
        );
        let Color::Oklch { l, c, h, alpha } = lch.sample_precise(0.25) else {
            panic!("expected Oklch")
        };
        assert_components([l, c, h, alpha], [0.375, 0.1875, 355.0, 0.375]);

        let hsl = precise_pair(
            ColorSpace::Hsl,
            Color::hsl(350.0, 0.25, 0.25, 0.25),
            Color::hsl(10.0, 0.75, 0.75, 0.75),
        );
        let Color::Hsl { h, s, l, a } = hsl.sample_precise(0.25) else {
            panic!("expected HSL")
        };
        assert_components([h, s, l, a], [355.0, 0.375, 0.375, 0.375]);

        let hsv = precise_pair(
            ColorSpace::Hsv,
            Color::hsv(350.0, 0.25, 0.25, 0.25),
            Color::hsv(10.0, 0.75, 0.75, 0.75),
        );
        let Color::Hsv { h, s, v, a } = hsv.sample_precise(0.25) else {
            panic!("expected HSV")
        };
        assert_components([h, s, v, a], [355.0, 0.375, 0.375, 0.375]);

        let cmyk = precise_pair(
            ColorSpace::Cmyk,
            Color::cmyk(0.0, 0.25, 1.0, 0.25),
            Color::cmyk(1.0, 0.75, 0.0, 0.75),
        );
        let Color::Cmyk { c, m, y, k } = cmyk.sample_precise(0.25) else {
            panic!("expected CMYK")
        };
        assert_components([c, m, y, k], [0.25, 0.375, 0.75, 0.375]);
    }

    #[test]
    fn sample_precise_spot_keeps_complete_identity_or_uses_process_fallback() {
        use crate::entities::color::{SpotColor, SpotColorant};

        let colorant = Arc::new(SpotColorant {
            name: Some("ink".into()),
            fallback: Color::rgb(0, 0, 0),
        });
        let first =
            Color::Spot(SpotColor { colorant: colorant.clone(), tint: Ratio(0.25) });
        let last =
            Color::Spot(SpotColor { colorant: colorant.clone(), tint: Ratio(0.75) });
        let radial = precise_pair(ColorSpace::Spot((*colorant).clone()), first, last);
        let Color::Spot(sample) = radial.sample_precise(0.25) else {
            panic!("matching complete colorants must retain Spot")
        };
        assert_eq!(sample.colorant.as_ref(), colorant.as_ref());
        assert!(Arc::ptr_eq(&sample.colorant, &colorant));
        assert_eq!(sample.tint, Ratio(0.375));

        // Full tints of black and white have analytic Oklab endpoints (0,0,0)
        // and (1,0,0). Equal names alone must not equate different fallbacks.
        let white = Arc::new(SpotColorant {
            name: Some("ink".into()),
            fallback: Color::rgb(255, 255, 255),
        });
        let mixed = precise_pair(
            ColorSpace::Spot((*colorant).clone()),
            Color::Spot(SpotColor { colorant: colorant.clone(), tint: Ratio(1.0) }),
            Color::Spot(SpotColor { colorant: white, tint: Ratio(1.0) }),
        );
        let Color::Oklab { l, a, b, alpha } = mixed.sample_precise(0.25) else {
            panic!("different colorants must interpolate process colors")
        };
        assert_components([l, a, b, alpha], [0.25, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn sample_precise_preserves_empty_single_clamp_and_coincident_boundaries() {
        assert_eq!(
            precise_radial(ColorSpace::Oklab, vec![]).sample_precise(0.5),
            Color::rgb(0, 0, 0)
        );
        let single = Color::hsl(123.0, 0.25, 0.5, 0.75);
        let radial = precise_radial(
            ColorSpace::Oklab,
            vec![GradientStop::unspaced(single.clone())],
        );
        for t in [-1.0, 0.5, 2.0] {
            assert_eq!(radial.sample_precise(t), single);
        }
        let first = Color::srgb_f32(0.0, 0.25, 1.0, 0.25);
        let last = Color::srgb_f32(1.0, 0.75, 0.0, 0.75);
        let radial = precise_pair(ColorSpace::Srgb, first.clone(), last.clone());
        assert_eq!(radial.sample_precise(-1.0), first);
        assert_eq!(radial.sample_precise(2.0), last);

        let coincident = precise_radial(
            ColorSpace::Srgb,
            vec![
                GradientStop::new(first.clone(), Ratio(0.5)),
                GradientStop::new(last.clone(), Ratio(0.5)),
            ],
        );
        assert_eq!(coincident.sample_precise(0.5), first);
        assert_eq!(coincident.sample_precise(0.25), first);
        assert_eq!(coincident.sample_precise(0.75), last);
    }
}
