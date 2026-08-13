//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/overline.md
//! @prompt-hash 79333c17
//! @layer L1
//! @updated 2026-06-11
//!
//! `OverlineElem` — Lote 4 P319 (decorações de texto). Linha acima (P284).
//! Contentor de prosa: recurse em map_content E map_text (precedente Heading).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::layout_types::{Color, Length};
use crate::entities::source_result::SourceResult;

/// Decoração `overline(body)` — linha acima do corpo. `stroke`/`offset`/`extent`
/// cosméticos (ADR-0054 graded).
#[derive(Debug, Clone, PartialEq)]
pub struct OverlineElem {
    pub body: Content,
    pub stroke: Option<Color>,
    pub offset: Option<Length>,
    pub extent: Option<Length>,
}

// `Hash` manual via `Debug` (paridade `content_hash::hash_content`).
// CAUSA: `Length` carrega `f64` e não implementa `Hash`; o trait `Element`
// exige `Hash`, logo `#[derive(Hash)]` não compila.
// RESSALVA: hash-via-Debug pode violar o contrato Hash/Eq para `-0.0` vs `0.0`
// (PartialEq-iguais, Debug distinto). Aceitável: é o regime do `content_hash`
// pré-existente e os `…Elem` NÃO são chaves de mapa. Se algum vier a ser chave
// (HashMap/HashSet), revisitar (hash canónico dos floats por bits).
impl std::hash::Hash for OverlineElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for OverlineElem {
    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    fn is_empty(&self) -> bool {
        self.body.is_empty()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Overline(Arc::new(OverlineElem {
            body: self.body.map_content(transform)?,
            stroke: self.stroke,
            offset: self.offset,
            extent: self.extent,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Overline(Arc::new(OverlineElem {
            body: self.body.map_text(transform),
            stroke: self.stroke,
            offset: self.offset,
            extent: self.extent,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn ex() -> OverlineElem {
        OverlineElem {
            body: Content::text("abc"),
            stroke: None,
            offset: None,
            extent: None,
        }
    }

    fn h(e: &OverlineElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn plain_text_transparente() {
        assert_eq!(ex().plain_text(), "abc");
    }

    #[test]
    fn is_empty_delega_ao_body() {
        assert!(!ex().is_empty());
        let vazio = OverlineElem {
            body: Content::Empty,
            stroke: None,
            offset: None,
            extent: None,
        };
        assert!(vazio.is_empty());
    }

    #[test]
    fn map_text_recurse_preserva_cosmeticos() {
        let mut e = ex();
        e.offset = Some(Length::pt(2.0));
        match e.map_text(&mut |s| s.to_uppercase()) {
            Content::Overline(el) => {
                assert_eq!(el.plain_text(), "ABC");
                assert_eq!(el.offset, Some(Length::pt(2.0)));
            }
            _ => panic!("esperado Overline"),
        }
    }

    #[test]
    fn campo_diferente_produz_hash_diferente() {
        let a = ex();
        let mut b = a.clone();
        b.offset = Some(Length::pt(2.0));
        assert_ne!(h(&a), h(&b), "offset distinto → hash distinto");
        assert_eq!(h(&a), h(&a.clone()), "mesmo conteúdo → mesmo hash");
    }
}
