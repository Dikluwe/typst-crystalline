//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/html.md
//! @prompt-hash 02c81c86
//! @layer L1

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::entities::content::Content;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Feature {
    Html,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Features {
    html: bool,
}

impl Features {
    pub const fn empty() -> Self {
        Self { html: false }
    }

    pub const fn html() -> Self {
        Self { html: true }
    }

    pub const fn contains(self, feature: Feature) -> bool {
        match feature {
            Feature::Html => self.html,
        }
    }

    pub fn enable(&mut self, feature: Feature) {
        match feature {
            Feature::Html => self.html = true,
        }
    }
}

pub type HtmlAttrs = IndexMap<EcoString, EcoString, FxBuildHasher>;

#[derive(Debug, Clone, PartialEq)]
pub struct HtmlElem {
    pub tag: EcoString,
    pub attrs: Option<HtmlAttrs>,
    pub body: Option<Box<Content>>,
}

impl HtmlElem {
    pub fn new(tag: EcoString, attrs: Option<HtmlAttrs>, body: Option<Content>) -> Self {
        Self { tag, attrs, body: body.map(Box::new) }
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
