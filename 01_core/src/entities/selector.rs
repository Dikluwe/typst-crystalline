//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/selector.md
//! @prompt-hash 92ddd3cd
//! @layer L1
//! @updated 2026-04-29
//!
//! `Selector` — predicado para `Introspector::query` (P175 / M9
//! sub-passo 5).
//!
//! Vanilla equivalente: `foundations/selector.rs` com 10+ variants.
//! Cristalino P175 implementa apenas `Kind(ElementKind)` — minimal
//! viable. Refino futuro (`Label`, `And`, `Or`, `Where`, ...) adiado
//! para passos dedicados quando consumers reais necessitarem.

use crate::entities::element_kind::ElementKind;

/// Predicado para `Introspector::query`. Variants futuros adiados.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Selector {
    /// Selector de kind — matches todos os elementos de um tipo.
    Kind(ElementKind),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test]
    fn igualdade_estrutural() {
        let a = Selector::Kind(ElementKind::Heading);
        let b = Selector::Kind(ElementKind::Heading);
        assert_eq!(a, b);
    }

    #[test]
    fn kinds_distintos_sao_diferentes() {
        let h = Selector::Kind(ElementKind::Heading);
        let f = Selector::Kind(ElementKind::Figure);
        assert_ne!(h, f);
    }

    #[test]
    fn hash_determinismo() {
        let s = Selector::Kind(ElementKind::Heading);
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        s.hash(&mut h1);
        s.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }
}
