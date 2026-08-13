//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_styled.md
//! @prompt-hash 38a184b1
//! @layer L1
//! @updated 2026-06-10
//!
//! `MathStyledElem` — meio do lote piloto D (campos + math layout).
//! Mecanismo do variant: ADR-0102/0103.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::math_style::MathStyleKind;
use crate::entities::source_result::SourceResult;

/// Wrap de estilo matemático (`bb`/`bold`/`cal`/…). Composição outer-wins
/// resolvida no `MathLayouter` (rules/math/layout).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathStyledElem {
    pub kind: Option<MathStyleKind>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub body: Content,
    pub cramped: Option<bool>,
}

impl Element for MathStyledElem {
    fn plain_text(&self) -> String {
        // Transparente para plain_text (content.rs:1642).
        self.body.plain_text()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        // Recursivo no body; preserva kind/flags (content.rs:2145).
        Ok(Content::MathStyled(Arc::new(MathStyledElem {
            kind: self.kind,
            bold: self.bold,
            italic: self.italic,
            body: self.body.map_content(transform)?,
            cramped: self.cramped,
        })))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        // Terminal em map_text (math structural; content.rs:2636).
        Content::MathStyled(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ms() -> MathStyledElem {
        MathStyledElem {
            kind: Some(MathStyleKind::DoubleStruck),
            bold: None,
            italic: None,
            body: Content::text("x"),
            cramped: None,
        }
    }

    #[test]
    fn plain_text_transparente() {
        assert_eq!(ms().plain_text(), "x");
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(ms(), ms());
        let mut other = ms();
        other.bold = Some(true);
        assert_ne!(ms(), other);
    }

    #[test]
    fn map_content_preserva_kind() {
        let mut f = |_c: &Content| -> SourceResult<Option<Content>> { Ok(None) };
        let r = ms().map_content(&mut f).unwrap();
        match r {
            Content::MathStyled(e) => {
                assert_eq!(e.kind, Some(MathStyleKind::DoubleStruck))
            }
            _ => panic!("esperado MathStyled"),
        }
    }
}
