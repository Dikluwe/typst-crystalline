//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/smartquote.md
//! @prompt-hash 08af9b7e
//! @layer L1
//! @updated 2026-06-11
//!
//! `SmartQuoteElem` — Lote 9 P324 (por largura). `smartquote(double)` —
//! aspa lang-aware. Leaf não-locatável; `map_*` terminais.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Aspa "smart" (programática). `double = true` → aspa dupla; `false` → simples.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SmartQuoteElem {
    pub double: bool,
}

impl Element for SmartQuoteElem {
    fn plain_text(&self) -> String {
        if self.double {
            "\"".to_string()
        } else {
            "'".to_string()
        }
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::SmartQuote(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::SmartQuote(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_double_e_single() {
        assert_eq!(SmartQuoteElem { double: true }.plain_text(), "\"");
        assert_eq!(SmartQuoteElem { double: false }.plain_text(), "'");
    }

    #[test]
    fn is_empty_default_false() {
        assert!(!SmartQuoteElem { double: true }.is_empty());
    }

    #[test]
    fn map_content_terminal() {
        let mut f = |_c: &Content| -> SourceResult<Option<Content>> { Ok(None) };
        assert!(matches!(
            SmartQuoteElem { double: true }.map_content(&mut f).unwrap(),
            Content::SmartQuote(_)
        ));
    }

    #[test]
    fn eq_compara_double() {
        assert_eq!(SmartQuoteElem { double: true }, SmartQuoteElem { double: true });
        assert_ne!(SmartQuoteElem { double: true }, SmartQuoteElem { double: false });
    }
}
