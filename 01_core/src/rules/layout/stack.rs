//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash 720bdf3e
//! @layer L1
//! @updated 2026-06-18
//!
//! Atomização (ADR-0109, P376): o layout de `Stack` movido do monólito
//! `layout_content` para o arquivo da feature. Content-preserving — a lógica
//! é idêntica; só o endereço mudou. O `match` no núcleo delega numa linha.

use crate::entities::content::Content;
use crate::entities::elements::stack::StackElem;
use crate::entities::layout_types::Pt;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout do container compositivo `Stack` (P156I/P273.9).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &StackElem,
) {
    let children = &e.children;
    let dir = &e.dir;
    let spacing = &e.spacing;
    let font = layouter.font_size_pt.val();
    let space_pt = spacing.map_or(0.0, |l| l.resolve_pt(font));

    // Stack é STRUCTURAL: força flush_line antes.
    if layouter.regions.current.cursor_x.0 > layouter.regions.current.line_start_x.0 {
        layouter.flush_line();
    }

    let n = children.len();
    if n == 0 { return; }

    // P273.9 — measure stack bbox via Layouter::measure_stack
    // helper (P273.11 extract — substitui replicação inline
    // ~25 LOC por 1 chamada; bit-exact preserved).
    // Layout duplo arquitectural aceite (sub-padrão N=1 inaugural P273.9).
    // Defaults rigorosos: popular apenas se measured w/h > 0.
    let saved_parent_bbox_p273_9 = layouter.parent_bbox;
    let stack_avail_w = layouter.available_width();
    let (stack_w, stack_h) =
        layouter.measure_stack(children, *dir, *spacing, stack_avail_w);
    if stack_w > 0.0 && stack_h > 0.0 {
        layouter.parent_bbox = Some(crate::entities::layout_types::Rect {
            x: layouter.regions.current.cursor_x,
            y: layouter.regions.current.cursor_y,
            w: Pt(stack_w),
            h: Pt(stack_h),
        });
    }

    // Iteração base — forward para LTR/TTB, reverse para
    // RTL/BTT (per ADR-0054 graded; geometria reverse real
    // adiada para refino futuro).
    let iter: Box<dyn Iterator<Item = (usize, &Content)>> = if dir.is_reverse() {
        Box::new(children.iter().rev().enumerate())
    } else {
        Box::new(children.iter().enumerate())
    };

    if dir.is_vertical() {
        // TTB/BTT: layout cada child em "linha" própria;
        // spacing entre via cursor_y advance.
        for (i, child) in iter {
            if i > 0 && space_pt > 0.0 {
                layouter.regions.current.cursor_y += Pt(space_pt);
            }
            layouter.layout_content(child);
            layouter.flush_line();
        }
    } else {
        // LTR/RTL: layout inline; spacing entre via
        // cursor_x advance.
        for (i, child) in iter {
            if i > 0 && space_pt > 0.0 {
                layouter.regions.current.cursor_x += Pt(space_pt);
            }
            layouter.layout_content(child);
        }
        layouter.flush_line();
    }

    // P273.9 — restore parent_bbox (LIFO).
    layouter.parent_bbox = saved_parent_bbox_p273_9;
}
