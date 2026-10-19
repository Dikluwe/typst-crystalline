//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/render.md
//! @prompt-hash eac4713e
//! @layer L3
//! @updated 2026-09-28
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
use typst_core::entities::frame_paint::{FramePaint, FrameStroke as Stroke, TilingFrame};

use typst_core::entities::font_book::FontVariant;
use typst_core::entities::font_list::FontList;
use typst_core::entities::font_variations::FontVariations;
use typst_core::entities::geometry::{
    DashLength, FillRule, LineCap, LineJoin, PathItem, ShapeKind,
};
use typst_core::entities::image_format::{detect_image_format, ImageFormat};

use super::pdf_defaults;
use typst_core::entities::layout_types::{
    Color, FrameItem, Page, PagedDocument, Point, Pt, TransformMatrix,
};
use typst_core::entities::shaped_glyph::ShapedGlyph;

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
struct State<'a> {
    transform: sk::Transform,
    local_to_root: TransformMatrix,
    root_size: (f32, f32),
    sample_scale: (f32, f32),
    mask: Option<&'a sk::Mask>,
}

impl State<'_> {
    fn new(pixel_per_pt: f32) -> Self {
        Self {
            transform: sk::Transform::from_scale(pixel_per_pt, pixel_per_pt),
            mask: None,
            local_to_root: TransformMatrix::identity(),
            root_size: (0.0, 0.0),
            sample_scale: (pixel_per_pt, pixel_per_pt),
        }
    }

    fn pre_translate(self, x: f32, y: f32) -> Self {
        Self {
            transform: self.transform.pre_translate(x, y),
            ..self
        }
    }

    fn pre_concat(self, other: sk::Transform) -> Self {
        Self {
            transform: self.transform.pre_concat(other),
            ..self
        }
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
    let prepared = fonts.map(|f| super::font_binding::PreparedFonts::new(page, f));
    let fonts = prepared.as_ref();
    let pixel_per_pt = opts.pixel_per_pt.max(0.01);
    let width = if opts.render_bleed { page.canvas_width() } else { page.width };
    let height = if opts.render_bleed { page.canvas_height() } else { page.height };
    let pxw = (pixel_per_pt * width as f32).round().max(1.0) as u32;
    let pxh = (pixel_per_pt * height as f32).round().max(1.0) as u32;

    let mut canvas = sk::Pixmap::new(pxw, pxh).expect("tamanho validado acima");
    let mut state = State {
        root_size: (width as f32, height as f32),
        ..State::new(pixel_per_pt)
    };
    match &page.fill {
        typst_core::entities::page_canvas::PageFill::Auto => {
            canvas.fill(sk::Color::WHITE)
        }
        typst_core::entities::page_canvas::PageFill::None => {}
        typst_core::entities::page_canvas::PageFill::Paint(paint) => match paint {
            typst_core::entities::paint::Paint::Gradient(g) => render_shape(
                &mut canvas,
                state,
                &Point::ZERO,
                &ShapeKind::Rect,
                width,
                height,
                Some(&FramePaint::Gradient(g.clone())),
                None,
                FillRule::NonZero,
                fonts,
            ),
            typst_core::entities::paint::Paint::Solid(_)
            | typst_core::entities::paint::Paint::Tiling(_) => {
                canvas.fill(color_to_sk(paint.to_color()))
            }
        },
    }

    state.root_size = (page.width as f32, page.height as f32);
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
            let mut rgba = img.to_rgba8();
            for pixel in rgba.pixels_mut() {
                let [r, g, b, a] = pixel.0;
                let color = sk::ColorU8::from_rgba(r, g, b, a).premultiply();
                *pixel = image::Rgba([
                    color.red(),
                    color.green(),
                    color.blue(),
                    color.alpha(),
                ]);
            }
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
    state: State<'_>,
    item: &FrameItem,
    fonts: Option<&super::font_binding::PreparedFonts>,
) {
    match item {
        FrameItem::TextSeparator { .. } | FrameItem::Text { .. } => {}
        FrameItem::TextShaped { pos, glyphs, style, units_per_em, font, .. } => {
            if let Some(fonts) = fonts {
                render_text_shaped(
                    canvas,
                    state,
                    pos,
                    glyphs,
                    style,
                    *units_per_em,
                    fonts,
                    font.as_ref(),
                );
            }
        }
        FrameItem::Line { start, end, thickness, color, cap } => {
            render_line(canvas, state, start, end, *thickness, color.clone(), *cap);
        }
        FrameItem::Glyph { .. } => {}
        FrameItem::Image {
            pos, data, width, height, orientation, clip_rect, ..
        } => {
            let mask = if let Some(clip) = clip_rect {
                let Some(rect) = sk::Rect::from_xywh(
                    clip.x.0 as f32,
                    clip.y.0 as f32,
                    clip.w.0 as f32,
                    clip.h.0 as f32,
                ) else {
                    return;
                };
                let path = sk::PathBuilder::from_rect(rect);
                let Some(mask) = intersect_clip(canvas, state, &path, state.transform)
                else {
                    return;
                };
                Some(mask)
            } else {
                None
            };
            let state = State { mask: mask.as_ref().or(state.mask), ..state };
            render_image(canvas, state, pos, data, *width, *height, *orientation);
        }
        FrameItem::Shape {
            pos, kind, width, height, fill, stroke, fill_rule, ..
        } => {
            render_shape(
                canvas,
                state,
                pos,
                kind,
                *width,
                *height,
                fill.as_ref(),
                stroke.as_ref(),
                *fill_rule,
                fonts,
            );
        }
        FrameItem::Group {
            paint_root,
            pos,
            matrix,
            clip_mask,
            inner_width,
            inner_height,
            items,
            ..
        } => {
            let state = State {
                root_size: if *paint_root {
                    (*inner_width as f32, *inner_height as f32)
                } else {
                    state.root_size
                },
                local_to_root: if *paint_root {
                    TransformMatrix::identity()
                } else {
                    state
                        .local_to_root
                        .concat(&TransformMatrix::translate(pos.x.0, pos.y.0))
                        .concat(matrix)
                },
                ..state
            };
            render_group(
                canvas,
                state,
                pos,
                matrix,
                clip_mask.as_ref().map(|clip| (clip, *inner_width, *inner_height)),
                items,
                fonts,
            );
        }
        FrameItem::Link { items, .. } => {
            for item in items {
                render_item(canvas, state, item, fonts);
            }
        }
        FrameItem::TextReplacement { items, .. } | FrameItem::Semantic { items, .. } => {
            for item in items {
                render_item(canvas, state, item, fonts);
            }
        }
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "C047: Leaf raster usa oito inputs efetivos de pintura; texto autoral não é transportado para PNG"
)]
fn render_text_shaped(
    canvas: &mut sk::Pixmap,
    state: State<'_>,
    pos: &Point,
    glyphs: &[ShapedGlyph],
    style: &typst_core::entities::layout_types::TextStyle,
    units_per_em: u16,
    fonts: &super::font_binding::PreparedFonts,
    font: Option<&typst_core::entities::world_types::Font>,
) {
    let Some((_, font_bytes)) = fonts.get(style, font) else { return };
    let face = match ttf_parser::Face::parse(font_bytes, 0) {
        Ok(f) => f,
        Err(_) => return,
    };

    let upem = units_per_em.max(1) as f64;
    let size = style.size.0;
    let scale = size / upem;
    let fill = style.fill.clone().unwrap_or(Color::rgb(0, 0, 0));
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

#[expect(
    clippy::too_many_arguments,
    reason = "C048: Leaf raster recebe alvo, estado gráfico, face/GID, posição, tamanho e cor resolvidos"
)]
fn render_glyph(
    canvas: &mut sk::Pixmap,
    state: State<'_>,
    face: &ttf_parser::Face<'_>,
    id: GlyphId,
    x: f32,
    y: f32,
    size: f32,
    color: sk::Color,
) {
    let ts = state.transform;
    let ppem = size * ts.sy;
    if ts.kx != 0.0 || ts.ky != 0.0 || ts.sx != ts.sy || !(0.0..=100.0).contains(&ppem) {
        struct Outline(sk::PathBuilder);
        impl ttf_parser::OutlineBuilder for Outline {
            fn move_to(&mut self, x: f32, y: f32) {
                self.0.close();
                self.0.move_to(x, y);
            }
            fn line_to(&mut self, x: f32, y: f32) {
                self.0.line_to(x, y);
            }
            fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
                self.0.quad_to(x1, y1, x, y);
            }
            fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
                self.0.cubic_to(x1, y1, x2, y2, x, y);
            }
            fn close(&mut self) {
                self.0.close();
            }
        }
        let mut outline = Outline(sk::PathBuilder::new());
        if face.outline_glyph(id, &mut outline).is_none() {
            return;
        }
        // CFF2 outlines can omit the final close command.
        outline.0.close();
        let Some(path) = outline.0.finish() else {
            return;
        };
        let scale = size / f32::from(face.units_per_em());
        let transform = ts.pre_translate(x, y).pre_scale(scale, -scale);
        let mut paint = sk::Paint::default();
        paint.set_color(color);
        canvas.fill_path(&path, &paint, sk::FillRule::Winding, transform, state.mask);
        return;
    }
    let Some(glyph) = pixglyph::Glyph::load(face, id) else {
        return;
    };

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
            let pi = (py * cw + px) as usize;
            let cov = bitmap.coverage[ai];
            let cov = state.mask.map_or(cov, |mask| {
                ((u16::from(cov) * u16::from(mask.data()[pi]) + 127) / 255) as u8
            });
            if cov == 0 {
                continue;
            }

            let applied = alpha_mul(color_u32, cov as u32);
            pixels[pi] = blend_src_over(applied, pixels[pi]);
        }
    }
}

fn render_line(
    canvas: &mut sk::Pixmap,
    state: State<'_>,
    start: &Point,
    end: &Point,
    thickness: f64,
    color: Option<Color>,
    cap: LineCap,
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
        line_cap: match cap {
            LineCap::Butt => sk::LineCap::Butt,
            LineCap::Round => sk::LineCap::Round,
            LineCap::Square => sk::LineCap::Square,
        },
        ..Default::default()
    };
    canvas.stroke_path(&path, &paint, &stroke, state.transform, state.mask);
}

fn render_image(
    canvas: &mut sk::Pixmap,
    state: State<'_>,
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
    canvas.fill_rect(rect, &paint, ts, state.mask);
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

fn sk_transform(m: TransformMatrix) -> sk::Transform {
    sk::Transform::from_row(
        m.a as f32,
        m.b as f32,
        m.c as f32,
        m.d as f32,
        m.tx as f32,
        m.ty as f32,
    )
}

fn frame_paint<'a>(
    paint: &FramePaint,
    texture: Option<&'a (sk::Pixmap, sk::Transform)>,
) -> Option<sk::Paint<'a>> {
    let shader = match paint {
        FramePaint::Solid(color) => sk::Shader::SolidColor(color_to_sk(color.clone())),
        FramePaint::Gradient(_) => {
            let (pixmap, matrix) = texture?;
            sk::Pattern::new(
                pixmap.as_ref(),
                sk::SpreadMode::Pad,
                sk::FilterQuality::Nearest,
                1.0,
                *matrix,
            )
        }
        FramePaint::Tiling(_) => {
            let (pixmap, matrix) = texture?;
            sk::Pattern::new(
                pixmap.as_ref(),
                sk::SpreadMode::Repeat,
                sk::FilterQuality::Bilinear,
                1.0,
                *matrix,
            )
        }
    };
    let anti_alias = match paint {
        FramePaint::Gradient(g) => match g {
            typst_core::entities::gradient::Gradient::Linear(g) => g.anti_alias,
            typst_core::entities::gradient::Gradient::Radial(g) => g.anti_alias,
            typst_core::entities::gradient::Gradient::Conic(g) => g.anti_alias,
        },
        FramePaint::Solid(_) | FramePaint::Tiling(_) => true,
    };
    Some(sk::Paint { shader, anti_alias, ..Default::default() })
}

fn prepare_texture(
    paint: &FramePaint,
    state: State<'_>,
    origin: Point,
    geometry: Option<PaintGeometry>,
    fonts: Option<&super::font_binding::PreparedFonts>,
) -> Option<(sk::Pixmap, sk::Transform)> {
    let cell = match paint {
        FramePaint::Solid(_) => return None,
        FramePaint::Gradient(g) => return gradient_texture(g, state, origin, geometry?),
        FramePaint::Tiling(cell) => cell,
    };
    let placement = super::tiling::placement(cell, origin, state.local_to_root)?;
    let device = state.transform.pre_concat(sk_transform(placement));
    let pitch_x = cell.frame.size.width.0 + cell.spacing.width.0;
    let pitch_y = cell.frame.size.height.0 + cell.spacing.height.0;
    let scale_x = f64::from(libm::hypotf(device.sx, device.ky));
    let scale_y = f64::from(libm::hypotf(device.kx, device.sy));
    let px = (pitch_x * scale_x).ceil().max(1.0) as u32;
    let py = (pitch_y * scale_y).ceil().max(1.0) as u32;
    // Supersampling resolves coverage even when an entire period is subpixel.
    let high_x = if pitch_x * scale_x < 1.0 { 16 } else { px };
    let high_y = if pitch_y * scale_y < 1.0 { 16 } else { py };
    let period = raster_period(cell, high_x, high_y, fonts)?;
    let texture = if (px, py) == (high_x, high_y) {
        period
    } else {
        let mut reduced_texture = sk::Pixmap::new(px, py)?;
        let nx = high_x / px;
        let ny = high_y / py;
        for y in 0..py {
            for x in 0..px {
                let mut sums = [0u32; 4];
                for dy in 0..ny {
                    for dx in 0..nx {
                        let at = (((y * ny + dy) * high_x + x * nx + dx) * 4) as usize;
                        for (sum, byte) in sums.iter_mut().zip(&period.data()[at..at + 4])
                        {
                            *sum += u32::from(*byte);
                        }
                    }
                }
                let at = ((y * px + x) * 4) as usize;
                for (byte, sum) in
                    reduced_texture.data_mut()[at..at + 4].iter_mut().zip(sums)
                {
                    *byte = ((sum + nx * ny / 2) / (nx * ny)) as u8;
                }
            }
        }
        reduced_texture
    };
    let matrix = TransformMatrix::translate(-origin.x.0, -origin.y.0)
        .concat(&placement)
        .concat(&TransformMatrix::scale(
            pitch_x / f64::from(px),
            pitch_y / f64::from(py),
        ));
    Some((texture, sk_transform(matrix)))
}

#[derive(Clone, Copy)]
struct PaintGeometry {
    bounds: sk::Rect,
    mirror: (bool, bool),
}

fn gradient_texture(
    gradient: &typst_core::entities::gradient::Gradient,
    state: State<'_>,
    origin: Point,
    geometry: PaintGeometry,
) -> Option<(sk::Pixmap, sk::Transform)> {
    use typst_core::entities::gradient::{Gradient, RelativeTo};
    let relative = match gradient {
        Gradient::Linear(g) => g.relative,
        Gradient::Radial(g) => g.relative,
        Gradient::Conic(g) => g.relative,
    }
    .unwrap_or_default();
    let (size, base, mirror) = match relative {
        RelativeTo::Self_ => (
            (geometry.bounds.width(), geometry.bounds.height()),
            sk::Transform::from_translate(geometry.bounds.left(), geometry.bounds.top()),
            geometry.mirror,
        ),
        RelativeTo::Parent => (
            state.root_size,
            sk_transform(
                TransformMatrix::translate(-origin.x.0, -origin.y.0)
                    .concat(&super::tiling::inverse(state.local_to_root)?),
            ),
            (false, false),
        ),
    };
    let (sx, sy) = state.sample_scale;
    let width = (size.0.abs() * sx).ceil().max(sx.ceil()).max(1.0) as u32;
    let height = (size.1.abs() * sy).ceil().max(sy.ceil()).max(1.0) as u32;
    let mut texture = sk::Pixmap::new(width, height)?;
    for (i, pixel) in texture.pixels_mut().iter_mut().enumerate() {
        let x = (i % width as usize) as f32;
        let y = (i / width as usize) as f32;
        let x = if mirror.0 { width as f32 - x } else { x };
        let y = if mirror.1 { height as f32 - y } else { y };
        let color = gradient_sample(
            gradient,
            x / width as f32,
            y / height as f32,
            (width as f32 / sx) / (height as f32 / sy),
        );
        let (r, g, b, a) = color.to_rgba_f32();
        *pixel = sk::Color::from_rgba(
            r.clamp(0.0, 1.0),
            g.clamp(0.0, 1.0),
            b.clamp(0.0, 1.0),
            a.clamp(0.0, 1.0),
        )?
        .premultiply()
        .to_color_u8();
    }
    Some((texture, base.pre_scale(1.0 / sx, 1.0 / sy)))
}

fn gradient_sample(
    gradient: &typst_core::entities::gradient::Gradient,
    x: f32,
    y: f32,
    aspect: f32,
) -> Color {
    use typst_core::entities::gradient::Gradient;
    let (x, y, aspect) = (f64::from(x), f64::from(y), f64::from(aspect));
    match gradient {
        Gradient::Linear(g) => {
            let angle = libm::atan2(
                libm::sin(g.angle.to_rad()) / aspect.abs(),
                libm::cos(g.angle.to_rad()),
            );
            let (sin, cos) = (libm::sin(angle), libm::cos(angle));
            let x = if cos < 0.0 { 1.0 - x } else { x };
            let y = if sin < 0.0 { 1.0 - y } else { y };
            g.sample(((x * cos.abs() + y * sin.abs()) / (cos.abs() + sin.abs())) as f32)
        }
        Gradient::Radial(g) => {
            let (dx, dy) = (x - g.focal_center.x.0, y - g.focal_center.y.0);
            let distance = libm::hypot(dx, dy);
            let t = if distance <= g.focal_radius.0 || distance == 0.0 {
                0.0
            } else if libm::hypot(x - g.center.x.0, y - g.center.y.0) > g.radius.0 {
                1.0
            } else {
                let (qx, qy) = (
                    g.focal_center.x.0 - g.center.x.0,
                    g.focal_center.y.0 - g.center.y.0,
                );
                let a = (qx * dx + qy * dy) / distance;
                let rho = g.radius.0 * g.radius.0 - qx * qx - qy * qy;
                let boundary = libm::sqrt((a * a + rho).max(0.0)) - a;
                (distance - g.focal_radius.0) / (boundary - g.focal_radius.0)
            };
            g.sample(t.clamp(0.0, 1.0) as f32)
        }
        Gradient::Conic(g) => {
            let angle = libm::atan2((y - g.center.y.0) / aspect.abs(), x - g.center.x.0);
            let t = (std::f64::consts::PI + angle - g.angle.to_rad())
                .rem_euclid(std::f64::consts::TAU)
                / std::f64::consts::TAU;
            g.sample(t as f32)
        }
    }
}

fn raster_period(
    cell: &TilingFrame,
    px: u32,
    py: u32,
    fonts: Option<&super::font_binding::PreparedFonts>,
) -> Option<sk::Pixmap> {
    let width = cell.frame.size.width.0;
    let height = cell.frame.size.height.0;
    let pitch_x = width + cell.spacing.width.0;
    let pitch_y = height + cell.spacing.height.0;
    let sx = f64::from(px) / pitch_x;
    let sy = f64::from(py) / pitch_y;
    let cw = (width * sx).ceil().max(1.0) as u32;
    let ch = (height * sy).ceil().max(1.0) as u32;
    let mut body = sk::Pixmap::new(cw, ch)?;
    let transform = sk::Transform::from_scale(sx as f32, sy as f32);
    let mut mask = sk::Mask::new(cw, ch)?;
    let rect = sk::Rect::from_xywh(0.0, 0.0, width as f32, height as f32)?;
    mask.fill_path(
        &sk::PathBuilder::from_rect(rect),
        sk::FillRule::Winding,
        true,
        transform,
    );
    let state = State {
        transform,
        local_to_root: TransformMatrix::identity(),
        root_size: (width as f32, height as f32),
        sample_scale: (sx as f32, sy as f32),
        mask: Some(&mask),
    };
    for item in &cell.frame.items {
        render_item(&mut body, state, item, fonts);
    }
    let mut period = sk::Pixmap::new(px, py)?;
    // Only overlap contributors to one period; never enumerate the target area.
    let nx = (width / pitch_x).ceil() as i64;
    let ny = (height / pitch_y).ceil() as i64;
    for y in (1 - ny)..=0 {
        for x in (1 - nx)..=0 {
            let tx = x.checked_mul(i64::from(px))?;
            let ty = y.checked_mul(i64::from(py))?;
            period.draw_pixmap(
                i32::try_from(tx).ok()?,
                i32::try_from(ty).ok()?,
                body.as_ref(),
                &sk::PixmapPaint::default(),
                sk::Transform::identity(),
                None,
            );
        }
    }
    Some(period)
}

#[expect(
    clippy::too_many_arguments,
    reason = "C049: Leaf de Shape consome geometria resolvida, fill/stroke/fill-rule, estado e fontes para textura"
)]
fn render_shape(
    canvas: &mut sk::Pixmap,
    state: State<'_>,
    pos: &Point,
    kind: &ShapeKind<typst_core::entities::layout_types::Pt>,
    width: f64,
    height: f64,
    fill: Option<&FramePaint>,
    stroke: Option<&Stroke>,
    fill_rule: FillRule,
    fonts: Option<&super::font_binding::PreparedFonts>,
) {
    let ts = state.transform.pre_translate(pos.x.0 as f32, pos.y.0 as f32);

    let path = shape_to_path(kind, width, height, stroke, true);
    let Some(path) = path else { return };

    let mirror = (
        matches!(kind, ShapeKind::Rect) && width < 0.0,
        matches!(kind, ShapeKind::Rect) && height < 0.0,
    );
    if let Some(fill) = fill {
        let geometry = if matches!(fill, FramePaint::Gradient(_)) {
            path.compute_tight_bounds()
                .map(|bounds| PaintGeometry { bounds, mirror })
        } else {
            None
        };
        let texture = prepare_texture(fill, state, *pos, geometry, fonts);
        if let Some(mut paint) = frame_paint(fill, texture.as_ref()) {
            if matches!(kind, ShapeKind::Rect) && matches!(fill, FramePaint::Gradient(_))
            {
                paint.anti_alias = false;
            }
            canvas.fill_path(
                &path,
                &paint,
                match fill_rule {
                    FillRule::NonZero => sk::FillRule::Winding,
                    FillRule::EvenOdd => sk::FillRule::EvenOdd,
                },
                ts,
                state.mask,
            );
        }
    }

    if let Some(stroke) = stroke {
        let rounded_path = if matches!(kind, ShapeKind::RoundedRect { .. }) {
            shape_to_path(kind, width, height, Some(stroke), false)
        } else {
            None
        };
        let path = rounded_path.as_ref().unwrap_or(&path);
        let sk_stroke = raster_stroke(stroke);
        let bounds = if !matches!(stroke.paint, FramePaint::Gradient(_)) {
            None
        } else if matches!(kind, ShapeKind::Rect) {
            let half = stroke.thickness as f32 / 2.0;
            sk::Rect::from_xywh(
                (width as f32).min(0.0) - half,
                (height as f32).min(0.0) - half,
                width.abs() as f32 + 2.0 * half,
                height.abs() as f32 + 2.0 * half,
            )
        } else {
            let dashed = sk_stroke.dash.as_ref().and_then(|dash| path.dash(dash, 1.0));
            dashed
                .as_ref()
                .unwrap_or(path)
                .stroke(&sk_stroke, 1.0)
                .and_then(|p| p.compute_tight_bounds())
        };
        let texture = prepare_texture(
            &stroke.paint,
            state,
            *pos,
            bounds.map(|bounds| PaintGeometry { bounds, mirror }),
            fonts,
        );
        let Some(paint) = frame_paint(&stroke.paint, texture.as_ref()) else {
            return;
        };
        canvas.stroke_path(path, &paint, &sk_stroke, ts, state.mask);
    }
}

fn raster_stroke(stroke: &Stroke) -> sk::Stroke {
    sk::Stroke {
        width: stroke.thickness as f32,
        line_cap: match stroke.cap {
            LineCap::Butt => sk::LineCap::Butt,
            LineCap::Round => sk::LineCap::Round,
            LineCap::Square => sk::LineCap::Square,
        },
        line_join: match stroke.join {
            LineJoin::Miter => sk::LineJoin::Miter,
            LineJoin::Round => sk::LineJoin::Round,
            LineJoin::Bevel => sk::LineJoin::Bevel,
        },
        miter_limit: stroke.miter_limit as f32,
        dash: stroke.dash.as_ref().and_then(|dash| {
            let len = dash.array.len();
            let count = if len % 2 == 0 { len } else { len * 2 };
            let array = dash
                .array
                .iter()
                .cycle()
                .take(count)
                .map(|length| {
                    (match length {
                        DashLength::Length(length) => *length,
                        DashLength::LineWidth => stroke.thickness,
                    }) as f32
                })
                .collect();
            sk::StrokeDash::new(array, dash.phase as f32)
        }),
    }
}

fn shape_to_path(
    kind: &ShapeKind<typst_core::entities::layout_types::Pt>,
    width: f64,
    height: f64,
    stroke: Option<&Stroke>,
    closed: bool,
) -> Option<sk::Path> {
    match kind {
        ShapeKind::Rect => {
            let rect = sk::Rect::from_xywh(
                width.min(0.0) as f32,
                height.min(0.0) as f32,
                width.abs() as f32,
                height.abs() as f32,
            )?;
            Some(sk::PathBuilder::from_rect(rect))
        }
        ShapeKind::RoundedRect { radii } => {
            let rect = sk::Rect::from_xywh(0.0, 0.0, width as f32, height as f32)?;
            Some(rounded_rect_path(
                rect,
                radii,
                stroke.map_or(0.0, |s| s.thickness / 2.0),
                closed,
            ))
        }
        ShapeKind::Ellipse => {
            sk::Rect::from_xywh(0.0, 0.0, width as f32, height as f32)?;
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
    inset: f64,
    closed: bool,
) -> sk::Path {
    let mut pb = sk::PathBuilder::new();
    let x = rect.left();
    let y = rect.top();
    let w = rect.width();
    let h = rect.height();
    let k = pdf_defaults::BEZIER_CIRCLE_KAPPA as f32;
    let max_r = w.min(h) / 2.0;
    let tl = ((radii.top_left.0 - inset) as f32).clamp(0.0, max_r);
    let tr = ((radii.top_right.0 - inset) as f32).clamp(0.0, max_r);
    let br = ((radii.bottom_right.0 - inset) as f32).clamp(0.0, max_r);
    let bl = ((radii.bottom_left.0 - inset) as f32).clamp(0.0, max_r);

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
    if closed {
        pb.close();
    }
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
    state: State<'_>,
    pos: &Point,
    matrix: &TransformMatrix,
    clip_mask: Option<(&ShapeKind<Pt>, f64, f64)>,
    items: &[FrameItem],
    fonts: Option<&super::font_binding::PreparedFonts>,
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
    let mask = if let Some((kind, width, height)) = clip_mask {
        let Some(path) = shape_to_path(kind, width, height, None, true) else { return };
        let Some(mask) = intersect_clip(canvas, new_state, &path, new_state.transform)
        else {
            return;
        };
        Some(mask)
    } else {
        None
    };
    let new_state = State {
        mask: mask.as_ref().or(new_state.mask),
        ..new_state
    };
    for item in items {
        render_item(canvas, new_state, item, fonts);
    }
}

fn intersect_clip(
    canvas: &sk::Pixmap,
    state: State<'_>,
    path: &sk::Path,
    transform: sk::Transform,
) -> Option<sk::Mask> {
    if let Some(parent) = state.mask {
        let mut mask = parent.clone();
        mask.intersect_path(path, sk::FillRule::Winding, true, transform);
        Some(mask)
    } else {
        let mut mask = sk::Mask::new(canvas.width(), canvas.height())?;
        mask.fill_path(path, sk::FillRule::Winding, true, transform);
        Some(mask)
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
    let data = canvas
        .pixels()
        .iter()
        .flat_map(|pixel| {
            let pixel = pixel.demultiply();
            [pixel.red(), pixel.green(), pixel.blue(), pixel.alpha()]
        })
        .collect();
    let rgba = image::RgbaImage::from_raw(width, height, data)
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

    fn point(x: f64, y: f64) -> Point {
        Point { x: Pt(x), y: Pt(y) }
    }

    fn gradient_page(
        items: Vec<FrameItem>,
        fill: Option<FramePaint>,
    ) -> image::RgbaImage {
        let page = Page {
            width: 64.0,
            height: 64.0,
            numbering: None,
            logical_number: 1,
            supplement: typst_core::entities::content::Content::Empty,
            bleed: Default::default(),
            fill: match fill {
                Some(FramePaint::Gradient(g)) => {
                    typst_core::entities::page_canvas::PageFill::Paint(
                        typst_core::entities::paint::Paint::Gradient(g),
                    )
                }
                None => typst_core::entities::page_canvas::PageFill::None,
                Some(FramePaint::Solid(_) | FramePaint::Tiling(_)) => {
                    panic!("gradient fixture required")
                }
            },
            background: vec![],
            foreground: vec![],
            items,
        };
        image::load_from_memory(&render_page_to_png(
            &page,
            &RenderOptions { pixel_per_pt: 1.0, render_bleed: false },
        ))
        .unwrap()
        .to_rgba8()
    }

    fn gradient_fill() -> FramePaint {
        use typst_core::entities::{
            color::ColorSpace,
            gradient::{Gradient, GradientStop},
            layout_types::{Angle, Ratio},
        };
        FramePaint::Gradient(Gradient::linear_with_space(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            Angle::deg(0.0),
            ColorSpace::Srgb,
        ))
    }

    fn gradient_rect(fill: FramePaint) -> FrameItem {
        FrameItem::Shape {
            pos: point(8.0, 8.0),
            kind: ShapeKind::Rect,
            width: 40.0,
            height: 40.0,
            fill: Some(fill),
            stroke: None,
            fill_rule: FillRule::NonZero,
            parent_bbox_at_emit: None,
        }
    }

    fn close_rgba(actual: image::Rgba<u8>, expected: [u8; 4]) {
        for (a, b) in actual.0.into_iter().zip(expected) {
            assert!(a.abs_diff(b) <= 2, "{actual:?} != {expected:?}");
        }
    }

    #[test]
    fn png_gradients_linear_and_page_preserve_position() {
        let image = gradient_page(vec![gradient_rect(gradient_fill())], None);
        close_rgba(*image.get_pixel(8, 20), [255, 0, 0, 255]);
        close_rgba(*image.get_pixel(28, 20), [128, 0, 128, 255]);
        close_rgba(*image.get_pixel(47, 20), [6, 0, 249, 255]);
        let page = gradient_page(vec![], Some(gradient_fill()));
        close_rgba(*page.get_pixel(32, 20), [128, 0, 128, 255]);
    }

    #[test]
    fn png_gradients_radial_and_conic_sample_geometry() {
        use typst_core::entities::{
            axes::Axes,
            color::ColorSpace,
            gradient::{Gradient, GradientStop},
            layout_types::{Angle, Ratio},
        };
        let stops = vec![
            GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
            GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
        ];
        let radial = Gradient::radial_with_space(
            stops.clone(),
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
            ColorSpace::Srgb,
        );
        let image =
            gradient_page(vec![gradient_rect(FramePaint::Gradient(radial))], None);
        close_rgba(*image.get_pixel(28, 28), [255, 0, 0, 255]);
        close_rgba(*image.get_pixel(38, 28), [128, 0, 128, 255]);
        close_rgba(*image.get_pixel(8, 28), [0, 0, 255, 255]);
        let conic = Gradient::conic_with_space(
            stops,
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Angle::deg(0.0),
            ColorSpace::Srgb,
        );
        let image = gradient_page(vec![gradient_rect(FramePaint::Gradient(conic))], None);
        for (x, y, c) in [
            (8, 28, [255, 0, 0, 255]),
            (38, 28, [128, 0, 128, 255]),
            (28, 18, [191, 0, 64, 255]),
            (28, 38, [64, 0, 191, 255]),
        ] {
            close_rgba(*image.get_pixel(x, y), c);
        }
    }

    #[test]
    fn png_gradients_parent_uses_root_size_and_group_origin() {
        let FramePaint::Gradient(typst_core::entities::gradient::Gradient::Linear(mut g)) =
            gradient_fill()
        else {
            unreachable!()
        };
        Arc::make_mut(&mut g).relative =
            Some(typst_core::entities::gradient::RelativeTo::Parent);
        let item = gradient_rect(FramePaint::Gradient(
            typst_core::entities::gradient::Gradient::Linear(g),
        ));
        let image = gradient_page(vec![item.clone()], None);
        close_rgba(*image.get_pixel(32, 20), [128, 0, 128, 255]);
        let parent = FrameItem::Group {
            paint_root: true,
            pos: point(4.0, 4.0),
            matrix: TransformMatrix::identity(),
            clip_mask: None,
            inner_width: 40.0,
            inner_height: 40.0,
            items: vec![item],
        };
        let image = gradient_page(vec![parent], None);
        close_rgba(*image.get_pixel(24, 24), [128, 0, 128, 255]);
    }

    #[test]
    fn png_gradients_alpha_and_stroke_are_not_solid() {
        use typst_core::entities::{
            color::ColorSpace,
            gradient::{Gradient, GradientStop},
            layout_types::{Angle, Ratio},
        };
        let g = Gradient::linear_with_space(
            vec![
                GradientStop::new(
                    Color::Srgb { r: 1.0, g: 0.0, b: 0.0, a: 0.2 },
                    Ratio(0.0),
                ),
                GradientStop::new(
                    Color::Srgb { r: 0.0, g: 0.0, b: 1.0, a: 0.8 },
                    Ratio(1.0),
                ),
            ],
            Angle::deg(0.0),
            ColorSpace::Srgb,
        );
        let image = gradient_page(vec![gradient_rect(FramePaint::Gradient(g))], None);
        close_rgba(*image.get_pixel(28, 20), [128, 0, 128, 128]);
        let mut item = gradient_rect(gradient_fill());
        if let FrameItem::Shape { fill, stroke, .. } = &mut item {
            *fill = None;
            *stroke = Some(Stroke {
                paint: gradient_fill(),
                thickness: 6.0,
                cap: LineCap::Butt,
                join: LineJoin::Miter,
                miter_limit: 4.0,
                dash: None,
                overhang: false,
                specified: Default::default(),
            });
        }
        let image = gradient_page(vec![item], None);
        close_rgba(*image.get_pixel(28, 7), [128, 0, 128, 255]);
        assert_eq!(image.get_pixel(28, 28).0[3], 0);
    }

    #[test]
    fn png_gradients_negative_rect_mirrors_self_direction() {
        let mut item = gradient_rect(gradient_fill());
        if let FrameItem::Shape { pos, width, .. } = &mut item {
            *pos = point(48.0, 8.0);
            *width = -40.0;
        }
        let image = gradient_page(vec![item], None);
        close_rgba(*image.get_pixel(28, 20), [128, 0, 128, 255]);
        close_rgba(*image.get_pixel(8, 20), [0, 0, 255, 255]);
    }

    #[test]
    fn png_rounded_stroke_keeps_declared_outer_radius() {
        let mut item = gradient_rect(FramePaint::Solid(Color::rgb(255, 0, 0)));
        if let FrameItem::Shape { kind, fill, stroke, .. } = &mut item {
            *kind = ShapeKind::RoundedRect {
                radii: Corners::new(Pt(8.0), Pt(8.0), Pt(8.0), Pt(8.0)),
            };
            *fill = None;
            *stroke = Some(Stroke {
                paint: FramePaint::Solid(Color::rgb(255, 0, 0)),
                thickness: 6.0,
                ..Default::default()
            });
        }
        let item = FrameItem::Group {
            paint_root: false,
            pos: Point::ZERO,
            matrix: TransformMatrix::scale(2.0, 2.0),
            clip_mask: None,
            inner_width: 40.0,
            inner_height: 40.0,
            items: vec![item],
        };
        let image = gradient_page(vec![item], None);
        assert!(
            image.get_pixel(21, 11).0[3] > 235,
            "declared outer corner must cover this pixel"
        );
        assert_eq!(image.get_pixel(10, 10).0[3], 0, "outside outer radius");
    }

    #[test]
    fn png_gradients_parent_survives_affine_groups_and_siblings() {
        use typst_core::entities::gradient::{Gradient, RelativeTo};
        let FramePaint::Gradient(Gradient::Linear(mut g)) = gradient_fill() else {
            unreachable!()
        };
        Arc::make_mut(&mut g).relative = Some(RelativeTo::Parent);
        let mut item = gradient_rect(FramePaint::Gradient(Gradient::Linear(g)));
        if let FrameItem::Shape { pos, width, height, .. } = &mut item {
            *pos = Point::ZERO;
            *width = 20.0;
            *height = 20.0;
        }
        let group = FrameItem::Group {
            paint_root: false,
            pos: point(8.0, 8.0),
            matrix: TransformMatrix { a: 1.5, b: 0.0, c: 0.5, d: 1.0, tx: 0.0, ty: 0.0 },
            clip_mask: None,
            inner_width: 20.0,
            inner_height: 20.0,
            items: vec![item.clone()],
        };
        let image = gradient_page(vec![group, item], None);
        close_rgba(*image.get_pixel(32, 20), [128, 0, 128, 255]);
        close_rgba(*image.get_pixel(8, 4), [223, 0, 32, 255]);
    }

    #[test]
    fn png_gradients_tiling_root_and_anisotropic_sampling() {
        use typst_core::entities::{
            gradient::{Gradient, RelativeTo},
            layout_types::{Angle, Frame, Size},
        };
        let FramePaint::Gradient(Gradient::Linear(mut g)) = gradient_fill() else {
            unreachable!()
        };
        Arc::make_mut(&mut g).relative = Some(RelativeTo::Parent);
        Arc::make_mut(&mut g).angle = Angle::deg(45.0);
        let mut item = gradient_rect(FramePaint::Gradient(Gradient::Linear(g)));
        if let FrameItem::Shape { pos, width, height, .. } = &mut item {
            *pos = Point::ZERO;
            *width = 8.0;
            *height = 8.0;
        }
        let mut frame = Frame::new(Size { width: Pt(8.0), height: Pt(8.0) });
        frame.items.push(item);
        let cell = Arc::new(TilingFrame {
            frame,
            spacing: Size { width: Pt(0.0), height: Pt(0.0) },
            offset: Point::ZERO,
            angle: Angle::deg(0.0),
            relative: RelativeTo::Self_,
        });
        let mut tiled = gradient_rect(FramePaint::Tiling(cell));
        if let FrameItem::Shape { pos, width, height, .. } = &mut tiled {
            *pos = Point::ZERO;
            *width = 16.0;
            *height = 16.0;
        }
        let group = FrameItem::Group {
            paint_root: false,
            pos: Point::ZERO,
            matrix: TransformMatrix::scale(2.0, 1.0),
            clip_mask: None,
            inner_width: 16.0,
            inner_height: 16.0,
            items: vec![tiled],
        };
        let image = gradient_page(vec![group], None);
        // Local (2,6) in each 8pt square: 45° gradient is halfway, even
        // when the texture has twice as many horizontal device pixels.
        close_rgba(*image.get_pixel(4, 6), [128, 0, 128, 255]);
        close_rgba(*image.get_pixel(20, 6), [128, 0, 128, 255]);
    }

    #[test]
    fn empty_page_produces_png() {
        let page = Page {
            width: 50.0,
            height: 50.0,
            numbering: None,
            logical_number: 1,
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
            logical_number: 1,
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
                fill: Some(typst_core::entities::frame_paint::FramePaint::Solid(
                    Color::rgb(255, 0, 0),
                )),
                stroke: None,
                fill_rule: Default::default(),
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
        let path = rounded_rect_path(
            rect,
            &Corners::new(Pt(2.0), Pt(4.0), Pt(6.0), Pt(8.0)),
            0.0,
            true,
        );
        assert_eq!(path.bounds(), rect);
    }
}

#[cfg(test)]
mod mask_regressions {
    use super::*;
    use typst_core::entities::frame_paint::FramePaint as Paint;
    use typst_core::entities::geometry::{DashLength, DashPattern, FillRule, LineJoin};
    use typst_core::entities::layout_types::{Rect, TextStyle};

    fn point(x: f64, y: f64) -> Point {
        Point { x: Pt(x), y: Pt(y) }
    }
    fn shape(
        kind: ShapeKind<Pt>,
        fill_rule: FillRule,
        stroke: Option<Stroke>,
    ) -> FrameItem {
        FrameItem::Shape {
            pos: Point::ZERO,
            kind,
            width: 64.0,
            height: 64.0,
            fill: stroke.is_none().then(|| Paint::Solid(Color::rgb(255, 0, 0))),
            stroke,
            fill_rule,
            parent_bbox_at_emit: None,
        }
    }
    fn group(
        pos: Point,
        matrix: TransformMatrix,
        clip: Option<ShapeKind<Pt>>,
        size: (f64, f64),
        items: Vec<FrameItem>,
    ) -> FrameItem {
        FrameItem::Group {
            paint_root: false,
            pos,
            matrix,
            clip_mask: clip,
            inner_width: size.0,
            inner_height: size.1,
            items,
        }
    }
    fn canvas(items: &[FrameItem]) -> sk::Pixmap {
        let mut canvas = sk::Pixmap::new(64, 64).unwrap();
        for item in items {
            render_item(&mut canvas, State::new(1.0), item, None);
        }
        canvas
    }
    fn alpha(canvas: &sk::Pixmap, x: u32, y: u32) -> u8 {
        canvas.pixel(x, y).unwrap().alpha()
    }
    #[test]
    fn raster_masks_intersect_in_device_space_and_do_not_leak_to_siblings() {
        let red = shape(ShapeKind::Rect, FillRule::NonZero, None);
        let inner = group(
            point(10.0, 5.0),
            TransformMatrix::identity(),
            Some(ShapeKind::Rect),
            (20.0, 20.0),
            vec![red.clone()],
        );
        let outer = group(
            point(8.0, 6.0),
            TransformMatrix::identity(),
            Some(ShapeKind::Rect),
            (20.0, 20.0),
            vec![inner],
        );
        let sibling = group(
            point(40.0, 40.0),
            TransformMatrix::identity(),
            None,
            (1.0, 1.0),
            vec![red],
        );
        let c = canvas(&[outer, sibling]);
        assert_eq!(alpha(&c, 20, 15), 255);
        for (x, y) in [(30, 15), (20, 28), (10, 10)] {
            assert_eq!(alpha(&c, x, y), 0, "({x},{y}) outside intersection");
        }
        assert_eq!(alpha(&c, 45, 45), 255, "sibling must not inherit local clip");
        let scaled = canvas(&[group(
            point(10.0, 5.0),
            TransformMatrix::scale(2.0, 1.0),
            Some(ShapeKind::Rect),
            (10.0, 10.0),
            vec![shape(ShapeKind::Rect, FillRule::NonZero, None)],
        )]);
        assert_eq!(alpha(&scaled, 12, 6), 255);
        assert_eq!(alpha(&scaled, 35, 6), 0);
    }
    #[test]
    fn raster_masks_use_curved_paths_and_empty_clips_paint_nothing() {
        let red = shape(ShapeKind::Rect, FillRule::NonZero, None);
        let c = canvas(&[group(
            point(10.0, 10.0),
            TransformMatrix::identity(),
            Some(ShapeKind::Ellipse),
            (20.0, 20.0),
            vec![red.clone()],
        )]);
        assert_eq!(alpha(&c, 20, 20), 255);
        assert_eq!(alpha(&c, 10, 10), 0);
        assert!(
            c.pixels().iter().any(|p| p.alpha() > 0 && p.alpha() < 255),
            "antialiased boundary"
        );
        for clip in [ShapeKind::Path(vec![]), ShapeKind::Rect] {
            let c = canvas(&[group(
                Point::ZERO,
                TransformMatrix::identity(),
                Some(clip),
                (0.0, 0.0),
                vec![red.clone()],
            )]);
            assert!(c.pixels().iter().all(|p| p.alpha() == 0));
        }
    }
    #[test]
    fn raster_masks_apply_to_lines_and_image_cover_rect() {
        let line = FrameItem::Line {
            start: point(0.0, 10.0),
            end: point(50.0, 10.0),
            thickness: 4.0,
            color: Some(Color::rgb(255, 0, 0)),
            cap: LineCap::Round,
        };
        let c = canvas(&[group(
            point(4.0, 3.0),
            TransformMatrix::identity(),
            Some(ShapeKind::Rect),
            (10.0, 20.0),
            vec![line],
        )]);
        assert_eq!(alpha(&c, 8, 13), 255);
        assert_eq!(alpha(&c, 20, 13), 0);
        let mut texture = sk::Pixmap::new(8, 4).unwrap();
        texture.fill(sk::Color::from_rgba8(255, 0, 0, 255));
        let image = FrameItem::Image {
            pos: Point::ZERO,
            data: Arc::new(encode_png(&texture)),
            alt: None,
            width: Pt(32.0),
            height: Pt(16.0),
            intrinsic_width: 8,
            intrinsic_height: 4,
            clip_rect: Some(Rect { x: Pt(10.0), y: Pt(0.0), w: Pt(12.0), h: Pt(16.0) }),
            orientation: 1,
        };
        let c = canvas(&[group(
            point(3.0, 5.0),
            TransformMatrix::identity(),
            None,
            (1.0, 1.0),
            vec![image],
        )]);
        assert_eq!(alpha(&c, 14, 10), 255);
        assert_eq!(alpha(&c, 8, 10), 0);
        assert_eq!(alpha(&c, 26, 10), 0);
    }
    fn square_path(left: f64, top: f64, right: f64, bottom: f64) -> Vec<PathItem> {
        vec![
            PathItem::MoveTo(point(left, top)),
            PathItem::LineTo(point(right, top)),
            PathItem::LineTo(point(right, bottom)),
            PathItem::LineTo(point(left, bottom)),
            PathItem::ClosePath,
        ]
    }
    #[test]
    fn raster_fill_rule_preserves_holes_without_erasing_stroke() {
        let mut path = square_path(4.0, 4.0, 44.0, 44.0);
        path.extend(square_path(14.0, 14.0, 34.0, 34.0));
        for (rule, expected) in [(FillRule::NonZero, 255), (FillRule::EvenOdd, 0)] {
            let mut item = shape(ShapeKind::Path(path.clone()), rule, None);
            if let FrameItem::Shape { stroke, .. } = &mut item {
                *stroke = Some(Stroke { thickness: 2.0, ..Stroke::default() });
            }
            let c = canvas(&[item]);
            assert_eq!(alpha(&c, 24, 24), expected);
            assert_eq!(alpha(&c, 5, 5), 255);
            assert_eq!(alpha(&c, 14, 24), 255);
        }
    }
    #[test]
    fn raster_shape_stroke_preserves_caps_dashes_phase_and_miter_limit() {
        let line = ShapeKind::Line { dx: 40.0, dy: 0.0 };
        let paint = Paint::Solid(Color::rgb(255, 0, 0));
        let stroke = Stroke { paint, thickness: 4.0, ..Stroke::default() };
        let draw = |stroke: Stroke| {
            let mut item = shape(line.clone(), FillRule::NonZero, Some(stroke));
            if let FrameItem::Shape { pos, .. } = &mut item {
                *pos = point(8.0, 20.0)
            };
            canvas(&[item])
        };
        assert_eq!(alpha(&draw(stroke.clone()), 6, 20), 0);
        assert!(
            alpha(&draw(Stroke { cap: LineCap::Round, ..stroke.clone() }), 6, 20) > 0
        );
        for array in [
            vec![DashLength::Length(4.0), DashLength::Length(4.0)],
            vec![DashLength::Length(4.0)],
            vec![DashLength::LineWidth, DashLength::LineWidth],
        ] {
            let dashed = Stroke {
                dash: Some(DashPattern { array, phase: 0.0 }),
                ..stroke.clone()
            };
            let c = draw(dashed.clone());
            assert_eq!(alpha(&c, 9, 20), 255);
            assert_eq!(alpha(&c, 13, 20), 0);
            let mut shifted = dashed;
            shifted.dash.as_mut().unwrap().phase = 4.0;
            let c = draw(shifted);
            assert_eq!(alpha(&c, 9, 20), 0);
            assert_eq!(alpha(&c, 13, 20), 255);
        }
        let path = ShapeKind::Path(vec![
            PathItem::MoveTo(point(10.0, 35.0)),
            PathItem::LineTo(point(30.0, 15.0)),
            PathItem::LineTo(point(50.0, 35.0)),
        ]);
        for (join, limit, expected) in [
            (LineJoin::Miter, 4.0, true),
            (LineJoin::Miter, 1.0, false),
            (LineJoin::Bevel, 4.0, false),
            (LineJoin::Round, 4.0, false),
        ] {
            let c = canvas(&[shape(
                path.clone(),
                FillRule::NonZero,
                Some(Stroke {
                    join,
                    miter_limit: limit,
                    thickness: 8.0,
                    ..stroke.clone()
                }),
            )]);
            assert_eq!(alpha(&c, 30, 10) > 0, expected, "{join:?} limit={limit}");
        }
    }
    #[test]
    fn raster_masks_clip_shaped_glyph_coverage_without_changing_inner_pixels() {
        let bytes = include_bytes!("../../fixtures/fonts/NotoSans-Regular.ttf");
        let face = ttf_parser::Face::parse(bytes, 0).unwrap();
        let gid = face.glyph_index('M').unwrap().0;
        let style = TextStyle {
            size: Pt(30.0),
            font: Some(FontList::single("Noto Sans".into())),
            ..TextStyle::default()
        };
        let glyph = FrameItem::TextShaped {
            pos: point(2.0, 32.0),
            glyphs: vec![ShapedGlyph {
                glyph_id: gid,
                x_advance: 0,
                x_offset: 0,
                y_offset: 0,
                cluster: 0,
                char_code: 'M',
            }],
            style: style.clone(),
            text: "M".into(),
            units_per_em: face.units_per_em(),
            font: None,
            bidi: None,
        };
        let page = Page {
            width: 64.0,
            height: 64.0,
            numbering: None,
            logical_number: 1,
            supplement: typst_core::entities::content::Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items: vec![glyph.clone()],
        };
        let fonts = vec![(
            (
                FontList::single("Noto Sans".into()),
                crate::font_variant::text_style_to_font_variant(&style),
                FontVariations::default(),
            ),
            bytes.to_vec(),
        )];
        let prepared = super::super::font_binding::PreparedFonts::new(&page, &fonts);
        let mut plain = sk::Pixmap::new(64, 64).unwrap();
        render_item(&mut plain, State::new(1.0), &glyph, Some(&prepared));
        let clipped = group(
            Point::ZERO,
            TransformMatrix::identity(),
            Some(ShapeKind::Rect),
            (6.0, 64.0),
            vec![glyph],
        );
        let mut c = sk::Pixmap::new(64, 64).unwrap();
        render_item(&mut c, State::new(1.0), &clipped, Some(&prepared));
        assert!(plain.pixels().iter().any(|p| p.alpha() > 0));
        for y in 0..64 {
            for x in 0..64 {
                if x < 6 {
                    assert_eq!(c.pixel(x, y), plain.pixel(x, y));
                } else {
                    assert_eq!(alpha(&c, x, y), 0);
                }
            }
        }
    }
    #[test]
    fn raster_png_serializes_straight_rgba_without_darkening_transparency() {
        let mut c = sk::Pixmap::new(2, 2).unwrap();
        c.fill(sk::Color::from_rgba8(255, 0, 0, 128));
        let decoded = image::load_from_memory(&encode_png(&c)).unwrap().to_rgba8();
        assert_eq!(decoded.get_pixel(0, 0).0, [255, 0, 0, 128]);
    }

    #[test]
    fn raster_document_stack_premultiplies_decoded_pages_before_composition() {
        let page = Page {
            width: 4.0,
            height: 4.0,
            numbering: None,
            logical_number: 1,
            supplement: typst_core::entities::content::Content::Empty,
            bleed: Default::default(),
            fill: typst_core::entities::page_canvas::PageFill::None,
            background: vec![],
            foreground: vec![],
            items: vec![FrameItem::Shape {
                pos: Point::ZERO,
                kind: ShapeKind::Rect,
                width: 4.0,
                height: 4.0,
                fill: Some(Paint::Solid(Color::rgba(64, 128, 192, 128))),
                stroke: None,
                fill_rule: FillRule::NonZero,
                parent_bbox_at_emit: None,
            }],
        };
        let doc = PagedDocument::new(vec![page.clone(), page]);
        let png = render_document_to_png(
            &doc,
            &RenderOptions { pixel_per_pt: 1.0, render_bleed: false },
            2.0,
        );
        let image = image::load_from_memory(&png).unwrap().to_rgba8();
        assert_eq!(image.dimensions(), (4, 10));
        assert_eq!(image.get_pixel(1, 1).0, [159, 191, 223, 255]);
        assert_eq!(image.get_pixel(1, 5).0, [255, 255, 255, 255]);
        assert_eq!(image.get_pixel(1, 7).0, [159, 191, 223, 255]);
    }
}

#[cfg(test)]
mod affine_glyph_regressions {
    use super::*;

    fn draw(ts: sk::Transform, mask: Option<&sk::Mask>) -> sk::Pixmap {
        let bytes = include_bytes!("../../fixtures/fonts/NotoSans-Regular.ttf");
        let face = ttf_parser::Face::parse(bytes, 0).unwrap();
        let mut canvas = sk::Pixmap::new(180, 180).unwrap();
        render_glyph(
            &mut canvas,
            State { transform: ts, mask, ..State::new(1.0) },
            &face,
            face.glyph_index('L').unwrap(),
            30.0,
            50.0,
            30.0,
            sk::Color::from_rgba8(20, 40, 80, 160),
        );
        canvas
    }

    fn centroid(canvas: &sk::Pixmap) -> (f64, f64) {
        let (mut mass, mut x, mut y) = (0.0, 0.0, 0.0);
        for py in 0..canvas.height() {
            for px in 0..canvas.width() {
                let alpha = canvas.pixel(px, py).unwrap().alpha() as f64;
                mass += alpha;
                x += (px as f64 + 0.5) * alpha;
                y += (py as f64 + 0.5) * alpha;
            }
        }
        assert!(mass > 0.0, "transformed glyph must retain its ink");
        (x / mass, y / mass)
    }

    #[test]
    fn affine_glyph_preserves_ink_position_under_rotation_scale_and_reflection() {
        let (x, y) = centroid(&draw(sk::Transform::identity(), None));
        for ts in [
            sk::Transform::from_row(0.0, 1.0, -1.0, 0.0, 140.0, 0.0),
            sk::Transform::from_scale(2.0, 1.0),
            sk::Transform::from_row(1.0, 0.0, 0.4, 1.0, 0.0, 0.0),
            sk::Transform::from_row(1.0, 0.0, 0.0, -1.0, 0.0, 140.0),
        ] {
            let (actual_x, actual_y) = centroid(&draw(ts, None));
            let expected_x = x * ts.sx as f64 + y * ts.kx as f64 + ts.tx as f64;
            let expected_y = x * ts.ky as f64 + y * ts.sy as f64 + ts.ty as f64;
            assert!(
                (actual_x - expected_x).abs() < 1.0
                    && (actual_y - expected_y).abs() < 1.0,
                "{ts:?}: ({actual_x},{actual_y}) vs ({expected_x},{expected_y})"
            );
        }
    }

    #[test]
    fn affine_glyph_outline_obeys_device_clip_and_keeps_alpha() {
        let ts = sk::Transform::from_row(0.0, 1.0, -1.0, 0.0, 140.0, 0.0);
        let plain = draw(ts, None);
        let mut mask = sk::Mask::new(180, 180).unwrap();
        for y in 0..180 {
            for x in 0..100 {
                mask.data_mut()[y * 180 + x] = 255;
            }
        }
        let clipped = draw(ts, Some(&mask));
        let mut retained = false;
        let mut removed = false;
        for y in 0..180 {
            for x in 0..180 {
                let before = plain.pixel(x, y).unwrap();
                let after = clipped.pixel(x, y).unwrap();
                if x < 100 {
                    assert_eq!(before, after);
                    retained |= after.alpha() > 0;
                } else {
                    assert_eq!(after.alpha(), 0);
                    removed |= before.alpha() > 0;
                }
                assert!(after.alpha() <= 160);
            }
        }
        assert!(retained && removed, "the clip must cut actual transformed ink");
    }
}
