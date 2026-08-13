//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_align_point.md
//! @prompt-hash 1e0e41f5
//! @layer L1
//! @updated 2026-06-11
//!
//! `MathAlignPointElem` — Lote 2 P317 (família math). Ponto de alinhamento `&`.
//! Marcador unit; comportamento idêntico ao braço anterior do hub.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Ponto de alinhamento em equações (`&`). Separa colunas no layout de grelha.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathAlignPointElem;

impl Element for MathAlignPointElem {
    fn plain_text(&self) -> String {
        String::new()
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        // Terminal (marcador unit; sem filhos).
        Ok(Content::MathAlignPoint(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        // Terminal (math structural; não desce).
        Content::MathAlignPoint(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_vazio() {
        assert_eq!(MathAlignPointElem.plain_text(), "");
    }

    #[test]
    fn igualdade_trivial() {
        assert_eq!(MathAlignPointElem, MathAlignPointElem);
    }

    #[test]
    fn map_content_terminal() {
        let mut f = |_c: &Content| -> SourceResult<Option<Content>> { Ok(None) };
        let r = MathAlignPointElem.map_content(&mut f).unwrap();
        assert!(matches!(r, Content::MathAlignPoint(_)));
    }
}
