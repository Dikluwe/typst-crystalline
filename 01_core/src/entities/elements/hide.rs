//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/hide.md
//! @prompt-hash 00077ced
//! @layer L1
//! @updated 2026-06-11
//!
//! `HideElem` — Lote 7 P322. `hide(body)` — ocupa espaço, invisível.
//! Contentor: recurse em map_content E map_text.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Esconde o `body` (reserva o espaço de layout, não desenha).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct HideElem {
    pub body: Content,
}

impl Element for HideElem {
    fn plain_text(&self) -> String {
        // Invisível — sem texto plano (content.rs:1812).
        String::new()
    }

    fn is_empty(&self) -> bool {
        self.body.is_empty()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Hide(Arc::new(HideElem { body: self.body.map_content(transform)? })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Hide(Arc::new(HideElem { body: self.body.map_text(transform) }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_vazio_mesmo_com_body() {
        assert_eq!(HideElem { body: Content::text("secret") }.plain_text(), "");
    }

    #[test]
    fn is_empty_delega_ao_body() {
        assert!(!HideElem { body: Content::text("x") }.is_empty());
        assert!(HideElem { body: Content::Empty }.is_empty());
    }

    #[test]
    fn map_content_recurse_body() {
        let e = HideElem { body: Content::text("a") };
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "a" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match e.map_content(&mut f).unwrap() {
            Content::Hide(el) => {
                assert!(matches!(&el.body, Content::Text(s) if s.as_str() == "Z"))
            }
            _ => panic!("esperado Hide"),
        }
    }
}
