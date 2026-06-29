//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/image.md
//! @prompt-hash dd2977d5
//! @layer L1
//! @updated 2026-06-11
//!
//! `ImageElem` — Lote 7 P322. Imagem embebida (`image(path)`). Folha.

use std::sync::Arc;

use ecow::EcoString;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::ptr_eq_arc::PtrEqArc;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

/// Imagem. `data` partilhado via `PtrEqArc` (clone O(1), eq por ponteiro).
#[derive(Debug, Clone, PartialEq)]
pub struct ImageElem {
    pub path:   String,
    pub data:   PtrEqArc<Vec<u8>>,
    pub width:  Option<Box<Value>>,
    pub height: Option<Box<Value>>,
    pub fit:    EcoString,
}

// `Hash` manual via `Debug` (paridade `content_hash`): `Value` carrega `f64` e
// `PtrEqArc` não implementam `Hash`. Ressalva aceitável (`…Elem` não são chaves).
impl std::hash::Hash for ImageElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for ImageElem {
    fn plain_text(&self) -> String {
        String::new()
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Image(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Image(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    fn ex() -> ImageElem {
        ImageElem {
            path:   "a.png".into(),
            data:   PtrEqArc(Arc::new(vec![1, 2, 3])),
            width:  None,
            height: None,
            fit:    "cover".into(),
        }
    }

    fn h(e: &ImageElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn plain_text_vazio() {
        assert_eq!(ex().plain_text(), "");
    }

    #[test]
    fn hash_path_diferente_difere() {
        let a = ex();
        let mut b = a.clone();
        b.path = "b.png".into();
        assert_ne!(h(&a), h(&b));
        assert_eq!(h(&a), h(&a.clone()));
    }
}
