//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/gradient/linear.md
//! @prompt-hash b426ff8d
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Linear gradient — paridade vanilla LinearGradient.
///
/// ColorSpace fixo Oklab (scope-out ADR-0087 — paridade vanilla
/// default). Campos `space`/`relative`/`anti_alias` vanilla
/// não materializados; ver ADR-0087 §scope-outs.
#[derive(Debug, Clone, PartialEq)]
pub struct Linear {
    pub stops: Arc<[GradientStop]>,
    pub angle: Angle,
    /// P270 — ColorSpace runtime activado per ADR-0091 EM VIGOR.
    /// Default via construtor `Gradient::linear(...)` = `ColorSpace::Oklab`
    /// (preserva P262 behavior bit-exact).
    pub space: ColorSpace,
    /// P273 — `RelativeTo` cross-variant runtime field per ADR-0091
    /// §"Anotação cumulativa P273". `None` = Auto = `Self_` default;
    /// preserva P262/P263/P270.1/P270.2 bit-exact.
    pub relative: Option<RelativeTo>,
    /// P1145 — constructors normais usam true; `gradient.sharp` usa false.
    pub anti_alias: bool,
}

impl Linear {
    /// Calcula offsets efectivos com auto-spacing aplicado
    /// (paridade vanilla — stops com offset=None recebem
    /// distribuição uniforme entre stops com offset explícito
    /// ou extremos implícitos 0.0 / 1.0).
    ///
    /// Algoritmo:
    /// 1. Identifica runs consecutivos de offset=None.
    /// 2. Cada run delimitado por offset explícito (ou extremos
    ///    implícitos 0.0 / 1.0).
    /// 3. Interpola offsets em [prev, next] distribuição uniforme.
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

        // Extremos implícitos quando ausentes.
        if offs[0].is_none() {
            offs[0] = Some(0.0);
        }
        if offs[n - 1].is_none() {
            offs[n - 1] = Some(1.0);
        }

        // Preenche runs de None entre offsets explícitos.
        let mut result = vec![0.0_f32; n];
        let mut i = 0;
        while i < n {
            if let Some(v) = offs[i] {
                result[i] = v;
                i += 1;
                continue;
            }
            // Encontra próximo offset explícito.
            let mut j = i;
            while j < n && offs[j].is_none() {
                j += 1;
            }
            // i-1 tem offset explícito (já preenchido em result[i-1]).
            // j tem offset explícito (offs[j]).
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

    /// Resolve os offsets no carrier preciso da superfície de linguagem.
    ///
    /// Mantém o método histórico `effective_offsets` em `f32` para os
    /// consumidores Rust existentes, mas evita arredondar os `Ratio(f64)`
    /// usados para reconhecer identidades de `gradient.stops` e pelo sampling
    /// preciso.
    pub(crate) fn effective_offsets_precise(&self) -> Vec<f64> {
        effective_offsets_precise_for(&self.stops)
    }

    /// Amostra a cor interpolada em parâmetro t ∈ [0, 1].
    ///
    /// **P270**: interpola no `self.space` via dispatcher
    /// `interpolate_in_space`. Default `space: Oklab` preserva P262
    /// behavior bit-exact (dispatcher chama `interpolate_oklab` original
    /// para arm Oklab).
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

        // Encontrar par [i, i+1] tal que offs[i] <= t <= offs[i+1].
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
        // Fallback (clamp): extremo apropriado.
        if t <= offs[0] {
            self.stops[0].color.clone()
        } else {
            self.stops[n - 1].color.clone()
        }
    }

    pub(crate) fn sample_precise(&self, t: f64) -> Color {
        let t = t.clamp(0.0, 1.0);
        let n = self.stops.len();
        if n == 0 {
            return Color::rgb(0, 0, 0);
        }
        if n == 1 {
            return self.stops[0].color.clone();
        }
        let resolved = self.effective_offsets_precise();
        for i in 0..(n - 1) {
            let (o0, o1) = (resolved[i], resolved[i + 1]);
            if t >= o0 && t <= o1 {
                let local = if o1 > o0 { (t - o0) / (o1 - o0) } else { 0.0 };
                if self.space == ColorSpace::Oklab {
                    let (l0, a0, b0, alpha0) =
                        color_to_oklab_with_alpha(self.stops[i].color.clone());
                    let (l1, a1, b1, alpha1) =
                        color_to_oklab_with_alpha(self.stops[i + 1].color.clone());
                    let (w0, w1) = ((1.0 - local) as f32, local as f32);
                    let total = w0 + w1;
                    return Color::oklab(
                        (w0 * l0 + w1 * l1) / total,
                        (w0 * a0 + w1 * a1) / total,
                        (w0 * b0 + w1 * b1) / total,
                        (w0 * alpha0 + w1 * alpha1) / total,
                    );
                }
                return interpolate_in_space(
                    self.stops[i].color.clone(),
                    self.stops[i + 1].color.clone(),
                    local as f32,
                    self.space.clone(),
                );
            }
        }
        if t <= resolved[0] {
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
        let gradient = Gradient::linear(stops(), Angle::deg(0.0));
        let Gradient::Linear(linear) = gradient else { panic!("expected linear") };
        assert_eq!(linear.effective_offsets(), vec![0.0, 1.0]);
    }
}
