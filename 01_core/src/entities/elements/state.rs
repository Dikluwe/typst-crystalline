//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/state.md
//! @prompt-hash d501b3ec
//! @layer L1
//! @updated 2026-06-11
//!
//! `StateElem` — Lote 6 P321 (família state/counter). `state(key, init)`. P171.
//! **Locatável** — absorve `extract_payload`.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

/// Declaração de estado documental. `init` é o valor inicial.
#[derive(Debug, Clone, PartialEq)]
pub struct StateElem {
    pub key: String,
    pub init: Box<Value>,
}

// `Hash` manual via `Debug` (`Value` carrega `f64`; paridade `content_hash`).
impl std::hash::Hash for StateElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for StateElem {
    fn plain_text(&self) -> String {
        String::new()
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::State(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::State(Arc::new(self.clone()))
    }

    fn element_kind(&self) -> Option<ElementKind> {
        Some(ElementKind::State)
    }

    fn to_payload(&self) -> Option<ElementPayload> {
        Some(ElementPayload::State { key: self.key.clone(), init: self.init.clone() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> StateElem {
        StateElem { key: "ctr".into(), init: Box::new(Value::Int(0)) }
    }

    #[test]
    fn locatavel_state() {
        assert_eq!(ex().element_kind(), Some(ElementKind::State));
        assert!(matches!(ex().to_payload(), Some(ElementPayload::State { .. })));
    }

    #[test]
    fn plain_text_vazio() {
        assert_eq!(ex().plain_text(), "");
    }
}
