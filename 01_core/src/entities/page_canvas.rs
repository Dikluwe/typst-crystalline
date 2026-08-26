//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/page_canvas.md
//! @prompt-hash ef6f2dcb
//! @layer L1
use crate::entities::{layout_types::Length, paint::Paint, rel::Rel};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PageBleedSpec {
    pub left: Option<Rel<Length>>,
    pub right: Option<Rel<Length>>,
    pub top: Option<Rel<Length>>,
    pub bottom: Option<Rel<Length>>,
    pub two_sided: Option<bool>,
}
impl PageBleedSpec {
    pub fn uniform(value: Rel<Length>) -> Self {
        Self {
            left: Some(value),
            right: Some(value),
            top: Some(value),
            bottom: Some(value),
            two_sided: Some(false),
        }
    }

    pub fn fold(self, outer: Self) -> Self {
        Self {
            left: self.left.or(outer.left),
            right: self.right.or(outer.right),
            top: self.top.or(outer.top),
            bottom: self.bottom.or(outer.bottom),
            two_sided: self.two_sided.or(outer.two_sided),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PageBleed {
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64,
}
impl PageBleed {
    pub fn is_zero(self) -> bool {
        self.left == 0.0 && self.right == 0.0 && self.top == 0.0 && self.bottom == 0.0
    }

    pub fn horizontal(self) -> f64 {
        self.left + self.right
    }
    pub fn vertical(self) -> f64 {
        self.top + self.bottom
    }

    pub fn resolve(
        spec: PageBleedSpec,
        width: f64,
        height: f64,
        binding: crate::entities::page_geometry::PageBinding,
        page: std::num::NonZeroUsize,
        font_size: f64,
    ) -> Self {
        let resolve = |value: Option<Rel<Length>>, axis: f64| {
            value
                .map(|value| value.rel * axis + value.abs.resolve_pt(font_size))
                .unwrap_or(0.0)
        };
        let mut out = Self {
            left: resolve(spec.left, width),
            right: resolve(spec.right, width),
            top: resolve(spec.top, height),
            bottom: resolve(spec.bottom, height),
        };
        if spec.two_sided == Some(true) && binding.swap(page) {
            std::mem::swap(&mut out.left, &mut out.right);
        }
        out
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
pub enum PageFill {
    #[default]
    Auto,
    None,
    Paint(Paint),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::page_geometry::PageBinding;
    use std::num::NonZeroUsize;

    #[test]
    fn bleed_uniforme_e_fold_preservam_delta_interno() {
        let one = Rel { rel: 0.0, abs: Length::pt(1.0) };
        let two = Rel { rel: 0.0, abs: Length::pt(2.0) };
        let inner = PageBleedSpec { left: Some(two), ..PageBleedSpec::default() };
        let folded = inner.fold(PageBleedSpec::uniform(one));
        assert_eq!(folded.left, Some(two));
        assert_eq!(folded.right, Some(one));
        assert_eq!(folded.two_sided, Some(false));
    }

    #[test]
    fn bleed_percentual_resolve_por_eixo_e_troca_na_pagina_par() {
        let spec = PageBleedSpec {
            left: Some(Rel::from_percent(10.0)),
            right: Some(Rel::from_percent(20.0)),
            top: Some(Rel::from_percent(5.0)),
            bottom: None,
            two_sided: Some(true),
        };
        let bleed = PageBleed::resolve(
            spec,
            200.0,
            100.0,
            PageBinding::Left,
            NonZeroUsize::new(2).unwrap(),
            11.0,
        );
        assert_eq!((bleed.left, bleed.right), (40.0, 20.0));
        assert_eq!((bleed.top, bleed.bottom), (5.0, 0.0));
    }

    #[test]
    fn fill_preserva_tres_estados() {
        assert_eq!(PageFill::default(), PageFill::Auto);
        assert_ne!(PageFill::Auto, PageFill::None);
    }
}
