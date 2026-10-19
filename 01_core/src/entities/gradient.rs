//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/gradient.md
//! @prompt-hash a761ebb9
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
//! **ADR-0039 preservado**: TextStyle.fill: `Option<Color>`
//! inalterado.
//!
//! Cross-references:
//! - Vanilla `lab/typst-original/.../visualize/gradient.rs` (1366 LoC).
//! - ADR-0087 — Gradient Linear-only (IMPLEMENTADO P262).
//! - ADR-0086 — Paint wrapper (Solid only → §"Critério revisão"
//!   cumprido por este passo).

use crate::entities::color::{Color, ColorSpace};
use crate::entities::layout_types::{Angle, Ratio};
use std::sync::Arc;

/// Sub-componente per ADR-0029 §exclusões.
///
/// `offset: Option<Ratio>` permite auto-spacing (paridade vanilla):
/// stops com `None` recebem distribuição uniforme entre stops
/// com offset explícito (ou nos extremos implícitos 0% / 100%).
#[derive(Debug, Clone, PartialEq)]
pub struct GradientStop {
    pub color: Color,
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

/// Resolve offsets sem estreitar o carrier público `Ratio(f64)` (P1271-C3).
/// P273 — Define a que bounding box o gradient é relativo.
///
/// Paridade vanilla `RelativeTo { Self_, Parent }`. Cristalino
/// simplifica `Smart<RelativeTo>` → `Option<RelativeTo>` per
/// ADR-0064 §Caso A (`None` = Auto = `Self_` default).
///
/// **Default `Self_`** (vanilla `unwrap_or_else(|| if on_text {
/// Parent } else { Self_ })` simplificado — cristalino ignora
/// contexto `on_text` por enquanto; materializável futuro se
/// necessário).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RelativeTo {
    /// Relative ao próprio shape (bounding box self).
    #[default]
    Self_,
    /// Relative ao parent container (bounding box parent).
    Parent,
}

mod conic;
mod interpolation;
mod kind;
mod linear;
mod offsets;
mod radial;

pub use self::conic::Conic;
pub use self::interpolation::color_to_oklab_with_alpha;
pub use self::kind::Gradient;
pub use self::linear::Linear;
pub use self::radial::Radial;
use self::{interpolation::*, offsets::*};

#[cfg(test)]
mod atomization_tests {
    use super::*;

    #[test]
    fn implicit_middle_offset_is_halfway() {
        let stops = [
            GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
            GradientStop::unspaced(Color::rgb(0, 255, 0)),
            GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
        ];
        assert_eq!(offsets::effective_offsets_precise_for(&stops), vec![0.0, 0.5, 1.0]);
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
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
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
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
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
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
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
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
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
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
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
            stops: Arc::from(vec![GradientStop::new(
                Color::rgb(100, 100, 100),
                Ratio(0.3),
            )]),
            angle: Angle::deg(0.0),
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
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
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
        };
        // t > 1.0 deve clamp.
        let c = l.sample(1.5);
        let c_ref = l.sample(1.0);
        assert_eq!(c, c_ref);
    }

    // ── P264 (ADR-0088 Gradient Radial-only) ───────────────────────────

    use crate::entities::axes::Axes;

    #[test]
    fn p264_radial_construcao_2_stops() {
        let g = Gradient::radial(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
        );
        if let Gradient::Radial(r) = &g {
            assert_eq!(r.stops.len(), 2);
            assert_eq!(r.center.x, Ratio(0.5));
            assert_eq!(r.center.y, Ratio(0.5));
            assert_eq!(r.radius, Ratio(0.5));
        } else {
            panic!("esperado Gradient::Radial");
        }
    }

    #[test]
    fn p264_radial_first_stop_color() {
        let g = Gradient::radial(
            vec![
                GradientStop::new(Color::rgb(100, 200, 50), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 0), Ratio(1.0)),
            ],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
        );
        assert_eq!(g.first_stop_color(), Color::rgb(100, 200, 50));
    }

    #[test]
    fn p264_radial_clone_arc_o1() {
        let g = Gradient::radial(
            vec![GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0))],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
        );
        let g2 = g.clone();
        if let (Gradient::Radial(r1), Gradient::Radial(r2)) = (&g, &g2) {
            assert!(Arc::ptr_eq(r1, r2), "Arc clone deve partilhar storage");
        }
    }

    #[test]
    fn p264_radial_partial_eq() {
        let g1 = Gradient::radial(
            vec![GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0))],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
        );
        let g2 = Gradient::radial(
            vec![GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0))],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
        );
        let g3 = Gradient::radial(
            vec![GradientStop::new(Color::rgb(0, 255, 0), Ratio(0.0))],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
        );
        assert_eq!(g1, g2);
        assert_ne!(g1, g3);
    }

    #[test]
    fn p264_radial_effective_offsets_auto_spacing() {
        let center = Axes::new(Ratio(0.5), Ratio(0.5));
        let r = Radial {
            stops: Arc::from(vec![
                GradientStop::unspaced(Color::rgb(255, 0, 0)),
                GradientStop::unspaced(Color::rgb(0, 255, 0)),
                GradientStop::unspaced(Color::rgb(0, 0, 255)),
            ]),
            center,
            radius: Ratio(0.5),
            focal_center: center,
            focal_radius: Ratio(0.0),
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
        };
        let offs = r.effective_offsets();
        assert!((offs[0] - 0.0).abs() < 1e-5);
        assert!((offs[1] - 0.5).abs() < 1e-5);
        assert!((offs[2] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn p264_radial_sample_extremos() {
        let center = Axes::new(Ratio(0.5), Ratio(0.5));
        let r = Radial {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]),
            center,
            radius: Ratio(0.5),
            focal_center: center,
            focal_radius: Ratio(0.0),
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
        };
        // Amostragem nos extremos: r próximo de vermelho/azul.
        let c0 = r.sample(0.0);
        let c1 = r.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (r1, _, b1, _) = c1.to_rgba_f32();
        assert!(r0 > 0.9, "sample(0.0) ≈ vermelho, r={}", r0);
        assert!(r1 < 0.1 && b1 > 0.9, "sample(1.0) ≈ azul, r={}, b={}", r1, b1);
    }

    #[test]
    fn p264_radial_sample_clamp_above_1() {
        let center = Axes::new(Ratio(0.5), Ratio(0.5));
        let r = Radial {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]),
            center,
            radius: Ratio(0.5),
            focal_center: center,
            focal_radius: Ratio(0.0),
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
        };
        let c = r.sample(1.5);
        let c_ref = r.sample(1.0);
        assert_eq!(c, c_ref);
    }

    #[test]
    fn p264_gradient_radial_to_paint_via_from() {
        use crate::entities::paint::Paint;
        let g = Gradient::radial(
            vec![GradientStop::new(Color::rgb(0, 0, 0), Ratio(0.0))],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
        );
        let p: Paint = g.into();
        assert!(matches!(p, Paint::Gradient(Gradient::Radial(_))));
    }

    #[test]
    fn p264_radial_center_non_default() {
        let center = Axes::new(Ratio(0.25), Ratio(0.75));
        let r = Radial {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(0, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(255, 255, 255), Ratio(1.0)),
            ]),
            center,
            radius: Ratio(0.4),
            focal_center: center,
            focal_radius: Ratio(0.0),
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
        };
        assert_eq!(r.center.x, Ratio(0.25));
        assert_eq!(r.center.y, Ratio(0.75));
        assert_eq!(r.radius, Ratio(0.4));
    }

    // ── P269 (ADR-0088 §focal_* revogado parcialmente) — Radial focal_center + focal_radius

    #[test]
    fn p269_radial_construcao_default_focal_preserva_p264() {
        // Gradient::radial(...) sem focal args → focal=(center, 0)
        // → bytes /Coords idênticos P265.
        let center = Axes::new(Ratio(0.5), Ratio(0.5));
        let g = Gradient::radial(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            center,
            Ratio(0.5),
        );
        if let Gradient::Radial(r) = &g {
            assert_eq!(r.focal_center, center, "default focal_center = center");
            assert_eq!(r.focal_radius, Ratio(0.0), "default focal_radius = 0");
        } else {
            panic!("esperado Gradient::Radial");
        }
    }

    #[test]
    fn p269_gradient_radial_with_focal_construct() {
        let g = Gradient::radial_with_focal(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            Axes::new(Ratio(0.5), Ratio(0.5)), // center
            Ratio(0.5),                        // radius
            Axes::new(Ratio(0.3), Ratio(0.4)), // focal_center
            Ratio(0.1),                        // focal_radius
        );
        if let Gradient::Radial(r) = &g {
            assert_eq!(r.focal_center, Axes::new(Ratio(0.3), Ratio(0.4)));
            assert_eq!(r.focal_radius, Ratio(0.1));
        } else {
            panic!("esperado Gradient::Radial");
        }
    }

    #[test]
    fn p269_radial_focal_struct_pub_fields() {
        // focal_center + focal_radius são pub fields acessíveis directamente.
        let r = Radial {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(0, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(255, 255, 255), Ratio(1.0)),
            ]),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            radius: Ratio(0.5),
            focal_center: Axes::new(Ratio(0.2), Ratio(0.3)),
            focal_radius: Ratio(0.05),
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
        };
        assert_eq!(r.focal_center.x, Ratio(0.2));
        assert_eq!(r.focal_center.y, Ratio(0.3));
        assert_eq!(r.focal_radius, Ratio(0.05));
    }

    #[test]
    fn p269_radial_partial_eq_focal_distingue() {
        // Radials com focal diferente NÃO devem comparar igual.
        let stops = Arc::from(vec![
            GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
            GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
        ]);
        let r1 = Radial {
            stops: Arc::clone(&stops),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            radius: Ratio(0.5),
            focal_center: Axes::new(Ratio(0.5), Ratio(0.5)),
            focal_radius: Ratio(0.0),
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
        };
        let r2 = Radial {
            stops: Arc::clone(&stops),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            radius: Ratio(0.5),
            focal_center: Axes::new(Ratio(0.3), Ratio(0.4)), // diff
            focal_radius: Ratio(0.0),
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
        };
        assert_ne!(r1, r2, "focal_center diferente → PartialEq false");
    }

    #[test]
    fn p269_radial_partial_eq_focal_radius_distingue() {
        let stops = Arc::from(vec![
            GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
            GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
        ]);
        let r1 = Radial {
            stops: Arc::clone(&stops),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            radius: Ratio(0.5),
            focal_center: Axes::new(Ratio(0.5), Ratio(0.5)),
            focal_radius: Ratio(0.0),
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
        };
        let r2 = Radial {
            stops: Arc::clone(&stops),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            radius: Ratio(0.5),
            focal_center: Axes::new(Ratio(0.5), Ratio(0.5)),
            focal_radius: Ratio(0.1), // diff,
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
        };
        assert_ne!(r1, r2, "focal_radius diferente → PartialEq false");
    }

    #[test]
    fn p269_radial_clone_arc_o1_focal_preservado() {
        let g = Gradient::radial_with_focal(
            vec![GradientStop::new(Color::rgb(0, 0, 0), Ratio(0.0))],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
            Axes::new(Ratio(0.3), Ratio(0.4)),
            Ratio(0.05),
        );
        let g2 = g.clone();
        if let (Gradient::Radial(r1), Gradient::Radial(r2)) = (&g, &g2) {
            assert_eq!(r1.focal_center, r2.focal_center);
            assert_eq!(r1.focal_radius, r2.focal_radius);
            // Arc::ptr_eq garante O(1) clone (mesmo allocation interno).
            assert!(Arc::ptr_eq(r1, r2), "clone deve preservar Arc identity");
        } else {
            panic!("esperado Gradient::Radial em ambos");
        }
    }

    #[test]
    fn p269_radial_focal_radius_zero_default_idempotente() {
        // 2 construções via Gradient::radial(...) produzem focal_radius=0.
        let center = Axes::new(Ratio(0.5), Ratio(0.5));
        let g1 = Gradient::radial(
            vec![GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0))],
            center,
            Ratio(0.5),
        );
        let g2 = Gradient::radial(
            vec![GradientStop::new(Color::rgb(0, 0, 255), Ratio(0.0))],
            center,
            Ratio(0.5),
        );
        if let (Gradient::Radial(r1), Gradient::Radial(r2)) = (&g1, &g2) {
            assert_eq!(r1.focal_radius, Ratio(0.0));
            assert_eq!(r2.focal_radius, Ratio(0.0));
            assert_eq!(r1.focal_radius, r2.focal_radius);
        }
    }

    #[test]
    fn p269_radial_sample_inalterado_por_focal_default() {
        // sample(t) é 1D em cristalino — não usa focal. Verificar
        // que defaults focal não afectam saídas P264.
        let center = Axes::new(Ratio(0.5), Ratio(0.5));
        let g_default = Gradient::radial(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            center,
            Ratio(0.5),
        );
        if let Gradient::Radial(r) = &g_default {
            // sample(0.0) ≈ vermelho; sample(1.0) ≈ azul; idêntico P264.
            let c0 = r.sample(0.0);
            let c1 = r.sample(1.0);
            let (r0, _, _, _) = c0.to_rgba_f32();
            let (r1, _, b1, _) = c1.to_rgba_f32();
            assert!(r0 > 0.9, "sample(0.0) ≈ red preservado P264");
            assert!(r1 < 0.1 && b1 > 0.9, "sample(1.0) ≈ blue preservado P264");
        }
    }

    #[test]
    fn p269_radial_sample_inalterado_com_focal_explicit() {
        // sample(t) é 1D — focal arbitrário NÃO afecta output sample.
        // Esta é uma propriedade arquitectural cristalino vs vanilla
        // (vanilla tem sample_at(x,y) que usa focal; cristalino só tem
        // sample(t) 1D).
        let g_focal = Gradient::radial_with_focal(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
            Axes::new(Ratio(0.2), Ratio(0.3)), // focal arbitrário
            Ratio(0.1),
        );
        if let Gradient::Radial(r) = &g_focal {
            let c0 = r.sample(0.0);
            let c1 = r.sample(1.0);
            let (r0, _, _, _) = c0.to_rgba_f32();
            let (r1, _, b1, _) = c1.to_rgba_f32();
            // sample(t) ignora focal — endpoints idênticos default.
            assert!(r0 > 0.9, "sample(0.0) ≈ red ignora focal");
            assert!(r1 < 0.1 && b1 > 0.9, "sample(1.0) ≈ blue ignora focal");
        }
    }

    #[test]
    fn p269_radial_first_stop_color_inalterado_focal() {
        // first_stop_color é independente de focal.
        let g = Gradient::radial_with_focal(
            vec![
                GradientStop::new(Color::rgb(123, 45, 67), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 0), Ratio(1.0)),
            ],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
            Axes::new(Ratio(0.1), Ratio(0.2)),
            Ratio(0.05),
        );
        let c = g.first_stop_color();
        let (r, _g, _b, _) = c.to_rgba_f32();
        assert!((r * 255.0 - 123.0).abs() < 1.5, "first_stop_color ignora focal");
    }

    // ── P267 (ADR-0089 Gradient Conic-only) ────────────────────────────

    #[test]
    fn p267_conic_construcao_2_stops() {
        let g = Gradient::conic(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Angle::deg(0.0),
        );
        if let Gradient::Conic(c) = &g {
            assert_eq!(c.stops.len(), 2);
            assert_eq!(c.center.x, Ratio(0.5));
            assert_eq!(c.angle.to_rad(), 0.0);
        } else {
            panic!("esperado Gradient::Conic");
        }
    }

    #[test]
    fn p267_conic_first_stop_color() {
        let g = Gradient::conic(
            vec![
                GradientStop::new(Color::rgb(200, 100, 50), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 0), Ratio(1.0)),
            ],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Angle::deg(0.0),
        );
        assert_eq!(g.first_stop_color(), Color::rgb(200, 100, 50));
    }

    #[test]
    fn p267_conic_clone_arc_o1() {
        let g = Gradient::conic(
            vec![GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0))],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Angle::deg(0.0),
        );
        let g2 = g.clone();
        if let (Gradient::Conic(c1), Gradient::Conic(c2)) = (&g, &g2) {
            assert!(Arc::ptr_eq(c1, c2), "Arc clone deve partilhar storage");
        }
    }

    #[test]
    fn p267_conic_partial_eq() {
        let g1 = Gradient::conic(
            vec![GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0))],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Angle::deg(0.0),
        );
        let g2 = Gradient::conic(
            vec![GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0))],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Angle::deg(0.0),
        );
        let g3 = Gradient::conic(
            vec![GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0))],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Angle::deg(90.0),
        );
        assert_eq!(g1, g2);
        assert_ne!(g1, g3);
    }

    #[test]
    fn p267_conic_effective_offsets_auto_spacing() {
        let c = Conic {
            stops: Arc::from(vec![
                GradientStop::unspaced(Color::rgb(255, 0, 0)),
                GradientStop::unspaced(Color::rgb(0, 255, 0)),
                GradientStop::unspaced(Color::rgb(0, 0, 255)),
            ]),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            angle: Angle::deg(0.0),
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
        };
        let offs = c.effective_offsets();
        assert!((offs[0] - 0.0).abs() < 1e-5);
        assert!((offs[1] - 0.5).abs() < 1e-5);
        assert!((offs[2] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn p267_conic_sample_extremos() {
        let c = Conic {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            angle: Angle::deg(0.0),
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
        };
        let c0 = c.sample(0.0);
        let c1 = c.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (r1, _, b1, _) = c1.to_rgba_f32();
        assert!(r0 > 0.9, "sample(0.0) ≈ vermelho, r={}", r0);
        assert!(r1 < 0.1 && b1 > 0.9, "sample(1.0) ≈ azul, r={}, b={}", r1, b1);
    }

    #[test]
    fn p267_conic_sample_clamp_above_1() {
        let c = Conic {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            angle: Angle::deg(0.0),
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
        };
        let c_clamp = c.sample(1.5);
        let c_ref = c.sample(1.0);
        assert_eq!(c_clamp, c_ref);
    }

    #[test]
    fn p267_gradient_conic_to_paint_via_from() {
        use crate::entities::paint::Paint;
        let g = Gradient::conic(
            vec![GradientStop::new(Color::rgb(0, 0, 0), Ratio(0.0))],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Angle::deg(0.0),
        );
        let p: Paint = g.into();
        assert!(matches!(p, Paint::Gradient(Gradient::Conic(_))));
    }

    #[test]
    fn p267_conic_angle_non_default() {
        let c = Conic {
            stops: Arc::from(vec![
                GradientStop::new(Color::rgb(0, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(255, 255, 255), Ratio(1.0)),
            ]),
            center: Axes::new(Ratio(0.25), Ratio(0.75)),
            angle: Angle::deg(90.0),
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
        };
        assert_eq!(c.center.x, Ratio(0.25));
        assert_eq!(c.center.y, Ratio(0.75));
        assert!((c.angle.to_rad() - std::f64::consts::FRAC_PI_2).abs() < 1e-9);
    }

    // ── P270 (ADR-0091 EM VIGOR — ColorSpace runtime cross-variant) ──

    fn red_blue_stops() -> Vec<GradientStop> {
        vec![
            GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
            GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
        ]
    }

    // ── Hue-wrap shorter helper (4 tests) ──

    #[test]
    fn p270_hue_shorter_no_wrap() {
        // diff < 180°: caminho directo.
        // h0=0, h1=90, t=0.5 → 45.
        let h = interpolate_hue_shorter(0.0, 90.0, 0.5);
        assert!((h - 45.0).abs() < 1e-3, "no-wrap: got {}", h);
    }

    #[test]
    fn p270_hue_shorter_wrap_positive_to_negative() {
        // diff > 180° positivo → wrap pelo lado negativo.
        // h0=10, h1=350, diff=340; wrap: h1=350-360=-10; t=0.5 → 0.
        let h = interpolate_hue_shorter(10.0, 350.0, 0.5);
        // Resultado esperado: caminho curto via 0°; (10 + (-10-10)*0.5)=0 mod 360 = 0.
        assert!(
            (h - 0.0).abs() < 1e-3 || (h - 360.0).abs() < 1e-3,
            "wrap positive: got {}",
            h
        );
    }

    #[test]
    fn p270_hue_shorter_wrap_negative_to_positive() {
        // diff < -180° → wrap pelo lado positivo.
        // h0=350, h1=10, diff=-340; wrap: h1=10+360=370; t=0.5 → 360 mod 360 = 0.
        let h = interpolate_hue_shorter(350.0, 10.0, 0.5);
        assert!(
            (h - 0.0).abs() < 1e-3 || (h - 360.0).abs() < 1e-3,
            "wrap negative: got {}",
            h
        );
    }

    #[test]
    fn p270_hue_shorter_exactly_180() {
        // Edge case diff == 180° → wrap fica em sentido positivo
        // (CSS default; implementação cristalina: condição estrita > 180° não dispara
        // para exactly 180°, então caminho directo é usado).
        let h = interpolate_hue_shorter(0.0, 180.0, 0.5);
        assert!((h - 90.0).abs() < 1e-3, "exactly_180: got {}", h);
    }

    // ── L1 sample multi-space — 8 spaces × 3 variants = 24 tests ──

    fn make_linear_with_space(space: ColorSpace) -> Linear {
        Linear {
            stops: Arc::from(red_blue_stops()),
            angle: Angle::rad(0.0),
            space,
            relative: None,
            anti_alias: true,
        }
    }
    fn make_radial_with_space(space: ColorSpace) -> Radial {
        Radial {
            stops: Arc::from(red_blue_stops()),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            radius: Ratio(0.5),
            focal_center: Axes::new(Ratio(0.5), Ratio(0.5)),
            focal_radius: Ratio(0.0),
            space,
            relative: None,
            anti_alias: true,
        }
    }
    fn make_conic_with_space(space: ColorSpace) -> Conic {
        Conic {
            stops: Arc::from(red_blue_stops()),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            angle: Angle::rad(0.0),
            space,
            relative: None,
            anti_alias: true,
        }
    }

    // Linear

    #[test]
    fn p270_linear_sample_oklab_preserva_p262() {
        // Default Oklab — preserva P262 (red↔blue endpoints).
        let l = make_linear_with_space(ColorSpace::Oklab);
        let c0 = l.sample(0.0);
        let c1 = l.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (r1, _, b1, _) = c1.to_rgba_f32();
        assert!(r0 > 0.9, "sample(0.0).r ≈ 1.0; got {}", r0);
        assert!(r1 < 0.1 && b1 > 0.9, "sample(1.0) ≈ blue; got r={}, b={}", r1, b1);
    }

    #[test]
    fn p1253_linear_oklab_endpoint_components_palette() {
        let red = Color::Srgb { r: 1.0, g: 0.254902, b: 0.211765, a: 1.0 };
        let gradient = Linear {
            stops: Arc::from([
                GradientStop::new(red, Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]),
            angle: Angle::rad(0.0),
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
        };
        let Color::Oklab { l, a, b, alpha } = gradient.sample(0.0) else {
            panic!("sample Oklab deve devolver Color::Oklab");
        };
        assert!((l - 0.6595).abs() < 0.00005, "L público arredondado: {l}");
        assert_eq!(a.to_bits(), 0.199_735_3_f32.to_bits());
        assert_eq!(b.to_bits(), 0.108_188_12_f32.to_bits());
        assert_eq!(alpha.to_bits(), 1.0_f32.to_bits());
    }

    #[test]
    fn p270_linear_sample_srgb_endpoints() {
        let l = make_linear_with_space(ColorSpace::Srgb);
        let c0 = l.sample(0.0);
        let c1 = l.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (_, _, b1, _) = c1.to_rgba_f32();
        assert!(r0 > 0.9 && b1 > 0.9);
    }

    #[test]
    fn p270_linear_sample_linear_rgb_endpoints() {
        let l = make_linear_with_space(ColorSpace::LinearRgb);
        let c0 = l.sample(0.0);
        let c1 = l.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (_, _, b1, _) = c1.to_rgba_f32();
        assert!(r0 > 0.9 && b1 > 0.9);
    }

    #[test]
    fn p270_linear_sample_luma_endpoints() {
        let l = make_linear_with_space(ColorSpace::Luma);
        let c0 = l.sample(0.0);
        let c1 = l.sample(1.0);
        // red→blue em Luma → grayscale: ambos endpoints próximos a 0 (red=0.21 luma; blue=0.07).
        // Confirma que sample produz cores válidas (no panic).
        let _ = (c0.to_rgba_f32(), c1.to_rgba_f32());
    }

    #[test]
    fn p270_linear_sample_oklch_shorter_hue_endpoints() {
        let l = make_linear_with_space(ColorSpace::Oklch);
        let c0 = l.sample(0.0);
        let c1 = l.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (_, _, b1, _) = c1.to_rgba_f32();
        assert!(r0 > 0.7, "Oklch sample(0.0) ≈ red; r={}", r0);
        assert!(b1 > 0.7, "Oklch sample(1.0) ≈ blue; b={}", b1);
    }

    #[test]
    fn p270_linear_sample_hsl_endpoints() {
        let l = make_linear_with_space(ColorSpace::Hsl);
        let c0 = l.sample(0.0);
        let c1 = l.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (_, _, b1, _) = c1.to_rgba_f32();
        assert!(r0 > 0.7);
        assert!(b1 > 0.7);
    }

    #[test]
    fn p270_linear_sample_hsv_endpoints() {
        let l = make_linear_with_space(ColorSpace::Hsv);
        let c0 = l.sample(0.0);
        let c1 = l.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (_, _, b1, _) = c1.to_rgba_f32();
        assert!(r0 > 0.7);
        assert!(b1 > 0.7);
    }

    #[test]
    fn p270_linear_sample_cmyk_endpoints() {
        let l = make_linear_with_space(ColorSpace::Cmyk);
        let c0 = l.sample(0.0);
        let c1 = l.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (_, _, b1, _) = c1.to_rgba_f32();
        // CMYK red ≈ (0,1,1,0) → rgb≈(1,0,0); CMYK blue ≈ (1,1,0,0) → rgb≈(0,0,1).
        assert!(r0 > 0.7);
        assert!(b1 > 0.7);
    }

    // Radial

    #[test]
    fn p270_radial_sample_oklab_preserva_p264() {
        let r = make_radial_with_space(ColorSpace::Oklab);
        let c0 = r.sample(0.0);
        let c1 = r.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (_, _, b1, _) = c1.to_rgba_f32();
        assert!(r0 > 0.9 && b1 > 0.9);
    }

    #[test]
    fn p270_radial_sample_srgb() {
        let r = make_radial_with_space(ColorSpace::Srgb);
        let _ = r.sample(0.5);
    }

    #[test]
    fn p270_radial_sample_linear_rgb() {
        let r = make_radial_with_space(ColorSpace::LinearRgb);
        let _ = r.sample(0.5);
    }

    #[test]
    fn p270_radial_sample_luma() {
        let r = make_radial_with_space(ColorSpace::Luma);
        let _ = r.sample(0.5);
    }

    #[test]
    fn p270_radial_sample_oklch() {
        let r = make_radial_with_space(ColorSpace::Oklch);
        let _ = r.sample(0.5);
    }

    #[test]
    fn p270_radial_sample_hsl() {
        let r = make_radial_with_space(ColorSpace::Hsl);
        let _ = r.sample(0.5);
    }

    #[test]
    fn p270_radial_sample_hsv() {
        let r = make_radial_with_space(ColorSpace::Hsv);
        let _ = r.sample(0.5);
    }

    #[test]
    fn p270_radial_sample_cmyk() {
        let r = make_radial_with_space(ColorSpace::Cmyk);
        let _ = r.sample(0.5);
    }

    // Conic

    #[test]
    fn p270_conic_sample_oklab_preserva_p267() {
        let c = make_conic_with_space(ColorSpace::Oklab);
        let c0 = c.sample(0.0);
        let c1 = c.sample(1.0);
        let (r0, _, _, _) = c0.to_rgba_f32();
        let (_, _, b1, _) = c1.to_rgba_f32();
        assert!(r0 > 0.9 && b1 > 0.9);
    }

    #[test]
    fn p270_conic_sample_srgb() {
        let c = make_conic_with_space(ColorSpace::Srgb);
        let _ = c.sample(0.5);
    }

    #[test]
    fn p270_conic_sample_linear_rgb() {
        let c = make_conic_with_space(ColorSpace::LinearRgb);
        let _ = c.sample(0.5);
    }

    #[test]
    fn p270_conic_sample_luma() {
        let c = make_conic_with_space(ColorSpace::Luma);
        let _ = c.sample(0.5);
    }

    #[test]
    fn p270_conic_sample_oklch() {
        let c = make_conic_with_space(ColorSpace::Oklch);
        let _ = c.sample(0.5);
    }

    #[test]
    fn p270_conic_sample_hsl() {
        let c = make_conic_with_space(ColorSpace::Hsl);
        let _ = c.sample(0.5);
    }

    #[test]
    fn p270_conic_sample_hsv() {
        let c = make_conic_with_space(ColorSpace::Hsv);
        let _ = c.sample(0.5);
    }

    #[test]
    fn p270_conic_sample_cmyk() {
        let c = make_conic_with_space(ColorSpace::Cmyk);
        let _ = c.sample(0.5);
    }

    // Construtores defaults preservam bit-exact P262/P264/P267

    #[test]
    fn p270_linear_default_construtor_space_oklab() {
        let g = Gradient::linear(red_blue_stops(), Angle::rad(0.0));
        if let Gradient::Linear(l) = g {
            assert_eq!(
                l.space,
                ColorSpace::Oklab,
                "Gradient::linear default deve ser ColorSpace::Oklab"
            );
        } else {
            panic!("expected Linear");
        }
    }

    #[test]
    fn p270_radial_default_construtor_space_oklab() {
        let g = Gradient::radial(
            red_blue_stops(),
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
        );
        if let Gradient::Radial(r) = g {
            assert_eq!(r.space, ColorSpace::Oklab);
        } else {
            panic!("expected Radial");
        }
    }

    #[test]
    fn p270_conic_default_construtor_space_oklab() {
        let g = Gradient::conic(
            red_blue_stops(),
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Angle::rad(0.0),
        );
        if let Gradient::Conic(c) = g {
            assert_eq!(c.space, ColorSpace::Oklab);
        } else {
            panic!("expected Conic");
        }
    }

    #[test]
    fn p270_linear_with_space_explicit() {
        let g = Gradient::linear_with_space(
            red_blue_stops(),
            Angle::rad(0.0),
            ColorSpace::Hsl,
        );
        if let Gradient::Linear(l) = g {
            assert_eq!(l.space, ColorSpace::Hsl);
        } else {
            panic!("expected Linear");
        }
    }
}
