//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/table_footer.md
//! @prompt-hash c5d872c6
//! @layer L1
//! @updated 2026-06-11
//!
//! `TableFooterElem` — Lote 5 P320. Rodapé de tabela (`body` + `repeat`).
//! Contentor de prosa: recurse em map_content E map_text (precedente Heading).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Rodapé de tabela (`body` + `repeat`).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct TableFooterElem {
    pub body: Content,
    pub repeat: bool,
}

impl Element for TableFooterElem {
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
        Ok(Content::TableFooter(Arc::new(TableFooterElem {
            body: self.body.map_content(transform)?,
            repeat: self.repeat,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::TableFooter(Arc::new(TableFooterElem {
            body: self.body.map_text(transform),
            repeat: self.repeat,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> TableFooterElem {
        TableFooterElem { body: Content::text("h"), repeat: true }
    }

    #[test]
    fn plain_text_do_body() {
        assert_eq!(ex().plain_text(), "h");
    }

    #[test]
    fn is_empty_delega_ao_body() {
        assert!(!ex().is_empty());
        assert!(TableFooterElem { body: Content::Empty, repeat: false }.is_empty());
    }

    #[test]
    fn map_text_recurse_preserva_repeat() {
        match ex().map_text(&mut |s| s.to_uppercase()) {
            Content::TableFooter(e) => {
                assert_eq!(e.plain_text(), "H");
                assert!(e.repeat);
            }
            _ => panic!("esperado TableFooter"),
        }
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(ex(), ex());
        assert_ne!(ex(), TableFooterElem { body: Content::text("h"), repeat: false });
    }
}
