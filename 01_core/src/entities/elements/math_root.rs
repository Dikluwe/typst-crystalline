//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_root.md
//! @prompt-hash a2864f85
//! @layer L1
//! @updated 2026-06-11
//!
//! `MathRootElem` — Lote 2 P317 (família math). Raiz (`sqrt`/`root`).
//! Comportamento idêntico ao braço anterior do hub (content-preserving).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Raiz matemática (`√x`, `∛x`). `index`: `None` = raiz quadrada,
/// `Some(n)` = raiz n-ésima.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathRootElem {
    pub index: Option<Content>,
    pub radicand: Content,
}

impl Element for MathRootElem {
    fn plain_text(&self) -> String {
        match &self.index {
            None => format!("sqrt({})", self.radicand.plain_text()),
            Some(i) => {
                format!("root({}, {})", i.plain_text(), self.radicand.plain_text())
            }
        }
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::MathRoot(Arc::new(MathRootElem {
            index: self.index.as_ref().map(|c| c.map_content(transform)).transpose()?,
            radicand: self.radicand.map_content(transform)?,
        })))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        // Terminal (math structural; não desce).
        Content::MathRoot(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_sqrt_e_root() {
        let sq = MathRootElem { index: None, radicand: Content::text("x") };
        assert_eq!(sq.plain_text(), "sqrt(x)");
        let nth = MathRootElem {
            index: Some(Content::text("3")),
            radicand: Content::text("x"),
        };
        assert_eq!(nth.plain_text(), "root(3, x)");
    }

    #[test]
    fn igualdade_estrutural() {
        let a = MathRootElem { index: None, radicand: Content::text("x") };
        let b = MathRootElem {
            index: Some(Content::text("2")),
            radicand: Content::text("x"),
        };
        assert_eq!(a.clone(), a.clone());
        assert_ne!(a, b);
    }

    #[test]
    fn map_content_recurse_preserva_none() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "x" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        let sq = MathRootElem { index: None, radicand: Content::text("x") };
        match sq.map_content(&mut f).unwrap() {
            Content::MathRoot(e) => {
                assert!(e.index.is_none());
                assert_eq!(e.plain_text(), "sqrt(Z)");
            }
            _ => panic!("esperado MathRoot"),
        }
    }
}
