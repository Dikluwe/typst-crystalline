//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/table_cell.md
//! @prompt-hash 316f937e
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P380): o layout de `TableCellElem` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::table_cell::TableCellElem;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `TableCell` isolado: single render do body (x/y/colspan/rowspan diferidos, DEBT-34e).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &TableCellElem,
) {
    layouter.layout_content(&e.body);
}
