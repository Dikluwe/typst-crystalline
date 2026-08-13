//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/place.md
//! @prompt-hash bc2e9c07
//! @layer L1
//! @updated 2026-06-11
//!
//! `PlaceElem` — Lote 9 P324 (por largura). `place(...)` — posicionamento
//! absoluto/flutuante. Contentor: recurse no body em map_content E map_text.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::layout_types::{Align2D, Length, PlaceScope};
use crate::entities::source_result::SourceResult;

/// Posiciona o `body` com `alignment` + offset `dx`/`dy`; `scope`/`float`/
/// `clearance` controlam o regime de fluxo.
#[derive(Debug, Clone, PartialEq)]
pub struct PlaceElem {
    pub alignment: Align2D,
    pub dx: f64,
    pub dy: f64,
    pub scope: PlaceScope,
    pub float: bool,
    pub clearance: Option<Length>,
    pub body: Content,
}

// `Hash` manual via `Debug` (paridade `content_hash`): `dx`/`dy` são `f64`;
// `Align2D`/`Option<Length>` também não implementam `Hash`. Ressalva `-0.0` vs
// `0.0` aceitável (`…Elem` não são chaves de mapa).
impl std::hash::Hash for PlaceElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for PlaceElem {
    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    // `is_empty` fica no default `false` — paridade `content.rs` (`Place` cai
    // em `_ => false`, não delega ao body). Precedente Align (Lote 7).

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Place(Arc::new(PlaceElem {
            alignment: self.alignment,
            dx: self.dx,
            dy: self.dy,
            scope: self.scope,
            float: self.float,
            clearance: self.clearance,
            body: self.body.map_content(transform)?,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Place(Arc::new(PlaceElem {
            alignment: self.alignment,
            dx: self.dx,
            dy: self.dy,
            scope: self.scope,
            float: self.float,
            clearance: self.clearance,
            body: self.body.map_text(transform),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn ex() -> PlaceElem {
        PlaceElem {
            alignment: Align2D::default(),
            dx: 0.0,
            dy: 0.0,
            scope: PlaceScope::default(),
            float: false,
            clearance: None,
            body: Content::text("a"),
        }
    }

    #[test]
    fn plain_text_delega_ao_body() {
        assert_eq!(ex().plain_text(), "a");
    }

    #[test]
    fn is_empty_default_false_nao_delega() {
        let mut p = ex();
        p.body = Content::Empty;
        assert!(!p.is_empty());
    }

    #[test]
    fn map_content_recurse_body_preserva_campos() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "a" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match ex().map_content(&mut f).unwrap() {
            Content::Place(e) => {
                assert!(!e.float);
                assert!(matches!(&e.body, Content::Text(s) if s.as_str() == "Z"));
            }
            _ => panic!("esperado Place"),
        }
    }

    fn h(e: &PlaceElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        let mut outro = ex();
        outro.dx = 5.0;
        assert_ne!(h(&ex()), h(&outro));
    }
}
