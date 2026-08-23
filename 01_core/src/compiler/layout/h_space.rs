//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/atomizacao_elementos.md
//! @prompt-hash 59c9666b
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P380): o layout de `HSpace` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::h_space::HSpaceElem;
use crate::entities::layout_types::Pt;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `h(...)` (espaço horizontal).
///
/// `Absolute`: avança o cursor_x imediatamente. `Fractional` (**P842,
/// achado #38**): regista a fração em `pending_fr` da região — a expansão
/// acontece no `flush_line`/`finish`, quando o espaço restante da linha é
/// conhecido (paridade vanilla `HElem` com `Spacing::Fractional`,
/// `layout/spacing.rs`). `weak` diferido.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &HSpaceElem,
) {
    match e.amount {
        crate::entities::elements::h_space::Spacing::Absolute(l) => {
            let pt = l.resolve_pt(layouter.style.size.val());
            layouter.regions.current.cursor_x += Pt(pt);
        }
        crate::entities::elements::h_space::Spacing::Fractional(fr) => {
            let insert_at = layouter.regions.current.current_line.len();
            layouter.regions.current.pending_fr.push((insert_at, fr));
        }
    }
}
