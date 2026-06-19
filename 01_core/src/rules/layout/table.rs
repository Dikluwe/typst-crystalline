//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash 03f97ccc
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P380): o layout de `Table` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.
//! Cluster `layout_grid` (P379): delega ao mesmo motor que `Grid`.

use crate::entities::elements::table::TableElem;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `table(...)` (P157A, ADR-0060 Fase 2): delega a `layout_grid`
/// (clone simples — herda stroke + fill; inset zero).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &TableElem,
) {
    // P224+P227+P228 — Table delegate; herda stroke + fill.
    layouter.layout_grid(&e.columns, &e.rows, &e.children,
                         None, None,
                         crate::entities::sides::Sides::uniform(
                             crate::entities::layout_types::Length::pt(0.0)),
                         None, None,
                         e.stroke.as_ref(), e.fill.as_ref());
}
