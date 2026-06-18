//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_attach.md
//! @prompt-hash b7df5f87
//! @layer L1
//! @updated 2026-06-11
//!
//! `MathAttachElem` — Lote 2 P317 (família math). Base com scripts (`x_1^2`).
//! Comportamento idêntico ao braço anterior do hub (content-preserving).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Base com índice e/ou expoente (`x_1^2`, `{}^{14}_6 C`). `tl`/`bl` =
/// pre-scripts à esquerda; `sub`/`sup` = scripts à direita.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathAttachElem {
    pub base: Content,
    pub tl:   Option<Content>,
    pub bl:   Option<Content>,
    pub sub:  Option<Content>,
    pub sup:  Option<Content>,
}

impl Element for MathAttachElem {
    fn plain_text(&self) -> String {
        let mut s = String::new();
        if let Some(tl) = &self.tl { s.push_str(&format!("^{}", tl.plain_text())); }
        if let Some(bl) = &self.bl { s.push_str(&format!("_{}", bl.plain_text())); }
        s.push_str(&self.base.plain_text());
        if let Some(sub) = &self.sub { s.push_str(&format!("_{}", sub.plain_text())); }
        if let Some(sup) = &self.sup { s.push_str(&format!("^{}", sup.plain_text())); }
        s
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::MathAttach(Arc::new(MathAttachElem {
            base: self.base.map_content(transform)?,
            tl:   self.tl.as_ref().map(|c| c.map_content(transform)).transpose()?,
            bl:   self.bl.as_ref().map(|c| c.map_content(transform)).transpose()?,
            sub:  self.sub.as_ref().map(|c| c.map_content(transform)).transpose()?,
            sup:  self.sup.as_ref().map(|c| c.map_content(transform)).transpose()?,
        })))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        // Terminal (math structural; não desce).
        Content::MathAttach(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> MathAttachElem {
        MathAttachElem {
            base: Content::text("x"),
            tl:   None,
            bl:   None,
            sub:  Some(Content::text("1")),
            sup:  Some(Content::text("2")),
        }
    }

    #[test]
    fn plain_text_ordem() {
        assert_eq!(ex().plain_text(), "x_1^2");
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(ex(), ex());
        let mut other = ex();
        other.sup = None;
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
        match ex().map_content(&mut f).unwrap() {
            Content::MathAttach(e) => {
                assert!(e.tl.is_none() && e.bl.is_none());
                assert_eq!(e.plain_text(), "Z_1^2");
            }
            _ => panic!("esperado MathAttach"),
        }
    }
}
