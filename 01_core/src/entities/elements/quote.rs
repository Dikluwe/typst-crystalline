//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/quote.md
//! @prompt-hash b2bd5ba7
//! @layer L1
//! @updated 2026-06-11
//!
//! `QuoteElem` — Lote 8 P323 (por largura). `quote(body, attribution, …)`.
//! Contentor: recurse em `body` E `attribution`, em map_content E map_text.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Citação com `body`, `attribution` opcional, e flags `block`/`quotes`.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct QuoteElem {
    pub body:        Content,
    pub attribution: Option<Content>,
    pub block:       bool,
    pub quotes:      bool,
}

impl Element for QuoteElem {
    fn plain_text(&self) -> String {
        // Paridade content.rs:1835: aspas opcionais + atribuição com travessão.
        let body_txt = self.body.plain_text();
        let with_quotes = if self.quotes {
            format!("\"{}\"", body_txt)
        } else {
            body_txt
        };
        match &self.attribution {
            Some(a) => format!("{} — {}", with_quotes, a.plain_text()),
            None    => with_quotes,
        }
    }

    fn is_empty(&self) -> bool {
        self.body.is_empty()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Quote(Arc::new(QuoteElem {
            body:        self.body.map_content(transform)?,
            attribution: self.attribution.as_ref()
                .map(|c| c.map_content(transform))
                .transpose()?,
            block:       self.block,
            quotes:      self.quotes,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Quote(Arc::new(QuoteElem {
            body:        self.body.map_text(transform),
            attribution: self.attribution.as_ref().map(|c| c.map_text(transform)),
            block:       self.block,
            quotes:      self.quotes,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    fn ex() -> QuoteElem {
        QuoteElem {
            body:        Content::text("vida"),
            attribution: Some(Content::text("alguém")),
            block:       true,
            quotes:      true,
        }
    }

    #[test]
    fn plain_text_com_aspas_e_atribuicao() {
        assert_eq!(ex().plain_text(), "\"vida\" — alguém");
    }

    #[test]
    fn plain_text_sem_aspas_sem_atribuicao() {
        let e = QuoteElem { body: Content::text("x"), attribution: None, block: false, quotes: false };
        assert_eq!(e.plain_text(), "x");
    }

    #[test]
    fn is_empty_delega_ao_body() {
        assert!(!ex().is_empty());
    }

    #[test]
    fn map_content_recurse_body_e_attribution() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s, _) if s.as_str() == "vida" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match ex().map_content(&mut f).unwrap() {
            Content::Quote(e) => {
                assert!(matches!(&e.body, Content::Text(s, _) if s.as_str() == "Z"));
                assert!(e.block && e.quotes);
            }
            _ => panic!("esperado Quote"),
        }
    }

    fn h(e: &QuoteElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        let mut b = ex();
        b.quotes = false;
        assert_ne!(h(&ex()), h(&b));
    }
}
