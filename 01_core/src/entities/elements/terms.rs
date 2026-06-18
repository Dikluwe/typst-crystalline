//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/terms.md
//! @prompt-hash 66a31ad1
//! @layer L1
//! @updated 2026-06-11
//!
//! `TermsElem` — Lote 3 P318 (família lista/termos). Lista de termos.
//! Contentor de prosa: recurse em map_content E map_text (precedente Heading).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Lista de termos (`/ termo: descrição`). Contém tipicamente `TermItem`.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct TermsElem {
    pub items: Vec<Content>,
}

impl Element for TermsElem {
    fn plain_text(&self) -> String {
        self.items.iter()
            .map(|t| t.plain_text())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let items: SourceResult<Vec<Content>> =
            self.items.iter().map(|c| c.map_content(transform)).collect();
        Ok(Content::Terms(Arc::new(TermsElem { items: items? })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Terms(Arc::new(TermsElem {
            items: self.items.iter().map(|c| c.map_text(transform)).collect(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> TermsElem {
        TermsElem { items: vec![Content::text("a"), Content::text("b")] }
    }

    #[test]
    fn plain_text_join_newline() {
        assert_eq!(ex().plain_text(), "a\nb");
    }

    #[test]
    fn is_empty_sem_items() {
        assert!(!ex().is_empty());
        assert!(TermsElem { items: vec![] }.is_empty());
    }

    #[test]
    fn map_content_recurse_items() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "a" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match ex().map_content(&mut f).unwrap() {
            Content::Terms(e) => assert_eq!(e.plain_text(), "Z\nb"),
            _ => panic!("esperado Terms"),
        }
    }
}
