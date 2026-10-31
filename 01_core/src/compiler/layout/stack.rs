//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/stack.md
//! @prompt-hash 9afb0a7f
//! @layer L1
//! @updated 2026-08-21
//!
//! Layout dinâmico do container compositivo `Stack` (Passo 1121).

use super::{FontMetrics, ImageSizer, Layouter};
use crate::entities::{
    elements::stack::StackElem,
    layout_types::{Align2D, FrameItem, HAlign, Pt, Rect},
    value::Value,
};

struct ChildFrame {
    width: f64,
    height: f64,
    items: Vec<FrameItem>,
}

fn block_margin<M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
    edge: &str,
) -> (f64, bool) {
    for key in [edge, "block.spacing"] {
        if let Some(Value::Length(length)) = l.chain.custom(key) {
            return (length.resolve_pt(l.style.size.0), true);
        }
        if l.chain.custom(key) == Some(&Value::Auto) {
            break;
        }
    }
    (super::paragraph::spacing(&l.chain), false)
}

fn publish<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    end: f64,
    below: f64,
    explicit: bool,
) {
    l.regions.current.cursor_x = l.regions.current.line_start_x;
    l.regions.current.cursor_y = Pt(end);
    l.prev_line_baseline = end;
    l.prev_block_equation_descent = 0.0;
    l.last_block_descent_y = Some(end);
    l.prev_block_below_pending = below;
    l.prev_margin_is_parbreak = !explicit;
    l.block_chain_active = true;
    l.initial_baseline_pending = false;
    l.last_flush_advance = 0.0;
    l.record_flow_baseline(None);
    l.record_flow_placement_end(end);
}

fn next_region<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    outer: Option<Rect>,
    stack_bbox: Option<Rect>,
) -> f64 {
    l.parent_bbox = outer;
    l.new_page();
    l.parent_bbox = stack_bbox;
    let top = if l.initial_baseline_pending {
        0.0
    } else {
        l.metrics.text_edges(l.style.size, &l.style).0 .0
    };
    l.regions.current.cursor_y.0 - top
}

/// Compose source-ordered children using logical frames, never text ink as size.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    e: &StackElem,
) {
    if e.children.is_empty() {
        return;
    }
    super::h_space::close_paragraph(l);
    let had_text =
        super::html_frame::items_have_causal_line(&l.regions.current.current_line);
    if had_text {
        l.ensure_initial_baseline();
    }
    let text_baseline = l.regions.current.cursor_y.0;
    l.flush_line();
    let (above, above_explicit) = block_margin(l, "block.above");
    let (below, below_explicit) = block_margin(l, "block.below");
    let mut top = if had_text {
        text_baseline + above
    } else if l.block_chain_active {
        let gap = if above_explicit && l.prev_margin_is_parbreak {
            above
        } else if !above_explicit && !l.prev_margin_is_parbreak {
            l.prev_block_below_pending
        } else {
            above.max(l.prev_block_below_pending)
        };
        l.prev_line_baseline + l.prev_block_equation_descent + gap
    } else if !l.is_sub_frame
        && !l.initial_baseline_pending
        && l.current_page_is_empty()
        && l.flow_placement_y().is_none()
    {
        l.regions.current.cursor_y.0
            - l.line_assumed_ascent
                .unwrap_or_else(|| l.metrics.text_edges(l.style.size, &l.style).0 .0)
    } else {
        l.regions.current.cursor_y.0
    };
    let saved_bbox = l.parent_bbox;
    let base_x = l.regions.current.line_start_x.0;
    let available =
        (l.regions.current.width - l.page_config.margin.right - base_x).max(0.0);
    let (estimate_w, estimate_h) =
        l.measure_stack(&e.children, e.dir, e.spacing, available);
    if estimate_w > 0.0 && estimate_h > 0.0 {
        l.parent_bbox = Some(Rect {
            x: Pt(base_x),
            y: Pt(top),
            w: Pt(estimate_w),
            h: Pt(estimate_h),
        });
    }
    let stack_bbox = l.parent_bbox;
    let spacing = e.spacing.map_or(0.0, |v| v.resolve_pt(l.style.size.0));
    let mut frames = Vec::with_capacity(e.children.len());
    for (index, child) in e.children.iter().enumerate() {
        let planned = l.planned_child_cursor(
            crate::compiler::introspect::page_running::PlanEdge::StackChild(index),
        );
        let (height, mut items, _deco, align_x, _oy) = l.layout_stack_child(
            child,
            super::sub_frame::SubLayoutRegion {
                origin_x: base_x,
                width: available,
                height: None,
                align_rtl: false,
                unconstrained_height: true,
            },
            planned,
            !e.dir.is_vertical(),
        );
        let width = l.last_sub_frame_flow_width;
        if !e.dir.is_vertical() {
            // These local paths own the completed child, never the parent's queue.
            let saved = std::mem::replace(&mut l.pending_align_centering, align_x);
            l.apply_pending_align_fixups(
                &mut items,
                width + l.page_config.margin.horizontal(),
            );
            l.pending_align_centering = saved;
        }
        frames.push(ChildFrame { height, width, items });
    }
    let vertical = e.dir.is_vertical();
    let mut first = 0;
    while first < frames.len() {
        let mut last = first;
        let mut main = 0.0;
        let mut cross = 0.0_f64;
        while last < frames.len() {
            let frame = &frames[last];
            let extent = if vertical { frame.height } else { frame.width };
            let addition = extent + if last > first { spacing } else { 0.0 };
            if vertical
                && super::block::allows_fragmentation(l)
                && top + main + addition > l.page_bottom_limit()
            {
                if last > first {
                    break;
                }
                if !l.current_page_is_empty() || top > l.page_config.margin.top {
                    top = next_region(l, saved_bbox, stack_bbox);
                }
            }
            main += addition;
            cross = cross.max(if vertical { frame.width } else { frame.height });
            last += 1;
        }
        let height = if vertical { main.max(0.0) } else { cross };
        let width = if vertical { cross } else { main.max(0.0) };
        if !vertical
            && super::block::allows_fragmentation(l)
            && top + height > l.page_bottom_limit()
            && (!l.current_page_is_empty() || top > l.page_config.margin.top)
        {
            top = next_region(l, saved_bbox, stack_bbox);
        }
        let frame_start = l.regions.current.current_items.len();
        let exterior_rtl = !vertical
            && l.explicit_line_alignment.is_none()
            && matches!(l.chain.custom("text.dir"), Some(Value::Dir(dir)) if *dir == crate::entities::dir::Dir::RTL);
        let regional_width =
            l.regions.current.width - l.page_config.margin.right - base_x;
        let exterior_x =
            if exterior_rtl && !l.block_auto_width && regional_width.is_finite() {
                regional_width - width
            } else {
                0.0
            };
        let reverse_extent =
            if vertical && e.dir.is_reverse() && !super::block::allows_fragmentation(l) {
                super::block::stack_region_bottom(l)
                    .or_else(|| {
                        l.block_place_region
                            .filter(|region| region.h.0.is_finite())
                            .map(|region| region.y.0 + region.h.0)
                    })
                    .map_or(main, |bottom| main.min((bottom - top).max(0.0)))
            } else {
                main
            };
        let mut cursor = 0.0;
        for (offset, frame) in frames[first..last].iter_mut().enumerate() {
            if offset > 0 {
                cursor += spacing;
            }
            let extent = if vertical { frame.height } else { frame.width };
            let position = if e.dir.is_reverse() {
                reverse_extent - extent - cursor
            } else {
                cursor
            };
            let (dx, dy) = if vertical { (0.0, top + position) } else { (position, top) };
            for mut item in frame.items.drain(..) {
                super::helpers::offset_frame_item(&mut item, dx + exterior_x, dy);
                l.regions.current.current_items.push(item);
            }
            if !l.is_sub_frame && (frame.width > 0.0 || frame.height > 0.0) {
                l.page_has_unpainted_material = true;
            }
            cursor += extent;
        }
        if exterior_rtl && l.block_auto_width {
            let count = l.regions.current.current_items.len() - frame_start;
            if count > 0 {
                l.pending_align_centering.push((
                    vec![frame_start],
                    count,
                    Align2D { h: Some(HAlign::Right), v: None },
                    width,
                    base_x,
                    base_x,
                ));
            }
        }
        l.record_flow_width_at(base_x, width);
        publish(
            l,
            top + height,
            if last == frames.len() { below } else { 0.0 },
            if last == frames.len() { below_explicit } else { false },
        );
        first = last;
        if first < frames.len() {
            top = next_region(l, saved_bbox, stack_bbox);
        }
    }
    l.parent_bbox = saved_bbox;
}

#[cfg(test)]
mod stack_image_tests {
    use crate::compiler::layout::{FixedMetrics, Layouter};
    use crate::entities::{
        content::Content,
        dir::Dir,
        elements::image::ImageElem,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{
            FrameItem, Length, PageMarginSpec, PageMargins, PagedDocument, Pt,
        },
        ptr_eq_arc::PtrEqArc,
        value::Value,
    };
    use comemo::Track;
    use std::sync::Arc;

    fn photo(w: f64, h: f64) -> Content {
        Content::Image(Arc::new(ImageElem {
            path: String::new(),
            alt: None,
            data: PtrEqArc(Arc::new(vec![])),
            width: Some(Box::new(Value::Float(w))),
            height: Some(Box::new(Value::Float(h))),
            fit: "stretch".into(),
        }))
    }
    fn pics(n: usize) -> Vec<Content> {
        (0..n).map(|_| photo(30.0, 20.0)).collect()
    }
    fn stack(children: Vec<Content>, dir: Dir, spacing: f64) -> Content {
        Content::stack(children, dir, Some(Length::pt(spacing)))
    }
    fn doc(content: Content, height: f64) -> PagedDocument {
        doc_width(content, 400.0, height)
    }
    fn doc_width(content: Content, width: f64, height: f64) -> PagedDocument {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = width;
        l.page_config.height = height;
        l.page_config.margin = PageMargins::uniform(5.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(5.0);
        l.regions.current.width = width;
        l.regions.current.height = height;
        l.regions.current.cursor_x = Pt(5.0);
        l.regions.current.line_start_x = Pt(5.0);
        l.regions.current.cursor_y = Pt(5.0);
        l.layout_content(&content);
        l.finish()
    }
    fn images(doc: &PagedDocument) -> Vec<Vec<(f64, f64)>> {
        doc.pages
            .iter()
            .map(|p| {
                p.items
                    .iter()
                    .filter_map(|i| match i {
                        FrameItem::Image { pos, .. } => Some((pos.x.0, pos.y.0)),
                        _ => None,
                    })
                    .collect()
            })
            .collect()
    }
    fn ys(actual: &[(f64, f64)], expected: &[f64]) {
        assert_eq!(actual.len(), expected.len());
        for ((_, a), b) in actual.iter().zip(expected) {
            assert!((a - b).abs() < 1e-9, "image top {a} != {b}");
        }
    }
    #[test]
    fn stack_image_vertical_frames_have_physical_extent() {
        let d = doc(stack(pics(16), Dir::TTB, 0.0), 400.0);
        let p = images(&d);
        assert_eq!(p.len(), 1);
        ys(&p[0], &(0..16).map(|i| 5.0 + 20.0 * i as f64).collect::<Vec<_>>());
    }
    #[test]
    fn stack_image_reverse_directions_keep_source_order() {
        let d = doc(
            stack(
                vec![photo(30.0, 20.0), photo(40.0, 10.0), photo(15.0, 30.0)],
                Dir::BTT,
                0.0,
            ),
            400.0,
        );
        ys(&images(&d)[0], &[45.0, 35.0, 5.0]);
    }
    #[test]
    fn stack_image_horizontal_uses_logical_width_and_geometric_top() {
        for (dir, xs) in [(Dir::LTR, [5.0, 42.0, 89.0]), (Dir::RTL, [74.0, 27.0, 5.0])] {
            let d = doc(
                stack(
                    vec![photo(30.0, 20.0), photo(40.0, 10.0), photo(15.0, 30.0)],
                    dir,
                    7.0,
                ),
                400.0,
            );
            let p = images(&d);
            ys(&p[0], &[5.0, 5.0, 5.0]);
            for ((x, _), expected) in p[0].iter().zip(xs) {
                assert!((x - expected).abs() < 1e-9);
            }
        }
    }
    #[test]
    fn stack_image_spacing_and_invisible_frame_keep_extent() {
        for spacing in [0.0, 7.0, -3.0] {
            let d = doc(
                stack(
                    vec![
                        photo(30.0, 20.0),
                        Content::hide(photo(30.0, 20.0)),
                        photo(30.0, 20.0),
                    ],
                    Dir::TTB,
                    spacing,
                ),
                400.0,
            );
            ys(&images(&d)[0], &[5.0, 45.0 + 2.0 * spacing]);
        }
    }
    #[test]
    fn stack_image_twenty_children_fragment_with_reverse_coordinates() {
        for dir in [Dir::TTB, Dir::BTT] {
            let d = doc(stack(pics(20), dir, 0.0), 400.0);
            let p = images(&d);
            assert_eq!(p.iter().map(Vec::len).collect::<Vec<_>>(), vec![19, 1]);
            ys(
                &p[0],
                &(0..19)
                    .map(|i| 5.0 + 20.0 * if dir == Dir::TTB { i } else { 18 - i } as f64)
                    .collect::<Vec<_>>(),
            );
            ys(&p[1], &[5.0]);
        }
    }
    #[test]
    fn stack_image_interchild_spacing_is_trimmed_at_region_break() {
        let d = doc(stack(pics(20), Dir::TTB, 7.0), 400.0);
        let p = images(&d);
        assert_eq!(p.iter().map(Vec::len).collect::<Vec<_>>(), vec![14, 6]);
        for page in &p {
            ys(page, &(0..page.len()).map(|i| 5.0 + 27.0 * i as f64).collect::<Vec<_>>());
        }
    }
    #[test]
    fn stack_image_block_neighbors_use_physical_end() {
        let body = Content::Sequence(
            vec![photo(30.0, 20.0), stack(pics(2), Dir::TTB, 0.0), photo(30.0, 20.0)]
                .into(),
        );
        ys(&images(&doc(body, 400.0))[0], &[5.0, 38.2, 58.2, 91.4]);
    }
    #[test]
    fn stack_image_horizontal_paragraph_boundary_is_closed_once() {
        use crate::entities::elements::h_space::{HSpaceElem, Spacing};
        let h = Content::HSpace(Arc::new(HSpaceElem {
            amount: Spacing::Absolute(Length::pt(1.0)),
            weak: false,
            weak_explicit: false,
        }));
        let body = Content::Sequence(
            vec![
                photo(30.0, 20.0),
                h.clone(),
                stack(pics(2), Dir::TTB, 0.0),
                h,
                photo(30.0, 20.0),
            ]
            .into(),
        );
        ys(&images(&doc(body, 400.0))[0], &[5.0, 51.4, 71.4, 117.8]);
    }
    #[test]
    fn stack_image_text_children_use_text_edge_frame_height() {
        let d = doc(
            stack(
                vec![
                    Content::text("A"),
                    photo(30.0, 20.0),
                    Content::text("A"),
                    photo(30.0, 20.0),
                ],
                Dir::TTB,
                0.0,
            ),
            400.0,
        );
        ys(&images(&d)[0], &[12.7, 40.4]);
        assert_eq!(d.plain_text(), "A A");
    }
    #[test]
    fn stack_image_exact_fit_does_not_page_for_lower_margin() {
        let d = doc(stack(pics(1), Dir::TTB, 0.0), 30.0);
        assert_eq!(d.pages.len(), 1);
        ys(&images(&d)[0], &[5.0]);
    }
    #[test]
    fn stack_image_clip_target_width_does_not_use_drawing_width() {
        let cover = || {
            Content::Image(Arc::new(ImageElem {
                path: String::new(),
                alt: None,
                data: PtrEqArc(Arc::new(vec![])),
                width: Some(Box::new(Value::Float(10.0))),
                height: Some(Box::new(Value::Float(40.0))),
                fit: "cover".into(),
            }))
        };
        let d = doc(stack(vec![cover(), cover()], Dir::LTR, 0.0), 400.0);
        let p = images(&d);
        ys(&p[0], &[5.0, 5.0]);
        assert!((p[0][1].0 - p[0][0].0 - 10.0).abs() < 1e-9);
    }
    #[test]
    fn stack_image_horizontal_text_control_preserved() {
        let d = doc(
            stack(vec![Content::text("A"), Content::text("B")], Dir::LTR, 7.0),
            400.0,
        );
        assert_eq!(d.plain_text(), "A B");
        let positions: Vec<_> = d.pages[0]
            .items
            .iter()
            .filter_map(|i| match i {
                FrameItem::Text { pos, .. } | FrameItem::TextShaped { pos, .. } => {
                    Some(*pos)
                }
                _ => None,
            })
            .collect();
        assert_eq!(positions.len(), 2);
        assert!((positions[0].y.0 - positions[1].y.0).abs() < 1e-9);
        assert!((positions[1].x.0 - positions[0].x.0 - 13.6).abs() < 1e-9);
    }
    #[test]
    fn stack_image_original_locations_follow_fragmented_child_frames() {
        use crate::entities::content::{LocatedContent, RealizedContent};
        use crate::entities::elements::metadata::MetadataElem;
        use crate::entities::locator::Locator;
        let mut locator = Locator::new();
        let locations: Vec<_> = (0..20).map(|_| locator.next()).collect();
        let originals: Vec<_> = locations
            .iter()
            .map(|&location| {
                Content::Located(Arc::new(LocatedContent {
                    location,
                    body: Content::Metadata(Arc::new(MetadataElem {
                        value: Box::new(Value::None),
                    })),
                }))
            })
            .collect();
        let children = originals
            .iter()
            .map(|original| {
                Content::Realized(Arc::new(RealizedContent {
                    original: original.clone(),
                    visual: photo(30.0, 20.0),
                }))
            })
            .collect();
        let d = doc(
            Content::Realized(Arc::new(RealizedContent {
                original: Content::Sequence(originals.into()),
                visual: stack(children, Dir::TTB, 0.0),
            })),
            400.0,
        );
        assert_eq!(d.pages.len(), 2);
        for (index, location) in locations.into_iter().enumerate() {
            let position = d.extracted_positions.position_of(location).unwrap();
            assert_eq!(position.page.get(), if index < 19 { 1 } else { 2 });
            assert!(
                (position.point.y.0 - (5.0 + (index % 19) as f64 * 20.0)).abs() < 1e-9
            );
        }
    }
    #[test]
    fn stack_image_closed_text_tail_does_not_reuse_earlier_block_bottom() {
        use crate::entities::elements::linebreak::LinebreakElem;
        let child = Content::Sequence(
            vec![
                photo(30.0, 20.0),
                Content::text("g"),
                Content::Linebreak(Arc::new(LinebreakElem {
                    justify: false,
                    justify_explicit: false,
                })),
            ]
            .into(),
        );
        let child = Content::styled(
            Box::new(child),
            crate::entities::style::Styles::from_delta(
                crate::entities::style_chain::StyleDelta {
                    bottom_edge: Some(crate::entities::layout_types::TextEdge::Metric(
                        "descender".into(),
                    )),
                    ..crate::entities::style_chain::StyleDelta::empty()
                },
            ),
        );
        let d = doc(stack(vec![child, photo(30.0, 20.0)], Dir::TTB, 0.0), 400.0);
        let baseline = d.pages[0]
            .items
            .iter()
            .find_map(|item| match item {
                FrameItem::Text { pos, text, .. }
                | FrameItem::TextShaped { pos, text, .. }
                    if text.as_str() == "g" =>
                {
                    Some(pos.y.0)
                }
                _ => None,
            })
            .unwrap();
        assert!(
            images(&d)[0][1].1 >= baseline + 2.2,
            "next image {:?}, baseline {baseline}",
            images(&d)[0][1]
        );
    }

    fn display_math(text: &str) -> Content {
        Content::equation(Content::MathText(text.into()), true)
    }
    fn formula_xs(d: &PagedDocument) -> Vec<f64> {
        d.pages[0]
            .items
            .iter()
            .filter_map(|item| match item {
                FrameItem::Semantic {
                    kind: crate::entities::layout_types::SemanticKind::Formula,
                    items,
                    ..
                } => items
                    .first()
                    .map(|item| crate::compiler::layout::helpers::item_pos(item).0),
                _ => None,
            })
            .collect()
    }
    fn math_box(body: Content, width: Option<f64>) -> Content {
        Content::Boxed(Arc::new(crate::entities::elements::boxed::BoxedElem {
            body,
            width: width.map(Length::pt),
            width_fraction: None,
            height: None,
            inset: crate::entities::sides::Sides::uniform(Length::ZERO),
            baseline: Length::ZERO,
            outset: crate::entities::sides::Sides::uniform(Length::ZERO),
            radius: crate::entities::corners::Corners::uniform(Length::ZERO),
            clip: false,
            fill: None,
            stroke: None,
        }))
    }
    fn near(actual: f64, expected: f64) {
        assert!((actual - expected).abs() < 1e-9, "{actual} != {expected}");
    }
    #[test]
    fn math_stack_width_natural_advance_independent_of_constraint() {
        for width in [200.0, 400.0] {
            let d = doc_width(
                stack(vec![display_math("x"), photo(30.0, 20.0)], Dir::LTR, 7.0),
                width,
                400.0,
            );
            near(images(&d)[0][0].0, 18.6);
            near(formula_xs(&d)[0], 5.0);
        }
    }
    #[test]
    fn math_stack_width_reverse_placement_uses_natural_frame() {
        let d =
            doc(stack(vec![display_math("x"), photo(30.0, 20.0)], Dir::RTL, 7.0), 400.0);
        near(images(&d)[0][0].0, 5.0);
        near(formula_xs(&d)[0], 42.0);
    }
    #[test]
    fn math_stack_width_different_display_lines_center_in_completed_child() {
        let body = Content::sequence(vec![display_math("x"), display_math("xxx")]);
        let d = doc(stack(vec![body, photo(30.0, 20.0)], Dir::LTR, 7.0), 400.0);
        near(images(&d)[0][0].0, 31.8);
        let xs = formula_xs(&d);
        assert_eq!(xs.len(), 2);
        near(xs[0], 11.6);
        near(xs[1], 5.0);
    }
    #[test]
    fn math_stack_width_compound_text_owns_final_centering_width() {
        let body = Content::sequence(vec![
            display_math("x"),
            Content::Parbreak,
            Content::Text("Wide".into()),
        ]);
        let d = doc(stack(vec![body, photo(30.0, 20.0)], Dir::LTR, 7.0), 400.0);
        near(images(&d)[0][0].0, 38.4);
        near(formula_xs(&d)[0], 14.9);
    }
    #[test]
    fn math_stack_width_auto_box_shrinks_but_explicit_box_retains_allocation() {
        for (width, expected) in [(None, 18.6), (Some(100.0), 112.0)] {
            let d = doc(
                stack(
                    vec![math_box(display_math("x"), width), photo(30.0, 20.0)],
                    Dir::LTR,
                    7.0,
                ),
                400.0,
            );
            near(images(&d)[0][0].0, expected);
        }
    }
    #[test]
    fn math_stack_width_numbered_equation_retains_finite_slot() {
        let numbered = Content::styled(
            Box::new(display_math("x")),
            crate::entities::style::Styles::new()
                .push_custom("equation.numbering", Value::Str("(1)".into())),
        );
        let d = doc(stack(vec![numbered, photo(30.0, 20.0)], Dir::LTR, 7.0), 400.0);
        near(images(&d)[0][0].0, 402.0);
    }
    #[test]
    fn math_stack_width_exterior_equation_keeps_page_centering() {
        let d = doc(
            Content::sequence(vec![
                stack(vec![display_math("x"), photo(30.0, 20.0)], Dir::LTR, 7.0),
                display_math("z"),
            ]),
            400.0,
        );
        let xs = formula_xs(&d);
        assert_eq!(xs.len(), 2);
        near(xs[0], 5.0);
        near(xs[1], 196.7);
    }
    #[test]
    fn math_stack_width_invisible_mathematics_keeps_logical_advance() {
        let d = doc(
            stack(
                vec![Content::hide(display_math("x")), photo(30.0, 20.0)],
                Dir::LTR,
                7.0,
            ),
            400.0,
        );
        near(images(&d)[0][0].0, 18.6);
    }
    #[test]
    fn math_stack_width_preserves_finite_prose_wrapping() {
        let prose = Content::Text("word ".repeat(20).into());
        let control = doc_width(
            stack(vec![prose.clone(), photo(30.0, 20.0)], Dir::LTR, 7.0),
            100.0,
            400.0,
        );
        let width = images(&control)[0][0].0 - 12.0;
        assert!(width > 6.6 && width < 90.0, "constrained prose width {width}");
        let body = Content::sequence(vec![prose, Content::Parbreak, display_math("x")]);
        let d =
            doc_width(stack(vec![body, photo(30.0, 20.0)], Dir::LTR, 7.0), 100.0, 400.0);
        near(images(&d)[0][0].0, images(&control)[0][0].0);
        near(formula_xs(&d)[0], 5.0 + (width - 6.6) / 2.0);
    }

    fn rtl_content(body: Content) -> Content {
        Content::styled(
            Box::new(body),
            crate::entities::style::Styles::new()
                .push_custom("text.dir", Value::Dir(Dir::RTL)),
        )
    }
    #[test]
    fn math_stack_width_rtl_exterior_places_the_completed_frame_once() {
        for (dir, image_x, equation_x) in
            [(Dir::LTR, 365.0, 351.4), (Dir::RTL, 351.4, 388.4)]
        {
            let d = doc(
                rtl_content(stack(vec![display_math("x"), photo(30.0, 20.0)], dir, 7.0)),
                400.0,
            );
            near(images(&d)[0][0].0, image_x);
            near(formula_xs(&d)[0], equation_x);
        }
    }
    #[test]
    fn math_stack_width_nested_rtl_does_not_repeat_exterior_slack() {
        let inner = stack(vec![display_math("x"), photo(30.0, 20.0)], Dir::LTR, 7.0);
        let d =
            doc(rtl_content(stack(vec![inner, photo(30.0, 20.0)], Dir::LTR, 7.0)), 400.0);
        let p = images(&d);
        assert_eq!(p[0].len(), 2);
        near(p[0][0].0, 328.0);
        near(p[0][1].0, 365.0);
        near(formula_xs(&d)[0], 314.4);
    }
    fn nested_formula_positions(items: &[FrameItem]) -> Vec<(f64, f64)> {
        use crate::entities::layout_types::{SemanticKind, TransformMatrix};
        fn walk(items: &[FrameItem], matrix: TransformMatrix, out: &mut Vec<(f64, f64)>) {
            for item in items {
                match item {
                    FrameItem::Group { pos, matrix: local, items, .. } => {
                        let transform = matrix
                            .concat(&TransformMatrix::translate(pos.x.0, pos.y.0))
                            .concat(local);
                        walk(items, transform, out);
                    }
                    FrameItem::Semantic {
                        kind: SemanticKind::Formula, items, ..
                    } => {
                        if let Some(first) = items.first() {
                            let (x, y) =
                                crate::compiler::layout::helpers::item_pos(first);
                            out.push(matrix.apply(x, y));
                        }
                    }
                    FrameItem::Semantic { items, .. }
                    | FrameItem::Link { items, .. }
                    | FrameItem::TextReplacement { items, .. } => {
                        walk(items, matrix, out)
                    }
                    FrameItem::Text { .. }
                    | FrameItem::TextShaped { .. }
                    | FrameItem::TextSeparator { .. }
                    | FrameItem::Line { .. }
                    | FrameItem::Image { .. }
                    | FrameItem::Shape { .. }
                    | FrameItem::Glyph { .. } => {}
                }
            }
        }
        let mut result = Vec::new();
        walk(items, TransformMatrix::identity(), &mut result);
        result
    }
    #[test]
    fn math_stack_width_block_inset_retains_its_local_math_origin() {
        let zero = Length::ZERO;
        let block =
            Content::Block(Arc::new(crate::entities::elements::block::BlockElem {
                body: display_math("x"),
                width: None,
                height: None,
                inset: crate::entities::sides::Sides::uniform(Length::pt(3.0)),
                breakable: true,
                outset: crate::entities::sides::Sides::uniform(zero),
                radius: crate::entities::corners::Corners::uniform(zero),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            }));
        let d = doc(stack(vec![block, photo(30.0, 20.0)], Dir::LTR, 7.0), 400.0);
        near(images(&d)[0][0].0, 24.6);
        let positions = nested_formula_positions(&d.pages[0].items);
        assert_eq!(positions.len(), 1);
        let position = positions[0];
        near(position.0, 8.0);
        near(position.1, 8.0);
    }
}

#[cfg(test)]
mod issue_8790_inset_tests {
    use crate::compiler::layout::{FixedMetrics, Layouter};
    use crate::entities::{
        content::Content,
        dir::Dir,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{Color, FrameItem, Length, PageMarginSpec, PageMargins, Pt},
        rel::Rel,
        sides::Sides,
    };
    use comemo::Track;
    use std::sync::Arc;

    fn block(body: Content, width: Option<f64>, inset: Sides<Length>) -> Content {
        let Content::Block(mut e) = Content::block(
            body,
            width.map(Length::pt),
            Some(Length::pt(70.0)),
            inset,
            false,
        ) else {
            unreachable!()
        };
        let elem = Arc::make_mut(&mut e);
        elem.above = Some(Length::ZERO);
        elem.below = Some(Length::ZERO);
        Content::Block(e)
    }
    fn stack(dir: Dir) -> Content {
        let children = (1..=5)
            .map(|i| {
                let Content::Block(mut e) = Content::block(
                    Content::text("X"),
                    None,
                    Some(Length::pt(22.0)),
                    Sides::uniform(Length::ZERO),
                    true,
                ) else {
                    unreachable!()
                };
                let child = Arc::make_mut(&mut e);
                child.width = Some(Rel::<Length>::from_percent(100.0));
                child.fill = Some(Color::rgb(i, 0, 0));
                child.above = Some(Length::ZERO);
                child.below = Some(Length::ZERO);
                Content::Block(e)
            })
            .collect();
        Content::stack(children, dir, None)
    }
    fn widths(content: Content, shift: f64, following: bool) -> Vec<f64> {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            9.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = 400.0;
        l.page_config.height = 500.0;
        l.page_config.margin = PageMargins::uniform(10.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(10.0);
        l.regions.current.width = 400.0;
        l.regions.current.height = 500.0;
        l.regions.current.line_start_x = Pt(10.0 + shift);
        l.regions.current.cursor_x = Pt(10.0 + shift);
        l.regions.current.cursor_y = Pt(10.0);
        l.layout_content(&content);
        if following {
            l.layout_content(&stack(Dir::TTB));
        }
        l.finish()
            .pages
            .iter()
            .flat_map(|page| {
                super::super::tests::frame_items_recursive(&page.items)
                    .into_iter()
                    .filter_map(|item| match item {
                        FrameItem::Shape { width, height, fill: Some(_), .. }
                            if (*height - 22.0).abs() < 1e-8 =>
                        {
                            Some(*width)
                        }
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
    fn inset(left: f64, right: f64) -> Sides<Length> {
        let mut inset = Sides::uniform(Length::ZERO);
        inset.left = Length::pt(left);
        inset.right = Length::pt(right);
        inset
    }
    #[test]
    fn issue_8790_inset_local_width_in_four_directions() {
        for dir in [Dir::TTB, Dir::BTT, Dir::LTR, Dir::RTL] {
            for (left, right, expected) in [
                (2.0, 2.0, 136.0),
                (7.0, 11.0, 122.0),
                (0.0, 2.0, 138.0),
                (0.0, 0.0, 140.0),
            ] {
                assert_eq!(
                    widths(
                        block(stack(dir), Some(140.0), inset(left, right)),
                        0.0,
                        false
                    ),
                    vec![expected; 5],
                    "{dir:?}, inset={left}/{right}"
                );
            }
        }
    }
    #[test]
    fn issue_8790_inset_width_is_invariant_under_local_translation() {
        for shift in [0.0, 30.0] {
            assert_eq!(
                widths(
                    block(stack(Dir::BTT), Some(140.0), inset(2.0, 2.0)),
                    shift,
                    false
                ),
                vec![136.0; 5],
                "shift={shift}"
            );
        }
    }
    #[test]
    fn issue_8790_inset_nested_auto_and_outer_restoration() {
        let nested = block(stack(Dir::BTT), None, inset(0.0, 0.0));
        let mut expected = vec![136.0; 5];
        expected.extend(vec![380.0; 5]);
        assert_eq!(
            widths(block(nested, Some(140.0), inset(2.0, 2.0)), 0.0, true),
            expected
        );
    }
}
