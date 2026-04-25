//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/parity.md
//! @prompt-hash af8490cb
//! @layer L1
//! @updated 2026-04-25
//!
//! Tipo enum `Parity` (Even / Odd) usado como atributo `to` em
//! `Content::Pagebreak`. Adicionado no Passo 156E (Fase 1
//! Layout, ADR-0061) como infraestrutura genérica análoga ao
//! `Sides<T>` criado em P156C.
//!
//! Réplica simplificada de
//! `lab/typst-original/crates/typst-library/src/layout/page.rs::Parity`,
//! reduzida ao essencial para o consumer cristalino actual.
//! Vanilla expõe `Auto` via `Smart<Parity>`; cristalino usa
//! `Option<Parity>` (None == Auto).

/// Paridade de página (par/ímpar).
///
/// Usada por `Content::Pagebreak::to` para forçar a próxima
/// página a uma paridade específica. Quando `to: Some(parity)`
/// e a próxima página não bate a paridade, o Layouter insere
/// uma página vazia para ajustar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parity {
    Even,
    Odd,
}

impl Parity {
    /// Retorna `true` se `page_number` (1-indexed) bate esta
    /// paridade. Página 1 é ímpar, 2 é par, etc.
    pub fn matches(self, page_number: usize) -> bool {
        match self {
            Self::Even => page_number % 2 == 0,
            Self::Odd  => page_number % 2 == 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parity_matches_even_pages() {
        assert!(Parity::Even.matches(2));
        assert!(Parity::Even.matches(4));
        assert!(Parity::Even.matches(100));
        assert!(!Parity::Even.matches(1));
        assert!(!Parity::Even.matches(3));
        assert!(!Parity::Even.matches(99));
    }

    #[test]
    fn parity_matches_odd_pages() {
        assert!(Parity::Odd.matches(1));
        assert!(Parity::Odd.matches(3));
        assert!(Parity::Odd.matches(99));
        assert!(!Parity::Odd.matches(2));
        assert!(!Parity::Odd.matches(4));
        assert!(!Parity::Odd.matches(100));
    }

    #[test]
    fn parity_partial_eq() {
        assert_eq!(Parity::Even, Parity::Even);
        assert_eq!(Parity::Odd, Parity::Odd);
        assert_ne!(Parity::Even, Parity::Odd);
    }
}
