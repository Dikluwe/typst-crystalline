//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/render.md
//! @prompt-hash 219ac788
//! @layer L3
//! @updated 2026-07-23
//!
//! Rasterizador de páginas Typst para PNG.
//!
//! Baseado na estrutura de `typst-render` do vanilla
//! (`lab/typst-original/crates/typst-render/`), mas adaptado aos tipos
//! cristalinos (`FrameItem`, `ShapeKind`, `Color`, etc.).
//!
//! P870 — implementa as variantes simples (sem fontes resolvidas,
//! texto omitido) e `_with_fonts` (texto rasterizado via `pixglyph`).

use std::sync::Arc;

use tiny_skia as sk;
use ttf_parser::GlyphId;

use typst_core::entities::font_book::FontVariant;
use typst_core::entities::font_list::FontList;
use typst_core::entities::font_variations::FontVariations;
use typst_core::entities::geometry::{PathItem, ShapeKind, Stroke};
use typst_core::entities::image_format::{detect_image_format, ImageFormat};

use super::pdf_defaults;
use typst_core::entities::layout_types::{
    Color, FrameItem, Page, PagedDocument, Point, Pt, TransformMatrix,
};
use typst_core::entities::shaped_glyph::ShapedGlyph;

use crate::font_variant::text_style_to_font_variant;

/// Tipo da chave de fontes usada pelas variantes `_with_fonts`.
pub type FontKey = ((FontList, FontVariant, FontVariations), Vec<u8>);

/// Opções de rasterização PNG.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderOptions {
    /// Pixels por ponto tipográfico. Default 2.0 (paridade vanilla).
    pub pixel_per_pt: f32,
    /// Se true, expande a área renderizada para incluir bleed.
    pub render_bleed: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self { pixel_per_pt: 2.0, render_bleed: false }
    }
}

/// Estado de renderização — transformação afim.
#[derive(Clone, Copy)]
struct State {
    transform: sk::Transform,
}

impl State {
    fn new(pixel_per_pt: f32) -> Self {
        Self {
            transform: sk::Transform::from_scale(pixel_per_pt, pixel_per_pt),
        }
    }

    fn pre_translate(self, x: f32, y: f32) -> Self {
        Self { transform: self.transform.pre_translate(x, y) }
    }

    fn pre_concat(self, other: sk::Transform) -> Self {
        Self { transform: self.transform.pre_concat(other) }
    }
}

/// Renderiza uma página para PNG (texto sem fontes resolvidas é omitido).
pub fn render_page_to_png(page: &Page, opts: &RenderOptions) -> Vec<u8> {
    render_page_to_png_with_fonts_inner(page, opts, None)
}

/// Renderiza uma página para PNG com fontes resolvidas.
pub fn render_page_to_png_with_fonts(
    page: &Page,
    opts: &RenderOptions,
    fonts: &[FontKey],
) -> Vec<u8> {
    render_page_to_png_with_fonts_inner(page, opts, Some(fonts))
}

fn render_page_to_png_with_fonts_inner(
    page: &Page,
    opts: &RenderOptions,
    fonts: Option<&[FontKey]>,
) -> Vec<u8> {
    let pixel_per_pt = opts.pixel_per_pt.max(0.01);
    let width = if opts.render_bleed { page.canvas_width() } else { page.width };
    let height = if opts.render_bleed { page.canvas_height() } else { page.height };
    let pxw = (pixel_per_pt * width as f32).round().max(1.0) as u32;
    let pxh = (pixel_per_pt * height as f32).round().max(1.0) as u32;

    let mut canvas = sk::Pixmap::new(pxw, pxh).expect("tamanho validado acima");
    match &page.fill {
        typst_core::entities::page_canvas::PageFill::Auto => {
            canvas.fill(sk::Color::WHITE)
        }
        typst_core::entities::page_canvas::PageFill::None => {}
        typst_core::entities::page_canvas::PageFill::Paint(paint) => {
            canvas.fill(color_to_sk(paint.to_color()));
        }
    }

    let mut state = State::new(pixel_per_pt);
    if opts.render_bleed {
        state = state.pre_translate(page.bleed.left as f32, page.bleed.top as f32);
    }
    for item in page
        .background
        .iter()
        .chain(page.items.iter())
        .chain(page.foreground.iter())
    {
        render_item(&mut canvas, state, item, fonts);
    }

    encode_png(&canvas)
}

/// Renderiza um documento completo, empilhando páginas verticalmente.
pub fn render_document_to_png(
    doc: &PagedDocument,
    opts: &RenderOptions,
    gap_pt: f64,
) -> Vec<u8> {
    let pixel_per_pt = opts.pixel_per_pt.max(0.01);
    let gap = (pixel_per_pt * gap_pt as f32).round() as u32;

    let pixmaps: Vec<_> =
        doc.pages.iter().map(|page| render_page_to_png(page, opts)).collect();

    // Re-decodifica os PNGs para pixmaps tiny-skia para os empilhar.
    // Abordagem simples; performance não é objectivo deste passo.
    let mut widths = Vec::new();
    let mut heights = Vec::new();
    let mut decoded = Vec::new();
    for bytes in &pixmaps {
        if let Ok(img) = image::load_from_memory(bytes) {
            let rgba = img.to_rgba8();
            widths.push(rgba.width());
            heights.push(rgba.height());
            decoded.push(rgba.into_raw());
        }
    }

    let pxw = widths.iter().copied().max().unwrap_or(0);
    let pxh = heights.iter().copied().sum::<u32>()
        + gap * heights.len().saturating_sub(1) as u32;

    let mut canvas = sk::Pixmap::new(pxw, pxh)
        .unwrap_or_else(|| sk::Pixmap::new(1, 1).expect("1x1 é sempre válido"));
    canvas.fill(sk::Color::from_rgba(1.0, 1.0, 1.0, 1.0).unwrap());

    let mut y = 0i32;
    for (i, raw) in decoded.iter().enumerate() {
        let w = widths[i];
        let h = heights[i];
        let Some(pixmap) =
            sk::Pixmap::from_vec(raw.clone(), sk::IntSize::from_wh(w, h).unwrap())
        else {
            y += h as i32 + gap as i32;
            continue;
        };
        canvas.draw_pixmap(
            0,
            y,
            pixmap.as_ref(),
            &sk::PixmapPaint::default(),
            sk::Transform::identity(),
            None,
        );
        y += h as i32 + gap as i32;
    }

    encode_png(&canvas)
}

fn render_item(
    canvas: &mut sk::Pixmap,
    state: State,
    item: &FrameItem,
    fonts: Option<&[FontKey]>,
) {
    match item {
        FrameItem::Text { .. } => {}
        FrameItem::TextShaped { pos, glyphs, style, text, units_per_em } => {
            if let Some(fonts) = fonts {
                render_text_shaped(
                    canvas,
                    state,
                    pos,
                    glyphs,
                    style,
                    text,
                    *units_per_em,
                    fonts,
                );
            }
        }
        FrameItem::Line { start, end, thickness, color } => {
            render_line(canvas, state, start, end, *thickness, *color);
        }
        FrameItem::Glyph { .. } => {}
        FrameItem::Image { pos, data, width, height, orientation, .. } => {
            render_image(canvas, state, pos, data, *width, *height, *orientation);
        }
        FrameItem::Shape { pos, kind, width, height, fill, stroke, .. } => {
            render_shape(
                canvas,
                state,
                pos,
                kind,
                *width,
                *height,
                *fill,
                stroke.as_ref(),
            );
        }
        FrameItem::Group { pos, matrix, clip_mask, items, .. } => {
            render_group(canvas, state, pos, matrix, clip_mask.as_ref(), items, fonts);
        }
        FrameItem::Link { items, .. } => {
            for item in items {
                render_item(canvas, state, item, fonts);
            }
        }
        FrameItem::Semantic { items, .. } => {
            for item in items {
                render_item(canvas, state, item, fonts);
            }
        }
    }
}

fn render_text_shaped(
    canvas: &mut sk::Pixmap,
    state: State,
    pos: &Point,
    glyphs: &[ShapedGlyph],
    style: &typst_core::entities::layout_types::TextStyle,
    _text: &ecow::EcoString,
    units_per_em: u16,
    fonts: &[FontKey],
) {
    let Some(font_list) = &style.font else {
        return;
    };
    let variant = text_style_to_font_variant(style);
    let variations = style.variations.clone().unwrap_or_default();
    let Some((_, font_bytes)) = fonts
        .iter()
        .find(|((fl, v, var), _)| fl == font_list && v == &variant && var == &variations)
    else {
        return;
    };

    let face = match ttf_parser::Face::parse(font_bytes, 0) {
        Ok(f) => f,
        Err(_) => return,
    };

    let upem = units_per_em.max(1) as f64;
    let size = style.size.0;
    let scale = size / upem;
    let fill = style.fill.unwrap_or(Color::rgb(0, 0, 0));
    let sk_color = color_to_sk(fill);

    let mut x = 0.0f64;
    let y = 0.0f64;
    for glyph in glyphs {
        let gid = GlyphId(glyph.glyph_id);
        let x_offset = x + glyph.x_offset as f64 * scale;
        let y_offset = y + glyph.y_offset as f64 * scale;

        let gx = (pos.x.0 + x_offset) as f32;
        let gy = (pos.y.0 - y_offset) as f32;

        render_glyph(canvas, state, &face, gid, gx, gy, size as f32, sk_color);

        x += glyph.x_advance as f64 * scale;
    }
}

fn render_glyph(
    canvas: &mut sk::Pixmap,
    state: State,
    face: &ttf_parser::Face<'_>,
    id: GlyphId,
    x: f32,
    y: f32,
    size: f32,
    color: sk::Color,
) {
    let Some(glyph) = pixglyph::Glyph::load(face, id) else {
        return;
    };

    let ts = state.transform;
    let ppem = size * ts.sy;

    let bitmap = glyph.rasterize(ts.tx + x * ts.sx, ts.ty + y * ts.sy, ppem);

    let mw = bitmap.width as i32;
    let mh = bitmap.height as i32;
    let left = bitmap.left;
    let top = bitmap.top;
    let cw = canvas.width() as i32;
    let ch = canvas.height() as i32;

    let right = left + mw;
    let bottom = top + mh;

    let color_u8 = sk::ColorU8::from_rgba(
        (color.red() * 255.0).round() as u8,
        (color.green() * 255.0).round() as u8,
        (color.blue() * 255.0).round() as u8,
        (color.alpha() * 255.0).round() as u8,
    );
    let color_premul = color_u8.premultiply();
    let color_u32: u32 = bytemuck::cast(color_premul);
    let pixels = bytemuck::cast_slice_mut::<u8, u32>(canvas.data_mut());

    for px in left.clamp(0, cw)..right.clamp(0, cw) {
        for py in top.clamp(0, ch)..bottom.clamp(0, ch) {
            let ai = ((py - top) * mw + (px - left)) as usize;
            let cov = bitmap.coverage[ai];
            if cov == 0 {
                continue;
            }

            let applied = alpha_mul(color_u32, cov as u32);
            let pi = (py * cw + px) as usize;
            pixels[pi] = blend_src_over(applied, pixels[pi]);
        }
    }
}

fn render_line(
    canvas: &mut sk::Pixmap,
    state: State,
    start: &Point,
    end: &Point,
    thickness: f64,
    color: Option<Color>,
) {
    let mut pb = sk::PathBuilder::new();
    pb.move_to(start.x.0 as f32, start.y.0 as f32);
    pb.line_to(end.x.0 as f32, end.y.0 as f32);
    let Some(path) = pb.finish() else { return };

    let paint = sk::Paint {
        shader: sk::Shader::SolidColor(color_to_sk(color.unwrap_or(Color::rgb(0, 0, 0)))),
        ..Default::default()
    };
    let stroke = sk::Stroke {
        width: thickness as f32,
        line_cap: sk::LineCap::Butt,
        ..Default::default()
    };
    canvas.stroke_path(&path, &paint, &stroke, state.transform, None);
}

fn render_image(
    canvas: &mut sk::Pixmap,
    state: State,
    pos: &Point,
    data: &Arc<Vec<u8>>,
    width: Pt,
    height: Pt,
    _orientation: u32,
) {
    let Some(texture) = build_texture(data, width.0 as f32, height.0 as f32) else {
        return;
    };

    let paint = sk::Paint {
        shader: sk::Pattern::new(
            texture.as_ref(),
            sk::SpreadMode::Pad,
            sk::FilterQuality::Nearest,
            1.0,
            sk::Transform::from_scale(
                width.0 as f32 / texture.width() as f32,
                height.0 as f32 / texture.height() as f32,
            ),
        ),
        ..Default::default()
    };

    let rect = sk::Rect::from_xywh(0.0, 0.0, width.0 as f32, height.0 as f32).unwrap();
    let ts = state.transform.pre_translate(pos.x.0 as f32, pos.y.0 as f32);
    canvas.fill_rect(rect, &paint, ts, None);
}

fn build_texture(data: &[u8], view_width: f32, view_height: f32) -> Option<sk::Pixmap> {
    let format = detect_image_format(data);
    if format == ImageFormat::Unknown {
        return None;
    }

    let img = image::load_from_memory(data).ok()?;
    let aspect = img.width() as f32 / img.height().max(1) as f32;
    let w = (view_width.max(aspect * view_height)).ceil().max(1.0) as u32;
    let h = ((w as f32 / aspect).ceil().max(1.0)) as u32;

    let resized = img.resize_exact(w, h, image::imageops::FilterType::Lanczos3);
    let rgba = resized.to_rgba8();

    let mut texture = sk::Pixmap::new(w, h)?;
    for (src, dest) in rgba.pixels().zip(texture.pixels_mut()) {
        let image::Rgba([r, g, b, a]) = *src;
        *dest = sk::ColorU8::from_rgba(r, g, b, a).premultiply();
    }
    Some(texture)
}

fn render_shape(
    canvas: &mut sk::Pixmap,
    state: State,
    pos: &Point,
    kind: &ShapeKind<typst_core::entities::layout_types::Pt>,
    width: f64,
    height: f64,
    fill: Option<Color>,
    stroke: Option<&Stroke>,
) {
    let ts = state.transform.pre_translate(pos.x.0 as f32, pos.y.0 as f32);

    let path = shape_to_path(kind, width, height);
    let Some(path) = path else { return };

    if let Some(fill) = fill {
        let paint = sk::Paint {
            shader: sk::Shader::SolidColor(color_to_sk(fill)),
            anti_alias: true,
            ..Default::default()
        };
        canvas.fill_path(&path, &paint, sk::FillRule::Winding, ts, None);
    }

    if let Some(stroke) = stroke {
        let paint = sk::Paint {
            shader: sk::Shader::SolidColor(color_to_sk(stroke.paint.to_color())),
            anti_alias: true,
            ..Default::default()
        };
        let sk_stroke = sk::Stroke {
            width: stroke.thickness as f32,
            line_cap: sk::LineCap::Butt,
            line_join: sk::LineJoin::Miter,
            ..Default::default()
        };
        canvas.stroke_path(&path, &paint, &sk_stroke, ts, None);
    }
}

fn shape_to_path(
    kind: &ShapeKind<typst_core::entities::layout_types::Pt>,
    width: f64,
    height: f64,
) -> Option<sk::Path> {
    match kind {
        ShapeKind::Rect => {
            let rect = sk::Rect::from_xywh(0.0, 0.0, width as f32, height as f32)?;
            Some(sk::PathBuilder::from_rect(rect))
        }
        ShapeKind::RoundedRect { radii } => {
            let rect = sk::Rect::from_xywh(0.0, 0.0, width as f32, height as f32)?;
            Some(rounded_rect_path(rect, radii))
        }
        ShapeKind::Ellipse => {
            let rx = (width / 2.0) as f32;
            let ry = (height / 2.0) as f32;
            let cx = rx;
            let cy = ry;
            Some(ellipse_path(cx, cy, rx, ry))
        }
        ShapeKind::Line { dx, dy } => {
            let mut pb = sk::PathBuilder::new();
            pb.move_to(0.0, 0.0);
            pb.line_to(*dx as f32, *dy as f32);
            pb.finish()
        }
        ShapeKind::Path(items) => path_items_to_sk(items),
    }
}

fn rounded_rect_path(
    rect: sk::Rect,
    radii: &typst_core::entities::corners::Corners<
        typst_core::entities::layout_types::Pt,
    >,
) -> sk::Path {
    let mut pb = sk::PathBuilder::new();
    let x = rect.left();
    let y = rect.top();
    let w = rect.width();
    let h = rect.height();
    let k = pdf_defaults::BEZIER_CIRCLE_KAPPA as f32;
    let max_r = w.min(h) / 2.0;
    let tl = (radii.top_left.0 as f32).clamp(0.0, max_r);
    let tr = (radii.top_right.0 as f32).clamp(0.0, max_r);
    let br = (radii.bottom_right.0 as f32).clamp(0.0, max_r);
    let bl = (radii.bottom_left.0 as f32).clamp(0.0, max_r);

    pb.move_to(x + tl, y);
    pb.line_to(x + w - tr, y);
    pb.cubic_to(x + w - tr + k * tr, y, x + w, y + tr - k * tr, x + w, y + tr);
    pb.line_to(x + w, y + h - br);
    pb.cubic_to(
        x + w,
        y + h - br + k * br,
        x + w - br + k * br,
        y + h,
        x + w - br,
        y + h,
    );
    pb.line_to(x + bl, y + h);
    pb.cubic_to(x + bl - k * bl, y + h, x, y + h - bl + k * bl, x, y + h - bl);
    pb.line_to(x, y + tl);
    pb.cubic_to(x, y + tl - k * tl, x + tl - k * tl, y, x + tl, y);
    pb.close();
    pb.finish().unwrap_or_else(|| sk::PathBuilder::from_rect(rect))
}

fn ellipse_path(cx: f32, cy: f32, rx: f32, ry: f32) -> sk::Path {
    let mut pb = sk::PathBuilder::new();
    let k = pdf_defaults::BEZIER_CIRCLE_KAPPA as f32;
    let kx = k * rx;
    let ky = k * ry;

    pb.move_to(cx - rx, cy);
    pb.cubic_to(cx - rx, cy - ky, cx - kx, cy - ry, cx, cy - ry);
    pb.cubic_to(cx + kx, cy - ry, cx + rx, cy - ky, cx + rx, cy);
    pb.cubic_to(cx + rx, cy + ky, cx + kx, cy + ry, cx, cy + ry);
    pb.cubic_to(cx - kx, cy + ry, cx - rx, cy + ky, cx - rx, cy);
    pb.close();
    pb.finish().unwrap_or_else(|| {
        sk::PathBuilder::from_rect(
            sk::Rect::from_xywh(cx - rx, cy - ry, rx * 2.0, ry * 2.0)
                .unwrap_or(sk::Rect::from_ltrb(0.0, 0.0, 1.0, 1.0).unwrap()),
        )
    })
}

fn path_items_to_sk(items: &[PathItem]) -> Option<sk::Path> {
    let mut pb = sk::PathBuilder::new();
    for item in items {
        match item {
            PathItem::MoveTo(p) => {
                pb.move_to(p.x.0 as f32, p.y.0 as f32);
            }
            PathItem::LineTo(p) => {
                pb.line_to(p.x.0 as f32, p.y.0 as f32);
            }
            PathItem::CubicTo(p1, p2, p3) => {
                pb.cubic_to(
                    p1.x.0 as f32,
                    p1.y.0 as f32,
                    p2.x.0 as f32,
                    p2.y.0 as f32,
                    p3.x.0 as f32,
                    p3.y.0 as f32,
                );
            }
            PathItem::ClosePath => {
                pb.close();
            }
        }
    }
    pb.finish()
}

fn render_group(
    canvas: &mut sk::Pixmap,
    state: State,
    pos: &Point,
    matrix: &TransformMatrix,
    _clip_mask: Option<&ShapeKind<typst_core::entities::layout_types::Pt>>,
    items: &[FrameItem],
    fonts: Option<&[FontKey]>,
) {
    let sk_matrix = sk::Transform::from_row(
        matrix.a as f32,
        matrix.b as f32,
        matrix.c as f32,
        matrix.d as f32,
        matrix.tx as f32,
        matrix.ty as f32,
    );
    let new_state = state
        .pre_translate(pos.x.0 as f32, pos.y.0 as f32)
        .pre_concat(sk_matrix);
    for item in items {
        render_item(canvas, new_state, item, fonts);
    }
}

fn color_to_sk(color: Color) -> sk::Color {
    let (r, g, b, a) = color.to_srgb();
    sk::Color::from_rgba(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0,
    )
    .unwrap_or(sk::Color::BLACK)
}

fn encode_png(canvas: &sk::Pixmap) -> Vec<u8> {
    let mut buf = Vec::new();
    let width = canvas.width();
    let height = canvas.height();
    let data = canvas.data();
    let rgba = image::RgbaImage::from_raw(width, height, data.to_vec())
        .unwrap_or_else(|| image::RgbaImage::new(width.max(1), height.max(1)));
    let dynamic = image::DynamicImage::ImageRgba8(rgba);
    let mut writer = std::io::Cursor::new(&mut buf);
    dynamic.write_to(&mut writer, image::ImageOutputFormat::Png).ok();
    buf
}

fn alpha_mul(color: u32, scale: u32) -> u32 {
    let mask = 0xff00ff;
    let rb = ((color & mask) * scale) >> 8;
    let ag = ((color >> 8) & mask) * scale;
    (rb & mask) | (ag & !mask)
}

fn blend_src_over(src: u32, dst: u32) -> u32 {
    src + alpha_mul(dst, 256 - (src >> 24))
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::entities::corners::Corners;
    use typst_core::entities::geometry::ShapeKind;
    use typst_core::entities::layout_types::{FrameItem, Page, Point, Pt};

    #[test]
    fn empty_page_produces_png() {
        let page = Page {
            width: 50.0,
            height: 50.0,
            numbering: None,
            supplement: typst_core::entities::content::Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items: vec![],
        };
        let png = render_page_to_png(&page, &RenderOptions::default());
        assert!(!png.is_empty());
        assert_eq!(&png[0..4], &[0x89, 0x50, 0x4E, 0x47]);
    }

    #[test]
    fn rect_shape_produces_non_empty_png() {
        let page = Page {
            width: 100.0,
            height: 100.0,
            numbering: None,
            supplement: typst_core::entities::content::Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items: vec![FrameItem::Shape {
                pos: Point { x: Pt(10.0), y: Pt(10.0) },
                kind: ShapeKind::Rect,
                width: 80.0,
                height: 80.0,
                fill: Some(Color::rgb(255, 0, 0)),
                stroke: None,
                parent_bbox_at_emit: None,
            }],
        };
        let png = render_page_to_png(
            &page,
            &RenderOptions { pixel_per_pt: 2.0, render_bleed: false },
        );
        assert!(!png.is_empty());
    }

    #[test]
    fn p1133_rounded_rect_com_cantos_distintos_produz_path() {
        let rect = sk::Rect::from_xywh(0.0, 0.0, 100.0, 80.0).unwrap();
        let path =
            rounded_rect_path(rect, &Corners::new(Pt(2.0), Pt(4.0), Pt(6.0), Pt(8.0)));
        assert_eq!(path.bounds(), rect);
    }
}
