//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/tiling.md
//! @prompt-hash 31b81dd4
//! @layer L1

use crate::entities::geometry::ShapeKind;
use crate::entities::image_sizer::ImageSizer;
use crate::entities::layout_types::{FrameItem, Point, Pt, TransformMatrix};
use crate::entities::tiling::{Tiling, TilingBody, TilingRelative};

use super::{FontMetrics, Layouter, SubLayoutRegion};

/// Materializa um fill declarativo em grupos normais do pipeline.
/// `None` significa que o fragmento permanece Unknown para este consumer.
pub(super) fn materialize_fill<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    tiling: &Tiling,
    pos: Point,
    clip_kind: ShapeKind<Pt>,
    width: f64,
    height: f64,
) -> Option<FrameItem> {
    let TilingBody::Content(body) = &tiling.body else {
        return None;
    };
    if !width.is_finite() || !height.is_finite() || width <= 0.0 || height <= 0.0 {
        return None;
    }

    let explicit = tiling.size;
    let layout_width = explicit.map(|size| size.width.0).unwrap_or(f64::INFINITY);
    if !layout_width.is_finite() && explicit.is_some() || layout_width <= 0.0 {
        return None;
    }
    let (body_height, body_items, _, _, _) = layouter.layout_sub_frame(
        body,
        SubLayoutRegion {
            origin_x: 0.0,
            width: layout_width,
            height: explicit.map(|size| size.height.0),
            align_rtl: false,
            unconstrained_height: explicit.is_none(),
        },
    );
    let body_width = layouter.last_sub_frame_width;
    let cell_width = explicit.map(|size| size.width.0).unwrap_or(body_width);
    let cell_height = explicit.map(|size| size.height.0).unwrap_or(body_height);
    let spacing = tiling.spacing.unwrap_or(crate::entities::layout_types::Size {
        width: Pt(0.0),
        height: Pt(0.0),
    });
    let pitch_x = cell_width + spacing.width.0;
    let pitch_y = cell_height + spacing.height.0;
    if body_items.is_empty()
        || !cell_width.is_finite()
        || !cell_height.is_finite()
        || !pitch_x.is_finite()
        || !pitch_y.is_finite()
        || cell_width <= 0.0
        || cell_height <= 0.0
        || pitch_x <= 0.0
        || pitch_y <= 0.0
    {
        return None;
    }

    let offset_x = tiling.offset.x.rel * pitch_x + tiling.offset.x.abs.abs.0;
    let offset_y = tiling.offset.y.rel * pitch_y + tiling.offset.y.abs.abs.0;
    let angle = tiling.angle.to_rad();
    if !offset_x.is_finite() || !offset_y.is_finite() || !angle.is_finite() {
        return None;
    }

    let radius = width.hypot(height) + cell_width.max(cell_height);
    let min_x = ((-radius - offset_x) / pitch_x).floor() as i32;
    let max_x = ((radius - offset_x) / pitch_x).ceil() as i32;
    let min_y = ((-radius - offset_y) / pitch_y).floor() as i32;
    let max_y = ((radius - offset_y) / pitch_y).ceil() as i32;
    let span_x = i64::from(max_x) - i64::from(min_x) + 1;
    let span_y = i64::from(max_y) - i64::from(min_y) + 1;
    let count = span_x.saturating_mul(span_y);
    if count <= 0 || count > 10_000 {
        return None;
    }

    let mut cells = Vec::with_capacity(count as usize);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            cells.push(FrameItem::Group {
                pos: Point {
                    x: Pt(f64::from(x) * pitch_x),
                    y: Pt(f64::from(y) * pitch_y),
                },
                matrix: TransformMatrix::identity(),
                clip_mask: Some(ShapeKind::Rect),
                inner_width: cell_width,
                inner_height: cell_height,
                items: body_items.clone(),
            });
        }
    }

    // Shapes resolve `auto` to `self`. An explicit `parent` keeps the grid's
    // phase in the parent's coordinate system, so translating sibling shapes
    // does not translate the pattern origin with them.
    let placement = placement_transform(tiling.relative, pos, offset_x, offset_y, angle);
    Some(FrameItem::Group {
        pos,
        matrix: TransformMatrix::identity(),
        clip_mask: Some(clip_kind),
        inner_width: width,
        inner_height: height,
        items: vec![FrameItem::Group {
            pos: Point::ZERO,
            matrix: placement,
            clip_mask: None,
            inner_width: width,
            inner_height: height,
            items: cells,
        }],
    })
}

fn placement_transform(
    relative: TilingRelative,
    pos: Point,
    offset_x: f64,
    offset_y: f64,
    angle: f64,
) -> TransformMatrix {
    let (basis_x, basis_y) = match relative {
        TilingRelative::Parent => (-pos.x.0, -pos.y.0),
        TilingRelative::Auto | TilingRelative::Itself => (0.0, 0.0),
    };
    TransformMatrix::rotate(angle)
        .concat(&TransformMatrix::translate(offset_x + basis_x, offset_y + basis_y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1254_offset_e_aplicado_antes_da_rotacao_clockwise() {
        let matrix = placement_transform(
            TilingRelative::Itself,
            Point::ZERO,
            3.0,
            4.0,
            std::f64::consts::FRAC_PI_2,
        );
        assert!((matrix.tx + 4.0).abs() < 1e-9);
        assert!((matrix.ty - 3.0).abs() < 1e-9);
    }

    #[test]
    fn p1254_parent_e_self_preservam_fases_distintas() {
        let pos = Point { x: Pt(20.0), y: Pt(30.0) };
        let itself = placement_transform(TilingRelative::Itself, pos, 2.0, 3.0, 0.0);
        let parent = placement_transform(TilingRelative::Parent, pos, 2.0, 3.0, 0.0);
        assert_eq!((itself.tx, itself.ty), (2.0, 3.0));
        assert_eq!((parent.tx, parent.ty), (-18.0, -27.0));
    }

    #[test]
    fn p1254_transformacao_e_deterministica_sob_repeticao() {
        let args =
            (TilingRelative::Parent, Point { x: Pt(7.0), y: Pt(11.0) }, 2.0, 5.0, 0.3);
        let first = placement_transform(args.0, args.1, args.2, args.3, args.4);
        let second = placement_transform(args.0, args.1, args.2, args.3, args.4);
        assert_eq!(first, second);
    }
}
