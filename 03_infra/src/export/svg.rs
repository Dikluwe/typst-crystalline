//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/svg.md
//! @prompt-hash 00cab541
//! @layer L3
//! @updated 2026-09-28
//! SVG facade and exhaustive renderer dispatcher.

use geometry::{render_line, render_shape};
use glyphs::{
    render_direct_glyph, render_html_text, render_text_shaped, write_glyph_fallback,
    GlyphDefs,
};
pub use glyphs::{GlyphFontRequest, SvgGlyphFontContext};
use groups::render_group;
use images::render_image;
use navigation::render_link;
pub use navigation::SvgDestinationContext;
use paints::PaintDefs;
use serialization::{color_to_css, fmt_num};
use typst_core::entities::font_book::FontVariant;
use typst_core::entities::font_list::FontList;
use typst_core::entities::font_variations::FontVariations;
use typst_core::entities::layout_types::{FrameItem, Page, Pt, Size, TransformMatrix};
use xmlwriter::XmlWriter;

mod geometry;
mod glyphs;
mod groups;
mod images;
mod navigation;
mod paints;
#[cfg(test)]
mod pretty_serialization_tests;
mod serialization;
#[cfg(test)]
mod tests;

/// Opções de exportação SVG.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SvgOptions {
    /// Se true, formata o SVG com indentação.
    pub pretty: bool,
    /// Se true, inclui o canvas físico; false recorta ao TrimBox.
    pub render_bleed: bool,
}

/// Tipo da chave de fontes usada pelas variantes `_with_fonts`.
pub type FontKey = ((FontList, FontVariant, FontVariations), Vec<u8>);
pub type GlyphFontIdentity = (FontList, FontVariant, FontVariations);

/// Exporta uma página para SVG (texto sem fontes resolvidas é omitido).
pub fn export_svg(page: &Page, opts: &SvgOptions) -> String {
    export_svg_with_context(page, opts, &SvgDestinationContext::empty())
}

pub fn export_svg_with_context(
    page: &Page,
    opts: &SvgOptions,
    destinations: &SvgDestinationContext,
) -> String {
    export_svg_with_fonts_inner(
        page,
        opts,
        None,
        destinations,
        &SvgGlyphFontContext::empty(),
        false,
    )
}

/// Exporta uma página para SVG com fontes resolvidas.
///
/// O texto é renderizado como outlines de glifos nas fontes resolvidas
/// pelo mapa de `fonts`. Se uma fonte não for encontrada, o run é omitido.
pub fn export_svg_with_fonts(
    page: &Page,
    opts: &SvgOptions,
    fonts: &[FontKey],
) -> String {
    export_svg_with_fonts_and_context(page, opts, fonts, &SvgDestinationContext::empty())
}

pub fn export_svg_with_fonts_and_context(
    page: &Page,
    opts: &SvgOptions,
    fonts: &[FontKey],
    destinations: &SvgDestinationContext,
) -> String {
    export_svg_with_fonts_and_contexts(
        page,
        opts,
        fonts,
        destinations,
        &SvgGlyphFontContext::empty(),
    )
}

pub fn export_svg_with_fonts_and_contexts(
    page: &Page,
    opts: &SvgOptions,
    fonts: &[FontKey],
    destinations: &SvgDestinationContext,
    glyph_fonts: &SvgGlyphFontContext,
) -> String {
    export_svg_with_fonts_inner(page, opts, Some(fonts), destinations, glyph_fonts, false)
}

/// Exporta um fragmento intrínseco para inserção inline em HTML.
pub fn export_svg_html_frame_with_fonts(
    page: &Page,
    opts: &SvgOptions,
    fonts: &[FontKey],
    glyph_fonts: &SvgGlyphFontContext,
) -> String {
    export_svg_with_fonts_inner(
        page,
        opts,
        Some(fonts),
        &SvgDestinationContext::empty(),
        glyph_fonts,
        true,
    )
}

impl SvgOptions {
    pub fn export_html_frame_with_fonts(
        &self,
        page: &Page,
        fonts: &[FontKey],
        glyph_fonts: &SvgGlyphFontContext,
    ) -> String {
        export_svg_html_frame_with_fonts(page, self, fonts, glyph_fonts)
    }
}

fn export_svg_with_fonts_inner(
    page: &Page,
    opts: &SvgOptions,
    fonts: Option<&[FontKey]>,
    destinations: &SvgDestinationContext,
    glyph_fonts: &SvgGlyphFontContext,
    html_frame: bool,
) -> String {
    let prepared = fonts.map(|f| super::font_binding::PreparedFonts::new(page, f));
    let fonts = prepared.as_ref();
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
    if html_frame {
        xml.write_attribute("style", "display: block; overflow: visible");
    }

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

    destinations.write_nodes(&mut xml);

    let mut glyph_defs = GlyphDefs::default();
    let mut paint_defs = PaintDefs::default();
    let mut clip_next_id = 0usize;
    let mut session = SvgRenderSession {
        fonts,
        glyph_defs: &mut glyph_defs,
        paint_defs: &mut paint_defs,
        clip_next_id: &mut clip_next_id,
        destinations,
        glyph_fonts,
        html_frame,
    };
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
        render_item(&mut xml, item, &mut session);
    }
    if opts.render_bleed {
        xml.end_element();
    }

    // Emite as definições de glifos antes de fechar o SVG.
    session.paint_defs.write(
        &mut xml,
        session.fonts,
        session.glyph_defs,
        session.clip_next_id,
        session.glyph_fonts,
    );
    session.glyph_defs.write(&mut xml);

    xml.end_element();
    xml.end_document()
}

struct SvgRenderSession<'a> {
    fonts: Option<&'a super::font_binding::PreparedFonts>,
    glyph_defs: &'a mut GlyphDefs,
    paint_defs: &'a mut PaintDefs,
    clip_next_id: &'a mut usize,
    destinations: &'a SvgDestinationContext,
    glyph_fonts: &'a SvgGlyphFontContext,
    html_frame: bool,
}

fn render_item(
    xml: &mut XmlWriter,
    item: &FrameItem,
    session: &mut SvgRenderSession<'_>,
) {
    match item {
        FrameItem::TextSeparator { pos, style, .. } => {
            if session.html_frame {
                render_html_text(xml, pos, &ecow::EcoString::from(" "), style);
            }
        }
        FrameItem::Text { pos, text, style, .. } => {
            if session.html_frame {
                render_html_text(xml, pos, text, style);
            }
        }
        FrameItem::TextShaped {
            pos, glyphs, style, text, units_per_em, font, ..
        } => {
            if session.html_frame {
                render_html_text(xml, pos, text, style);
            } else if let Some(fonts) = session.fonts {
                render_text_shaped(
                    xml,
                    pos,
                    glyphs,
                    style,
                    *units_per_em,
                    fonts,
                    session.glyph_defs,
                    font.as_ref(),
                );
            }
        }
        FrameItem::Line { start, end, thickness, color, .. } => {
            render_line(xml, start, end, *thickness, color.clone());
        }
        FrameItem::Glyph { pos, glyph_id, size, style, base_char, .. } => {
            if let Some(fonts) = session.fonts {
                render_direct_glyph(
                    xml,
                    pos,
                    *glyph_id,
                    *size,
                    style,
                    *base_char,
                    fonts.fonts(),
                    session.glyph_fonts,
                    session.glyph_defs,
                );
            } else {
                write_glyph_fallback(xml, pos, "fontless-wrapper");
            }
        }
        FrameItem::Image {
            pos, data, width, height, clip_rect, orientation, ..
        } => {
            render_image(
                xml,
                pos,
                data,
                *width,
                *height,
                clip_rect.as_ref(),
                *orientation,
                session.clip_next_id,
            );
        }
        FrameItem::Shape {
            pos, kind, width, height, fill, stroke, fill_rule, ..
        } => {
            render_shape(
                xml,
                pos,
                kind,
                *width,
                *height,
                fill.as_ref(),
                stroke.as_ref(),
                *fill_rule,
                session.paint_defs,
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
        } => {
            let previous_root = session.paint_defs.local_to_root;
            session.paint_defs.local_to_root = if *paint_root {
                TransformMatrix::identity()
            } else {
                previous_root
                    .concat(&TransformMatrix::translate(pos.x.0, pos.y.0))
                    .concat(matrix)
            };
            render_group(
                xml,
                pos,
                matrix,
                clip_mask.as_ref(),
                Size { width: Pt(*inner_width), height: Pt(*inner_height) },
                items,
                session,
            );
            session.paint_defs.local_to_root = previous_root;
        }
        FrameItem::Link { items, .. } if session.paint_defs.decorative => {
            let previous_html_frame = session.html_frame;
            session.html_frame = false;
            for item in items {
                render_item(xml, item, session);
            }
            session.html_frame = previous_html_frame;
        }
        FrameItem::Link { target, items, pos, size } => {
            render_link(xml, target, items, pos, size, session);
        }
        FrameItem::TextReplacement { items, .. } | FrameItem::Semantic { items, .. } => {
            for item in items {
                render_item(xml, item, session);
            }
        }
    }
}
