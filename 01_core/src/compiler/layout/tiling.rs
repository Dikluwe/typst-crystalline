//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/tiling.md
//! @prompt-hash 7903d601
//! @layer L1

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::frame_paint::{FramePaint, FrameStroke, TilingFrame};
use crate::entities::geometry::{ShapeKind, Stroke};
use crate::entities::gradient::RelativeTo;
use crate::entities::image_sizer::ImageSizer;
use crate::entities::layout_types::{Frame, Length, Point, Pt, Rect, Size};
use crate::entities::paint::Paint;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::tiling::{Tiling, TilingBody, TilingRelative};
use crate::entities::value::Value;

#[cfg(test)]
use crate::entities::layout_types::FrameItem;

use super::{FontMetrics, Layouter, SubLayoutRegion};

/// Convert declarative paint only at the layout boundary.
pub(super) fn prepare_paint<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    paint: &Paint,
) -> SourceResult<FramePaint> {
    Ok(match paint {
        Paint::Solid(color) => FramePaint::Solid(color.clone()),
        Paint::Gradient(gradient) => FramePaint::Gradient(gradient.clone()),
        Paint::Tiling(tiling) => {
            FramePaint::Tiling(Arc::new(prepare_cell(layouter, tiling)?))
        }
    })
}

pub(super) fn prepare_stroke<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    stroke: &Stroke,
) -> SourceResult<FrameStroke> {
    Ok(FrameStroke {
        paint: prepare_paint(layouter, &stroke.paint)?,
        thickness: stroke.thickness,
        cap: stroke.cap,
        join: stroke.join,
        dash: stroke.dash.clone(),
        miter_limit: stroke.miter_limit,
        specified: stroke.specified,
        overhang: stroke.overhang,
    })
}

pub(super) fn resolve_paint<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    paint: Option<&Paint>,
) -> Option<FramePaint> {
    match paint.map(|paint| prepare_paint(layouter, paint)).transpose() {
        Ok(paint) => paint,
        Err(errors) => {
            layouter.layout_errors.extend(errors);
            None
        }
    }
}

pub(super) fn resolve_stroke<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    stroke: Option<&Stroke>,
) -> Option<FrameStroke> {
    match stroke.map(|stroke| prepare_stroke(layouter, stroke)).transpose() {
        Ok(stroke) => stroke,
        Err(errors) => {
            layouter.layout_errors.extend(errors);
            None
        }
    }
}

/// Prepare exactly one cell. Target coverage and exporter repetition are absent
/// from this boundary; the returned frame establishes an implicit paint root.
pub(super) fn prepare_cell<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    tiling: &Tiling,
) -> SourceResult<TilingFrame> {
    let explicit = tiling.size;
    if explicit.is_some_and(|size| !valid_size(size)) {
        return Err(cell_error("tile size must be non-zero and non-infinite"));
    }
    let spacing = tiling.spacing.unwrap_or(Size { width: Pt(0.0), height: Pt(0.0) });
    if !spacing.width.0.is_finite() || !spacing.height.0.is_finite() {
        return Err(cell_error("tile spacing must be finite"));
    }
    if tiling.offset.x.abs.em != 0.0 || tiling.offset.y.abs.em != 0.0 {
        return Err(cell_error("tile offset must not be font-relative"));
    }
    if !tiling.offset.x.rel.is_finite()
        || !tiling.offset.y.rel.is_finite()
        || !tiling.offset.x.abs.abs.0.is_finite()
        || !tiling.offset.y.abs.abs.0.is_finite()
    {
        return Err(cell_error("tile offset must be finite"));
    }
    if !tiling.angle.to_rad().is_finite() {
        return Err(cell_error("tile angle must be finite"));
    }
    if let Some(size) = explicit {
        validate_placement(tiling, size, spacing)?;
    }

    // Rust legacy bodies cross the same L1 boundary explicitly. They never
    // acquire a target dependency or representative-color conversion.
    let bridged;
    let body = match &tiling.body {
        TilingBody::Content(body) => body.as_ref(),
        TilingBody::Image(image) => {
            bridged = Content::Image(Arc::new(image.clone()));
            &bridged
        }
        TilingBody::Color(_) | TilingBody::Gradient(_) => {
            let Some(size) = explicit else {
                return Err(cell_error(
                    "tile size must be explicit for a color or gradient body",
                ));
            };
            let paint = match &tiling.body {
                TilingBody::Color(color) => Paint::Solid(color.clone()),
                TilingBody::Gradient(gradient) => Paint::Gradient(gradient.clone()),
                _ => unreachable!(),
            };
            bridged = Content::shape(
                ShapeKind::Rect,
                Some(Box::new(Value::Length(Length::pt(size.width.0)))),
                Some(Box::new(Value::Length(Length::pt(size.height.0)))),
                Some(paint),
                None,
            );
            &bridged
        }
    };

    // The flow checkpoint resets its own transaction envelope on restore.
    // Suspend the parent's envelope so cell layout cannot consume its replay
    // or reset the dispatch depth of a transaction currently on the stack.
    let outer_transaction = (
        std::mem::take(&mut layouter.flow_suffix_checkpoint_pending),
        std::mem::take(&mut layouter.flow_suffix_transaction_active),
        std::mem::take(&mut layouter.flow_suffix_transaction_rejected),
        layouter.flow_suffix_checkpoint.take(),
        std::mem::take(&mut layouter.flow_suffix_replay),
        std::mem::take(&mut layouter.flow_suffix_dispatch_depth),
    );
    let checkpoint = layouter.capture_flow_suffix_checkpoint();
    let errors_before = layouter.layout_errors.len();
    let warnings_before = layouter.layout_warnings.len();
    layouter.parent_bbox = explicit.map(|size| Rect {
        x: Pt(0.0),
        y: Pt(0.0),
        w: size.width,
        h: size.height,
    });
    let (body_height, body_items, _, _, _) = layouter
        .layout_sub_frame_at_geometric_origin(
            body,
            SubLayoutRegion {
                origin_x: 0.0,
                width: explicit.map_or(f64::INFINITY, |size| size.width.0),
                height: explicit.map(|size| size.height.0),
                align_rtl: false,
                unconstrained_height: explicit.is_none(),
            },
        );
    let body_width = layouter
        .last_sub_frame_flow_width
        .max(layouter.last_sub_frame_width)
        .max(super::helpers::line_content_right(&body_items, &layouter.metrics));
    let errors = layouter.layout_errors.split_off(errors_before);
    let warnings = layouter.layout_warnings.split_off(warnings_before);
    layouter.restore_flow_suffix_checkpoint(checkpoint);
    (
        layouter.flow_suffix_checkpoint_pending,
        layouter.flow_suffix_transaction_active,
        layouter.flow_suffix_transaction_rejected,
        layouter.flow_suffix_checkpoint,
        layouter.flow_suffix_replay,
        layouter.flow_suffix_dispatch_depth,
    ) = outer_transaction;
    layouter.layout_warnings.extend(warnings);
    if !errors.is_empty() {
        return Err(errors);
    }
    let size =
        explicit.unwrap_or(Size { width: Pt(body_width), height: Pt(body_height) });
    if !size.width.0.is_finite() || !size.height.0.is_finite() {
        return Err(cell_error("tile size must be non-zero and non-infinite"));
    }
    if size.width.0 <= 0.0 || size.height.0 <= 0.0 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "tile size must be non-zero",
        )
        .with_hint("try setting the size manually")]);
    }
    let offset = validate_placement(tiling, size, spacing)?;
    Ok(TilingFrame {
        frame: Frame { size, items: body_items },
        spacing,
        offset,
        angle: tiling.angle,
        relative: match tiling.relative {
            TilingRelative::Auto | TilingRelative::Itself => RelativeTo::Self_,
            TilingRelative::Parent => RelativeTo::Parent,
        },
    })
}

fn valid_size(size: Size) -> bool {
    size.width.0.is_finite()
        && size.height.0.is_finite()
        && size.width.0 > 0.0
        && size.height.0 > 0.0
}

fn cell_error(message: &str) -> Vec<SourceDiagnostic> {
    vec![SourceDiagnostic::error(Span::detached(), message)]
}

fn validate_placement(tiling: &Tiling, size: Size, spacing: Size) -> SourceResult<Point> {
    let pitch_x = size.width.0 + spacing.width.0;
    let pitch_y = size.height.0 + spacing.height.0;
    if !pitch_x.is_finite() || !pitch_y.is_finite() || pitch_x <= 0.0 || pitch_y <= 0.0 {
        return Err(cell_error("tile pitch must be positive and finite"));
    }
    let x = tiling.offset.x.rel * pitch_x + tiling.offset.x.abs.abs.0;
    let y = tiling.offset.y.rel * pitch_y + tiling.offset.y.abs.abs.0;
    if !x.is_finite() || !y.is_finite() {
        return Err(cell_error("tile offset must be finite"));
    }
    Ok(Point { x: Pt(x), y: Pt(y) })
}

#[cfg(test)]
mod preparation_regressions {
    use super::*;
    use crate::compiler::layout::FixedMetrics;
    use crate::entities::content::Content;
    use crate::entities::gradient::{Gradient, GradientStop};
    use crate::entities::image_sizer::NullImageSizer;
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use crate::entities::layout_types::{Angle, Color, Length, Ratio, Rect, Size};
    use crate::entities::paint::Paint;
    use crate::entities::value::Value;
    use comemo::Track;
    use std::sync::Arc;

    fn sized(body: TilingBody) -> Tiling {
        let mut tile = Tiling::new(body);
        tile.size = Some(Size { width: Pt(8.0), height: Pt(6.0) });
        tile
    }

    fn with_layout(f: impl FnOnce(&mut Layouter<FixedMetrics, NullImageSizer>)) {
        let intr = TagIntrospector::empty();
        let intr: &dyn Introspector = &intr;
        let mut layout = Layouter::new(FixedMetrics, NullImageSizer, 12.0, intr.track());
        f(&mut layout);
    }

    #[test]
    fn tiling_preparation_invalid_cell_produces_causal_diagnostic() {
        with_layout(|layout| {
            let mut tile = sized(TilingBody::Content(Arc::new(Content::Empty)));
            tile.size.as_mut().unwrap().width = Pt(0.0);
            let paint = resolve_paint(layout, Some(&Paint::Tiling(tile)));
            assert!(paint.is_none());
            assert_eq!(layout.layout_errors.len(), 1);
            assert_eq!(
                layout.layout_errors[0].message,
                "tile size must be non-zero and non-infinite"
            );
        });
    }

    #[test]
    fn tiling_preparation_legacy_color_and_gradient_keep_original_paints() {
        let color = Color::rgb(21, 67, 159);
        let gradient = Gradient::linear(
            vec![
                GradientStop::new(color.clone(), Ratio(0.0)),
                GradientStop::new(Color::rgb(211, 60, 9), Ratio(1.0)),
            ],
            Angle::deg(23.0),
        );
        for (body, expected) in [
            (TilingBody::Color(color.clone()), Paint::Solid(color)),
            (TilingBody::Gradient(gradient.clone()), Paint::Gradient(gradient)),
        ] {
            with_layout(|layout| {
                let cell = prepare_cell(layout, &sized(body)).expect("legacy paint cell");
                let FrameItem::Shape { pos, width, height, fill: Some(actual), .. } =
                    &cell.frame.items[0]
                else {
                    panic!("paint cell")
                };
                assert_eq!(*pos, Point::ZERO);
                assert_eq!((*width, *height), (8.0, 6.0));
                match (actual, &expected) {
                    (FramePaint::Solid(a), Paint::Solid(b)) => assert_eq!(a, b),
                    (FramePaint::Gradient(a), Paint::Gradient(b)) => assert_eq!(a, b),
                    _ => panic!("paint changed kind"),
                }
            });
        }
    }

    #[test]
    fn tiling_preparation_cell_parent_is_local_and_outer_state_is_preserved() {
        with_layout(|layout| {
            let outer = Rect { x: Pt(17.0), y: Pt(29.0), w: Pt(90.0), h: Pt(70.0) };
            layout.parent_bbox = Some(outer);
            let cursor =
                (layout.regions.current.cursor_x, layout.regions.current.cursor_y);
            let body = Content::shape(
                ShapeKind::Rect,
                Some(Box::new(Value::Length(Length::pt(2.0)))),
                Some(Box::new(Value::Length(Length::pt(3.0)))),
                Some(Paint::Solid(Color::rgb(21, 67, 159))),
                None,
            );
            let cell = prepare_cell(layout, &sized(TilingBody::Content(Arc::new(body))))
                .unwrap();
            let FrameItem::Shape { parent_bbox_at_emit, .. } = &cell.frame.items[0]
            else {
                panic!("body shape")
            };
            assert_eq!(
                *parent_bbox_at_emit,
                Some(Rect { x: Pt(0.0), y: Pt(0.0), w: Pt(8.0), h: Pt(6.0) })
            );
            assert_eq!(layout.parent_bbox, Some(outer));
            assert_eq!(
                (layout.regions.current.cursor_x, layout.regions.current.cursor_y),
                cursor
            );
        });
    }

    #[test]
    fn tiling_preparation_resolves_placement_and_keeps_one_cell() {
        with_layout(|layout| {
            let body = Content::shape(
                ShapeKind::Rect,
                Some(Box::new(Value::Length(Length::pt(2.0)))),
                Some(Box::new(Value::Length(Length::pt(3.0)))),
                Some(Paint::Solid(Color::rgb(21, 67, 159))),
                None,
            );
            let mut tile = sized(TilingBody::Content(Arc::new(body)));
            tile.spacing = Some(Size { width: Pt(-2.0), height: Pt(4.0) });
            tile.offset.x.rel = 0.5;
            tile.offset.x.abs = Length::pt(2.0);
            tile.offset.y.rel = -0.25;
            tile.angle = Angle::deg(37.0);
            tile.relative = TilingRelative::Parent;
            let cell = prepare_cell(layout, &tile).unwrap();
            assert_eq!(cell.frame.items.len(), 1, "transport must not expand a lattice");
            let FrameItem::Shape { pos, width, height, .. } = &cell.frame.items[0] else {
                panic!("body shape")
            };
            assert_eq!((*pos, *width, *height), (Point::ZERO, 2.0, 3.0));
            assert_eq!(cell.frame.size, Size { width: Pt(8.0), height: Pt(6.0) });
            assert_eq!(cell.spacing, tile.spacing.unwrap());
            assert_eq!(cell.offset, Point { x: Pt(5.0), y: Pt(-2.5) });
            assert_eq!(cell.angle, tile.angle);
            assert_eq!(cell.relative, RelativeTo::Parent);
            tile.size = None;
            tile.spacing = None;
            tile.relative = TilingRelative::Auto;
            let auto = prepare_cell(layout, &tile).unwrap();
            assert_eq!(auto.frame.size, Size { width: Pt(2.0), height: Pt(3.0) });
            assert_eq!(auto.relative, RelativeTo::Self_);
        });
    }

    #[test]
    fn tiling_preparation_empty_explicit_cell_is_valid_but_auto_needs_size() {
        with_layout(|layout| {
            let mut tile = sized(TilingBody::Content(Arc::new(Content::Empty)));
            let cell = prepare_cell(layout, &tile).unwrap();
            assert!(cell.frame.items.is_empty());
            tile.size = None;
            let errors = prepare_cell(layout, &tile).unwrap_err();
            assert_eq!(errors[0].message, "tile size must be non-zero");
            assert_eq!(errors[0].hints, ["try setting the size manually"]);
        });
    }

    #[test]
    fn tiling_preparation_labels_do_not_escape_the_cell() {
        with_layout(|layout| {
            use crate::entities::elements::label::LabelElem;
            use crate::entities::label::Label;
            let outer = Label("outside".into());
            layout.runtime.label_pages.insert(outer.clone(), 7);
            layout.runtime.label_positions.insert(outer.clone(), Point::ZERO);
            let body = Content::Label(Arc::new(LabelElem {
                name: "tile-only".into(),
                body: Content::text("inside"),
                auto: false,
            }));
            let cell = prepare_cell(layout, &sized(TilingBody::Content(Arc::new(body))))
                .unwrap();
            assert_eq!(cell.frame.plain_text(), "inside");
            assert_eq!(layout.runtime.label_pages.len(), 1);
            assert_eq!(layout.runtime.label_pages.get(&outer), Some(&7));
            assert_eq!(layout.runtime.label_positions.len(), 1);
            assert_eq!(layout.runtime.label_positions.get(&outer), Some(&Point::ZERO));
        });
    }

    #[test]
    fn tiling_preparation_preserves_outer_flow_transaction() {
        with_layout(|layout| {
            layout.flow_suffix_checkpoint = Some(layout.capture_flow_suffix_checkpoint());
            layout.flow_suffix_checkpoint_pending = true;
            layout.flow_suffix_transaction_active = true;
            layout.flow_suffix_transaction_rejected = true;
            layout.flow_suffix_dispatch_depth = 3;
            layout.flow_suffix_replay.push(Content::text("outer replay"));
            prepare_cell(layout, &sized(TilingBody::Content(Arc::new(Content::Empty))))
                .unwrap();
            assert!(layout.flow_suffix_checkpoint.is_some());
            assert!(layout.flow_suffix_checkpoint_pending);
            assert!(layout.flow_suffix_transaction_active);
            assert!(layout.flow_suffix_transaction_rejected);
            assert_eq!(layout.flow_suffix_dispatch_depth, 3);
            assert_eq!(layout.flow_suffix_replay.len(), 1);
            assert_eq!(layout.flow_suffix_replay[0].plain_text(), "outer replay");
        });
    }

    #[test]
    fn tiling_preparation_body_diagnostic_is_returned_without_consuming_prior_errors() {
        with_layout(|layout| {
            let prior = SourceDiagnostic::error(Span::detached(), "outer error");
            layout.layout_errors.push(prior.clone());
            let body =
                Content::Ref(Arc::new(crate::entities::elements::r#ref::RefElem {
                    name: "missing-tile-label".into(),
                    supplement: None,
                    form: Default::default(),
                }));
            let errors =
                prepare_cell(layout, &sized(TilingBody::Content(Arc::new(body))))
                    .unwrap_err();
            assert_eq!(errors.len(), 1);
            assert!(errors[0].message.contains("missing-tile-label"), "{:?}", errors);
            assert_eq!(layout.layout_errors, [prior]);
        });
    }

    #[test]
    fn tiling_preparation_image_is_laid_out_once_and_preserves_resource() {
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
        let intr = TagIntrospector::empty();
        let intr: &dyn Introspector = &intr;
        let mut layout = Layouter::new(
            FixedMetrics,
            CountImage(std::cell::Cell::new(0)),
            12.0,
            intr.track(),
        );
        let bytes = Arc::new(vec![1, 2, 3]);
        let image = crate::entities::elements::image::ImageElem {
            path: "cell.png".into(),
            alt: Some("cell".into()),
            data: crate::entities::ptr_eq_arc::PtrEqArc(bytes.clone()),
            width: None,
            height: None,
            fit: "cover".into(),
        };
        let cell =
            prepare_cell(&mut layout, &Tiling::new(TilingBody::Image(image))).unwrap();
        assert_eq!(layout.sizer.0.get(), 1);
        assert_eq!(cell.frame.size, Size { width: Pt(8.0), height: Pt(6.0) });
        let FrameItem::Image { data, pos, intrinsic_width, intrinsic_height, .. } =
            &cell.frame.items[0]
        else {
            panic!("image cell")
        };
        assert!(Arc::ptr_eq(data, &bytes));
        assert_eq!(*pos, Point::ZERO);
        assert_eq!((*intrinsic_width, *intrinsic_height), (8, 6));
    }

    #[test]
    fn tiling_preparation_invalid_placement_is_rejected_before_body_layout() {
        struct UntouchedImage;
        impl ImageSizer for UntouchedImage {
            fn size(&self, _: &[u8]) -> Option<(u32, u32)> {
                panic!("invalid placement executed body")
            }
            fn dpi(&self, _: &[u8]) -> Option<f64> {
                panic!("invalid placement executed body")
            }
            fn orientation(&self, _: &[u8]) -> Option<u32> {
                panic!("invalid placement executed body")
            }
        }
        let intr = TagIntrospector::empty();
        let intr: &dyn Introspector = &intr;
        let mut layout = Layouter::new(FixedMetrics, UntouchedImage, 12.0, intr.track());
        let image = crate::entities::elements::image::ImageElem {
            path: "cell.png".into(),
            alt: None,
            data: crate::entities::ptr_eq_arc::PtrEqArc(Arc::new(vec![1, 2, 3])),
            width: None,
            height: None,
            fit: "cover".into(),
        };
        let original = sized(TilingBody::Image(image));
        type InvalidPlacementCase = (fn(&mut Tiling), &'static str);
        let cases: [InvalidPlacementCase; 6] = [
            (
                |t| t.spacing = Some(Size { width: Pt(f64::INFINITY), height: Pt(0.0) }),
                "tile spacing must be finite",
            ),
            (
                |t| t.spacing = Some(Size { width: Pt(-8.0), height: Pt(0.0) }),
                "tile pitch must be positive and finite",
            ),
            (
                |t| t.offset.x.abs = Length::em(1.0),
                "tile offset must not be font-relative",
            ),
            (|t| t.offset.x.rel = f64::NAN, "tile offset must be finite"),
            (|t| t.offset.x.rel = f64::MAX, "tile offset must be finite"),
            (|t| t.angle = Angle::rad(f64::INFINITY), "tile angle must be finite"),
        ];
        for (mutate, expected) in cases {
            let mut tile = original.clone();
            mutate(&mut tile);
            let errors = prepare_cell(&mut layout, &tile).unwrap_err();
            assert_eq!(errors[0].message, expected);
        }
    }
}
