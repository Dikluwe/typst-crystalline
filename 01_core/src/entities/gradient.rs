//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/gradient.md
//! @prompt-hash 391208e2
//! @layer L1
//! @updated 2026-05-15
//!
//! **P262 (M9d / M7+5; ADR-0087 PROPOSTO Gradient Linear-only;
//! cumpre ADR-0086 §"Critério revisão" Paint::Gradient variant
//! activada; precedente N=3 do pattern P257/P261)** — wrapper
//! enum sobre tipos de gradient (Linear apenas; Radial/Conic
//! comentários reserva). Sub-componente `GradientStop` com
//! `Option<Ratio>` paridade vanilla auto-spacing.
//!
//! **ColorSpace fixo Oklab** per decisão user P262 Q2 (paridade
//! vanilla default). Interpolação em Oklab via `Color::oklab`;
//! conversão final PDF via `Color::to_rgba_f32()`.
//!
//! **ADR-0039 preservado**: TextStyle.fill: Option<Color>
//! inalterado.
//!
//! Cross-references:
//! - Vanilla `lab/typst-original/.../visualize/gradient.rs` (1366 LoC).
//! - ADR-0087 — Gradient Linear-only (IMPLEMENTADO P262).
//! - ADR-0086 — Paint wrapper (Solid only → §"Critério revisão"
//!   cumprido por este passo).

use std::sync::Arc;
use crate::entities::color::Color;
use crate::entities::layout_types::{Angle, Ratio};

/// Sub-componente per ADR-0029 §exclusões.
///
/// `offset: Option<Ratio>` permite auto-spacing (paridade vanilla):
/// stops com `None` recebem distribuição uniforme entre stops
/// com offset explícito (ou nos extremos implícitos 0% / 100%).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GradientStop {
    pub color:  Color,
    pub offset: Option<Ratio>,
}

impl GradientStop {
    pub fn new(color: Color, offset: Ratio) -> Self {
        Self { color, offset: Some(offset) }
    }

    pub fn unspaced(color: Color) -> Self {
        Self { color, offset: None }
    }
}

/// Linear gradient — paridade vanilla LinearGradient.
///
/// ColorSpace fixo Oklab (scope-out ADR-0087 — paridade vanilla
/// default). Campos `space`/`relative`/`anti_alias` vanilla
/// não materializados; ver ADR-0087 §scope-outs.
#[derive(Debug, Clone, PartialEq)]
pub struct Linear {
    pub stops: Arc<[GradientStop]>,
    pub angle: Angle,
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
        if n == 0 { return Vec::new(); }
        if n == 1 {
            return vec![self.stops[0].offset.map(|r| r.0 as f32).unwrap_or(0.0)];
        }

        let mut offs: Vec<Option<f32>> = self.stops.iter()
            .map(|s| s.offset.map(|r| r.0 as f32))
            .collect();

        // Extremos implícitos quando ausentes.
        if offs[0].is_none() { offs[0] = Some(0.0); }
        if offs[n - 1].is_none() { offs[n - 1] = Some(1.0); }

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
            while j < n && offs[j].is_none() { j += 1; }
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

    /// Amostra a cor interpolada em parâmetro t ∈ [0, 1].
    /// Interpolação em Oklab (paridade vanilla default).
    pub fn sample(&self, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        let offs = self.effective_offsets();
        let n = self.stops.len();
        if n == 0 { return Color::rgb(0, 0, 0); }
        if n == 1 { return self.stops[0].color; }

        // Encontrar par [i, i+1] tal que offs[i] <= t <= offs[i+1].
        for i in 0..(n - 1) {
            let o0 = offs[i];
            let o1 = offs[i + 1];
            if t >= o0 && t <= o1 {
                let local_t = if o1 > o0 { (t - o0) / (o1 - o0) } else { 0.0 };
                return interpolate_oklab(self.stops[i].color, self.stops[i + 1].color, local_t);
            }
        }
        // Fallback (clamp): extremo apropriado.
        if t <= offs[0] { self.stops[0].color } else { self.stops[n - 1].color }
    }
}

/// Interpolação linear em Oklab.
///
/// Converte ambas as cores para representação Oklab (via
/// to_rgba_f32 → linear path); interpola linearmente os componentes
/// L, a, b; retorna `Color::oklab` com alpha interpolado.
fn interpolate_oklab(c0: Color, c1: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    let (l0, a0, b0, alpha0) = color_to_oklab_with_alpha(c0);
    let (l1, a1, b1, alpha1) = color_to_oklab_with_alpha(c1);
    Color::oklab(
        l0 + (l1 - l0) * t,
        a0 + (a1 - a0) * t,
        b0 + (b1 - b0) * t,
        alpha0 + (alpha1 - alpha0) * t,
    )
}

/// Helper privado: converte qualquer Color para Oklab (L, a, b, alpha).
///
/// Para cores já em Oklab retorna campos directos; para outras
/// converte via sRGB → linear → Oklab (caminho inverso).
fn color_to_oklab_with_alpha(c: Color) -> (f32, f32, f32, f32) {
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
fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

/// linear sRGB → Oklab (paridade ICC; constantes da publicação Björn Ottosson 2020).
fn linear_rgb_to_oklab(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let l = 0.412_221_46 * r + 0.536_332_55 * g + 0.051_445_995 * b;
    let m = 0.211_903_5  * r + 0.680_699_56 * g + 0.107_396_96  * b;
    let s = 0.088_302_46 * r + 0.281_718_85 * g + 0.629_978_71  * b;

    let l_ = l.cbrt();
    let m_ = m.cbrt();
    let s_ = s.cbrt();

    let l_lab = 0.210_454_26 * l_ + 0.793_617_8   * m_ - 0.004_072_047 * s_;
    let a_lab = 1.977_998_5  * l_ - 2.428_592_2   * m_ + 0.450_593_7   * s_;
    let b_lab = 0.025_904_037 * l_ + 0.782_771_77 * m_ - 0.808_675_77  * s_;

    (l_lab, a_lab, b_lab)
}

/// Gradient — enum tagged paridade vanilla.
#[derive(Debug, Clone, PartialEq)]
pub enum Gradient {
    Linear(Arc<Linear>),
    // Radial(Arc<Radial>),  // P-Gradient-Radial — comentário reserva
    // Conic(Arc<Conic>),    // P-Gradient-Conic — comentário reserva
}

impl Gradient {
    pub fn linear(
        stops: impl Into<Arc<[GradientStop]>>,
        angle: Angle,
    ) -> Self {
        Gradient::Linear(Arc::new(Linear {
            stops: stops.into(),
            angle,
        }))
    }

    /// Retorna primeira cor do primeiro stop. Usado como
    /// fallback para `Paint::to_color()` quando consumer
    /// precisa de Color literal (Solid path).
    pub fn first_stop_color(&self) -> Color {
        match self {
            Gradient::Linear(l) => l.stops.first()
                .map(|s| s.color)
                .unwrap_or(Color::rgb(0, 0, 0)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gradient_stop_new_com_offset() {
        let s = GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.5));
        assert_eq!(s.color, Color::rgb(255, 0, 0));
        assert_eq!(s.offset, Some(Ratio(0.5)));
    }

    #[test]
    fn gradient_stop_unspaced() {
        let s = GradientStop::unspaced(Color::rgb(0, 255, 0));
        assert_eq!(s.color, Color::rgb(0, 255, 0));
        assert_eq!(s.offset, None);
    }

    #[test]
    fn gradient_linear_construcao_2_stops() {
        let g = Gradient::linear(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            Angle::deg(0.0),
        );
        if let Gradient::Linear(l) = &g {
            assert_eq!(l.stops.len(), 2);
            assert_eq!(l.angle.to_rad(), 0.0);
        } else {
            panic!("esperado Gradient::Linear");
        }
    }

    #[test]
    fn gradient_first_stop_color() {
        let g = Gradient::linear(
            vec![
                GradientStop::new(Color::rgb(100, 50, 25), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 0), Ratio(1.0)),
            ],
            Angle::deg(0.0),
        );
        assert_eq!(g.first_stop_color(), Color::rgb(100, 50, 25));
    }

    #[test]
    fn linear_effective_offsets_explicit() {
        let l = Linear {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 255, 0), Ratio(0.5)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]),
            angle: Angle::deg(0.0),
        };
        let offs = l.effective_offsets();
        assert_eq!(offs, vec![0.0, 0.5, 1.0]);
    }

    #[test]
    fn linear_effective_offsets_auto_spacing_all_none() {
        let l = Linear {
            stops: Arc::from(vec![
                GradientStop::unspaced(Color::rgb(255, 0, 0)),
                GradientStop::unspaced(Color::rgb(0, 255, 0)),
                GradientStop::unspaced(Color::rgb(0, 0, 255)),
            ]),
            angle: Angle::deg(0.0),
        };
        let offs = l.effective_offsets();
        // 3 stops igualmente espaçados: 0.0 / 0.5 / 1.0
        assert!((offs[0] - 0.0).abs() < 1e-5);
        assert!((offs[1] - 0.5).abs() < 1e-5);
        assert!((offs[2] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn linear_effective_offsets_auto_spacing_middle_explicit() {
        let l = Linear {
            stops: Arc::from(vec![
                GradientStop::unspaced(Color::rgb(255, 0, 0)),
                GradientStop::new(Color::rgb(0, 255, 0), Ratio(0.7)),
                GradientStop::unspaced(Color::rgb(0, 0, 255)),
            ]),
            angle: Angle::deg(0.0),
        };
        let offs = l.effective_offsets();
        assert!((offs[0] - 0.0).abs() < 1e-5);
        assert!((offs[1] - 0.7).abs() < 1e-5);
        assert!((offs[2] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn linear_sample_extremos_returns_stops() {
        let l = Linear {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]),
            angle: Angle::deg(0.0),
        };
        // Amostragem nos extremos deve retornar cores dos stops
        // (após Oklab roundtrip — pequena tolerância).
        let c0 = l.sample(0.0);
        let c1 = l.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (r1, _, _, _) = c1.to_rgba_f32();
        // Vermelho stop @ 0 → r próximo de 1.0
        assert!(r0 > 0.9, "sample(0.0) deve retornar próximo de vermelho, r={}", r0);
        // Azul stop @ 1 → r próximo de 0.0
        assert!(r1 < 0.1, "sample(1.0) deve retornar próximo de azul, r={}", r1);
    }

    #[test]
    fn linear_sample_meio_interpola() {
        let l = Linear {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]),
            angle: Angle::deg(0.0),
        };
        // Sample em 0.5 deve ser mistura entre vermelho e azul.
        let c_meio = l.sample(0.5);
        let (r, _, b, _) = c_meio.to_rgba_f32();
        // Em Oklab → sRGB, a mistura tem componentes r e b
        // ambos > 0 mas nenhum dominante.
        assert!(r > 0.05 && r < 0.95, "sample(0.5) r intermediate: {}", r);
        assert!(b > 0.05 && b < 0.95, "sample(0.5) b intermediate: {}", b);
    }

    #[test]
    fn gradient_clone_arc_o1() {
        let g = Gradient::linear(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            Angle::deg(0.0),
        );
        let g2 = g.clone();
        // Arc clones share storage.
        if let (Gradient::Linear(l1), Gradient::Linear(l2)) = (&g, &g2) {
            assert!(Arc::ptr_eq(l1, l2), "Arc clone deve partilhar storage");
        }
    }

    #[test]
    fn gradient_partial_eq() {
        let g1 = Gradient::linear(
            vec![GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0))],
            Angle::deg(0.0),
        );
        let g2 = Gradient::linear(
            vec![GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0))],
            Angle::deg(0.0),
        );
        let g3 = Gradient::linear(
            vec![GradientStop::new(Color::rgb(0, 255, 0), Ratio(0.0))],
            Angle::deg(0.0),
        );
        assert_eq!(g1, g2);
        assert_ne!(g1, g3);
    }

    #[test]
    fn linear_effective_offsets_1_stop() {
        let l = Linear {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(100, 100, 100), Ratio(0.3)),
            ]),
            angle: Angle::deg(0.0),
        };
        assert_eq!(l.effective_offsets(), vec![0.3]);
    }

    #[test]
    fn linear_sample_clamp_above_1() {
        let l = Linear {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]),
            angle: Angle::deg(0.0),
        };
        // t > 1.0 deve clamp.
        let c = l.sample(1.5);
        let c_ref = l.sample(1.0);
        assert_eq!(c, c_ref);
    }
}
