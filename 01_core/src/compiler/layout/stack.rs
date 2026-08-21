//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/atomizacao_elementos.md
//! @prompt-hash 018a34a7
//! @layer L1
//! @updated 2026-08-21
//!
//! Layout dinâmico do container compositivo `Stack` (Passo 1121).

use crate::entities::elements::stack::StackElem;
use crate::entities::layout_types::{FrameItem, Pt};

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout do container compositivo `Stack` (P156I/P273.9/P1121).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &StackElem,
) {
    let children = &e.children;
    let dir = &e.dir;
    let spacing = &e.spacing;
    let font = layouter.style.size.val();
    let space_pt = spacing.map_or(0.0, |l| l.resolve_pt(font));

    if layouter.regions.current.cursor_x.0 > layouter.regions.current.line_start_x.0 {
        layouter.flush_line();
    }
    layouter.ensure_initial_baseline();

    let n = children.len();
    if n == 0 {
        return;
    }

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

    let (top_edge, bottom_edge) =
        layouter.metrics.text_edges(layouter.style.size, &layouter.style);
    let default_below_pt = layouter.style.size.val() * 0.65;

    if dir.is_vertical() {
        // TTB: avanço vertical dinâmico de baselines baseado nas alturas dos sub-frames
        let base_x = layouter.regions.current.line_start_x.0;
        let mut cur_y = layouter.regions.current.cursor_y.0;
        let mut last_child_bottom = cur_y;

        for (i, child) in children.iter().enumerate() {
            if i > 0 && space_pt > 0.0 {
                cur_y += space_pt;
            }

            let (_child_w, items, _deco, _ox, _oy) = layouter.layout_sub_frame(
                child,
                super::sub_frame::SubLayoutRegion {
                    origin_x: base_x,
                    width: stack_avail_w,
                    height: None,
                    align_rtl: false,
                    unconstrained_height: true,
                },
            );

            let item_ascent = items
                .iter()
                .find_map(|item| match item {
                    FrameItem::Text { pos, .. }
                    | FrameItem::TextShaped { pos, .. }
                    | FrameItem::Glyph { pos, .. } => Some(pos.y.0),
                    _ => None,
                })
                .unwrap_or(top_edge.0);

            let mut child_max_y = 0.0_f64;
            for mut item in items {
                let item_bottom = super::helpers::item_bottom_y(&item);
                child_max_y = child_max_y.max(item_bottom);
                super::helpers::offset_frame_item(&mut item, 0.0, cur_y - item_ascent);
                layouter.regions.current.current_items.push(item);
            }

            last_child_bottom = cur_y + (child_max_y - item_ascent).max(0.0);
            cur_y = last_child_bottom;
        }

        layouter.regions.current.cursor_x = layouter.regions.current.line_start_x;
        layouter.regions.current.cursor_y = Pt(cur_y);
        layouter.prev_line_baseline = cur_y;
        layouter.last_block_descent_y = Some(last_child_bottom);
        layouter.prev_block_below_pending = default_below_pt;
        layouter.block_chain_active = true;
    } else {
        // LTR: avanço horizontal dinâmico de baselines baseado nas larguras dos sub-frames
        let base_x = layouter.regions.current.line_start_x.0;
        let base_y = layouter.regions.current.cursor_y.0;
        let mut cur_x = base_x;
        let mut max_descent = bottom_edge.0.abs();

        for (i, child) in children.iter().enumerate() {
            if i > 0 && space_pt > 0.0 {
                cur_x += space_pt;
            }
            let (_child_w, items, _deco, _ox, _oy) = layouter.layout_sub_frame(
                child,
                super::sub_frame::SubLayoutRegion {
                    origin_x: base_x,
                    width: stack_avail_w,
                    height: None,
                    align_rtl: false,
                    unconstrained_height: true,
                },
            );

            let item_ascent = items
                .iter()
                .find_map(|item| match item {
                    FrameItem::Text { pos, .. }
                    | FrameItem::TextShaped { pos, .. }
                    | FrameItem::Glyph { pos, .. } => Some(pos.y.0),
                    _ => None,
                })
                .unwrap_or(top_edge.0);

            let mut child_w = 0.0_f64;
            for mut item in items {
                let (ix, _) = super::helpers::item_pos(&item);
                let w = super::helpers::item_width(&item, &layouter.metrics);
                child_w = child_w.max(ix - base_x + w);

                let item_bottom = super::helpers::item_bottom_y(&item);
                max_descent = max_descent.max(item_bottom - item_ascent);

                super::helpers::offset_frame_item(&mut item, cur_x - base_x, base_y - item_ascent);
                layouter.regions.current.current_items.push(item);
            }
            cur_x += child_w;
        }

        layouter.regions.current.cursor_x = layouter.regions.current.line_start_x;
        layouter.regions.current.cursor_y = Pt(base_y);
        layouter.prev_line_baseline = base_y;
        layouter.last_block_descent_y = Some(base_y + max_descent);
        layouter.prev_block_below_pending = default_below_pt;
        layouter.block_chain_active = true;
    }

    layouter.parent_bbox = saved_parent_bbox_p273_9;
}
