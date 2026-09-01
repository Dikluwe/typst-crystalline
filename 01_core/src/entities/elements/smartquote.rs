//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/smartquote.md
//! @prompt-hash 3aca74bf
//! @layer L1
//! @updated 2026-06-11
//!
//! `SmartQuoteElem` — Lote 9 P324 (por largura). `smartquote(double)` —
//! aspa lang-aware. Leaf não-locatável; `map_*` terminais.

use std::sync::Arc;

use ecow::EcoString;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SmartQuotePair {
    pub open: EcoString,
    pub close: EcoString,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SmartQuoteOverrides {
    pub single: Option<SmartQuotePair>,
    pub double: Option<SmartQuotePair>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SmartQuoteQuotes {
    Auto,
    Custom(SmartQuoteOverrides),
}

/// Aspa "smart" (programática). `double = true` → aspa dupla; `false` → simples.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SmartQuoteElem {
    pub double: bool,
    pub alternative: Option<bool>,
    pub quotes: Option<SmartQuoteQuotes>,
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
        assert_eq!(elem(true).plain_text(), "\"");
        assert_eq!(elem(false).plain_text(), "'");
    }

    #[test]
    fn is_empty_default_false() {
        assert!(!elem(true).is_empty());
    }

    #[test]
    fn map_content_terminal() {
        let mut f = |_c: &Content| -> SourceResult<Option<Content>> { Ok(None) };
        assert!(matches!(
            elem(true).map_content(&mut f).unwrap(),
            Content::SmartQuote(_)
        ));
    }

    #[test]
    fn eq_compara_double() {
        assert_eq!(elem(true), elem(true));
        assert_ne!(elem(true), elem(false));
    }

    fn elem(double: bool) -> SmartQuoteElem {
        SmartQuoteElem { double, alternative: None, quotes: None }
    }
}
