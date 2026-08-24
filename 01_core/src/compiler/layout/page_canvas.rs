//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/page_canvas.md
//! @prompt-hash 23884959
//! @layer L1
//!
//! Composição isolada dos layers decorativos da página (ADR-0109 forma B).

use crate::entities::content::Content;
use crate::entities::image_sizer::ImageSizer;
use crate::entities::layout_types::{FrameItem, Point, Pt, TransformMatrix};
use crate::entities::page_canvas::PageBleed;

use super::metrics::FontMetrics;
use super::sub_frame::SubLayoutRegion;
use super::Layouter;

pub(super) fn layout_layer<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    content: Option<&Content>,
    bleed: PageBleed,
    trim_width: f64,
    trim_height: f64,
) -> Vec<FrameItem> {
    let Some(content) = content else { return vec![] };
    let canvas_width = trim_width + bleed.horizontal();
    let canvas_height = trim_height + bleed.vertical();
    let (_, items, _, _, _) = layouter.layout_sub_frame(
        content,
        SubLayoutRegion {
            origin_x: 0.0,
            width: canvas_width,
            height: Some(canvas_height),
            align_rtl: false,
            unconstrained_height: false,
        },
    );
    vec![FrameItem::Group {
        pos: Point { x: Pt(-bleed.left), y: Pt(-bleed.top) },
        matrix: TransformMatrix::identity(),
        clip_mask: None,
        inner_width: canvas_width,
        inner_height: canvas_height,
        items,
    }]
}
