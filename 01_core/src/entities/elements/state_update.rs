//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/state_update.md
//! @prompt-hash 39584362
//! @layer L1
//! @updated 2026-06-11
//!
//! `StateUpdateElem` — Lote 6 P321 (família state/counter). P171/P173.
//! **Locatável** — absorve `extract_payload`.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;
use crate::entities::state_update::StateUpdate;

/// `state(key).update(value|func)`. Aplicado ao `StateRegistry` em `from_tags`.
#[derive(Debug, Clone, PartialEq)]
pub struct StateUpdateElem {
    pub key: String,
    pub update: StateUpdate,
}

// `Hash` manual via `Debug`: `state_update::StateUpdate` deriva só `Debug, Clone`
// (sem `Hash`; tem `impl PartialEq` manual, por isso o derive de `PartialEq`
// acima funciona). Paridade `content_hash`.
impl std::hash::Hash for StateUpdateElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for StateUpdateElem {
    fn plain_text(&self) -> String {
        String::new()
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::StateUpdate(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::StateUpdate(Arc::new(self.clone()))
    }

    fn element_kind(&self) -> Option<ElementKind> {
        Some(ElementKind::StateUpdate)
    }

    fn to_payload(&self) -> Option<ElementPayload> {
        Some(ElementPayload::StateUpdate {
            key: self.key.clone(),
            update: self.update.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::value::Value;

    fn ex() -> StateUpdateElem {
        StateUpdateElem {
            key: "ctr".into(),
            update: StateUpdate::Set(Box::new(Value::Int(1))),
        }
    }

    #[test]
    fn locatavel_state_update() {
        assert_eq!(ex().element_kind(), Some(ElementKind::StateUpdate));
        assert!(matches!(ex().to_payload(), Some(ElementPayload::StateUpdate { .. })));
    }

    #[test]
    fn plain_text_vazio() {
        assert_eq!(ex().plain_text(), "");
    }
}
