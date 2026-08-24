//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/svg.md
//! @prompt-hash f9b6e2c1
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

use std::collections::HashMap;
use std::sync::Arc;

use base64::Engine;
use ttf_parser::{Face, GlyphId, OutlineBuilder};
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
    /// Se true, inclui o canvas físico; false recorta ao TrimBox.
    pub render_bleed: bool,
}

impl Default for SvgOptions {
    fn default() -> Self {
        Self { pretty: false, render_bleed: false }
    }
}

/// Tipo da chave de fontes usada pelas variantes `_with_fonts`.
pub type FontKey = ((FontList, FontVariant, FontVariations), Vec<u8>);

/// Cache de definições de glifos para o SVG.
/// Cada glifo único (fonte + glyph_id + escala) é extraído uma vez e
/// referenciado via `<use xlink:href="#id"/>`.
#[derive(Default)]
struct GlyphDefs {
    next_id: usize,
    ids: HashMap<(usize, u16), String>,
    paths: Vec<(String, String)>,
}

impl GlyphDefs {
    /// Obtém ou cria um identificador para o glifo dado.
    fn get_or_insert(
        &mut self,
        font_idx: usize,
        glyph_id: u16,
        font_data: &[u8],
        scale: f64,
    ) -> Option<String> {
        let key = (font_idx, glyph_id);
        if let Some(id) = self.ids.get(&key) {
            return Some(id.clone());
        }

        let face = Face::parse(font_data, 0).ok()?;
        let mut builder = SvgGlyphPathBuilder::new(scale);
        face.outline_glyph(GlyphId(glyph_id), &mut builder)?;
        let path = builder.finish();

        let id = format!("g{}", self.next_id);
        self.next_id += 1;
        self.ids.insert(key, id.clone());
        self.paths.push((id.clone(), path));
        Some(id)
    }

    fn write(&self, xml: &mut XmlWriter) {
        if self.paths.is_empty() {
            return;
        }
        xml.start_element("defs");
        for (id, path) in &self.paths {
            xml.start_element("symbol");
            xml.write_attribute("id", id);
            xml.write_attribute("overflow", "visible");
            xml.start_element("path");
            xml.write_attribute("d", path);
            xml.end_element();
            xml.end_element();
        }
        xml.end_element();
    }
}

/// Builder que converte um outline TrueType em string SVG `d`.
/// Usa comandos relativos (`m`, `l`, `q`, `c`, `Z`), como o vanilla.
struct SvgGlyphPathBuilder {
    path: String,
    scale: f64,
    last_point: (f64, f64),
    last_close: (f64, f64),
}

impl SvgGlyphPathBuilder {
    fn new(scale: f64) -> Self {
        Self {
            path: String::from("M 0 0"),
            scale,
            last_point: (0.0, 0.0),
            last_close: (0.0, 0.0),
        }
    }

    fn finish(self) -> String {
        self.path
    }

    fn push(&mut self, cmd: &str, coords: &[f64]) {
        self.path.push(' ');
        self.path.push_str(cmd);
        for c in coords {
            self.path.push(' ');
            self.path.push_str(&fmt_num(c * self.scale));
        }
    }

    fn map(&self, x: f32, y: f32) -> (f64, f64) {
        let x = x as f64 - self.last_point.0;
        let y = y as f64 - self.last_point.1;
        (x, y)
    }
}

impl OutlineBuilder for SvgGlyphPathBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        let (dx, dy) = self.map(x, y);
        self.last_close = (x as f64, y as f64);
        self.last_point = (x as f64, y as f64);
        if dx != 0.0 || dy != 0.0 {
            self.push("m", &[dx, dy]);
        }
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let (dx, dy) = self.map(x, y);
        self.last_point = (x as f64, y as f64);
        if dx != 0.0 && dy != 0.0 {
            self.push("l", &[dx, dy]);
        } else if dx != 0.0 {
            self.push("h", &[dx]);
        } else if dy != 0.0 {
            self.push("v", &[dy]);
        }
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let (dx1, dy1) = self.map(x1, y1);
        let (dx, dy) = self.map(x, y);
        self.last_point = (x as f64, y as f64);
        self.push("q", &[dx1, dy1, dx, dy]);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let (dx1, dy1) = self.map(x1, y1);
        let (dx2, dy2) = self.map(x2, y2);
        let (dx, dy) = self.map(x, y);
        self.last_point = (x as f64, y as f64);
        self.push("c", &[dx1, dy1, dx2, dy2, dx, dy]);
    }

    fn close(&mut self) {
        self.path.push_str(" Z");
        self.last_point = self.last_close;
    }
}

/// Exporta uma página para SVG (texto sem fontes resolvidas é omitido).
pub fn export_svg(page: &Page, opts: &SvgOptions) -> String {
    export_svg_with_fonts_inner(page, opts, None)
}

/// Exporta uma página para SVG com fontes resolvidas.
///
/// O texto é renderizado como elementos `<text>` com `font-family`
/// resolvida a partir do mapa de `fonts`. Se uma fonte não for
/// encontrada no mapa, o run é omitido.
pub fn export_svg_with_fonts(
    page: &Page,
    opts: &SvgOptions,
    fonts: &[FontKey],
) -> String {
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

    let w = if opts.render_bleed { page.canvas_width() } else { page.width };
    let h = if opts.render_bleed { page.canvas_height() } else { page.height };

    xml.start_element("svg");
    xml.write_attribute("xmlns", "http://www.w3.org/2000/svg");
    xml.write_attribute("xmlns:xlink", "http://www.w3.org/1999/xlink");
    xml.write_attribute("viewBox", &format!("0 0 {w} {h}"));
    xml.write_attribute("width", &format!("{w}pt"));
    xml.write_attribute("height", &format!("{h}pt"));

    if let Some(fill) = match &page.fill {
        typst_core::entities::page_canvas::PageFill::Auto => Some("#ffffff".to_string()),
        typst_core::entities::page_canvas::PageFill::None => None,
        typst_core::entities::page_canvas::PageFill::Paint(paint) => {
            Some(color_to_css(paint.to_color()))
        }
    } {
        xml.start_element("rect");
        xml.write_attribute("x", "0");
        xml.write_attribute("y", "0");
        xml.write_attribute("width", &fmt_num(w));
        xml.write_attribute("height", &fmt_num(h));
        xml.write_attribute("fill", &fill);
        xml.end_element();
    }

    let mut glyph_defs = GlyphDefs::default();
    if opts.render_bleed {
        xml.start_element("g");
        xml.write_attribute(
            "transform",
            &format!("translate({} {})", page.bleed.left, page.bleed.top),
        );
    }
    for item in page
        .background
        .iter()
        .chain(page.items.iter())
        .chain(page.foreground.iter())
    {
        render_item(&mut xml, item, fonts, &mut glyph_defs);
    }
    if opts.render_bleed {
        xml.end_element();
    }

    // Emite as definições de glifos antes de fechar o SVG.
    glyph_defs.write(&mut xml);

    xml.end_element();
    xml.end_document()
}

fn render_item(
    xml: &mut XmlWriter,
    item: &FrameItem,
    fonts: Option<&[FontKey]>,
    glyph_defs: &mut GlyphDefs,
) {
    match item {
        FrameItem::Text { .. } => {
            // P483 — Text plano está deprecated; sem fontes resolvidas
            // não há como renderizar texto fiável.
        }
        FrameItem::TextShaped { pos, glyphs, style, text, units_per_em } => {
            if let Some(fonts) = fonts {
                render_text_shaped(
                    xml,
                    pos,
                    glyphs,
                    style,
                    text,
                    *units_per_em,
                    fonts,
                    glyph_defs,
                );
            }
        }
        FrameItem::Line { start, end, thickness, color } => {
            render_line(xml, start, end, *thickness, *color);
        }
        FrameItem::Glyph { .. } => {
            // Glifos matemáticos directos: scope-out neste passo.
        }
        FrameItem::Image { pos, data, width, height, orientation, .. } => {
            render_image(xml, pos, data, *width, *height, *orientation);
        }
        FrameItem::Shape { pos, kind, width, height, fill, stroke, .. } => {
            render_shape(xml, pos, kind, *width, *height, *fill, stroke.as_ref());
        }
        FrameItem::Group { pos, matrix, clip_mask, items, .. } => {
            render_group(xml, pos, matrix, clip_mask.as_ref(), items, fonts, glyph_defs);
        }
        FrameItem::Link { target, items, pos, size } => {
            render_link(xml, target, items, pos, size, fonts, glyph_defs);
        }
        FrameItem::Semantic { items, .. } => {
            for item in items {
                render_item(xml, item, fonts, glyph_defs);
            }
        }
    }
}

fn render_text_shaped(
    xml: &mut XmlWriter,
    pos: &Point,
    glyphs: &[ShapedGlyph],
    style: &typst_core::entities::layout_types::TextStyle,
    _text: &ecow::EcoString,
    units_per_em: u16,
    fonts: &[FontKey],
    glyph_defs: &mut GlyphDefs,
) {
    let Some(font_list) = &style.font else {
        return;
    };
    let variant = text_style_to_font_variant(style);
    let variations = style.variations.clone().unwrap_or_default();
    let Some((font_idx, (_, font_bytes))) =
        fonts.iter().enumerate().find(|(_, ((fl, v, var), _))| {
            fl == font_list && v == &variant && var == &variations
        })
    else {
        return;
    };

    if units_per_em == 0 {
        return;
    }
    let scale = style.size.0 / f64::from(units_per_em);

    // Inverte o eixo Y porque fontes usam coordenadas Y-up, enquanto o SVG
    // usa Y-down. O vanilla faz exactamente isto com
    // `matrix(1 0 0 -1 x y)`.
    xml.start_element("g");
    xml.write_attribute(
        "transform",
        &format!("matrix(1 0 0 -1 {} {})", fmt_num(pos.x.0), fmt_num(pos.y.0)),
    );

    let fill = style.fill.map(color_to_css).unwrap_or_else(|| "#000000".to_string());
    let mut x = 0.0;
    let mut y = 0.0;
    for glyph in glyphs {
        let x_offset = f64::from(glyph.x_offset) * scale;
        let y_offset = f64::from(glyph.y_offset) * scale;
        let Some(id) =
            glyph_defs.get_or_insert(font_idx, glyph.glyph_id, font_bytes, scale)
        else {
            x += f64::from(glyph.x_advance) * scale;
            continue;
        };

        xml.start_element("use");
        xml.write_attribute("xlink:href", &format!("#{id}"));
        xml.write_attribute("x", &fmt_num(x + x_offset));
        xml.write_attribute("y", &fmt_num(y + y_offset));
        xml.write_attribute("fill", &fill);
        xml.end_element();

        x += f64::from(glyph.x_advance) * scale;
    }

    xml.end_element();
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
    kind: &ShapeKind<typst_core::entities::layout_types::Pt>,
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
            xml.start_element("path");
            xml.write_attribute(
                "d",
                &rounded_rect_path_data(pos.x.0, pos.y.0, width, height, radii),
            );
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

fn rounded_rect_path_data(
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    radii: &typst_core::entities::corners::Corners<
        typst_core::entities::layout_types::Pt,
    >,
) -> String {
    let max_r = width.min(height) / 2.0;
    let tl = radii.top_left.0.clamp(0.0, max_r);
    let tr = radii.top_right.0.clamp(0.0, max_r);
    let br = radii.bottom_right.0.clamp(0.0, max_r);
    let bl = radii.bottom_left.0.clamp(0.0, max_r);
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
    _clip_mask: Option<&ShapeKind<typst_core::entities::layout_types::Pt>>,
    items: &[FrameItem],
    fonts: Option<&[FontKey]>,
    glyph_defs: &mut GlyphDefs,
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
        render_item(xml, item, fonts, glyph_defs);
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
    glyph_defs: &mut GlyphDefs,
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
        render_item(xml, item, fonts, glyph_defs);
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
    m.a == 1.0 && m.b == 0.0 && m.c == 0.0 && m.d == 1.0 && m.tx == 0.0 && m.ty == 0.0
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
    use ecow::EcoString;
    use typst_core::entities::corners::Corners;
    use typst_core::entities::font_book::FontVariant;
    use typst_core::entities::font_list::FontList;
    use typst_core::entities::font_variations::FontVariations;
    use typst_core::entities::geometry::ShapeKind;
    use typst_core::entities::layout_types::{FrameItem, Page, Point, Pt, TextStyle};
    use typst_core::entities::shaped_glyph::ShapedGlyph;

    #[test]
    fn empty_page_produces_svg() {
        let page = Page {
            width: 100.0,
            height: 150.0,
            numbering: None,
            supplement: typst_core::entities::content::Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
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
            supplement: typst_core::entities::content::Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
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

    #[test]
    fn p1133_rounded_rect_svg_preserva_quatro_cantos_em_pt() {
        let d = rounded_rect_path_data(
            10.0,
            20.0,
            100.0,
            80.0,
            &Corners::new(Pt(2.0), Pt(4.0), Pt(6.0), Pt(8.0)),
        );
        assert!(d.starts_with("M 12 20 L 106 20"));
        assert!(d.contains("L 110 94"));
        assert!(d.contains("L 18 100"));
        assert!(d.contains("L 10 22"));
    }

    #[test]
    fn text_shaped_emits_glyph_paths_not_text() {
        // NotoSans-Regular tem glifo 0 (.notdef) com outline; usamo-lo
        // como canário para confirmar que TextShaped gera <use>/<symbol>.
        let font_data = include_bytes!(
            "../../../lab/krilla-reference/assets/fonts/NotoSans-Regular.ttf"
        ) as &[u8];
        let font_list = FontList::single(EcoString::from("noto sans"));
        let variant = FontVariant::default();
        let variations = FontVariations::default();
        let key = ((font_list.clone(), variant, variations), font_data.to_vec());

        let style = TextStyle {
            size: Pt(11.0),
            font: Some(font_list),
            ..TextStyle::default()
        };

        let page = Page {
            width: 200.0,
            height: 200.0,
            numbering: None,
            supplement: typst_core::entities::content::Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items: vec![FrameItem::TextShaped {
                pos: Point { x: Pt(10.0), y: Pt(20.0) },
                glyphs: vec![ShapedGlyph {
                    glyph_id: 0,
                    x_advance: 500,
                    x_offset: 0,
                    y_offset: 0,
                    cluster: 0,
                    char_code: ' ',
                }],
                style,
                text: EcoString::from(" "),
                units_per_em: 1000,
            }],
        };

        let svg = export_svg_with_fonts(&page, &SvgOptions::default(), &[key]);
        assert!(svg.contains("<use"), "deve usar <use> para glifos");
        assert!(svg.contains("<symbol"), "deve definir glifos via <symbol>");
        assert!(svg.contains("<path"), "deve conter path do glifo");
        assert!(!svg.contains("<text"), "nao deve usar <text>");
    }
}
