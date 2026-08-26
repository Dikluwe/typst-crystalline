//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/block.md
//! @prompt-hash 6adaed6a
//! @layer L1
//! @updated 2026-06-12
//!
//! `BlockElem` — Lote 15 P330 (lote tardio da triagem DEBT-58; o ÚLTIMO lote).
//! `block(body, …)` — container block denso (14 campos). Contentor: recurse no
//! body. Não-locatável.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::corners::Corners;
use crate::entities::elements::Element;
use crate::entities::geometry::Stroke;
use crate::entities::layout_types::{Color, Length};
use crate::entities::sides::Sides;
use crate::entities::source_result::SourceResult;

/// Container block (`block`). `body` + 13 cosméticos.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockElem {
    pub body: Content,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub inset: Sides<Length>,
    pub breakable: bool,
    pub outset: Sides<Length>,
    pub radius: Corners<Length>,
    pub clip: bool,
    pub fill: Option<Color>,
    pub stroke: Option<Stroke>,
    pub spacing: Option<Length>,
    pub above: Option<Length>,
    pub below: Option<Length>,
    pub sticky: bool,
}

// `Hash` manual via `Debug` (paridade `content_hash`): `Length`/`Sides`/`Corners`/
// `Color`/`Stroke` carregam `f64`. Ressalva `-0.0` vs `0.0` aceitável.
impl std::hash::Hash for BlockElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for BlockElem {
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
        Ok(Content::Block(Arc::new(BlockElem {
            body: self.body.map_content(transform)?,
            ..(*self).clone()
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Block(Arc::new(BlockElem {
            body: self.body.map_text(transform),
            ..(*self).clone()
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn ex() -> BlockElem {
        BlockElem {
            body: Content::text("a"),
            width: None,
            height: None,
            inset: Sides::uniform(Length::pt(0.0)),
            breakable: true,
            outset: Sides::uniform(Length::pt(0.0)),
            radius: Corners::uniform(Length::ZERO),
            clip: false,
            fill: None,
            stroke: None,
            spacing: None,
            above: None,
            below: None,
            sticky: false,
        }
    }

    #[test]
    fn plain_text_delega_ao_body() {
        assert_eq!(ex().plain_text(), "a");
    }

    #[test]
    fn is_empty_delega_ao_body() {
        assert!(!ex().is_empty());
        let mut v = ex();
        v.body = Content::Empty;
        assert!(v.is_empty());
    }

    #[test]
    fn map_content_recurse_body_preserva_cosmeticos() {
        let mut v = ex();
        v.sticky = true;
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "a" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match v.map_content(&mut f).unwrap() {
            Content::Block(e) => {
                assert!(e.sticky);
                assert!(matches!(&e.body, Content::Text(s) if s.as_str() == "Z"));
            }
            _ => panic!("esperado Block"),
        }
    }

    fn h(e: &BlockElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        let mut v = ex();
        v.sticky = true;
        assert_ne!(h(&ex()), h(&v));
    }
}
