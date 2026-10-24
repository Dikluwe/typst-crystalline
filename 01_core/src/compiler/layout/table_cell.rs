//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/table_cell.md
//! @prompt-hash cd90026b
//! @layer L1
//! @updated 2026-10-07
//!
//! Atomização (ADR-0109, P380): o layout de `TableCellElem` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::table_cell::TableCellElem;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `TableCell` isolado: single render do body (x/y/colspan/rowspan diferidos, DEBT-34e).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &TableCellElem,
) {
    if let Some(resolved) = &e.resolved {
        let nonzero = resolved.inset
            != crate::entities::sides::Sides::uniform(
                crate::entities::layout_types::Length::ZERO,
            );
        if nonzero
            && super::html_frame::items_have_material(
                &layouter.regions.current.current_line,
            )
        {
            let mut visual = e.visual().clone();
            if let Some(align) = resolved.align {
                visual = crate::entities::content::Content::align(align, visual);
            }
            let body = crate::entities::content::Content::block(
                visual,
                None,
                None,
                resolved.inset,
                resolved.breakable,
            );
            if let Some(cursor) = layouter.planned_child_cursor(
                crate::compiler::introspect::page_running::PlanEdge::CellSelectedVisual,
            ) {
                let mut edges =
                    vec![crate::compiler::introspect::page_running::PlanEdge::BlockBody];
                if resolved.align.is_some() {
                    edges.push(
                        crate::compiler::introspect::page_running::PlanEdge::AlignBody,
                    );
                }
                layouter.layout_planned_cursor(
                    cursor.through_generated(&edges),
                    &body,
                    false,
                );
            } else {
                layouter.layout_content(&body);
            }
            return;
        }
    }
    layouter.layout_planned_child(
        crate::compiler::introspect::page_running::PlanEdge::CellSelectedVisual,
        e.visual(),
    );
}
