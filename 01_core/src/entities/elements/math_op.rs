//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_op.md
//! @prompt-hash b9e3c701
//! @layer L1
//! @updated 2026-06-11
//!
//! `MathOpElem` — Lote 2 P317 (família math). Operador textual (P298).
//! Comportamento idêntico ao braço anterior do hub (content-preserving).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Operador textual matemático — vanilla `OpElem` (P298). `limits` é
/// discriminador de layout cross-variant (afeta o `MathAttach` pai).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathOpElem {
    pub text:   Content,
    pub limits: bool,
}

impl Element for MathOpElem {
    fn plain_text(&self) -> String {
        // `limits` é discriminador de layout; plain_text só o texto.
        self.text.plain_text()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::MathOp(Arc::new(MathOpElem {
            text:   self.text.map_content(transform)?,
            limits: self.limits,
        })))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        // Terminal (math structural; não desce).
        Content::MathOp(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> MathOpElem {
        MathOpElem { text: Content::text("lim"), limits: true }
    }

    #[test]
    fn plain_text_so_texto() {
        assert_eq!(ex().plain_text(), "lim");
    }

    #[test]
    fn igualdade_preserva_limits() {
        assert_eq!(ex(), ex());
        let mut other = ex();
        other.limits = false;
        assert_ne!(ex(), other);
    }

    #[test]
    fn map_content_recurse_preserva_limits() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "lim" => Ok(Some(Content::text("max"))),
                _ => Ok(None),
            }
        };
        match ex().map_content(&mut f).unwrap() {
            Content::MathOp(e) => {
                assert_eq!(e.plain_text(), "max");
                assert!(e.limits);
            }
            _ => panic!("esperado MathOp"),
        }
    }
}
