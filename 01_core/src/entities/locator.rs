//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/locator.md
//! @prompt-hash 224ef9ef
//! @layer L1
//! @updated 2026-04-30
//!
//! `Locator` — gerador determinístico de `Location`s usado durante
//! o walk de introspecção. P161 sub-passo .4.
//!
//! Implementação minimalista: contador `u64` incremental. Refinos
//! futuros (combinação com hash de path para suporte
//! cross-memoization) ficam para M2/M3 conforme desenho.

use crate::entities::location::Location;

/// Gerador determinístico de `Location`s.
///
/// Determinismo: dois `Locator::new()` independentes, recebendo a
/// mesma sequência de chamadas a `next()`, produzem `Location`s
/// iguais. Walk com mesmo `Content` produz mesma sequência.
///
/// **Não** é `Clone` por design — clonar romperia o invariante de
/// unicidade entre instâncias do mesmo walk.
pub struct Locator {
    counter: u64,
}

impl Locator {
    /// Cria um novo `Locator` no estado inicial (counter = 0).
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    /// Produz a próxima `Location` e avança o contador interno.
    ///
    /// O `u128` interno da `Location` é o counter directo
    /// (zero-extended de `u64`). Garantia de unicidade dentro de
    /// uma instância e de igualdade entre instâncias paralelas.
    pub fn next(&mut self) -> Location {
        let raw = self.counter as u128;
        self.counter = self.counter.checked_add(1).expect(
            "Locator counter overflow — > 2^64 elementos numa única passagem"
        );
        Location::from_raw(raw)
    }
}

impl Default for Locator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_duas_chamadas_produzem_locations_distintas() {
        let mut l = Locator::new();
        let a = l.next();
        let b = l.next();
        assert_ne!(a, b);
    }

    #[test]
    fn duas_instancias_paralelas_produzem_sequencias_iguais() {
        let mut l1 = Locator::new();
        let mut l2 = Locator::new();
        let s1: Vec<_> = (0..5).map(|_| l1.next()).collect();
        let s2: Vec<_> = (0..5).map(|_| l2.next()).collect();
        assert_eq!(s1, s2);
    }

    #[test]
    fn counter_e_monotonico_crescente() {
        let mut l = Locator::new();
        let a = l.next();
        let b = l.next();
        let c = l.next();
        assert!(a.as_u128() < b.as_u128());
        assert!(b.as_u128() < c.as_u128());
    }

    #[test]
    fn default_e_equivalente_a_new() {
        let mut l1 = Locator::default();
        let mut l2 = Locator::new();
        assert_eq!(l1.next(), l2.next());
    }
}
