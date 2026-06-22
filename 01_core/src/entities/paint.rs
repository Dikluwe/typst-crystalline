//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/paint.md
//! @prompt-hash f9855284
//! @layer L1
//! @updated 2026-05-15
//!
//! **P261 (M9d / M7+5; ADR-0086 PROPOSTO Paint wrapper Solid only;
//! cita ADR-0083 P257 precedente N=2 do mesmo pattern)** — wrapper
//! enum sobre fontes de cor (`Color` directo apenas; variants
//! `Gradient`/`Tiling` comentários reserva). Substitui
//! `Stroke.paint: Color` (P252) por `Stroke.paint: Paint`. Abre
//! caminho para Gradient real consumer em P262+.
//!
//! **Decisão minimalista** (paridade P25 → P257 Color pattern):
//! Solid only inicialmente; expansão consumer-driven.
//!
//! **ADR-0039 preservado**: `TextStyle.fill: Option<Color>`
//! inalterado. P261 toca apenas `Stroke.paint`.
//!
//! Cross-references:
//! - Vanilla `lab/typst-original/.../visualize/paint.rs` (3 variants).
//! - ADR-0086 — Paint wrapper Solid only (IMPLEMENTADO P261).
//! - ADR-0083 — Color paridade vanilla (precedente N=2).

use crate::entities::color::Color;
use crate::entities::gradient::Gradient;
use crate::entities::tiling::Tiling;

/// Wrapper enum sobre fontes de cor.
///
/// **P262 (ADR-0087)** — `Paint::Gradient(Gradient)` variant
/// activada via ADR-0086 §"Critério revisão" cumprido.
/// `Copy` removido — Gradient não é Copy (Arc<Linear>).
///
/// Per ADR-0086 + ADR-0087 + P395, materializa `Solid(Color)` +
/// `Gradient(Gradient)` e `Tiling(Tiling)`.
#[derive(Debug, Clone, PartialEq)]
pub enum Paint {
    /// Cor sólida (uniforme). Materializado P261.
    Solid(Color),
    /// Gradient (Linear only por agora; ADR-0087).
    Gradient(Gradient),
    /// Tiling / pattern fill. Materializado P395; consumer real é scope-out
    /// ADR-0054 graded — fallback a cor representativa.
    Tiling(Tiling),
}

impl Paint {
    /// Construtor `Solid` ergonómico.
    pub fn solid(c: Color) -> Self {
        Paint::Solid(c)
    }

    /// Construtor `Gradient` ergonómico.
    pub fn gradient(g: Gradient) -> Self {
        Paint::Gradient(g)
    }

    /// Construtor `Tiling` ergonómico.
    pub fn tiling(t: Tiling) -> Self {
        Paint::Tiling(t)
    }

    /// Extrai uma `Color` representativa.
    ///
    /// - `Solid` → cor literal.
    /// - `Gradient` → primeiro stop.
    /// - `Tiling` → fallback definido por `Tiling::to_color()`.
    pub fn to_color(&self) -> Color {
        match self {
            Paint::Solid(c) => *c,
            Paint::Gradient(g) => g.first_stop_color(),
            Paint::Tiling(t) => t.to_color(),
        }
    }
}

impl From<Color> for Paint {
    fn from(c: Color) -> Self {
        Paint::Solid(c)
    }
}

impl From<Gradient> for Paint {
    fn from(g: Gradient) -> Self {
        Paint::Gradient(g)
    }
}

impl From<Tiling> for Paint {
    fn from(t: Tiling) -> Self {
        Paint::Tiling(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paint_solid_construcao() {
        let c = Color::rgb(255, 0, 0);
        let p = Paint::solid(c);
        assert_eq!(p, Paint::Solid(c));
    }

    #[test]
    fn paint_to_color_solid() {
        let c = Color::rgb(0, 128, 255);
        assert_eq!(Paint::Solid(c).to_color(), c);
    }

    #[test]
    fn paint_to_color_solid_via_solid_helper() {
        let c = Color::rgb(64, 64, 64);
        assert_eq!(Paint::solid(c).to_color(), c);
    }

    #[test]
    fn paint_from_color() {
        let c = Color::rgb(64, 64, 64);
        let p: Paint = c.into();
        assert_eq!(p, Paint::Solid(c));
    }

    #[test]
    fn paint_partial_eq() {
        let p1 = Paint::Solid(Color::rgb(1, 2, 3));
        let p2 = Paint::Solid(Color::rgb(1, 2, 3));
        let p3 = Paint::Solid(Color::rgb(4, 5, 6));
        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }

    #[test]
    fn paint_clone() {
        // P262 — Paint deixa de ser Copy (Gradient via Arc).
        // Apenas Clone preservado.
        let p1 = Paint::Solid(Color::rgb(10, 20, 30));
        let p2 = p1.clone();
        assert_eq!(p1, p2);
        let p3 = p2.clone();
        assert_eq!(p1, p3);
    }

    #[test]
    fn paint_debug_format() {
        let p = Paint::Solid(Color::rgb(0, 0, 0));
        let s = format!("{:?}", p);
        assert!(s.contains("Solid"));
    }

    // ── P395 — Tiling integration ────────────────────────────────────────────

    #[test]
    fn paint_tiling_variant_and_to_color() {
        use crate::entities::tiling::{Tiling, TilingBody};
        let t = Tiling::new(TilingBody::Color(Color::rgb(0, 128, 255)));
        let p = Paint::tiling(t);
        assert_eq!(p.to_color(), Color::rgb(0, 128, 255));
    }

    #[test]
    fn paint_tiling_from_tiling() {
        use crate::entities::tiling::{Tiling, TilingBody};
        let t = Tiling::new(TilingBody::Color(Color::rgb(255, 0, 0)));
        let p: Paint = t.clone().into();
        assert_eq!(p.to_color(), Color::rgb(255, 0, 0));
    }
}
