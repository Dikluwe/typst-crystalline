//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/svg/groups.md
//! @prompt-hash 35d7a6f4
//! @layer L3
//! @updated 2026-09-26

use super::geometry::{path_items_to_svg, rounded_rect_path_data};
use super::serialization::fmt_num;
use super::{render_item, SvgRenderSession};
use typst_core::entities::geometry::{PathItem, ShapeKind};
use typst_core::entities::layout_types::{FrameItem, Point, Pt, Size, TransformMatrix};
use xmlwriter::XmlWriter;

pub(super) fn render_group(
    xml: &mut XmlWriter,
    pos: &Point,
    matrix: &TransformMatrix,
    clip_mask: Option<&ShapeKind<typst_core::entities::layout_types::Pt>>,
    size: Size,
    items: &[FrameItem],
    session: &mut SvgRenderSession<'_>,
) {
    let inner_width = size.width.0;
    let inner_height = size.height.0;
    let clip = clip_mask.map(|kind| {
        write_clip_definition(xml, kind, inner_width, inner_height, session.clip_next_id)
    });
    xml.start_element("g");
    let transform = if is_identity(matrix) {
        format!("translate({} {})", fmt_num(pos.x.0), fmt_num(pos.y.0))
    } else {
        format!(
            "translate({x} {y}) matrix({a} {b} {c} {d} {tx} {ty})",
            x = fmt_num(pos.x.0),
            y = fmt_num(pos.y.0),
            a = fmt_num(matrix.a),
            b = fmt_num(matrix.b),
            c = fmt_num(matrix.c),
            d = fmt_num(matrix.d),
            tx = fmt_num(matrix.tx),
            ty = fmt_num(matrix.ty),
        )
    };
    xml.write_attribute("transform", &transform);
    if let Some(Ok(id)) = &clip {
        xml.write_attribute("clip-path", &format!("url(#{id})"));
    } else if let Some(Err(reason)) = clip {
        xml.write_attribute("data-crystalline-clip-fallback", reason);
    }
    for item in items {
        render_item(xml, item, session);
    }
    xml.end_element();
}
fn write_clip_definition(
    xml: &mut XmlWriter,
    kind: &ShapeKind<Pt>,
    width: f64,
    height: f64,
    next_id: &mut usize,
) -> Result<String, &'static str> {
    if !width.is_finite() || !height.is_finite() || width < 0.0 || height < 0.0 {
        return Err("unrepresentable-geometry");
    }
    if !clip_kind_is_representable(kind) {
        return Err("unrepresentable-geometry");
    }

    let id = format!("c{}", *next_id);
    *next_id += 1;
    xml.start_element("defs");
    xml.start_element("clipPath");
    xml.write_attribute("id", &id);
    xml.write_attribute("clipPathUnits", "userSpaceOnUse");
    match kind {
        ShapeKind::Rect => {
            xml.start_element("rect");
            xml.write_attribute("x", "0");
            xml.write_attribute("y", "0");
            xml.write_attribute("width", &fmt_num(width));
            xml.write_attribute("height", &fmt_num(height));
            xml.end_element();
        }
        ShapeKind::RoundedRect { radii } => {
            xml.start_element("path");
            xml.write_attribute(
                "d",
                &rounded_rect_path_data(0.0, 0.0, width, height, radii, 0.0),
            );
            xml.write_attribute("clip-rule", "nonzero");
            xml.end_element();
        }
        ShapeKind::Ellipse => {
            xml.start_element("ellipse");
            xml.write_attribute("cx", &fmt_num(width / 2.0));
            xml.write_attribute("cy", &fmt_num(height / 2.0));
            xml.write_attribute("rx", &fmt_num(width / 2.0));
            xml.write_attribute("ry", &fmt_num(height / 2.0));
            xml.end_element();
        }
        ShapeKind::Path(items) => {
            xml.start_element("path");
            xml.write_attribute("d", &path_items_to_svg(items, &Point::ZERO));
            xml.write_attribute("clip-rule", "nonzero");
            xml.end_element();
        }
        ShapeKind::Line { .. } => unreachable!("line clips are rejected above"),
    }
    xml.end_element();
    xml.end_element();
    Ok(id)
}

fn clip_kind_is_representable(kind: &ShapeKind<Pt>) -> bool {
    let finite = |point: &Point| point.x.0.is_finite() && point.y.0.is_finite();
    match kind {
        ShapeKind::Rect | ShapeKind::Ellipse => true,
        ShapeKind::RoundedRect { radii } => [
            radii.top_left.0,
            radii.top_right.0,
            radii.bottom_right.0,
            radii.bottom_left.0,
        ]
        .into_iter()
        .all(|radius| radius.is_finite() && radius >= 0.0),
        ShapeKind::Line { .. } => false,
        ShapeKind::Path(items) => {
            !items.is_empty()
                && matches!(items.last(), Some(PathItem::ClosePath))
                && items.iter().all(|item| match item {
                    PathItem::MoveTo(point) | PathItem::LineTo(point) => finite(point),
                    PathItem::CubicTo(a, b, c) => finite(a) && finite(b) && finite(c),
                    PathItem::ClosePath => true,
                })
        }
    }
}
fn is_identity(m: &TransformMatrix) -> bool {
    m.a == 1.0 && m.b == 0.0 && m.c == 0.0 && m.d == 1.0 && m.tx == 0.0 && m.ty == 0.0
}
