//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/atomizacao_elementos.md
//! @prompt-hash 6a13ed42
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P378): o layout de `Place` movido do monólito
//! `layout_content` para o arquivo da feature (forma B — free function na
//! camada de render). Content-preserving — a lógica é idêntica.

use crate::entities::elements::place::PlaceElem;
use crate::entities::layout_types::{Align2D, VAlign};

use super::{DeferredFloat, FontMetrics, ImageSizer, Layouter};

/// Layout de `place(...)`: `float: true` captura o body num sub-frame e empurra
/// para `floats_pending` (emitido no flush da página); `float: false` preserva
/// o caminho in-place (`layout_place`).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &PlaceElem,
) {
    let alignment = &e.alignment;
    let dx = &e.dx;
    let dy = &e.dy;
    let scope = &e.scope;
    let float = &e.float;
    let clearance = &e.clearance;
    let body = &e.body;
    // P232 — Resolver effective alignment per eixo via `.or()`.
    let effective_alignment = match layouter.cell_align {
        Some(grid_a) => Align2D {
            h: alignment.h.or(grid_a.h),
            v: alignment.v.or(grid_a.v),
        },
        None => *alignment,
    };

    if *float {
        // P245 — float real: layout body em sub-frame,
        // capturar items + dimensões, push ao buffer.
        let avail_w_page = layouter.available_width();
        let (body_height, body_items, deco_segments, orphaned_align_x, orphaned_align_y) =
            layouter.layout_sub_frame(
                body,
                super::sub_frame::SubLayoutRegion {
                    origin_x: 0.0,
                    width: avail_w_page,
                    height: None,
                    align_rtl: false,
                    unconstrained_height: true,
                },
            );
        // **P904 (Item 3)** — mesmo bug/fix de `placement.rs::layout_place`
        // (ramo `float: false`): `measure_content` devolvia `(0.0, 0.0)`
        // para `Content::Text`. Mede a partir de `body_items` já
        // layoutados via `FontMetrics::line_content_right` (mecanismo de
        // P772j, `origin_x` do sub-frame é sempre `0.0` aqui).
        let body_item_refs: Vec<&crate::entities::layout_types::FrameItem> =
            body_items.iter().collect();
        let content_w = layouter.metrics.line_content_right(&body_item_refs).max(0.0);
        let resolved_clearance = clearance
            .map(|l| l.resolve_pt(layouter.style.size.val()))
            .unwrap_or(0.0);

        // Reserva espaço top se alignment.y == Top.
        // Bottom + Horizon reservam ao fundo (paridade vanilla
        // default bottom).
        let is_top = matches!(effective_alignment.v, Some(VAlign::Top));
        if is_top {
            layouter.cursor_y_top_reserve += body_height + resolved_clearance;
        } else {
            layouter.cursor_y_bottom_reserve += body_height + resolved_clearance;
        }

        layouter.floats_pending.push(DeferredFloat {
            alignment: effective_alignment,
            body_items,
            body_height,
            body_width: content_w,
            clearance: resolved_clearance,
            deco_segments,
            // **P908** — já relativas a `body_items` (mesmo referencial
            // local, origem 0,0); `emit_deferred_float` rebaseia-as no
            // flush, quando a translação final é conhecida.
            orphaned_align_x,
            orphaned_align_y,
        });
        // Cursor.y NÃO avança — float não consome flow space.
        // dx/dy aplicado durante flush (não in-place).
        let _ = dx; // dx aplicado em flush via translate
        let _ = dy; // dy aplicado em flush via translate
        let _ = scope; // scope: Parent + float: true (DEBT-37 sentinela)
    } else {
        // P223 preserved literal: float: false → comportamento P84.5+P84.6.
        layouter.layout_place(effective_alignment, *dx, *dy, *scope, body);
    }
}
