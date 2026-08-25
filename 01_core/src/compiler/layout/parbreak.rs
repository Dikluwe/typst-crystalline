//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout.md
//! @prompt-hash a4b2dd97
//! @layer L1
//! @updated 2026-08-24
//!
//! Materialização transparente da fronteira de parágrafo (P1140.13).

use crate::entities::layout_types::{
    FrameItem, Point, Pt, SemanticKind, SemanticPlacement,
};

use super::{FontMetrics, ImageSizer, Layouter};

pub(super) fn mark_boundary<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
) {
    let Some((x, y)) = layouter
        .regions
        .current
        .current_line
        .first()
        .map(super::helpers::item_pos)
    else {
        return;
    };

    layouter.regions.current.current_line.push(FrameItem::Semantic {
        kind: SemanticKind::ParbreakBoundary,
        placement: SemanticPlacement::Block,
        alt: None,
        items: vec![FrameItem::Text {
            pos: Point { x: Pt(x), y: Pt(y) },
            text: "".into(),
            style: layouter.style.clone(),
        }],
    });
}
