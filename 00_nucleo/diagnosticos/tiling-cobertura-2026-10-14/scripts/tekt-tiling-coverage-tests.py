from pathlib import Path
p=Path('01_core/src/compiler/layout/tiling.rs')
s=p.read_text();s+='''

#[cfg(test)]
mod coverage_regressions {
    use super::*;
    use crate::compiler::layout::FixedMetrics;
    use crate::entities::content::Content;
    use crate::entities::image_sizer::NullImageSizer;
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use crate::entities::layout_types::{Angle, Color, Length, Size};
    use crate::entities::paint::Paint;
    use crate::entities::value::Value;
    use comemo::Track;
    use std::sync::Arc;

    fn tile(size: f64) -> Tiling {
        let body = Content::shape(
            ShapeKind::Rect,
            Some(Box::new(Value::Length(Length::pt(size)))),
            Some(Box::new(Value::Length(Length::pt(size)))),
            Some(Paint::Solid(Color::rgb(255, 0, 0))),
            None,
        );
        let mut tiling = Tiling::new(TilingBody::Content(Arc::new(body)));
        tiling.size = Some(Size { width: Pt(size), height: Pt(size) });
        tiling
    }

    fn materialize(tiling: &Tiling, pos: Point, width: f64, height: f64) -> Option<FrameItem> {
        let intr = TagIntrospector::empty();
        let intr_dyn: &dyn Introspector = &intr;
        let mut layouter = Layouter::new(FixedMetrics, NullImageSizer, 12.0, intr_dyn.track());
        materialize_fill(&mut layouter, tiling, pos, ShapeKind::Rect, width, height)
    }

    fn cells(group: &FrameItem) -> (&TransformMatrix, &[FrameItem]) {
        let FrameItem::Group { items, .. } = group else { panic!("expected clipped fill") };
        let FrameItem::Group { matrix, items, .. } = &items[0] else { panic!("expected placement") };
        (matrix, items)
    }

    #[test]
    fn tiling_coverage_empty_body_is_successful_transparency() {
        let mut t = tile(1.0);
        t.body = TilingBody::Content(Arc::new(Content::Empty));
        let group = materialize(&t, Point::ZERO, 100.0, 100.0)
            .expect("valid empty tile must not fall back to black");
        let FrameItem::Group { items, .. } = group else { panic!("expected empty group") };
        assert!(items.is_empty());
    }

    #[test]
    fn tiling_coverage_shape_body_starts_at_cell_origin() {
        let group = materialize(&tile(4.0), Point::ZERO, 16.0, 16.0).unwrap();
        let (_, cells) = cells(&group);
        let FrameItem::Group { items, .. } = &cells[0] else { panic!("expected cell") };
        let FrameItem::Shape { pos, width, height, .. } = &items[0] else { panic!("expected body") };
        assert_eq!(*pos, Point::ZERO, "cell clipping must not discard body at a text baseline");
        assert_eq!((*width, *height), (4.0, 4.0));
    }

    #[test]
    fn tiling_coverage_auto_shape_uses_actual_block_extent() {
        let mut t = tile(4.0);
        t.size = None;
        let group = materialize(&t, Point::ZERO, 16.0, 16.0).expect("auto block has nonzero width");
        let (_, cells) = cells(&group);
        let FrameItem::Group { inner_width, inner_height, .. } = &cells[0] else { panic!("expected cell") };
        assert_eq!((*inner_width, *inner_height), (4.0, 4.0));
    }

    #[test]
    fn tiling_coverage_dense_pattern_does_not_use_outside_diagonal_square() {
        let group = materialize(&tile(2.0), Point::ZERO, 100.0, 100.0)
            .expect("2500 visible cells fit below the existing 10000 limit");
        assert!(cells(&group).1.len() <= 10000);
    }

    #[test]
    fn tiling_coverage_translated_rotated_parent_covers_target() {
        let mut t = tile(8.0);
        t.relative = TilingRelative::Parent;
        t.angle = Angle::rad(0.61);
        let group = materialize(&t, Point { x: Pt(180.0), y: Pt(60.0) }, 40.0, 24.0).unwrap();
        let (matrix, cells) = cells(&group);
        let det = matrix.a * matrix.d - matrix.b * matrix.c;
        for (x, y) in [(0.25, 0.25), (39.75, 0.25), (0.25, 23.75), (39.75, 23.75), (20.0, 12.0)] {
            let dx = x - matrix.tx;
            let dy = y - matrix.ty;
            let local = ((matrix.d * dx - matrix.c * dy) / det, (matrix.a * dy - matrix.b * dx) / det);
            assert!(cells.iter().any(|cell| {
                let FrameItem::Group { pos, inner_width, inner_height, .. } = cell else { return false };
                local.0 >= pos.x.0 && local.0 <= pos.x.0 + inner_width
                    && local.1 >= pos.y.0 && local.1 <= pos.y.0 + inner_height
            }), "pattern lattice leaves target point ({x},{y}) uncovered");
        }
    }

    #[test]
    fn tiling_coverage_large_whole_period_offset_is_equivalent() {
        let plain = tile(8.0);
        let mut shifted = plain.clone();
        shifted.offset.x.abs = Length::pt(1_000_000_000_000.0);
        shifted.offset.y.abs = Length::pt(-1_000_000_000_000.0);
        let a = materialize(&plain, Point::ZERO, 40.0, 24.0).unwrap();
        let b = materialize(&shifted, Point::ZERO, 40.0, 24.0).unwrap();
        assert_eq!(a, b, "whole periods must preserve geometry, including large offsets");
    }
}
''';p.write_text(s)
