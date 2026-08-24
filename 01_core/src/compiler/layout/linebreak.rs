//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout.md
//! @layer L1
//! @updated 2026-08-24
//!
//! Consumer atomizado de `LinebreakElem` (ADR-0109, forma B).

use crate::entities::elements::linebreak::LinebreakElem;
use crate::entities::layout_types::{
    FrameItem, Point, Pt, SemanticKind, SemanticPlacement,
};

use super::{FontMetrics, ImageSizer, Layouter};

pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    elem: &LinebreakElem,
) {
    if let Some((x, y)) = layouter
        .regions
        .current
        .current_line
        .first()
        .map(super::helpers::item_pos)
    {
        // P1140.12 — conserva a causa do flush para o reflow bidi. O filho
        // vazio fornece baseline sem produzir texto ou desenho.
        layouter.regions.current.current_line.push(FrameItem::Semantic {
            kind: SemanticKind::ExplicitLinebreakBoundary,
            placement: SemanticPlacement::Inline,
            alt: None,
            items: vec![FrameItem::Text {
                pos: Point { x: Pt(x), y: Pt(y) },
                text: "".into(),
                style: layouter.style.clone(),
            }],
        });
    }
    if elem.justify {
        layouter.justify_current_line();
    }
    layouter.flush_line();
}
