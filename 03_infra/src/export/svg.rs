//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/svg.md
//! @prompt-hash bc81b63a
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
use typst_core::entities::gradient::Gradient;
use typst_core::entities::image_format::{detect_image_format, ImageFormat};
use typst_core::entities::label::Label;
use typst_core::entities::layout_types::{
    Color, FrameItem, LinkTarget, Page, Point, Pt, Size, TransformMatrix,
};
use typst_core::entities::paint::Paint;
use typst_core::entities::tiling::{TilingBody, TilingRelative};

use typst_core::entities::shaped_glyph::ShapedGlyph;

use super::gradients::svg_adaptive_stops;
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
pub type GlyphFontIdentity = (FontList, FontVariant, FontVariations);

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

/// Registro imutável de destinos internos pertencentes à página SVG atual.
#[derive(Debug, Clone, Default)]
pub struct SvgDestinationContext {
    entries: Vec<SvgDestination>,
}

#[derive(Debug, Clone)]
struct SvgDestination {
    label: Label,
    point: Point,
    id: String,
}

impl SvgDestinationContext {
    pub fn empty() -> Self {
        Self::default()
    }

    /// Constrói identidades locais por ordem semântica de label, nunca por
    /// posição, endereço ou ordem incidental do mapa de origem.
    pub fn from_destinations(
        destinations: impl IntoIterator<Item = (Label, Point)>,
    ) -> Self {
        let mut values: Vec<(Label, Point)> = destinations
            .into_iter()
            .filter(|(_, point)| point.x.0.is_finite() && point.y.0.is_finite())
            .collect();
        values.sort_by(|(a, _), (b, _)| a.0.cmp(&b.0));

        let mut entries: Vec<SvgDestination> = Vec::new();
        for (label, point) in values {
            if let Some(previous) = entries.last() {
                if previous.label == label {
                    // O registry produtivo é único por Label. Duplicatas não
                    // acrescentam nós; conflito não é promovido pelo exporter.
                    continue;
                }
            }
            let id = format!("crystalline-destination-{}", entries.len());
            entries.push(SvgDestination { label, point, id });
        }
        Self { entries }
    }

    pub fn id_for(&self, label: &Label) -> Option<&str> {
        self.entries
            .binary_search_by(|entry| entry.label.0.cmp(&label.0))
            .ok()
            .map(|index| self.entries[index].id.as_str())
    }

    fn write_nodes(&self, xml: &mut XmlWriter) {
        for entry in &self.entries {
            xml.start_element("g");
            xml.write_attribute("id", &entry.id);
            xml.write_attribute("data-crystalline-destination", &entry.label.0);
            xml.write_attribute(
                "transform",
                &format!(
                    "translate({} {})",
                    fmt_num(entry.point.x.0),
                    fmt_num(entry.point.y.0)
                ),
            );
            xml.end_element();
        }
    }
}

/// Cache de definições de glifos para o SVG.
/// Cada glifo único (fonte + glyph_id + escala) é extraído uma vez e
/// referenciado via `<use xlink:href="#id"/>`.
#[derive(Default)]
struct GlyphDefs {
    next_id: usize,
    ids: HashMap<(usize, u16, u64), String>,
    paths: Vec<(String, String)>,
}

/// Paint servers referenced by shapes. The linear vector is intentional: Paint
/// has semantic equality but no mechanical Hash, and SVG pages normally carry
/// few distinct paints.
#[derive(Default)]
struct PaintDefs {
    paints: Vec<SvgPaintDef>,
}

struct SvgPaintDef {
    paint: Paint,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

struct PaintReference {
    value: String,
    fallback: Option<&'static str>,
}

impl PaintDefs {
    fn reference(
        &mut self,
        paint: &Paint,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> PaintReference {
        if let Paint::Solid(color) = paint {
            return PaintReference { value: color_to_css(*color), fallback: None };
        }
        if !paint_is_svg_native(paint) {
            return PaintReference {
                value: color_to_css(paint.to_color()),
                fallback: Some(unsupported_paint_reason(paint)),
            };
        }
        let (paint_x, paint_y, paint_width, paint_height) =
            if matches!(paint, Paint::Gradient(Gradient::Conic(_)))
                && width.is_finite()
                && height.is_finite()
                && width > 0.0
                && height > 0.0
            {
                (x, y, width, height)
            } else {
                (0.0, 0.0, 1.0, 1.0)
            };
        let index = self
            .paints
            .iter()
            .position(|candidate| {
                candidate.paint == *paint
                    && candidate.x.to_bits() == paint_x.to_bits()
                    && candidate.y.to_bits() == paint_y.to_bits()
                    && candidate.width.to_bits() == paint_width.to_bits()
                    && candidate.height.to_bits() == paint_height.to_bits()
            })
            .unwrap_or_else(|| {
                self.paints.push(SvgPaintDef {
                    paint: paint.clone(),
                    x: paint_x,
                    y: paint_y,
                    width: paint_width,
                    height: paint_height,
                });
                self.paints.len() - 1
            });
        PaintReference { value: format!("url(#p{index})"), fallback: None }
    }

    fn write(&self, xml: &mut XmlWriter) {
        if self.paints.is_empty() {
            return;
        }
        xml.start_element("defs");
        for (index, definition) in self.paints.iter().enumerate() {
            write_paint_def(
                xml,
                &format!("p{index}"),
                &definition.paint,
                definition.x,
                definition.y,
                definition.width,
                definition.height,
            );
        }
        xml.end_element();
    }
}

fn unsupported_paint_reason(paint: &Paint) -> &'static str {
    match paint {
        Paint::Gradient(Gradient::Conic(_)) => "conic-gradient",
        Paint::Gradient(_) => "gradient-color-space",
        Paint::Tiling(_) => "tiling-content-or-size",
        Paint::Solid(_) => "none",
    }
}

fn paint_is_svg_native(paint: &Paint) -> bool {
    use typst_core::entities::color::ColorSpace;

    match paint {
        Paint::Solid(_) => true,
        Paint::Gradient(Gradient::Linear(value)) => matches!(
            value.space,
            ColorSpace::Srgb
                | ColorSpace::Oklab
                | ColorSpace::Oklch
                | ColorSpace::LinearRgb
                | ColorSpace::Hsl
                | ColorSpace::Hsv
        ),
        Paint::Gradient(Gradient::Radial(value)) => matches!(
            value.space,
            ColorSpace::Srgb
                | ColorSpace::Oklab
                | ColorSpace::Oklch
                | ColorSpace::LinearRgb
                | ColorSpace::Hsl
                | ColorSpace::Hsv
        ),
        Paint::Gradient(Gradient::Conic(value)) => matches!(
            value.space,
            ColorSpace::Srgb
                | ColorSpace::Oklab
                | ColorSpace::LinearRgb
                | ColorSpace::Hsv
        ),
        Paint::Tiling(tiling) => {
            tiling.size.is_some() && matches!(tiling.body, TilingBody::Color(_))
        }
    }
}

fn write_paint_def(
    xml: &mut XmlWriter,
    id: &str,
    paint: &Paint,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    match paint {
        Paint::Gradient(Gradient::Linear(value)) => {
            let angle = value.angle.to_rad();
            let dx = angle.cos() / 2.0;
            let dy = angle.sin() / 2.0;
            xml.start_element("linearGradient");
            xml.write_attribute("id", id);
            xml.write_attribute("gradientUnits", "objectBoundingBox");
            xml.write_attribute("x1", &fmt_num(0.5 - dx));
            xml.write_attribute("y1", &fmt_num(0.5 + dy));
            xml.write_attribute("x2", &fmt_num(0.5 + dx));
            xml.write_attribute("y2", &fmt_num(0.5 - dy));
            write_svg_gradient_stops(
                xml,
                paint,
                &value.stops,
                &value.effective_offsets(),
            );
            xml.end_element();
        }
        Paint::Gradient(Gradient::Radial(value)) => {
            xml.start_element("radialGradient");
            xml.write_attribute("id", id);
            xml.write_attribute("gradientUnits", "objectBoundingBox");
            xml.write_attribute("cx", &fmt_num(value.center.x.get()));
            xml.write_attribute("cy", &fmt_num(value.center.y.get()));
            xml.write_attribute("r", &fmt_num(value.radius.get()));
            xml.write_attribute("fx", &fmt_num(value.focal_center.x.get()));
            xml.write_attribute("fy", &fmt_num(value.focal_center.y.get()));
            xml.write_attribute("fr", &fmt_num(value.focal_radius.get()));
            write_svg_gradient_stops(
                xml,
                paint,
                &value.stops,
                &value.effective_offsets(),
            );
            xml.end_element();
        }
        Paint::Gradient(Gradient::Conic(value)) => {
            write_conic_paint_def(xml, id, value, x, y, width, height);
        }
        Paint::Tiling(tiling) => {
            let size = tiling.size.expect("paint_is_svg_native checked size");
            let (spacing_width, spacing_height) = tiling
                .spacing
                .map(|spacing| (spacing.width.0, spacing.height.0))
                .unwrap_or((0.0, 0.0));
            xml.start_element("pattern");
            xml.write_attribute("id", id);
            xml.write_attribute(
                "patternUnits",
                match tiling.relative {
                    TilingRelative::Auto | TilingRelative::Itself => "objectBoundingBox",
                    TilingRelative::Parent => "userSpaceOnUse",
                },
            );
            xml.write_attribute("width", &fmt_num(size.width.0 + spacing_width));
            xml.write_attribute("height", &fmt_num(size.height.0 + spacing_height));
            if let TilingBody::Color(color) = tiling.body {
                xml.start_element("rect");
                xml.write_attribute("width", &fmt_num(size.width.0));
                xml.write_attribute("height", &fmt_num(size.height.0));
                xml.write_attribute("fill", &color_to_css(color));
                xml.end_element();
            }
            xml.end_element();
        }
        Paint::Solid(_) => {}
    }
}

/// P1230: aproxima o campo angular por cunhas vetoriais. A cardinalidade
/// espelha a heurística ratificada, mas o contrato observa o budget, não 360.
fn write_conic_paint_def(
    xml: &mut XmlWriter,
    id: &str,
    conic: &typst_core::entities::gradient::Conic,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    const SEGMENTS: usize = 360;
    let cx = conic.center.x.get();
    let cy = conic.center.y.get();
    let pattern_cx = 0.5 * (cx + 0.5);
    let pattern_cy = 0.5 * (cy + 0.5);
    let aspect_ratio = width / height;
    let inverse_ratio = if aspect_ratio.is_finite() && aspect_ratio > 0.0 {
        aspect_ratio.recip()
    } else {
        1.0
    };
    let start = conic.angle.to_rad();
    let step = std::f64::consts::TAU / SEGMENTS as f64;

    let corrected = |angle: f64| {
        -(angle.sin() / inverse_ratio.abs())
            .atan2(angle.cos())
            .rem_euclid(std::f64::consts::TAU)
    };
    let sample = |t: f32| {
        if t == 0.0 {
            let offsets = conic.effective_offsets();
            let mut index = 0;
            while index + 1 < offsets.len() && offsets[index + 1] == 0.0 {
                index += 1;
            }
            conic
                .stops
                .get(index)
                .map(|stop| stop.color)
                .unwrap_or(Color::rgb(0, 0, 0))
        } else {
            conic.sample(t)
        }
    };

    xml.start_element("pattern");
    xml.write_attribute("id", &format!("{id}b"));
    xml.write_attribute("viewBox", "0 0 1 1");
    xml.write_attribute("preserveAspectRatio", "none");
    xml.write_attribute("patternUnits", "userSpaceOnUse");
    xml.write_attribute("width", "2");
    xml.write_attribute("height", "2");
    xml.write_attribute("x", "-0.5");
    xml.write_attribute("y", "-0.5");

    xml.start_element("defs");
    for index in 0..SEGMENTS {
        let theta1 = corrected(start + step * index as f64);
        let theta2 = corrected(start + step * (index + 1) as f64);
        let c0 = sample(index as f32 / SEGMENTS as f32);
        let c1 = sample((index + 1) as f32 / SEGMENTS as f32);
        xml.start_element("linearGradient");
        xml.write_attribute("id", &format!("{id}s{index}"));
        xml.write_attribute("gradientUnits", "objectBoundingBox");
        xml.write_attribute("x1", &fmt_num(2.0 - theta1.cos() + cx));
        xml.write_attribute("y1", &fmt_num(theta1.sin() + cy));
        xml.write_attribute("x2", &fmt_num(2.0 - theta2.cos() + cx));
        xml.write_attribute("y2", &fmt_num(theta2.sin() + cy));
        write_gradient_stops(
            xml,
            &[
                typst_core::entities::gradient::GradientStop::new(
                    c0,
                    typst_core::entities::layout_types::Ratio(0.0),
                ),
                typst_core::entities::gradient::GradientStop::new(
                    c1,
                    typst_core::entities::layout_types::Ratio(1.0),
                ),
            ],
            &[0.0, 1.0],
        );
        xml.end_element();
    }
    xml.end_element();

    for index in 0..SEGMENTS {
        let theta1 = corrected(start + step * index as f64);
        let theta2 = corrected(start + step * (index + 1) as f64);
        // O pattern ocupa a caixa unitária. Raio 1 cobre toda a caixa e
        // espelha `correct_tiling_pos`, que reduz o raio 2 do domínio Typst
        // pela metade ao escrevê-lo no viewBox do SVG.
        let x1 = pattern_cx - theta1.cos();
        let y1 = pattern_cy + theta1.sin();
        let x2 = pattern_cx - theta2.cos();
        let y2 = pattern_cy + theta2.sin();
        // O builder vanilla calcula mecanicamente os raios relativos ao
        // ponto corrente. Preservar essa geometria evita uma borda diferente
        // nos renderizadores SVG, embora a grafia do path seja livre.
        let rx = 1.0 - x1;
        let ry = 1.0 - y1;
        let d = format!(
            "M {} {} L {} {} A {} {} 0 0 1 {} {} Z",
            fmt_num(pattern_cx),
            fmt_num(pattern_cy),
            fmt_num(x1),
            fmt_num(y1),
            fmt_num(rx),
            fmt_num(ry),
            fmt_num(x2),
            fmt_num(y2),
        );
        xml.start_element("path");
        xml.write_attribute("d", &d);
        xml.write_attribute("fill", &format!("url(#{id}s{index})"));
        xml.write_attribute("stroke", "none");
        xml.write_attribute("shape-rendering", "optimizeSpeed");
        xml.end_element();
    }
    xml.end_element();

    xml.start_element("pattern");
    xml.write_attribute("id", id);
    xml.write_attribute(
        "patternTransform",
        &format!(
            "translate({} {}) scale({} {})",
            fmt_num(x),
            fmt_num(y),
            fmt_num(width),
            fmt_num(height)
        ),
    );
    xml.write_attribute("href", &format!("#{id}b"));
    xml.write_attribute("xlink:href", &format!("#{id}b"));
    xml.end_element();
}

fn write_svg_gradient_stops(
    xml: &mut XmlWriter,
    paint: &Paint,
    native_stops: &[typst_core::entities::gradient::GradientStop],
    native_offsets: &[f32],
) {
    use typst_core::entities::color::ColorSpace;
    let gradient = match paint {
        Paint::Gradient(gradient) => gradient,
        _ => return write_gradient_stops(xml, native_stops, native_offsets),
    };
    let is_srgb = match gradient {
        Gradient::Linear(value) => value.space == ColorSpace::Srgb,
        Gradient::Radial(value) => value.space == ColorSpace::Srgb,
        Gradient::Conic(_) => false,
    };
    if is_srgb {
        write_gradient_stops(xml, native_stops, native_offsets);
    } else {
        let sampled = svg_adaptive_stops(gradient);
        let offsets: Vec<f64> = sampled
            .iter()
            .map(|stop| stop.offset.expect("P1229 sampled stop has offset").0)
            .collect();
        let repr_all_offsets = adaptive_offsets_use_ratio_repr(gradient);
        write_adaptive_gradient_stops(xml, &sampled, &offsets, repr_all_offsets);
    }
}

fn adaptive_offsets_use_ratio_repr(gradient: &Gradient) -> bool {
    use typst_core::entities::color::ColorSpace;

    match gradient {
        Gradient::Linear(value) => matches!(
            value.space,
            ColorSpace::Oklab
                | ColorSpace::LinearRgb
                | ColorSpace::Oklch
                | ColorSpace::Hsl
                | ColorSpace::Hsv
        ),
        Gradient::Radial(value) => matches!(
            value.space,
            ColorSpace::Oklab
                | ColorSpace::LinearRgb
                | ColorSpace::Oklch
                | ColorSpace::Hsl
                | ColorSpace::Hsv
        ),
        Gradient::Conic(_) => false,
    }
}

fn write_adaptive_gradient_stops(
    xml: &mut XmlWriter,
    stops: &[typst_core::entities::gradient::GradientStop],
    offsets: &[f64],
    repr_all_offsets: bool,
) {
    let last_interval = stops.len().saturating_sub(2);
    for (index, (stop, offset)) in stops.iter().zip(offsets).enumerate() {
        let serialized_offset = if repr_all_offsets {
            fmt_ratio_repr(*offset)
        } else if index >= last_interval {
            fmt_ratio_repr_p1261(*offset as f32)
        } else {
            fmt_num(f64::from(*offset as f32))
        };
        write_gradient_stop(xml, stop, &serialized_offset);
    }
}

fn write_gradient_stops(
    xml: &mut XmlWriter,
    stops: &[typst_core::entities::gradient::GradientStop],
    offsets: &[f32],
) {
    for (stop, offset) in stops.iter().zip(offsets) {
        write_gradient_stop(xml, stop, &fmt_num(f64::from(*offset)));
    }
}

fn write_gradient_stop(
    xml: &mut XmlWriter,
    stop: &typst_core::entities::gradient::GradientStop,
    offset: &str,
) {
    let (r, g, b, alpha) = stop.color.to_srgb();
    xml.start_element("stop");
    xml.write_attribute("offset", offset);
    xml.write_attribute("stop-color", &format!("#{r:02x}{g:02x}{b:02x}"));
    if alpha != 255 {
        xml.write_attribute("stop-opacity", &fmt_num(f64::from(alpha) / 255.0));
    }
    xml.end_element();
}

fn fmt_ratio_repr(offset: f64) -> String {
    // `Ratio::repr` converte primeiro a razão para percentagem e só depois
    // arredonda essa percentagem a duas casas. A ordem é observável nos
    // valores binários junto de meia unidade.
    let percent = ((offset * 100.0) * 100.0).round() / 100.0;
    format!("{}%", fmt_num(percent))
}

fn fmt_ratio_repr_p1261(offset: f32) -> String {
    let percent = (f64::from(offset) * 10_000.0).round_ties_even() / 100.0;
    format!("{}%", fmt_num(percent))
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
        let key = (font_idx, glyph_id, scale.to_bits());
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
    )
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
    export_svg_with_fonts_inner(page, opts, Some(fonts), destinations, glyph_fonts)
}

fn export_svg_with_fonts_inner(
    page: &Page,
    opts: &SvgOptions,
    fonts: Option<&[FontKey]>,
    destinations: &SvgDestinationContext,
    glyph_fonts: &SvgGlyphFontContext,
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
            );
        }
        FrameItem::Semantic { items, .. } => {
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
                );
            }
        }
    }
}

fn write_glyph_fallback(xml: &mut XmlWriter, pos: &Point, reason: &'static str) {
    xml.start_element("g");
    xml.write_attribute("data-crystalline-glyph-fallback", reason);
    xml.write_attribute(
        "transform",
        &format!("translate({} {})", fmt_num(pos.x.0), fmt_num(pos.y.0)),
    );
    xml.end_element();
}

#[allow(clippy::too_many_arguments)]
fn render_direct_glyph(
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
    let Some((font_idx, (_, bytes))) = fonts
        .iter()
        .enumerate()
        .find(|(_, (candidate, _))| candidate == identity)
    else {
        write_glyph_fallback(xml, pos, "mapped-font-missing");
        return;
    };
    let Ok(face) = Face::parse(bytes, 0) else {
        write_glyph_fallback(xml, pos, "invalid-font");
        return;
    };
    let units_per_em = face.units_per_em();
    if units_per_em == 0 {
        write_glyph_fallback(xml, pos, "invalid-scale");
        return;
    }
    let scale = size.0 / f64::from(units_per_em);
    let Some(id) = glyph_defs.get_or_insert(font_idx, glyph_id, bytes, scale) else {
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
        &style.fill.map(color_to_css).unwrap_or_else(|| "#000000".to_string()),
    );
    xml.end_element();
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

fn render_shape(
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
            let d = path_items_to_svg(items, pos);
            xml.start_element("path");
            xml.write_attribute("d", &d);
            if fill_rule == typst_core::entities::geometry::FillRule::EvenOdd {
                xml.write_attribute("fill-rule", "evenodd");
            }
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

fn render_group(
    xml: &mut XmlWriter,
    pos: &Point,
    matrix: &TransformMatrix,
    clip_mask: Option<&ShapeKind<typst_core::entities::layout_types::Pt>>,
    inner_width: f64,
    inner_height: f64,
    items: &[FrameItem],
    fonts: Option<&[FontKey]>,
    glyph_defs: &mut GlyphDefs,
    paint_defs: &mut PaintDefs,
    clip_next_id: &mut usize,
    destinations: &SvgDestinationContext,
    glyph_fonts: &SvgGlyphFontContext,
) {
    let clip = clip_mask.map(|kind| {
        write_clip_definition(xml, kind, inner_width, inner_height, clip_next_id)
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
        render_item(
            xml,
            item,
            fonts,
            glyph_defs,
            paint_defs,
            clip_next_id,
            destinations,
            glyph_fonts,
        );
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

fn render_link(
    xml: &mut XmlWriter,
    target: &LinkTarget,
    items: &[FrameItem],
    pos: &Point,
    size: &Size,
    fonts: Option<&[FontKey]>,
    glyph_defs: &mut GlyphDefs,
    paint_defs: &mut PaintDefs,
    clip_next_id: &mut usize,
    destinations: &SvgDestinationContext,
    glyph_fonts: &SvgGlyphFontContext,
) {
    xml.start_element("a");
    match target {
        LinkTarget::Url(url) => {
            xml.write_attribute("xlink:href", url.as_str());
        }
        LinkTarget::Destination(label) => {
            if let Some(id) = destinations.id_for(label) {
                xml.write_attribute("xlink:href", &format!("#{id}"));
            } else {
                xml.write_attribute(
                    "data-crystalline-link-fallback",
                    "destination-unknown",
                );
            }
        }
    }
    xml.start_element("rect");
    xml.write_attribute("x", &fmt_num(pos.x.0));
    xml.write_attribute("y", &fmt_num(pos.y.0));
    xml.write_attribute("width", &fmt_num(size.width.0));
    xml.write_attribute("height", &fmt_num(size.height.0));
    xml.write_attribute("fill", "transparent");
    xml.write_attribute("data-crystalline-link-hit-area", "true");
    xml.end_element();
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
        );
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

    fn p1227_page(items: Vec<FrameItem>) -> Page {
        Page {
            width: 100.0,
            height: 100.0,
            numbering: None,
            supplement: typst_core::entities::content::Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items,
        }
    }

    fn p1227_rect(fill: Option<Paint>, stroke: Option<Stroke>) -> FrameItem {
        FrameItem::Shape {
            pos: Point { x: Pt(5.0), y: Pt(7.0) },
            kind: ShapeKind::Rect,
            width: 40.0,
            height: 20.0,
            fill,
            stroke,
            fill_rule: Default::default(),
            parent_bbox_at_emit: None,
        }
    }
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
                fill: Some(Paint::Solid(Color::rgb(255, 0, 0))),
                stroke: None,
                fill_rule: Default::default(),
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
            0.0,
        );
        assert!(d.starts_with("M 12 20 L 106 20"));
        assert!(d.contains("L 110 94"));
        assert!(d.contains("L 18 100"));
        assert!(d.contains("L 10 22"));
    }

    #[test]
    fn p1222_rounded_rect_inseta_centro_do_stroke_e_separa_paints() {
        use typst_core::entities::geometry::Stroke;
        use typst_core::entities::paint::Paint;
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
                pos: Point { x: Pt(10.0), y: Pt(20.0) },
                kind: ShapeKind::RoundedRect { radii: Corners::uniform(Pt(4.0)) },
                width: 30.0,
                height: 14.0,
                fill: Some(Paint::Solid(Color::rgb(255, 65, 54))),
                stroke: Some(Stroke {
                    paint: Paint::Solid(Color::rgb(0, 116, 217)),
                    thickness: 2.0,
                    overhang: true,
                    ..Stroke::default()
                }),
                fill_rule: Default::default(),
                parent_bbox_at_emit: None,
            }],
        };
        let svg = export_svg(&page, &SvgOptions::default());
        assert_eq!(
            svg.matches("<path").count(),
            2,
            "fill e stroke devem ser operações separadas"
        );
        assert!(svg.contains("M 13 20"), "raio central esperado: 4pt - 1pt");
        assert!(svg.contains("fill=\"#ff4136\""));
        assert!(svg.contains("fill=\"none\" stroke=\"#0074d9\" stroke-width=\"2\""));
    }

    #[test]
    fn p1224_svg_emite_stroke_complexo_sem_perder_ordem_ou_phase() {
        use typst_core::entities::geometry::{
            DashLength, DashPattern, LineCap, LineJoin, Stroke,
        };
        use typst_core::entities::paint::Paint;
        let stroke = Stroke {
            paint: Paint::Solid(Color::rgba(255, 65, 54, 128)),
            thickness: 2.0,
            cap: LineCap::Round,
            join: LineJoin::Bevel,
            dash: Some(DashPattern {
                array: vec![DashLength::Length(3.0), DashLength::LineWidth],
                phase: -0.5,
            }),
            miter_limit: 2.0,
            overhang: true,
            specified: Default::default(),
        };
        let page = Page {
            width: 20.0,
            height: 20.0,
            numbering: None,
            supplement: typst_core::entities::content::Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items: vec![FrameItem::Shape {
                pos: Point { x: Pt(1.0), y: Pt(2.0) },
                kind: ShapeKind::Line { dx: 10.0, dy: 3.0 },
                width: 10.0,
                height: 3.0,
                fill: None,
                stroke: Some(stroke),
                fill_rule: Default::default(),
                parent_bbox_at_emit: None,
            }],
        };
        let svg = export_svg(&page, &SvgOptions::default());
        assert!(svg.contains("stroke=\"rgba(255,65,54,0.502)\""));
        assert!(svg.contains("stroke-linecap=\"round\""));
        assert!(svg.contains("stroke-linejoin=\"bevel\""));
        assert!(svg.contains("stroke-miterlimit=\"2\""));
        assert!(svg.contains("stroke-dasharray=\"3 2\""));
        assert!(svg.contains("stroke-dashoffset=\"-0.5\""));
    }

    #[test]
    fn p1227_linear_fill_emite_servidor_e_alpha_separado() {
        use typst_core::entities::color::ColorSpace;
        use typst_core::entities::gradient::{Gradient, GradientStop};
        use typst_core::entities::layout_types::{Angle, Ratio};
        let gradient = Gradient::linear_with_space(
            vec![
                GradientStop::new(Color::rgba(255, 0, 0, 128), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            Angle::deg(0.0),
            ColorSpace::Srgb,
        );
        let svg = export_svg(
            &p1227_page(vec![p1227_rect(Some(Paint::Gradient(gradient)), None)]),
            &SvgOptions::default(),
        );
        assert!(svg.contains("fill=\"url(#p0)\""));
        assert!(svg.contains("<linearGradient id=\"p0\""));
        assert!(svg.contains("stop-color=\"#ff0000\" stop-opacity=\"0.501960784"));
        assert!(svg.contains("offset=\"0\""));
        assert!(svg.contains("offset=\"1\""));
    }

    #[test]
    fn p1261_endpoint_near_serializa_penultimo_offset_com_repr_vanilla() {
        use typst_core::entities::axes::Axes;
        use typst_core::entities::color::ColorSpace;
        use typst_core::entities::gradient::{Gradient, GradientStop};
        use typst_core::entities::layout_types::{Angle, Ratio};

        let stops = || {
            vec![
                GradientStop::new(Color::rgb(255, 65, 54), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 116, 217), Ratio(1.0)),
            ]
        };
        for (kind, space, expected_count, expected_penultimate) in [
            ("linear", ColorSpace::Oklab, 25, "#1674d7"),
            ("radial", ColorSpace::Oklab, 25, "#1674d7"),
            ("linear", ColorSpace::LinearRgb, 29, "#2273d8"),
            ("radial", ColorSpace::LinearRgb, 29, "#2273d8"),
        ] {
            let gradient = if kind == "linear" {
                Gradient::linear_with_space(stops(), Angle::deg(0.0), space)
            } else {
                Gradient::radial_with_space(
                    stops(),
                    Axes::new(Ratio(0.5), Ratio(0.5)),
                    Ratio(0.5),
                    space,
                )
            };
            let sampled = svg_adaptive_stops(&gradient);
            assert_eq!(sampled.len(), expected_count, "{kind}/{space:?}");
            let penultimate = sampled[sampled.len() - 2];
            assert_eq!(penultimate.offset, Some(Ratio(0.984375)), "{kind}/{space:?}");
            assert_eq!(sampled.last().unwrap().offset, Some(Ratio(1.0)));
            assert_eq!(sampled.last().unwrap().color, Color::rgb(0, 116, 217));

            let paint = Paint::Gradient(gradient);
            let (native_stops, native_offsets) = match &paint {
                Paint::Gradient(Gradient::Linear(value)) => {
                    (&*value.stops, value.effective_offsets())
                }
                Paint::Gradient(Gradient::Radial(value)) => {
                    (&*value.stops, value.effective_offsets())
                }
                Paint::Gradient(Gradient::Conic(_)) => unreachable!(),
                _ => unreachable!(),
            };
            let mut xml = XmlWriter::new(xmlwriter::Options::default());
            xml.start_element("gradient");
            write_svg_gradient_stops(&mut xml, &paint, native_stops, &native_offsets);
            let serialized = xml.end_document();
            assert_eq!(serialized.matches("<stop ").count(), expected_count);
            assert!(
                serialized.contains(&format!(
                    "offset=\"98.44%\" stop-color=\"{expected_penultimate}\""
                )),
                "penultimo stop divergente em {kind}/{space:?}: {serialized}"
            );
            assert_eq!(serialized.matches("offset=\"100%\"").count(), 1);
            assert!(serialized.contains("offset=\"100%\" stop-color=\"#0074d9\""));
        }
    }

    #[test]
    fn p1262_ratio_repr_preserva_ordem_aritmetica_vanilla() {
        let first_interval = |position: f64| 0.0 + (0.37 - 0.0) * position / 8.0;
        let second_interval = |position: f64| 0.37 + (1.0 - 0.37) * position / 8.0;
        assert_eq!(fmt_ratio_repr(first_interval(3.0)), "13.87%");
        assert_eq!(fmt_ratio_repr(first_interval(7.0)), "32.38%");
        assert_eq!(fmt_ratio_repr(second_interval(3.0)), "60.62%");
        assert_eq!(fmt_ratio_repr(second_interval(5.0)), "76.38%");
    }

    #[test]
    fn p1263_linear_rgb_serializa_todos_offsets_adaptativos_com_repr_vanilla() {
        use typst_core::entities::axes::Axes;
        use typst_core::entities::color::ColorSpace;
        use typst_core::entities::gradient::{Gradient, GradientStop};
        use typst_core::entities::layout_types::{Angle, Ratio};

        let stops = || {
            vec![
                GradientStop::new(Color::rgb(255, 65, 54), Ratio(0.0)),
                GradientStop::new(Color::rgba(46, 204, 64, 102), Ratio(0.37)),
                GradientStop::new(Color::rgb(0, 116, 217), Ratio(1.0)),
            ]
        };

        for kind in ["linear", "radial"] {
            let gradient = if kind == "linear" {
                Gradient::linear_with_space(
                    stops(),
                    Angle::deg(25.0),
                    ColorSpace::LinearRgb,
                )
            } else {
                Gradient::radial_with_space(
                    stops(),
                    Axes::new(Ratio(0.5), Ratio(0.5)),
                    Ratio(0.5),
                    ColorSpace::LinearRgb,
                )
            };
            let sampled = svg_adaptive_stops(&gradient);
            assert_eq!(sampled.len(), 36, "{kind}");

            let paint = Paint::Gradient(gradient);
            let (native_stops, native_offsets) = match &paint {
                Paint::Gradient(Gradient::Linear(value)) => {
                    (&*value.stops, value.effective_offsets())
                }
                Paint::Gradient(Gradient::Radial(value)) => {
                    (&*value.stops, value.effective_offsets())
                }
                _ => unreachable!(),
            };
            let mut xml = XmlWriter::new(xmlwriter::Options::default());
            xml.start_element("gradient");
            write_svg_gradient_stops(&mut xml, &paint, native_stops, &native_offsets);
            let serialized = xml.end_document();
            assert_eq!(serialized.matches("<stop ").count(), 36, "{kind}");
            assert!(
                serialized.contains("offset=\"1.16%\""),
                "offset Ratio::repr ausente em {kind}: {serialized}"
            );
            assert!(
                !serialized.contains("offset=\"0.0115625\""),
                "offset cru sobreviveu em {kind}: {serialized}"
            );
        }
    }

    #[test]
    fn p1278_offsets_adaptativos_polares_usam_ratio_repr_completo() {
        use typst_core::entities::color::ColorSpace;
        use typst_core::entities::gradient::{Gradient, GradientStop};
        use typst_core::entities::layout_types::{Angle, Ratio};

        let stops = || {
            vec![
                GradientStop::new(Color::rgb(255, 65, 54), Ratio(0.0)),
                GradientStop::new(Color::rgb(46, 204, 64), Ratio(0.37)),
                GradientStop::new(Color::rgb(0, 116, 217), Ratio(1.0)),
            ]
        };
        for space in [ColorSpace::Oklch, ColorSpace::Hsl, ColorSpace::Hsv] {
            let gradient = Gradient::linear_with_space(stops(), Angle::deg(25.0), space);
            assert!(
                adaptive_offsets_use_ratio_repr(&gradient),
                "Linear/{space:?} deve serializar todos os offsets via Ratio::repr"
            );
        }

        let luma =
            Gradient::linear_with_space(stops(), Angle::deg(25.0), ColorSpace::Luma);
        assert!(!adaptive_offsets_use_ratio_repr(&luma));
    }

    #[test]
    fn p1274_linear_oklab_usa_servidor_adaptativo_nativo() {
        use typst_core::entities::gradient::{Gradient, GradientStop};
        use typst_core::entities::layout_types::{Angle, Ratio};
        let gradient = Gradient::linear(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            Angle::deg(25.0),
        );
        let svg = export_svg(
            &p1227_page(vec![p1227_rect(Some(Paint::Gradient(gradient)), None)]),
            &SvgOptions::default(),
        );
        assert!(svg.contains("fill=\"url(#p0)\""));
        assert!(svg.contains("<linearGradient id=\"p0\""));
        assert!(!svg.contains("data-crystalline-fill-fallback"));
        assert!(svg.matches("<stop ").count() > 2);
    }

    #[test]
    fn p1274_linear_linear_rgb_usa_servidor_adaptativo_nativo() {
        use typst_core::entities::color::ColorSpace;
        use typst_core::entities::gradient::{Gradient, GradientStop};
        use typst_core::entities::layout_types::{Angle, Ratio};
        let gradient = Gradient::linear_with_space(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            Angle::deg(25.0),
            ColorSpace::LinearRgb,
        );
        let svg = export_svg(
            &p1227_page(vec![p1227_rect(Some(Paint::Gradient(gradient)), None)]),
            &SvgOptions::default(),
        );
        assert!(svg.contains("fill=\"url(#p0)\""));
        assert!(svg.contains("<linearGradient id=\"p0\""));
        assert!(!svg.contains("data-crystalline-fill-fallback"));
        assert!(svg.matches("<stop ").count() > 2);
    }

    #[test]
    fn p1280_seis_pares_polares_usam_servidor_adaptativo_em_fill_e_stroke() {
        use typst_core::entities::axes::Axes;
        use typst_core::entities::color::ColorSpace;
        use typst_core::entities::gradient::{Gradient, GradientStop};
        use typst_core::entities::layout_types::{Angle, Ratio};

        let stops = || {
            vec![
                GradientStop::new(Color::rgba(255, 0, 0, 128), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 255, 0), Ratio(0.37)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]
        };
        for space in [ColorSpace::Oklch, ColorSpace::Hsl, ColorSpace::Hsv] {
            let gradients = [
                (
                    "linear",
                    "linearGradient",
                    Gradient::linear_with_space(stops(), Angle::deg(25.0), space),
                ),
                (
                    "radial",
                    "radialGradient",
                    Gradient::radial_with_space(
                        stops(),
                        Axes::new(Ratio(0.4), Ratio(0.6)),
                        Ratio(0.7),
                        space,
                    ),
                ),
            ];
            for (kind, variant, gradient) in gradients {
                let fill_svg = export_svg(
                    &p1227_page(vec![p1227_rect(
                        Some(Paint::Gradient(gradient.clone())),
                        None,
                    )]),
                    &SvgOptions::default(),
                );
                assert!(fill_svg.contains("fill=\"url(#p0)\""), "{kind}/{space:?}");
                assert!(
                    fill_svg.contains(&format!("<{variant} id=\"p0\"")),
                    "{kind}/{space:?}: {fill_svg}"
                );
                assert!(
                    !fill_svg.contains("data-crystalline-fill-fallback"),
                    "{kind}/{space:?}"
                );
                assert!(fill_svg.matches("<stop ").count() > 3, "{kind}/{space:?}");
                assert!(fill_svg.contains("stop-opacity"), "{kind}/{space:?}");

                let stroke = Stroke {
                    paint: Paint::Gradient(gradient),
                    thickness: 2.0,
                    ..Stroke::default()
                };
                let stroke_svg = export_svg(
                    &p1227_page(vec![p1227_rect(None, Some(stroke))]),
                    &SvgOptions::default(),
                );
                assert!(stroke_svg.contains("stroke=\"url(#p0)\""), "{kind}/{space:?}");
                assert!(
                    stroke_svg.contains(&format!("<{variant} id=\"p0\"")),
                    "{kind}/{space:?}: {stroke_svg}"
                );
                assert!(
                    !stroke_svg.contains("data-crystalline-stroke-fallback"),
                    "{kind}/{space:?}"
                );
                assert!(stroke_svg.matches("<stop ").count() > 3, "{kind}/{space:?}");
                assert!(stroke_svg.contains("stop-opacity"), "{kind}/{space:?}");
            }
        }
    }

    #[test]
    fn p1274_radial_oklab_usa_servidor_adaptativo_nativo() {
        use typst_core::entities::axes::Axes;
        use typst_core::entities::gradient::{Gradient, GradientStop};
        use typst_core::entities::layout_types::Ratio;
        let gradient = Gradient::radial(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
        );
        let svg = export_svg(
            &p1227_page(vec![p1227_rect(Some(Paint::Gradient(gradient)), None)]),
            &SvgOptions::default(),
        );
        assert!(svg.contains("fill=\"url(#p0)\""));
        assert!(svg.contains("<radialGradient id=\"p0\""));
        assert!(!svg.contains("data-crystalline-fill-fallback"));
        assert!(svg.matches("<stop ").count() > 2);
    }

    #[test]
    fn p1274_radial_linear_rgb_usa_servidor_adaptativo_em_stroke() {
        use typst_core::entities::axes::Axes;
        use typst_core::entities::color::ColorSpace;
        use typst_core::entities::gradient::{Gradient, GradientStop};
        use typst_core::entities::layout_types::Ratio;
        let gradient = Gradient::radial_with_space(
            vec![
                GradientStop::new(Color::rgba(255, 0, 0, 128), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Ratio(0.5),
            ColorSpace::LinearRgb,
        );
        let stroke = Stroke {
            paint: Paint::Gradient(gradient),
            thickness: 2.0,
            ..Stroke::default()
        };
        let svg = export_svg(
            &p1227_page(vec![p1227_rect(None, Some(stroke))]),
            &SvgOptions::default(),
        );
        assert!(svg.contains("stroke=\"url(#p0)\""));
        assert!(svg.contains("<radialGradient id=\"p0\""));
        assert!(!svg.contains("data-crystalline-stroke-fallback"));
        assert!(svg.matches("<stop ").count() > 2);
    }

    #[test]
    fn p1280_luma_linear_radial_permanece_unknown() {
        use typst_core::entities::axes::Axes;
        use typst_core::entities::color::ColorSpace;
        use typst_core::entities::gradient::{Gradient, GradientStop};
        use typst_core::entities::layout_types::{Angle, Ratio};
        let stops = || {
            vec![
                GradientStop::new(Color::rgba(255, 0, 0, 128), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 255, 0), Ratio(0.37)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ]
        };
        for linear in [true, false] {
            let space = ColorSpace::Luma;
            let gradient = if linear {
                Gradient::linear_with_space(stops(), Angle::deg(25.0), space)
            } else {
                let mut gradient = Gradient::radial(
                    stops(),
                    Axes::new(Ratio(0.4), Ratio(0.6)),
                    Ratio(0.7),
                );
                if let Gradient::Radial(value) = &mut gradient {
                    Arc::make_mut(value).space = space;
                }
                gradient
            };
            let svg = export_svg(
                &p1227_page(vec![p1227_rect(Some(Paint::Gradient(gradient)), None)]),
                &SvgOptions::default(),
            );
            assert!(
                svg.contains("data-crystalline-fill-fallback=\"gradient-color-space\""),
                "missing explicit fallback for {space:?}"
            );
            assert!(!svg.contains("<linearGradient"));
            assert!(!svg.contains("<radialGradient"));
        }
    }

    #[test]
    fn p1231_linear_radial_cmyk_continuam_unknown() {
        use typst_core::entities::axes::Axes;
        use typst_core::entities::color::ColorSpace;
        use typst_core::entities::gradient::{Gradient, GradientStop};
        use typst_core::entities::layout_types::{Angle, Ratio};
        let stops = || {
            vec![
                GradientStop::new(Color::cmyk(0.0, 1.0, 1.0, 0.0), Ratio(0.0)),
                GradientStop::new(Color::cmyk(1.0, 1.0, 0.0, 0.0), Ratio(1.0)),
            ]
        };
        let linear =
            Gradient::linear_with_space(stops(), Angle::deg(0.0), ColorSpace::Cmyk);
        let mut radial =
            Gradient::radial(stops(), Axes::new(Ratio(0.5), Ratio(0.5)), Ratio(0.5));
        if let Gradient::Radial(value) = &mut radial {
            Arc::make_mut(value).space = ColorSpace::Cmyk;
        }
        for gradient in [linear, radial] {
            let svg = export_svg(
                &p1227_page(vec![p1227_rect(Some(Paint::Gradient(gradient)), None)]),
                &SvgOptions::default(),
            );
            assert!(
                svg.contains("data-crystalline-fill-fallback=\"gradient-color-space\"")
            );
            assert!(!svg.contains("<linearGradient"));
            assert!(!svg.contains("<radialGradient"));
        }
    }

    #[test]
    fn p1227_radial_preserva_centro_foco_raio_e_stroke() {
        use typst_core::entities::axes::Axes;
        use typst_core::entities::color::ColorSpace;
        use typst_core::entities::gradient::{Gradient, GradientStop};
        use typst_core::entities::layout_types::Ratio;
        let mut gradient = Gradient::radial_with_focal(
            vec![
                GradientStop::new(Color::rgb(255, 255, 255), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 0), Ratio(1.0)),
            ],
            Axes::new(Ratio(0.4), Ratio(0.6)),
            Ratio(0.7),
            Axes::new(Ratio(0.2), Ratio(0.3)),
            Ratio(0.1),
        );
        if let Gradient::Radial(value) = &mut gradient {
            Arc::make_mut(value).space = ColorSpace::Srgb;
        }
        let stroke = Stroke {
            paint: Paint::Gradient(gradient),
            thickness: 2.0,
            ..Stroke::default()
        };
        let svg = export_svg(
            &p1227_page(vec![p1227_rect(None, Some(stroke))]),
            &SvgOptions::default(),
        );
        assert!(svg.contains("stroke=\"url(#p0)\""));
        assert!(svg.contains("<radialGradient id=\"p0\""));
        assert!(svg.contains("cx=\"0.4\" cy=\"0.6\" r=\"0.7\""));
        assert!(svg.contains("fx=\"0.2\" fy=\"0.3\" fr=\"0.1\""));
    }

    #[test]
    fn p1227_reutiliza_paint_morfologicamente_igual() {
        use typst_core::entities::color::ColorSpace;
        use typst_core::entities::gradient::{Gradient, GradientStop};
        use typst_core::entities::layout_types::{Angle, Ratio};
        let paint = Paint::Gradient(Gradient::linear_with_space(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            Angle::deg(90.0),
            ColorSpace::Srgb,
        ));
        let svg = export_svg(
            &p1227_page(vec![
                p1227_rect(Some(paint.clone()), None),
                p1227_rect(Some(paint), None),
            ]),
            &SvgOptions::default(),
        );
        assert_eq!(svg.matches("<linearGradient").count(), 1);
        assert_eq!(svg.matches("fill=\"url(#p0)\"").count(), 2);
    }

    #[test]
    fn p1227_tiling_de_cor_emite_pattern_com_tamanho_e_spacing() {
        use typst_core::entities::tiling::{Tiling, TilingBody};
        let mut tiling = Tiling::new(TilingBody::Color(Color::rgb(0, 128, 255)));
        tiling.size = Some(Size { width: Pt(4.0), height: Pt(6.0) });
        tiling.spacing = Some(Size { width: Pt(1.0), height: Pt(2.0) });
        let svg = export_svg(
            &p1227_page(vec![p1227_rect(Some(Paint::Tiling(tiling)), None)]),
            &SvgOptions::default(),
        );
        assert!(svg.contains("fill=\"url(#p0)\""));
        assert!(svg.contains("<pattern id=\"p0\""));
        assert!(svg.contains("width=\"5\" height=\"8\""));
        assert!(svg.contains("width=\"4\" height=\"6\" fill=\"#0080ff\""));
    }

    #[test]
    fn p1230_conic_oklab_emite_pattern_vetorial_fechado() {
        use typst_core::entities::axes::Axes;
        use typst_core::entities::gradient::{Gradient, GradientStop};
        use typst_core::entities::layout_types::{Angle, Ratio};
        let conic = Paint::Gradient(Gradient::conic(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Angle::deg(0.0),
        ));
        let svg = export_svg(
            &p1227_page(vec![p1227_rect(Some(conic), None)]),
            &SvgOptions::default(),
        );
        assert!(svg.contains("fill=\"url(#p0)\""));
        assert!(svg.contains("<pattern id=\"p0\""));
        assert!(svg.contains("<linearGradient id=\"p0s0\""));
        assert!(svg.contains("fill=\"url(#p0s0)\""));
        assert!(!svg.contains("data-crystalline-fill-fallback"));
        assert!(!svg.contains("<radialGradient"));
    }

    #[test]
    fn p1230_conic_cmyk_permanece_unknown_por_adr0097() {
        use typst_core::entities::axes::Axes;
        use typst_core::entities::color::ColorSpace;
        use typst_core::entities::gradient::{Gradient, GradientStop};
        use typst_core::entities::layout_types::{Angle, Ratio};
        let conic = Paint::Gradient(Gradient::conic_with_space(
            vec![
                GradientStop::new(Color::cmyk(0.0, 1.0, 1.0, 0.0), Ratio(0.0)),
                GradientStop::new(Color::cmyk(1.0, 1.0, 0.0, 0.0), Ratio(1.0)),
            ],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Angle::deg(0.0),
            ColorSpace::Cmyk,
        ));
        let svg = export_svg(
            &p1227_page(vec![p1227_rect(Some(conic), None)]),
            &SvgOptions::default(),
        );
        assert!(svg.contains("data-crystalline-fill-fallback=\"conic-gradient\""));
        assert!(!svg.contains("<pattern id=\"p0\""));
    }

    #[test]
    fn p1230_conic_stroke_usa_caixa_pintada() {
        use typst_core::entities::axes::Axes;
        use typst_core::entities::gradient::{Gradient, GradientStop};
        use typst_core::entities::layout_types::{Angle, Ratio};
        let paint = Paint::Gradient(Gradient::conic(
            vec![
                GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
                GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
            ],
            Axes::new(Ratio(0.5), Ratio(0.5)),
            Angle::deg(0.0),
        ));
        let stroke = Stroke { paint, thickness: 8.0, ..Stroke::default() };
        let svg = export_svg(
            &p1227_page(vec![p1227_rect(None, Some(stroke))]),
            &SvgOptions::default(),
        );
        assert!(svg.contains("patternTransform=\"translate(1 3) scale(48 28)\""));
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

    fn p1246_group(kind: ShapeKind<Pt>, width: f64, height: f64) -> Page {
        p1227_page(vec![FrameItem::Group {
            pos: Point { x: Pt(10.0), y: Pt(20.0) },
            matrix: TransformMatrix { a: 2.0, b: 0.0, c: 0.0, d: 1.0, tx: 0.0, ty: 0.0 },
            clip_mask: Some(kind),
            inner_width: width,
            inner_height: height,
            items: vec![p1227_rect(Some(Paint::Solid(Color::rgb(255, 0, 0))), None)],
        }])
    }

    #[test]
    fn p1246_rect_clip_fecha_referencia_no_espaco_local() {
        let svg =
            export_svg(&p1246_group(ShapeKind::Rect, 8.0, 6.0), &SvgOptions::default());
        assert!(svg.contains("<clipPath id=\"c0\""));
        assert!(svg.contains("<rect x=\"0\" y=\"0\" width=\"8\" height=\"6\""));
        assert!(svg.contains("clip-path=\"url(#c0)\""));
        assert_eq!(svg.matches("url(#c0)").count(), 1);
        assert!(svg.contains("translate(10 20) matrix(2 0 0 1 0 0)"));
    }

    #[test]
    fn p1246_rounded_clip_preserva_quatro_raios() {
        let svg = export_svg(
            &p1246_group(
                ShapeKind::RoundedRect {
                    radii: Corners::new(Pt(5.0), Pt(3.0), Pt(2.0), Pt(1.0)),
                },
                20.0,
                12.0,
            ),
            &SvgOptions::default(),
        );
        assert!(svg.contains("M 5 0 L 17 0"));
        assert!(svg.contains("L 20 10"));
        assert!(svg.contains("L 1 12"));
        assert!(svg.contains("L 0 5"));
    }

    #[test]
    fn p1246_ellipse_e_path_clip_sao_geometrias_locais() {
        let ellipse = export_svg(
            &p1246_group(ShapeKind::Ellipse, 20.0, 10.0),
            &SvgOptions::default(),
        );
        assert!(ellipse.contains("<ellipse cx=\"10\" cy=\"5\" rx=\"10\" ry=\"5\""));

        let path = ShapeKind::Path(vec![
            PathItem::MoveTo(Point { x: Pt(0.0), y: Pt(0.0) }),
            PathItem::LineTo(Point { x: Pt(12.0), y: Pt(0.0) }),
            PathItem::LineTo(Point { x: Pt(12.0), y: Pt(12.0) }),
            PathItem::ClosePath,
        ]);
        let svg = export_svg(&p1246_group(path, 12.0, 12.0), &SvgOptions::default());
        assert!(svg.contains("<path d=\"M 0 0 L 12 0 L 12 12 Z "));
        assert!(svg.contains("clip-rule=\"nonzero\""));
    }

    #[test]
    fn p1246_nested_clips_mantem_arestas_ancestral_e_local() {
        let inner = FrameItem::Group {
            pos: Point { x: Pt(2.0), y: Pt(3.0) },
            matrix: TransformMatrix::default(),
            clip_mask: Some(ShapeKind::Ellipse),
            inner_width: 8.0,
            inner_height: 8.0,
            items: vec![p1227_rect(Some(Paint::Solid(Color::rgb(0, 0, 255))), None)],
        };
        let page = p1227_page(vec![FrameItem::Group {
            pos: Point { x: Pt(5.0), y: Pt(7.0) },
            matrix: TransformMatrix::default(),
            clip_mask: Some(ShapeKind::Rect),
            inner_width: 12.0,
            inner_height: 10.0,
            items: vec![inner],
        }]);
        let svg = export_svg(&page, &SvgOptions::default());
        assert_eq!(svg.matches("<clipPath id=").count(), 2);
        assert_eq!(svg.matches("clip-path=\"url(#c").count(), 2);
    }

    fn p1247_link(label: &str) -> FrameItem {
        FrameItem::Link {
            target: LinkTarget::Destination(typst_core::entities::label::Label(
                label.to_string(),
            )),
            items: vec![p1227_rect(Some(Paint::Solid(Color::rgb(0, 0, 255))), None)],
            pos: Point { x: Pt(2.0), y: Pt(3.0) },
            size: Size { width: Pt(18.0), height: Pt(7.0) },
        }
    }

    #[test]
    fn p1247_same_page_fecha_href_e_no_destino() {
        let label = typst_core::entities::label::Label("target-alpha".to_string());
        let context = SvgDestinationContext::from_destinations([(
            label,
            Point { x: Pt(40.0), y: Pt(30.0) },
        )]);
        let svg = export_svg_with_context(
            &p1227_page(vec![p1247_link("target-alpha")]),
            &SvgOptions::default(),
            &context,
        );
        assert_eq!(svg.matches("data-crystalline-destination=").count(), 1);
        assert!(svg.contains("transform=\"translate(40 30)\""));
        let id = context
            .id_for(&typst_core::entities::label::Label("target-alpha".to_string()))
            .unwrap();
        assert!(svg.contains(&format!("id=\"{id}\"")));
        assert!(svg.contains(&format!("xlink:href=\"#{id}\"")));
        assert!(svg.contains("data-crystalline-link-hit-area=\"true\""));
        assert!(svg.contains("width=\"18\" height=\"7\""));
    }

    #[test]
    fn p1247_destino_ausente_nao_emite_fragmento_pendente() {
        let svg = export_svg_with_context(
            &p1227_page(vec![p1247_link("missing")]),
            &SvgOptions::default(),
            &SvgDestinationContext::empty(),
        );
        assert!(!svg.contains("xlink:href=\"#"));
        assert!(svg.contains("<rect"), "filhos visuais devem permanecer");
    }

    #[test]
    fn p1247_labels_distintos_na_mesma_posicao_nao_colidem() {
        let point = Point { x: Pt(10.0), y: Pt(20.0) };
        let a = typst_core::entities::label::Label("a&b".to_string());
        let b = typst_core::entities::label::Label("a<b".to_string());
        let context = SvgDestinationContext::from_destinations([
            (a.clone(), point),
            (b.clone(), point),
        ]);
        assert_ne!(context.id_for(&a), context.id_for(&b));
        let svg = export_svg_with_context(
            &p1227_page(vec![p1247_link("a&b"), p1247_link("a<b")]),
            &SvgOptions::default(),
            &context,
        );
        assert_eq!(svg.matches("data-crystalline-destination=").count(), 2);
    }

    fn p1248_image(
        data: &[u8],
        orientation: u32,
        clip_rect: Option<typst_core::entities::layout_types::Rect>,
    ) -> FrameItem {
        FrameItem::Image {
            pos: Point { x: Pt(7.0), y: Pt(11.0) },
            data: std::sync::Arc::new(data.to_vec()),
            width: Pt(6.0),
            height: Pt(2.0),
            intrinsic_width: 2,
            intrinsic_height: 2,
            clip_rect,
            orientation,
        }
    }

    #[test]
    fn p1248_mimes_raster_caixa_e_payload_permanecem_coerentes() {
        use image::{
            DynamicImage, GenericImageView, ImageBuffer, ImageFormat as RasterFormat,
            Rgba,
        };
        use std::io::Cursor;
        let pixels = ImageBuffer::from_fn(2, 2, |x, y| match (x, y) {
            (0, 0) => Rgba([255, 0, 0, 255]),
            (1, 0) => Rgba([0, 255, 0, 128]),
            (0, 1) => Rgba([0, 0, 255, 0]),
            _ => Rgba([255, 255, 255, 255]),
        });
        let image = DynamicImage::ImageRgba8(pixels);
        let mut cases = Vec::new();
        for (format, mime) in [
            (RasterFormat::Png, "image/png"),
            (RasterFormat::Jpeg, "image/jpeg"),
            (RasterFormat::Gif, "image/gif"),
            (RasterFormat::WebP, "image/webp"),
        ] {
            let mut encoded = Cursor::new(Vec::new());
            image.write_to(&mut encoded, format).unwrap();
            cases.push((encoded.into_inner(), mime));
        }
        for (bytes, mime) in &cases {
            let svg = export_svg(
                &p1227_page(vec![p1248_image(bytes, 1, None)]),
                &SvgOptions::default(),
            );
            assert!(svg.contains(&format!("data:{mime};base64,")));
            assert!(svg.contains("x=\"7\" y=\"11\" width=\"6\" height=\"2\""));
            let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
            assert!(
                svg.contains(&encoded),
                "payload/alpha deve permanecer byte-preservado"
            );
            assert_eq!(image::load_from_memory(bytes).unwrap().dimensions(), (2, 2));
        }
    }

    #[test]
    fn p1248_svg_aninhado_autossuficiente_preserva_landmarks() {
        let nested = br##"<?xml version="1.0"?><!--lead--><svg xmlns="http://www.w3.org/2000/svg" width="4" height="2"><rect fill="#ff0000"/><rect fill="#0000ff" fill-opacity="0.5"/></svg>"##;
        let mut page = p1227_page(vec![p1248_image(nested, 1, None)]);
        page.fill = typst_core::entities::page_canvas::PageFill::None;
        let svg = export_svg(&page, &SvgOptions::default());
        assert!(svg.contains("data:image/svg+xml;base64,"));
        let encoded = base64::engine::general_purpose::STANDARD.encode(nested);
        assert!(svg.contains(&encoded));
        assert!(!svg.contains("data-crystalline-image-fallback"));
    }

    #[test]
    fn p1248_unknown_e_svg_opaco_ficam_marcados_sem_image() {
        for bytes in [
            b"\x00\xffnot-image".as_slice(),
            b"\x1f\x8b\x08\x00\x00\x00\x00\x00\x00\x03".as_slice(),
            br#"<?xml version="1.0"?><metadata/><svg/>"#.as_slice(),
            br#"<svg xmlns="http://www.w3.org/2000/svg"><image href="https://invalid.example/x.png"/></svg>"#.as_slice(),
        ] {
            let svg = export_svg(&p1227_page(vec![p1248_image(bytes, 1, None)]), &SvgOptions::default());
            assert!(svg.contains("data-crystalline-image-fallback=\"unknown-format\""));
            assert!(!svg.contains("<image"));
            assert!(!svg.contains("data:image/"));
        }
    }

    #[test]
    fn p1248_clip_cover_e_orientacao_seis_aplicam_uma_vez() {
        let clip = typst_core::entities::layout_types::Rect {
            x: Pt(8.0),
            y: Pt(11.0),
            w: Pt(3.0),
            h: Pt(2.0),
        };
        let svg = export_svg(
            &p1227_page(vec![p1248_image(b"\xff\xd8\xffjpeg", 6, Some(clip))]),
            &SvgOptions::default(),
        );
        assert!(svg.contains("<clipPath id=\"c0\""));
        assert!(svg.contains("x=\"8\" y=\"11\" width=\"3\" height=\"2\""));
        assert_eq!(svg.matches("clip-path=\"url(#c0)\"").count(), 1);
        assert!(svg.contains("transform=\"matrix(0 2 -6 0 13 11)\""));
    }

    #[test]
    fn p1248_saida_de_imagem_e_deterministica() {
        let item = p1248_image(b"\x89PNG\r\n\x1a\nalpha", 1, None);
        let a = export_svg(&p1227_page(vec![item.clone()]), &SvgOptions::default());
        let b = export_svg(&p1227_page(vec![item]), &SvgOptions::default());
        assert_eq!(a, b);
    }

    fn p1248_rasterize(svg: &str) -> image::RgbaImage {
        use std::io::Write;
        use std::process::{Command, Stdio};
        let mut child = Command::new("rsvg-convert")
            .args(["--format", "png", "--width", "100", "--height", "100"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("rsvg-convert deve estar disponível para o oracle visual P1248");
        child.stdin.as_mut().unwrap().write_all(svg.as_bytes()).unwrap();
        let output = child.wait_with_output().unwrap();
        assert!(output.status.success());
        image::load_from_memory(&output.stdout).unwrap().to_rgba8()
    }

    #[test]
    fn p1248_svg_aninhado_e_renderizado_com_cor_e_alpha() {
        let nested = br##"<svg xmlns="http://www.w3.org/2000/svg" width="4" height="2" viewBox="0 0 4 2"><rect width="3" height="2" fill="#ff0000"/><rect x="1" width="3" height="2" fill="#0000ff" fill-opacity="0.5"/></svg>"##;
        let svg = export_svg(
            &p1227_page(vec![p1248_image(nested, 1, None)]),
            &SvgOptions::default(),
        );
        let marker = "data:image/svg+xml;base64,";
        let start = svg.find(marker).unwrap() + marker.len();
        let end = svg[start..].find('"').unwrap() + start;
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&svg[start..end])
            .unwrap();
        let pixels = p1248_rasterize(std::str::from_utf8(&decoded).unwrap());
        let left = pixels.get_pixel(10, 50).0;
        let overlap = pixels.get_pixel(50, 50).0;
        let right = pixels.get_pixel(90, 50).0;
        assert!(left[0] > 240 && left[2] < 15 && left[3] > 240, "{left:?}");
        assert!(overlap[0] > 110 && overlap[2] > 110 && overlap[3] > 240, "{overlap:?}");
        assert!(right[0] < 15 && right[1] < 15 && right[2] > 240, "{right:?}");
        assert!(
            right[3] >= 120 && right[3] <= 136,
            "alpha deve permanecer 0.5: {right:?}"
        );
    }

    #[test]
    fn p1248_clip_e_comprovado_por_amostragem_visual() {
        use image::{DynamicImage, ImageBuffer, ImageFormat as RasterFormat, Rgba};
        use std::io::Cursor;
        let source = ImageBuffer::from_fn(3, 1, |x, _| match x {
            0 => Rgba([255, 0, 0, 255]),
            1 => Rgba([0, 255, 0, 255]),
            _ => Rgba([0, 0, 255, 255]),
        });
        let mut bytes = Cursor::new(Vec::new());
        DynamicImage::ImageRgba8(source)
            .write_to(&mut bytes, RasterFormat::Png)
            .unwrap();
        let clip = typst_core::entities::layout_types::Rect {
            x: Pt(9.0),
            y: Pt(11.0),
            w: Pt(2.0),
            h: Pt(2.0),
        };
        let mut page = p1227_page(vec![p1248_image(bytes.get_ref(), 1, Some(clip))]);
        page.fill = typst_core::entities::page_canvas::PageFill::None;
        let svg = export_svg(&page, &SvgOptions::default());
        let pixels = p1248_rasterize(&svg);
        let outside_left = pixels.get_pixel(8, 12).0;
        let center = pixels.get_pixel(10, 12).0;
        let outside_right = pixels.get_pixel(12, 12).0;
        assert_eq!(outside_left[3], 0);
        assert!(center[1] > 180 && center[0] < 80 && center[2] < 80, "{center:?}");
        assert_eq!(outside_right[3], 0);
    }

    #[test]
    fn p1248_orientacao_seis_rasteriza_landmarks_assimetricos_uma_vez() {
        use image::{DynamicImage, ImageBuffer, ImageFormat as RasterFormat, Rgb};
        use std::io::Cursor;
        let source = ImageBuffer::from_fn(3, 2, |x, y| match (x, y) {
            (0, 0) => Rgb([255, 0, 0]),
            (1, 0) => Rgb([0, 255, 0]),
            (2, 0) => Rgb([255, 255, 255]),
            (0, 1) => Rgb([0, 0, 255]),
            (1, 1) => Rgb([0, 0, 0]),
            _ => Rgb([255, 255, 0]),
        });
        let mut encoded = Cursor::new(Vec::new());
        DynamicImage::ImageRgb8(source)
            .write_to(&mut encoded, RasterFormat::Jpeg)
            .unwrap();
        let item = FrameItem::Image {
            pos: Point { x: Pt(20.0), y: Pt(20.0) },
            data: std::sync::Arc::new(encoded.into_inner()),
            width: Pt(2.0),
            height: Pt(3.0),
            intrinsic_width: 3,
            intrinsic_height: 2,
            clip_rect: None,
            orientation: 6,
        };
        let mut page = p1227_page(vec![item]);
        page.fill = typst_core::entities::page_canvas::PageFill::None;
        let svg = export_svg(&page, &SvgOptions::default());
        assert!(svg.contains("data:image/jpeg;base64,"));
        assert!(svg.contains("transform=\"matrix(0 3 -2 0 22 20)\""));
        let pixels = p1248_rasterize(&svg);
        let top_left = pixels.get_pixel(20, 20).0;
        let top_right = pixels.get_pixel(21, 20).0;
        assert!(
            top_left[2] > top_left[0],
            "azul deve ocupar o topo esquerdo: {top_left:?}"
        );
        assert!(
            top_right[0] > top_right[2],
            "vermelho deve ocupar o topo direito: {top_right:?}"
        );
    }

    #[test]
    fn p1248_mutantes_m01_a_m13_alteram_saidas_reais_e_sao_rejeitados() {
        use image::{DynamicImage, ImageBuffer, ImageFormat as RasterFormat, Rgba};
        use std::io::Cursor;
        let source = DynamicImage::ImageRgba8(ImageBuffer::from_pixel(
            2,
            2,
            Rgba([9, 80, 170, 64]),
        ));
        let mut cursor = Cursor::new(Vec::new());
        source.write_to(&mut cursor, RasterFormat::Png).unwrap();
        let png = cursor.into_inner();
        let payload = base64::engine::general_purpose::STANDARD.encode(&png);
        let raster = export_svg(
            &p1227_page(vec![p1248_image(&png, 1, None)]),
            &SvgOptions::default(),
        );
        let clip = typst_core::entities::layout_types::Rect {
            x: Pt(9.0),
            y: Pt(11.0),
            w: Pt(2.0),
            h: Pt(2.0),
        };
        let clipped = export_svg(
            &p1227_page(vec![p1248_image(&png, 1, Some(clip))]),
            &SvgOptions::default(),
        );
        let oriented = export_svg(
            &p1227_page(vec![p1248_image(&png, 6, None)]),
            &SvgOptions::default(),
        );
        let nested_bytes = br##"<svg xmlns="http://www.w3.org/2000/svg"><rect fill="#ff0000"/><circle fill="#0000ff" fill-opacity="0.5"/></svg>"##;
        let nested_payload =
            base64::engine::general_purpose::STANDARD.encode(nested_bytes);
        let nested = export_svg(
            &p1227_page(vec![p1248_image(nested_bytes, 1, None)]),
            &SvgOptions::default(),
        );
        let unknown = export_svg(
            &p1227_page(vec![p1248_image(b"\x00\xff", 1, None)]),
            &SvgOptions::default(),
        );

        let valid =
            |raster: &str, clipped: &str, oriented: &str, nested: &str, unknown: &str| {
                raster.contains("data:image/png;base64,")
                    && raster.contains(&payload)
                    && raster.contains("x=\"7\" y=\"11\" width=\"6\" height=\"2\"")
                    && clipped.contains("clip-path=\"url(#c0)\"")
                    && clipped.contains("x=\"9\" y=\"11\" width=\"2\" height=\"2\"")
                    && oriented.contains("transform=\"matrix(0 2 -6 0 13 11)\"")
                    && nested.contains("data:image/svg+xml;base64,")
                    && nested.contains(&nested_payload)
                    && unknown
                        .contains("data-crystalline-image-fallback=\"unknown-format\"")
                    && !unknown.contains("<image")
            };
        assert!(valid(&raster, &clipped, &oriented, &nested, &unknown));
        for mutation in 1..=13 {
            let (mut r, mut c, mut o, mut n, mut u) = (
                raster.clone(),
                clipped.clone(),
                oriented.clone(),
                nested.clone(),
                unknown.clone(),
            );
            match mutation {
                1 => r = r.replace("data:image/png", "data:image/jpeg"),
                2 | 3 => r = r.replace(&payload, "AAAA"),
                4 => r = r.replace("x=\"7\" y=\"11\"", "x=\"0\" y=\"0\""),
                5 => {
                    r = r.replace("width=\"6\" height=\"2\"", "width=\"2\" height=\"2\"")
                }
                6 => c = c.replace("clip-path=\"url(#c0)\"", ""),
                7 => c = c.replace("x=\"9\" y=\"11\"", "x=\"109\" y=\"111\""),
                8 => o = o.replace("matrix(0 2 -6 0 13 11)", "matrix(6 0 0 2 7 11)"),
                9 => o = o.replace("matrix(0 2 -6 0 13 11)", "matrix(-6 0 0 -2 13 13)"),
                10 => n = n.replace("data:image/svg+xml", "data:application/omitted"),
                11 => n = n.replace(&nested_payload, "AAAA"),
                12 => {
                    u = u.replace(
                        "<g data-crystalline-image-fallback=\"unknown-format\"",
                        "<image xlink:href=\"data:image/png;base64,AAAA\"",
                    )
                }
                13 => {
                    u = u
                        .replace("data-crystalline-image-fallback=\"unknown-format\"", "")
                }
                _ => unreachable!(),
            }
            assert!(!valid(&r, &c, &o, &n, &u), "M{mutation:02} sobreviveu");
        }
    }

    #[test]
    fn p1254_svg_executa_grafo_repetido_e_recortado_do_layout() {
        use std::sync::Arc;
        use typst_core::entities::layout_types::{Length, Size};
        use typst_core::entities::tiling::{Tiling, TilingBody};
        use typst_core::entities::value::Value;

        let landmark = |color| {
            typst_core::entities::content::Content::shape(
                ShapeKind::Rect,
                Some(Box::new(Value::Length(Length::pt(2.0)))),
                Some(Box::new(Value::Length(Length::pt(2.0)))),
                Some(Paint::Solid(color)),
                None,
            )
        };
        let body = typst_core::entities::content::Content::stack(
            vec![landmark(Color::rgb(255, 0, 0)), landmark(Color::rgb(0, 0, 255))],
            typst_core::entities::dir::Dir::LTR,
            None,
        );
        let mut tiling = Tiling::new(TilingBody::Content(Arc::new(body)));
        tiling.size = Some(Size { width: Pt(6.0), height: Pt(3.0) });
        let content = typst_core::entities::content::Content::shape(
            ShapeKind::Rect,
            Some(Box::new(Value::Length(Length::pt(18.0)))),
            Some(Box::new(Value::Length(Length::pt(9.0)))),
            Some(Paint::Tiling(tiling)),
            None,
        );
        let render = || {
            let document = typst_core::compiler::layout::layout(&content);
            export_svg(&document.pages[0], &SvgOptions::default())
        };
        let first = render();
        let second = render();
        assert_eq!(first, second);
        assert!(first.contains("<clipPath"));
        assert!(first.matches("fill=\"#ff0000\"").count() > 1);
        assert!(first.matches("fill=\"#0000ff\"").count() > 1);
        assert!(first.find("fill=\"#ff0000\"") < first.find("fill=\"#0000ff\""));
    }

    fn p1249_fixture() -> (FrameItem, FontKey, GlyphFontRequest) {
        let bytes = include_bytes!("../../fixtures/fonts/NewCMMath-Book.otf").to_vec();
        let face = Face::parse(&bytes, 0).unwrap();
        let glyph_id = face.glyph_index('∑').unwrap().0;
        let family = FontList::single(EcoString::from("New Computer Modern Math"));
        let mut style = typst_core::entities::layout_types::TextStyle::regular(Pt(20.0));
        style.font = Some(family.clone());
        style.math = true;
        style.fill = Some(Color::rgba(255, 0, 0, 64));
        let request = GlyphFontRequest::new('∑', &style);
        let key = ((family, FontVariant::default(), FontVariations::default()), bytes);
        let item = FrameItem::Glyph {
            pos: Point { x: Pt(31.0), y: Pt(47.0) },
            glyph_id,
            x_advance: Pt(17.0),
            size: Pt(20.0),
            style,
            base_char: '∑',
        };
        (item, key, request)
    }

    #[test]
    fn p1249_glifo_direto_preserva_identidade_outline_posicao_escala_e_fill() {
        let (item, key, request) = p1249_fixture();
        let identity = key.0.clone();
        let context = SvgGlyphFontContext::from_resolutions([(request, identity)]);
        let svg = export_svg_with_fonts_and_contexts(
            &p1227_page(vec![item]),
            &SvgOptions::default(),
            &[key],
            &SvgDestinationContext::empty(),
            &context,
        );
        assert!(svg.contains("<symbol id=\"g0\""));
        assert!(svg.contains("<path d=\"M 0 0"));
        assert!(svg.contains("transform=\"matrix(1 0 0 -1 31 47)\""));
        assert!(svg.contains("fill=\"rgba(255,0,0,0.251)\""));
        assert!(!svg.contains("data-crystalline-glyph-fallback"));
    }

    #[test]
    fn p1249_unknown_nao_promove_fonte_incidental_ou_wrapper_fontless() {
        let (item, key, request) = p1249_fixture();
        let without_mapping = export_svg_with_fonts_and_contexts(
            &p1227_page(vec![item.clone()]),
            &SvgOptions::default(),
            std::slice::from_ref(&key),
            &SvgDestinationContext::empty(),
            &SvgGlyphFontContext::empty(),
        );
        assert!(without_mapping.contains("font-identity-unknown"));
        assert!(!without_mapping.contains("<use"));

        let other = (
            FontList::single(EcoString::from("conflicting face")),
            FontVariant::default(),
            FontVariations::default(),
        );
        let conflicted = SvgGlyphFontContext::from_resolutions([
            (request.clone(), key.0.clone()),
            (request, other),
        ]);
        let conflict_svg = export_svg_with_fonts_and_contexts(
            &p1227_page(vec![item.clone()]),
            &SvgOptions::default(),
            &[key],
            &SvgDestinationContext::empty(),
            &conflicted,
        );
        assert!(conflict_svg.contains("font-identity-unknown"));
        assert!(!conflict_svg.contains("<use"));

        let fontless = export_svg(&p1227_page(vec![item]), &SvgOptions::default());
        assert!(fontless.contains("fontless-wrapper"));
        assert!(!fontless.contains("<use"));
        assert!(!fontless.contains("∑"));
    }

    #[test]
    fn p1249_reordem_e_repeticao_preservam_resultado_semantico() {
        let alpha_bytes =
            include_bytes!("../../fixtures/fonts/NewCMMath-Book.otf").to_vec();
        let beta_bytes =
            include_bytes!("../../fixtures/fonts/NimbusSans-Regular.otf").to_vec();
        let alpha_face = Face::parse(&alpha_bytes, 0).unwrap();
        let beta_face = Face::parse(&beta_bytes, 0).unwrap();
        let alpha_gid = alpha_face.glyph_index('A').unwrap().0;
        let beta_gid = beta_face.glyph_index('A').unwrap().0;
        assert_eq!(alpha_gid, beta_gid, "fixture deve colidir glyph_id entre faces");
        let alpha_identity = (
            FontList::single(EcoString::from("alpha math")),
            FontVariant::default(),
            FontVariations::default(),
        );
        let beta_identity = (
            FontList::single(EcoString::from("beta sans")),
            FontVariant::default(),
            FontVariations::default(),
        );
        let alpha = (alpha_identity, alpha_bytes);
        let beta = (beta_identity.clone(), beta_bytes);
        let mut style = typst_core::entities::layout_types::TextStyle::regular(Pt(20.0));
        style.font = Some(beta_identity.0.clone());
        let request = GlyphFontRequest::new('A', &style);
        let item = FrameItem::Glyph {
            pos: Point { x: Pt(31.0), y: Pt(47.0) },
            glyph_id: beta_gid,
            x_advance: Pt(97.0),
            size: Pt(20.0),
            style,
            base_char: 'A',
        };
        let context = SvgGlyphFontContext::from_resolutions([(request, beta_identity)]);
        let render = |fonts: &[FontKey]| {
            export_svg_with_fonts_and_contexts(
                &p1227_page(vec![item.clone()]),
                &SvgOptions::default(),
                fonts,
                &SvgDestinationContext::empty(),
                &context,
            )
        };
        let first = render(&[alpha.clone(), beta.clone()]);
        let reordered = render(&[beta.clone(), alpha]);
        let beta_only = render(std::slice::from_ref(&beta));
        assert_eq!(first, reordered);
        assert_eq!(first, beta_only);
        assert!(!first.contains("font-identity-unknown"));
    }

    #[test]
    fn p1249_assembly_transform_e_advance_sao_preservados_sem_relayout() {
        let (piece, key, request) = p1249_fixture();
        let identity = key.0.clone();
        let context = SvgGlyphFontContext::from_resolutions([(request, identity)]);
        let mut second = piece.clone();
        let mut third = piece.clone();
        if let FrameItem::Glyph { pos, .. } = &mut second {
            pos.y = Pt(57.0);
        }
        if let FrameItem::Glyph { pos, .. } = &mut third {
            pos.y = Pt(67.0);
        }
        let landmark = FrameItem::Shape {
            pos: Point { x: Pt(80.0), y: Pt(10.0) },
            kind: ShapeKind::Rect,
            width: 2.0,
            height: 2.0,
            fill: Some(Paint::Solid(Color::rgb(0, 0, 255))),
            stroke: None,
            fill_rule: Default::default(),
            parent_bbox_at_emit: None,
        };
        let group = FrameItem::Group {
            pos: Point { x: Pt(5.0), y: Pt(7.0) },
            matrix: TransformMatrix { a: 0.0, b: 1.0, c: -1.0, d: 0.0, tx: 3.0, ty: 4.0 },
            clip_mask: None,
            inner_width: 100.0,
            inner_height: 100.0,
            items: vec![piece, second, third, landmark],
        };
        let svg = export_svg_with_fonts_and_contexts(
            &p1227_page(vec![group]),
            &SvgOptions::default(),
            &[key],
            &SvgDestinationContext::empty(),
            &context,
        );
        assert_eq!(svg.matches("<use ").count(), 3, "assembly deve manter três peças");
        assert!(svg.contains("translate(5 7) matrix(0 1 -1 0 3 4)"));
        assert!(svg.contains("x=\"80\" y=\"10\""), "x_advance não pode mover landmark");
        assert_eq!(svg.matches("matrix(1 0 0 -1 31 ").count(), 3);
    }

    #[test]
    fn p1249_discrimina_variante_glyph_id_e_escala_uma_vez() {
        let math_bytes =
            include_bytes!("../../fixtures/fonts/NewCMMath-Book.otf").to_vec();
        let sans_bytes =
            include_bytes!("../../fixtures/fonts/NimbusSans-Regular.otf").to_vec();
        let math = Face::parse(&math_bytes, 0).unwrap();
        let gid_a = math.glyph_index('A').unwrap().0;
        let gid_sum = math.glyph_index('∑').unwrap().0;
        assert_ne!(gid_a, gid_sum);
        let family = FontList::single(EcoString::from("variant witness"));
        let normal = (family.clone(), FontVariant::default(), FontVariations::default());
        let mut bold_style =
            typst_core::entities::layout_types::TextStyle::regular(Pt(20.0));
        bold_style.font = Some(family.clone());
        bold_style.bold = true;
        let bold = (
            family.clone(),
            text_style_to_font_variant(&bold_style),
            FontVariations::default(),
        );
        let mut normal_style =
            typst_core::entities::layout_types::TextStyle::regular(Pt(20.0));
        normal_style.font = Some(family);
        let normal_request = GlyphFontRequest::new('A', &normal_style);
        let bold_request = GlyphFontRequest::new('A', &bold_style);
        assert_ne!(normal_request, bold_request);
        let context = SvgGlyphFontContext::from_resolutions([
            (normal_request, normal.clone()),
            (bold_request, bold.clone()),
        ]);
        let glyph = |pos, gid, size, style| FrameItem::Glyph {
            pos,
            glyph_id: gid,
            x_advance: Pt(0.0),
            size,
            style,
            base_char: 'A',
        };
        let svg = export_svg_with_fonts_and_contexts(
            &p1227_page(vec![
                glyph(
                    Point { x: Pt(10.0), y: Pt(30.0) },
                    gid_sum,
                    Pt(20.0),
                    normal_style.clone(),
                ),
                glyph(Point { x: Pt(35.0), y: Pt(30.0) }, gid_a, Pt(20.0), bold_style),
                glyph(
                    Point { x: Pt(60.0), y: Pt(30.0) },
                    gid_sum,
                    Pt(40.0),
                    normal_style.clone(),
                ),
                glyph(Point { x: Pt(85.0), y: Pt(30.0) }, gid_a, Pt(20.0), normal_style),
            ]),
            &SvgOptions::default(),
            &[(normal, math_bytes.clone()), (bold, sans_bytes)],
            &SvgDestinationContext::empty(),
            &context,
        );
        assert_eq!(
            svg.matches("<symbol id=").count(),
            4,
            "variante, glyph_id e escala devem formar definições distintas"
        );
        let paths: Vec<&str> = svg
            .split("<path d=\"")
            .skip(1)
            .filter_map(|tail| tail.split('"').next())
            .collect();
        assert_eq!(paths.len(), 4);
        let expected = |gid, size| {
            let mut builder =
                SvgGlyphPathBuilder::new(size / f64::from(math.units_per_em()));
            math.outline_glyph(GlyphId(gid), &mut builder).unwrap();
            builder.finish()
        };
        assert_eq!(
            paths[0],
            expected(gid_sum, 20.0),
            "M04: deve extrair o glyph_id selecionado da face mapeada"
        );
        assert_eq!(
            paths[3],
            expected(gid_a, 20.0),
            "M04: base_char na mesma face é testemunha distinta"
        );
        assert_ne!(paths[0], paths[3]);
        assert_eq!(
            paths[2],
            expected(gid_sum, 40.0),
            "M05: escala deve ser exatamente size / units_per_em uma vez"
        );
    }
}
