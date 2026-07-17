//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/link.md
//! @prompt-hash 865ff477
//! @layer L1
//! @updated 2026-06-11
//!
//! `LinkElem` — Lote 3 P318 (família lista/termos). Hiperligação.
//! Contentor de prosa: recurse em map_content E map_text (precedente Heading).

use std::sync::Arc;

use ecow::EcoString;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Hiperligação (`https://...`).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct LinkElem {
    pub url:  EcoString,
    pub body: Content,
}

impl Element for LinkElem {
    fn plain_text(&self) -> String {
        // Transparente: o texto plano é o do corpo (content.rs:1637).
        self.body.plain_text()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Link(Arc::new(LinkElem {
            url:  self.url.clone(),
            body: self.body.map_content(transform)?,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Link(Arc::new(LinkElem {
            url:  self.url.clone(),
            body: self.body.map_text(transform),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lk() -> LinkElem {
        LinkElem { url: "https://x".into(), body: Content::text("texto") }
    }

    #[test]
    fn plain_text_transparente() {
        assert_eq!(lk().plain_text(), "texto");
    }

    #[test]
    fn map_content_preserva_url() {
        let mut f = |_c: &Content| -> SourceResult<Option<Content>> { Ok(None) };
        match lk().map_content(&mut f).unwrap() {
            Content::Link(e) => assert_eq!(e.url.as_str(), "https://x"),
            _ => panic!("esperado Link"),
        }
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(lk(), lk());
        let mut other = lk();
        other.url = "https://y".into();
        assert_ne!(lk(), other);
    }
}
