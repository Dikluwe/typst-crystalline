//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/labelled.md
//! @prompt-hash 978b04b7
//! @layer L1
//! @updated 2026-06-12
//!
//! `LabelledElem` — Lote 14 P329 (reclassificado da triagem DEBT-58).
//! Wrapper de label: `map_*` recursam no `target`. Não-locatável no trait
//! (o payload é emitido pelo walk arm em pós-recursão, P195B — inalterado).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::label::Label;
use crate::entities::source_result::SourceResult;

/// Associa uma `Label` a um `target` (wrapper transparente de identidade).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct LabelledElem {
    pub target: Content,
    pub label:  Label,
}

impl Element for LabelledElem {
    fn plain_text(&self) -> String {
        self.target.plain_text()
    }

    fn is_empty(&self) -> bool {
        self.target.is_empty()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Labelled(Arc::new(LabelledElem {
            target: self.target.map_content(transform)?,
            label:  self.label.clone(),
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Labelled(Arc::new(LabelledElem {
            target: self.target.map_text(transform),
            label:  self.label.clone(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    fn ex() -> LabelledElem {
        LabelledElem { target: Content::text("x"), label: Label("intro".to_string()) }
    }

    #[test]
    fn plain_text_delega_ao_target() {
        assert_eq!(ex().plain_text(), "x");
    }

    #[test]
    fn is_empty_delega_ao_target() {
        assert!(!ex().is_empty());
        assert!(LabelledElem { target: Content::Empty, label: Label("l".to_string()) }.is_empty());
    }

    #[test]
    fn map_content_recurse_target_preserva_label() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s, _) if s.as_str() == "x" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match ex().map_content(&mut f).unwrap() {
            Content::Labelled(e) => {
                assert_eq!(e.label.0, "intro");
                assert!(matches!(&e.target, Content::Text(s, _) if s.as_str() == "Z"));
            }
            _ => panic!("esperado Labelled"),
        }
    }

    fn h(e: &LabelledElem) -> u64 {
        let mut s = DefaultHasher::new(); e.hash(&mut s); s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        assert_ne!(h(&ex()), h(&LabelledElem { target: Content::text("x"), label: Label("outro".to_string()) }));
    }
}
