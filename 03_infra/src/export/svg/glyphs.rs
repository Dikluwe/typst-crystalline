//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/svg/glyphs.md
//! @prompt-hash 348940e6
//! @layer L3
//! @updated 2026-10-24

use super::serialization::{color_to_css, fmt_num};
use super::{FontKey, GlyphFontIdentity};
use crate::font_variant::{
    axis_variations_for_font_variant, merge_explicit_variations,
    text_style_to_font_variant,
};
use std::collections::HashMap;
use ttf_parser::{Face, GlyphId, OutlineBuilder};
use typst_core::entities::font_book::FontVariant;
use typst_core::entities::font_list::FontList;
use typst_core::entities::font_variations::FontVariations;
use typst_core::entities::layout_types::{Point, Pt};
use typst_core::entities::shaped_glyph::ShapedGlyph;
use xmlwriter::XmlWriter;

#[derive(Debug, Clone, PartialEq)]
pub struct GlyphFontRequest {
    base_char: char,
    requested_font: Option<FontList>,
    variant: FontVariant,
    variations: FontVariations,
    math: bool,
}

impl GlyphFontRequest {
    pub fn new(
        base_char: char,
        style: &typst_core::entities::layout_types::TextStyle,
    ) -> Self {
        Self {
            base_char,
            requested_font: style.font.clone(),
            variant: text_style_to_font_variant(style),
            variations: style.variations.clone().unwrap_or_default(),
            math: style.math,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SvgGlyphFontContext {
    entries: Vec<(GlyphFontRequest, Option<GlyphFontIdentity>)>,
}

impl SvgGlyphFontContext {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn from_resolutions(
        resolutions: impl IntoIterator<Item = (GlyphFontRequest, GlyphFontIdentity)>,
    ) -> Self {
        let mut entries: Vec<(GlyphFontRequest, Option<GlyphFontIdentity>)> = Vec::new();
        for (request, identity) in resolutions {
            if let Some((_, current)) =
                entries.iter_mut().find(|(key, _)| key == &request)
            {
                if current.as_ref().is_some_and(|value| value != &identity) {
                    *current = None;
                }
            } else {
                entries.push((request, Some(identity)));
            }
        }
        Self { entries }
    }

    fn identity_for(&self, request: &GlyphFontRequest) -> Option<&GlyphFontIdentity> {
        self.entries
            .iter()
            .find(|(key, _)| key == request)
            .and_then(|(_, value)| value.as_ref())
    }
}
/// Cache de definições de glifos para o SVG.
/// Cada glifo único (fonte + glyph_id + escala) é extraído uma vez e
/// referenciado via `<use xlink:href="#id"/>`.
enum GlyphDefinition {
    Outline(String),
    Raster { data: Vec<u8>, x: f64, y: f64, width: f64, height: f64 },
}

#[derive(Default)]
pub(super) struct GlyphDefs {
    next_id: usize,
    ids: HashMap<(usize, u16, u64), String>,
    paths: Vec<(String, GlyphDefinition)>,
}
impl GlyphDefs {
    /// Obtém ou cria um identificador para o glifo dado.
    fn get_or_insert(
        &mut self,
        font_idx: usize,
        glyph_id: u16,
        font: &FontKey,
        scale: f64,
    ) -> Option<String> {
        let key = (font_idx, glyph_id, scale.to_bits());
        if let Some(id) = self.ids.get(&key) {
            return Some(id.clone());
        }

        let (font_key, font_data) = font;
        let mut face = Face::parse(font_data, 0).ok()?;
        for axis in merge_explicit_variations(
            axis_variations_for_font_variant(&font_key.1),
            &font_key.2,
        ) {
            face.set_variation(axis.tag, axis.value);
        }
        let mut builder = SvgGlyphPathBuilder::new(scale);
        let path = if face.outline_glyph(GlyphId(glyph_id), &mut builder).is_some() {
            GlyphDefinition::Outline(builder.finish())
        } else {
            let image = face.glyph_raster_image(GlyphId(glyph_id), u16::MAX)?;
            if image.format != ttf_parser::RasterImageFormat::PNG
                || image.pixels_per_em == 0
            {
                return None;
            }
            let factor =
                scale * f64::from(face.units_per_em()) / f64::from(image.pixels_per_em);
            GlyphDefinition::Raster {
                data: image.data.to_vec(),
                x: f64::from(image.x) * factor,
                y: f64::from(image.y) * factor,
                width: f64::from(image.width) * factor,
                height: f64::from(image.height) * factor,
            }
        };

        let id = format!("g{}", self.next_id);
        self.next_id += 1;
        self.ids.insert(key, id.clone());
        self.paths.push((id.clone(), path));
        Some(id)
    }

    pub(super) fn write(&self, xml: &mut XmlWriter) {
        if self.paths.is_empty() {
            return;
        }
        xml.start_element("defs");
        for (id, path) in &self.paths {
            xml.start_element("symbol");
            xml.write_attribute("id", id);
            xml.write_attribute("overflow", "visible");
            match path {
                GlyphDefinition::Outline(path) => {
                    xml.start_element("path");
                    xml.write_attribute("d", path);
                    xml.end_element();
                }
                GlyphDefinition::Raster { data, x, y, width, height } => {
                    use base64::Engine;
                    xml.start_element("image");
                    xml.write_attribute(
                        "xlink:href",
                        &format!(
                            "data:image/png;base64,{}",
                            base64::engine::general_purpose::STANDARD.encode(data)
                        ),
                    );
                    xml.write_attribute("x", &fmt_num(*x));
                    xml.write_attribute("y", &fmt_num(-y - height));
                    xml.write_attribute("width", &fmt_num(*width));
                    xml.write_attribute("height", &fmt_num(*height));
                    xml.write_attribute("transform", "scale(1 -1)");
                    xml.write_attribute("preserveAspectRatio", "none");
                    xml.end_element();
                }
            }
            xml.end_element();
        }
        xml.end_element();
    }
}

/// Builder que converte um outline TrueType em string SVG `d`.
/// Usa comandos relativos (`m`, `l`, `q`, `c`, `Z`), como o vanilla.
pub(super) struct SvgGlyphPathBuilder {
    path: String,
    scale: f64,
    last_point: (f64, f64),
    last_close: (f64, f64),
}

impl SvgGlyphPathBuilder {
    pub(super) fn new(scale: f64) -> Self {
        Self {
            path: String::from("M 0 0"),
            scale,
            last_point: (0.0, 0.0),
            last_close: (0.0, 0.0),
        }
    }

    pub(super) fn finish(self) -> String {
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
pub(super) fn write_glyph_fallback(
    xml: &mut XmlWriter,
    pos: &Point,
    reason: &'static str,
) {
    xml.start_element("g");
    xml.write_attribute("data-crystalline-glyph-fallback", reason);
    xml.write_attribute(
        "transform",
        &format!("translate({} {})", fmt_num(pos.x.0), fmt_num(pos.y.0)),
    );
    xml.end_element();
}

#[allow(clippy::too_many_arguments)]
pub(super) fn render_direct_glyph(
    xml: &mut XmlWriter,
    pos: &Point,
    glyph_id: u16,
    size: Pt,
    style: &typst_core::entities::layout_types::TextStyle,
    base_char: char,
    fonts: &[FontKey],
    glyph_fonts: &SvgGlyphFontContext,
    glyph_defs: &mut GlyphDefs,
) {
    if !pos.x.0.is_finite() || !pos.y.0.is_finite() || !size.0.is_finite() {
        write_glyph_fallback(xml, pos, "invalid-geometry");
        return;
    }
    let request = GlyphFontRequest::new(base_char, style);
    let Some(identity) = glyph_fonts.identity_for(&request) else {
        write_glyph_fallback(xml, pos, "font-identity-unknown");
        return;
    };
    let Some((font_idx, font_key)) = fonts
        .iter()
        .enumerate()
        .find(|(_, (candidate, _))| candidate == identity)
    else {
        write_glyph_fallback(xml, pos, "mapped-font-missing");
        return;
    };
    let Ok(face) = Face::parse(&font_key.1, 0) else {
        write_glyph_fallback(xml, pos, "invalid-font");
        return;
    };
    let units_per_em = face.units_per_em();
    if units_per_em == 0 {
        write_glyph_fallback(xml, pos, "invalid-scale");
        return;
    }
    let scale = size.0 / f64::from(units_per_em);
    let Some(id) = glyph_defs.get_or_insert(font_idx, glyph_id, font_key, scale) else {
        write_glyph_fallback(xml, pos, "outline-unavailable");
        return;
    };
    xml.start_element("use");
    xml.write_attribute("xlink:href", &format!("#{id}"));
    xml.write_attribute(
        "transform",
        &format!("matrix(1 0 0 -1 {} {})", fmt_num(pos.x.0), fmt_num(pos.y.0)),
    );
    xml.write_attribute(
        "fill",
        &style
            .fill
            .clone()
            .map(color_to_css)
            .unwrap_or_else(|| "#000000".to_string()),
    );
    xml.end_element();
}

#[expect(
    clippy::too_many_arguments,
    reason = "C066: Leaf SVG usa oito inputs efetivos de contorno, face, escala e definições; texto HTML segue writer separado"
)]
pub(super) fn render_text_shaped(
    xml: &mut XmlWriter,
    pos: &Point,
    glyphs: &[ShapedGlyph],
    style: &typst_core::entities::layout_types::TextStyle,
    units_per_em: u16,
    fonts: &super::super::font_binding::PreparedFonts,
    glyph_defs: &mut GlyphDefs,
    font: Option<&typst_core::entities::world_types::Font>,
) {
    let Some((font_idx, font_key)) = fonts.get_original(style, font) else { return };

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

    let fill = style
        .fill
        .clone()
        .map(color_to_css)
        .unwrap_or_else(|| "#000000".to_string());
    let mut x = 0.0;
    let y = 0.0;
    for (index, glyph) in glyphs.iter().enumerate() {
        let x_offset = f64::from(glyph.x_offset) * scale;
        let y_offset = f64::from(glyph.y_offset) * scale;
        let Some(id) =
            glyph_defs.get_or_insert(font_idx, glyph.glyph_id, font_key, scale)
        else {
            x += f64::from(glyph.x_advance) * scale
                + crate::shaper::tracking::residual(
                    glyphs,
                    index,
                    style,
                    f64::from(units_per_em),
                );
            continue;
        };

        xml.start_element("use");
        xml.write_attribute("xlink:href", &format!("#{id}"));
        xml.write_attribute("x", &fmt_num(x + x_offset));
        xml.write_attribute("y", &fmt_num(y + y_offset));
        xml.write_attribute("fill", &fill);
        xml.end_element();

        x += f64::from(glyph.x_advance) * scale
            + crate::shaper::tracking::residual(
                glyphs,
                index,
                style,
                f64::from(units_per_em),
            );
    }

    xml.end_element();
}
pub(super) fn render_html_text(
    xml: &mut XmlWriter,
    pos: &Point,
    text: &ecow::EcoString,
    style: &typst_core::entities::layout_types::TextStyle,
) {
    xml.start_element("text");
    xml.write_attribute("x", &fmt_num(pos.x.0));
    xml.write_attribute("y", &fmt_num(pos.y.0));
    xml.write_attribute("font-size", &format!("{}pt", fmt_num(style.size.0)));
    xml.write_attribute(
        "fill",
        &style
            .fill
            .clone()
            .map(color_to_css)
            .unwrap_or_else(|| "#000000".to_string()),
    );
    xml.write_text(text.as_str());
    xml.end_element();
}

#[cfg(test)]
mod r10_bitmap_tests {
    use super::*;
    #[test]
    fn r10_missing_outline_raster_retains_actual_glyph_paint() {
        let bytes = include_bytes!("../../../fixtures/fonts/raster-glyph.ttf");
        let face = Face::parse(bytes, 0).unwrap();
        let gid = face.glyph_index('☃').unwrap();
        assert!(face.glyph_raster_image(gid, u16::MAX).is_some());
        let key = (
            (
                FontList::single(
                    crate::fonts::font_info_from_bytes(bytes, 0).unwrap().family.into(),
                ),
                FontVariant::default(),
                FontVariations::default(),
            ),
            bytes.to_vec(),
        );
        let mut defs = GlyphDefs::default();
        let first = defs
            .get_or_insert(0, gid.0, &key, 0.01)
            .expect("a supplied PNG raster glyph must remain visible");
        assert_eq!(Some(first), defs.get_or_insert(0, gid.0, &key, 0.01));
        let mut xml = XmlWriter::new(xmlwriter::Options::default());
        defs.write(&mut xml);
        let svg = xml.end_document();
        assert!(svg.contains("data:image/png;base64,"));
    }
}

#[cfg(test)]
mod r10_fractional_tracking_tests {
    use super::*;
    use typst_core::entities::{
        content::Content,
        layout_types::{FrameItem, Length, Page, TextStyle},
    };
    #[test]
    fn r10_svg_positions_preserve_signed_fractional_tracking_between_clusters() {
        let bytes = include_bytes!("../../../fixtures/fonts/DejaVuSans.ttf");
        let face = Face::parse(bytes, 0).unwrap();
        let upem = f64::from(face.units_per_em());
        let family = FontList::single("DejaVu Sans".into());
        let pattern = regex::Regex::new(r#"<use\b[^>]*\bx="([^"]+)""#).unwrap();
        for size in [11.0, 6.6] {
            for tracking in [0.2, -0.2, 0.333] {
                let q = (tracking * upem / size).round() as i32;
                let mut nominal = Vec::new();
                let glyphs = ['A', 'B', 'C']
                    .into_iter()
                    .enumerate()
                    .map(|(i, c)| {
                        let gid = face.glyph_index(c).unwrap();
                        let width = i32::from(face.glyph_hor_advance(gid).unwrap());
                        nominal.push(width);
                        ShapedGlyph {
                            glyph_id: gid.0,
                            x_advance: width + if i == 1 { q } else { 0 },
                            x_offset: 0,
                            y_offset: 0,
                            cluster: if i < 2 { 0 } else { 2 },
                            char_code: c,
                        }
                    })
                    .collect();
                let style = TextStyle {
                    font: Some(family.clone()),
                    size: Pt(size),
                    tracking: Some(Length::pt(tracking)),
                    ..TextStyle::default()
                };
                let page = Page {
                    width: 100.0,
                    height: 100.0,
                    logical_number: 1,
                    numbering: None,
                    supplement: Content::Empty,
                    bleed: Default::default(),
                    fill: Default::default(),
                    background: vec![],
                    foreground: vec![],
                    items: vec![FrameItem::TextShaped {
                        pos: Point { x: Pt(0.0), y: Pt(20.0) },
                        glyphs,
                        style,
                        text: "ABC".into(),
                        units_per_em: face.units_per_em(),
                        bidi: None,
                        font: None,
                    }],
                };
                let fonts = vec![(
                    (family.clone(), FontVariant::default(), FontVariations::default()),
                    bytes.to_vec(),
                )];
                let svg = crate::export::svg::export_svg_with_fonts(
                    &page,
                    &crate::export::svg::SvgOptions::default(),
                    &fonts,
                );
                let xs = pattern
                    .captures_iter(&svg)
                    .map(|capture| capture[1].parse::<f64>().unwrap())
                    .collect::<Vec<_>>();
                assert_eq!(xs.len(), 3);
                assert!(
                    (xs[1] - f64::from(nominal[0]) * size / upem).abs() < 1e-8,
                    "same source cluster must not acquire another interval"
                );
                let expected =
                    f64::from(nominal[0] + nominal[1]) * size / upem + tracking;
                assert!(
                    (xs[2] - expected).abs() < 1e-8,
                    "size={size} tracking={tracking} actual={} expected={expected}",
                    xs[2]
                );
            }
        }
    }
}
