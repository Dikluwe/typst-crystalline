//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/v_space.md
//! @prompt-hash 1dee3141
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
    // Explicit spacing is language content even when its extent is zero.
    if !layouter.is_sub_frame {
        layouter.page_has_unpainted_material = true;
    }
    let pt = e.amount.resolve_pt(layouter.style.size.val());
    // `v` is block-level: it ends the paragraph in progress, which keeps the
    // spacing between paragraphs.
    if super::html_frame::items_have_causal_line(&layouter.regions.current.current_line) {
        super::parbreak::layout(layouter);
    } else if layouter.regions.current.cursor_x.0 > layouter.regions.current.line_start_x.0 {
        layouter.flush_line();
    }
    if e.weak {
        // Weak spacing at the start of a region disappears.
        if !layouter.block_chain_active {
            return;
        }
        // Between blocks or paragraphs it replaces their spacing.
        layouter.prev_block_below_pending = pt;
        layouter.prev_margin_is_parbreak = false;
        layouter.fixed_gap = true;
        return;
    }
    let end = layouter
        .flow_placement_y()
        .unwrap_or(layouter.regions.current.cursor_y.0)
        + pt;
    layouter.regions.current.cursor_y += Pt(pt);
    if !e.weak {
        if layouter.last_block_descent_y.is_some() {
            layouter.last_block_descent_y = Some(end);
        }
        if layouter.block_chain_active {
            layouter.prev_line_baseline += pt;
        }
    }
    layouter.record_flow_placement_end(end);
}
