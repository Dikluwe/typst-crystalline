//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/counter_update.md
//! @prompt-hash 385efdf3
//! @layer L1
//! @updated 2026-06-11
//!
//! `CounterUpdateElem` — Lote 6 P321 (família state/counter). P198C. A mais
//! larga do lote (39). **Locatável** — absorve `extract_payload`.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::counter::CounterKey;
use crate::entities::counter_update::CounterUpdate as CounterAction;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// `counter(key).step()`/`.update(n)`. Aplicado à `CounterRegistry` em
/// `from_tags`. `action` (`counter_update::CounterUpdate`) deriva `Hash`.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct CounterUpdateElem {
    pub key: CounterKey,
    pub action: CounterAction,
}

impl Element for CounterUpdateElem {
    fn plain_text(&self) -> String {
        String::new()
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::CounterUpdate(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::CounterUpdate(Arc::new(self.clone()))
    }

    fn element_kind(&self) -> Option<ElementKind> {
        Some(ElementKind::CounterUpdate)
    }

    fn to_payload(&self) -> Option<ElementPayload> {
        Some(ElementPayload::CounterUpdate {
            key: self.key.clone(),
            action: self.action.clone(),
        })
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> CounterUpdateElem {
        CounterUpdateElem {
            key: CounterKey::Str("heading".into()),
            action: CounterAction::Step,
        }
    }

    #[test]
    fn locatavel_counter_update() {
        assert_eq!(ex().element_kind(), Some(ElementKind::CounterUpdate));
        assert!(matches!(ex().to_payload(), Some(ElementPayload::CounterUpdate { .. })));
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(ex(), ex());
        assert_ne!(
            ex(),
            CounterUpdateElem {
                key: CounterKey::Str("figure".into()),
                action: CounterAction::Step,
            }
        );
    }
}
