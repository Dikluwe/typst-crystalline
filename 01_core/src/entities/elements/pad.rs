//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/pad.md
//! @prompt-hash 2885bfcd
//! @layer L1
//! @updated 2026-06-11
//!
//! `PadElem` — Lote 10 P325 (por largura). `pad(body, sides)` — margem interna.
//! Contentor: recurse no body em map_content E map_text.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::layout_types::Length;
use crate::entities::sides::Sides;
use crate::entities::source_result::SourceResult;

/// Adiciona margem (`sides`) à volta do `body`.
#[derive(Debug, Clone, PartialEq)]
pub struct PadElem {
    pub body: Content,
    pub sides: Sides<Option<Length>>,
}

// `Hash` manual via `Debug` (paridade `content_hash`): `Sides<Option<Length>>`
// carrega `f64`. Ressalva `-0.0` vs `0.0` aceitável (`…Elem` não são chaves de mapa).
impl std::hash::Hash for PadElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for PadElem {
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
        Ok(Content::Pad(Arc::new(PadElem {
            body: self.body.map_content(transform)?,
            sides: self.sides,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Pad(Arc::new(PadElem {
            body: self.body.map_text(transform),
            sides: self.sides,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn ex() -> PadElem {
        PadElem { body: Content::text("a"), sides: Sides::default() }
    }

    #[test]
    fn plain_text_delega_ao_body() {
        assert_eq!(ex().plain_text(), "a");
    }

    #[test]
    fn is_empty_delega_ao_body() {
        assert!(!ex().is_empty());
        assert!(PadElem { body: Content::Empty, sides: Sides::default() }.is_empty());
    }

    #[test]
    fn map_content_recurse_body_preserva_sides() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "a" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match ex().map_content(&mut f).unwrap() {
            Content::Pad(e) => {
                assert!(matches!(&e.body, Content::Text(s) if s.as_str() == "Z"))
            }
            _ => panic!("esperado Pad"),
        }
    }

    fn h(e: &PadElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        assert_ne!(
            h(&ex()),
            h(&PadElem { body: Content::text("b"), sides: Sides::default() })
        );
    }
}
