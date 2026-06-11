//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_accent.md
//! @prompt-hash aa6a0bd0
//! @layer L1
//! @updated 2026-06-11
//!
//! `MathAccentElem` — Lote 2 P317 (família math). Acento matemático (P296).
//! Comportamento idêntico ao braço anterior do hub (content-preserving).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Acento matemático — vanilla `AccentElem` minimal (P296). Layouter posiciona
/// `accent` centrado horizontalmente acima da `base`.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathAccentElem {
    pub base:   Content,
    pub accent: Content,
}

impl Element for MathAccentElem {
    fn plain_text(&self) -> String {
        format!("{}{}", self.base.plain_text(), self.accent.plain_text())
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::MathAccent(Arc::new(MathAccentElem {
            base:   self.base.map_content(transform)?,
            accent: self.accent.map_content(transform)?,
        })))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        // Terminal (math structural; não desce).
        Content::MathAccent(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> MathAccentElem {
        MathAccentElem { base: Content::text("x"), accent: Content::text("^") }
    }

    #[test]
    fn plain_text_concat() {
        assert_eq!(ex().plain_text(), "x^");
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(ex(), ex());
        let mut other = ex();
        other.accent = Content::text("~");
        assert_ne!(ex(), other);
    }

    #[test]
    fn map_content_recurse_ambos() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s, _) if s.as_str() == "x" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        let r = ex().map_content(&mut f).unwrap();
        match r {
            Content::MathAccent(e) => assert_eq!(e.plain_text(), "Z^"),
            _ => panic!("esperado MathAccent"),
        }
    }
}
