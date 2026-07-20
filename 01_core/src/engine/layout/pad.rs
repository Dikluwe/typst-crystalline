//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/atomizacao_elementos.md
//! @prompt-hash a54abe5a
//! @layer L1
//! @updated 2026-06-18
//!
//! Atomização (ADR-0109, P376): o layout de `Pad` movido do monólito
//! `layout_content` para o arquivo da feature. Content-preserving — a lógica
//! é idêntica; só o endereço mudou. O `match` no núcleo delega numa linha.

use crate::entities::elements::pad::PadElem;
use crate::entities::layout_types::Pt;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout do container `Pad` (P156C/P156L/P243/P273.9).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &PadElem,
) {
    let body = &e.body;
    let sides = &e.sides;
    // P156L: cada side é Option<Length>; None ↔ default
    // vanilla zero (resolvido aqui em vez de em native_pad).
    // **P243**: `right` agora reduz `regions.current.width`
    // efectiva durante body layout via save/restore (paridade
    // mecânica vanilla).
    let font = layouter.style.size.val();
    let left   = sides.left  .map_or(0.0, |l| l.resolve_pt(font));
    let top    = sides.top   .map_or(0.0, |l| l.resolve_pt(font));
    let right  = sides.right .map_or(0.0, |l| l.resolve_pt(font));
    let bottom = sides.bottom.map_or(0.0, |l| l.resolve_pt(font));

    if layouter.regions.current.cursor_x.0 > layouter.regions.current.line_start_x.0 {
        layouter.flush_line();
    }
    layouter.regions.current.cursor_y += Pt(top);

    let saved_line_start = layouter.regions.current.line_start_x;
    let saved_width      = layouter.regions.current.width;
    layouter.regions.current.line_start_x = saved_line_start + Pt(left);
    layouter.regions.current.cursor_x     = layouter.regions.current.line_start_x;
    // P243 — reduz width útil por `right` (clamp a ≥ 0).
    layouter.regions.current.width = (saved_width - right).max(0.0);

    // P273.9 — save/restore parent_bbox INNER (body region;
    // paralela a Block semantic). Layout duplo via
    // measure_content_constrained pre-layout (sub-padrão
    // "Layout duplo arquitectural aceite" N=1 inaugural).
    // Defaults rigorosos: popular apenas se measured w/h > 0.
    let saved_parent_bbox_p273_9 = layouter.parent_bbox;
    let pad_avail_inner = layouter.available_width();
    let (pad_body_w, pad_body_h) =
        layouter.measure_content_constrained(body, pad_avail_inner);
    if pad_body_w > 0.0 && pad_body_h > 0.0 {
        layouter.parent_bbox = Some(crate::entities::layout_types::Rect {
            x: layouter.regions.current.cursor_x,
            y: layouter.regions.current.cursor_y,
            w: Pt(pad_body_w),
            h: Pt(pad_body_h),
        });
    }

    layouter.layout_content(body);
    layouter.flush_line();

    // P273.9 — restore parent_bbox (LIFO).
    layouter.parent_bbox = saved_parent_bbox_p273_9;

    layouter.regions.current.cursor_y += Pt(bottom);
    layouter.regions.current.line_start_x = saved_line_start;
    layouter.regions.current.cursor_x     = saved_line_start;
    // P243 — restaurar width original.
    layouter.regions.current.width = saved_width;
}
