//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/page_running.md
//! @prompt-hash c8eb2a49
//! @layer L1
//! @updated 2026-08-24

use crate::entities::content::Content;
use crate::entities::layout_types::HAlign;
use crate::entities::layout_types::Length;
use crate::entities::rel::Rel;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PageNumberVAlign {
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PageNumberAlign {
    pub horizontal: HAlign,
    pub vertical: PageNumberVAlign,
}

impl Default for PageNumberAlign {
    fn default() -> Self {
        Self {
            horizontal: HAlign::Center,
            vertical: PageNumberVAlign::Bottom,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PageMarginal {
    Auto,
    None,
    Content(Arc<Content>),
}

impl Default for PageMarginal {
    fn default() -> Self {
        Self::Auto
    }
}

pub type PageMarginalOffset = Rel<Length>;

pub fn default_marginal_offset() -> PageMarginalOffset {
    Rel { rel: 0.3, abs: Length::ZERO }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1140_24_defaults_sao_center_bottom_auto_trinta_porcento() {
        let a = PageNumberAlign::default();
        assert_eq!(a.horizontal, HAlign::Center);
        assert_eq!(a.vertical, PageNumberVAlign::Bottom);
        assert_eq!(default_marginal_offset().rel, 0.3);
        assert_eq!(PageMarginal::default(), PageMarginal::Auto);
    }
}
