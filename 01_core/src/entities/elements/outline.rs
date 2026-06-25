//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/outline.md
//! @prompt-hash 9ce22615
//! @layer L1
//! @updated 2026-06-11
//!
//! `OutlineElem` — Lote 8 P323 (por largura). `outline()` — índice do documento.
//! P457: expande para `{ title, depth, indent }` (campos settable do vanilla).
//! **Locatável** (queryable desde P178) — absorve `extract_payload`.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Marcador de índice (table of contents).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct OutlineElem {
    pub title:  Option<Content>,
    pub depth:  usize,
    pub indent: bool,
}

impl OutlineElem {
    /// Defaults do vanilla: sem título customizado, profundidade 3, indentação activa.
    pub fn new(title: Option<Content>, depth: usize, indent: bool) -> Self {
        Self { title, depth, indent }
    }
}

impl Element for OutlineElem {
    fn plain_text(&self) -> String {
        String::new()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Outline(Arc::new(OutlineElem {
            title: self
                .title
                .as_ref()
                .map(|c| c.map_content(transform))
                .transpose()?,
            depth: self.depth,
            indent: self.indent,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Outline(Arc::new(OutlineElem {
            title: self.title.as_ref().map(|c| c.map_text(transform)),
            depth: self.depth,
            indent: self.indent,
        }))
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
        assert_eq!(OutlineElem::new(None, 3, true).plain_text(), "");
    }

    #[test]
    fn is_empty_default_false() {
        assert!(!OutlineElem::new(None, 3, true).is_empty());
    }

    #[test]
    fn igualdade_singleton() {
        assert_eq!(
            OutlineElem::new(Some(Content::text("A")), 2, true),
            OutlineElem::new(Some(Content::text("A")), 2, true)
        );
        assert_ne!(
            OutlineElem::new(None, 3, true),
            OutlineElem::new(None, 2, true)
        );
    }

    #[test]
    fn map_content_terminal_preserva_campos() {
        let mut f = |_c: &Content| -> SourceResult<Option<Content>> { Ok(None) };
        let elem = OutlineElem::new(Some(Content::text("T")), 3, true);
        match elem.map_content(&mut f).unwrap() {
            Content::Outline(e) => {
                assert_eq!(e.title.as_ref().map(|c| c.plain_text()), Some("T".to_string()));
                assert_eq!(e.depth, 3);
                assert!(e.indent);
            }
            _ => panic!("esperado Outline"),
        }
    }

    #[test]
    fn locatavel_kind_e_payload() {
        assert_eq!(OutlineElem::new(None, 3, true).element_kind(), Some(ElementKind::Outline));
        assert_eq!(OutlineElem::new(None, 3, true).to_payload(), Some(ElementPayload::Outline));
    }
}
