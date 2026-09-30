//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/svg.md
//! @prompt-hash aef9223a
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
use typst_core::entities::layout_types::{FrameItem, Page};
use xmlwriter::XmlWriter;

mod geometry;
mod glyphs;
mod groups;
mod images;
mod navigation;
#[cfg(test)]
mod p1360_ab_tests;
mod paints;
mod serialization;
#[cfg(test)]
mod tests;

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
        render_item(
            &mut xml,
            item,
            fonts,
            &mut glyph_defs,
            &mut paint_defs,
            &mut clip_next_id,
            destinations,
            glyph_fonts,
            html_frame,
        );
    }
    if opts.render_bleed {
        xml.end_element();
    }

    // Emite as definições de glifos antes de fechar o SVG.
    paint_defs.write(&mut xml);
    glyph_defs.write(&mut xml);

    xml.end_element();
    xml.end_document()
}

fn render_item(
    xml: &mut XmlWriter,
    item: &FrameItem,
    fonts: Option<&[FontKey]>,
    glyph_defs: &mut GlyphDefs,
    paint_defs: &mut PaintDefs,
    clip_next_id: &mut usize,
    destinations: &SvgDestinationContext,
    glyph_fonts: &SvgGlyphFontContext,
    html_frame: bool,
) {
    match item {
        FrameItem::TextSeparator { pos, style, .. } => { if html_frame { render_html_text(xml, pos, &ecow::EcoString::from(" "), style); } },
        FrameItem::Text { pos, text, style } => {
            if html_frame {
                render_html_text(xml, pos, text, style);
            }
        }
        FrameItem::TextShaped { pos, glyphs, style, text, units_per_em } => {
            if html_frame {
                render_html_text(xml, pos, text, style);
            } else if let Some(fonts) = fonts {
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
        FrameItem::Line { start, end, thickness, color, .. } => {
            render_line(xml, start, end, *thickness, color.clone());
        }
        FrameItem::Glyph { pos, glyph_id, size, style, base_char, .. } => {
            if let Some(fonts) = fonts {
                render_direct_glyph(
                    xml,
                    pos,
                    *glyph_id,
                    *size,
                    style,
                    *base_char,
                    fonts,
                    glyph_fonts,
                    glyph_defs,
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
                clip_next_id,
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
                paint_defs,
            );
        }
        FrameItem::Group {
            pos,
            matrix,
            clip_mask,
            inner_width,
            inner_height,
            items,
        } => {
            render_group(
                xml,
                pos,
                matrix,
                clip_mask.as_ref(),
                *inner_width,
                *inner_height,
                items,
                fonts,
                glyph_defs,
                paint_defs,
                clip_next_id,
                destinations,
                glyph_fonts,
                html_frame,
            );
        }
        FrameItem::Link { target, items, pos, size } => {
            render_link(
                xml,
                target,
                items,
                pos,
                size,
                fonts,
                glyph_defs,
                paint_defs,
                clip_next_id,
                destinations,
                glyph_fonts,
                html_frame,
            );
        }
        FrameItem::TextReplacement { items, .. } | FrameItem::Semantic { items, .. } => {
            for item in items {
                render_item(
                    xml,
                    item,
                    fonts,
                    glyph_defs,
                    paint_defs,
                    clip_next_id,
                    destinations,
                    glyph_fonts,
                    html_frame,
                );
            }
        }
    }
}
