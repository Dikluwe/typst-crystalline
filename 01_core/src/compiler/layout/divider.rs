//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/atomizacao_elementos.md
//! @prompt-hash 018a34a7
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P381): o layout de `Divider` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::divider::DividerElem;
use crate::entities::geometry::{ShapeKind, Stroke};
use crate::entities::layout_types::{Color, FrameItem, Point, Pt};
use crate::entities::paint::Paint;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `divider` (`---`): emite uma linha horizontal de margem a margem
/// e avança o cursor vertical. `_e` ignorado (singleton estrutural).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    _e: &DividerElem,
) {
    if layouter.regions.current.cursor_x.0 > layouter.page_config.margin {
        layouter.flush_line();
    }
    let margin = layouter.page_config.margin;
    // BLOCK_SPACING, ver vanilla_defaults.rs
    let spacing = Pt(layouter.style.size.val() * super::vanilla_defaults::BLOCK_SPACING);
    layouter.regions.current.cursor_y += spacing;
    // ref: lab/typst-original/crates/typst-library/src/visualize/line.rs:20
    let width_pt = layouter.regions.current.width - 2.0 * margin;
    layouter.regions.current.current_items.push(FrameItem::Shape {
        pos: Point {
            x: Pt(margin),
            y: layouter.regions.current.cursor_y,
        },
        kind: ShapeKind::Line { dx: width_pt, dy: 0.0 },
        width: width_pt,
        height: 0.5,
        fill: None,
        stroke: Some(Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 0.5,
            overhang: false,
        }),
        parent_bbox_at_emit: None,
    });
    layouter.regions.current.cursor_y += spacing;
}
