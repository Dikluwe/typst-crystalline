//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/counter_display_callback.md
//! @prompt-hash 75d4d661
//! @layer L1
//! @updated 2026-06-11
//!
//! `CounterDisplayCallbackElem` — Lote 6 P321 (família state/counter). P241.
//! **Locatável** (kind → CounterDisplay) — absorve `extract_payload`.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::Element;
use crate::entities::func::Func;
use crate::entities::source_result::SourceResult;

/// `counter(key).display(callback)` — valor pré-renderizado pós-fixpoint.
#[derive(Debug, Clone, PartialEq)]
pub struct CounterDisplayCallbackElem {
    pub key: String,
    pub callback: Option<Func>,
}

// `Hash` manual via `Debug` (paridade `content_hash`): `Func` não implementa
// `Hash` (tem `Arc<FuncRepr>`). `PartialEq` deriva (Func via `Arc::ptr_eq`).
impl std::hash::Hash for CounterDisplayCallbackElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for CounterDisplayCallbackElem {
    fn plain_text(&self) -> String {
        String::new()
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::CounterDisplayCallback(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::CounterDisplayCallback(Arc::new(self.clone()))
    }

    fn element_kind(&self) -> Option<ElementKind> {
        Some(ElementKind::CounterDisplay)
    }

    fn to_payload(&self) -> Option<ElementPayload> {
        Some(ElementPayload::CounterDisplay {
            key: self.key.clone(),
            callback: self.callback.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> CounterDisplayCallbackElem {
        CounterDisplayCallbackElem { key: "heading".into(), callback: None }
    }

    #[test]
    fn locatavel_kind_counter_display() {
        assert_eq!(ex().element_kind(), Some(ElementKind::CounterDisplay));
        assert!(matches!(ex().to_payload(), Some(ElementPayload::CounterDisplay { .. })));
    }

    #[test]
    fn plain_text_vazio() {
        assert_eq!(ex().plain_text(), "");
    }

    #[test]
    fn igualdade_por_key() {
        assert_eq!(ex(), ex());
        assert_ne!(
            ex(),
            CounterDisplayCallbackElem { key: "figure".into(), callback: None }
        );
    }
}
