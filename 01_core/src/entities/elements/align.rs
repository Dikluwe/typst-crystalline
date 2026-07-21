//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/align.md
//! @prompt-hash 5fabaf88
//! @layer L1
//! @updated 2026-06-11
//!
//! `AlignElem` — Lote 7 P322. Alinhamento de bloco (`align(alignment, body)`).
//! Contentor: recurse em map_content E map_text (precedente Heading).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::layout_types::Align2D;
use crate::entities::source_result::SourceResult;

/// Alinhamento horizontal/vertical aplicado ao `body`.
#[derive(Debug, Clone, PartialEq)]
pub struct AlignElem {
    pub alignment: Align2D,
    pub body: Content,
}

// `Hash` manual via `Debug` (paridade `content_hash`): `Align2D` deriva
// `Debug, Clone, Copy, PartialEq, Default` mas **não** `Hash`.
impl std::hash::Hash for AlignElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for AlignElem {
    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Align(Arc::new(AlignElem {
            alignment: self.alignment,
            body: self.body.map_content(transform)?,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Align(Arc::new(AlignElem {
            alignment: self.alignment,
            body: self.body.map_text(transform),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> AlignElem {
        AlignElem {
            alignment: Align2D::default(),
            body: Content::text("x"),
        }
    }

    #[test]
    fn plain_text_do_body() {
        assert_eq!(ex().plain_text(), "x");
    }

    #[test]
    fn map_text_recurse_preserva_alignment() {
        match ex().map_text(&mut |s| s.to_uppercase()) {
            Content::Align(e) => {
                assert_eq!(e.plain_text(), "X");
                assert_eq!(e.alignment, Align2D::default());
            }
            _ => panic!("esperado Align"),
        }
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(ex(), ex());
        assert_ne!(
            ex(),
            AlignElem {
                alignment: Align2D::default(),
                body: Content::text("y")
            }
        );
    }
}
