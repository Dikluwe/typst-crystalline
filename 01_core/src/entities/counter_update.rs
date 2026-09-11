//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/counter_update.md
//! @prompt-hash c21c6622
//! @layer L1
//! @updated 2026-04-30
//!
//! `CounterUpdate` — operação completa a aplicar a um contador.

use std::num::NonZeroUsize;

use crate::entities::func::Func;

/// Instrução de modificação de um contador.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CounterUpdate {
    /// Substitui o estado hierárquico completo.
    Set(Vec<usize>),
    /// Avança o nível indicado (1-based).
    Step(NonZeroUsize),
    /// Calcula o novo estado a partir do estado corrente.
    Func(Func),
}

impl CounterUpdate {
    /// Step canônico de nível 1 para producers automáticos.
    pub const fn step() -> Self {
        Self::Step(NonZeroUsize::MIN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_e_set_sao_distintos() {
        assert_ne!(CounterUpdate::step(), CounterUpdate::Set(vec![1]));
    }

    #[test]
    fn set_compara_estado_completo() {
        assert_eq!(CounterUpdate::Set(vec![7, 2]), CounterUpdate::Set(vec![7, 2]));
        assert_ne!(CounterUpdate::Set(vec![7]), CounterUpdate::Set(vec![7, 2]));
    }

    #[test]
    fn clone_preserva_variantes() {
        let a = CounterUpdate::step();
        let b = CounterUpdate::Set(vec![42]);
        assert_eq!(a.clone(), CounterUpdate::step());
        assert_eq!(b.clone(), CounterUpdate::Set(vec![42]));
    }

    #[test]
    fn hash_distingue_variantes() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        CounterUpdate::step().hash(&mut h1);
        CounterUpdate::Set(vec![0]).hash(&mut h2);
        assert_ne!(h1.finish(), h2.finish());
    }
}
