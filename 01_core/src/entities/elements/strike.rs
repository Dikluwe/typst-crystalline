//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/strike.md
//! @prompt-hash c0dba6a4
//! @layer L1
//! @updated 2026-06-11
//!
//! `StrikeElem` — Lote 4 P319 (decorações de texto). Riscado (P284).
//! Contentor de prosa: recurse em map_content E map_text (precedente Heading).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::layout_types::{Color, Length};
use crate::entities::source_result::SourceResult;

/// Decoração `strike(body)` — linha sobre o corpo. `stroke`/`offset`/`extent`
/// cosméticos (ADR-0054 graded).
#[derive(Debug, Clone, PartialEq)]
pub struct StrikeElem {
    pub body: Content,
    pub stroke: Option<Color>,
    pub offset: Option<Length>,
    pub extent: Option<Length>,
}

// `Hash` manual via `Debug` (paridade `content_hash::hash_content`).
// CAUSA: `Length` carrega `f64` e não implementa `Hash`; o trait `Element`
// exige `Hash`, logo `#[derive(Hash)]` não compila.
// RESSALVA: hash-via-Debug pode violar Hash/Eq para `-0.0` vs `0.0`
// (PartialEq-iguais, Debug distinto). Aceitável: regime do `content_hash`
// pré-existente e os `…Elem` NÃO são chaves de mapa. Revisitar se vier a sê-lo.
impl std::hash::Hash for StrikeElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for StrikeElem {
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
        Ok(Content::Strike(Arc::new(StrikeElem {
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
        Content::Strike(Arc::new(StrikeElem {
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

    fn ex() -> StrikeElem {
        StrikeElem {
            body: Content::text("abc"),
            stroke: None,
            offset: None,
            extent: None,
        }
    }

    fn h(e: &StrikeElem) -> u64 {
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
        let vazio = StrikeElem {
            body: Content::Empty,
            stroke: None,
            offset: None,
            extent: None,
        };
        assert!(vazio.is_empty());
    }

    #[test]
    fn map_content_recurse_preserva_cosmeticos() {
        let mut e = ex();
        e.extent = Some(Length::pt(1.0));
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "abc" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match e.map_content(&mut f).unwrap() {
            Content::Strike(el) => {
                assert_eq!(el.plain_text(), "Z");
                assert_eq!(el.extent, Some(Length::pt(1.0)));
            }
            _ => panic!("esperado Strike"),
        }
    }

    #[test]
    fn campo_diferente_produz_hash_diferente() {
        let a = ex();
        let mut b = a.clone();
        b.extent = Some(Length::pt(1.0));
        assert_ne!(h(&a), h(&b), "extent distinto → hash distinto");
        assert_eq!(h(&a), h(&a.clone()), "mesmo conteúdo → mesmo hash");
    }
}
