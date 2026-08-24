//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout.md
//! @prompt-hash 9096d4eb
//! @layer L1
use crate::entities::layout_types::{
    PageConfig, PageDimension, PageMarginSpec, PageMargins,
};
use crate::entities::page_geometry::{PageBinding, Paper};
pub(super) fn apply(
    c: &mut PageConfig,
    dir: crate::entities::dir::Dir,
    paper: Option<Paper>,
    flipped: Option<bool>,
    binding: Option<PageBinding>,
    width: Option<PageDimension>,
    height: Option<PageDimension>,
    margin: Option<PageMarginSpec>,
) {
    let dim = |d| match d {
        PageDimension::Auto => f64::INFINITY,
        PageDimension::Length(v) => v,
    };
    c.base_width = width
        .map(dim)
        .or_else(|| paper.map(Paper::width_pt))
        .unwrap_or(c.base_width);
    c.base_height = height
        .map(dim)
        .or_else(|| paper.map(Paper::height_pt))
        .unwrap_or(c.base_height);
    let f = flipped.unwrap_or(c.flipped);
    let (w, h) =
        if f { (c.base_height, c.base_width) } else { (c.base_width, c.base_height) };
    c.width = w;
    c.height = h;
    c.flipped = f;
    c.page_dir = dir;
    if let Some(b) = binding {
        c.binding = b;
    }
    if let Some(m) = margin {
        c.margin_spec = m.fold(c.margin_spec);
    }
    c.margin = PageMargins::resolve_for_page(
        c.margin_spec,
        c.auto_margin(),
        c.binding.resolve(dir),
        std::num::NonZeroUsize::MIN,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1140_20_1_paper_override_e_flip() {
        let mut c = PageConfig::default();
        apply(
            &mut c,
            crate::entities::dir::Dir::LTR,
            Paper::from_name("us-letter"),
            Some(true),
            None,
            Some(PageDimension::Length(300.0)),
            None,
            None,
        );
        assert_eq!(c.height, 300.0);
        assert_eq!(c.width, Paper::from_name("us-letter").unwrap().height_pt());
    }
    #[test]
    fn p1140_20_1_margem_logica_por_paridade() {
        use std::num::NonZeroUsize;
        let s = PageMarginSpec {
            left: Some(10.0),
            right: Some(20.0),
            top: None,
            bottom: None,
            two_sided: Some(true),
        };
        let odd = PageMargins::resolve_for_page(
            s,
            5.0,
            PageBinding::Left,
            NonZeroUsize::new(1).unwrap(),
        );
        let even = PageMargins::resolve_for_page(
            s,
            5.0,
            PageBinding::Left,
            NonZeroUsize::new(2).unwrap(),
        );
        assert_eq!((odd.left, odd.right), (10.0, 20.0));
        assert_eq!((even.left, even.right), (20.0, 10.0));
    }
}
