//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/sides.md
//! @prompt-hash c47b14e6
//! @layer L1
//! @updated 2026-04-25
//!
//! Tipo geométrico genérico `Sides<T>` (left/top/right/bottom).
//! Adicionado no Passo 156C (Fase 1 Layout, ADR-0061) como suporte
//! a `Content::Pad { padding: Sides<Length> }`. Reusable por outras
//! features Layout futuras (e.g. PageConfig refino — Sides<Length>
//! para margens; Stroke — Sides<Stroke>).
//!
//! Estilo paralelo ao vanilla
//! `lab/typst-original/crates/typst-library/src/layout/sides.rs`,
//! reduzido ao essencial para o consumer cristalino actual.

/// Quatro lados de um rectângulo (left/top/right/bottom).
///
/// Genérico em `T` para suportar qualquer valor por lado:
/// - `Sides<Length>` para padding/margem.
/// - `Sides<f64>` para offsets brutos.
/// - `Sides<bool>` para máscaras de presença.
///
/// `Copy` derivado quando `T: Copy` (caso comum de `Length`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sides<T> {
    pub left:   T,
    pub top:    T,
    pub right:  T,
    pub bottom: T,
}

impl<T> Sides<T> {
    /// Constrói `Sides` com cada lado independente.
    pub fn new(left: T, top: T, right: T, bottom: T) -> Self {
        Self { left, top, right, bottom }
    }
}

impl<T: Clone> Sides<T> {
    /// Constrói `Sides` com o mesmo valor em todos os lados.
    pub fn uniform(value: T) -> Self {
        Self {
            left:   value.clone(),
            top:    value.clone(),
            right:  value.clone(),
            bottom: value,
        }
    }
}

impl<T: Default> Default for Sides<T> {
    fn default() -> Self {
        Self {
            left:   T::default(),
            top:    T::default(),
            right:  T::default(),
            bottom: T::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sides_new_preserves_each_value() {
        let s = Sides::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(s.left,   1.0);
        assert_eq!(s.top,    2.0);
        assert_eq!(s.right,  3.0);
        assert_eq!(s.bottom, 4.0);
    }

    #[test]
    fn sides_uniform_replicates_value() {
        let s = Sides::uniform(7.5_f64);
        assert_eq!(s.left,   7.5);
        assert_eq!(s.top,    7.5);
        assert_eq!(s.right,  7.5);
        assert_eq!(s.bottom, 7.5);
    }

    #[test]
    fn sides_default_is_zero_for_numeric() {
        let s: Sides<f64> = Sides::default();
        assert_eq!(s.left,   0.0);
        assert_eq!(s.top,    0.0);
        assert_eq!(s.right,  0.0);
        assert_eq!(s.bottom, 0.0);
    }

    #[test]
    fn sides_partial_eq() {
        let a = Sides::new(1, 2, 3, 4);
        let b = Sides::new(1, 2, 3, 4);
        let c = Sides::new(1, 2, 3, 5);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
