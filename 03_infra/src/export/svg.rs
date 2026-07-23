//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/svg.md
//! @prompt-hash 2efe9af1
//! @layer L3
//! @updated 2026-07-23
//!
//! Exportador de páginas Typst para SVG.
//!
//! Baseado na estrutura de `typst-svg` do vanilla
//! (`lab/typst-original/crates/typst-svg/`), mas adaptado aos tipos
//! cristalinos (`FrameItem`, `ShapeKind`, `Color`, etc.).
//!
//! P870 — implementa as variantes simples (sem fontes resolvidas,
//! texto omitido/tofu) e `_with_fonts` (texto como `<text>` com
//! `font-family` resolvida).

use std::sync::Arc;

use base64::Engine;
use xmlwriter::XmlWriter;

use typst_core::entities::font_book::FontVariant;
use typst_core::entities::font_list::FontList;
use typst_core::entities::font_variations::FontVariations;
use typst_core::entities::geometry::{PathItem, ShapeKind, Stroke};
use typst_core::entities::image_format::{detect_image_format, ImageFormat};
use typst_core::entities::layout_types::{
    Color, FrameItem, LinkTarget, Page, Point, Pt, Size, TransformMatrix,
};

use typst_core::entities::shaped_glyph::ShapedGlyph;

use crate::font_variant::text_style_to_font_variant;

/// Opções de exportação SVG.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SvgOptions {
    /// Se true, formata o SVG com indentação.
    pub pretty: bool,
}

impl Default for SvgOptions {
    fn default() -> Self {
        Self { pretty: false }
    }
}

/// Tipo da chave de fontes usada pelas variantes `_with_fonts`.
pub type FontKey = ((FontList, FontVariant, FontVariations), Vec<u8>);

/// Exporta uma página para SVG (texto sem fontes resolvidas é omitido).
pub fn export_svg(page: &Page, opts: &SvgOptions) -> String {
    export_svg_with_fonts_inner(page, opts, None)
}

/// Exporta uma página para SVG com fontes resolvidas.
///
/// O texto é renderizado como elementos `<text>` com `font-family`
/// resolvida a partir do mapa de `fonts`. Se uma fonte não for
/// encontrada no mapa, o run é omitido.
pub fn export_svg_with_fonts(page: &Page, opts: &SvgOptions, fonts: &[FontKey]) -> String {
    export_svg_with_fonts_inner(page, opts, Some(fonts))
}

fn export_svg_with_fonts_inner(
    page: &Page,
    opts: &SvgOptions,
    fonts: Option<&[FontKey]>,
) -> String {
    let mut xml = XmlWriter::new(xmlwriter::Options {
        use_single_quote: false,
        indent: if opts.pretty {
            xmlwriter::Indent::Spaces(2)
        } else {
            xmlwriter::Indent::None
        },
        attributes_indent: xmlwriter::Indent::None,
    });

    let w = page.width;
    let h = page.height;

    xml.start_element("svg");
    xml.write_attribute("xmlns", "http://www.w3.org/2000/svg");
    xml.write_attribute("xmlns:xlink", "http://www.w3.org/1999/xlink");
    xml.write_attribute("viewBox", &format!("0 0 {w} {h}"));
    xml.write_attribute("width", &format!("{w}pt"));
    xml.write_attribute("height", &format!("{h}pt"));

    // Fundo branco se não houver fill explícito na página.
    // O vanilla usa `page.fill_or_white()`; o cristalino ainda não
    // modela `fill` em `Page`, portanto assume branco para observáveis
    // consistentes.
    {
        xml.start_element("rect");
        xml.write_attribute("x", "0");
        xml.write_attribute("y", "0");
        xml.write_attribute("width", &fmt_num(w));
        xml.write_attribute("height", &fmt_num(h));
        xml.write_attribute("fill", "#ffffff");
        xml.end_element();
    }

    for item in &page.items {
        render_item(&mut xml, item, fonts);
    }

    xml.end_element();
    xml.end_document()
}

fn render_item(xml: &mut XmlWriter, item: &FrameItem, fonts: Option<&[FontKey]>) {
    match item {
        FrameItem::Text { .. } => {
            // P483 — Text plano está deprecated; sem fontes resolvidas
            // não há como renderizar texto fiável.
        }
        FrameItem::TextShaped { pos, glyphs, style, text, units_per_em } => {
            if let Some(fonts) = fonts {
                render_text_shaped(xml, pos, glyphs, style, text, *units_per_em, fonts);
            }
        }
        FrameItem::Line { start, end, thickness, color } => {
            render_line(xml, start, end, *thickness, *color);
        }
        FrameItem::Glyph { .. } => {
            // Glifos matemáticos directos: scope-out neste passo.
        }
        FrameItem::Image {
            pos,
            data,
            width,
            height,
            orientation,
            ..
        } => {
            render_image(xml, pos, data, *width, *height, *orientation);
        }
        FrameItem::Shape {
            pos,
            kind,
            width,
            height,
            fill,
            stroke,
            ..
        } => {
            render_shape(xml, pos, kind, *width, *height, *fill, stroke.as_ref());
        }
        FrameItem::Group {
            pos,
            matrix,
            clip_mask,
            items,
            ..
        } => {
            render_group(xml, pos, matrix, clip_mask.as_ref(), items, fonts);
        }
        FrameItem::Link { target, items, pos, size } => {
            render_link(xml, target, items, pos, size, fonts);
        }
    }
}

fn render_text_shaped(
    xml: &mut XmlWriter,
    pos: &Point,
    glyphs: &[ShapedGlyph],
    style: &typst_core::entities::layout_types::TextStyle,
    text: &ecow::EcoString,
    _units_per_em: u16,
    fonts: &[FontKey],
) {
    let Some(font_list) = &style.font else {
        return;
    };
    let variant = text_style_to_font_variant(style);
    let variations = style.variations.clone().unwrap_or_default();
    let Some((_, _font_bytes)) = fonts.iter().find(|((fl, v, var), _)| {
        fl == font_list && v == &variant && var == &variations
    }) else {
        return;
    };

    xml.start_element("text");
    xml.write_attribute("x", &fmt_num(pos.x.0));
    xml.write_attribute("y", &fmt_num(pos.y.0));
    xml.write_attribute("font-size", &fmt_num(style.size.0));
    let family = font_list
        .as_slice()
        .first()
        .and_then(|f| f.name.as_str())
        .unwrap_or("serif");
    xml.write_attribute("font-family", family);
    if style.bold {
        xml.write_attribute("font-weight", "bold");
    }
    if style.italic {
        xml.write_attribute("font-style", "italic");
    }
    if let Some(fill) = style.fill {
        xml.write_attribute("fill", &color_to_css(fill));
    } else {
        xml.write_attribute("fill", "#000000");
    }
    xml.write_attribute("dominant-baseline", "alphabetic");
    xml.write_text(text.as_str());
    xml.end_element();

    // Os glifos são ignorados aqui; usamos o texto plano. As posições
    // são aproximadas pela baseline. Isto é suficiente para casos simples.
    let _ = glyphs;
}

fn render_line(
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

fn render_image(
    xml: &mut XmlWriter,
    pos: &Point,
    data: &Arc<Vec<u8>>,
    width: Pt,
    height: Pt,
    _orientation: u32,
) {
    let format = detect_image_format(data);
    let mime = match format {
        ImageFormat::Jpeg => "image/jpeg",
        ImageFormat::Png => "image/png",
        ImageFormat::Gif => "image/gif",
        ImageFormat::WebP => "image/webp",
        ImageFormat::Unknown => return,
    };
    let b64 = base64::engine::general_purpose::STANDARD.encode(data.as_slice());
    let url = format!("data:{mime};base64,{b64}");

    xml.start_element("image");
    xml.write_attribute("x", &fmt_num(pos.x.0));
    xml.write_attribute("y", &fmt_num(pos.y.0));
    xml.write_attribute("width", &fmt_num(width.0));
    xml.write_attribute("height", &fmt_num(height.0));
    xml.write_attribute("xlink:href", &url);
    xml.write_attribute("preserveAspectRatio", "none");
    xml.end_element();
}

fn render_shape(
    xml: &mut XmlWriter,
    pos: &Point,
    kind: &ShapeKind,
    width: f64,
    height: f64,
    fill: Option<Color>,
    stroke: Option<&Stroke>,
) {
    let attrs = ShapeAttrs {
        fill: fill.map(color_to_css),
        stroke: stroke.map(|s| color_to_css(s.paint.to_color())),
        stroke_width: stroke.map(|s| s.thickness),
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
            let radius = radii.top_left.resolve_pt(0.0);
            xml.start_element("rect");
            xml.write_attribute("x", &fmt_num(pos.x.0));
            xml.write_attribute("y", &fmt_num(pos.y.0));
            xml.write_attribute("width", &fmt_num(width));
            xml.write_attribute("height", &fmt_num(height));
            xml.write_attribute("rx", &fmt_num(radius));
            xml.write_attribute("ry", &fmt_num(radius));
            write_shape_attrs(xml, &attrs);
            xml.end_element();
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
            let d = path_items_to_svg(items, pos);
            xml.start_element("path");
            xml.write_attribute("d", &d);
            write_shape_attrs(xml, &attrs);
            xml.end_element();
        }
    }
}

struct ShapeAttrs {
    fill: Option<String>,
    stroke: Option<String>,
    stroke_width: Option<f64>,
}

fn write_shape_attrs(xml: &mut XmlWriter, attrs: &ShapeAttrs) {
    if let Some(fill) = &attrs.fill {
        xml.write_attribute("fill", fill);
    } else {
        xml.write_attribute("fill", "none");
    }
    if let Some(stroke) = &attrs.stroke {
        xml.write_attribute("stroke", stroke);
    }
    if let Some(width) = attrs.stroke_width {
        xml.write_attribute("stroke-width", &fmt_num(width));
    }
}

fn render_group(
    xml: &mut XmlWriter,
    pos: &Point,
    matrix: &TransformMatrix,
    _clip_mask: Option<&ShapeKind>,
    items: &[FrameItem],
    fonts: Option<&[FontKey]>,
) {
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
    // clip_mask: scope-out neste passo.
    for item in items {
        render_item(xml, item, fonts);
    }
    xml.end_element();
}

fn render_link(
    xml: &mut XmlWriter,
    target: &LinkTarget,
    items: &[FrameItem],
    _pos: &Point,
    _size: &Size,
    fonts: Option<&[FontKey]>,
) {
    xml.start_element("a");
    match target {
        LinkTarget::Url(url) => {
            xml.write_attribute("xlink:href", url.as_str());
        }
        LinkTarget::Destination(_) => {
            // Links internos: scope-out.
        }
    }
    for item in items {
        render_item(xml, item, fonts);
    }
    xml.end_element();
}

fn path_items_to_svg(items: &[PathItem], offset: &Point) -> String {
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

fn color_to_css(color: Color) -> String {
    let (r, g, b, a) = color.to_srgb();
    if a == 255 {
        format!("#{r:02x}{g:02x}{b:02x}")
    } else {
        format!("rgba({r},{g},{b},{:.3})", a as f64 / 255.0)
    }
}

fn is_identity(m: &TransformMatrix) -> bool {
    m.a == 1.0
        && m.b == 0.0
        && m.c == 0.0
        && m.d == 1.0
        && m.tx == 0.0
        && m.ty == 0.0
}

fn fmt_num(n: f64) -> String {
    const ROUNDING_FACTOR: f64 = 1_000_000_000.0;
    let n = (ROUNDING_FACTOR * n).round() / ROUNDING_FACTOR;
    if n == n as i64 as f64 {
        format!("{}", n as i64)
    } else {
        let mut buf = ryu::Buffer::new();
        buf.format(n).to_string()
    }
}

fn fmt_pair(x: f64, y: f64) -> String {
    format!("{} {}", fmt_num(x), fmt_num(y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::entities::geometry::ShapeKind;
    use typst_core::entities::layout_types::{FrameItem, Page, Point, Pt};

    #[test]
    fn empty_page_produces_svg() {
        let page = Page {
            width: 100.0,
            height: 150.0,
            numbering: None,
            items: vec![],
        };
        let svg = export_svg(&page, &SvgOptions::default());
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("viewBox=\"0 0 100 150\""));
    }

    #[test]
    fn rect_shape_produces_rect_element() {
        let page = Page {
            width: 200.0,
            height: 200.0,
            numbering: None,
            items: vec![FrameItem::Shape {
                pos: Point { x: Pt(10.0), y: Pt(20.0) },
                kind: ShapeKind::Rect,
                width: 50.0,
                height: 60.0,
                fill: Some(Color::rgb(255, 0, 0)),
                stroke: None,
                parent_bbox_at_emit: None,
            }],
        };
        let svg = export_svg(&page, &SvgOptions::default());
        assert!(svg.contains("<rect"));
        assert!(svg.contains("fill=\"#ff0000\""));
    }
}
