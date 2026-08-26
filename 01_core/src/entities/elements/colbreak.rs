//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/colbreak.md
//! @prompt-hash a98c5f8d
//! @layer L1
//! @updated 2026-06-11
//!
//! `ColbreakElem` — Lote 5 P320. Quebra de coluna (`colbreak()`). Event leaf.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Quebra de coluna. `weak` é colapsável; nunca vazio (event observable).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ColbreakElem {
    pub weak: bool,
    /// Se `weak` foi fornecido explicitamente na chamada Typst.
    pub weak_explicit: bool,
}

impl Element for ColbreakElem {
    fn plain_text(&self) -> String {
        String::new()
    }

    fn is_empty(&self) -> bool {
        // Event observable — nunca vazio (paridade Pagebreak/Divider).
        false
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Colbreak(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Colbreak(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_vazio() {
        assert_eq!(ColbreakElem { weak: false, weak_explicit: false }.plain_text(), "");
    }

    #[test]
    fn is_empty_sempre_false() {
        assert!(!ColbreakElem { weak: true, weak_explicit: true }.is_empty());
        assert!(!ColbreakElem { weak: false, weak_explicit: false }.is_empty());
    }

    #[test]
    fn igualdade_por_weak() {
        assert_eq!(
            ColbreakElem { weak: true, weak_explicit: true },
            ColbreakElem { weak: true, weak_explicit: true }
        );
        assert_ne!(
            ColbreakElem { weak: true, weak_explicit: true },
            ColbreakElem { weak: false, weak_explicit: false }
        );
        assert_ne!(
            ColbreakElem { weak: false, weak_explicit: false },
            ColbreakElem { weak: false, weak_explicit: true }
        );
    }
}
