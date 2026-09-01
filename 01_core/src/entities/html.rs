//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/html.md
//! @prompt-hash 1051b620
//! @layer L1

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

pub use crate::entities::compiler_features::{Feature, Features};
use crate::entities::content::Content;

pub type HtmlAttrs = IndexMap<EcoString, EcoString, FxBuildHasher>;

#[derive(Debug, Clone, PartialEq)]
pub enum HtmlBody {
    Unset,
    None,
    Content(Box<Content>),
}

impl HtmlBody {
    pub fn content(&self) -> Option<&Content> {
        match self {
            Self::Content(body) => Some(body),
            Self::Unset | Self::None => None,
        }
    }

    pub fn map_content(&self, transform: impl FnOnce(&Content) -> Content) -> Self {
        match self {
            Self::Unset => Self::Unset,
            Self::None => Self::None,
            Self::Content(body) => Self::Content(Box::new(transform(body))),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HtmlElem {
    pub tag: EcoString,
    pub attrs: Option<HtmlAttrs>,
    pub body: HtmlBody,
}

impl HtmlElem {
    pub fn new(tag: EcoString, attrs: Option<HtmlAttrs>, body: HtmlBody) -> Self {
        Self { tag, attrs, body }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn features_default_off_and_explicit_on() {
        assert!(!Features::default().contains(Feature::Html));
        let mut features = Features::empty();
        features.enable(Feature::Html);
        assert!(features.contains(Feature::Html));
    }
}
