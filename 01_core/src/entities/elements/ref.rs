//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/ref.md
//! @prompt-hash 20e8e1f0
//! @layer L1
//! @updated 2026-06-11
//!
//! `RefElem` — Lote 8 P323 (por largura). `@label` — referência a uma label.
//! Leaf não-locatável; `map_*` terminais.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::label::Label;
use crate::entities::source_result::SourceResult;

/// Referência a uma `Label` (`@target`). Resolvida na introspecção.
// Construtor `Content::reference` (não `r#ref`: a keyword `ref` exigiria raw
// identifier em cada call-site — fricção desnecessária).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefElem {
    pub target: Label,
}

impl Element for RefElem {
    fn plain_text(&self) -> String {
        format!("@{}", self.target.0)
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Ref(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Ref(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    fn ex() -> RefElem {
        RefElem { target: Label("intro".to_string()) }
    }

    #[test]
    fn plain_text_com_arroba() {
        assert_eq!(ex().plain_text(), "@intro");
    }

    #[test]
    fn is_empty_default_false() {
        assert!(!ex().is_empty());
    }

    #[test]
    fn map_content_terminal() {
        let mut f = |_c: &Content| -> SourceResult<Option<Content>> { Ok(None) };
        assert!(matches!(ex().map_content(&mut f).unwrap(), Content::Ref(_)));
    }

    #[test]
    fn eq_compara_target() {
        assert_eq!(ex(), ex());
        assert_ne!(ex(), RefElem { target: Label("outro".to_string()) });
    }

    fn h(e: &RefElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        assert_ne!(h(&ex()), h(&RefElem { target: Label("outro".to_string()) }));
    }
}
