//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/stdlib/context.md
//! @prompt-hash 6005f386
//! @layer L1
//! @updated 2026-06-30
//!
//! `ContextBlockElem` — bloco de delayed evaluation para `context { expr }`.
//! P506 — runtime state via context.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::Element;
use crate::entities::func::Func;
use crate::entities::source_result::SourceResult;

/// Bloco `context { expr }`. O corpo é uma closure que será avaliada na
/// fase de expansão pós-introspecção, com acesso ao `TagIntrospector` e à
/// `Location` onde o bloco aparece no documento.
pub struct ContextBlockElem {
    pub id: u64,
    pub closure: Func,
}

impl std::fmt::Debug for ContextBlockElem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContextBlockElem").field("id", &self.id).finish()
    }
}

impl Clone for ContextBlockElem {
    fn clone(&self) -> Self {
        Self { id: self.id, closure: self.closure.clone() }
    }
}

impl PartialEq for ContextBlockElem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::hash::Hash for ContextBlockElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Element for ContextBlockElem {
    fn plain_text(&self) -> String {
        String::new()
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::ContextBlock(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::ContextBlock(Arc::new(self.clone()))
    }

    fn element_kind(&self) -> Option<ElementKind> {
        Some(ElementKind::ContextBlock)
    }

    fn to_payload(&self) -> Option<ElementPayload> {
        Some(ElementPayload::ContextBlock { id: self.id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_block_eq_por_id() {
        // Func não é PartialEq; a igualdade de ContextBlockElem usa só id.
        let dummy_closure = Func::native("dummy", |_ctx, _args, _world, _file| {
            Ok(crate::entities::value::Value::None)
        });
        let a = ContextBlockElem { id: 7, closure: dummy_closure.clone() };
        let b = ContextBlockElem { id: 7, closure: dummy_closure };
        assert_eq!(a, b);
    }
}
