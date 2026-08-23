//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/atomizacao_elementos.md
//! @prompt-hash 59c9666b
//! @layer L1
//! @updated 2026-08-21
//!
//! Layout dinâmico do container compositivo `Stack` (Passo 1121).

use crate::entities::elements::stack::StackElem;
use crate::entities::layout_types::{FrameItem, Pt};

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout do container compositivo `Stack` (P156I/P273.9/P1121).
fn extract_frame_ascent_and_descent<M: FontMetrics>(
    items: &[FrameItem],
    top_edge: f64,
    metrics: &M,
) -> (f64, f64, f64) {
    let mut min_y = f64::INFINITY;
    let mut max_bottom = 0.0_f64;
    let mut found_text = false;
    let mut max_ink_ascent = 0.0_f64;

    fn walk<M: FontMetrics>(
        item: &FrameItem,
        cur_y: f64,
        min_y: &mut f64,
        max_bottom: &mut f64,
        found_text: &mut bool,
        max_ink_ascent: &mut f64,
        metrics: &M,
    ) {
        match item {
            FrameItem::Text { pos, text, style } => {
                *min_y = min_y.min(cur_y + pos.y.0);
                *found_text = true;
                let ib = metrics.text_ink_bounds_signed(text, style.size, style);
                *max_ink_ascent = max_ink_ascent.max(ib.0 .0);
                *max_bottom = max_bottom.max(cur_y + pos.y.0 + ib.1 .0);
            }
            FrameItem::TextShaped { pos, text, style, .. } => {
                *min_y = min_y.min(cur_y + pos.y.0);
                *found_text = true;
                let ib = metrics.text_ink_bounds_signed(text, style.size, style);
                *max_ink_ascent = max_ink_ascent.max(ib.0 .0);
                *max_bottom = max_bottom.max(cur_y + pos.y.0 + ib.1 .0);
            }
            FrameItem::Glyph { pos, size, style, base_char, .. } => {
                *min_y = min_y.min(cur_y + pos.y.0);
                *found_text = true;
                let mut buf = [0u8; 4];
                let s = base_char.encode_utf8(&mut buf);
                let ib = metrics.text_ink_bounds_signed(s, *size, style);
                *max_ink_ascent = max_ink_ascent.max(ib.0 .0);
                *max_bottom = max_bottom.max(cur_y + pos.y.0 + ib.1 .0);
            }
            FrameItem::Group { pos, items, .. } => {
                for child in items {
                    walk(
                        child,
                        cur_y + pos.y.0,
                        min_y,
                        max_bottom,
                        found_text,
                        max_ink_ascent,
                        metrics,
                    );
                }
            }
            _ => {
                let b = super::helpers::item_bottom_y(item);
                *max_bottom = max_bottom.max(cur_y + b);
            }
        }
    }

    for item in items {
        walk(
            item,
            0.0,
            &mut min_y,
            &mut max_bottom,
            &mut found_text,
            &mut max_ink_ascent,
            metrics,
        );
    }

    if found_text {
        let ascent = min_y;
        let descent = (max_bottom - min_y).max(0.0);
        (ascent, descent, max_ink_ascent)
    } else {
        (top_edge, 0.0, top_edge)
    }
}

pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &StackElem,
) {
    let children = &e.children;
    let dir = &e.dir;
    let spacing = &e.spacing;
    let font = layouter.style.size.val();
    let space_pt = spacing.map_or(0.0, |l| l.resolve_pt(font));
    let parent_block_chain = layouter.block_chain_active;
    let parent_pending_below = layouter.prev_block_below_pending;
    let parent_prev_baseline = layouter.prev_line_baseline;
    let parent_last_bottom = layouter.last_block_descent_y;

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
    let default_below_pt = layouter.style.size.val() * 1.2;

    if dir.is_vertical() {
        // TTB: avanço vertical dinâmico exato entre baselines:
        // baseline[i] = baseline[i-1] + descent[i-1] + spacing + ascent[i]
        let base_x = layouter.regions.current.line_start_x.0;
        let mut sub_frames = Vec::with_capacity(children.len());

        for child in children.iter() {
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

            let (item_ascent, max_descent, ink_ascent) =
                extract_frame_ascent_and_descent(&items, top_edge.0, &layouter.metrics);

            sub_frames.push((items, item_ascent, max_descent, ink_ascent));
        }

        let mut cur_baseline = if parent_block_chain && parent_pending_below > 0.0 {
            parent_prev_baseline + parent_pending_below + sub_frames[0].3
        } else {
            layouter.regions.current.cursor_y.0
        };
        let mut prev_descent = 0.0_f64;
        let mut last_bottom = cur_baseline;

        for (i, (items, item_ascent, max_descent, _)) in
            sub_frames.into_iter().enumerate()
        {
            if i > 0 {
                cur_baseline += prev_descent + space_pt + item_ascent;
            }

            for mut item in items {
                super::helpers::offset_frame_item(
                    &mut item,
                    0.0,
                    cur_baseline - item_ascent,
                );
                layouter.regions.current.current_items.push(item);
            }

            prev_descent = max_descent;
            last_bottom = cur_baseline + max_descent;
        }

        layouter.regions.current.cursor_x = layouter.regions.current.line_start_x;
        layouter.regions.current.cursor_y = Pt(cur_baseline);
        layouter.prev_line_baseline = cur_baseline;
        layouter.last_block_descent_y = Some(last_bottom);
        layouter.prev_block_below_pending = default_below_pt;
        layouter.block_chain_active = true;
    } else {
        // LTR: avanço horizontal dinâmico de baselines baseado nas larguras dos sub-frames
        let base_x = layouter.regions.current.line_start_x.0;
        let mut base_y = layouter.regions.current.cursor_y.0;
        let mut cur_x = base_x;
        let mut max_descent = bottom_edge.0.abs();
        let mut common_top_offset = None;

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
            let (_, child_ink_descent, child_ink_ascent) =
                extract_frame_ascent_and_descent(&items, top_edge.0, &layouter.metrics);
            max_descent = max_descent.max(child_ink_descent);
            if i == 0 && parent_block_chain && parent_pending_below > 0.0 {
                base_y = parent_last_bottom.unwrap_or(parent_prev_baseline)
                    + parent_pending_below
                    + child_ink_ascent;
            }
            let top_offset = *common_top_offset.get_or_insert(base_y - item_ascent);

            let mut child_w = 0.0_f64;
            for mut item in items {
                let (ix, _) = super::helpers::item_pos(&item);
                let w = super::helpers::item_width(&item, &layouter.metrics);
                child_w = child_w.max(ix - base_x + w);

                let item_bottom = super::helpers::item_bottom_y(&item);
                max_descent = max_descent.max(item_bottom - item_ascent);

                super::helpers::offset_frame_item(&mut item, cur_x - base_x, top_offset);
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
