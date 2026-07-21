//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/state_display.md
//! @prompt-hash b25f5e73
//! @layer L1
//! @updated 2026-06-11
//!
//! `StateDisplayElem` — Lote 6 P321 (família state/counter). P240.
//! **Locatável** — absorve `extract_payload`.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::Element;
use crate::entities::func::Func;
use crate::entities::source_result::SourceResult;

/// `state(key).display(callback)` — valor pré-renderizado pós-fixpoint.
#[derive(Debug, Clone, PartialEq)]
pub struct StateDisplayElem {
    pub key: String,
    pub callback: Option<Func>,
}

// `Hash` manual via `Debug` (`Func`; paridade `content_hash`).
impl std::hash::Hash for StateDisplayElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for StateDisplayElem {
    fn plain_text(&self) -> String {
        String::new()
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::StateDisplay(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::StateDisplay(Arc::new(self.clone()))
    }

    fn element_kind(&self) -> Option<ElementKind> {
        Some(ElementKind::StateDisplay)
    }

    fn to_payload(&self) -> Option<ElementPayload> {
        Some(ElementPayload::StateDisplay {
            key: self.key.clone(),
            callback: self.callback.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> StateDisplayElem {
        StateDisplayElem { key: "ctr".into(), callback: None }
    }

    #[test]
    fn locatavel_state_display() {
        assert_eq!(ex().element_kind(), Some(ElementKind::StateDisplay));
        assert!(matches!(ex().to_payload(), Some(ElementPayload::StateDisplay { .. })));
    }

    #[test]
    fn igualdade_por_key() {
        assert_eq!(ex(), ex());
        assert_ne!(ex(), StateDisplayElem { key: "other".into(), callback: None });
    }
}
