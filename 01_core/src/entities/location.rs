//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/location.md
//! @prompt-hash 42a74416
//! @layer L1
//! @updated 2026-04-30
//!
//! `Location` — identificador único e estável de um elemento dentro
//! do documento durante a passagem de introspecção. P161 sub-passo .3
//! (peça inicial da arquitectura Introspection com fixpoint).
//!
//! Construtor não-`pub` por design: só o `Locator` (em
//! `entities/locator.rs`) deve produzir `Location`s.

/// Identificador único de um elemento indexado pela introspecção.
///
/// Forma `u128` em paridade com `lab/typst-original/.../introspection/location.rs`.
/// `Copy`, `Eq`, `Hash` — usável como chave em qualquer mapa hashado.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location(u128);

impl Location {
    /// Construtor restrito ao crate L1. Único call-site legítimo:
    /// `Locator::next()`. Visibilidade `pub(crate)` força a entrada
    /// pelo gerador determinístico.
    pub(crate) fn from_raw(raw: u128) -> Self {
        Self(raw)
    }

    /// Extrai o hash interno em forma raw — escape hatch para
    /// `Tag::End(location, content_hash)` e debugging. Não usar
    /// para construção (que passa sempre pelo `Locator`).
    pub fn as_u128(&self) -> u128 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iguais_se_u128_iguais() {
        let a = Location::from_raw(42);
        let b = Location::from_raw(42);
        assert_eq!(a, b);
    }

    #[test]
    fn diferentes_se_u128_distintos() {
        let a = Location::from_raw(1);
        let b = Location::from_raw(2);
        assert_ne!(a, b);
    }

    #[test]
    fn as_u128_devolve_hash_interno() {
        let l = Location::from_raw(0xABCD);
        assert_eq!(l.as_u128(), 0xABCD);
    }

    #[test]
    fn copy_preserva_valor() {
        let l = Location::from_raw(7);
        let c = l;
        assert_eq!(l.as_u128(), c.as_u128());
    }

    #[test]
    fn hash_e_estavel() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let l = Location::from_raw(99);
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        l.hash(&mut h1);
        l.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }
}
