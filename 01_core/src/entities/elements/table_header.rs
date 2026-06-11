//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/table_header.md
//! @prompt-hash 2654733e
//! @layer L1
//! @updated 2026-06-11
//!
//! `TableHeaderElem` — Lote 5 P320. Cabeçalho de tabela (`body` + `repeat`).
//! Contentor de prosa: recurse em map_content E map_text (precedente Heading).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Cabeçalho de tabela (`body` + `repeat`).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct TableHeaderElem {
    pub body:   Content,
    pub repeat: bool,
}

impl Element for TableHeaderElem {
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
        Ok(Content::TableHeader(Arc::new(TableHeaderElem {
            body:   self.body.map_content(transform)?,
            repeat: self.repeat,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::TableHeader(Arc::new(TableHeaderElem {
            body:   self.body.map_text(transform),
            repeat: self.repeat,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> TableHeaderElem {
        TableHeaderElem { body: Content::text("h"), repeat: true }
    }

    #[test]
    fn plain_text_do_body() {
        assert_eq!(ex().plain_text(), "h");
    }

    #[test]
    fn is_empty_delega_ao_body() {
        assert!(!ex().is_empty());
        assert!(TableHeaderElem { body: Content::Empty, repeat: false }.is_empty());
    }

    #[test]
    fn map_text_recurse_preserva_repeat() {
        match ex().map_text(&mut |s| s.to_uppercase()) {
            Content::TableHeader(e) => {
                assert_eq!(e.plain_text(), "H");
                assert!(e.repeat);
            }
            _ => panic!("esperado TableHeader"),
        }
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(ex(), ex());
        assert_ne!(ex(), TableHeaderElem { body: Content::text("h"), repeat: false });
    }
}
