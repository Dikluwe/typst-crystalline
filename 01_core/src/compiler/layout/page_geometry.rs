//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/page_geometry.md
//! @prompt-hash ee6ee1f6
//! @layer L1
use crate::entities::layout_types::{
    PageConfig, PageDimension, PageMarginSpec, PageMargins,
};
use crate::entities::page_geometry::{PageBinding, Paper};
#[expect(
    clippy::too_many_arguments,
    reason = "C011: Aplicador puro de geometria recebe apenas os seis overrides geométricos, direção e PageConfig mutável"
)]
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

impl<'a, M: super::metrics::FontMetrics, S: crate::entities::image_sizer::ImageSizer>
    super::Layouter<'a, M, S>
{
    /// **P867** — calcula a largura real da página actual quando `width: auto`,
    /// baseando-se na extensão horizontal dos items já emitidos.
    pub(super) fn compute_page_width(&self) -> f64 {
        let mut content_right = super::helpers::line_content_right(
            self.regions
                .current
                .current_items
                .iter()
                .chain(self.regions.current.current_line.iter()),
            &self.metrics,
        );
        // A largura da página auto inclui whitespace literal, inclusive
        // quando não há tinta. O alinhamento RTL continua a usar apenas a
        // extensão de material em helpers::line_content_right.
        let literal_right = self
            .regions
            .current
            .current_items
            .iter()
            .chain(self.regions.current.current_line.iter())
            .filter_map(|item| match item {
                crate::entities::layout_types::FrameItem::TextSeparator {
                    pos,
                    advance,
                    origin: crate::entities::layout_types::TextSeparatorOrigin::Literal,
                    ..
                } => Some(pos.x.0 + advance.0),
                _ => None,
            })
            .fold(0.0_f64, f64::max);
        content_right = content_right.max(literal_right);
        // **P1096** — equações de bloco pendentes de centragem carregam a
        // sua largura integral (incluindo trailing space_after_script).
        for (_, _, eq_width, applied_offset) in &self.pending_equation_centering {
            content_right = content_right.max(applied_offset + eq_width);
        }
        // **P987** — a linha de cada equação numerada pendente reserva
        // `eq_width + 2 × (number_width + gutter)`, `gutter = 0.5em`
        // (vanilla `add_equation_number`/`resize_equation`,
        // `typst-layout/src/math/mod.rs:209-330`). Sem esta reserva a página
        // encolhe ao conteúdo e o número fica sobreposto/fora da margem.
        let numbering_right = self
            .pending_equation_numbering
            .iter()
            .map(|(_, _, style, number_width, eq_width, applied_offset, _)| {
                let gutter = 0.5 * style.size.val();
                applied_offset + eq_width + 2.0 * (number_width + gutter)
            })
            .fold(f64::NEG_INFINITY, f64::max);
        (content_right.max(numbering_right) + self.page_config.margin.right)
            .max(self.page_config.margin.horizontal())
    }

    /// **P867** — calcula a altura real da página actual quando `height: auto`.
    /// O cursor_y já reflecte a posição vertical após o último flush; adiciona
    /// a margem inferior como aproximação do fundo do conteúdo.
    pub(super) fn compute_page_height(&self) -> f64 {
        let base_y = self.last_block_descent_y.unwrap_or(self.regions.current.cursor_y.0);
        let literal_top = if !super::html_frame::items_have_material(
            &self.regions.current.current_items,
        ) && !super::html_frame::items_have_material(
            &self.regions.current.current_line,
        ) {
            self.regions
                .current
                .current_items
                .iter()
                .chain(self.regions.current.current_line.iter())
                .filter_map(|item| match item {
                    crate::entities::layout_types::FrameItem::TextSeparator {
                        pos,
                        origin:
                            crate::entities::layout_types::TextSeparatorOrigin::Literal,
                        ..
                    } => Some(pos.y.0),
                    _ => None,
                })
                .fold(0.0_f64, f64::max)
        } else {
            0.0
        };
        (base_y.max(literal_top) + self.page_config.margin.bottom)
            .max(self.page_config.margin.vertical())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn auto_page_with_only_literal_spaces_keeps_logical_width_and_text_top() {
        use super::super::FontMetrics;
        use crate::entities::{
            content::Content,
            image_sizer::NullImageSizer,
            introspector::{Introspector, TagIntrospector},
            layout_types::{Length, Pt},
        };
        use comemo::Track;
        let intr = TagIntrospector::empty();
        for t in [0.2, -0.2, 0.0] {
            let mut l = super::super::Layouter::new(
                super::super::FixedMetrics,
                NullImageSizer,
                12.0,
                (&intr as &dyn Introspector).track(),
            );
            l.page_config.width = f64::INFINITY;
            l.page_config.height = f64::INFINITY;
            l.regions.current.width = f64::INFINITY;
            l.page_config.margin =
                crate::entities::layout_types::PageMargins::uniform(0.0);
            l.regions.current.cursor_x = Pt(0.0);
            l.regions.current.line_start_x = Pt(0.0);
            l.regions.current.cursor_y = Pt(0.0);
            l.style.tracking = Some(Length::pt(t));
            let style = super::super::text::effective_style(&l);
            let space = l.metrics.advance(" ", style.size, &style).0;
            let top = l.metrics.text_edges(style.size, &style).0 .0;
            l.layout_content(&Content::text("   "));
            let page = &l.finish().pages[0];
            assert!((page.width - (3.0 * space + 2.0 * t)).abs() < 1e-9);
            assert!((page.height - top).abs() < 1e-9);
        }
    }

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
