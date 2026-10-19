//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/color/adjustments.md
//! @prompt-hash 49ba0bfe
//! @layer L1
//! @updated 2026-09-30
//!
//! Ajustes cromáticos e de alpha.

use super::*;

/// **P742** — fórmula `increase` do palette 0.7.6
/// (`macros/lighten_saturate.rs`): para `f >= 0`, `c + max(0, 1-c) * f`;
/// para `f < 0`, `c + max(0, c) * f`. Clamp [0, 1].
fn increase_p742(c: f32, f: f32) -> f32 {
    let difference = if f >= 0.0 { (1.0 - c).max(0.0) } else { c.max(0.0) };
    (c + difference * f).clamp(0.0, 1.0)
}

impl Color {
    /// **P742** — aumenta luminância por `factor` **no espaço da própria
    /// cor** (paridade palette `Lighten::lighten` medida): `increase` sobre
    /// os canais de estímulo (Srgb/LinearRgb: r,g,b; Luma/Oklab/Oklch/Hsl:
    /// l; Hsv: v). Cmyk (tipo próprio do vanilla, `color.rs:2235`):
    /// `u - u*factor` por componente. Alpha preservado.
    pub fn lighten(self, factor: f32) -> Self {
        match self {
            Color::Srgb { r, g, b, a } => Color::srgb_f32(
                increase_p742(r, factor),
                increase_p742(g, factor),
                increase_p742(b, factor),
                a,
            ),
            Color::Luma { l, a } => Color::Luma { l: increase_p742(l, factor), a },
            Color::LinearRgb { r, g, b, a } => Color::linear_rgb(
                increase_p742(r, factor),
                increase_p742(g, factor),
                increase_p742(b, factor),
                a,
            ),
            Color::Oklab { l, a, b, alpha } => {
                Color::oklab(increase_p742(l, factor), a, b, alpha)
            }
            Color::Oklch { l, c, h, alpha } => {
                Color::oklch(increase_p742(l, factor), c, h, alpha)
            }
            Color::Hsl { h, s, l, a } => Color::hsl(h, s, increase_p742(l, factor), a),
            Color::Hsv { h, s, v, a } => Color::hsv(h, s, increase_p742(v, factor), a),
            Color::Spot(mut spot) => {
                let tint = spot.tint.get();
                spot.tint = Ratio((tint - tint * factor as f64).clamp(0.0, 1.0));
                Color::Spot(spot)
            }
            Color::Cmyk { c, m, y, k } => {
                let f = |u: f32| (u - u * factor).clamp(0.0, 1.0);
                Color::cmyk(f(c), f(m), f(y), f(k))
            }
        }
    }

    /// **P742** — diminui luminância por `factor`: `lighten(-factor)` para
    /// as variantes palette (paridade `Darken::darken`); Cmyk:
    /// `u + (1-u)*factor` por componente (`color.rs:2240`).
    pub fn darken(self, factor: f32) -> Self {
        match self {
            Color::Spot(mut spot) => {
                let tint = spot.tint.get();
                spot.tint = Ratio((tint + (1.0 - tint) * factor as f64).clamp(0.0, 1.0));
                Color::Spot(spot)
            }
            Color::Cmyk { c, m, y, k } => {
                let f = |u: f32| (u + (1.0 - u) * factor).clamp(0.0, 1.0);
                Color::cmyk(f(c), f(m), f(y), f(k))
            }
            Color::Srgb { .. }
            | Color::Luma { .. }
            | Color::LinearRgb { .. }
            | Color::Oklab { .. }
            | Color::Oklch { .. }
            | Color::Hsl { .. }
            | Color::Hsv { .. } => self.lighten(-factor),
        }
    }

    /// **P744** — negação no espaço indicado (default Oklab). Converte para o
    /// espaço, aplica a negação própria desse espaço, converte de volta ao
    /// espaço original de `self`.
    pub fn negate(self, space: Option<ColorSpace>) -> Self {
        if let Color::Spot(mut spot) = self.clone() {
            spot.tint = Ratio(1.0 - spot.tint.get());
            return Color::Spot(spot);
        }
        let original = self.space();
        let space = space.unwrap_or(ColorSpace::Oklab);
        let converted =
            self.clone().to_space(space).unwrap_or_else(|_| self.to_process());
        let negated = match converted {
            Color::Srgb { r, g, b, a } => Color::srgb_f32(1.0 - r, 1.0 - g, 1.0 - b, a),
            Color::Luma { l, a } => Color::Luma { l: 1.0 - l, a },
            Color::LinearRgb { r, g, b, a } => {
                Color::linear_rgb(1.0 - r, 1.0 - g, 1.0 - b, a)
            }
            Color::Oklab { l, a, b, alpha } => Color::oklab(1.0 - l, -a, -b, alpha),
            Color::Oklch { l, c, h, alpha } => Color::oklch(1.0 - l, c, h + 180.0, alpha),
            Color::Hsl { h, s, l, a } => Color::hsl(h + 180.0, s, l, a),
            Color::Hsv { h, s, v, a } => Color::hsv(h + 180.0, s, v, a),
            Color::Cmyk { c, m, y, k } => Color::cmyk(1.0 - c, 1.0 - m, 1.0 - y, k),
            Color::Spot(spot) => return Color::Spot(spot),
        };
        negated.clone().to_space(original).unwrap_or(negated)
    }

    /// **P744** — rotação de hue no espaço indicado (default Oklch). Só é
    /// válida em espaços com hue (Oklch, Hsl, Hsv); os restantes devolvem
    /// `None` para o chamador emitir o erro vanilla.
    pub fn rotate(self, angle_deg: f32, space: Option<ColorSpace>) -> Option<Self> {
        if matches!(&self, Color::Spot(_)) {
            return None;
        }
        let original = self.space();
        let space = space.unwrap_or(ColorSpace::Oklch);
        if !matches!(space, ColorSpace::Oklch | ColorSpace::Hsl | ColorSpace::Hsv) {
            return None;
        }
        let rotated = match self.to_space(space).ok()? {
            Color::Oklch { l, c, h, alpha } => Color::oklch(l, c, h + angle_deg, alpha),
            Color::Hsl { h, s, l, a } => Color::hsl(h + angle_deg, s, l, a),
            Color::Hsv { h, s, v, a } => Color::hsv(h + angle_deg, s, v, a),
            Color::Srgb { .. }
            | Color::Luma { .. }
            | Color::LinearRgb { .. }
            | Color::Oklab { .. }
            | Color::Cmyk { .. }
            | Color::Spot(_) => {
                unreachable!("to_space(hue-space) devolve sempre o espaço hue")
            }
        };
        rotated.to_space(original).ok()
    }

    /// **P742** — aumenta saturação por `factor` (paridade palette medida):
    /// Hsl/Hsv → `increase` sobre a saturação; restantes → converte para
    /// Hsv, `increase` sobre `s`, converte de volta ao espaço original.
    /// Luma → `None` (o vanilla erra "cannot saturate grayscale color" —
    /// o chamador em rules emite a mensagem verbatim com span).
    pub fn saturate(self, factor: f32) -> Option<Self> {
        match self {
            Color::Luma { .. } | Color::Spot(_) => None,
            Color::Hsl { h, s, l, a } => {
                Some(Color::hsl(h, increase_p742(s, factor), l, a))
            }
            Color::Hsv { h, s, v, a } => {
                Some(Color::hsv(h, increase_p742(s, factor), v, a))
            }
            Color::Srgb { .. }
            | Color::LinearRgb { .. }
            | Color::Oklab { .. }
            | Color::Oklch { .. }
            | Color::Cmyk { .. } => {
                let original = self.space();
                let Color::Hsv { h, s, v, a } = self.to_space(ColorSpace::Hsv).ok()?
                else {
                    return None;
                };
                Color::hsv(h, increase_p742(s, factor), v, a).to_space(original).ok()
            }
        }
    }

    /// **P742** — diminui saturação por `factor` (paridade vanilla:
    /// `saturate(-factor)`). Luma → `None`.
    pub fn desaturate(self, factor: f32) -> Option<Self> {
        self.saturate(-factor)
    }

    /// **P744** — ajusta o alpha por `scale`. `scale > 0` → opacify;
    /// `scale < 0` → transparentize. Fórmula vanilla (`scale_alpha`):
    /// `alpha' = alpha + scale * (scale > 0 ? 1 - alpha : alpha)`.
    fn scale_alpha(self, scale: f32) -> Option<Self> {
        let factor = if scale > 0.0 { 1.0 - self.alpha() } else { self.alpha() };
        let alpha = self.alpha();
        self.with_alpha((alpha + scale * factor).clamp(0.0, 1.0))
    }

    /// Alpha da cor (Cmyk não tem → `None` no caller).
    fn alpha(&self) -> f32 {
        match self.clone() {
            Color::Srgb { a, .. } => a,
            Color::Luma { a, .. } => a,
            Color::LinearRgb { a, .. } => a,
            Color::Oklab { alpha, .. } => alpha,
            Color::Oklch { alpha, .. } => alpha,
            Color::Hsl { a, .. } => a,
            Color::Hsv { a, .. } => a,
            Color::Cmyk { .. } | Color::Spot(_) => 1.0,
        }
    }

    /// Cria cópia da cor com novo alpha (Cmyk não tem alpha → `None`).
    fn with_alpha(self, alpha: f32) -> Option<Self> {
        Some(match self {
            Color::Srgb { r, g, b, .. } => Color::srgb_f32(r, g, b, alpha),
            Color::Luma { l, .. } => Color::Luma { l, a: alpha },
            Color::LinearRgb { r, g, b, .. } => Color::linear_rgb(r, g, b, alpha),
            Color::Oklab { l, a, b, .. } => Color::oklab(l, a, b, alpha),
            Color::Oklch { l, c, h, .. } => Color::oklch(l, c, h, alpha),
            Color::Hsl { h, s, l, .. } => Color::hsl(h, s, l, alpha),
            Color::Hsv { h, s, v, .. } => Color::hsv(h, s, v, alpha),
            Color::Cmyk { .. } | Color::Spot(_) => return None,
        })
    }

    /// **P744** — torna a cor mais transparente. `None` = Cmyk (sem alpha).
    pub fn transparentize(self, factor: f32) -> Option<Self> {
        self.scale_alpha(-factor)
    }

    /// **P744** — torna a cor mais opaca. `None` = Cmyk (sem alpha).
    pub fn opacify(self, factor: f32) -> Option<Self> {
        self.scale_alpha(factor)
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}
