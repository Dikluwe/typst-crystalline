//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/gradient/radial.md
//! @prompt-hash b137636c
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
                    _ => interpolate_in_space(
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
}
