//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/repeat.md
//! @prompt-hash da7cb223
//! @layer L1
//! @updated 2026-06-11
//!
//! `RepeatElem` — Lote 7 P322. `repeat(body)` — repete o body para preencher.
//! Contentor: recurse em map_content E map_text.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::layout_types::Length;
use crate::entities::source_result::SourceResult;

/// Repete o `body` ao longo da largura disponível. `gap` entre cópias;
/// `justify` distribui o espaço.
#[derive(Debug, Clone, PartialEq)]
pub struct RepeatElem {
    pub body:    Content,
    pub gap:     Option<Length>,
    pub justify: bool,
}

// `Hash` manual via `Debug` (paridade `content_hash`): `Length` carrega `f64`.
impl std::hash::Hash for RepeatElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for RepeatElem {
    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    fn is_empty(&self) -> bool {
        self.body.is_empty()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Repeat(Arc::new(RepeatElem {
            body:    self.body.map_content(transform)?,
            gap:     self.gap,
            justify: self.justify,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Repeat(Arc::new(RepeatElem {
            body:    self.body.map_text(transform),
            gap:     self.gap,
            justify: self.justify,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> RepeatElem {
        RepeatElem { body: Content::text("ab"), gap: Some(Length::pt(2.0)), justify: true }
    }

    #[test]
    fn plain_text_do_body() {
        assert_eq!(ex().plain_text(), "ab");
    }

    #[test]
    fn is_empty_delega_ao_body() {
        assert!(!ex().is_empty());
        assert!(RepeatElem { body: Content::Empty, gap: None, justify: false }.is_empty());
    }

    #[test]
    fn map_text_recurse_preserva_gap_justify() {
        match ex().map_text(&mut |s| s.to_uppercase()) {
            Content::Repeat(e) => {
                assert_eq!(e.plain_text(), "AB");
                assert_eq!(e.gap, Some(Length::pt(2.0)));
                assert!(e.justify);
            }
            _ => panic!("esperado Repeat"),
        }
    }
}
