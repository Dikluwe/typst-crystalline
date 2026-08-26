//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/transform.md
//! @prompt-hash b47feb7b
//! @layer L1
//! @updated 2026-06-11
//!
//! `TransformElem` — Lote 9 P324 (por largura). `transform(matrix, body)`.
//! Contentor: recurse no body em map_content E map_text.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::layout_types::TransformMatrix;
use crate::entities::source_result::SourceResult;

/// Aplica `matrix` (transformação afim 2D) ao `body`.
#[derive(Debug, Clone, PartialEq)]
pub struct TransformElem {
    pub matrix: TransformMatrix,
    pub body: Content,
}

// `Hash` manual via `Debug` (paridade `content_hash`): `TransformMatrix` é
// 6×`f64`. Ressalva `-0.0` vs `0.0` aceitável (`…Elem` não são chaves de mapa).
impl std::hash::Hash for TransformElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for TransformElem {
    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    // `is_empty` fica no default `false` — paridade `content.rs` (`Transform`
    // cai em `_ => false`, não delega ao body). Precedente Align (Lote 7).

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Transform(Arc::new(TransformElem {
            matrix: self.matrix,
            body: self.body.map_content(transform)?,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Transform(Arc::new(TransformElem {
            matrix: self.matrix,
            body: self.body.map_text(transform),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn ex() -> TransformElem {
        TransformElem {
            matrix: TransformMatrix::identity(),
            body: Content::text("a"),
        }
    }

    #[test]
    fn plain_text_delega_ao_body() {
        assert_eq!(ex().plain_text(), "a");
    }

    #[test]
    fn is_empty_default_false_nao_delega() {
        // Mesmo com body Empty, Transform não é "vazio" (paridade hub _ => false).
        let t = TransformElem {
            matrix: TransformMatrix::identity(),
            body: Content::Empty,
        };
        assert!(!t.is_empty());
    }

    #[test]
    fn map_content_recurse_body_preserva_matrix() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "a" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match ex().map_content(&mut f).unwrap() {
            Content::Transform(e) => {
                assert_eq!(e.matrix, TransformMatrix::identity());
                assert!(matches!(&e.body, Content::Text(s) if s.as_str() == "Z"));
            }
            _ => panic!("esperado Transform"),
        }
    }

    fn h(e: &TransformElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        let outro = TransformElem {
            matrix: TransformMatrix::identity(),
            body: Content::text("b"),
        };
        assert_ne!(h(&ex()), h(&outro));
    }
}
