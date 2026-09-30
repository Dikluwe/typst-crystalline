//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/gradient/conic.md
//! @prompt-hash 3ce7c884
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Conic gradient — paridade vanilla `ConicGradient` subset
/// (per ADR-0089 P267).
///
/// **3 campos materializados**: stops + center + angle.
/// Scope-outs (per ADR-0089): space (Oklab fixo), relative
/// (bbox-local), anti_alias (true). **Sem `focal_*`** — não
/// existe em ConicGradient vanilla (exclusivo Radial).
///
/// **PDF render Conic fallback Solid** (first_stop_color) até
/// P268 dedicado materializar shading real.
#[derive(Debug, Clone, PartialEq)]
pub struct Conic {
    pub stops: Arc<[GradientStop]>,
    pub center: crate::entities::axes::Axes<Ratio>,
    pub angle: Angle,
    /// P270 — ColorSpace runtime activado per ADR-0091 EM VIGOR.
    /// Default via construtor `Gradient::conic(...)` = `ColorSpace::Oklab`.
    pub space: ColorSpace,
    /// P273 — `RelativeTo` cross-variant runtime field. `None` = Auto =
    /// `Self_` default; preserva P267/P272 bit-exact (Coons RGB N=stops*4
    /// + Coons CMYK N=stops).
    pub relative: Option<RelativeTo>,
    /// P1145 — constructors normais usam true; `gradient.sharp` usa false.
    pub anti_alias: bool,
}

impl Conic {
    /// Auto-spacing paridade `Linear::effective_offsets` (P262)
    /// e `Radial::effective_offsets` (P264).
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

    /// Amostragem 1D em t ∈ [0, 1] (paridade `Linear::sample` +
    /// `Radial::sample`).
    ///
    /// Para Conic, `t` representa a fração da circumferência
    /// (0 = ângulo inicial; 1 = volta completa).
    ///
    /// **P270**: interpola no `self.space` via dispatcher
    /// `interpolate_in_space`. Default `space: Oklab` preserva P267
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
        let gradient = Gradient::conic(
            stops(),
            crate::entities::axes::Axes::new(Ratio(0.5), Ratio(0.5)),
            Angle::deg(0.0),
        );
        let Gradient::Conic(conic) = gradient else { panic!("expected conic") };
        assert_eq!(conic.effective_offsets(), vec![0.0, 1.0]);
    }
}
