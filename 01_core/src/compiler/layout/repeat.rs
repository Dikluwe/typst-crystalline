//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/repeat.md
//! @prompt-hash c45c8d4a
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P380): o layout de `RepeatElem` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::repeat::RepeatElem;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `Repeat`: single-render do body no contexto actual (paridade estrutural).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &RepeatElem,
) {
    layouter.layout_content(&e.body);
}
