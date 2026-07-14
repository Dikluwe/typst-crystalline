//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash e6442e3f
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P377): o layout de `Shape` movido do monólito
//! `layout_content` para o arquivo da feature (forma B — free function na
//! camada de render). Content-preserving — a lógica é idêntica.

use crate::entities::elements::shape::ShapeElem;
use crate::entities::geometry::ShapeKind;
use crate::entities::layout_types::{FrameItem, Point, Pt};

use super::helpers::resolve_pt;
use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de uma forma (`rect`/`circle`/`line`/…): resolve dimensões, emite
/// um `FrameItem::Shape` na página actual e avança o cursor vertical.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &ShapeElem,
) {
    let (kind, width, height, fill, stroke) = (&e.kind, &e.width, &e.height, &e.fill, &e.stroke);
    let available_w = layouter.available_width();
    let (resolved_w, resolved_h) = match kind {
        // P242 — RoundedRect partilha dimensões com Rect.
        ShapeKind::Rect | ShapeKind::RoundedRect { .. } | ShapeKind::Ellipse | ShapeKind::Path(_) => {
            let w = resolve_pt(width.as_deref(), available_w);
            let h = resolve_pt(height.as_deref(), 0.0);
            (w, h)
        }
        ShapeKind::Line { dx, dy } => (dx.abs(), dy.abs()),
    };

    if layouter.regions.current.cursor_y.0 + resolved_h > layouter.regions.current.height - layouter.page_config.margin {
        layouter.new_page();
    }
    layouter.flush_line();

    // P748 — no fluxo principal o cursor_y representa a baseline do texto;
    // o topo de uma forma deve alinhar-se com o topo da linha
    // (baseline − ascender), não com a baseline. Em sub-frames o cursor_y
    // já é relativo ao topo local (inicializado a ascender e depois
    // transladado por −ascender no grid), logo não subtraímos de novo.
    let shape_top = if layouter.is_sub_frame {
        layouter.regions.current.cursor_y
    } else {
        let (ascender, _) = layouter.metrics.vertical_metrics(layouter.style.size);
        layouter.regions.current.cursor_y - ascender
    };
    let pos = Point { x: layouter.regions.current.cursor_x, y: shape_top };
    layouter.regions.current.current_items.push(FrameItem::Shape {
        pos,
        kind:   kind.clone(),
        width:  resolved_w,
        height: resolved_h,
        fill:   fill.as_ref().map(|p| p.to_color()),
        stroke: stroke.clone(),
        // P273.6 — populated by Layouter.parent_bbox (Block save/restore).
        parent_bbox_at_emit: layouter.parent_bbox,
    });

    layouter.regions.current.cursor_y += Pt(resolved_h);
}
