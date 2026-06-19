//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash 03f97ccc
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P380): o layout de `TableHeaderElem` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::table_header::TableHeaderElem;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `TableHeader` isolado: single render do body (repeat diferido, DEBT-56).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &TableHeaderElem,
) {
    layouter.layout_content(&e.body);
}
