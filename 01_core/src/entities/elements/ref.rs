//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/ref.md
//! @layer L1
//! @updated 2026-06-25
//!
//! `RefElem` — P462. Referência cruzada `@label` / `ref("label")`.
//! Leaf não-locatável; `map_*` terminais. O número é resolvido no layout
//! via `Introspector` (oráculo de counters); aqui guarda-se apenas o nome
//! do label e o supplement opcional.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;
use ecow::EcoString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RefForm {
    #[default]
    Normal,
    Page,
}

/// Referência a um label nomeado.
///
/// Construtores ergonómicos em `Content::reference` / `Content::reference_with_supplement`
/// (a keyword `ref` exigiria raw identifier em cada call-site — fricção desnecessária).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct RefElem {
    pub name: EcoString,
    pub supplement: Option<Content>,
    pub form: RefForm,
}

impl Element for RefElem {
    fn plain_text(&self) -> String {
        format!("@{}", self.name)
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
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn ex() -> RefElem {
        RefElem {
            name: "intro".into(),
            supplement: None,
            form: RefForm::Normal,
        }
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
    fn eq_compara_name() {
        assert_eq!(ex(), ex());
        assert_ne!(
            ex(),
            RefElem {
                name: "outro".into(),
                supplement: None,
                form: RefForm::Normal
            }
        );
    }

    #[test]
    fn supplement_afeta_eq() {
        let with_sup = RefElem {
            name: "intro".into(),
            supplement: Some(Content::text("Section ")),
            form: RefForm::Normal,
        };
        assert_ne!(ex(), with_sup);
    }

    fn h(e: &RefElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        assert_ne!(
            h(&ex()),
            h(&RefElem {
                name: "outro".into(),
                supplement: None,
                form: RefForm::Normal
            })
        );
    }
}
