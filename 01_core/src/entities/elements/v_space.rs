//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/v_space.md
//! @prompt-hash a72fc045
//! @layer L1
//! @updated 2026-06-11
//!
//! `VSpaceElem` — Lote 5 P320. Espaço vertical (`v(amount)`).
//! Leaf terminal em map_*; `is_empty` quando `amount` é zero.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::layout_types::Length;
use crate::entities::source_result::SourceResult;

/// Espaço vertical. `amount` é a altura; `weak` colapsa nas bordas.
#[derive(Debug, Clone, PartialEq)]
pub struct VSpaceElem {
    pub amount: Length,
    pub weak: bool,
    /// Se `weak` foi fornecido explicitamente na chamada Typst.
    pub weak_explicit: bool,
}

// `Hash` manual via `Debug` (paridade `content_hash::hash_content`).
// CAUSA: `Length` carrega `f64` e não implementa `Hash`; o trait `Element`
// exige `Hash`, logo `#[derive(Hash)]` não compila.
// RESSALVA: hash-via-Debug pode violar Hash/Eq para `-0.0` vs `0.0`
// (PartialEq-iguais, Debug distinto). Aceitável: regime do `content_hash`
// pré-existente e os `…Elem` NÃO são chaves de mapa. Revisitar se vier a sê-lo.
impl std::hash::Hash for VSpaceElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for VSpaceElem {
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
        Ok(Content::VSpace(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        // Terminal (leaf sem texto).
        Content::VSpace(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn h(e: &VSpaceElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn plain_text_vazio() {
        assert_eq!(
            VSpaceElem {
                amount: Length::pt(5.0),
                weak: false,
                weak_explicit: false,
            }
            .plain_text(),
            ""
        );
    }

    #[test]
    fn is_empty_quando_amount_zero() {
        assert!(VSpaceElem {
            amount: Length::ZERO,
            weak: false,
            weak_explicit: false,
        }
        .is_empty());
        assert!(!VSpaceElem {
            amount: Length::pt(2.0),
            weak: false,
            weak_explicit: false,
        }
        .is_empty());
    }

    #[test]
    fn campo_diferente_produz_hash_diferente() {
        let a = VSpaceElem {
            amount: Length::pt(2.0),
            weak: false,
            weak_explicit: false,
        };
        let b = VSpaceElem { weak: true, weak_explicit: true, ..a.clone() };
        assert_ne!(h(&a), h(&b), "weak distinto → hash distinto");
        assert_eq!(h(&a), h(&a.clone()), "mesmo conteúdo → mesmo hash");
    }

    #[test]
    fn igualdade_estrutural() {
        let a = VSpaceElem {
            amount: Length::pt(2.0),
            weak: true,
            weak_explicit: true,
        };
        assert_eq!(a.clone(), a.clone());
        assert_ne!(
            a,
            VSpaceElem {
                amount: Length::pt(3.0),
                weak: true,
                weak_explicit: true,
            }
        );
        let omitted = VSpaceElem {
            amount: Length::pt(2.0),
            weak: false,
            weak_explicit: false,
        };
        assert_ne!(omitted, VSpaceElem { weak_explicit: true, ..omitted.clone() });
    }
}
