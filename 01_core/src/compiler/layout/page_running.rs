//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/page_running.md
//! @prompt-hash f412b469
//! @layer L1
//! @updated 2026-08-24

use super::metrics::FontMetrics;
use super::sub_frame::SubLayoutRegion;
use super::Layouter;
use crate::entities::content::Content;
use crate::entities::image_sizer::ImageSizer;
use crate::entities::layout_types::{FrameItem, HAlign, Point, Pt, TransformMatrix};
use crate::entities::page_running::{PageMarginal, PageNumberAlign, PageNumberVAlign};

pub(super) fn numbering_enabled(marginal: &PageMarginal) -> bool {
    matches!(marginal, PageMarginal::Auto)
}

pub(super) fn number_position(
    align: PageNumberAlign,
    page_width: f64,
    page_height: f64,
    margins: crate::entities::layout_types::PageMargins,
    text_width: f64,
    header_ascent: f64,
    footer_descent: f64,
) -> Point {
    let x = match align.horizontal {
        HAlign::Left | HAlign::Start => margins.left,
        HAlign::Center => (page_width - text_width) / 2.0,
        HAlign::Right | HAlign::End => page_width - margins.right - text_width,
    };
    let y = match align.vertical {
        PageNumberVAlign::Top => margins.top - header_ascent,
        PageNumberVAlign::Bottom => page_height - margins.bottom + footer_descent,
    };
    Point { x: Pt(x), y: Pt(y) }
}

pub(super) fn resolve_offset(
    offset: crate::entities::page_running::PageMarginalOffset,
    margin: f64,
    font_size: f64,
) -> f64 {
    margin * offset.rel + offset.abs.resolve_pt(font_size)
}

fn layout_content<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    content: &Content,
    width: f64,
    height: f64,
    y: f64,
) -> Vec<FrameItem> {
    let (_, items, _, _, _) = layouter.layout_sub_frame(
        content,
        SubLayoutRegion {
            origin_x: 0.0,
            width,
            height: Some(height.max(0.0)),
            align_rtl: false,
            unconstrained_height: false,
        },
    );
    vec![FrameItem::Group {
        pos: Point { x: Pt(layouter.page_config.margin.left), y: Pt(y) },
        matrix: TransformMatrix::identity(),
        clip_mask: None,
        inner_width: width,
        inner_height: height.max(0.0),
        items,
    }]
}

pub(super) fn explicit_layers<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    page_width: f64,
    page_height: f64,
) -> Vec<FrameItem> {
    let config = layouter.page_config.clone();
    let width = (page_width - config.margin.horizontal()).max(0.0);
    let font_size = layouter.style.size.0;
    let ha = resolve_offset(config.header_ascent, config.margin.top, font_size);
    let fd = resolve_offset(config.footer_descent, config.margin.bottom, font_size);
    let mut out = vec![];
    if let PageMarginal::Content(content) = &config.header {
        out.extend(layout_content(layouter, content, width, config.margin.top - ha, 0.0));
    }
    if let PageMarginal::Content(content) = &config.footer {
        out.extend(layout_content(
            layouter,
            content,
            width,
            config.margin.bottom - fd,
            page_height - config.margin.bottom + fd,
        ));
    }
    out
}

pub(super) fn realized_numbering_layer<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    content: &Content,
    page_width: f64,
    page_height: f64,
) -> Vec<FrameItem> {
    if content.is_empty() {
        return vec![];
    }
    let style = crate::entities::layout_types::TextStyle::from(&layouter.chain);
    let text_width =
        layouter.metrics.advance(&content.plain_text(), style.size, &style).0;
    let ha = resolve_offset(
        layouter.page_config.header_ascent,
        layouter.page_config.margin.top,
        layouter.style.size.0,
    );
    let fd = resolve_offset(
        layouter.page_config.footer_descent,
        layouter.page_config.margin.bottom,
        layouter.style.size.0,
    );
    let pos = number_position(
        layouter.page_config.number_align,
        page_width,
        page_height,
        layouter.page_config.margin,
        text_width,
        ha,
        fd,
    );
    let (_, items, _, _, _) = layouter.layout_sub_frame(
        content,
        SubLayoutRegion {
            origin_x: 0.0,
            width: text_width.max(1.0),
            height: None,
            align_rtl: false,
            unconstrained_height: true,
        },
    );
    vec![FrameItem::Group {
        pos,
        matrix: TransformMatrix::identity(),
        clip_mask: None,
        inner_width: text_width.max(1.0),
        inner_height: layouter.style.size.0,
        items,
    }]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1140_24_posicao_respeita_alinhamento_e_offsets() {
        let margins = crate::entities::layout_types::PageMargins::uniform(20.0);
        let top = number_position(
            PageNumberAlign {
                horizontal: HAlign::Right,
                vertical: PageNumberVAlign::Top,
            },
            200.0,
            100.0,
            margins,
            10.0,
            6.0,
            6.0,
        );
        assert_eq!(top.x.0, 170.0);
        assert_eq!(top.y.0, 14.0);
    }
}
