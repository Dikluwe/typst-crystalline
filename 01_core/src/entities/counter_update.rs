//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/counter_update.md
//! @prompt-hash 7a335015
//! @layer L1
//! @updated 2026-04-30
//!
//! `CounterUpdate` — operação a aplicar a um contador. Extraído de
//! `counter_state.rs` no P161 sub-passo .6 (renomeação de
//! `CounterAction`). Variantes preservadas como estavam para não
//! mexer em call-sites; reshape Set/Step(usize) deferido.

/// Instrução de modificação de um contador.
///
/// Renomeação directa do `CounterAction` legacy (P161). Variantes
/// não foram alteradas — `Step` permanece sem payload (semântica
/// "step by 1 / next level"); `Update(usize)` permanece com value
/// fixo. Reshape para forma vanilla literal (`Set(usize)`,
/// `Step(usize)`) fica para passo dedicado quando consumers
/// estiverem prontos.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CounterUpdate {
    /// Avança o contador em 1 (flat) ou avança o nível (hierárquico).
    Step,
    /// Força o contador para o valor indicado.
    Update(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_e_update_sao_distintos() {
        assert_ne!(CounterUpdate::Step, CounterUpdate::Update(1));
    }

    #[test]
    fn update_compara_por_valor() {
        assert_eq!(CounterUpdate::Update(7), CounterUpdate::Update(7));
        assert_ne!(CounterUpdate::Update(7), CounterUpdate::Update(8));
    }

    #[test]
    fn clone_preserva_variantes() {
        let a = CounterUpdate::Step;
        let b = CounterUpdate::Update(42);
        assert_eq!(a.clone(), CounterUpdate::Step);
        assert_eq!(b.clone(), CounterUpdate::Update(42));
    }

    #[test]
    fn hash_distingue_variantes() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        CounterUpdate::Step.hash(&mut h1);
        CounterUpdate::Update(0).hash(&mut h2);
        assert_ne!(h1.finish(), h2.finish());
    }
}
