//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/outline.md
//! @prompt-hash 5774554d
//! @layer L1
//! @updated 2026-06-11
//!
//! `OutlineElem` — Lote 8 P323 (por largura). `outline()` — índice do documento.
//! Unit struct (precedente `DividerElem`/`LinebreakElem`). **Locatável**
//! (queryable desde P178) — absorve `extract_payload`. Campos do vanilla
//! (`title`/`depth`/`indent`/…) pendentes de cobertura — aterram aqui sem
//! tocar o hub.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Marcador de índice (table of contents). Sem campos no estado atual.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OutlineElem;

impl Element for OutlineElem {
    fn plain_text(&self) -> String {
        String::new()
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Outline(Arc::new(OutlineElem)))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Outline(Arc::new(OutlineElem))
    }

    fn element_kind(&self) -> Option<ElementKind> {
        Some(ElementKind::Outline)
    }

    fn to_payload(&self) -> Option<ElementPayload> {
        Some(ElementPayload::Outline)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_vazio() {
        assert_eq!(OutlineElem.plain_text(), "");
    }

    #[test]
    fn is_empty_default_false() {
        assert!(!OutlineElem.is_empty());
    }

    #[test]
    fn igualdade_singleton() {
        assert_eq!(OutlineElem, OutlineElem);
    }

    #[test]
    fn map_content_terminal() {
        let mut f = |_c: &Content| -> SourceResult<Option<Content>> { Ok(None) };
        assert!(matches!(OutlineElem.map_content(&mut f).unwrap(), Content::Outline(_)));
    }

    #[test]
    fn locatavel_kind_e_payload() {
        assert_eq!(OutlineElem.element_kind(), Some(ElementKind::Outline));
        assert_eq!(OutlineElem.to_payload(), Some(ElementPayload::Outline));
    }
}
