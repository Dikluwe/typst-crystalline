//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/grid_footer.md
//! @prompt-hash 01ea882e
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P380): o layout de `GridFooterElem` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::grid_footer::GridFooterElem;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `GridFooter` isolado: renderiza o body.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &GridFooterElem,
) {
    layouter.layout_planned_child(
        crate::compiler::introspect::page_running::PlanEdge::GridFooterBody,
        &e.body,
    );
}
