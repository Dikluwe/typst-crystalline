//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_underover.md
//! @prompt-hash 687ad761
//! @layer L1
//! @updated 2026-06-11
//!
//! `MathUnderoverElem` — Lote 2 P317 (família math). Anotações verticais (P297).
//! Comportamento idêntico ao braço anterior do hub (content-preserving).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Anotações verticais sobre/sob conteúdo matemático — variant agregado P297
/// (ADR-0054 graded). Layouter empilha verticalmente conforme presença de cada.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathUnderoverElem {
    pub base: Content,
    pub under: Option<Content>,
    pub over: Option<Content>,
}

impl Element for MathUnderoverElem {
    fn plain_text(&self) -> String {
        let o = self.over.as_ref().map(|c| c.plain_text()).unwrap_or_default();
        let b = self.base.plain_text();
        let u = self.under.as_ref().map(|c| c.plain_text()).unwrap_or_default();
        format!("{}{}{}", o, b, u)
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::MathUnderover(Arc::new(MathUnderoverElem {
            base: self.base.map_content(transform)?,
            under: self.under.as_ref().map(|c| c.map_content(transform)).transpose()?,
            over: self.over.as_ref().map(|c| c.map_content(transform)).transpose()?,
        })))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        // Terminal (math structural; não desce).
        Content::MathUnderover(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> MathUnderoverElem {
        MathUnderoverElem {
            base: Content::text("x"),
            under: Some(Content::text("u")),
            over: Some(Content::text("o")),
        }
    }

    #[test]
    fn plain_text_ordem_visual() {
        assert_eq!(ex().plain_text(), "oxu");
        let so = MathUnderoverElem { base: Content::text("x"), under: None, over: None };
        assert_eq!(so.plain_text(), "x");
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(ex(), ex());
        let mut other = ex();
        other.under = None;
        assert_ne!(ex(), other);
    }

    #[test]
    fn map_content_recurse_preserva_none() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "x" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        let so = MathUnderoverElem { base: Content::text("x"), under: None, over: None };
        match so.map_content(&mut f).unwrap() {
            Content::MathUnderover(e) => {
                assert!(e.under.is_none() && e.over.is_none());
                assert_eq!(e.plain_text(), "Z");
            }
            _ => panic!("esperado MathUnderover"),
        }
    }
}
