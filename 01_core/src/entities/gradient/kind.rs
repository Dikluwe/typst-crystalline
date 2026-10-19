//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/gradient/kind.md
//! @prompt-hash 60f0a1ea
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Gradient — enum tagged paridade vanilla.
#[derive(Debug, Clone, PartialEq)]
pub enum Gradient {
    Linear(Arc<Linear>),
    Radial(Arc<Radial>), // P264 — descomentado per ADR-0088
    Conic(Arc<Conic>),   // P267 — descomentado per ADR-0089 (cluster 3/3 completo)
}

impl Gradient {
    /// Construtor Linear (P262; ADR-0087).
    ///
    /// **P270**: default `space: ColorSpace::Oklab` (preserva P262
    /// behavior bit-exact).
    pub fn linear(stops: impl Into<Arc<[GradientStop]>>, angle: Angle) -> Self {
        Gradient::Linear(Arc::new(Linear {
            stops: stops.into(),
            angle,
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
        }))
    }

    /// Construtor Linear com ColorSpace explícito (P270; ADR-0091
    /// EM VIGOR — §ColorSpace runtime activado).
    pub fn linear_with_space(
        stops: impl Into<Arc<[GradientStop]>>,
        angle: Angle,
        space: ColorSpace,
    ) -> Self {
        Gradient::Linear(Arc::new(Linear {
            stops: stops.into(),
            angle,
            space,
            relative: None,
            anti_alias: true,
        }))
    }

    /// Construtor Radial (P264; ADR-0088).
    ///
    /// **P269**: defaults focal_center = center; focal_radius = 0.
    /// **P270**: default `space: ColorSpace::Oklab` (preserva P264/P269
    /// behavior bit-exact).
    pub fn radial(
        stops: impl Into<Arc<[GradientStop]>>,
        center: crate::entities::axes::Axes<Ratio>,
        radius: Ratio,
    ) -> Self {
        Gradient::Radial(Arc::new(Radial {
            stops: stops.into(),
            center,
            radius,
            focal_center: center,     // P269 default
            focal_radius: Ratio(0.0), // P269 default
            space: ColorSpace::Oklab, // P270 default
            relative: None,
            anti_alias: true,
        }))
    }

    /// Construtor Radial com focal_* explícito (P269).
    /// Default `space: Oklab` (P270).
    pub fn radial_with_focal(
        stops: impl Into<Arc<[GradientStop]>>,
        center: crate::entities::axes::Axes<Ratio>,
        radius: Ratio,
        focal_center: crate::entities::axes::Axes<Ratio>,
        focal_radius: Ratio,
    ) -> Self {
        Gradient::Radial(Arc::new(Radial {
            stops: stops.into(),
            center,
            radius,
            focal_center,
            focal_radius,
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
        }))
    }

    /// Construtor Radial com ColorSpace explícito (P270; ADR-0091
    /// EM VIGOR). Defaults focal=(center, 0) preserva P264/P269.
    pub fn radial_with_space(
        stops: impl Into<Arc<[GradientStop]>>,
        center: crate::entities::axes::Axes<Ratio>,
        radius: Ratio,
        space: ColorSpace,
    ) -> Self {
        Gradient::Radial(Arc::new(Radial {
            stops: stops.into(),
            center,
            radius,
            focal_center: center,
            focal_radius: Ratio(0.0),
            space,
            relative: None,
            anti_alias: true,
        }))
    }

    /// Construtor Conic (P267; ADR-0089).
    ///
    /// **P270**: default `space: ColorSpace::Oklab` (preserva P267
    /// behavior bit-exact).
    pub fn conic(
        stops: impl Into<Arc<[GradientStop]>>,
        center: crate::entities::axes::Axes<Ratio>,
        angle: Angle,
    ) -> Self {
        Gradient::Conic(Arc::new(Conic {
            stops: stops.into(),
            center,
            angle,
            space: ColorSpace::Oklab,
            relative: None,
            anti_alias: true,
        }))
    }

    /// Construtor Conic com ColorSpace explícito (P270; ADR-0091
    /// EM VIGOR).
    pub fn conic_with_space(
        stops: impl Into<Arc<[GradientStop]>>,
        center: crate::entities::axes::Axes<Ratio>,
        angle: Angle,
        space: ColorSpace,
    ) -> Self {
        Gradient::Conic(Arc::new(Conic {
            stops: stops.into(),
            center,
            angle,
            space,
            relative: None,
            anti_alias: true,
        }))
    }

    /// Retorna primeira cor do primeiro stop. Usado como
    /// fallback para `Paint::to_color()` quando consumer
    /// precisa de Color literal (Solid path).
    pub fn first_stop_color(&self) -> Color {
        match self {
            Gradient::Linear(l) => l
                .stops
                .first()
                .map(|s| s.color.clone())
                .unwrap_or(Color::rgb(0, 0, 0)),
            Gradient::Radial(r) => r
                .stops
                .first()
                .map(|s| s.color.clone())
                .unwrap_or(Color::rgb(0, 0, 0)),
            Gradient::Conic(c) => c
                .stops
                .first()
                .map(|s| s.color.clone())
                .unwrap_or(Color::rgb(0, 0, 0)),
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
        assert_eq!(gradient.first_stop_color(), Color::rgb(255, 0, 0));
    }
}
