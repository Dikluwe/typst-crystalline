//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash 3331d6ba
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P381): o layout de `Link` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::link::LinkElem;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de uma hiperligação: renderiza o body (sublinhado/cor diferidos —
/// requer `FrameItem::Decoration`, futuro).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &LinkElem,
) {
    layouter.layout_content(&e.body);
}
