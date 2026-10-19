//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/svg/paints.md
//! @prompt-hash 3f4b15db
//! @layer L3
//! @updated 2026-09-26

use super::super::gradients::svg_adaptive_stops;
use super::serialization::{color_to_css, fmt_num};
use std::sync::Arc;
use typst_core::entities::frame_paint::FramePaint as Paint;
use typst_core::entities::gradient::Gradient;
use typst_core::entities::layout_types::Color;
use typst_core::entities::layout_types::{Point, Pt, TransformMatrix};
use xmlwriter::XmlWriter;

/// Paint servers referenced by shapes. The linear vector is intentional: Paint
/// has semantic equality but no mechanical Hash, and SVG pages normally carry
/// few distinct paints.
#[derive(Default)]
pub(super) struct PaintDefs {
    paints: Vec<SvgPaintDef>,
    pub(super) local_to_root: TransformMatrix,
    pub(super) decorative: bool,
}

#[derive(Clone)]
struct SvgPaintDef {
    matrix: TransformMatrix,
    paint: Paint,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

pub(super) struct PaintReference {
    pub(super) value: String,
    pub(super) fallback: Option<&'static str>,
}

impl PaintDefs {
    pub(super) fn reference(
        &mut self,
        paint: &Paint,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> PaintReference {
        if let Paint::Solid(color) = paint {
            return PaintReference { value: color_to_css(color.clone()), fallback: None };
        }
        if !paint_is_svg_native(paint) {
            return PaintReference {
                value: color_to_css(match paint {
                    Paint::Gradient(g) => {
                        typst_core::entities::paint::Paint::Gradient(g.clone()).to_color()
                    }
                    _ => unreachable!(),
                }),
                fallback: Some(unsupported_paint_reason(paint)),
            };
        }
        let matrix = if let Paint::Tiling(cell) = paint {
            let Some(matrix) = crate::export::tiling::placement(
                cell,
                Point { x: Pt(x), y: Pt(y) },
                self.local_to_root,
            ) else {
                return PaintReference {
                    value: "none".into(),
                    fallback: Some("noninvertible-tiling-transform"),
                };
            };
            matrix
        } else {
            TransformMatrix::identity()
        };
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
                same_paint(&candidate.paint, paint)
                    && candidate.matrix == matrix
                    && candidate.x.to_bits() == paint_x.to_bits()
                    && candidate.y.to_bits() == paint_y.to_bits()
                    && candidate.width.to_bits() == paint_width.to_bits()
                    && candidate.height.to_bits() == paint_height.to_bits()
            })
            .unwrap_or_else(|| {
                self.paints.push(SvgPaintDef {
                    paint: paint.clone(),
                    matrix,
                    x: paint_x,
                    y: paint_y,
                    width: paint_width,
                    height: paint_height,
                });
                self.paints.len() - 1
            });
        PaintReference { value: format!("url(#p{index})"), fallback: None }
    }

    pub(super) fn write(
        &mut self,
        xml: &mut XmlWriter,
        fonts: Option<&crate::export::font_binding::PreparedFonts>,
        glyph_defs: &mut super::GlyphDefs,
        clip_next_id: &mut usize,
        glyph_fonts: &super::SvgGlyphFontContext,
    ) {
        if self.paints.is_empty() {
            return;
        }
        xml.start_element("defs");
        let mut index = 0;
        while index < self.paints.len() {
            let definition = self.paints[index].clone();
            let id = format!("p{index}");
            if let Paint::Tiling(cell) = &definition.paint {
                let size = cell.frame.size;
                let clip = format!("c{}", *clip_next_id);
                *clip_next_id += 1;
                xml.start_element("clipPath");
                xml.write_attribute("id", &clip);
                xml.write_attribute("clipPathUnits", "userSpaceOnUse");
                xml.start_element("rect");
                xml.write_attribute("width", &fmt_num(size.width.0));
                xml.write_attribute("height", &fmt_num(size.height.0));
                xml.end_element();
                xml.end_element();
                xml.start_element("g");
                xml.write_attribute("id", &format!("{id}cell"));
                xml.write_attribute("clip-path", &format!("url(#{clip})"));
                let previous = (self.local_to_root, self.decorative);
                self.local_to_root = TransformMatrix::identity();
                self.decorative = true;
                {
                    let destinations = super::SvgDestinationContext::empty();
                    let mut session = super::SvgRenderSession {
                        fonts,
                        glyph_defs: &mut *glyph_defs,
                        paint_defs: &mut *self,
                        clip_next_id: &mut *clip_next_id,
                        destinations: &destinations,
                        glyph_fonts,
                        html_frame: false,
                    };
                    for item in &cell.frame.items {
                        super::render_item(xml, item, &mut session);
                    }
                }
                (self.local_to_root, self.decorative) = previous;
                xml.end_element();
                xml.start_element("pattern");
                xml.write_attribute("id", &id);
                xml.write_attribute("patternUnits", "userSpaceOnUse");
                xml.write_attribute("patternContentUnits", "userSpaceOnUse");
                xml.write_attribute(
                    "width",
                    &fmt_num(size.width.0 + cell.spacing.width.0),
                );
                xml.write_attribute(
                    "height",
                    &fmt_num(size.height.0 + cell.spacing.height.0),
                );
                xml.write_attribute("overflow", "hidden");
                let m = definition.matrix;
                xml.write_attribute(
                    "patternTransform",
                    &format!(
                        "matrix({} {} {} {} {} {})",
                        fmt_num(m.a),
                        fmt_num(m.b),
                        fmt_num(m.c),
                        fmt_num(m.d),
                        fmt_num(m.tx),
                        fmt_num(m.ty)
                    ),
                );
                for (x, y) in crate::export::tiling::contributions(cell)
                    .expect("prepared tiling contribution range")
                {
                    xml.start_element("use");
                    xml.write_attribute("href", &format!("#{id}cell"));
                    xml.write_attribute("xlink:href", &format!("#{id}cell"));
                    xml.write_attribute(
                        "transform",
                        &format!("translate({} {})", fmt_num(x), fmt_num(y)),
                    );
                    xml.end_element();
                }
                xml.end_element();
            } else {
                write_paint_def(
                    xml,
                    &id,
                    &definition.paint,
                    definition.x,
                    definition.y,
                    definition.width,
                    definition.height,
                );
            }
            index += 1;
        }
        xml.end_element();
    }
}

fn same_paint(a: &Paint, b: &Paint) -> bool {
    match (a, b) {
        (Paint::Solid(a), Paint::Solid(b)) => a == b,
        (Paint::Gradient(a), Paint::Gradient(b)) => a == b,
        (Paint::Tiling(a), Paint::Tiling(b)) => Arc::ptr_eq(a, b),
        (Paint::Solid(_), Paint::Gradient(_) | Paint::Tiling(_))
        | (Paint::Gradient(_), Paint::Solid(_) | Paint::Tiling(_))
        | (Paint::Tiling(_), Paint::Solid(_) | Paint::Gradient(_)) => false,
    }
}

fn unsupported_paint_reason(paint: &Paint) -> &'static str {
    match paint {
        Paint::Gradient(Gradient::Conic(_)) => "conic-gradient",
        Paint::Gradient(_) => "gradient-color-space",
        Paint::Tiling(_) => unreachable!("prepared tiling is native"),
        Paint::Solid(_) => "none",
    }
}

pub(super) fn paint_is_svg_native(paint: &Paint) -> bool {
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
        Paint::Tiling(_) => true,
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
            let dx = libm::cos(angle) / 2.0;
            let dy = libm::sin(angle) / 2.0;
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
        Paint::Tiling(_) => unreachable!("prepared cell emitted by PaintDefs"),
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
        -libm::atan2(libm::sin(angle) / inverse_ratio.abs(), libm::cos(angle))
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
                .map(|stop| stop.color.clone())
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
        xml.write_attribute("x1", &fmt_num(2.0 - libm::cos(theta1) + cx));
        xml.write_attribute("y1", &fmt_num(libm::sin(theta1) + cy));
        xml.write_attribute("x2", &fmt_num(2.0 - libm::cos(theta2) + cx));
        xml.write_attribute("y2", &fmt_num(libm::sin(theta2) + cy));
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
        let x1 = pattern_cx - libm::cos(theta1);
        let y1 = pattern_cy + libm::sin(theta1);
        let x2 = pattern_cx - libm::cos(theta2);
        let y2 = pattern_cy + libm::sin(theta2);
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

pub(super) fn write_svg_gradient_stops(
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

pub(super) fn adaptive_offsets_use_ratio_repr(gradient: &Gradient) -> bool {
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

pub(super) fn fmt_ratio_repr(offset: f64) -> String {
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
