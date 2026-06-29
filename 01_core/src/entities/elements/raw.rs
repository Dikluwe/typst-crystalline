//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/raw.md
//! @prompt-hash f15b63db
//! @layer L1
//! @updated 2026-06-11
//!
//! `RawElem` — Lote 7 P322. Código raw inline/bloco (`` `...` ``). Folha.

use std::sync::Arc;

use ecow::EcoString;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Código raw. `lang` opcional (syntax highlighting); `block` = bloco vs inline.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct RawElem {
    pub text:  EcoString,
    pub lang:  Option<EcoString>,
    pub block: bool,
}

impl Element for RawElem {
    fn plain_text(&self) -> String {
        self.text.to_string()
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Raw(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Raw(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> RawElem {
        RawElem { text: "let x = 1".into(), lang: Some("rust".into()), block: true }
    }

    #[test]
    fn plain_text_e_o_texto() {
        assert_eq!(ex().plain_text(), "let x = 1");
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(ex(), ex());
        let mut other = ex();
        other.block = false;
        assert_ne!(ex(), other);
    }
}
