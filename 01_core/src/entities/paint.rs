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

/// Wrapper enum sobre fontes de cor.
///
/// Per ADR-0086, materializa apenas `Solid(Color)`; variants
/// `Gradient`/`Tiling` ficam como comentários reserva no enum
/// até P262+ activar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Paint {
    /// Cor sólida (uniforme). Único variant materializado P261.
    Solid(Color),
    // Gradient(Gradient),  // P262 — comentário reserva
    // Tiling(Tiling),      // futuro — comentário reserva
}

impl Paint {
    /// Construtor `Solid` ergonómico.
    pub fn solid(c: Color) -> Self {
        Paint::Solid(c)
    }

    /// Extrai a cor (sempre `Solid` per ADR-0086 §scope-outs).
    ///
    /// Substitui `unwrap_solid()` panicking de vanilla — em
    /// cristalino `Solid` é garantido pela enum estrutura
    /// (Gradient/Tiling não materializados).
    pub fn to_color(&self) -> Color {
        match self {
            Paint::Solid(c) => *c,
        }
    }
}

impl From<Color> for Paint {
    fn from(c: Color) -> Self {
        Paint::Solid(c)
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
    fn paint_copy_clone() {
        let p1 = Paint::Solid(Color::rgb(10, 20, 30));
        let p2 = p1;
        // p1 ainda usável → Copy
        assert_eq!(p1, p2);
        let p3 = p1.clone();
        assert_eq!(p1, p3);
    }

    #[test]
    fn paint_debug_format() {
        let p = Paint::Solid(Color::rgb(0, 0, 0));
        let s = format!("{:?}", p);
        assert!(s.contains("Solid"));
    }
}
