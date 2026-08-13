//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/h_space.md
//! @prompt-hash 39bc2a3d
//! @layer L1
//! @updated 2026-06-11
//!
//! `HSpaceElem` — Lote 5 P320. Espaço horizontal (`h(amount)`).
//! Leaf terminal em map_*; `is_empty` quando `amount` é zero.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::layout_types::Length;
use crate::entities::source_result::SourceResult;

/// **P842 (#38)** — quantidade de um spacing `h()`: comprimento absoluto ou
/// fração do espaço restante da linha (paridade vanilla `Spacing`,
/// `layout/spacing.rs`). `Fractional` é expandida no `flush_line`/`finish`
/// do Layouter, quando o espaço restante é conhecido.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Spacing {
    /// Comprimento absoluto (`h(10pt)`, `h(2em)`).
    Absolute(Length),
    /// Fração do espaço restante (`h(1fr)`).
    Fractional(f64),
}

impl Spacing {
    /// Zero absoluto ou fração zero — não contribui espaço próprio.
    pub fn is_zero(&self) -> bool {
        match self {
            Self::Absolute(l) => l.is_zero(),
            Self::Fractional(f) => *f == 0.0,
        }
    }
}

/// Espaço horizontal. `amount` é a largura; `weak` colapsa nas bordas.
#[derive(Debug, Clone, PartialEq)]
pub struct HSpaceElem {
    pub amount: Spacing,
    pub weak: bool,
}

// `Hash` manual via `Debug` (paridade `content_hash::hash_content`).
// CAUSA: `Length` carrega `f64` e não implementa `Hash`; o trait `Element`
// exige `Hash`, logo `#[derive(Hash)]` não compila. **P842**: idem para
// `Spacing::Fractional` (`f64`).
// RESSALVA: hash-via-Debug pode violar Hash/Eq para `-0.0` vs `0.0`
// (PartialEq-iguais, Debug distinto). Aceitável: regime do `content_hash`
// pré-existente e os `…Elem` NÃO são chaves de mapa. Revisitar se vier a sê-lo.
impl std::hash::Hash for HSpaceElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for HSpaceElem {
    fn plain_text(&self) -> String {
        String::new()
    }

    fn is_empty(&self) -> bool {
        self.amount.is_zero()
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        // Terminal (leaf sem filhos).
        Ok(Content::HSpace(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        // Terminal (leaf sem texto).
        Content::HSpace(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn h(e: &HSpaceElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn plain_text_vazio() {
        assert_eq!(
            HSpaceElem { amount: Spacing::Absolute(Length::pt(5.0)), weak: false }
                .plain_text(),
            ""
        );
    }

    #[test]
    fn is_empty_quando_amount_zero() {
        assert!(HSpaceElem { amount: Spacing::Absolute(Length::ZERO), weak: false }.is_empty());
        assert!(!HSpaceElem { amount: Spacing::Absolute(Length::pt(2.0)), weak: false }.is_empty());
        // P842 — fração zero é vazia; fração positiva não.
        assert!(HSpaceElem { amount: Spacing::Fractional(0.0), weak: false }.is_empty());
        assert!(!HSpaceElem { amount: Spacing::Fractional(1.0), weak: false }.is_empty());
    }

    #[test]
    fn campo_diferente_produz_hash_diferente() {
        let a = HSpaceElem { amount: Spacing::Absolute(Length::pt(2.0)), weak: false };
        let b = HSpaceElem { amount: Spacing::Absolute(Length::pt(3.0)), weak: false };
        assert_ne!(h(&a), h(&b), "amount distinto → hash distinto");
        assert_eq!(h(&a), h(&a.clone()), "mesmo conteúdo → mesmo hash");
    }

    #[test]
    fn igualdade_estrutural() {
        let a = HSpaceElem { amount: Spacing::Absolute(Length::pt(2.0)), weak: true };
        assert_eq!(a.clone(), a.clone());
        assert_ne!(a, HSpaceElem { amount: Spacing::Absolute(Length::pt(2.0)), weak: false });
    }
}
