//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/atomizacao_elementos.md
//! @prompt-hash 59c9666b
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P380): o layout de `VSpace` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::v_space::VSpaceElem;
use crate::entities::layout_types::Pt;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `v(...)` (espaço vertical): flush da linha pendente + avança
/// cursor_y. `weak` diferido.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &VSpaceElem,
) {
    let pt = e.amount.resolve_pt(layouter.style.size.val());
    // Termina linha em curso se houver content pendente — caso
    // contrário texto na linha actual fica meio-render.
    if layouter.regions.current.cursor_x.0 > layouter.regions.current.line_start_x.0 {
        layouter.flush_line();
    }
    layouter.regions.current.cursor_y += Pt(pt);
}
