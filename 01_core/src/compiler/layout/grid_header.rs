//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/grid_header.md
//! @prompt-hash ccb25183
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P380): o layout de `GridHeaderElem` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::grid_header::GridHeaderElem;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `GridHeader` isolado: renderiza o body (dentro de Grid é consumido por layout_grid).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &GridHeaderElem,
) {
    layouter.layout_content(&e.body);
}
