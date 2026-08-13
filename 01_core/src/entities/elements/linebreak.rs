//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/linebreak.md
//! @prompt-hash 39a7dc96
//! @layer L1
//! @updated 2026-06-11
//!
//! `LinebreakElem` — Lote 5 P320. Quebra de linha (`\\`). Comando unit.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Quebra de linha forçada (`\\`).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct LinebreakElem;

impl Element for LinebreakElem {
    fn plain_text(&self) -> String {
        "\n".to_string()
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Linebreak(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Linebreak(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_newline() {
        assert_eq!(LinebreakElem.plain_text(), "\n");
    }

    #[test]
    fn is_empty_default_false() {
        assert!(!LinebreakElem.is_empty());
    }

    #[test]
    fn igualdade_trivial() {
        assert_eq!(LinebreakElem, LinebreakElem);
    }
}
