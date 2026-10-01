//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/color/mixing.md
//! @prompt-hash b4b9a199
//! @layer L1
//! @updated 2026-09-30
//!
//! Interpolação e redução ponderada entre cores.

use super::*;

impl Color {
    /// **P744** — converte a cor para um vec4 no espaço indicado. Usado por
    /// `mix` (paridade vanilla `ProcessColor::to_vec4`).
    fn to_vec4_in_space(self, space: ColorSpace) -> [f32; 4] {
        let c = if self.space() == space {
            self.clone()
        } else {
            self.clone().to_space(space).unwrap_or_else(|_| self.to_process())
        };
        match c {
            Color::Srgb { r, g, b, a } => [r, g, b, a],
            Color::Luma { l, a } => [l, 0.0, 0.0, a],
            Color::LinearRgb { r, g, b, a } => [r, g, b, a],
            Color::Oklab { l, a, b, alpha } => [l, a, b, alpha],
            Color::Oklch { l, c, h, alpha } => [l, c, h, alpha],
            Color::Hsl { h, s, l, a } => [h, s, l, a],
            Color::Hsv { h, s, v, a } => [h, s, v, a],
            Color::Cmyk { c, m, y, k } => [c, m, y, k],
            Color::Spot(spot) => [spot.tint.get() as f32, 0.0, 0.0, 1.0],
        }
    }

    /// **P744** — constrói uma cor a partir de um vec4 no espaço indicado.
    fn from_vec4_in_space(v: [f32; 4], space: ColorSpace) -> Self {
        match space {
            ColorSpace::Srgb => Color::srgb_f32(v[0], v[1], v[2], v[3]),
            ColorSpace::Luma => Color::Luma { l: v[0], a: v[3] },
            ColorSpace::LinearRgb => Color::linear_rgb(v[0], v[1], v[2], v[3]),
            ColorSpace::Oklab => Color::oklab(v[0], v[1], v[2], v[3]),
            ColorSpace::Oklch => Color::oklch(v[0], v[1], v[2].rem_euclid(360.0), v[3]),
            ColorSpace::Hsl => Color::hsl(v[0].rem_euclid(360.0), v[1], v[2], v[3]),
            ColorSpace::Hsv => Color::hsv(v[0].rem_euclid(360.0), v[1], v[2], v[3]),
            ColorSpace::Cmyk => Color::cmyk(v[0], v[1], v[2], v[3]),
            ColorSpace::Spot(colorant) => Color::Spot(SpotColor {
                colorant: Arc::new(colorant),
                tint: Ratio(v[0] as f64),
            }),
        }
    }

    /// Interpolação linear entre `self` e `other` no espaço indicado.
    /// `weight` [0.0, 1.0]: 0.0 = self; 1.0 = other.
    /// Default `space: None` = Oklab (paridade P476/P742). O resultado fica
    /// no espaço indicado (P744).
    pub fn mix(self, other: Self, weight: f32, space: Option<ColorSpace>) -> Self {
        let space = space.unwrap_or_else(|| match (&self, &other) {
            (Color::Spot(a), Color::Spot(b)) if a.colorant == b.colorant => {
                ColorSpace::Spot((*a.colorant).clone())
            }
            _ => ColorSpace::Oklab,
        });
        let mut c0 = self.to_vec4_in_space(space.clone());
        let mut c1 = other.to_vec4_in_space(space.clone());
        let t = weight.clamp(0.0, 1.0);

        // Para espaços com hue, percorre o círculo cromático pelo caminho
        // mais curto (paridade vanilla `mix_iter` hue_index).
        let hue_idx = match &space {
            ColorSpace::Oklch => Some(2),
            ColorSpace::Hsl | ColorSpace::Hsv => Some(0),
            _other => None, // neutro: N16[β] — espaços cromáticos sem componente angular/hue não necessitam de correcção circular
        };
        if let Some(idx) = hue_idx {
            if (c0[idx] - c1[idx]).abs() > 180.0 {
                if c0[idx] < c1[idx] {
                    c0[idx] += 360.0;
                } else {
                    c1[idx] += 360.0;
                }
            }
        }

        let mixed = [
            c0[0] + (c1[0] - c0[0]) * t,
            c0[1] + (c1[1] - c0[1]) * t,
            c0[2] + (c1[2] - c0[2]) * t,
            c0[3] + (c1[3] - c0[3]) * t,
        ];
        Color::from_vec4_in_space(mixed, space)
    }

    /// Redução ponderada usada pela superfície variádica de `color.mix`.
    /// Permanece interna ao crate; a API pública binária acima não muda.
    pub(crate) fn mix_weighted(
        colors: &[(Self, f32)],
        space: Option<ColorSpace>,
    ) -> Option<Self> {
        let total: f32 = colors.iter().map(|(_, weight)| *weight).sum();
        if total <= 0.0 || !total.is_finite() {
            return None;
        }
        if colors.is_empty() {
            return None;
        }
        let space = space.unwrap_or_else(|| {
            match &colors[0].0 {
            Color::Spot(first) if colors.iter().all(|(color, _)| {
                matches!(color, Color::Spot(spot) if spot.colorant == first.colorant)
            }) => ColorSpace::Spot((*first.colorant).clone()),
            _ => ColorSpace::Oklab,
        }
        });
        let hue_idx = match &space {
            ColorSpace::Oklch => Some(2),
            ColorSpace::Hsl | ColorSpace::Hsv => Some(0),
            _ => None,
        };
        if hue_idx.is_some() && colors.len() > 2 {
            return None;
        }
        if let Some(idx) = hue_idx {
            if colors.len() == 1 {
                return colors[0].0.clone().to_space(space).ok();
            }
            let mut first = colors[0].0.clone().to_vec4_in_space(space.clone());
            let mut second = colors[1].0.clone().to_vec4_in_space(space.clone());
            if (first[idx] - second[idx]).abs() > 180.0 {
                if first[idx] < second[idx] {
                    first[idx] += 360.0;
                } else {
                    second[idx] += 360.0;
                }
            }
            let mixed = [0, 1, 2, 3].map(|component| {
                (first[component] * colors[0].1 + second[component] * colors[1].1) / total
            });
            return Some(Self::from_vec4_in_space(mixed, space));
        }

        let mut mixed = [0.0_f32; 4];
        for (color, weight) in colors {
            let components = color.clone().to_vec4_in_space(space.clone());
            for index in 0..4 {
                mixed[index] += components[index] * *weight / total;
            }
        }
        Some(Self::from_vec4_in_space(mixed, space))
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}
