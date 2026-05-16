//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/axes.md
//! @prompt-hash 9b5d3f18
//! @layer L1
//! @updated 2026-05-15
//!
//! **P264 (M9d / M7+5; ADR-0088 PROPOSTO Gradient Radial-only)**
//! — tipo genérico minimal `Axes<T>` para coordenadas 2D.
//! Criado para `Radial.center: Axes<Ratio>` (paridade vanilla
//! `Axes<T>`).
//!
//! Paridade minimal per ADR-0080 §"L0 minimal para refactors
//! aditivos".

/// Par 2D genérico (x, y). Reutilizável para Ratios, Lengths,
/// e outros tipos.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Axes<T> {
    pub x: T,
    pub y: T,
}

impl<T> Axes<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Eq> Eq for Axes<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axes_new_armazena_x_y() {
        let a = Axes::new(1, 2);
        assert_eq!(a.x, 1);
        assert_eq!(a.y, 2);
    }

    #[test]
    fn axes_copy_clone() {
        let a = Axes::new(0.5f64, 0.7f64);
        let b = a;
        assert_eq!(a, b);
        let c = a.clone();
        assert_eq!(a, c);
    }

    #[test]
    fn axes_partial_eq() {
        let a1 = Axes::new(1.0, 2.0);
        let a2 = Axes::new(1.0, 2.0);
        let a3 = Axes::new(1.0, 3.0);
        assert_eq!(a1, a2);
        assert_ne!(a1, a3);
    }
}
