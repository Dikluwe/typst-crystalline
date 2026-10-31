//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/shape.md
//! @prompt-hash 9cdf037c
//! @layer L1
//! @updated 2026-09-27
//!
//! Atomização (ADR-0109, P377): o layout de `Shape` movido do monólito
//! `layout_content` para o arquivo da feature (forma B — free function na
//! camada de render). Content-preserving — a lógica é idêntica.
//!
//! P767a — `Content::Shape` comporta-se como bloco no fluxo principal,
//! replicando `BlockElem::single_layouter` do vanilla.
//! P767c — ancoramento vertical corrigido quando a forma sucede texto não-bloco.

mod rectangle;

use crate::entities::elements::shape::ShapeElem;
use crate::entities::geometry::ShapeKind;
use crate::entities::layout_types::{FrameItem, Length, Point, Pt};

#[cfg(test)]
use crate::entities::paint::Paint;

use super::{resolve_pt, resolve_shape_kind, FontMetrics, ImageSizer, Layouter};

pub(super) fn resolve_relative_line(
    line: crate::entities::elements::shape::RelativeLine,
    width: f64,
    height: f64,
    font: f64,
) -> (ShapeKind, f64, f64) {
    let resolve = |point: [crate::entities::rel::Rel<Length>; 2]| Point {
        x: Pt(point[0].abs.resolve_pt(font)
            + if point[0].rel == 0.0 { 0.0 } else { point[0].rel * width }),
        y: Pt(point[1].abs.resolve_pt(font)
            + if point[1].rel == 0.0 { 0.0 } else { point[1].rel * height }),
    };
    let start = resolve(line.start);
    let end = resolve(line.end);
    (
        ShapeKind::Path(vec![
            crate::entities::geometry::PathItem::MoveTo(start),
            crate::entities::geometry::PathItem::LineTo(end),
        ]),
        0.0_f64.max(start.x.0).max(end.x.0),
        0.0_f64.max(start.y.0).max(end.y.0),
    )
}

/// **P767a** — espaçamento por defeito de um bloco de forma, equivalente ao
/// `BlockElem::spacing` por defeito do vanilla (`BLOCK_SPACING`).
const SHAPE_BLOCK_SPACING_EM: f64 = super::vanilla_defaults::BLOCK_SPACING;

/// Intrinsic defaults belong to the empty rectangle, independently of its paint.
pub(super) fn intrinsic_defaults(
    kind: &ShapeKind,
    width: Option<&crate::entities::value::Value>,
    height: Option<&crate::entities::value::Value>,
    available_w: f64,
    available_h: f64,
) -> (Option<f64>, Option<f64>) {
    let automatic = |value: Option<&crate::entities::value::Value>| {
        value.is_none() || matches!(value, Some(crate::entities::value::Value::Auto))
    };
    match kind {
        ShapeKind::Rect | ShapeKind::RoundedRect { .. } => {
            // ref: lab/typst-original/crates/typst-layout/src/shapes.rs:542-558
            let width = automatic(width).then(|| 45.0_f64.min(available_w).max(0.0));
            let height = automatic(height).then(|| 30.0_f64.min(available_h).max(0.0));
            (width, height)
        }
        ShapeKind::Ellipse | ShapeKind::Path(_) | ShapeKind::Line { .. } => (None, None),
    }
}

pub(super) fn resolve_dimension(
    value: Option<&crate::entities::value::Value>,
    reference: f64,
    font: f64,
    fallback: f64,
) -> f64 {
    use crate::entities::value::Value;
    let relative = |ratio: f64| {
        let value = ratio * reference;
        if value.is_finite() {
            value
        } else {
            0.0
        }
    };
    if let Some(Value::Length(length)) = value {
        return length.resolve_pt(font);
    }
    if let Some(Value::Ratio(ratio)) = value {
        return relative(ratio.0);
    }
    if let Some(Value::Relative(length)) = value {
        return relative(length.rel) + length.abs.resolve_pt(font);
    }
    resolve_pt(value, fallback)
}

#[cfg(test)]
mod prepared_paint_regressions {
    use super::*;
    use crate::compiler::layout::FixedMetrics;
    use crate::entities::elements::image::ImageElem;
    use crate::entities::geometry::Stroke;
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use crate::entities::layout_types::Size;
    use crate::entities::ptr_eq_arc::PtrEqArc;
    use crate::entities::tiling::{Tiling, TilingBody};
    use crate::entities::value::Value;
    use comemo::Track;
    use std::sync::Arc;

    struct CountImage(std::cell::Cell<usize>);
    impl ImageSizer for CountImage {
        fn size(&self, _: &[u8]) -> Option<(u32, u32)> {
            self.0.set(self.0.get() + 1);
            Some((8, 6))
        }
        fn dpi(&self, _: &[u8]) -> Option<f64> {
            Some(72.0)
        }
        fn orientation(&self, _: &[u8]) -> Option<u32> {
            Some(1)
        }
    }

    #[test]
    fn prepared_shape_transports_fill_and_stroke_without_a_repetition_grid() {
        let intr = TagIntrospector::empty();
        let intr: &dyn Introspector = &intr;
        for width in [40.0, 200_000.0] {
            let mut layouter = Layouter::new(
                FixedMetrics,
                CountImage(std::cell::Cell::new(0)),
                12.0,
                intr.track(),
            );
            layouter.is_sub_frame = true;
            let mut tile = Tiling::new(TilingBody::Image(ImageElem {
                path: "cell.png".into(),
                alt: None,
                data: PtrEqArc(Arc::new(vec![1, 2, 3])),
                width: None,
                height: None,
                fit: "cover".into(),
            }));
            tile.size = Some(Size { width: Pt(8.0), height: Pt(6.0) });
            let paint = Paint::Tiling(tile);
            let elem = ShapeElem {
                origin: crate::entities::elements::shape::ShapeOrigin::Geometry,
                rect_radius: None,
                rect_stroke: None,
                relative_line: None,
                kind: ShapeKind::Rect,
                width: Some(Box::new(Value::Length(Length::pt(width)))),
                height: Some(Box::new(Value::Length(Length::pt(20.0)))),
                fill: Some(paint.clone()),
                stroke: Some(Stroke {
                    paint,
                    thickness: crate::entities::layout_types::Length::pt(3.0),
                    ..Default::default()
                }),
                fill_rule: crate::entities::geometry::FillRule::EvenOdd,
            };
            layout(&mut layouter, &elem);
            assert_eq!(
                layouter.sizer.0.get(),
                2,
                "each paint occurrence must prepare its cell exactly once"
            );
            assert!(layouter.layout_errors.is_empty());
            assert_eq!(layouter.regions.current.current_items.len(), 1);
            let FrameItem::Shape {
                width: actual_width,
                height,
                fill,
                stroke,
                fill_rule,
                ..
            } = &layouter.regions.current.current_items[0]
            else {
                panic!("shape paint must not expand into Group copies");
            };
            assert_eq!((*actual_width, *height), (width, 20.0));
            assert!(fill.is_some());
            assert_eq!(stroke.as_ref().unwrap().thickness, 3.0);
            assert_eq!(*fill_rule, crate::entities::geometry::FillRule::EvenOdd);
        }
    }
}

/// Layout de uma forma (`rect`/`circle`/`line`/…): resolve dimensões, emite
/// um `FrameItem::Shape` na página actual e avança o cursor vertical.
///
/// **P767a** — no fluxo principal as formas comportam-se como blocos:
/// quebram o parágrafo corrente e aplicam `above`/`below` de 1.2em com
/// colapso de margem, replicando `BlockElem::single_layouter` do vanilla.
/// Em sub-frames (`place`, células de grid, etc.) preserva-se o
/// comportamento anterior para não afectar posicionamento absoluto.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &ShapeElem,
) {
    // A fresh block starts at a geometric top, including root Place.
    // Legacy baseline-origin subframes arrive with the pending flag cleared.
    let fresh_block = layouter.initial_baseline_pending
        && layouter
            .regions
            .current
            .current_line
            .iter()
            .all(super::html_frame::is_context_marker);
    if fresh_block {
        layouter.initial_baseline_pending = false;
    } else if layouter.initial_baseline_pending {
        let (top, _) = layouter.metrics.text_edges(layouter.style.size, &layouter.style);
        layouter.regions.current.cursor_y += top;
        layouter.initial_baseline_pending = false;
    }
    let (kind, width, height, fill, stroke) =
        (&e.kind, &e.width, &e.height, &e.fill, &e.stroke);
    let available_w = layouter.available_width();
    let relative_width = layouter.block_place_region.map_or(available_w, |r| r.w.0);
    let font = layouter.style.size.val();
    let relative_height = layouter
        .block_place_region
        .filter(|r| r.h.0.is_finite())
        .map(|r| r.h.0)
        .or(layouter.relative_region_size.map(|size| size.1))
        .unwrap_or_else(|| layouter.available_height());
    let relative_geometry = e
        .relative_line
        .map(|line| resolve_relative_line(line, relative_width, relative_height, font));
    let kind = relative_geometry.as_ref().map_or(kind, |geometry| &geometry.0);
    let (resolved_w, resolved_h) = if let Some((_, width, height)) = &relative_geometry {
        (*width, *height)
    } else {
        match kind {
            // P242 — RoundedRect partilha dimensões com Rect.
            ShapeKind::Rect
            | ShapeKind::RoundedRect { .. }
            | ShapeKind::Ellipse
            | ShapeKind::Path(_) => {
                let (default_w, default_h) = intrinsic_defaults(
                    kind,
                    width.as_deref(),
                    height.as_deref(),
                    relative_width,
                    relative_height,
                );
                let w = resolve_dimension(
                    width.as_deref(),
                    relative_width,
                    font,
                    default_w.unwrap_or(available_w),
                );
                let h = resolve_dimension(
                    height.as_deref(),
                    relative_height,
                    font,
                    default_h.unwrap_or(0.0),
                );
                (w, h)
            }
            ShapeKind::Line { dx, dy } => (dx.abs(), dy.abs()),
        }
    };

    // **P767c** — guardar a baseline da linha que vai ser descarregada.
    // Após `flush_line`, o cursor aponta para a baseline da *próxima* linha;
    // para ancorar a forma na grelha de linhas do parágrafo actual, usamos
    // a baseline *antes* do avanço.
    let baseline_before_flush = layouter.regions.current.cursor_y;
    layouter.flush_line();
    let cursor_after_flush = layouter.regions.current.cursor_y;
    let had_text_line = cursor_after_flush.0 > baseline_before_flush.0 + 1e-6;

    // **P767a** — protocolo de bloco no fluxo principal.
    let in_main_flow = !layouter.is_sub_frame;
    let above_pt = Length::em(SHAPE_BLOCK_SPACING_EM).resolve_pt(font);
    let below_pt = Length::em(SHAPE_BLOCK_SPACING_EM).resolve_pt(font);
    let cap_height =
        if fresh_block || !in_main_flow || layouter.block_chain_active || had_text_line {
            Pt(0.0)
        } else {
            layouter.metrics.cap_height(layouter.style.size, &layouter.style)
        };

    if in_main_flow {
        // A forma de bloco alinha-se à margem esquerda do contentor.
        layouter.regions.current.cursor_x = layouter.regions.current.line_start_x;
    }

    // A shape occupies a geometric frame. A preceding text line supplies its
    // baseline; a preceding block supplies its physical end and pending gap.
    // Only the legacy fresh-region cursor still carries an assumed text top.
    let candidate_shape_base = if !in_main_flow {
        layouter.regions.current.cursor_y
    } else if had_text_line {
        baseline_before_flush + Pt(above_pt)
    } else if layouter.block_chain_active {
        Pt(layouter.prev_line_baseline
            + layouter.prev_block_equation_descent
            + layouter.prev_block_below_pending.max(above_pt))
    } else {
        layouter.regions.current.cursor_y - cap_height
    };
    let page_bottom =
        layouter.regions.current.height - layouter.page_config.margin.bottom;
    let shape_base =
        if !layouter.is_sub_frame && candidate_shape_base.0 + resolved_h > page_bottom {
            layouter.new_page();
            // The new region may prepare a text baseline. Recover its physical
            // top so an exact-fitting shape does not inherit that text offset.
            let page_top = if layouter.initial_baseline_pending {
                Pt(0.0)
            } else {
                layouter.metrics.text_edges(layouter.style.size, &layouter.style).0
            };
            layouter.regions.current.cursor_y - page_top
        } else {
            candidate_shape_base
        };
    let pos = Point {
        x: layouter.regions.current.cursor_x,
        y: shape_base,
    };
    let resolved_kind = resolve_shape_kind(kind, layouter.style.size);
    if !((e.rect_radius.is_some() || e.rect_stroke.is_some())
        && rectangle::emit(
            layouter,
            e,
            pos,
            resolved_w,
            resolved_h,
            resolved_kind.clone(),
        ))
    {
        let fill = super::tiling::resolve_paint(layouter, fill.as_ref());
        let stroke = super::tiling::resolve_stroke(layouter, stroke.as_ref());
        layouter.regions.current.current_items.push(FrameItem::Shape {
            pos,
            kind: resolved_kind,
            width: resolved_w,
            height: resolved_h,
            fill,
            stroke,
            fill_rule: e.fill_rule,
            parent_bbox_at_emit: layouter.parent_bbox,
        });
    }

    // The frame end is causal; below remains pending for the next block.
    if in_main_flow {
        let end = shape_base.0 + resolved_h;
        layouter.regions.current.cursor_y = Pt(end + below_pt);
        layouter.prev_line_baseline = end;
        layouter.last_block_descent_y = Some(end);
        layouter.prev_block_equation_descent = 0.0;
        layouter.prev_block_below_pending = below_pt;
        layouter.prev_margin_is_parbreak = false;
        layouter.block_chain_active = true;
    } else {
        layouter.regions.current.cursor_y += Pt(resolved_h);
    }
    layouter.record_flow_baseline(None);
    layouter.record_flow_placement_end(shape_base.0 + resolved_h);
    layouter.record_flow_width_at(pos.x.0, resolved_w);
}

#[cfg(test)]
mod initial_nontext_block_regressions {
    use super::*;
    use crate::compiler::layout::FixedMetrics;
    use crate::entities::image_sizer::NullImageSizer;
    use crate::entities::layout_types::{TextEdge, TextStyle};
    use comemo::Track;

    struct NoTypography;
    impl FontMetrics for NoTypography {
        fn advance(&self, _: &str, _: Pt, _: &TextStyle) -> Pt {
            panic!("unexpected text advance")
        }
        fn vertical_metrics(&self, _: Pt, _: &TextStyle) -> (Pt, Pt) {
            panic!("unexpected vertical metrics")
        }
        fn cap_height(&self, _: Pt, _: &TextStyle) -> Pt {
            panic!("unexpected cap height")
        }
        fn text_edges(&self, _: Pt, _: &TextStyle) -> (Pt, Pt) {
            panic!("unexpected text edges")
        }
    }

    fn check<M: FontMetrics>(metrics: M, edge: Option<TextEdge>, subframe: bool) {
        let intr = crate::entities::introspector::TagIntrospector::empty();
        let intr_dyn: &dyn crate::entities::introspector::Introspector = &intr;
        let mut layouter = Layouter::new(metrics, NullImageSizer, 12.0, intr_dyn.track());
        layouter.style.top_edge = edge;
        layouter.is_sub_frame = subframe;
        layouter.initial_baseline_pending = !subframe;
        let origin = layouter.regions.current.cursor_y.0;
        let elem = crate::entities::elements::shape::ShapeElem {
            origin: crate::entities::elements::shape::ShapeOrigin::Geometry,
            rect_radius: None,
            rect_stroke: None,
            relative_line: None,
            kind: crate::entities::geometry::ShapeKind::Rect,
            width: Some(Box::new(crate::entities::value::Value::Float(80.0))),
            height: Some(Box::new(crate::entities::value::Value::Float(40.0))),
            fill: None,
            stroke: None,
            fill_rule: Default::default(),
        };
        super::layout(&mut layouter, &elem);
        let y = match &layouter.regions.current.current_items[0] {
            FrameItem::Shape { pos, .. } => pos.y.0,
            other => panic!("unexpected first block {other:?}"),
        };
        assert!((y - origin).abs() < 1e-9, "block top {y} != physical origin {origin}");
        assert!(!layouter.initial_baseline_pending);
        assert_eq!(layouter.block_chain_active, !subframe);
    }

    #[test]
    fn initial_nontext_block_top_is_independent_of_text_edge() {
        for edge in [
            None,
            Some(TextEdge::Length(Length::pt(0.0))),
            Some(TextEdge::Length(Length::pt(20.0))),
        ] {
            check(FixedMetrics, edge, false);
        }
    }

    #[test]
    fn initial_nontext_block_does_not_acquire_typography() {
        check(NoTypography, None, false);
    }
    #[test]
    fn ordinary_subframe_block_does_not_query_unused_typography() {
        check(NoTypography, None, true);
    }
}

#[cfg(test)]
mod r13_tests {
    use super::*;
    use crate::entities::elements::shape::RelativeLine;
    use crate::entities::image_sizer::NullImageSizer;
    use crate::entities::rel::Rel;
    use comemo::Track;
    fn check(start: [Rel<Length>; 2], end: [Rel<Length>; 2], expected: [f64; 6]) {
        let intr = crate::entities::introspector::TagIntrospector::empty();
        let intr_dyn: &dyn crate::entities::introspector::Introspector = &intr;
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            intr_dyn.track(),
        );
        l.block_place_region = Some(crate::entities::layout_types::Rect {
            x: Pt::ZERO,
            y: Pt::ZERO,
            w: Pt(260.0),
            h: Pt(140.0),
        });
        let crate::entities::content::Content::Shape(mut e) =
            crate::entities::content::Content::shape(
                ShapeKind::Path(vec![]),
                None,
                None,
                None,
                None,
            )
        else {
            panic!("shape")
        };
        std::sync::Arc::make_mut(&mut e).relative_line =
            Some(RelativeLine { start, end });
        layout(&mut l, &e);
        let FrameItem::Shape { kind: ShapeKind::Path(path), width, height, .. } =
            &l.regions.current.current_items[0]
        else {
            panic!("path")
        };
        assert_eq!(path.len(), 2);
        let [crate::entities::geometry::PathItem::MoveTo(a), crate::entities::geometry::PathItem::LineTo(b)] =
            path.as_slice()
        else {
            panic!("endpoints")
        };
        for (a, b) in [a.x.0, a.y.0, b.x.0, b.y.0, *width, *height]
            .into_iter()
            .zip(expected)
        {
            assert!((a - b).abs() < 1e-9, "{a} != {b}");
        }
    }
    #[test]
    fn r13_layout_resolves_projected_axes_independently() {
        let c = libm::cos(std::f64::consts::PI / 6.0);
        check(
            [Rel::zero(); 2],
            [
                Rel { rel: 0.5 * c, abs: Length::ZERO },
                Rel { rel: 0.25, abs: Length::ZERO },
            ],
            [0.0, 0.0, 130.0 * c, 35.0, 130.0 * c, 35.0],
        );
    }
    #[test]
    fn r13_layout_resolves_mixed_and_em_at_effective_size() {
        check(
            [Rel::zero(); 2],
            [Rel { rel: 0.5, abs: Length::em(2.0) }, Rel::zero()],
            [0.0, 0.0, 152.0, 0.0, 152.0, 0.0],
        );
    }
    #[test]
    fn r13_layout_preserves_origin_and_causal_negative_bounds() {
        check(
            [Rel { rel: 0.0, abs: Length::pt(10.0) }, Rel::zero()],
            [Rel { rel: -0.5, abs: Length::ZERO }, Rel::zero()],
            [10.0, 0.0, -130.0, 0.0, 10.0, 0.0],
        );
    }
}

#[cfg(test)]
mod issue8976_tests {
    use super::*;
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::PageMargins,
        value::Value,
    };
    use comemo::Track;

    fn rect(width: Option<Value>, height: Option<Value>, rounded: bool) -> Content {
        Content::shape(
            if rounded {
                ShapeKind::RoundedRect {
                    radii: crate::entities::corners::Corners::uniform(Length::pt(3.0)),
                }
            } else {
                ShapeKind::Rect
            },
            width.map(Box::new),
            height.map(Box::new),
            None,
            None,
        )
    }

    fn observe(
        body: &Content,
        width: f64,
        height: f64,
        font: f64,
    ) -> ((f64, f64), (f64, f64)) {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            font,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = width;
        l.page_config.height = height;
        l.page_config.margin = PageMargins::uniform(0.0);
        l.regions.current.width = width;
        l.regions.current.height = height;
        l.regions.current.cursor_x = Pt(0.0);
        l.regions.current.line_start_x = Pt(0.0);
        l.regions.current.cursor_y = Pt(0.0);
        l.initial_baseline_pending = true;
        let measured = l.measure_content_constrained(body, width);
        l.layout_content(body);
        let doc = l.finish();
        assert_eq!(doc.pages.len(), 1);
        let emitted = doc.pages[0]
            .items
            .iter()
            .find_map(|item| match item {
                FrameItem::Shape { width, height, .. } => Some((*width, *height)),
                _ => None,
            })
            .expect("rectangle must be emitted");
        (measured, emitted)
    }

    #[test]
    fn issue8976_omitted_and_auto_axes_have_intrinsic_frame() {
        for rounded in [false, true] {
            for auto in [false, true] {
                for font in [11.0, 24.0] {
                    let v = auto.then_some(Value::Auto);
                    let body = rect(v.clone(), v, rounded);
                    assert_eq!(
                        observe(&body, 200.0, 100.0, font),
                        ((45.0, 30.0), (45.0, 30.0))
                    );
                }
            }
        }
    }

    #[test]
    fn issue8976_default_axes_respect_small_and_infinite_regions() {
        for (w, h, expected) in [
            (20.0, 100.0, (20.0, 30.0)),
            (100.0, 15.0, (45.0, 15.0)),
            (f64::INFINITY, f64::INFINITY, (45.0, 30.0)),
        ] {
            assert_eq!(
                observe(&rect(None, None, false), w, h, 11.0),
                (expected, expected)
            );
        }
    }

    #[test]
    fn issue8976_each_explicit_axis_replaces_only_itself() {
        for (w, h, expected) in [
            (Some(Value::Length(Length::pt(70.0))), None, (70.0, 30.0)),
            (None, Some(Value::Length(Length::pt(18.0))), (45.0, 18.0)),
            (
                Some(Value::Length(Length::ZERO)),
                Some(Value::Length(Length::ZERO)),
                (0.0, 0.0),
            ),
        ] {
            assert_eq!(
                observe(&rect(w, h, false), 200.0, 100.0, 11.0),
                (expected, expected)
            );
        }
    }

    #[test]
    fn issue8976_explicit_and_relative_geometry_remain_unchanged() {
        for (w, h, expected) in [
            (
                Value::Length(Length::pt(45.0)),
                Value::Length(Length::pt(30.0)),
                (45.0, 30.0),
            ),
            (
                Value::Ratio(crate::entities::layout_types::Ratio(0.5)),
                Value::Ratio(crate::entities::layout_types::Ratio(0.25)),
                (100.0, 25.0),
            ),
            (
                Value::Length(Length::em(2.0)),
                Value::Length(Length::em(3.0)),
                (22.0, 33.0),
            ),
        ] {
            assert_eq!(
                observe(&rect(Some(w), Some(h), false), 200.0, 100.0, 11.0),
                (expected, expected)
            );
        }
    }
}
