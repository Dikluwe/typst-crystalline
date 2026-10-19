//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/tiling.md
//! @prompt-hash d8e615f6
//! @layer L3

use typst_core::entities::frame_paint::TilingFrame;
use typst_core::entities::gradient::RelativeTo;
use typst_core::entities::layout_types::{Point, TransformMatrix};

pub(super) fn finite(m: TransformMatrix) -> bool {
    [m.a, m.b, m.c, m.d, m.tx, m.ty].into_iter().all(f64::is_finite)
}

pub(super) fn inverse(m: TransformMatrix) -> Option<TransformMatrix> {
    let determinant = m.a * m.d - m.b * m.c;
    if !finite(m) || !determinant.is_finite() || determinant == 0.0 {
        return None;
    }
    let result = TransformMatrix {
        a: m.d / determinant,
        b: -m.b / determinant,
        c: -m.c / determinant,
        d: m.a / determinant,
        tx: (m.c * m.ty - m.d * m.tx) / determinant,
        ty: (m.b * m.tx - m.a * m.ty) / determinant,
    };
    finite(result).then_some(result)
}

/// Map cell coordinates to the frame coordinates used by a Shape path.
pub(super) fn placement(
    cell: &TilingFrame,
    origin: Point,
    local_to_root: TransformMatrix,
) -> Option<TransformMatrix> {
    let width = cell.frame.size.width.0;
    let height = cell.frame.size.height.0;
    let pitch_x = width + cell.spacing.width.0;
    let pitch_y = height + cell.spacing.height.0;
    if [width, height, pitch_x, pitch_y]
        .into_iter()
        .any(|v| !v.is_finite() || v <= 0.0)
        || !cell.offset.x.0.is_finite()
        || !cell.offset.y.0.is_finite()
        || !cell.angle.to_rad().is_finite()
    {
        return None;
    }
    let base = match cell.relative {
        RelativeTo::Self_ => TransformMatrix::translate(origin.x.0, origin.y.0),
        RelativeTo::Parent => inverse(local_to_root)?,
    };
    let matrix = base
        .concat(&TransformMatrix::translate(cell.offset.x.0, cell.offset.y.0))
        .concat(&TransformMatrix::rotate(cell.angle.to_rad()));
    let inv = inverse(matrix)?;
    // inv.translation is the negation of the translation in lattice space.
    let x = (-inv.tx).rem_euclid(pitch_x);
    let y = (-inv.ty).rem_euclid(pitch_y);
    let matrix = TransformMatrix {
        tx: matrix.a * x + matrix.c * y,
        ty: matrix.b * x + matrix.d * y,
        ..matrix
    };
    finite(matrix).then_some(matrix)
}

/// Cells whose clipped extent intersects the first period, in painter order.
pub(super) fn contributions(
    cell: &TilingFrame,
) -> Option<impl Iterator<Item = (f64, f64)>> {
    let w = cell.frame.size.width.0;
    let h = cell.frame.size.height.0;
    let px = w + cell.spacing.width.0;
    let py = h + cell.spacing.height.0;
    if [w, h, px, py].into_iter().any(|n| !n.is_finite() || n <= 0.0) {
        return None;
    }
    let nx = (w / px).ceil();
    let ny = (h / py).ceil();
    if !nx.is_finite()
        || !ny.is_finite()
        || nx >= i64::MAX as f64
        || ny >= i64::MAX as f64
    {
        return None;
    }
    Some((1 - ny as i64..=0).flat_map(move |y| {
        (1 - nx as i64..=0).map(move |x| (x as f64 * px, y as f64 * py))
    }))
}

/// Resource-only traversal: prepared paints are deliberately not page content.
pub(super) fn walk_resources<V: typst_core::entities::frame_visitor::FrameVisitor>(
    visitor: &mut V,
    items: &[typst_core::entities::layout_types::FrameItem],
) {
    use typst_core::entities::frame_paint::FramePaint;
    use typst_core::entities::layout_types::FrameItem;
    for item in items {
        match item {
            FrameItem::Group { items, .. }
            | FrameItem::Link { items, .. }
            | FrameItem::Semantic { items, .. }
            | FrameItem::TextReplacement { items, .. } => walk_resources(visitor, items),
            FrameItem::Shape { fill, stroke, .. } => {
                visitor.visit_shape(item);
                for paint in fill.iter().chain(stroke.iter().map(|s| &s.paint)) {
                    if let FramePaint::Tiling(cell) = paint {
                        walk_resources(visitor, &cell.frame.items);
                    }
                }
            }
            FrameItem::Text { .. }
            | FrameItem::TextSeparator { .. }
            | FrameItem::TextShaped { .. }
            | FrameItem::Line { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Image { .. } => visitor.visit_item(item),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::entities::layout_types::{Angle, Frame, Pt, Size};

    fn cell(relative: RelativeTo) -> TilingFrame {
        TilingFrame {
            frame: Frame::new(Size { width: Pt(8.0), height: Pt(12.0) }),
            spacing: Size { width: Pt(0.0), height: Pt(0.0) },
            offset: Point { x: Pt(3.0), y: Pt(5.0) },
            angle: Angle::deg(37.0),
            relative,
        }
    }
    fn phase(m: TransformMatrix, point: (f64, f64)) -> (f64, f64) {
        let (x, y) = inverse(m).unwrap().apply(point.0, point.1);
        (x.rem_euclid(8.0), y.rem_euclid(12.0))
    }
    #[test]
    fn parent_cancels_the_complete_affine_chain() {
        let cell = cell(RelativeTo::Parent);
        let root = TransformMatrix::translate(21.0, 34.0)
            .concat(&TransformMatrix::rotate(0.43))
            .concat(&TransformMatrix::scale(2.0, 0.7))
            .concat(&TransformMatrix::skew(0.1, 0.2));
        let actual = root
            .concat(&placement(&cell, Point { x: Pt(17.0), y: Pt(9.0) }, root).unwrap());
        let expected = TransformMatrix::translate(3.0, 5.0)
            .concat(&TransformMatrix::rotate(cell.angle.to_rad()));
        for p in [(0.2, 0.7), (31.5, 44.2), (-18.4, 7.6)] {
            let a = phase(actual, p);
            let b = phase(expected, p);
            assert!((a.0 - b.0).abs() < 1e-10 && (a.1 - b.1).abs() < 1e-10);
        }
    }
    #[test]
    fn self_uses_shape_origin_and_periodic_offsets_in_rotated_axes() {
        let mut cell = cell(RelativeTo::Self_);
        cell.angle = Angle::deg(90.0);
        let origin = Point { x: Pt(20.0), y: Pt(30.0) };
        let a = placement(&cell, origin, TransformMatrix::scale(3.0, 4.0)).unwrap();
        let expected = TransformMatrix::translate(23.0, 35.0)
            .concat(&TransformMatrix::rotate(cell.angle.to_rad()));
        cell.offset.x.0 += 12.0 * 1_000_000.0;
        let b = placement(&cell, origin, TransformMatrix::identity()).unwrap();
        for p in [(23.2, 35.7), (31.5, 44.2)] {
            let a = phase(a, p);
            let b = phase(b, p);
            let e = phase(expected, p);
            assert!((a.0 - e.0).abs() < 1e-10 && (a.1 - e.1).abs() < 1e-10);
            assert!((a.0 - b.0).abs() < 1e-7 && (a.1 - b.1).abs() < 1e-7);
        }
    }
    #[test]
    fn singular_or_invalid_input_has_no_placement() {
        let mut cell = cell(RelativeTo::Parent);
        assert!(placement(&cell, Point::ZERO, TransformMatrix::scale(0.0, 1.0)).is_none());
        cell.relative = RelativeTo::Self_;
        cell.spacing.width = Pt(-8.0);
        assert!(placement(&cell, Point::ZERO, TransformMatrix::identity()).is_none());
    }
    #[test]
    fn overlap_contributors_match_half_open_cell_intersections() {
        let mut c = cell(RelativeTo::Self_);
        c.frame.size = Size { width: Pt(8.0), height: Pt(9.0) };
        c.spacing = Size { width: Pt(-4.0), height: Pt(-5.0) };
        let translations: Vec<_> = contributions(&c).unwrap().collect();
        assert_eq!(
            translations,
            vec![
                (-4.0, -8.0),
                (0.0, -8.0),
                (-4.0, -4.0),
                (0.0, -4.0),
                (-4.0, 0.0),
                (0.0, 0.0)
            ]
        );
        c.spacing.width = Pt(-8.0);
        assert!(contributions(&c).is_none());
        c.spacing.width = Pt(f64::INFINITY);
        assert!(contributions(&c).is_none());
    }
}
