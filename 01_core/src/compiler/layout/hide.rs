//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/hide.md
//! @prompt-hash e3132e39
//! @layer L1
//! @updated 2026-09-29
//!
//! Atomização (ADR-0109, P381): o layout de `Hide` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::hide::HideElem;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `hide(...)`: executa o body para avançar o cursor, mas descarta
/// os items gerados (mantém só o avanço — per ADR-0054 graded).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &HideElem,
) {
    let saved_items = std::mem::take(&mut layouter.regions.current.current_items);
    let saved_dividers = std::mem::take(&mut layouter.pending_dividers);
    let saved_optical_overhang = layouter.line_optical_overhang.take();
    let saved_line = std::mem::take(&mut layouter.regions.current.current_line);
    layouter.layout_planned_child(
        crate::compiler::introspect::page_running::PlanEdge::HideBody,
        &e.body,
    );
    if !layouter.is_sub_frame && !layouter.current_page_is_empty() {
        layouter.page_has_unpainted_material = true;
    }
    layouter.regions.current.current_items = saved_items;
    layouter.pending_dividers = saved_dividers;
    layouter.regions.current.current_line = saved_line;
    layouter.line_optical_overhang = saved_optical_overhang;
    layouter.text_tracking_state.interrupt();
}
