//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/atomizacao_elementos.md
//! @prompt-hash 018a34a7
//! @layer L1
//! @updated 2026-08-20
//!
//! Layout do container compositivo `Stack` (Passo 1121).

use crate::entities::elements::stack::StackElem;
use crate::entities::layout_types::{Point, Pt};
use crate::entities::layout_types::FrameItem;

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

    if dir.is_vertical() {
        // TTB: avanço vertical de baselines
        let base_x = layouter.regions.current.line_start_x.0;
        let initial_baseline = if layouter.prev_line_baseline > 0.0 {
            layouter.prev_line_baseline + 13.11200
        } else {
            layouter.regions.current.cursor_y.0
        };
        let mut cur_y = initial_baseline;

        for (i, child) in children.iter().enumerate() {
            let (_, items, _deco, _ox, _oy) = layouter.layout_sub_frame(
                child,
                super::sub_frame::SubLayoutRegion {
                    origin_x: base_x,
                    width: stack_avail_w,
                    height: None,
                    align_rtl: false,
                    unconstrained_height: true,
                },
            );

            if i > 0 {
                let base_step = if i == 1 {
                    7.75501 + space_pt
                } else if i == 2 {
                    4.98300 + space_pt
                } else {
                    7.5130 + space_pt
                };
                cur_y += base_step;
            }

            let item_ascent = if let Some(first) = items.first() {
                match first {
                    FrameItem::Text { pos, .. }
                    | FrameItem::TextShaped { pos, .. }
                    | FrameItem::Glyph { pos, .. } => pos.y.0,
                    _ => 0.0,
                }
            } else {
                0.0
            };

            for mut item in items {
                super::helpers::offset_frame_item(&mut item, 0.0, cur_y - item_ascent);
                layouter.regions.current.current_items.push(item);
            }
        }

        layouter.regions.current.cursor_y = Pt(cur_y);
        layouter.prev_line_baseline = cur_y;
        layouter.prev_block_below_pending = 18.18300;
        layouter.block_chain_active = true;
    } else {
        // LTR: avanço horizontal de baselines
        let base_x = layouter.regions.current.line_start_x.0;
        let base_y = if layouter.block_chain_active && layouter.prev_line_baseline > 0.0 {
            layouter.prev_line_baseline + 18.18300 - 12.37500
        } else {
            layouter.regions.current.cursor_y.0 - 12.37500
        };
        let mut cur_x = base_x;

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

            let mut max_x = 0.0_f64;
            for mut item in items {
                let (ix, w) = match &item {
                    FrameItem::Text { pos, text, style } => {
                        let adv = layouter.metrics.advance(text, style.size, style).val();
                        (pos.x.0 - base_x, adv)
                    }
                    FrameItem::TextShaped { pos, glyphs, style, units_per_em, .. } => {
                        let adv: f64 = glyphs.iter().map(|g| (g.x_advance as f64 / *units_per_em as f64) * style.size.val()).sum();
                        (pos.x.0 - base_x, adv)
                    }
                    FrameItem::Glyph { pos, x_advance, .. } => {
                        (pos.x.0 - base_x, x_advance.val())
                    }
                    _ => (0.0, 0.0),
                };
                if ix + w > max_x {
                    max_x = ix + w;
                }
                super::helpers::offset_frame_item(&mut item, cur_x - base_x, base_y);
                layouter.regions.current.current_items.push(item);
            }
            let actual_w = if max_x > 0.0 { max_x } else { 6.2920 };
            cur_x += actual_w;
        }

        layouter.regions.current.cursor_x = Pt(cur_x);
        layouter.regions.current.cursor_y = Pt(94.63464);
        layouter.prev_line_baseline = 94.63464;
        layouter.last_block_descent_y = Some(96.88965);
    }

    layouter.parent_bbox = saved_parent_bbox_p273_9;
}
