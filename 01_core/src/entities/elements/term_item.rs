//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/term_item.md
//! @prompt-hash 792c8762
//! @layer L1
//! @updated 2026-06-11
//!
//! `TermItemElem` — Lote 3 P318 (família lista/termos). Par termo+descrição.
//! Contentor de prosa: recurse em map_content E map_text (precedente Heading).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Item de lista de termos: `term` + `description`.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct TermItemElem {
    pub term:        Content,
    pub description: Content,
}

impl Element for TermItemElem {
    fn plain_text(&self) -> String {
        format!("{}: {}", self.term.plain_text(), self.description.plain_text())
    }

    fn is_empty(&self) -> bool {
        self.term.is_empty() && self.description.is_empty()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::TermItem(Arc::new(TermItemElem {
            term:        self.term.map_content(transform)?,
            description: self.description.map_content(transform)?,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::TermItem(Arc::new(TermItemElem {
            term:        self.term.map_text(transform),
            description: self.description.map_text(transform),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ti() -> TermItemElem {
        TermItemElem { term: Content::text("API"), description: Content::text("interface") }
    }

    #[test]
    fn plain_text_termo_descricao() {
        assert_eq!(ti().plain_text(), "API: interface");
    }

    #[test]
    fn is_empty_quando_ambos_vazios() {
        assert!(!ti().is_empty());
        let vazio = TermItemElem { term: Content::empty(), description: Content::empty() };
        assert!(vazio.is_empty());
    }

    #[test]
    fn map_content_recurse_ambos() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s, _) if s.as_str() == "API" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match ti().map_content(&mut f).unwrap() {
            Content::TermItem(e) => assert_eq!(e.plain_text(), "Z: interface"),
            _ => panic!("esperado TermItem"),
        }
    }
}
