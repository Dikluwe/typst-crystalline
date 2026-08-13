//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_cancel.md
//! @prompt-hash 76da6fb2
//! @layer L1
//! @updated 2026-06-11
//!
//! `MathCancelElem` — Lote 2 P317 (família math). Linha de cancelamento (P296).
//! Comportamento idêntico ao braço anterior do hub (content-preserving).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Linha de cancelamento sobre conteúdo matemático — vanilla `CancelElem`
/// minimal (P296). Layouter emite `FrameItem::Line` diagonal sobre o bbox.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathCancelElem {
    pub body: Content,
}

impl Element for MathCancelElem {
    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::MathCancel(Arc::new(MathCancelElem {
            body: self.body.map_content(transform)?,
        })))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        // Terminal (math structural; não desce).
        Content::MathCancel(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> MathCancelElem {
        MathCancelElem { body: Content::text("xy") }
    }

    #[test]
    fn plain_text_transparente() {
        assert_eq!(ex().plain_text(), "xy");
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(ex(), ex());
        let mut other = ex();
        other.body = Content::text("z");
        assert_ne!(ex(), other);
    }

    #[test]
    fn map_content_recurse_body() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "xy" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        let r = ex().map_content(&mut f).unwrap();
        match r {
            Content::MathCancel(e) => assert_eq!(e.plain_text(), "Z"),
            _ => panic!("esperado MathCancel"),
        }
    }
}
