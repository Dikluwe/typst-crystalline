//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/corners.md
//! @prompt-hash 27da7063
//! @layer L1
//! @updated 2026-05-14
//!
//! Tipo geométrico genérico `Corners<T>`
//! (top_left/top_right/bottom_right/bottom_left).
//!
//! Adicionado em P242 (M9d / M7+5, ADR-0081 IMPLEMENTADO parcial 3/5)
//! como suporte a `ShapeKind::RoundedRect { radii: Corners<Length> }`
//! e refino `Content::Block.radius` + `Content::Boxed.radius`
//! `Option<Length>` → `Corners<Length>` (per-corner). Reusable por
//! outras features Layout futuras (e.g. stroke per-corner).
//!
//! Estilo paralelo a `entities/sides.rs` (P156C), substituindo a
//! ordem `left/top/right/bottom` por sentido horário começando
//! top-left (paridade vanilla
//! `lab/typst-original/.../layout/corners.rs`).

/// Quatro cantos de um rectângulo
/// (top_left/top_right/bottom_right/bottom_left).
///
/// Genérico em `T` para suportar qualquer valor por canto:
/// - `Corners<Length>` para radius rounded-rect.
/// - `Corners<f64>` para offsets brutos.
/// - `Corners<bool>` para máscaras de presença per canto.
///
/// `Copy` derivado quando `T: Copy` (caso comum de `Length`).
///
/// **Ordem dos campos** sentido horário começando top-left
/// (paridade vanilla). Útil para iteração que percorre cantos em
/// sequência natural (e.g. Bezier path generation).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Corners<T> {
    pub top_left:     T,
    pub top_right:    T,
    pub bottom_right: T,
    pub bottom_left:  T,
}

impl<T> Corners<T> {
    /// Constrói `Corners` com cada canto independente.
    pub fn new(top_left: T, top_right: T, bottom_right: T, bottom_left: T) -> Self {
        Self { top_left, top_right, bottom_right, bottom_left }
    }
}

impl<T: Clone> Corners<T> {
    /// Constrói `Corners` com o mesmo valor em todos os cantos.
    pub fn uniform(value: T) -> Self {
        Self {
            top_left:     value.clone(),
            top_right:    value.clone(),
            bottom_right: value.clone(),
            bottom_left:  value,
        }
    }
}

impl<T: Default> Default for Corners<T> {
    fn default() -> Self {
        Self {
            top_left:     T::default(),
            top_right:    T::default(),
            bottom_right: T::default(),
            bottom_left:  T::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p242_corners_new_preserva_4_cantos() {
        let c = Corners::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(c.top_left,     1.0);
        assert_eq!(c.top_right,    2.0);
        assert_eq!(c.bottom_right, 3.0);
        assert_eq!(c.bottom_left,  4.0);
    }

    #[test]
    fn p242_corners_uniform_clona_valor() {
        let c = Corners::uniform(7.5_f64);
        assert_eq!(c.top_left,     7.5);
        assert_eq!(c.top_right,    7.5);
        assert_eq!(c.bottom_right, 7.5);
        assert_eq!(c.bottom_left,  7.5);
    }

    #[test]
    fn p242_corners_default_zero_em_todos_cantos() {
        let c: Corners<f64> = Corners::default();
        assert_eq!(c.top_left,     0.0);
        assert_eq!(c.top_right,    0.0);
        assert_eq!(c.bottom_right, 0.0);
        assert_eq!(c.bottom_left,  0.0);
    }

    #[test]
    fn p242_corners_clone_eq_partial_eq_funcionam() {
        let a = Corners::new(1, 2, 3, 4);
        let b = a;  // Copy
        let c = Corners::new(1, 2, 3, 5);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
