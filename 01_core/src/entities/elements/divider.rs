//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/divider.md
//! @prompt-hash 20ad8533
//! @layer L1
//! @updated 2026-06-10
//!
//! `DividerElem` — singleton estrutural (P154B). Piso do lote piloto D
//! (sem campos — prova o mecanismo).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Divisor estrutural singleton. Sem campos.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct DividerElem;

impl Element for DividerElem {
    fn plain_text(&self) -> String {
        String::new()
    }

    fn is_empty(&self) -> bool {
        // Singleton estrutural: nunca vazio (paridade content.rs:1512).
        false
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Divider(Arc::new(DividerElem)))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Divider(Arc::new(DividerElem))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_vazio() {
        assert_eq!(DividerElem.plain_text(), "");
    }

    #[test]
    fn nunca_vazio() {
        assert!(!DividerElem.is_empty());
    }

    #[test]
    fn igualdade_singleton() {
        assert_eq!(DividerElem, DividerElem);
    }

    #[test]
    fn map_content_terminal() {
        let mut f = |_c: &Content| -> SourceResult<Option<Content>> { Ok(None) };
        let r = DividerElem.map_content(&mut f).unwrap();
        assert!(matches!(r, Content::Divider(_)));
    }
}
