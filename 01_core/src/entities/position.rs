//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/position.md
//! @prompt-hash 208e41b7
//! @layer L1
//! @updated 2026-05-06
//!
//! **P204D (M8)** — Position concreta para o cristalino.
//!
//! Réplica de `PagedPosition` do vanilla (`{ page: NonZeroUsize,
//! point: Point }`) per ADR-0073 (paridade vanilla literal). Resolve
//! o concern adiado por ADR-0066 (intermediário até M8) e
//! confirmado por P203 consolidado §13 como "concern ortogonal
//! coberto por M8".
//!
//! Pipeline cristalino — divergência intencional vs vanilla:
//! - **Vanilla**: Position calculada post-layout (fase 3 separada
//!   sobre `&[Page]` finalizadas).
//! - **Cristalino**: Position calculada single-pass durante layout
//!   (per P203A C5; P204D C5). Layouter popula
//!   `runtime.positions: HashMap<Location, Position>` à medida
//!   que processa locatables.
//!
//! Saída observable equivalente — mapping Location → Position
//! idêntico para um documento. Mecanismo difere; ADR-0033
//! (paridade observable) preservada.

use std::num::NonZeroUsize;

use crate::entities::layout_types::Point;

/// Posição física de um elemento em documento paginado.
///
/// Réplica de `PagedPosition` vanilla. `page` é 1-based
/// `NonZeroUsize` por construção (não há página 0; primeira página
/// é 1). `point` é coordenada 2D na página em pontos tipográficos.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    /// Número da página, 1-based.
    pub page: NonZeroUsize,
    /// Coordenadas (x, y) na página.
    pub point: Point,
}

// P204D: impl Hash necessário para `Option<Position>` retornado por
// trait method `Introspector::position_of` sob `#[comemo::track]`
// (per P204B padrão — restrição comemo "return values must
// implement Hash"). Point.x e Point.y são Pt(f64); usamos
// `to_bits()` para hash determinístico evitando NaN issues.
impl std::hash::Hash for Position {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.page.hash(state);
        self.point.x.val().to_bits().hash(state);
        self.point.y.val().to_bits().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::layout_types::Pt;

    #[test]
    fn position_construcao_basica() {
        let p = Position {
            page:  NonZeroUsize::new(1).unwrap(),
            point: Point { x: Pt(10.0), y: Pt(20.0) },
        };
        assert_eq!(p.page.get(), 1);
        assert_eq!(p.point.x.val(), 10.0);
        assert_eq!(p.point.y.val(), 20.0);
    }

    #[test]
    fn position_iguais_produzem_mesmo_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let p1 = Position {
            page:  NonZeroUsize::new(2).unwrap(),
            point: Point { x: Pt(15.0), y: Pt(30.0) },
        };
        let p2 = Position {
            page:  NonZeroUsize::new(2).unwrap(),
            point: Point { x: Pt(15.0), y: Pt(30.0) },
        };

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        p1.hash(&mut h1);
        p2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn position_distintos_produzem_hashes_distintos() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let p1 = Position {
            page:  NonZeroUsize::new(1).unwrap(),
            point: Point { x: Pt(10.0), y: Pt(20.0) },
        };
        let p2 = Position {
            page:  NonZeroUsize::new(2).unwrap(),
            point: Point { x: Pt(10.0), y: Pt(20.0) },
        };

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        p1.hash(&mut h1);
        p2.hash(&mut h2);
        assert_ne!(h1.finish(), h2.finish(),
            "page diferente → hashes distintos");
    }
}
