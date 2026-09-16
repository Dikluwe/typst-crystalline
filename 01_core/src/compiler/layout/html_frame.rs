//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/html_frame.md
//! @prompt-hash 44b880d7
//! @layer L1

use crate::entities::html::HtmlFrameElem;
use crate::entities::image_sizer::ImageSizer;
use crate::entities::introspector::{Introspector, TagIntrospector};
use crate::entities::layout_types::{FrameItem, Page, TextStyle};
use crate::entities::page_canvas::PageFill;
use crate::entities::style_chain::StyleChain;
use comemo::Track;

use super::{FontMetrics, Layouter};

pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    elem: &HtmlFrameElem,
) {
    layouter.layout_content(&elem.body);
}

pub(super) fn layout_intrinsic<M: FontMetrics, S: ImageSizer>(
    elem: &HtmlFrameElem,
    chain: &StyleChain,
    metrics: M,
    sizer: S,
) -> Page {
    let intr = TagIntrospector::empty();
    let intr_dyn: &dyn Introspector = &intr;
    let mut layouter = Layouter::new(metrics, sizer, chain.size(), intr_dyn.track());
    layouter.chain = chain.clone();
    layouter.style = TextStyle::from(chain);
    let (height, mut items, _, _, _) = layouter.layout_sub_frame(
        &elem.body,
        super::sub_frame::SubLayoutRegion {
            origin_x: 0.0,
            width: f64::INFINITY,
            height: None,
            align_rtl: false,
            unconstrained_height: true,
        },
    );
    let top = items
        .iter()
        .map(|item| match item {
            FrameItem::Text { pos, style, .. }
            | FrameItem::TextShaped { pos, style, .. } => {
                pos.y.0 - layouter.metrics.text_edges(style.size, style).0 .0
            }
            FrameItem::Glyph { pos, glyph_id, size, style, .. } => {
                pos.y.0 - layouter.metrics.glyph_ink_bounds(*glyph_id, *size, style).0 .0
            }
            FrameItem::Line { start, end, thickness, .. } => {
                start.y.0.min(end.y.0) - thickness / 2.0
            }
            FrameItem::Image { pos, .. }
            | FrameItem::Shape { pos, .. }
            | FrameItem::Group { pos, .. }
            | FrameItem::Link { pos, .. } => pos.y.0,
            FrameItem::Semantic { items, .. } => items
                .first()
                .map(|item| super::helpers::item_pos(item).1)
                .unwrap_or(0.0),
        })
        .fold(f64::INFINITY, f64::min);
    if top.is_finite() && top != 0.0 {
        for item in &mut items {
            super::helpers::offset_frame_item(item, 0.0, -top);
        }
    }
    let refs: Vec<_> = items.iter().collect();
    let width = layouter.metrics.line_content_right(&refs);
    Page {
        width,
        height,
        numbering: None,
        supplement: crate::entities::content::Content::Empty,
        bleed: Default::default(),
        fill: PageFill::None,
        background: vec![],
        foreground: vec![],
        items,
    }
}
