//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/metadata.md
//! @prompt-hash 93627981
//! @layer L1
//! @updated 2026-06-11
//!
//! `MetadataElem` — Lote 6 P321 (família state/counter). `metadata(value)`.
//! **Locatável** (queryable, M9 P169) — absorve `extract_payload`.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

/// Marcador de metadata queryable. `value` arbitrário.
#[derive(Debug, Clone, PartialEq)]
pub struct MetadataElem {
    pub value: Box<Value>,
}

// `Hash` manual via `Debug` (paridade `content_hash`): `Value` carrega `f64`
// (`Value::Float`) e não implementa `Hash`. Ressalva `-0.0` vs `0.0` aceitável
// (regime do `content_hash`; `…Elem` não são chaves de mapa).
impl std::hash::Hash for MetadataElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for MetadataElem {
    fn plain_text(&self) -> String {
        String::new()
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Metadata(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Metadata(Arc::new(self.clone()))
    }

    fn element_kind(&self) -> Option<ElementKind> {
        Some(ElementKind::Metadata)
    }

    fn to_payload(&self) -> Option<ElementPayload> {
        Some(ElementPayload::Metadata { value: self.value.clone() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn ex() -> MetadataElem {
        MetadataElem { value: Box::new(Value::Int(42)) }
    }

    fn h(e: &MetadataElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn locatavel_metadata() {
        assert_eq!(ex().element_kind(), Some(ElementKind::Metadata));
        assert!(matches!(ex().to_payload(), Some(ElementPayload::Metadata { .. })));
    }

    #[test]
    fn plain_text_vazio() {
        assert_eq!(ex().plain_text(), "");
    }

    #[test]
    fn hash_value_diferente_difere() {
        let a = ex();
        let b = MetadataElem { value: Box::new(Value::Int(7)) };
        assert_ne!(h(&a), h(&b));
        assert_eq!(h(&a), h(&a.clone()));
    }
}
