//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/layout.md
//! @layer L1
//! @updated 2026-06-23
//!
//! Layout de `Content::Sequence` — iteração peekable com sticky
//! lookahead e spacing collapse. Extraído de `layout/mod.rs` no P425
//! (ADR-0109 forma B).

use crate::entities::{content::Content, image_sizer::ImageSizer};

use super::metrics::FontMetrics;
use super::Layouter;

/// Layout de uma sequência de conteúdos.
///
/// P250 — refactor Sequence consumer para peekable + neighbour context.
/// Permite:
/// - Sticky lookahead (Block.sticky=true detecta next; se combined_h >
///   remaining + cabe em página inteira → new_page() antes do block).
/// - Spacing collapse via Layouter fields `prev_block_below_pending` +
///   `block_chain_active` (reset entre Sequences para isolamento).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    parts: &[Content],
) {
    let saved_below = layouter.prev_block_below_pending;
    let saved_chain = layouter.block_chain_active;
    layouter.prev_block_below_pending = 0.0;
    layouter.block_chain_active = false;

    let mut iter = parts.iter().peekable();
    while let Some(part) = iter.next() {
        // P250 — sticky pre-layout lookahead 1-block.
        if matches!(part, Content::Block(e) if e.sticky) {
            if let Some(next) = iter.peek() {
                let avail_w = layouter.available_width();
                let (_, part_h) = layouter.measure_content_constrained(part, avail_w);
                let (_, next_h) = layouter.measure_content_constrained(next, avail_w);
                let combined = part_h + next_h;
                let remaining =
                    layouter.page_bottom_limit() - layouter.regions.current.cursor_y.0;
                let page_usable = layouter.available_height();
                if combined > remaining && combined <= page_usable {
                    layouter.new_page();
                }
                // else: cabem ambos OU overlong → emit normal.
            }
        }
        layouter.layout_content(part);
        if !matches!(part, Content::Block { .. } | Content::Shape(_)) {
            // P250 — non-Block child quebra chain.
            // **P767a** — `Content::Shape` também é block-level, logo
            // mantém o estado de colapso de margem.
            layouter.block_chain_active = false;
            layouter.prev_block_below_pending = 0.0;
        }
        // P505 — apenas ListItem/EnumItem consecutivos com tight=false
        // partilham o espaçamento de parágrafo; qualquer outro conteúdo
        // reseta o estado.
        if !matches!(part, Content::ListItem(_) | Content::EnumItem(_)) {
            layouter.last_was_loose_item = false;
        }
        if !matches!(
            part,
            Content::EnumItem(_)
                | Content::Space
                | Content::Parbreak
                | Content::Empty
                | Content::Styled(..)
        ) {
            layouter.enum_counter = None;
        }
    }

    layouter.prev_block_below_pending = saved_below;
    layouter.block_chain_active = saved_chain;
}
