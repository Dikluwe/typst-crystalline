//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash 3331d6ba
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P381): o layout de `Hide` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::hide::HideElem;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `hide(...)`: executa o body para avançar o cursor, mas descarta
/// os items gerados (mantém só o avanço — per ADR-0054 graded).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &HideElem,
) {
    let saved_items = std::mem::take(&mut layouter.regions.current.current_items);
    let saved_line  = std::mem::take(&mut layouter.regions.current.current_line);
    layouter.layout_content(&e.body);
    layouter.regions.current.current_items = saved_items;
    layouter.regions.current.current_line  = saved_line;
}
