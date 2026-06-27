//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash e6442e3f
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P380): o layout de `TableFooterElem` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::table_footer::TableFooterElem;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `TableFooter` isolado: single render do body (repeat diferido, DEBT-56).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &TableFooterElem,
) {
    layouter.layout_content(&e.body);
}
