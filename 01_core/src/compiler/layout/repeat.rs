//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/repeat.md
//! @prompt-hash a3a62b26
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P380): o layout de `RepeatElem` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::repeat::RepeatElem;
use crate::entities::layout_types::{FrameItem, Pt};

use super::{FontMetrics, ImageSizer, Layouter};

/// P1407 — peça já disposta que aguarda a largura atribuída pelo flow.
#[derive(Debug, Clone)]
pub(super) struct PendingRepeat {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) width: f64,
    pub(super) gap: f64,
    pub(super) justify: bool,
}

/// Layout de `Repeat`: dispõe uma peça e conserva a faixa para expansão no
/// fechamento da linha, quando a largura fracionária já é conhecida.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &RepeatElem,
) {
    let items_before = layouter.regions.current.current_items.len();
    let start = layouter.regions.current.current_line.len();
    let x_before = layouter.regions.current.cursor_x.0;
    layouter.layout_content(&e.body);
    let end = layouter.regions.current.current_line.len();
    let stayed_inline = layouter.regions.current.current_items.len() == items_before;
    let width = layouter.regions.current.cursor_x.0 - x_before;
    let gap = e.gap.map_or(0.0, |value| value.resolve_pt(layouter.style.size.val()));
    if stayed_inline
        && end > start
        && width.is_finite()
        && width > 0.0
        && gap.is_finite()
        && gap >= 0.0
    {
        layouter.pending_repeats.push(PendingRepeat {
            start,
            end,
            width,
            gap,
            justify: e.justify,
        });
    }
}

/// Expande uma peça dentro de `fill`, preservando a cópia inicial. Retorna o
/// número de `FrameItem`s inseridos para correção de índices externos.
pub(super) fn expand_pending(
    line: &mut Vec<FrameItem>,
    pending: &PendingRepeat,
    fill: f64,
) -> usize {
    if pending.start >= pending.end
        || pending.end > line.len()
        || !fill.is_finite()
        || fill <= 0.0
    {
        return 0;
    }
    let stride = pending.width + pending.gap;
    if !stride.is_finite() || stride <= 0.0 {
        return 0;
    }
    let count = (((fill + pending.gap) / stride).floor() as usize).clamp(1, 1000);
    if count <= 1 {
        return 0;
    }
    let occupied = count as f64 * pending.width + (count - 1) as f64 * pending.gap;
    let residual = (fill - occupied).max(0.0);
    let gap = if pending.justify {
        pending.gap + residual / (count - 1) as f64
    } else {
        pending.gap
    };
    let template = line[pending.start..pending.end].to_vec();
    let template_len = template.len();
    let mut copies = Vec::with_capacity((count - 1).saturating_mul(template_len));
    for copy in 1..count {
        let dx = copy as f64 * (pending.width + gap);
        copies.extend(template.iter().cloned().map(|item| {
            let (x, y) = super::helpers::item_pos(&item);
            super::helpers::translate_frame_item(item, Pt(x + dx), Pt(y))
        }));
    }
    let inserted = copies.len();
    line.splice(pending.end..pending.end, copies);
    inserted
}
