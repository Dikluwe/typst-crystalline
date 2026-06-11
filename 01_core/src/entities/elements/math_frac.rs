//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_frac.md
//! @prompt-hash eae6a939
//! @layer L1
//! @updated 2026-06-11
//!
//! `MathFracElem` — Lote 2 P317 (família math). Fracção (`a/b`, `frac(a, b)`).
//! Comportamento idêntico ao braço anterior do hub (content-preserving).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Fracção matemática (`a/b` ou `frac(a, b)`).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathFracElem {
    pub num: Content,
    pub den: Content,
}

impl Element for MathFracElem {
    fn plain_text(&self) -> String {
        format!("({})/({})", self.num.plain_text(), self.den.plain_text())
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::MathFrac(Arc::new(MathFracElem {
            num: self.num.map_content(transform)?,
            den: self.den.map_content(transform)?,
        })))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        // Terminal (math structural; não desce).
        Content::MathFrac(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> MathFracElem {
        MathFracElem { num: Content::text("a"), den: Content::text("b") }
    }

    #[test]
    fn plain_text_formato() {
        assert_eq!(ex().plain_text(), "(a)/(b)");
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(ex(), ex());
        let mut other = ex();
        other.den = Content::text("c");
        assert_ne!(ex(), other);
    }

    #[test]
    fn map_content_recurse_ambos() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s, _) if s.as_str() == "b" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match ex().map_content(&mut f).unwrap() {
            Content::MathFrac(e) => assert_eq!(e.plain_text(), "(a)/(Z)"),
            _ => panic!("esperado MathFrac"),
        }
    }
}
