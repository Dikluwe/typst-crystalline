//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/svg/images.md
//! @prompt-hash cfd6a3a3
//! @layer L3
//! @updated 2026-09-26

use super::serialization::fmt_num;
use base64::Engine;
use std::sync::Arc;
use typst_core::entities::image_format::{detect_image_format, ImageFormat};
use typst_core::entities::layout_types::{Point, Pt, TransformMatrix};
use xmlwriter::XmlWriter;

pub(super) fn render_image(
    xml: &mut XmlWriter,
    pos: &Point,
    data: &Arc<Vec<u8>>,
    width: Pt,
    height: Pt,
    clip_rect: Option<&typst_core::entities::layout_types::Rect>,
    orientation: u32,
    clip_next_id: &mut usize,
) {
    let mime = match detect_image_format(data) {
        ImageFormat::Jpeg => "image/jpeg",
        ImageFormat::Png => "image/png",
        ImageFormat::Gif => "image/gif",
        ImageFormat::WebP => "image/webp",
        ImageFormat::Unknown if self_contained_svg(data) => "image/svg+xml",
        ImageFormat::Unknown => {
            write_image_fallback(xml, pos, width, height, "unknown-format");
            return;
        }
    };
    if !pos.x.0.is_finite()
        || !pos.y.0.is_finite()
        || !width.0.is_finite()
        || !height.0.is_finite()
        || width.0 < 0.0
        || height.0 < 0.0
    {
        write_image_fallback(xml, pos, width, height, "unrepresentable-geometry");
        return;
    }
    let b64 = base64::engine::general_purpose::STANDARD.encode(data.as_slice());
    let url = format!("data:{mime};base64,{b64}");

    let clip_id = clip_rect.and_then(|clip| write_image_clip(xml, clip, clip_next_id));
    if clip_rect.is_some() {
        xml.start_element("g");
        if let Some(id) = &clip_id {
            xml.write_attribute("clip-path", &format!("url(#{id})"));
        } else {
            xml.write_attribute(
                "data-crystalline-image-clip-fallback",
                "unrepresentable-geometry",
            );
        }
    }

    xml.start_element("image");
    if orientation == 1 {
        xml.write_attribute("x", &fmt_num(pos.x.0));
        xml.write_attribute("y", &fmt_num(pos.y.0));
        xml.write_attribute("width", &fmt_num(width.0));
        xml.write_attribute("height", &fmt_num(height.0));
    } else {
        let matrix =
            image_orientation_matrix(orientation, width.0, height.0, pos.x.0, pos.y.0);
        xml.write_attribute("x", "0");
        xml.write_attribute("y", "0");
        xml.write_attribute("width", "1");
        xml.write_attribute("height", "1");
        xml.write_attribute(
            "transform",
            &format!(
                "matrix({} {} {} {} {} {})",
                fmt_num(matrix.a),
                fmt_num(matrix.b),
                fmt_num(matrix.c),
                fmt_num(matrix.d),
                fmt_num(matrix.tx),
                fmt_num(matrix.ty),
            ),
        );
    }
    xml.write_attribute("xlink:href", &url);
    xml.write_attribute("preserveAspectRatio", "none");
    xml.end_element();
    if clip_rect.is_some() {
        xml.end_element();
    }
}

fn write_image_fallback(
    xml: &mut XmlWriter,
    pos: &Point,
    width: Pt,
    height: Pt,
    reason: &str,
) {
    xml.start_element("g");
    xml.write_attribute("data-crystalline-image-fallback", reason);
    xml.write_attribute("data-x", &fmt_num(pos.x.0));
    xml.write_attribute("data-y", &fmt_num(pos.y.0));
    xml.write_attribute("data-width", &fmt_num(width.0));
    xml.write_attribute("data-height", &fmt_num(height.0));
    xml.end_element();
}

fn write_image_clip(
    xml: &mut XmlWriter,
    clip: &typst_core::entities::layout_types::Rect,
    next_id: &mut usize,
) -> Option<String> {
    let values = [clip.x.0, clip.y.0, clip.w.0, clip.h.0];
    if !values.into_iter().all(f64::is_finite) || clip.w.0 < 0.0 || clip.h.0 < 0.0 {
        return None;
    }
    let id = format!("c{}", *next_id);
    *next_id += 1;
    xml.start_element("defs");
    xml.start_element("clipPath");
    xml.write_attribute("id", &id);
    xml.write_attribute("clipPathUnits", "userSpaceOnUse");
    xml.start_element("rect");
    xml.write_attribute("x", &fmt_num(clip.x.0));
    xml.write_attribute("y", &fmt_num(clip.y.0));
    xml.write_attribute("width", &fmt_num(clip.w.0));
    xml.write_attribute("height", &fmt_num(clip.h.0));
    xml.end_element();
    xml.end_element();
    xml.end_element();
    Some(id)
}

fn image_orientation_matrix(
    orientation: u32,
    width: f64,
    height: f64,
    x: f64,
    y: f64,
) -> TransformMatrix {
    let (a, b, c, d, tx, ty) = match orientation {
        2 => (-width, 0.0, 0.0, height, width, 0.0),
        3 => (-width, 0.0, 0.0, -height, width, height),
        4 => (width, 0.0, 0.0, -height, 0.0, height),
        5 => (0.0, height, width, 0.0, 0.0, 0.0),
        6 => (0.0, height, -width, 0.0, width, 0.0),
        7 => (0.0, -height, -width, 0.0, width, height),
        8 => (0.0, -height, width, 0.0, 0.0, height),
        _ => (width, 0.0, 0.0, height, 0.0, 0.0),
    };
    TransformMatrix { a, b, c, d, tx: tx + x, ty: ty + y }
}

fn self_contained_svg(data: &[u8]) -> bool {
    let Ok(mut text) = std::str::from_utf8(data) else { return false };
    text = text.strip_prefix('\u{feff}').unwrap_or(text).trim_start();
    loop {
        if text.starts_with("<?xml") {
            let Some(end) = text.find("?>") else { return false };
            text = text[end + 2..].trim_start();
        } else if text.starts_with("<!--") {
            let Some(end) = text.find("-->") else { return false };
            text = text[end + 3..].trim_start();
        } else {
            break;
        }
    }
    let root_is_svg = text.starts_with("<svg")
        && text
            .as_bytes()
            .get(4)
            .is_some_and(|b| b.is_ascii_whitespace() || *b == b'>');
    root_is_svg
        && !text.contains("href=\"http://")
        && !text.contains("href=\"https://")
        && !text.contains("href='http://")
        && !text.contains("href='https://")
        && !text.contains("url(http://")
        && !text.contains("url(https://")
}
