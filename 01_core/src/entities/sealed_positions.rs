//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/sealed-positions.md
//! @prompt-hash 89baeda9
//! @layer L1
//! @updated 2026-05-07
//!
//! **P205B (F3)** — Sub-store sealed para `Location → Position`
//! per ADR-0074 (PROPOSTO 2026-05-07).
//!
//! Congela `LayouterRuntimeState.positions` ao fim de cada
//! iteração de layout (`Layouter::finish`). Permite tracking
//! via comemo de queries `position_of` pós-layout. Fecha
//! pendência ADR-0073 §C6a (`TagIntrospector::position_of` ainda
//! retorna `None`; consumer migrate-se em P205C).
//!
//! Cristalino diverge intencionalmente de vanilla
//! (`PagedIntrospector::elements` global construído por
//! `PagedIntrospector::new(&pages)`); cristalino faz sealing
//! por sub-store, não global. P205A.div-1.

use std::collections::HashMap;

use crate::entities::location::Location;
use crate::entities::position::Position;

/// Sub-store sealed para `Location → Position` mappings.
///
/// Construído por `Layouter::finish` ao consumir
/// `LayouterRuntimeState.positions`. Read-only após construção.
#[derive(Debug, Clone, Default)]
pub struct SealedPositions {
    positions: HashMap<Location, Position>,
}

impl SealedPositions {
    /// Construtor vazio. Equivalente a `Default::default()`.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Constrói consumindo o HashMap populated single-pass por
    /// `Layouter` durante a iteração. Move o conteúdo —
    /// sealing point literal.
    pub fn from_runtime(positions: HashMap<Location, Position>) -> Self {
        Self { positions }
    }

    /// Número de positions registados.
    pub fn len(&self) -> usize {
        self.positions.len()
    }

    /// True se vazio (nenhum locatable processado).
    pub fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }
}

#[comemo::track]
impl SealedPositions {
    /// Position para a `Location` indicada, ou `None` se ausente.
    /// Tracked: queries repetidas reutilizam cache comemo.
    pub fn position_of(&self, location: Location) -> Option<Position> {
        self.positions.get(&location).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn loc(raw: u128) -> Location {
        Location::from_raw(raw)
    }

    fn pos(page_nz: usize, x: f64, y: f64) -> Position {
        use std::num::NonZeroUsize;
        use crate::entities::layout_types::{Point, Pt};
        Position {
            page:  NonZeroUsize::new(page_nz).unwrap(),
            point: Point { x: Pt(x), y: Pt(y) },
        }
    }

    // ── Sentinelas P205B ─────────────────────────────────────────────

    #[test]
    fn p205b_sealed_positions_struct_existe() {
        // Sentinel: confirma que SealedPositions::empty e from_runtime
        // existem. Falha de compilação se removidos.
        let _empty: SealedPositions = SealedPositions::empty();
        let _from: SealedPositions = SealedPositions::from_runtime(HashMap::new());
    }

    #[test]
    fn p205b_sealed_positions_e_track() {
        // Sentinel: confirma que SealedPositions implementa
        // `comemo::Track` (gerado pelo `#[comemo::track] impl`).
        // Falha de compilação se atributo for removido.
        fn assert_track<T: comemo::Track + ?Sized>() {}
        assert_track::<SealedPositions>();
    }

    // ── Tests de unidade ─────────────────────────────────────────────

    #[test]
    fn empty_devolve_none_em_qualquer_lookup() {
        use comemo::Track;
        let s = SealedPositions::empty();
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
        // Via Tracked handle — exercício do macro.
        let t = s.track();
        assert_eq!(t.position_of(loc(1)), None);
    }

    #[test]
    fn from_runtime_preserva_mappings() {
        use comemo::Track;
        let mut map = HashMap::new();
        map.insert(loc(1), pos(1, 10.0, 20.0));
        map.insert(loc(2), pos(2, 30.0, 40.0));

        let s = SealedPositions::from_runtime(map);
        assert_eq!(s.len(), 2);
        assert!(!s.is_empty());

        let t = s.track();
        assert_eq!(t.position_of(loc(1)), Some(pos(1, 10.0, 20.0)));
        assert_eq!(t.position_of(loc(2)), Some(pos(2, 30.0, 40.0)));
        assert_eq!(t.position_of(loc(99)), None);
    }
}
