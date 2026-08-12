//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/atomizacao_elementos.md
//! @prompt-hash bf8f0b19
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P380): o layout de `GridCellElem` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::grid_cell::GridCellElem;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `GridCell` isolado: renderiza o body (stroke/fill per-cell só dentro de Grid).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &GridCellElem,
) {
    layouter.layout_content(&e.body);
}
