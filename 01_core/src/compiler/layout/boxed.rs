//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/boxed.md
//! @prompt-hash 37ca0007
//! @layer L1
//! @updated 2026-10-21
//!
//! Inline Box composition from actual child geometry.
//! Certified fractional widths precede body composition; opaque scopes retain line closure.

use crate::entities::elements::boxed::BoxedElem;
use crate::entities::layout_types::{FrameItem, Pt};

use super::{FontMetrics, ImageSizer, Layouter};

/// Compose an inline Box using its actual body frame.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &BoxedElem,
) {
    if !super::html_frame::items_have_material(&layouter.regions.current.current_line) {
        if layouter.block_chain_active {
            let gap = if layouter.prev_margin_is_parbreak {
                layouter
                    .prev_block_below_pending
                    .max(super::paragraph::spacing_for_size(
                        &layouter.chain,
                        layouter.style.size.0,
                    ))
            } else {
                layouter.prev_block_below_pending
            };
            layouter.regions.current.cursor_y = Pt(layouter.prev_line_baseline
                + layouter.prev_block_equation_descent
                + gap);
            // The actual Box frame installs its own ascent at this geometric origin.
            layouter.initial_baseline_pending = true;
            layouter.line_assumed_ascent = None;
            layouter.prev_margin_is_parbreak = false;
            layouter.prev_block_below_pending = 0.0;
            layouter.block_chain_active = false;
            layouter.prev_block_equation_descent = 0.0;
        } else if at_fresh_root_origin(layouter) {
            layouter.regions.current.cursor_y = Pt(layouter.page_config.margin.top);
            layouter.initial_baseline_pending = true;
            layouter.line_assumed_ascent = None;
            layouter.prev_margin_is_parbreak = false;
        } else if !layouter.initial_baseline_pending {
            layouter.ensure_initial_baseline();
        }
    }

    let assigned =
        super::sequence::inline_allocation::assigned(layouter.inline_allocation.take());
    if e.width_fraction.is_some() {
        if let Some(width) = assigned {
            // The exterior certificate supplies a final total width. Compose
            // this body once in its real independent region, including flow.
            layout_sized(layouter, e, Some(width));
        } else {
            // An uncertified inline adapter still has the parent's provisional
            // width. Its descendants cannot treat that width as a local share.
            let saved = layouter.inline_allocation.replace(std::sync::Arc::new(
                super::sequence::inline_allocation::Node::Blocked,
            ));
            layout_fractional(layouter, e);
            layouter.inline_allocation = saved;
        }
    } else {
        layout_sized(layouter, e, None);
    }
}

// A new root region may have a predicted font baseline from the preceding
// text flow. With no causal unit here, the first frame owns its geometric top.
fn at_fresh_root_origin<M: FontMetrics, S: ImageSizer>(
    layouter: &Layouter<M, S>,
) -> bool {
    !layouter.initial_baseline_pending
        && !layouter.is_sub_frame
        && layouter.page_config.height.is_finite()
        && layouter.regions.cell.is_none()
        && !layouter.column_mode
        && layouter.page_columns.is_none()
        && layouter.regions.current.height == layouter.page_config.height
        && !super::html_frame::items_have_causal_line(
            &layouter.regions.current.current_items,
        )
        && !super::html_frame::items_have_causal_line(
            &layouter.regions.current.current_line,
        )
        && layouter.flow_placement_end.is_none()
        && layouter.last_flush_advance == 0.0
        && layouter.floats_pending.is_empty()
        && layouter.cursor_y_top_reserve == 0.0
        && layouter.cursor_y_bottom_reserve == 0.0
        && layouter.decoration_lines_collector.is_none()
        && !layouter.flow_suffix_checkpoint_pending
        && !layouter.flow_suffix_transaction_active
        && layouter.flow_suffix_checkpoint.is_none()
}

// Fractional widths are allocated by the existing line-close owner.
fn layout_fractional<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &BoxedElem,
) {
    let (
        body,
        width,
        width_fraction,
        height,
        inset,
        baseline,
        outset,
        radius,
        clip,
        fill,
        stroke,
    ) = (
        &e.body,
        &e.width,
        &e.width_fraction,
        &e.height,
        &e.inset,
        &e.baseline,
        &e.outset,
        &e.radius,
        &e.clip,
        &e.fill,
        &e.stroke,
    );
    let font = layouter.style.size.val();
    let inset_left = inset.left.resolve_pt(font);
    let inset_right = inset.right.resolve_pt(font);
    let inset_top = inset.top.resolve_pt(font);
    let inset_bottom = inset.bottom.resolve_pt(font);

    // Outset expands the backing paint around the actual Box height.
    // It does not change body position, nominal advance or line extents.
    let outset_left = outset.left.resolve_pt(font);
    let outset_right = outset.right.resolve_pt(font);
    let outset_top = outset.top.resolve_pt(font);
    let outset_bottom = outset.bottom.resolve_pt(font);
    let has_shape = fill.is_some() || stroke.is_some();
    let has_outset = outset_left != 0.0
        || outset_right != 0.0
        || outset_top != 0.0
        || outset_bottom != 0.0;

    // Retain the insertion point so the backing paints before its body.
    let items_before = layouter.regions.current.current_items.len();

    // The parent supplies either a geometric top or an installed baseline.
    // A pending first body receives top inset before installing real ascent.
    let start_x = layouter.regions.current.cursor_x.0;
    let parent_baseline = layouter.regions.current.cursor_y.0;
    let geometric_pending = layouter.initial_baseline_pending;
    if geometric_pending {
        layouter.regions.current.cursor_y += Pt(inset_top);
    }

    // The inline body starts after left inset; outset remains paint-only.
    layouter.regions.current.cursor_x += Pt(inset_left);

    // An explicit width temporarily constrains body layout and is restored
    // before the parent resumes its own line.
    let saved_width = layouter.regions.current.width;
    if let Some(w) = width {
        let w_pt = w.resolve_pt(font);
        let cursor_x_pt = layouter.regions.current.cursor_x.0;
        layouter.regions.current.width = (cursor_x_pt + w_pt).max(0.0);
    }

    let baseline_shift = baseline.resolve_pt(font);
    let body_measure_width = width.map(|w| w.resolve_pt(font)).unwrap_or_else(|| {
        super::paragraph_indent::current_width(layouter, layouter.available_width())
    });
    let (measured_body_width, measured_body_height) =
        layouter.measure_content_constrained(body, body_measure_width);

    // Capture the body's committed range separately from its pending tail.
    // Explicit-height clipping below retains the existing fractional path.
    let body_items_before = layouter.regions.current.current_items.len();

    // Preserve the bounded explicit-size parent paint reference policy.
    // The backing restores the exterior reference after body composition.
    let saved_parent_bbox = layouter.parent_bbox;
    if let (Some(w), Some(h)) = (width, height) {
        let w_pt = w.resolve_pt(font);
        let h_pt = h.resolve_pt(font);
        layouter.parent_bbox = Some(crate::entities::layout_types::Rect {
            x: layouter.regions.current.cursor_x,
            y: layouter.regions.current.cursor_y,
            w: Pt(w_pt),
            h: Pt(h_pt),
        });
    }

    let parent_line_len_before = layouter.regions.current.current_line.len();
    // Collect body extents independently; the Box's explicit total height
    // supplies the resulting parent-line contribution.
    let saved_line_ascent = layouter.line_inline_ascent;
    let saved_line_descent = layouter.line_inline_descent;
    layouter.line_inline_ascent = 0.0;
    layouter.line_inline_descent = 0.0;
    // Isolate the parent line while the one planned body is emitted.
    // Local alignment and internal flushes cannot consume the parent prefix.
    let parent_line = std::mem::take(&mut layouter.regions.current.current_line);
    // This body temporarily owns the line. Its committed local frames cannot
    // consume the enclosing flow's first-frame baseline decision.
    let parent_flow_baseline = layouter.flow_baseline.take();
    // Pending fraction/repeat indices belong to the line that owns them.
    let parent_pending_fr = std::mem::take(&mut layouter.regions.current.pending_fr);
    let parent_pending_fr_natural_widths =
        std::mem::take(&mut layouter.pending_fr_natural_widths);
    let parent_pending_repeats = std::mem::take(&mut layouter.pending_repeats);
    let body_origin_x = layouter.regions.current.cursor_x.0;
    let planned = layouter.planned_child_cursor(
        crate::compiler::introspect::page_running::PlanEdge::BoxedBody,
    );
    let saved_sub_frame = std::mem::replace(&mut layouter.is_sub_frame, true);
    let (_, mut body_items) = layouter.layout_sub_frame_inline_planned(
        body,
        super::sub_frame::SubLayoutRegion {
            origin_x: 0.0,
            width: 0.0,
            height: None,
            align_rtl: true,
            unconstrained_height: true,
        },
        planned,
    );
    layouter.is_sub_frame = saved_sub_frame;
    layouter.flow_baseline = parent_flow_baseline;
    // A body may have closed local lines before returning its pending tail.
    // Both ranges belong to this Box and must participate in its line extent.
    let mut committed_body: Vec<FrameItem> = layouter
        .regions
        .current
        .current_items
        .drain(body_items_before..)
        .collect();
    let committed_body_count = committed_body.len();
    committed_body.append(&mut body_items);
    let mut body_items = committed_body;
    let first_body_baseline = first_baseline(&body_items);
    let (actual_text_top, actual_text_bottom) =
        fractional_text_edges(&body_items, &layouter.metrics);
    let body_ascent = layouter.line_inline_ascent.max(actual_text_top);
    let body_descent = layouter.line_inline_descent.max(-actual_text_bottom);
    let body_has_text_baseline = frame_items_have_text_baseline(&body_items);
    let natural_body_height = if body_has_text_baseline {
        body_ascent + body_descent
    } else {
        measured_body_height
    };
    let box_height = height
        .map_or(inset_top + natural_body_height + inset_bottom, |h| h.resolve_pt(font));
    let natural_baseline =
        if body_has_text_baseline { inset_top + body_ascent } else { box_height };
    let frame_baseline = natural_baseline - baseline_shift;
    let installed_ascent = frame_baseline.max(0.0);
    let outer_baseline = if geometric_pending {
        parent_baseline + installed_ascent
    } else {
        parent_baseline
    };
    if let Some(first) = first_body_baseline {
        let shift_y = outer_baseline + baseline_shift - first.y.0;
        offset_items(&mut body_items, 0.0, shift_y);
    }
    layouter.regions.current.cursor_y = Pt(outer_baseline);
    if geometric_pending {
        layouter.line_assumed_ascent = Some(installed_ascent);
        layouter.initial_baseline_pending = false;
    }
    // Rebase the local pending indices before restoring the parent prefix.
    // Internal body flushes never see or drain those exterior queues.
    let mut body_pending_fr = std::mem::take(&mut layouter.regions.current.pending_fr);
    let mut body_pending_fr_natural_widths =
        std::mem::take(&mut layouter.pending_fr_natural_widths);
    let mut body_pending_repeats = std::mem::take(&mut layouter.pending_repeats);
    for (index, _) in &mut body_pending_fr {
        *index += committed_body_count + parent_line_len_before;
    }
    for repeat in &mut body_pending_repeats {
        repeat.start += committed_body_count + parent_line_len_before;
        repeat.end += committed_body_count + parent_line_len_before;
    }
    layouter.regions.current.current_line = parent_line;
    layouter.regions.current.pending_fr = parent_pending_fr;
    layouter.regions.current.pending_fr.append(&mut body_pending_fr);
    layouter.pending_fr_natural_widths = parent_pending_fr_natural_widths;
    layouter
        .pending_fr_natural_widths
        .append(&mut body_pending_fr_natural_widths);
    layouter.pending_repeats = parent_pending_repeats;
    layouter.pending_repeats.append(&mut body_pending_repeats);
    if body_items.is_empty() && measured_body_width > 0.0 {
        layouter.regions.current.cursor_x = Pt(body_origin_x + measured_body_width);
    }
    layouter.regions.current.current_line.extend(body_items);
    layouter.text_tracking_state.interrupt();
    let fraction_insert_at =
        width_fraction.map(|_| layouter.regions.current.current_line.len());

    // Restore the exterior paint reference before emitting the Box backing.
    layouter.parent_bbox = saved_parent_bbox;

    // Restore the parent's available width.
    layouter.regions.current.width = saved_width;

    // Record provisional nominal occupancy once; line closure replaces it
    // with the fractional allocation instead of adding both widths.
    let outer_w = if let Some(w) = width {
        w.resolve_pt(font)
    } else {
        layouter.regions.current.cursor_x.0 - start_x + inset_right
    };
    layouter.regions.current.cursor_x = Pt(start_x + outer_w);
    if let (Some(fr), Some(insert_at)) = (width_fraction, fraction_insert_at) {
        layouter.regions.current.pending_fr.push((insert_at, *fr));
        layouter.pending_fr_natural_widths.push(outer_w);
    }

    layouter.line_inline_ascent = saved_line_ascent;
    layouter.line_inline_descent = saved_line_descent;

    layouter.note_inline_extent(
        frame_baseline.max(0.0),
        (box_height - frame_baseline).max(0.0),
    );

    // Preserve the established fractional grouping and pending-index protocol
    // while recording explicit height and its optional clip.
    if let Some(h) = height {
        let h_pt = h.resolve_pt(font);
        let mut found_group = false;
        for item in &mut layouter.regions.current.current_line[parent_line_len_before..] {
            if let FrameItem::Group { inner_height, .. } = item {
                *inner_height = inner_height.max(h_pt);
                found_group = true;
            }
        }
        if !found_group
            && !layouter.regions.current.current_line[parent_line_len_before..].is_empty()
        {
            let pos_box = crate::entities::layout_types::Point {
                x: Pt(start_x + inset_left),
                y: layouter.regions.current.cursor_y,
            };
            // Body items use line coordinates. Normalize them to the group's
            // origin so the exporter adds that origin exactly once.
            let mut body_items: Vec<FrameItem> = layouter
                .regions
                .current
                .current_line
                .drain(parent_line_len_before..)
                .collect();
            offset_items(&mut body_items, -pos_box.x.0, -pos_box.y.0);
            layouter.regions.current.current_line.push(FrameItem::Group {
                paint_root: false,
                pos: pos_box,
                matrix: crate::entities::layout_types::TransformMatrix::identity(),
                clip_mask: None,
                inner_width: outer_w,
                inner_height: h_pt,
                items: body_items,
            });
        }
        if *clip {
            let avail_w_box = match width {
                Some(w) => w.resolve_pt(font),
                None => super::paragraph_indent::current_width(
                    layouter,
                    layouter.available_width(),
                ),
            };
            let (body_w_real, body_h_real) =
                layouter.measure_content_constrained(body, avail_w_box);
            if body_h_real > h_pt {
                let body_items: Vec<FrameItem> = layouter
                    .regions
                    .current
                    .current_line
                    .drain(parent_line_len_before..)
                    .collect();
                let pos_box = crate::entities::layout_types::Point {
                    x: Pt(start_x + inset_left),
                    y: layouter.regions.current.cursor_y
                        - layouter
                            .metrics
                            .vertical_metrics(layouter.style.size, &layouter.style)
                            .1,
                };
                layouter.regions.current.current_line.push(FrameItem::Group {
                    paint_root: false,
                    pos: pos_box,
                    matrix: crate::entities::layout_types::TransformMatrix::identity(),
                    clip_mask: Some(crate::entities::geometry::ShapeKind::Rect),
                    inner_width: body_w_real,
                    inner_height: h_pt,
                    items: body_items,
                });
            }
        }
    }

    // Backing geometry uses the actual Box baseline and total height.
    // Outset expands only this paint rectangle.
    if has_shape || has_outset {
        let outer_h = box_height + outset_top + outset_bottom;
        let pos = crate::entities::layout_types::Point {
            x: Pt(start_x - outset_left),
            y: Pt(outer_baseline - frame_baseline - outset_top),
        };
        // The stroke is centered on the backing edge and does not change
        // the Box's nominal layout dimensions.
        let radius_is_zero_p247 = radius.top_left
            == crate::entities::layout_types::Length::ZERO
            && radius.top_right == crate::entities::layout_types::Length::ZERO
            && radius.bottom_right == crate::entities::layout_types::Length::ZERO
            && radius.bottom_left == crate::entities::layout_types::Length::ZERO;
        let shape_kind = if radius_is_zero_p247 {
            crate::entities::geometry::ShapeKind::Rect
        } else {
            super::resolve_shape_kind(
                &crate::entities::geometry::ShapeKind::RoundedRect { radii: *radius },
                layouter.style.size,
            )
        };
        let prepared_stroke = super::tiling::resolve_stroke(layouter, stroke.as_ref());
        layouter.regions.current.current_items.insert(
            items_before,
            FrameItem::Shape {
                pos,
                kind: shape_kind,
                width: outer_w + outset_left + outset_right,
                height: outer_h,
                fill: fill.clone().map(crate::entities::frame_paint::FramePaint::Solid),
                stroke: prepared_stroke,
                // The backing resolves parent-relative paint against the
                // exterior reference restored above.
                fill_rule: Default::default(),
                parent_bbox_at_emit: layouter.parent_bbox,
            },
        );
    }
}

/// Compose the full body in a local region before placing this inline object.
fn layout_sized<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &BoxedElem,
    assigned_width: Option<f64>,
) {
    use crate::entities::geometry::ShapeKind;
    use crate::entities::layout_types::{Length, Point, Rect, TransformMatrix};

    let font = layouter.style.size.0;
    let resolve = |s: crate::entities::sides::Sides<Length>| {
        crate::entities::sides::Sides::new(
            s.left.resolve_pt(font),
            s.top.resolve_pt(font),
            s.right.resolve_pt(font),
            s.bottom.resolve_pt(font),
        )
    };
    let inset = resolve(e.inset);
    let outset = resolve(e.outset);
    let explicit_w =
        assigned_width.or_else(|| e.width.map(|v| v.resolve_pt(font).max(0.0)));
    let explicit_h = e.height.map(|v| v.resolve_pt(font).max(0.0));
    let inner_w = (explicit_w.unwrap_or_else(|| {
        super::paragraph_indent::current_width(layouter, layouter.available_width())
    }) - inset.left
        - inset.right)
        .max(0.0);
    let inner_h = explicit_h.map(|h| (h - inset.top - inset.bottom).max(0.0));
    let mut start = Point {
        x: layouter.regions.current.cursor_x,
        y: layouter.regions.current.cursor_y,
    };
    let saved_bbox = layouter.parent_bbox;
    if let (Some(w), Some(h)) = (explicit_w, explicit_h) {
        layouter.parent_bbox = Some(Rect {
            x: Pt(-inset.left),
            y: Pt(-inset.top),
            w: Pt(w),
            h: Pt(h),
        });
    }
    let planned = layouter.planned_child_cursor(
        crate::compiler::introspect::page_running::PlanEdge::BoxedBody,
    );
    let (body_h, mut body, deco, align_x, align_y) = layouter.layout_sub_frame_planned(
        &e.body,
        super::sub_frame::SubLayoutRegion {
            origin_x: 0.0,
            width: inner_w,
            height: inner_h,
            align_rtl: true,
            unconstrained_height: inner_h.is_none(),
        },
        true,
        planned,
    );
    // Capture this completed body's witness before any later child can
    // publish another subframe result. None remains a legacy fallback.
    let completed_body_baseline = if baseline_body_is_certified(&e.body) {
        layouter.last_sub_frame_baseline
    } else {
        None
    };
    layouter.parent_bbox = saved_bbox;
    let natural_w = super::helpers::line_content_right(&body, &layouter.metrics)
        .max(layouter.last_sub_frame_width);
    let box_w = explicit_w.unwrap_or((inset.left + natural_w + inset.right).max(0.0));
    let box_h = explicit_h.unwrap_or((inset.top + body_h + inset.bottom).max(0.0));
    let final_inner_w = (box_w - inset.left - inset.right).max(0.0);
    let final_inner_h = (box_h - inset.top - inset.bottom).max(0.0);

    // Deferred paths refer to this body, not to the parent's pending line.
    let saved_x = std::mem::replace(&mut layouter.pending_align_centering, align_x);
    layouter.apply_pending_align_fixups(
        &mut body,
        final_inner_w + layouter.page_config.margin.horizontal(),
    );
    layouter.pending_align_centering = saved_x;
    let saved_y = std::mem::replace(&mut layouter.pending_align_v_centering, align_y);
    layouter.apply_pending_align_v_fixups(
        &mut body,
        final_inner_h + layouter.page_config.margin.bottom,
    );
    layouter.pending_align_v_centering = saved_y;

    let natural_baseline = match completed_body_baseline {
        Some(Some(y)) => inset.top + y,
        Some(None) => box_h,
        None => first_baseline(&body).map_or(box_h, |p| inset.top + p.y.0),
    };
    let baseline = natural_baseline - e.baseline.resolve_pt(font);
    if layouter.initial_baseline_pending {
        // A first box installs its own ascent; zero is a valid installed value.
        let ascent = baseline.max(0.0);
        start.y += Pt(ascent);
        layouter.regions.current.cursor_y = start.y;
        layouter.line_assumed_ascent = Some(ascent);
        layouter.initial_baseline_pending = false;
    }
    let pos = Point { x: start.x, y: start.y - Pt(baseline) };
    let paint_w = (box_w + outset.left + outset.right).max(0.0);
    let paint_h = (box_h + outset.top + outset.bottom).max(0.0);
    let half_stroke = e.stroke.as_ref().map_or(0.0, |s| s.thickness.max(0.0) / 2.0);
    let kind = if e.radius == crate::entities::corners::Corners::uniform(Length::ZERO) {
        ShapeKind::Rect
    } else {
        super::resolve_shape_kind(
            &ShapeKind::RoundedRect { radii: e.radius },
            layouter.style.size,
        )
    };
    // The requested radius describes the outside of the uniform border.
    let kind = match kind {
        ShapeKind::RoundedRect { radii } => {
            let center = |r: Pt| {
                Pt((r.0.min(paint_w.min(paint_h) / 2.0 + half_stroke) - half_stroke)
                    .max(0.0))
            };
            ShapeKind::RoundedRect {
                radii: crate::entities::corners::Corners::new(
                    center(radii.top_left),
                    center(radii.top_right),
                    center(radii.bottom_right),
                    center(radii.bottom_left),
                ),
            }
        }
        other => other,
    };
    let mut contents = Vec::new();
    if e.fill.is_some() || e.stroke.is_some() {
        let prepared_stroke = super::tiling::resolve_stroke(layouter, e.stroke.as_ref());
        contents.push(FrameItem::Shape {
            pos: Point { x: Pt(-outset.left), y: Pt(-outset.top) },
            kind: kind.clone(),
            width: paint_w,
            height: paint_h,
            fill: e.fill.clone().map(crate::entities::frame_paint::FramePaint::Solid),
            stroke: prepared_stroke,
            fill_rule: Default::default(),
            parent_bbox_at_emit: saved_bbox.map(|r| Rect {
                x: r.x - pos.x,
                y: r.y - pos.y,
                ..r
            }),
        });
    }
    let body_start = contents.len();
    let body_pos = Point { x: Pt(inset.left), y: Pt(inset.top) };
    let body_frame = FrameItem::Group {
        paint_root: false,
        pos: body_pos,
        matrix: TransformMatrix::identity(),
        clip_mask: None,
        inner_width: final_inner_w,
        inner_height: final_inner_h,
        items: body,
    };
    if e.clip {
        // Clip at the inner edge so the body cannot paint over the border.
        let clip_kind = match kind {
            ShapeKind::RoundedRect { radii } => {
                let inner = |r: Pt| Pt((r.0 - half_stroke).max(0.0));
                ShapeKind::RoundedRect {
                    radii: crate::entities::corners::Corners::new(
                        inner(radii.top_left),
                        inner(radii.top_right),
                        inner(radii.bottom_right),
                        inner(radii.bottom_left),
                    ),
                }
            }
            other => other,
        };
        let mut child = body_frame;
        super::helpers::offset_frame_item(
            &mut child,
            outset.left - half_stroke,
            outset.top - half_stroke,
        );
        contents.push(FrameItem::Group {
            paint_root: false,
            pos: Point {
                x: Pt(half_stroke - outset.left),
                y: Pt(half_stroke - outset.top),
            },
            matrix: TransformMatrix::identity(),
            clip_mask: Some(clip_kind),
            inner_width: (paint_w - 2.0 * half_stroke).max(0.0),
            inner_height: (paint_h - 2.0 * half_stroke).max(0.0),
            items: vec![child],
        });
    } else {
        contents.push(body_frame);
    }
    // Only the descendants use the new root. The backing is painted in the
    // parent's reference space, before the container establishes its own.
    let body_items = contents.split_off(body_start);
    contents.push(FrameItem::Group {
        paint_root: true,
        pos: Point::ZERO,
        matrix: TransformMatrix::identity(),
        clip_mask: None,
        inner_width: box_w,
        inner_height: box_h,
        items: body_items,
    });
    if let Some(collector) = layouter.decoration_lines_collector.as_mut() {
        collector.extend(deco.into_iter().map(|seg| super::DecoSegment {
            start_x: seg.start_x + pos.x + Pt(inset.left),
            end_x: seg.end_x + pos.x + Pt(inset.left),
            baseline_y: seg.baseline_y + pos.y + Pt(inset.top),
            piece_line: seg.piece_line,
        }));
    }
    layouter.regions.current.current_line.push(FrameItem::Group {
        paint_root: false,
        pos,
        matrix: TransformMatrix::identity(),
        clip_mask: None,
        inner_width: box_w,
        inner_height: box_h,
        items: contents,
    });
    layouter.regions.current.cursor_x = start.x + Pt(box_w);
    layouter.note_inline_extent(baseline.max(0.0), (box_h - baseline).max(0.0));
    layouter.text_tracking_state.interrupt();
}

fn fractional_text_edges<M: FontMetrics>(items: &[FrameItem], metrics: &M) -> (f64, f64) {
    let mut top = 0.0_f64;
    let mut bottom = 0.0_f64;
    for item in items {
        let edges = match item {
            FrameItem::Text { style, .. } | FrameItem::TextShaped { style, .. } => {
                metrics.text_edges(style.size, style)
            }
            FrameItem::Link { items, .. }
            | FrameItem::Semantic { items, .. }
            | FrameItem::TextReplacement { items, .. } => {
                let (t, b) = fractional_text_edges(items, metrics);
                (Pt(t), Pt(b))
            }
            FrameItem::TextSeparator { .. }
            | FrameItem::Line { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Image { .. }
            | FrameItem::Shape { .. }
            | FrameItem::Group { .. } => (Pt(0.0), Pt(0.0)),
        };
        top = top.max(edges.0 .0);
        bottom = bottom.min(edges.1 .0);
    }
    (top, bottom)
}

/// A complete body proof is necessary: a later observed line cannot certify
/// an earlier opaque frame. This gate never dispatches or measures content.
fn baseline_body_is_certified(content: &crate::entities::content::Content) -> bool {
    use crate::entities::content::Content;
    match content {
        Content::Source(v) => baseline_body_is_certified(&v.body),
        Content::ShowRecipe(v) => baseline_body_is_certified(&v.body),
        Content::Located(v) => baseline_body_is_certified(&v.body),
        Content::Realized(v) => baseline_body_is_certified(&v.visual),
        Content::Styled(v) => baseline_body_is_certified(&v.body),
        Content::Strong(v) => baseline_body_is_certified(&v.body),
        Content::Emph(v) => baseline_body_is_certified(&v.body),
        Content::Label(v) => baseline_body_is_certified(&v.body),
        Content::Sequence(parts) => parts.iter().all(baseline_body_is_certified),
        Content::Par { body } => baseline_body_is_certified(body),
        Content::Boxed(v) => baseline_body_is_certified(&v.body),
        Content::Block(v) => baseline_body_is_certified(&v.body),
        Content::Repeat(v) => baseline_body_is_certified(&v.body),
        Content::Empty
        | Content::Text(_)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Linebreak(_)
        | Content::HSpace(_)
        | Content::VSpace(_)
        | Content::Shape(_)
        | Content::Metadata(_)
        | Content::State(_)
        | Content::StateUpdate(_)
        | Content::CounterUpdate(_)
        | Content::ContextBlock(_)
        | Content::Document { .. }
        | Content::Asset { .. } => true,
        Content::HtmlElem(_)
        | Content::HtmlFrame(_)
        | Content::Heading(_)
        | Content::Title(_)
        | Content::Raw(_)
        | Content::ListItem(_)
        | Content::EnumItem(_)
        | Content::Link(_)
        | Content::Equation(_)
        | Content::MathSequence(_)
        | Content::Symbol(_)
        | Content::MathIdent(_)
        | Content::MathText(_)
        | Content::MathFrac(_)
        | Content::MathAttach(_)
        | Content::MathPrimes(_)
        | Content::MathRoot(_)
        | Content::MathStretch(_)
        | Content::MathDelimited(_)
        | Content::MathLr(_)
        | Content::MathMid(_)
        | Content::MathOverbrace(_)
        | Content::MathUnderbrace(_)
        | Content::MathUnderbracket(_)
        | Content::MathUnderparen(_)
        | Content::MathUndershell(_)
        | Content::MathOverbracket(_)
        | Content::MathOverparen(_)
        | Content::MathOvershell(_)
        | Content::MathAlignPoint(_)
        | Content::MathMatrix(_)
        | Content::MathCases(_)
        | Content::MathAccent(_)
        | Content::MathCancel(_)
        | Content::MathUnderline(_)
        | Content::MathOverline(_)
        | Content::MathVec(_)
        | Content::MathClassOverride(_)
        | Content::MathLimitsOverride(_)
        | Content::MathUnderover(_)
        | Content::MathOp(_)
        | Content::MathStyled(_)
        | Content::Ref(_)
        | Content::CounterDisplay(_)
        | Content::Outline(_)
        | Content::OutlineEntry(_)
        | Content::Figure(_)
        | Content::FigureCaption(_)
        | Content::Image(_)
        | Content::Curve(_)
        | Content::Transform(_)
        | Content::Grid(_)
        | Content::GridHeader(_)
        | Content::GridFooter(_)
        | Content::GridCell(_)
        | Content::SetPage(_)
        | Content::PageRun(_)
        | Content::Align(_)
        | Content::Place(_)
        | Content::Flush(_)
        | Content::Divider(_)
        | Content::Terms(_)
        | Content::TermItem(_)
        | Content::Quote(_)
        | Content::SmartQuote(_)
        | Content::PdfAttach(_)
        | Content::PdfArtifact(_)
        | Content::Underline(_)
        | Content::Strike(_)
        | Content::Overline(_)
        | Content::SmallCaps { .. }
        | Content::Pad(_)
        | Content::Hide(_)
        | Content::Pagebreak(_)
        | Content::Colbreak(_)
        | Content::Stack(_)
        | Content::TableCell(_)
        | Content::Bibliography(_)
        | Content::Cite(_)
        | Content::Footnote(_)
        | Content::FootnoteEntry(_)
        | Content::TableHeader(_)
        | Content::TableFooter(_)
        | Content::GridHLine(_)
        | Content::GridVLine(_)
        | Content::TableHLine(_)
        | Content::TableVLine(_)
        | Content::Table(_)
        | Content::Columns(_)
        | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_)
        | Content::Dynamic(_) => false,
    }
}

/// First textual baseline in frame coordinates; wrappers do not erase it.
fn first_baseline(items: &[FrameItem]) -> Option<crate::entities::layout_types::Point> {
    use crate::entities::layout_types::Point;
    items.iter().find_map(|item| match item {
        FrameItem::Text { pos, .. }
        | FrameItem::TextShaped { pos, .. }
        | FrameItem::Glyph { pos, .. } => Some(*pos),
        FrameItem::Group { pos, matrix, items, .. } => first_baseline(items).map(|p| {
            let (x, y) = matrix.apply(p.x.0, p.y.0);
            Point { x: pos.x + Pt(x), y: pos.y + Pt(y) }
        }),
        FrameItem::Link { items, .. }
        | FrameItem::Semantic { items, .. }
        | FrameItem::TextReplacement { items, .. } => first_baseline(items),
        FrameItem::TextSeparator { .. }
        | FrameItem::Line { .. }
        | FrameItem::Image { .. }
        | FrameItem::Shape { .. } => None,
    })
}

/// Translate owned items when normalizing line coordinates to a group origin.
fn offset_items(items: &mut [FrameItem], dx: f64, dy: f64) {
    for item in items {
        match item {
            FrameItem::TextSeparator { pos, .. }
            | FrameItem::Text { pos, .. }
            | FrameItem::TextShaped { pos, .. }
            | FrameItem::Glyph { pos, .. }
            | FrameItem::Shape { pos, .. }
            | FrameItem::Image { pos, .. }
            | FrameItem::Group { pos, .. } => {
                pos.x = Pt(pos.x.0 + dx);
                pos.y = Pt(pos.y.0 + dy);
            }
            FrameItem::Line { start, end, .. } => {
                start.x = Pt(start.x.0 + dx);
                start.y = Pt(start.y.0 + dy);
                end.x = Pt(end.x.0 + dx);
                end.y = Pt(end.y.0 + dy);
            }
            FrameItem::Link { pos, items, .. } => {
                pos.x = Pt(pos.x.0 + dx);
                pos.y = Pt(pos.y.0 + dy);
                offset_items(items, dx, dy);
            }
            FrameItem::TextReplacement { items, .. }
            | FrameItem::Semantic { items, .. } => offset_items(items, dx, dy),
        }
    }
}

fn frame_items_have_text_baseline(items: &[FrameItem]) -> bool {
    items.iter().any(|item| match item {
        FrameItem::Text { .. }
        | FrameItem::TextShaped { .. }
        | FrameItem::Glyph { .. } => true,
        FrameItem::Group { items, .. }
        | FrameItem::TextReplacement { items, .. }
        | FrameItem::Semantic { items, .. } => frame_items_have_text_baseline(items),
        FrameItem::TextSeparator { .. }
        | FrameItem::Line { .. }
        | FrameItem::Image { .. }
        | FrameItem::Shape { .. }
        | FrameItem::Link { .. } => false,
    })
}

#[cfg(test)]
mod composition_regressions {
    use super::*;
    use crate::entities::{
        content::Content,
        corners::Corners,
        elements::shape::ShapeElem,
        geometry::ShapeKind,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{
            Color, Length, PageMarginSpec, PageMargins, Point, TransformMatrix,
        },
        paint::Paint,
        sides::Sides,
        value::Value,
    };
    use comemo::Track;
    use std::sync::Arc;

    fn box_element(body: Content) -> BoxedElem {
        BoxedElem {
            body,
            width: Some(Length::pt(20.0)),
            width_fraction: None,
            height: Some(Length::pt(14.0)),
            inset: Sides::uniform(Length::ZERO),
            baseline: Length::ZERO,
            outset: Sides::uniform(Length::ZERO),
            radius: Corners::uniform(Length::ZERO),
            clip: false,
            fill: None,
            stroke: None,
        }
    }
    fn shape(width: f64, height: f64) -> Content {
        Content::Shape(Arc::new(ShapeElem {
            origin: crate::entities::elements::shape::ShapeOrigin::Geometry,
            kind: ShapeKind::Rect,
            width: Some(Box::new(Value::Length(Length::pt(width)))),
            height: Some(Box::new(Value::Length(Length::pt(height)))),
            fill: Some(Paint::Solid(Color::rgb(255, 0, 0))),
            stroke: None,
            fill_rule: Default::default(),
        }))
    }
    #[derive(Default, Debug)]
    struct Geometry {
        text: Vec<(String, Point)>,
        shapes: Vec<(Point, f64, f64)>,
        clips: Vec<(Point, f64, f64, ShapeKind<Pt>)>,
    }
    fn inspect(items: &[FrameItem], matrix: TransformMatrix, g: &mut Geometry) {
        let point = |p: Point| {
            let (x, y) = matrix.apply(p.x.0, p.y.0);
            Point { x: Pt(x), y: Pt(y) }
        };
        for item in items {
            match item {
                FrameItem::Text { text, pos, .. }
                | FrameItem::TextShaped { text, pos, .. } => {
                    g.text.push((text.to_string(), point(*pos)))
                }
                FrameItem::Shape { pos, width, height, .. } => {
                    g.shapes.push((point(*pos), *width, *height))
                }
                FrameItem::Group {
                    paint_root: _,
                    pos,
                    matrix: local,
                    clip_mask,
                    inner_width,
                    inner_height,
                    items,
                } => {
                    let m = matrix
                        .concat(&TransformMatrix::translate(pos.x.0, pos.y.0))
                        .concat(local);
                    if let Some(kind) = clip_mask {
                        let (x, y) = m.apply(0.0, 0.0);
                        g.clips.push((
                            Point { x: Pt(x), y: Pt(y) },
                            *inner_width,
                            *inner_height,
                            kind.clone(),
                        ));
                    }
                    inspect(items, m, g);
                }
                FrameItem::Semantic { items, .. }
                | FrameItem::TextReplacement { items, .. }
                | FrameItem::Link { items, .. } => inspect(items, matrix, g),
                FrameItem::TextSeparator { .. }
                | FrameItem::Line { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. } => {}
            }
        }
    }
    fn render(e: BoxedElem) -> Geometry {
        render_content(Content::sequence(vec![
            Content::text("P"),
            Content::Boxed(Arc::new(e)),
            Content::text("Q"),
        ]))
    }
    fn render_content(content: Content) -> Geometry {
        let doc = compose_document(content);
        assert_eq!(doc.pages.len(), 1);
        let mut g = Geometry::default();
        inspect(&doc.pages[0].items, TransformMatrix::identity(), &mut g);
        g
    }
    fn compose_document(
        content: Content,
    ) -> crate::entities::layout_types::PagedDocument {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = 120.0;
        l.page_config.height = 100.0;
        l.page_config.margin = PageMargins::uniform(10.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(10.0);
        l.regions.current.width = 120.0;
        l.regions.current.height = 100.0;
        l.regions.current.cursor_x = Pt(10.0);
        l.regions.current.line_start_x = Pt(10.0);
        l.regions.current.cursor_y = Pt(10.0);
        l.layout_content(&content);
        l.finish()
    }
    fn point(g: &Geometry, text: &str) -> Point {
        g.text.iter().find(|(s, _)| s == text).unwrap().1
    }
    fn close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-8, "{a} != {b}");
    }

    #[test]
    fn explicit_dimensions_include_inset_and_exclude_outset() {
        let mut base = box_element(Content::Empty);
        base.fill = Some(Color::rgb(255, 0, 0));
        let plain = render(base.clone());
        base.inset = Sides::uniform(Length::pt(2.0));
        base.outset = Sides::uniform(Length::pt(3.0));
        let padded = render(base);
        assert_eq!(point(&padded, "Q"), point(&plain, "Q"));
        assert_eq!(point(&padded, "P"), point(&plain, "P"));
        assert_eq!(padded.shapes.len(), 1);
        close(padded.shapes[0].1, 26.0);
        close(padded.shapes[0].2, 20.0);
        close(padded.shapes[0].0.x.0, plain.shapes[0].0.x.0 - 3.0);
        close(padded.shapes[0].0.y.0, plain.shapes[0].0.y.0 - 3.0);
    }
    #[test]
    fn clipped_shape_is_local_and_preserves_the_outer_baseline() {
        let mut e = box_element(shape(40.0, 24.0));
        e.clip = true;
        let g = render(e);
        assert_eq!(g.shapes.len(), 1);
        assert_eq!(g.clips.len(), 1);
        close(point(&g, "P").y.0, point(&g, "Q").y.0);
        let (pos, w, h, _) = &g.clips[0];
        close(*w, 20.0);
        close(*h, 14.0);
        close(pos.x.0, 16.6);
        close(pos.y.0, 10.0);
        close(g.shapes[0].0.x.0, pos.x.0);
        close(g.shapes[0].0.y.0, pos.y.0);
    }
    #[test]
    fn clip_uses_horizontal_bounds_and_auto_height() {
        let mut e = box_element(shape(40.0, 8.0));
        e.clip = true;
        e.height = None;
        let g = render(e);
        assert_eq!(g.clips.len(), 1);
        close(g.clips[0].1, 20.0);
        close(g.clips[0].2, 8.0);
        close(point(&g, "P").y.0, point(&g, "Q").y.0);
    }
    #[test]
    fn inset_and_radius_apply_to_the_body_clip() {
        let mut e = box_element(shape(40.0, 24.0));
        e.clip = true;
        e.inset = Sides::uniform(Length::pt(2.0));
        e.radius = Corners::uniform(Length::pt(4.0));
        let g = render(e);
        assert_eq!(g.clips.len(), 1);
        assert!(matches!(g.clips[0].3, ShapeKind::RoundedRect { .. }));
        close(g.clips[0].1, 20.0);
        close(g.clips[0].2, 14.0);
        close(g.shapes[0].0.x.0, g.clips[0].0.x.0 + 2.0);
        close(g.shapes[0].0.y.0, g.clips[0].0.y.0 + 2.0);
    }
    #[test]
    fn no_clip_keeps_overflow_visible_and_inline() {
        let g = render(box_element(shape(40.0, 24.0)));
        assert!(g.clips.is_empty());
        assert_eq!(g.shapes.len(), 1);
        close(g.shapes[0].1, 40.0);
        close(g.shapes[0].2, 24.0);
        close(point(&g, "P").y.0, point(&g, "Q").y.0);
    }
    #[test]
    fn nested_box_has_no_phantom_font_descent() {
        let mut inner = box_element(shape(20.0, 30.0));
        inner.height = Some(Length::pt(30.0));
        let mut outer = box_element(Content::Boxed(Arc::new(inner)));
        outer.width = None;
        outer.height = None;
        outer.clip = true;
        let g = render(outer);
        assert_eq!(g.clips.len(), 1);
        close(g.clips[0].1, 20.0);
        close(g.clips[0].2, 30.0);
        close(g.shapes[0].0.x.0, g.clips[0].0.x.0);
        close(g.shapes[0].0.y.0, g.clips[0].0.y.0);
    }
    #[test]
    fn stroke_clip_preserves_the_inner_half_of_the_border() {
        for outset in [0.0, 3.0] {
            for radius in [0.0, 4.0] {
                let mut e = box_element(shape(40.0, 24.0));
                e.clip = true;
                e.outset = Sides::uniform(Length::pt(outset));
                e.radius = Corners::uniform(Length::pt(radius));
                e.stroke = Some(crate::entities::geometry::Stroke {
                    thickness: 2.0,
                    ..Default::default()
                });
                let g = render(e);
                assert_eq!(g.clips.len(), 1);
                close(g.clips[0].0.x.0, 16.6 - outset + 1.0);
                close(g.clips[0].0.y.0, 10.0 - outset + 1.0);
                close(g.clips[0].1, 18.0 + 2.0 * outset);
                close(g.clips[0].2, 12.0 + 2.0 * outset);
                if radius > 0.0 {
                    let ShapeKind::RoundedRect { radii } = &g.clips[0].3 else {
                        panic!("rounded clip")
                    };
                    assert_eq!(*radii, Corners::uniform(Pt(2.0)));
                }
                // The body origin stays at the box origin even as the clip moves.
                close(g.shapes[1].0.x.0, 16.6);
                close(g.shapes[1].0.y.0, 10.0);
            }
        }
    }

    #[test]
    fn constrained_measurement_preserves_explicit_box_dimensions() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        for body in [Content::Empty, shape(40.0, 24.0)] {
            let mut e = box_element(body);
            e.inset = Sides::uniform(Length::pt(2.0));
            e.outset = Sides::uniform(Length::pt(3.0));
            let content = Content::Boxed(Arc::new(e));
            for available in [100.0, 10.0] {
                assert_eq!(
                    l.measure_content_constrained(&content, available),
                    (20.0, 14.0)
                );
            }
        }
    }

    #[test]
    fn finite_subregion_owns_its_dimensions_even_inside_auto_page() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.margin = PageMargins::uniform(15.0);
        l.regions.current.line_start_x = Pt(3.0);
        l.regions.current.width = 58.0;
        l.regions.current.height = 30.0;
        l.is_sub_frame = true;
        for page in [100.0, f64::INFINITY] {
            l.page_config.width = page;
            l.page_config.height = page;
            close(l.available_width(), 40.0);
            close(l.available_height(), 30.0);
            close(l.page_bottom_limit(), 30.0);
        }
        l.is_sub_frame = false;
        l.page_config.width = 100.0;
        l.page_config.height = 100.0;
        close(l.available_width(), 28.0);
        close(l.available_height(), 0.0);
        close(l.page_bottom_limit(), 15.0);
    }
    #[test]
    fn initial_box_without_text_starts_at_the_region_top() {
        let mut e = box_element(shape(14.0, 10.0));
        e.height = Some(Length::pt(6.0));
        let g = render_content(Content::Boxed(Arc::new(e)));
        close(g.shapes[0].0.y.0, 10.0);
    }

    #[test]
    fn initial_box_followed_by_text_retains_the_text_baseline() {
        let control = render_content(Content::text("Q"));
        for height in [0.0, 6.0] {
            let mut e = box_element(shape(14.0, 10.0));
            e.height = Some(Length::pt(height));
            let g = render_content(Content::sequence(vec![
                Content::Boxed(Arc::new(e)),
                Content::text("Q"),
            ]));
            close(point(&g, "Q").y.0, point(&control, "Q").y.0);
            close(g.shapes[0].0.y.0, point(&g, "Q").y.0 - height);
        }
    }

    #[test]
    fn initial_boxes_share_their_own_maximum_ascent() {
        let mut first = box_element(shape(14.0, 10.0));
        first.height = Some(Length::pt(6.0));
        let mut second = first.clone();
        second.height = Some(Length::pt(4.0));
        let g = render_content(Content::sequence(vec![
            Content::Boxed(Arc::new(first)),
            Content::Boxed(Arc::new(second)),
        ]));
        close(g.shapes[0].0.y.0, 10.0);
        close(g.shapes[1].0.y.0, 12.0);
    }

    #[test]
    fn initial_negative_box_baseline_installs_zero_ascent() {
        let mut e = box_element(shape(14.0, 10.0));
        e.height = Some(Length::pt(6.0));
        e.baseline = Length::pt(10.0);
        let g = render_content(Content::Boxed(Arc::new(e.clone())));
        close(g.shapes[0].0.y.0, 14.0);
        let control = render_content(Content::text("Q"));
        let mixed = render_content(Content::sequence(vec![
            Content::Boxed(Arc::new(e)),
            Content::text("Q"),
        ]));
        close(point(&mixed, "Q").y.0, point(&control, "Q").y.0);
        close(mixed.shapes[0].0.y.0, point(&mixed, "Q").y.0 + 4.0);
    }

    #[test]
    fn initial_nested_box_has_no_extra_font_ascent() {
        let mut inner = box_element(shape(14.0, 10.0));
        inner.height = Some(Length::pt(6.0));
        let mut outer = box_element(Content::Boxed(Arc::new(inner)));
        outer.height = None;
        outer.clip = true;
        let g = render_content(Content::Boxed(Arc::new(outer)));
        close(g.shapes[0].0.y.0, 10.0);
        close(g.clips[0].0.y.0, 10.0);
        close(g.clips[0].2, 6.0);
    }

    #[test]
    fn initial_zero_box_and_text_inside_subframe_use_text_ascent() {
        let mut inner = box_element(shape(14.0, 10.0));
        inner.height = Some(Length::pt(0.0));
        let mut outer = box_element(Content::sequence(vec![
            Content::Boxed(Arc::new(inner)),
            Content::text("Q"),
        ]));
        outer.width = Some(Length::pt(40.0));
        outer.height = None;
        let g = render_content(Content::Boxed(Arc::new(outer)));
        let control = render_content(Content::text("Q"));
        close(point(&g, "Q").y.0, point(&control, "Q").y.0);
        close(g.shapes[0].0.y.0, point(&g, "Q").y.0);
    }
    #[test]
    fn initial_box_uses_the_actual_smaller_text_ascent() {
        use crate::entities::style::{Style, Styles};
        let small = Content::styled(
            Box::new(Content::text("Q")),
            Styles::from_iter([Style::Size(crate::entities::layout_types::Length::pt(
                10.0,
            ))]),
        );
        let control = render_content(small.clone());
        let mut e = box_element(small);
        e.height = None;
        let g = render_content(Content::Boxed(Arc::new(e)));
        close(point(&g, "Q").y.0, point(&control, "Q").y.0);
    }
    fn located(raw: u128, body: Content) -> Content {
        Content::Located(Arc::new(crate::entities::content::LocatedContent {
            location: crate::entities::location::Location::from_raw(raw),
            body,
        }))
    }
    fn published_point(
        doc: &crate::entities::layout_types::PagedDocument,
        raw: u128,
    ) -> Point {
        let pos = doc
            .extracted_positions
            .position_of(crate::entities::location::Location::from_raw(raw))
            .expect("captured Location must be published");
        assert_eq!(pos.page.get(), 1);
        pos.point
    }
    fn no_private_positions(items: &[FrameItem]) {
        for item in items {
            match item {
                FrameItem::Semantic { alt, items, .. } => {
                    assert!(!alt
                        .as_deref()
                        .is_some_and(|s| s.starts_with("\0tekt-position:")));
                    no_private_positions(items);
                }
                FrameItem::Group { items, .. }
                | FrameItem::Link { items, .. }
                | FrameItem::TextReplacement { items, .. } => no_private_positions(items),
                FrameItem::Text { .. }
                | FrameItem::TextShaped { .. }
                | FrameItem::TextSeparator { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Line { .. }
                | FrameItem::Shape { .. }
                | FrameItem::Image { .. } => {}
            }
        }
    }
    #[test]
    fn published_box_position_includes_inset_and_page_origin() {
        let mut e = box_element(located(9001, Content::text("A")));
        e.inset = Sides::uniform(Length::pt(2.0));
        let doc = compose_document(Content::Boxed(Arc::new(e)));
        let p = published_point(&doc, 9001);
        close(p.x.0, 12.0);
        close(p.y.0, 12.0);
        no_private_positions(&doc.pages[0].items);
    }
    #[test]
    fn published_box_position_preserves_parent_and_suffix_identity() {
        let e = box_element(located(9002, Content::text("A")));
        let doc = compose_document(Content::sequence(vec![
            located(9001, Content::text("P")),
            Content::Boxed(Arc::new(e)),
            located(9003, Content::text("Q")),
        ]));
        assert_eq!(doc.extracted_positions.len(), 3);
        close(published_point(&doc, 9001).x.0, 10.0);
        close(published_point(&doc, 9002).x.0, 16.6);
        close(published_point(&doc, 9002).y.0, 10.0);
        close(published_point(&doc, 9003).x.0, 36.6);
        no_private_positions(&doc.pages[0].items);
    }
    #[test]
    fn published_box_position_composes_nested_origins_once() {
        let mut inner = box_element(located(9001, Content::text("A")));
        inner.inset = Sides::uniform(Length::pt(2.0));
        let mut outer = box_element(Content::Boxed(Arc::new(inner)));
        outer.inset = Sides::uniform(Length::pt(3.0));
        let doc = compose_document(Content::Boxed(Arc::new(outer)));
        let p = published_point(&doc, 9001);
        close(p.x.0, 15.0);
        close(p.y.0, 15.0);
        no_private_positions(&doc.pages[0].items);
    }
    #[test]
    fn published_box_position_does_not_add_font_ascent_to_shape() {
        let mut e = box_element(located(9001, shape(14.0, 10.0)));
        e.inset = Sides::uniform(Length::pt(2.0));
        let doc = compose_document(Content::Boxed(Arc::new(e)));
        let p = published_point(&doc, 9001);
        close(p.x.0, 12.0);
        close(p.y.0, 12.0);
        let mut g = Geometry::default();
        inspect(&doc.pages[0].items, TransformMatrix::identity(), &mut g);
        assert_eq!(g.shapes.len(), 1);
        close(g.shapes[0].0.x.0, 12.0);
        close(g.shapes[0].0.y.0, 12.0);
        no_private_positions(&doc.pages[0].items);
    }
    #[test]
    fn published_box_position_survives_empty_context_body() {
        let mut e = box_element(located(9001, Content::Empty));
        e.inset = Sides::uniform(Length::pt(2.0));
        let doc = compose_document(Content::Boxed(Arc::new(e)));
        let p = published_point(&doc, 9001);
        close(p.x.0, 12.0);
        close(p.y.0, 12.0);
        no_private_positions(&doc.pages[0].items);
    }
    #[test]
    fn published_box_position_keeps_labels_in_the_same_frame() {
        let mut e =
            box_element(Content::label("anchor", located(9001, Content::text("A"))));
        e.inset = Sides::uniform(Length::pt(2.0));
        let doc = compose_document(Content::Boxed(Arc::new(e)));
        let label = crate::entities::label::Label("anchor".into());
        let p = doc.extracted_label_positions.get(&label).expect("label position");
        close(p.x.0, 12.0);
        close(p.y.0, 12.0);
        assert_eq!(doc.extracted_label_pages.get(&label), Some(&1));
        let context = published_point(&doc, 9001);
        close(context.x.0, p.x.0);
        close(context.y.0, p.y.0);
        no_private_positions(&doc.pages[0].items);
    }

    fn boundary_box(body: Content) -> BoxedElem {
        let mut e = box_element(body);
        e.width = Some(Length::pt(8.0));
        e.height = Some(Length::pt(7.0));
        e.fill = Some(Color::rgb(255, 0, 0));
        e
    }
    fn boundary_content(prefix: Vec<Content>, e: BoxedElem, suffix: Content) -> Content {
        Content::sequence(
            prefix
                .into_iter()
                .chain([Content::Boxed(Arc::new(e)), suffix])
                .collect(),
        )
    }
    fn finite_document(
        content: Content,
        height: f64,
    ) -> crate::entities::layout_types::PagedDocument {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = 120.0;
        l.page_config.height = height;
        l.page_config.margin = PageMargins::uniform(10.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(10.0);
        l.regions.current.width = 120.0;
        l.regions.current.height = height;
        l.regions.current.cursor_x = Pt(10.0);
        l.regions.current.line_start_x = Pt(10.0);
        l.regions.current.cursor_y = Pt(10.0);
        l.layout_content(&content);
        l.finish()
    }
    fn page_geometry(
        doc: &crate::entities::layout_types::PagedDocument,
        page: usize,
    ) -> Geometry {
        let mut g = Geometry::default();
        inspect(&doc.pages[page].items, TransformMatrix::identity(), &mut g);
        g
    }
    #[test]
    fn par_box_empty_prefix_uses_actual_box_ascent() {
        let g = render_content(boundary_content(
            vec![Content::par(Content::Empty)],
            boundary_box(Content::Empty),
            Content::Empty,
        ));
        close(g.shapes[0].0.y.0, 23.2);
    }
    #[test]
    fn par_box_suffix_text_settles_whole_line() {
        let g = render_content(boundary_content(
            vec![Content::par(Content::Empty)],
            boundary_box(Content::Empty),
            Content::text("Q"),
        ));
        close(point(&g, "Q").y.0, 30.9);
        close(g.shapes[0].0.y.0, 23.9);
    }
    #[test]
    fn par_box_previous_text_and_empty_par_have_two_collapsed_gaps() {
        let g = render_content(boundary_content(
            vec![Content::text("A"), Content::par(Content::Empty)],
            boundary_box(Content::Empty),
            Content::text("Q"),
        ));
        close(point(&g, "A").y.0, 17.7);
        close(point(&g, "Q").y.0, 51.8);
        close(g.shapes[0].0.y.0, 44.8);
    }
    #[test]
    fn par_box_explicit_spacing_survives_lexical_restore() {
        use crate::entities::style::Styles;
        let par = Content::styled(
            Content::par(Content::Empty),
            Styles::new().push_custom("par.spacing", Value::Length(Length::pt(30.0))),
        );
        let g = render_content(boundary_content(
            vec![Content::text("A"), par],
            boundary_box(Content::Empty),
            Content::text("Q"),
        ));
        close(point(&g, "Q").y.0, 85.4);
    }
    #[test]
    fn par_box_empty_par_preserves_scoped_font_spacing() {
        use crate::entities::style::{Style, Styles};
        for (size, baseline) in [(8.0, 51.8), (18.0, 68.6)] {
            let par = Content::styled(
                Content::par(Content::Empty),
                Styles::from_iter([Style::Size(
                    crate::entities::layout_types::Length::pt(size),
                )]),
            );
            let g = render_content(boundary_content(
                vec![Content::text("A"), par],
                boundary_box(Content::Empty),
                Content::text("Q"),
            ));
            close(point(&g, "Q").y.0, baseline);
        }
    }
    #[test]
    fn par_box_strong_below_has_no_font_strut() {
        for below in [0.0, 20.0] {
            let mut block = Content::block(
                Content::Empty,
                None,
                Some(Length::pt(8.0)),
                Sides::uniform(Length::ZERO),
                false,
            );
            if let Content::Block(e) = &mut block {
                Arc::make_mut(e).below = Some(Length::pt(below));
            }
            let g = render_content(boundary_content(
                vec![block],
                boundary_box(Content::Empty),
                Content::Empty,
            ));
            close(g.shapes[0].0.y.0, 18.0 + below);
        }
    }
    #[test]
    fn par_box_shifted_baseline_keeps_frame_and_suffix_together() {
        let mut e = boundary_box(Content::Empty);
        e.baseline = Length::pt(10.0);
        let g = render_content(boundary_content(
            vec![Content::par(Content::Empty)],
            e,
            Content::text("Q"),
        ));
        close(point(&g, "Q").y.0, 30.9);
        close(g.shapes[0].0.y.0, 33.9);
    }
    #[test]
    fn par_box_two_empty_units_keep_internal_gap() {
        let g = render_content(boundary_content(
            vec![Content::par(Content::Empty), Content::par(Content::Empty)],
            boundary_box(Content::Empty),
            Content::text("Q"),
        ));
        close(point(&g, "Q").y.0, 44.1);
    }
    #[test]
    fn par_box_zero_height_installs_zero_before_text_settles() {
        let mut e = boundary_box(Content::Empty);
        e.height = Some(Length::ZERO);
        let g = render_content(boundary_content(
            vec![Content::par(Content::Empty)],
            e,
            Content::text("Q"),
        ));
        close(point(&g, "Q").y.0, 30.9);
    }
    #[test]
    fn par_box_finite_late_ascent_moves_entire_uncommitted_line() {
        let doc = finite_document(
            boundary_content(
                vec![Content::text("A"), Content::par(Content::Empty)],
                boundary_box(Content::Empty),
                Content::text("Q"),
            ),
            61.5,
        );
        assert_eq!(doc.pages.len(), 2);
        let first = page_geometry(&doc, 0);
        let second = page_geometry(&doc, 1);
        assert_eq!(
            first
                .text
                .iter()
                .filter(|(text, _)| !text.is_empty())
                .map(|(text, _)| text.as_str())
                .collect::<Vec<_>>(),
            ["A"],
            "first={first:?}, second={second:?}"
        );
        assert!(first.shapes.is_empty());
        close(point(&second, "Q").y.0, 17.7);
        close(point(&second, "Q").x.0, 18.0);
        close(second.shapes[0].0.y.0, 10.7);
    }
    #[test]
    fn par_box_fitting_final_line_does_not_create_blank_page() {
        let doc = finite_document(
            boundary_content(
                vec![Content::text("A"), Content::par(Content::Empty)],
                boundary_box(Content::Empty),
                Content::text("Q"),
            ),
            62.0,
        );
        assert_eq!(doc.pages.len(), 1);
        close(point(&page_geometry(&doc, 0), "Q").y.0, 51.8);
    }
    #[test]
    fn par_box_initial_empty_unit_can_precede_region_transition() {
        let doc = finite_document(
            boundary_content(
                vec![Content::par(Content::Empty)],
                boundary_box(Content::Empty),
                Content::text("Q"),
            ),
            30.0,
        );
        assert_eq!(doc.pages.len(), 2);
        assert!(page_geometry(&doc, 0).text.is_empty());
        close(point(&page_geometry(&doc, 1), "Q").y.0, 17.7);
    }
    #[test]
    fn par_box_first_overlong_frame_makes_progress_without_blank_page() {
        let mut e = boundary_box(Content::Empty);
        e.height = Some(Length::pt(45.0));
        let doc = finite_document(boundary_content(vec![], e, Content::text("Q")), 30.0);
        assert_eq!(
            doc.pages.len(),
            1,
            "pages={:?}",
            (0..doc.pages.len())
                .map(|p| page_geometry(&doc, p))
                .collect::<Vec<_>>()
        );
        close(page_geometry(&doc, 0).shapes[0].0.y.0, 10.0);
    }
    #[test]
    fn par_box_position_inside_migrated_frame_has_causal_page() {
        let doc = finite_document(
            boundary_content(
                vec![Content::text("A"), Content::par(Content::Empty)],
                boundary_box(located(9001, Content::Empty)),
                located(9002, Content::text("Q")),
            ),
            61.5,
        );
        assert_eq!(doc.pages.len(), 2);
        for (raw, x, y) in [(9001, 10.0, 10.7), (9002, 18.0, 17.7)] {
            let p = doc
                .extracted_positions
                .position_of(crate::entities::location::Location::from_raw(raw))
                .unwrap();
            assert_eq!(p.page.get(), 2);
            close(p.point.x.0, x);
            close(p.point.y.0, y);
        }
        for page in &doc.pages {
            no_private_positions(&page.items);
        }
    }
    #[test]
    fn par_box_inline_control_keeps_existing_baseline() {
        let g = render_content(boundary_content(
            vec![Content::text("A")],
            boundary_box(Content::Empty),
            Content::text("Q"),
        ));
        close(point(&g, "A").y.0, 17.7);
        close(point(&g, "Q").y.0, 17.7);
        close(g.shapes[0].0.y.0, 10.7);
    }

    #[test]
    fn par_box_previous_region_does_not_make_first_overlong_frame_migrate_again() {
        let mut e = boundary_box(Content::Empty);
        e.height = Some(Length::pt(45.0));
        let doc = finite_document(
            boundary_content(
                vec![Content::par(Content::Empty), Content::pagebreak(false, None)],
                e,
                Content::text("Q"),
            ),
            30.0,
        );
        assert_eq!(doc.pages.len(), 2);
        assert!(page_geometry(&doc, 0).shapes.is_empty());
        close(page_geometry(&doc, 1).shapes[0].0.y.0, 10.0);
    }

    #[test]
    fn par_box_new_region_small_box_uses_geometric_top() {
        for previous in [Content::par(Content::Empty), Content::text("A")] {
            let doc = finite_document(
                boundary_content(
                    vec![previous, Content::pagebreak(false, None)],
                    boundary_box(Content::Empty),
                    Content::Empty,
                ),
                60.0,
            );
            assert_eq!(doc.pages.len(), 2);
            close(page_geometry(&doc, 1).shapes[0].0.y.0, 10.0);
        }
    }

    #[derive(Clone, Copy)]
    struct FractionalFaceMetrics {
        top_ratio: f64,
    }
    impl FontMetrics for FractionalFaceMetrics {
        fn advance(
            &self,
            text: &str,
            size: Pt,
            style: &crate::entities::layout_types::TextStyle,
        ) -> Pt {
            super::super::FixedMetrics.advance(text, size, style)
        }
        fn vertical_metrics(
            &self,
            size: Pt,
            _: &crate::entities::layout_types::TextStyle,
        ) -> (Pt, Pt) {
            (size * self.top_ratio, size * 1.2)
        }
        fn cap_height(
            &self,
            size: Pt,
            _: &crate::entities::layout_types::TextStyle,
        ) -> Pt {
            size * self.top_ratio
        }
        fn text_edges(
            &self,
            size: Pt,
            _: &crate::entities::layout_types::TextStyle,
        ) -> (Pt, Pt) {
            (size * self.top_ratio, Pt(0.0))
        }
    }
    fn fractional_box_element(body: Content, height: f64) -> BoxedElem {
        let mut e = box_element(body);
        e.width = None;
        e.width_fraction = Some(1.0);
        e.height = Some(Length::pt(height));
        e
    }
    fn fractional_metric_document(
        content: Content,
        top_ratio: f64,
        stale_paragraph_flag: bool,
    ) -> crate::entities::layout_types::PagedDocument {
        fractional_metric_document_with_width(
            content,
            top_ratio,
            stale_paragraph_flag,
            240.0,
        )
    }
    fn fractional_metric_document_with_width(
        content: Content,
        top_ratio: f64,
        stale_paragraph_flag: bool,
        page_width: f64,
    ) -> crate::entities::layout_types::PagedDocument {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FractionalFaceMetrics { top_ratio },
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = page_width;
        l.page_config.height = f64::INFINITY;
        l.page_config.margin = PageMargins::uniform(10.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(10.0);
        l.regions.current.width = page_width;
        l.regions.current.height = f64::INFINITY;
        l.regions.current.cursor_x = Pt(10.0);
        l.regions.current.line_start_x = Pt(10.0);
        l.regions.current.cursor_y = Pt(10.0);
        l.prev_margin_is_parbreak = stale_paragraph_flag;
        l.layout_content(&content);
        l.finish()
    }
    fn fractional_paragraph_content(e: BoxedElem, spacing: f64, leading: f64) -> Content {
        use crate::entities::style::Styles;
        Content::styled(
            Content::sequence(vec![
                Content::text("BEFORE"),
                Content::Parbreak,
                Content::Boxed(Arc::new(e)),
            ]),
            Styles::new()
                .push_custom("par.spacing", Value::Length(Length::pt(spacing)))
                .push_custom("par.leading", Value::Length(Length::pt(leading))),
        )
    }
    #[test]
    fn fractional_box_paragraph_spacing_uses_actual_face_not_calibration() {
        for top_ratio in [0.3, 0.9] {
            for spacing in [0.0, 23.0] {
                for leading in [0.0, 19.0] {
                    let e = fractional_box_element(Content::text("BOX"), 16.0);
                    let doc = fractional_metric_document(
                        fractional_paragraph_content(e, spacing, leading),
                        top_ratio,
                        false,
                    );
                    assert_eq!(doc.pages.len(), 1);
                    let g = page_geometry(&doc, 0);
                    close(
                        point(&g, "BOX").y.0 - point(&g, "BEFORE").y.0,
                        spacing + 11.0 * top_ratio,
                    );
                    assert_eq!(g.text.iter().filter(|(s, _)| s == "BOX").count(), 1);
                }
            }
        }
    }
    #[test]
    fn fractional_box_body_face_owns_ascent_and_explicit_total_height() {
        use crate::entities::style::{Style, Styles};
        let top_ratio = 0.7;
        let spacing = 13.0;
        for body_size in [8.0, 22.0] {
            let body = Content::styled(
                Content::text("BOX"),
                Styles::from_iter([Style::Size(Length::pt(body_size))]),
            );
            let e = fractional_box_element(body, 40.0);
            let doc = fractional_metric_document(
                fractional_paragraph_content(e, spacing, 7.0),
                top_ratio,
                false,
            );
            let g = page_geometry(&doc, 0);
            let geometric_top = point(&g, "BEFORE").y.0 + spacing;
            close(point(&g, "BOX").y.0, geometric_top + body_size * top_ratio);
            close(doc.pages[0].height, geometric_top + 40.0 + 10.0);
        }
    }
    #[test]
    fn fractional_box_safe_baseline_shift_preserves_body_geometric_top() {
        for shift in [0.0, 3.0] {
            let mut e = fractional_box_element(Content::text("BOX"), 16.0);
            e.baseline = Length::pt(shift);
            let doc = fractional_metric_document(
                fractional_paragraph_content(e, 13.0, 7.0),
                0.7,
                false,
            );
            let g = page_geometry(&doc, 0);
            let geometric_top = point(&g, "BEFORE").y.0 + 13.0;
            close(point(&g, "BOX").y.0, geometric_top + 11.0 * 0.7);
            close(doc.pages[0].height, geometric_top + 16.0 + 10.0);
        }
    }
    #[test]
    fn fractional_box_inset_reduces_body_region_within_total_height() {
        let mut e = fractional_box_element(Content::text("BOX"), 24.0);
        e.inset = Sides::uniform(Length::pt(3.0));
        let doc = fractional_metric_document(
            fractional_paragraph_content(e, 13.0, 7.0),
            0.7,
            false,
        );
        let g = page_geometry(&doc, 0);
        let geometric_top = point(&g, "BEFORE").y.0 + 13.0;
        close(point(&g, "BOX").x.0, 13.0);
        close(point(&g, "BOX").y.0, geometric_top + 3.0 + 11.0 * 0.7);
        close(doc.pages[0].height, geometric_top + 24.0 + 10.0);
    }
    #[test]
    fn fractional_box_outset_is_paint_only_for_body_and_suffix() {
        let content = |outset| {
            let mut e = fractional_box_element(Content::text("BOX"), 16.0);
            e.outset = Sides::uniform(Length::pt(outset));
            e.fill = Some(Color::rgb(255, 0, 0));
            Content::sequence(vec![Content::Boxed(Arc::new(e)), Content::text("AFTER")])
        };
        let plain = fractional_metric_document(content(0.0), 0.7, false);
        let expanded = fractional_metric_document(content(4.0), 0.7, false);
        let p = page_geometry(&plain, 0);
        let g = page_geometry(&expanded, 0);
        close(point(&g, "BOX").x.0, point(&p, "BOX").x.0);
        close(point(&g, "BOX").y.0, point(&p, "BOX").y.0);
        close(point(&g, "AFTER").x.0, point(&p, "AFTER").x.0);
        close(point(&g, "AFTER").y.0, point(&p, "AFTER").y.0);
        assert_eq!(g.shapes.len(), 1);
        close(g.shapes[0].0.x.0, p.shapes[0].0.x.0 - 4.0);
        close(g.shapes[0].0.y.0, p.shapes[0].0.y.0 - 4.0);
    }
    #[test]
    fn fractional_box_stale_paragraph_flag_without_chain_is_neutral() {
        let content =
            Content::Boxed(Arc::new(fractional_box_element(Content::text("BOX"), 16.0)));
        let plain = fractional_metric_document(content.clone(), 0.7, false);
        let stale = fractional_metric_document(content, 0.7, true);
        let p = page_geometry(&plain, 0);
        let g = page_geometry(&stale, 0);
        close(point(&g, "BOX").x.0, point(&p, "BOX").x.0);
        close(point(&g, "BOX").y.0, point(&p, "BOX").y.0);
        close(stale.pages[0].height, plain.pages[0].height);
    }
    #[test]
    fn fractional_box_published_body_position_follows_actual_geometric_top() {
        let e = fractional_box_element(located(9017, Content::text("BOX")), 16.0);
        let doc = fractional_metric_document(
            fractional_paragraph_content(e, 13.0, 7.0),
            0.7,
            false,
        );
        let g = page_geometry(&doc, 0);
        let published = published_point(&doc, 9017);
        close(published.x.0, point(&g, "BOX").x.0);
        close(published.y.0, point(&g, "BOX").y.0 - 11.0 * 0.7);
        no_private_positions(&doc.pages[0].items);
    }

    #[test]
    fn fractional_box_committed_geometric_body_keeps_frame_height() {
        let e = fractional_box_element(shape(7.0, 10.0), 16.0);
        let doc = fractional_metric_document(
            fractional_paragraph_content(e, 13.0, 7.0),
            0.7,
            false,
        );
        assert_eq!(doc.pages.len(), 1);
        let g = page_geometry(&doc, 0);
        assert_eq!(g.shapes.len(), 1);
        let geometric_top = point(&g, "BEFORE").y.0 + 13.0;
        close(g.shapes[0].0.x.0, 10.0);
        close(g.shapes[0].0.y.0, geometric_top);
        close(g.shapes[0].1, 7.0);
        close(g.shapes[0].2, 10.0);
        close(doc.pages[0].height, geometric_top + 16.0 + 10.0);
        assert_eq!(g.text.iter().filter(|(s, _)| s == "BEFORE").count(), 1);
    }

    fn nested_inline_space(weight: f64) -> Content {
        Content::HSpace(Arc::new(crate::entities::elements::h_space::HSpaceElem {
            amount: crate::entities::elements::h_space::Spacing::Fractional(weight),
            weak: false,
            weak_explicit: false,
        }))
    }
    fn nested_inline_body() -> Content {
        Content::sequence(vec![
            Content::text("A"),
            nested_inline_space(1.0),
            Content::text("B"),
        ])
    }
    #[test]
    fn nested_inline_child_fraction_uses_only_its_box_budget() {
        let glyph_width = 11.0 * 0.6;
        for exterior_spaces in [false, true] {
            let mut parts = vec![Content::text("P")];
            if exterior_spaces {
                parts.push(nested_inline_space(1.0));
            }
            parts.push(Content::Boxed(Arc::new(fractional_box_element(
                nested_inline_body(),
                16.0,
            ))));
            if exterior_spaces {
                parts.push(nested_inline_space(1.0));
            }
            parts.push(Content::text("Q"));
            let doc = fractional_metric_document(Content::sequence(parts), 0.7, false);
            let g = page_geometry(&doc, 0);
            let free_width = 220.0 - 2.0 * glyph_width;
            let budget = if exterior_spaces { free_width / 3.0 } else { free_width };
            let box_left =
                10.0 + glyph_width + if exterior_spaces { budget } else { 0.0 };
            close(point(&g, "A").x.0, box_left);
            close(point(&g, "B").x.0 + glyph_width, box_left + budget);
            close(point(&g, "Q").x.0 + glyph_width, 230.0);
            for marker in ["P", "A", "B", "Q"] {
                assert_eq!(g.text.iter().filter(|(s, _)| s == marker).count(), 1);
            }
        }
    }
    #[test]
    fn nested_inline_fractional_box_recurses_with_its_actual_inner_width() {
        let inner =
            Content::Boxed(Arc::new(fractional_box_element(nested_inline_body(), 16.0)));
        let outer = Content::Boxed(Arc::new(fractional_box_element(
            Content::sequence(vec![Content::text("I"), inner, Content::text("J")]),
            24.0,
        )));
        let doc = fractional_metric_document(
            Content::sequence(vec![Content::text("P"), outer, Content::text("Q")]),
            0.7,
            false,
        );
        let g = page_geometry(&doc, 0);
        let glyph_width = 11.0 * 0.6;
        close(point(&g, "I").x.0, 10.0 + glyph_width);
        close(point(&g, "A").x.0, 10.0 + 2.0 * glyph_width);
        close(point(&g, "B").x.0 + glyph_width, point(&g, "J").x.0);
        close(point(&g, "J").x.0 + glyph_width, point(&g, "Q").x.0);
        close(point(&g, "Q").x.0 + glyph_width, 230.0);
        for marker in ["P", "I", "A", "B", "J", "Q"] {
            assert_eq!(g.text.iter().filter(|(s, _)| s == marker).count(), 1);
        }
    }
    #[test]
    fn nested_inline_assigned_box_crosses_source_and_style_transparently() {
        use crate::entities::style::{Style, Styles};
        let wrapped =
            Content::Source(Arc::new(crate::entities::content::SourceContent {
                span: crate::entities::span::Span::detached(),
                body: Content::styled(
                    Content::sequence(vec![Content::Boxed(Arc::new(
                        fractional_box_element(nested_inline_body(), 16.0),
                    ))]),
                    Styles::from_iter([Style::bold(true)]),
                ),
            }));
        let doc = fractional_metric_document(
            Content::sequence(vec![Content::text("P"), wrapped, Content::text("Q")]),
            0.7,
            false,
        );
        let g = page_geometry(&doc, 0);
        let glyph_width = 11.0 * 0.6;
        close(point(&g, "A").x.0, 10.0 + glyph_width);
        close(point(&g, "B").x.0 + glyph_width, point(&g, "Q").x.0);
        close(point(&g, "Q").x.0 + glyph_width, 230.0);
    }
    #[test]
    fn nested_inline_complete_flow_scope_keeps_its_exterior_budget() {
        let styles = crate::entities::style::Styles::new()
            .push_custom("eval.flow.scope", crate::entities::value::Value::Bool(true));
        let tail = Content::Source(Arc::new(crate::entities::content::SourceContent {
            span: crate::entities::span::Span::detached(),
            body: Content::styled(
                Content::sequence(vec![
                    Content::text("P"),
                    Content::Boxed(Arc::new(fractional_box_element(
                        nested_inline_body(),
                        16.0,
                    ))),
                    Content::text("Q"),
                ]),
                styles,
            ),
        }));
        let doc = fractional_metric_document(
            Content::Sequence(Arc::from(vec![tail])),
            0.7,
            false,
        );
        let g = page_geometry(&doc, 0);
        let glyph_width = 11.0 * 0.6;
        close(point(&g, "A").x.0, 10.0 + glyph_width);
        close(point(&g, "B").x.0 + glyph_width, point(&g, "Q").x.0);
        close(point(&g, "Q").x.0 + glyph_width, 230.0);
        for marker in ["P", "A", "B", "Q"] {
            assert_eq!(g.text.iter().filter(|(s, _)| s == marker).count(), 1);
        }
    }

    #[test]
    fn nested_inline_fraction_rounding_keeps_body_and_suffix_on_the_line() {
        let mut fractional = fractional_box_element(nested_inline_body(), 16.0);
        fractional.width_fraction = Some(0.1);
        let mut suffix = box_element(Content::text("Q"));
        suffix.width = Some(Length::pt(6.6));
        suffix.height = Some(Length::pt(16.0));
        let doc = fractional_metric_document_with_width(
            Content::sequence(vec![
                Content::Boxed(Arc::new(fractional)),
                nested_inline_space(0.2),
                nested_inline_space(0.3),
                Content::Boxed(Arc::new(suffix)),
            ]),
            0.7,
            false,
            244.0,
        );
        let g = page_geometry(&doc, 0);
        let glyph_width = 11.0 * 0.6;
        let box_width = (224.0 - 6.6) * (0.1 / (0.1 + 0.2 + 0.3));
        close(point(&g, "A").x.0, 10.0);
        close(point(&g, "B").x.0 + glyph_width, 10.0 + box_width);
        close(point(&g, "Q").x.0 + glyph_width, 234.0);
        close(point(&g, "A").y.0, point(&g, "Q").y.0);
        for marker in ["A", "B", "Q"] {
            assert_eq!(g.text.iter().filter(|(s, _)| s == marker).count(), 1);
        }
    }

    #[test]
    fn nested_inline_zero_assigned_box_does_not_consume_body_advance() {
        for body in [Content::Empty, Content::text("A")] {
            let visible = !matches!(body, Content::Empty);
            let fractional = fractional_box_element(body, 16.0);
            let mut suffix = box_element(Content::text("Q"));
            suffix.width = Some(Length::pt(220.0));
            suffix.height = Some(Length::pt(16.0));
            let doc = fractional_metric_document(
                Content::sequence(vec![
                    Content::Boxed(Arc::new(fractional)),
                    Content::Boxed(Arc::new(suffix)),
                ]),
                0.7,
                false,
            );
            let g = page_geometry(&doc, 0);
            close(point(&g, "Q").x.0, 10.0);
            assert_eq!(g.text.iter().filter(|(s, _)| s == "Q").count(), 1);
            assert_eq!(
                g.text.iter().filter(|(s, _)| s == "A").count(),
                usize::from(visible)
            );
            if visible {
                close(point(&g, "A").x.0, 10.0);
            }
        }
    }
    // Independent declared-metric acceptance cases from the Repeat audit.
    // These values are fixture inputs, not measurements fitted to an output.
    struct RepeatBaselineMetrics;
    impl FontMetrics for RepeatBaselineMetrics {
        fn advance(
            &self,
            text: &str,
            _: Pt,
            _: &crate::entities::layout_types::TextStyle,
        ) -> Pt {
            Pt(text.chars().count() as f64 * 10.0)
        }
        fn text_width(
            &self,
            text: &str,
            size: Pt,
            style: &crate::entities::layout_types::TextStyle,
        ) -> Pt {
            self.advance(text, size, style)
        }
        fn vertical_metrics(
            &self,
            _: Pt,
            _: &crate::entities::layout_types::TextStyle,
        ) -> (Pt, Pt) {
            (Pt(6.0), Pt(8.0))
        }
        fn cap_height(&self, _: Pt, _: &crate::entities::layout_types::TextStyle) -> Pt {
            Pt(6.0)
        }
        fn text_edges(
            &self,
            _: Pt,
            _: &crate::entities::layout_types::TextStyle,
        ) -> (Pt, Pt) {
            (Pt(6.0), Pt(-2.0))
        }
    }
    fn repeat_baseline_box(
        body: Content,
        width: f64,
        height: f64,
        inset: f64,
        shift: f64,
    ) -> Content {
        let mut e = box_element(body);
        e.width = Some(Length::pt(width));
        e.height = Some(Length::pt(height));
        e.inset = Sides::uniform(Length::pt(inset));
        e.baseline = Length::pt(shift);
        Content::Boxed(Arc::new(e))
    }
    fn repeat_baseline_piece(gap: f64) -> Content {
        Content::Repeat(Arc::new(crate::entities::elements::repeat::RepeatElem {
            body: Content::text("X"),
            gap: Some(Length::pt(gap)),
            justify: false,
        }))
    }
    fn repeat_baseline_document(
        body: Content,
    ) -> crate::entities::layout_types::PagedDocument {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            RepeatBaselineMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = 260.0;
        l.page_config.height = f64::INFINITY;
        l.page_config.margin = PageMargins::uniform(10.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(10.0);
        l.regions.current.width = 260.0;
        l.regions.current.height = f64::INFINITY;
        l.regions.current.cursor_x = Pt(10.0);
        l.regions.current.line_start_x = Pt(10.0);
        l.regions.current.cursor_y = Pt(10.0);
        l.chain = l.chain.push_custom("par.spacing", Value::Length(Length::pt(12.0)));
        l.layout_content(&Content::sequence(vec![
            Content::text("BEFORE"),
            body,
            Content::text("TAIL"),
        ]));
        l.finish()
    }
    #[test]
    fn box_repeat_zero_copies_keep_the_prepared_piece_baseline() {
        for (gap, expected_copies) in [(-100.0, 0), (3.0, 6)] {
            let doc = repeat_baseline_document(repeat_baseline_box(
                repeat_baseline_piece(gap),
                80.0,
                16.0,
                0.0,
                0.0,
            ));
            let g = page_geometry(&doc, 0);
            assert_eq!(g.text.iter().filter(|(s, _)| s == "X").count(), expected_copies);
            close(point(&g, "BEFORE").y.0, 10.0 + 6.0);
            close(point(&g, "TAIL").y.0, 10.0 + 6.0);
            assert!(doc.layout_errors.is_empty());
        }
    }
    #[test]
    fn box_repeat_unpainted_baseline_does_not_replace_earlier_text() {
        let doc = repeat_baseline_document(repeat_baseline_box(
            Content::sequence(vec![Content::text("A"), repeat_baseline_piece(-100.0)]),
            80.0,
            40.0,
            0.0,
            0.0,
        ));
        let g = page_geometry(&doc, 0);
        assert!(!g.text.iter().any(|(s, _)| s == "X"));
        for marker in ["BEFORE", "A", "TAIL"] {
            close(point(&g, marker).y.0, 10.0 + 6.0);
        }
    }
    #[test]
    fn box_repeat_first_unpainted_baseline_precedes_later_text() {
        let doc = repeat_baseline_document(repeat_baseline_box(
            Content::sequence(vec![repeat_baseline_piece(-100.0), Content::text("A")]),
            80.0,
            40.0,
            0.0,
            0.0,
        ));
        let g = page_geometry(&doc, 0);
        assert!(!g.text.iter().any(|(s, _)| s == "X"));
        close(point(&g, "BEFORE").y.0, 10.0 + 6.0);
        close(point(&g, "TAIL").y.0, 10.0 + 6.0);
        close(point(&g, "A").y.0, 10.0 + 8.0 + 12.0 + 6.0);
    }
    #[test]
    fn box_repeat_unpainted_baseline_composes_nested_insets_and_shifts() {
        for (inner_shift, outer_shift) in [(0.0, 0.0), (1.0, 2.0)] {
            let inner = repeat_baseline_box(
                repeat_baseline_piece(-100.0),
                80.0,
                16.0,
                2.0,
                inner_shift,
            );
            let doc = repeat_baseline_document(repeat_baseline_box(
                inner,
                100.0,
                24.0,
                3.0,
                outer_shift,
            ));
            let g = page_geometry(&doc, 0);
            assert!(!g.text.iter().any(|(s, _)| s == "X"));
            let baseline = 10.0 + 6.0 + 2.0 + 3.0 - inner_shift - outer_shift;
            close(point(&g, "BEFORE").y.0, baseline);
            close(point(&g, "TAIL").y.0, baseline);
        }
    }
    #[test]
    fn box_repeat_witness_preserves_empty_body_bottom_baseline() {
        let doc = repeat_baseline_document(repeat_baseline_box(
            Content::Empty,
            80.0,
            16.0,
            0.0,
            0.0,
        ));
        let g = page_geometry(&doc, 0);
        close(point(&g, "BEFORE").y.0, 10.0 + 16.0);
        close(point(&g, "TAIL").y.0, 10.0 + 16.0);
    }
    #[test]
    fn box_repeat_witness_preserves_shape_body_bottom_baseline() {
        let doc = repeat_baseline_document(repeat_baseline_box(
            shape(7.0, 9.0),
            80.0,
            16.0,
            0.0,
            0.0,
        ));
        let g = page_geometry(&doc, 0);
        close(point(&g, "BEFORE").y.0, 10.0 + 16.0);
        close(point(&g, "TAIL").y.0, 10.0 + 16.0);
    }
    #[test]
    fn box_repeat_witness_does_not_invent_baseline_after_first_shape() {
        let doc = repeat_baseline_document(repeat_baseline_box(
            Content::sequence(vec![shape(7.0, 9.0), Content::text("A")]),
            80.0,
            40.0,
            0.0,
            0.0,
        ));
        let g = page_geometry(&doc, 0);
        close(point(&g, "BEFORE").y.0, 10.0 + 40.0);
        close(point(&g, "TAIL").y.0, 10.0 + 40.0);
        // Direct Shape subframes keep their existing9pt progress; this is
        // separate from the authoritative absence of a first-frame baseline.
        close(point(&g, "A").y.0, 10.0 + 9.0);
    }
    #[test]
    fn box_repeat_opaque_stack_prefix_keeps_existing_first_paint_fallback() {
        for inside_repeat in [false, true] {
            let stack =
                Content::Stack(Arc::new(crate::entities::elements::stack::StackElem {
                    children: Arc::from(vec![Content::text("S"), Content::text("T")]),
                    dir: crate::entities::dir::Dir::TTB,
                    spacing: Some(Length::pt(3.0)),
                }));
            let prefix = if inside_repeat {
                Content::Repeat(Arc::new(crate::entities::elements::repeat::RepeatElem {
                    body: stack,
                    gap: Some(Length::pt(3.0)),
                    justify: false,
                }))
            } else {
                stack
            };
            let doc = repeat_baseline_document(repeat_baseline_box(
                Content::sequence(vec![prefix, Content::text("A")]),
                80.0,
                60.0,
                0.0,
                0.0,
            ));
            let g = page_geometry(&doc, 0);
            // Stack's metadata producer is uncertified. Preserve the existing
            // affine first-painted Text fallback, including under Repeat.
            close(point(&g, "BEFORE").y.0, point(&g, "S").y.0);
            close(point(&g, "TAIL").y.0, point(&g, "S").y.0);
            assert!(point(&g, "A").y.0 > point(&g, "S").y.0);
            assert!(doc.layout_errors.is_empty());
        }
    }
    fn source_page_style_tail() -> Content {
        use crate::entities::style::Styles;
        let font = Styles::new()
            .push_custom(
                "text.font",
                Value::Array(vec![Value::Str("DejaVu Sans".into())]),
            )
            .push_custom("eval.flow.scope", Value::Bool(true));
        let page = Styles::new()
            .push_custom("page.margin-bottom", Value::Float(8.0))
            .push_custom("page.margin-top", Value::Float(8.0))
            .push_custom("page.margin-right", Value::Float(8.0))
            .push_custom("page.margin-left", Value::Float(8.0))
            .push_custom("page.height", Value::Auto)
            .push_custom("page.width", Value::Float(240.0))
            .push_custom("eval.flow.scope", Value::Bool(true));
        Content::styled(
            Content::Sequence(Arc::from(vec![
                Content::SpaceFromNewline,
                Content::styled(
                    Content::Sequence(Arc::from(vec![
                        Content::Empty,
                        Content::text("BEFORE"),
                        Content::Parbreak,
                        Content::Boxed(Arc::new(fractional_box_element(
                            nested_inline_body(),
                            16.0,
                        ))),
                        Content::Space,
                        Content::text("AFTER"),
                        Content::SpaceFromNewline,
                    ])),
                    font,
                ),
            ])),
            page,
        )
    }
    fn source_page_change() -> Content {
        Content::SetPage(Arc::new(crate::entities::content::SetPageDelta {
            width: Some(crate::entities::layout_types::PageDimension::Length(240.0)),
            height: Some(crate::entities::layout_types::PageDimension::Auto),
            margin: Some(PageMarginSpec::uniform(8.0)),
            ..Default::default()
        }))
    }
    fn assert_source_page_tail(
        doc: &crate::entities::layout_types::PagedDocument,
        page: usize,
    ) {
        let g = page_geometry(doc, page);
        let glyph = 11.0 * 0.6;
        close(doc.pages[page].width, 240.0);
        close(point(&g, "A").x.0, 8.0);
        close(point(&g, "B").x.0 + glyph, point(&g, "AFTER").x.0 - glyph);
        close(point(&g, "AFTER").x.0 + 5.0 * glyph, 240.0 - 8.0);
        for marker in ["BEFORE", "A", "B", "AFTER"] {
            assert_eq!(g.text.iter().filter(|(s, _)| s == marker).count(), 1);
        }
    }
    #[test]
    fn nested_inline_actual_page_style_tree_uses_dispatched_geometry() {
        let content = Content::Sequence(Arc::from(vec![
            source_page_change(),
            source_page_style_tail(),
        ]));
        let doc = fractional_metric_document_with_width(content, 0.7, false, 360.0);
        assert_eq!(doc.pages.len(), 1);
        assert_source_page_tail(&doc, 0);
    }
    #[test]
    fn nested_inline_actual_page_style_tree_preserves_prior_page_prefix() {
        let content = Content::Sequence(Arc::from(vec![
            Content::text("OLD"),
            source_page_change(),
            source_page_style_tail(),
        ]));
        let doc = fractional_metric_document_with_width(content, 0.7, false, 360.0);
        assert_eq!(doc.pages.len(), 2);
        close(doc.pages[0].width, 360.0);
        assert_eq!(
            page_geometry(&doc, 0).text.iter().filter(|(s, _)| s == "OLD").count(),
            1
        );
        assert_source_page_tail(&doc, 1);
    }

    fn repeat_auto_height_scene(siblings: bool) -> Content {
        use crate::entities::style::Styles;
        let repeat =
            |text| Content::repeat(Content::text(text), Some(Length::pt(2.0)), false);
        let fractional = |body, weight| {
            let mut e = box_element(body);
            e.width = None;
            e.width_fraction = Some(weight);
            e.height = None;
            Content::Boxed(Arc::new(e))
        };
        let parts = if siblings {
            vec![
                Content::text("A"),
                fractional(repeat("B"), 1.0),
                Content::text("M"),
                fractional(repeat("C"), 2.0),
                Content::text("TAIL"),
                Content::SpaceFromNewline,
            ]
        } else {
            let mut inner = box_element(repeat("B"));
            inner.width = Some(Length::pt(40.0));
            inner.height = None;
            vec![
                Content::text("A"),
                fractional(
                    Content::Sequence(Arc::from(vec![
                        Content::text("I"),
                        Content::Boxed(Arc::new(inner)),
                        Content::text("J"),
                    ])),
                    1.0,
                ),
                Content::text("TAIL"),
                Content::SpaceFromNewline,
            ]
        };
        let font = Styles::new()
            .push_custom(
                "text.font",
                Value::Array(vec![Value::Str("DejaVu Sans".into())]),
            )
            .push_custom("eval.flow.scope", Value::Bool(true));
        let page = Styles::new()
            .push_custom("page.margin-bottom", Value::Float(10.0))
            .push_custom("page.margin-top", Value::Float(10.0))
            .push_custom("page.margin-right", Value::Float(10.0))
            .push_custom("page.margin-left", Value::Float(10.0))
            .push_custom("page.height", Value::Auto)
            .push_custom("page.width", Value::Float(220.0))
            .push_custom("eval.flow.scope", Value::Bool(true));
        Content::styled(
            Content::Sequence(Arc::from(vec![
                Content::SpaceFromNewline,
                Content::styled(Content::Sequence(Arc::from(parts)), font),
            ])),
            page,
        )
    }
    fn repeat_auto_height_document(
        content: &Content,
        planned: bool,
        following: bool,
    ) -> crate::entities::layout_types::PagedDocument {
        use super::super::page_running_requests::TraceSection;
        use crate::compiler::introspect::page_running::prepare_page_running_body;
        use crate::compiler::realize::context_occurrences::ContextOccurrenceArena;
        let extended = following.then(|| {
            Content::Sequence(Arc::from(vec![
                content.clone(),
                Content::Parbreak,
                Content::text("LATER"),
            ]))
        });
        let content = extended.as_ref().unwrap_or(content);
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FractionalFaceMetrics { top_ratio: 0.7 },
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = 220.0;
        l.page_config.height = f64::INFINITY;
        l.page_config.margin = PageMargins::uniform(10.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(10.0);
        l.regions.current.width = 220.0;
        l.regions.current.height = f64::INFINITY;
        l.regions.current.cursor_x = Pt(10.0);
        l.regions.current.line_start_x = Pt(10.0);
        l.regions.current.cursor_y = Pt(10.0);
        if planned {
            let mut ctx = crate::compiler::eval::EvalContext::new();
            let mut arena = ContextOccurrenceArena::new(content);
            let mut frame = arena.frame();
            let body = arena
                .evaluate(&mut frame, &mut ctx, |ctx| {
                    prepare_page_running_body(content, ctx)
                })
                .expect("source-equivalent planned body");
            l.page_running_pass =
                Some(super::super::page_running_requests::PlannedPass::new(&body, None));
            l.layout_planned_root(&body, TraceSection::Body);
        } else {
            l.layout_content(content);
        }
        l.finish()
    }
    fn assert_repeat_auto_height(siblings: bool) {
        let scene = repeat_auto_height_scene(siblings);
        for planned in [false, true] {
            let doc = repeat_auto_height_document(&scene, planned, false);
            assert_eq!(doc.pages.len(), 1);
            assert!(doc.layout_errors.is_empty());
            let g = page_geometry(&doc, 0);
            for marker in ["A", "TAIL"] {
                assert_eq!(g.text.iter().filter(|(s, _)| s == marker).count(), 1);
            }
            let baseline = point(&g, "TAIL").y.0;
            close(baseline, 10.0 + 11.0 * 0.7);
            assert!(g.text.iter().all(|(_, p)| (p.y.0 - baseline).abs() < 1e-8));
            assert!(
                (doc.pages[0].height - baseline - 10.0).abs() < 1e-8,
                "planned={planned} actual height={} terminal baseline={baseline}",
                doc.pages[0].height
            );
        }
    }
    #[test]
    fn repeat_auto_height_nested_fixed_source_keeps_terminal_causal_baseline() {
        assert_repeat_auto_height(false);
    }
    #[test]
    fn repeat_auto_height_unequal_sibling_source_keeps_terminal_causal_baseline() {
        assert_repeat_auto_height(true);
    }

    #[test]
    fn repeat_auto_height_following_text_replaces_weakly_closed_line_bottom() {
        for siblings in [false, true] {
            let scene = repeat_auto_height_scene(siblings);
            for planned in [false, true] {
                let doc = repeat_auto_height_document(&scene, planned, true);
                assert_eq!(doc.pages.len(), 1);
                assert!(doc.layout_errors.is_empty());
                let g = page_geometry(&doc, 0);
                assert_eq!(g.text.iter().filter(|(s, _)| s == "LATER").count(), 1);
                let tail = point(&g, "TAIL").y.0;
                let later = point(&g, "LATER").y.0;
                assert!(later > tail, "real following text owns the next line");
                close(doc.pages[0].height, later + 10.0);
            }
        }
    }
    #[test]
    fn remaining_inline_shape_flow_boundary_preserves_following_fraction_budget() {
        let body = Content::Sequence(Arc::from(vec![
            shape(12.0, 8.0),
            Content::text("P"),
            Content::Boxed(Arc::new(fractional_box_element(nested_inline_body(), 16.0))),
            Content::text("Q"),
        ]));
        let doc = fractional_metric_document(body, 0.7, false);
        assert!(doc.layout_errors.is_empty());
        assert_eq!(doc.pages.len(), 1);
        let g = page_geometry(&doc, 0);
        let glyph = 11.0 * 0.6;
        close(point(&g, "A").x.0, 10.0 + glyph);
        close(point(&g, "B").x.0 + glyph, point(&g, "Q").x.0);
        close(point(&g, "Q").x.0 + glyph, 230.0);
        assert_eq!(g.shapes.len(), 1);
        for marker in ["P", "A", "B", "Q"] {
            assert_eq!(g.text.iter().filter(|(s, _)| s == marker).count(), 1);
        }
    }

    #[test]
    fn remaining_inline_negative_fraction_composes_body_in_its_actual_share() {
        let mut e = fractional_box_element(nested_inline_body(), 16.0);
        e.width_fraction = Some(-1.0);
        let doc = fractional_metric_document(
            Content::Sequence(Arc::from(vec![
                Content::text("P"),
                Content::Boxed(Arc::new(e)),
                Content::text("Q"),
            ])),
            0.7,
            false,
        );
        let g = page_geometry(&doc, 0);
        let glyph = 11.0 * 0.6;
        close(point(&g, "A").x.0, 10.0 + glyph);
        close(point(&g, "B").x.0 + glyph, point(&g, "Q").x.0);
        close(point(&g, "Q").x.0 + glyph, 230.0);
        for marker in ["P", "A", "B", "Q"] {
            assert_eq!(g.text.iter().filter(|(s, _)| s == marker).count(), 1);
        }
    }

    #[test]
    fn remaining_inline_rtl_latin_fraction_keeps_actual_local_body_width() {
        use crate::entities::{dir::Dir, style::Styles};
        let body = Content::styled(
            Content::Sequence(Arc::from(vec![
                Content::text("P"),
                Content::Boxed(Arc::new(fractional_box_element(
                    nested_inline_body(),
                    16.0,
                ))),
                Content::text("Q"),
            ])),
            Styles::new().push_custom("text.dir", Value::Dir(Dir::RTL)),
        );
        let doc = fractional_metric_document(body, 0.7, false);
        let g = page_geometry(&doc, 0);
        let glyph = 11.0 * 0.6;
        close(point(&g, "P").x.0, 10.0);
        close(point(&g, "A").x.0, 10.0 + glyph);
        close(point(&g, "B").x.0 + glyph, point(&g, "Q").x.0);
        close(point(&g, "Q").x.0 + glyph, 230.0);
        for marker in ["P", "A", "B", "Q"] {
            assert_eq!(g.text.iter().filter(|(s, _)| s == marker).count(), 1);
        }
    }

    #[test]
    fn remaining_inline_paragraph_after_tall_box_starts_after_physical_bottom() {
        use crate::entities::style::Styles;
        let body = Content::styled(
            Content::Sequence(Arc::from(vec![
                Content::text("P"),
                Content::Boxed(Arc::new(fractional_box_element(
                    Content::text("A"),
                    40.0,
                ))),
                Content::Parbreak,
                Content::text("NEXT"),
            ])),
            Styles::new()
                .push_custom("par.spacing", Value::Length(Length::pt(13.0)))
                .push_custom("par.leading", Value::Length(Length::pt(7.0))),
        );
        let doc = fractional_metric_document(body, 0.7, false);
        let g = page_geometry(&doc, 0);
        close(point(&g, "NEXT").y.0, 10.0 + 40.0 + 13.0 + 11.0 * 0.7);
        close(doc.pages[0].height, point(&g, "NEXT").y.0 + 10.0);
        for marker in ["P", "A", "NEXT"] {
            assert_eq!(g.text.iter().filter(|(s, _)| s == marker).count(), 1);
        }
    }
    #[test]
    fn remaining_inline_paragraph_descent_controls_cover_shifts_empty_and_double_breaks()
    {
        use crate::entities::style::Styles;
        for height in [0.0, 40.0] {
            for shift in [-5.0, 0.0, 5.0] {
                for double in [false, true] {
                    let mut e = fractional_box_element(Content::text("A"), height);
                    e.baseline = Length::pt(shift);
                    let mut parts = vec![
                        Content::text("P"),
                        Content::Boxed(Arc::new(e)),
                        Content::Parbreak,
                    ];
                    if double {
                        parts.push(Content::Parbreak);
                    }
                    parts.push(Content::text("NEXT"));
                    let body = Content::styled(
                        Content::Sequence(Arc::from(parts)),
                        Styles::new()
                            .push_custom("par.spacing", Value::Length(Length::pt(13.0)))
                            .push_custom("par.leading", Value::Length(Length::pt(7.0))),
                    );
                    let doc = fractional_metric_document(body, 0.7, false);
                    let g = page_geometry(&doc, 0);
                    let text_top = 11.0 * 0.7;
                    let box_baseline = text_top - shift;
                    let baseline = 10.0 + text_top.max(box_baseline);
                    let line_descent = (height - box_baseline).max(0.0);
                    close(
                        point(&g, "NEXT").y.0,
                        baseline + line_descent + 13.0 + text_top,
                    );
                    assert_eq!(g.text.iter().filter(|(s, _)| s == "NEXT").count(), 1);
                }
            }
        }
        let body = Content::styled(
            Content::Sequence(Arc::from(vec![
                Content::text("P"),
                Content::Parbreak,
                Content::text("NEXT"),
            ])),
            Styles::new()
                .push_custom("par.spacing", Value::Length(Length::pt(13.0)))
                .push_custom("par.leading", Value::Length(Length::pt(7.0))),
        );
        let doc = fractional_metric_document(body, 0.7, false);
        let g = page_geometry(&doc, 0);
        close(point(&g, "NEXT").y.0, point(&g, "P").y.0 + 13.0 + 11.0 * 0.7);
    }

    #[test]
    fn remaining_inline_shape_boundary_inside_sized_subframe_keeps_its_actual_region() {
        let body = Content::Sequence(Arc::from(vec![
            shape(12.0, 8.0),
            Content::text("P"),
            Content::Boxed(Arc::new(fractional_box_element(nested_inline_body(), 16.0))),
            Content::text("Q"),
        ]));
        let mut e = box_element(body);
        e.width = Some(Length::pt(100.0));
        e.height = None;
        let doc = fractional_metric_document(Content::Boxed(Arc::new(e)), 0.7, false);
        let g = page_geometry(&doc, 0);
        let glyph = 11.0 * 0.6;
        close(point(&g, "A").x.0, 10.0 + glyph);
        close(point(&g, "B").x.0 + glyph, point(&g, "Q").x.0);
        close(point(&g, "Q").x.0 + glyph, 110.0);
        assert_eq!(g.shapes.len(), 1);
    }

    #[test]
    fn remaining_inline_paragraph_to_shape_preserves_settled_descent_and_margin_collapse()
    {
        use crate::entities::style::Styles;
        let body = Content::styled(
            Content::Sequence(Arc::from(vec![
                Content::text("P"),
                Content::Boxed(Arc::new(fractional_box_element(
                    Content::text("A"),
                    40.0,
                ))),
                Content::Parbreak,
                shape(12.0, 8.0),
            ])),
            Styles::new()
                .push_custom("par.spacing", Value::Length(Length::pt(13.0)))
                .push_custom("par.leading", Value::Length(Length::pt(7.0))),
        );
        let doc = fractional_metric_document(body, 0.7, false);
        let g = page_geometry(&doc, 0);
        assert_eq!(g.shapes.len(), 1);
        close(g.shapes[0].0.y.0, 10.0 + 40.0 + (11.0_f64 * 1.2).max(13.0));
    }
    #[test]
    fn remaining_inline_known_weak_wrap_allocates_before_original_source_dispatch() {
        let body = Content::Sequence(Arc::from(vec![
            Content::text("P"),
            Content::Boxed(Arc::new(fractional_box_element(nested_inline_body(), 16.0))),
            Content::text("Q"),
            Content::Space,
            Content::text("STATE"),
            Content::text("1"),
            Content::Space,
            Content::text("COUNTER"),
            Content::text("1"),
            Content::Space,
            Content::text("LABELS"),
            Content::text("1"),
        ]));
        let doc = fractional_metric_document_with_width(body, 0.7, false, 180.0);
        let g = page_geometry(&doc, 0);
        let glyph = 11.0 * 0.6;
        let first_fixed = 16.0 * glyph + 2.0 * glyph;
        let share = 160.0 - first_fixed;
        close(point(&g, "A").x.0, 10.0 + glyph);
        close(point(&g, "B").x.0 + glyph, 10.0 + glyph + share);
        close(point(&g, "Q").x.0, 10.0 + glyph + share);
        close(point(&g, "LABELS").x.0, 10.0);
        assert!(point(&g, "LABELS").y.0 > point(&g, "COUNTER").y.0);
        for marker in ["P", "A", "B", "Q", "STATE", "COUNTER", "LABELS"] {
            assert_eq!(g.text.iter().filter(|(s, _)| s == marker).count(), 1);
        }
        assert_eq!(g.text.iter().filter(|(s, _)| s == "1").count(), 3);
    }
}
