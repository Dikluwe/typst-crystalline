//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/pagebreak.md
//! @prompt-hash 2ccc19fc
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P380): o layout de `Pagebreak` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::pagebreak::PagebreakElem;

use super::{FontMetrics, ImageSizer, Layouter};

/// Break intent controls whether the newly opened page is required.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &PagebreakElem,
) {
    let empty = page_is_empty(layouter);
    let consume_boundary = empty
        && (layouter.page_run_boundary_empty
            || (!e.weak && layouter.weak_page_successor));
    // A skipped weak break preserves an initial/strong required successor.
    let optional_successor = if e.weak && empty && !consume_boundary {
        layouter.weak_page_successor
    } else {
        e.weak
    };
    if consume_boundary {
        layouter.page_run_boundary_empty = false;
        layouter.weak_page_successor = e.weak;
    } else if !e.weak || !empty {
        if layouter.regions.current.cursor_x.0 > layouter.regions.current.line_start_x.0
            || layouter
                .regions
                .current
                .current_line
                .iter()
                .any(super::position_marker::is_marker)
        {
            layouter.flush_line();
        }
        layouter.new_page();
        layouter.weak_page_successor = e.weak;
    }
    // Skipping a weak break does not waive its physical parity requirement.
    if let Some(parity) = e.to {
        if !parity.matches(layouter.pages.len() + 1) {
            layouter.new_page();
            layouter.weak_page_successor = optional_successor;
        }
    }
}

/// Deferred insertions and closed columns also carry page material.
pub(super) fn page_is_empty<M: FontMetrics, S: ImageSizer>(
    layouter: &Layouter<M, S>,
) -> bool {
    !layouter.page_has_unpainted_material
        && layouter.current_page_is_empty()
        && !layouter
            .column_page_items
            .iter()
            .any(|items| super::html_frame::items_have_material(items))
        && layouter.floats_pending.is_empty()
        && layouter.pending_footnote_bodies.is_empty()
        && layouter.pending_cell_tails.is_empty()
}
