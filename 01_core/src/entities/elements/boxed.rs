//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/boxed.md
//! @prompt-hash 2326c6dc
//! @layer L1
//! @updated 2026-06-12
//!
//! `BoxedElem` — Lote 14 P329 (reclassificado da triagem DEBT-58). `box(body, …)`
//! — container inline denso (~10 campos). Contentor: recurse no body. Não-locatável.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::corners::Corners;
use crate::entities::elements::Element;
use crate::entities::geometry::Stroke;
use crate::entities::layout_types::{Color, Length};
use crate::entities::sides::Sides;
use crate::entities::source_result::SourceResult;

/// Container inline (`box`). `body` + 9 cosméticos de caixa.
#[derive(Debug, Clone, PartialEq)]
pub struct BoxedElem {
    pub body:     Content,
    pub width:    Option<Length>,
    pub height:   Option<Length>,
    pub inset:    Sides<Length>,
    pub baseline: Length,
    pub outset:   Sides<Length>,
    pub radius:   Corners<Length>,
    pub clip:     bool,
    pub fill:     Option<Color>,
    pub stroke:   Option<Stroke>,
}

// `Hash` manual via `Debug` (paridade `content_hash`): `Length`/`Sides`/`Corners`/
// `Color`/`Stroke` carregam `f64`. Ressalva `-0.0` vs `0.0` aceitável.
impl std::hash::Hash for BoxedElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for BoxedElem {
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
        Ok(Content::Boxed(Arc::new(BoxedElem {
            body:     self.body.map_content(transform)?,
            width:    self.width,
            height:   self.height,
            inset:    self.inset,
            baseline: self.baseline,
            outset:   self.outset,
            radius:   self.radius,
            clip:     self.clip,
            fill:     self.fill,
            stroke:   self.stroke.clone(),
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Boxed(Arc::new(BoxedElem {
            body:     self.body.map_text(transform),
            width:    self.width,
            height:   self.height,
            inset:    self.inset,
            baseline: self.baseline,
            outset:   self.outset,
            radius:   self.radius,
            clip:     self.clip,
            fill:     self.fill,
            stroke:   self.stroke.clone(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    fn ex() -> BoxedElem {
        BoxedElem {
            body: Content::text("a"),
            width: None, height: None,
            inset: Sides::uniform(Length::pt(0.0)),
            baseline: Length::pt(0.0),
            outset: Sides::uniform(Length::pt(0.0)),
            radius: Corners::uniform(Length::ZERO),
            clip: false, fill: None, stroke: None,
        }
    }

    #[test]
    fn plain_text_delega_ao_body() {
        assert_eq!(ex().plain_text(), "a");
    }

    #[test]
    fn is_empty_delega_ao_body() {
        assert!(!ex().is_empty());
        let mut v = ex(); v.body = Content::Empty;
        assert!(v.is_empty());
    }

    #[test]
    fn map_content_recurse_body_preserva_cosmeticos() {
        let mut v = ex(); v.clip = true;
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "a" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match v.map_content(&mut f).unwrap() {
            Content::Boxed(e) => {
                assert!(e.clip);
                assert!(matches!(&e.body, Content::Text(s) if s.as_str() == "Z"));
            }
            _ => panic!("esperado Boxed"),
        }
    }

    fn h(e: &BoxedElem) -> u64 {
        let mut s = DefaultHasher::new(); e.hash(&mut s); s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        let mut v = ex(); v.clip = true;
        assert_ne!(h(&ex()), h(&v));
    }
}
