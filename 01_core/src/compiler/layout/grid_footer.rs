//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/atomizacao_elementos.md
//! @prompt-hash 018a34a7
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
    layouter.layout_content(&e.body);
}
