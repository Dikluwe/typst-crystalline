//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash 3331d6ba
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P380): o layout de `HSpace` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::h_space::HSpaceElem;
use crate::entities::layout_types::Pt;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `h(...)` (espaço horizontal): avança o cursor_x. `weak` diferido.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &HSpaceElem,
) {
    let pt = e.amount.resolve_pt(layouter.font_size_pt.val());
    layouter.regions.current.cursor_x += Pt(pt);
}
