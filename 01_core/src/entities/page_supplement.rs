//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/page_supplement.md
//! @prompt-hash 2ae3871f
//! @layer L1
//! @updated 2026-08-24

use crate::entities::{content::Content, lang::Lang};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum PageSupplement {
    Auto,
    None,
    Content(Arc<Content>),
}

impl Default for PageSupplement {
    fn default() -> Self {
        Self::Auto
    }
}

pub fn local_name(lang: Option<Lang>) -> &'static str {
    match lang.as_ref().map(Lang::as_str) {
        Some("pt") | Some("por") | Some("es") | Some("spa") => "página",
        Some("de") | Some("deu") | Some("ger") => "Seite",
        Some("fr") | Some("fra") | Some("fre") => "page",
        _ => "page",
    }
}

impl PageSupplement {
    pub fn resolve(&self, lang: Option<Lang>) -> Content {
        match self {
            Self::Auto => Content::text(local_name(lang)),
            Self::None => Content::Empty,
            Self::Content(content) => (**content).clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    #[test]
    fn p1140_25_auto_localizado_e_none_vazio() {
        assert_eq!(
            PageSupplement::Auto
                .resolve(Some(Lang::from_str("pt").unwrap()))
                .plain_text(),
            "página"
        );
        assert_eq!(
            PageSupplement::Auto
                .resolve(Some(Lang::from_str("de").unwrap()))
                .plain_text(),
            "Seite"
        );
        assert!(PageSupplement::None.resolve(None).is_empty());
    }
}
