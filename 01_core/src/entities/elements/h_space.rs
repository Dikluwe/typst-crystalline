//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/h_space.md
//! @prompt-hash 6d532b46
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

/// Espaço horizontal. `amount` é a largura; `weak` colapsa nas bordas.
#[derive(Debug, Clone, PartialEq)]
pub struct HSpaceElem {
    pub amount: Length,
    pub weak:   bool,
}

// `Hash` manual via `Debug` (paridade `content_hash::hash_content`).
// CAUSA: `Length` carrega `f64` e não implementa `Hash`; o trait `Element`
// exige `Hash`, logo `#[derive(Hash)]` não compila.
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
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    fn h(e: &HSpaceElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn plain_text_vazio() {
        assert_eq!(HSpaceElem { amount: Length::pt(5.0), weak: false }.plain_text(), "");
    }

    #[test]
    fn is_empty_quando_amount_zero() {
        assert!(HSpaceElem { amount: Length::ZERO, weak: false }.is_empty());
        assert!(!HSpaceElem { amount: Length::pt(2.0), weak: false }.is_empty());
    }

    #[test]
    fn campo_diferente_produz_hash_diferente() {
        let a = HSpaceElem { amount: Length::pt(2.0), weak: false };
        let b = HSpaceElem { amount: Length::pt(3.0), weak: false };
        assert_ne!(h(&a), h(&b), "amount distinto → hash distinto");
        assert_eq!(h(&a), h(&a.clone()), "mesmo conteúdo → mesmo hash");
    }

    #[test]
    fn igualdade_estrutural() {
        let a = HSpaceElem { amount: Length::pt(2.0), weak: true };
        assert_eq!(a.clone(), a.clone());
        assert_ne!(a, HSpaceElem { amount: Length::pt(2.0), weak: false });
    }
}
