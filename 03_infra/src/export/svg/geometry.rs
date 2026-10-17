//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/svg/geometry.md
//! @prompt-hash 46b8c047
//! @layer L3
//! @updated 2026-09-26

use super::paints::PaintDefs;
use super::serialization::{color_to_css, fmt_num, fmt_pair};
use typst_core::entities::frame_paint::{FramePaint as Paint, FrameStroke as Stroke};
use typst_core::entities::geometry::{PathItem, ShapeKind};
use typst_core::entities::layout_types::{Color, Point};
use xmlwriter::XmlWriter;

pub(super) fn render_line(
    xml: &mut XmlWriter,
    start: &Point,
    end: &Point,
    thickness: f64,
    color: Option<Color>,
) {
    xml.start_element("line");
    xml.write_attribute("x1", &fmt_num(start.x.0));
    xml.write_attribute("y1", &fmt_num(start.y.0));
    xml.write_attribute("x2", &fmt_num(end.x.0));
    xml.write_attribute("y2", &fmt_num(end.y.0));
    xml.write_attribute("stroke", &color_to_css(color.unwrap_or(Color::rgb(0, 0, 0))));
    xml.write_attribute("stroke-width", &fmt_num(thickness));
    xml.write_attribute("stroke-linecap", "butt");
    xml.end_element();
}
#[expect(
    clippy::too_many_arguments,
    reason = "C065: Leaf SVG de Shape usa a geometria resolvida, fill/stroke/fill-rule e PaintDefs para referências locais"
)]
pub(super) fn render_shape(
    xml: &mut XmlWriter,
    pos: &Point,
    kind: &ShapeKind<typst_core::entities::layout_types::Pt>,
    width: f64,
    height: f64,
    fill: Option<&Paint>,
    stroke: Option<&Stroke>,
    fill_rule: typst_core::entities::geometry::FillRule,
    paint_defs: &mut PaintDefs,
) {
    let fill_reference =
        fill.map(|paint| paint_defs.reference(paint, pos.x.0, pos.y.0, width, height));
    // O paint de stroke é relativo à caixa pintada, que inclui metade da
    // espessura em cada lado. Usar apenas a caixa geométrica desloca o campo
    // angular e, com alpha, torna a diferença observável nas bordas.
    let stroke_reference = stroke.map(|value| {
        if matches!(value.paint, Paint::Tiling(_)) {
            return paint_defs.reference(&value.paint, pos.x.0, pos.y.0, width, height);
        }
        let outset = value.thickness / 2.0;
        paint_defs.reference(
            &value.paint,
            pos.x.0 - outset,
            pos.y.0 - outset,
            width + value.thickness,
            height + value.thickness,
        )
    });
    let attrs = ShapeAttrs {
        fill: fill_reference.as_ref().map(|reference| reference.value.clone()),
        fill_fallback: fill_reference.and_then(|reference| reference.fallback),
        stroke,
        stroke_paint: stroke_reference.as_ref().map(|reference| reference.value.clone()),
        stroke_fallback: stroke_reference.and_then(|reference| reference.fallback),
    };

    match kind {
        ShapeKind::Rect => {
            xml.start_element("rect");
            xml.write_attribute("x", &fmt_num(pos.x.0));
            xml.write_attribute("y", &fmt_num(pos.y.0));
            xml.write_attribute("width", &fmt_num(width));
            xml.write_attribute("height", &fmt_num(height));
            write_shape_attrs(xml, &attrs);
            xml.end_element();
        }
        ShapeKind::RoundedRect { radii } => {
            let inset = stroke.map_or(0.0, |value| value.thickness / 2.0);
            let d = rounded_rect_path_data(pos.x.0, pos.y.0, width, height, radii, inset);
            if let Some(fill) = attrs.fill.as_ref() {
                write_rounded_rect_paint(
                    xml,
                    &d,
                    Some(fill),
                    None,
                    None,
                    attrs.fill_fallback,
                );
            }
            if let Some(stroke) = attrs.stroke {
                let stroke_d = d.strip_suffix(" Z").unwrap_or(&d);
                write_rounded_rect_paint(
                    xml,
                    stroke_d,
                    None,
                    Some(stroke),
                    attrs.stroke_paint.as_deref(),
                    attrs.stroke_fallback,
                );
            }
            if attrs.fill.is_none() && attrs.stroke.is_none() {
                write_rounded_rect_paint(xml, &d, None, None, None, None);
            }
        }
        ShapeKind::Ellipse => {
            xml.start_element("ellipse");
            xml.write_attribute("cx", &fmt_num(pos.x.0 + width / 2.0));
            xml.write_attribute("cy", &fmt_num(pos.y.0 + height / 2.0));
            xml.write_attribute("rx", &fmt_num(width / 2.0));
            xml.write_attribute("ry", &fmt_num(height / 2.0));
            write_shape_attrs(xml, &attrs);
            xml.end_element();
        }
        ShapeKind::Line { dx, dy } => {
            xml.start_element("line");
            xml.write_attribute("x1", &fmt_num(pos.x.0));
            xml.write_attribute("y1", &fmt_num(pos.y.0));
            xml.write_attribute("x2", &fmt_num(pos.x.0 + dx));
            xml.write_attribute("y2", &fmt_num(pos.y.0 + dy));
            write_shape_attrs(xml, &attrs);
            xml.end_element();
        }
        ShapeKind::Path(items) => {
            xml.start_element("path");
            if let ([PathItem::MoveTo(start), PathItem::LineTo(end)], false) = (
                items.as_slice(),
                matches!(fill, Some(Paint::Tiling(_)))
                    || stroke.is_some_and(|s| matches!(s.paint, Paint::Tiling(_))),
            ) {
                xml.write_attribute(
                    "transform",
                    &format!(
                        "translate({} {})",
                        fmt_num(pos.x.0 + start.x.0),
                        fmt_num(pos.y.0 + start.y.0),
                    ),
                );
                xml.write_attribute(
                    "d",
                    &format!(
                        "M 0 0 l {} {}",
                        fmt_num(end.x.0 - start.x.0),
                        fmt_num(end.y.0 - start.y.0),
                    ),
                );
            } else {
                xml.write_attribute("d", &path_items_to_svg(items, pos));
            }
            if fill_rule == typst_core::entities::geometry::FillRule::EvenOdd {
                xml.write_attribute("fill-rule", "evenodd");
            }
            write_shape_attrs(xml, &attrs);
            xml.end_element();
        }
    }
}

pub(super) fn rounded_rect_path_data(
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    radii: &typst_core::entities::corners::Corners<
        typst_core::entities::layout_types::Pt,
    >,
    inset: f64,
) -> String {
    let max_r = width.min(height) / 2.0;
    let tl = (radii.top_left.0 - inset).clamp(0.0, max_r);
    let tr = (radii.top_right.0 - inset).clamp(0.0, max_r);
    let br = (radii.bottom_right.0 - inset).clamp(0.0, max_r);
    let bl = (radii.bottom_left.0 - inset).clamp(0.0, max_r);
    let k = crate::export::pdf_defaults::BEZIER_CIRCLE_KAPPA;
    let right = x + width;
    let bottom = y + height;

    format!(
        "M {} {} L {} {} C {} {} {} {} {} {} L {} {} C {} {} {} {} {} {} L {} {} C {} {} {} {} {} {} L {} {} C {} {} {} {} {} {} Z",
        fmt_num(x + tl), fmt_num(y),
        fmt_num(right - tr), fmt_num(y),
        fmt_num(right - tr + k * tr), fmt_num(y),
        fmt_num(right), fmt_num(y + tr - k * tr),
        fmt_num(right), fmt_num(y + tr),
        fmt_num(right), fmt_num(bottom - br),
        fmt_num(right), fmt_num(bottom - br + k * br),
        fmt_num(right - br + k * br), fmt_num(bottom),
        fmt_num(right - br), fmt_num(bottom),
        fmt_num(x + bl), fmt_num(bottom),
        fmt_num(x + bl - k * bl), fmt_num(bottom),
        fmt_num(x), fmt_num(bottom - bl + k * bl),
        fmt_num(x), fmt_num(bottom - bl),
        fmt_num(x), fmt_num(y + tl),
        fmt_num(x), fmt_num(y + tl - k * tl),
        fmt_num(x + tl - k * tl), fmt_num(y),
        fmt_num(x + tl), fmt_num(y),
    )
}

fn write_rounded_rect_paint(
    xml: &mut XmlWriter,
    d: &str,
    fill: Option<&str>,
    stroke: Option<&Stroke>,
    stroke_paint: Option<&str>,
    fallback: Option<&str>,
) {
    xml.start_element("path");
    xml.write_attribute("d", d);
    xml.write_attribute("fill", fill.unwrap_or("none"));
    if let Some(stroke) = stroke {
        write_stroke_attrs(xml, stroke, stroke_paint);
    }
    if let Some(reason) = fallback {
        xml.write_attribute(
            if stroke.is_some() {
                "data-crystalline-stroke-fallback"
            } else {
                "data-crystalline-fill-fallback"
            },
            reason,
        );
    }
    xml.end_element();
}

struct ShapeAttrs<'a> {
    fill: Option<String>,
    fill_fallback: Option<&'static str>,
    stroke: Option<&'a Stroke>,
    stroke_paint: Option<String>,
    stroke_fallback: Option<&'static str>,
}

fn write_shape_attrs(xml: &mut XmlWriter, attrs: &ShapeAttrs) {
    if let Some(fill) = &attrs.fill {
        xml.write_attribute("fill", fill);
    } else {
        xml.write_attribute("fill", "none");
    }
    if let Some(reason) = attrs.fill_fallback {
        xml.write_attribute("data-crystalline-fill-fallback", reason);
    }
    if let Some(stroke) = &attrs.stroke {
        write_stroke_attrs(xml, stroke, attrs.stroke_paint.as_deref());
        if let Some(reason) = attrs.stroke_fallback {
            xml.write_attribute("data-crystalline-stroke-fallback", reason);
        }
    }
}

fn write_stroke_attrs(xml: &mut XmlWriter, stroke: &Stroke, paint: Option<&str>) {
    use typst_core::entities::geometry::{DashLength, LineCap, LineJoin};

    xml.write_attribute("stroke", paint.unwrap_or("none"));
    xml.write_attribute("stroke-width", &fmt_num(stroke.thickness));
    xml.write_attribute(
        "stroke-linecap",
        match stroke.cap {
            LineCap::Butt => "butt",
            LineCap::Round => "round",
            LineCap::Square => "square",
        },
    );
    xml.write_attribute(
        "stroke-linejoin",
        match stroke.join {
            LineJoin::Miter => "miter",
            LineJoin::Round => "round",
            LineJoin::Bevel => "bevel",
        },
    );
    xml.write_attribute("stroke-miterlimit", &fmt_num(stroke.miter_limit));
    if let Some(dash) = &stroke.dash {
        let array = dash
            .array
            .iter()
            .map(|value| match value {
                DashLength::Length(value) => fmt_num(*value),
                DashLength::LineWidth => fmt_num(stroke.thickness),
            })
            .collect::<Vec<_>>()
            .join(" ");
        xml.write_attribute("stroke-dasharray", &array);
        xml.write_attribute("stroke-dashoffset", &fmt_num(dash.phase));
    }
}
pub(super) fn path_items_to_svg(items: &[PathItem], offset: &Point) -> String {
    let mut out = String::new();
    for item in items {
        match item {
            PathItem::MoveTo(p) => {
                out.push_str(&format!(
                    "M {} ",
                    fmt_pair(p.x.0 + offset.x.0, p.y.0 + offset.y.0)
                ));
            }
            PathItem::LineTo(p) => {
                out.push_str(&format!(
                    "L {} ",
                    fmt_pair(p.x.0 + offset.x.0, p.y.0 + offset.y.0)
                ));
            }
            PathItem::CubicTo(p1, p2, p3) => {
                out.push_str(&format!(
                    "C {} {} {} ",
                    fmt_pair(p1.x.0 + offset.x.0, p1.y.0 + offset.y.0),
                    fmt_pair(p2.x.0 + offset.x.0, p2.y.0 + offset.y.0),
                    fmt_pair(p3.x.0 + offset.x.0, p3.y.0 + offset.y.0),
                ));
            }
            PathItem::ClosePath => {
                out.push_str("Z ");
            }
        }
    }
    out
}
