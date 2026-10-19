//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/gradients/conic.md
//! @prompt-hash cde6dcfe
//! @layer L3
//! @updated 2026-05-19
//!
//! Conic Type 6 Coons Patch Mesh — bezier + compute_coons_patches + emit streams (P272).
//!
//! Extraído de `gradients.rs` em P307b.2 (ADR-0100 / diagnóstico
//! P307a §5). Conteúdo bit-exact pré e pós migração.

use super::cmyk::rgb_to_cmyk;
use typst_core::entities::gradient::{Conic, GradientStop};
use typst_core::entities::layout_types::{Color, ColorSpace, Ratio};

/// Physical reference size and visible canvas bounds in unit Y-up pattern space.
#[derive(Clone, Copy)]
pub(in crate::export) struct ConicDomain {
    pub size: (f64, f64),
    pub bounds: [f64; 4],
}

impl Default for ConicDomain {
    fn default() -> Self {
        Self { size: (1.0, 1.0), bounds: [0.0, 1.0, 0.0, 1.0] }
    }
}

pub(in crate::export) struct ConicMesh {
    pub bytes: Vec<u8>,
    pub decode: [f64; 4],
}

struct Geometry {
    center: (f64, f64),
    radius: (f64, f64),
    angle: f64,
    decode: [f64; 4],
}

impl Geometry {
    fn new(conic: &Conic, domain: ConicDomain) -> Self {
        let width = if domain.size.0.is_finite() && domain.size.0 != 0.0 {
            domain.size.0.abs()
        } else {
            1.0
        };
        let height = if domain.size.1.is_finite() && domain.size.1 != 0.0 {
            domain.size.1.abs()
        } else {
            1.0
        };
        let center = (conic.center.x.0, 1.0 - conic.center.y.0);
        let mut radius = 0.0_f64;
        for x in [domain.bounds[0], domain.bounds[1]] {
            for y in [domain.bounds[2], domain.bounds[3]] {
                radius = radius
                    .max(libm::hypot((x - center.0) * width, (y - center.1) * height));
            }
        }
        // The positive margin also protects the boundary against coordinate quantization.
        radius = radius.max(f64::MIN_POSITIVE) * 1.001;
        let radius = (radius / width, radius / height);
        let decode = [
            center.0 - radius.0 * 1.01,
            center.0 + radius.0 * 1.01 * (32767.0 / 32768.0),
            center.1 - radius.1 * 1.01,
            center.1 + radius.1 * 1.01 * (32767.0 / 32768.0),
        ];
        Self {
            center,
            radius,
            decode,
            angle: std::f64::consts::PI
                - conic.angle.to_rad().rem_euclid(std::f64::consts::TAU),
        }
    }

    fn patch(&self, t0: f64, t1: f64) -> [(f64, f64); 12] {
        let a = self.angle - std::f64::consts::TAU * t0;
        let b =
            if t1 == 1.0 { self.angle } else { self.angle - std::f64::consts::TAU * t1 };
        let (sa, ca) = libm::sincos(a);
        let (sb, cb) = libm::sincos(b);
        let (cx, cy) = self.center;
        let (rx, ry) = self.radius;
        let p = (cx + rx * ca, cy + ry * sa);
        let q = (cx + rx * cb, cy + ry * sb);
        let offset = (4.0 / 3.0) * libm::tan(-std::f64::consts::TAU * (t1 - t0) / 4.0);
        let c1 = (p.0 - rx * offset * sa, p.1 + ry * offset * ca);
        let c2 = (q.0 + rx * offset * sb, q.1 - ry * offset * cb);
        [
            self.center,
            (cx + (p.0 - cx) / 3.0, cy + (p.1 - cy) / 3.0),
            (cx + 2.0 * (p.0 - cx) / 3.0, cy + 2.0 * (p.1 - cy) / 3.0),
            p,
            c1,
            c2,
            q,
            (q.0 + (cx - q.0) / 3.0, q.1 + (cy - q.1) / 3.0),
            (q.0 + 2.0 * (cx - q.0) / 3.0, q.1 + 2.0 * (cy - q.1) / 3.0),
            self.center,
            self.center,
            self.center,
        ]
    }
}

fn components(color: Color, space: &ColorSpace) -> [f64; 4] {
    if let ColorSpace::Spot(_) = space {
        let tint = if let Color::Spot(s) = color { s.tint.0 } else { 0.0 };
        return [tint, 0.0, 0.0, 0.0];
    }
    if *space == ColorSpace::Cmyk {
        let (c, m, y, k) = match color {
            Color::Cmyk { c, m, y, k } => (c, m, y, k),
            other => {
                let (r, g, b, _) = other.to_rgba_f32();
                rgb_to_cmyk(r, g, b)
            }
        };
        return [c as f64, m as f64, y as f64, k as f64];
    }
    let (r, g, b, _) = color.to_rgba_f32();
    [r as f64, g as f64, b as f64, 0.0]
}

struct Interval {
    start: f64,
    end: f64,
    sampler: Conic,
}

fn intervals(conic: &Conic) -> Vec<Interval> {
    let offsets = conic.effective_offsets();
    let mut intervals = Vec::new();
    let Some(first) = conic.stops.first() else { return intervals };
    let mut push = |start: f64, end: f64, a: Color, b: Color| {
        let start = start.clamp(0.0, 1.0);
        let end = end.clamp(0.0, 1.0);
        if end <= start {
            return;
        }
        let mut sampler = conic.clone();
        sampler.stops = std::sync::Arc::from([
            GradientStop::new(a, Ratio(0.0)),
            GradientStop::new(b, Ratio(1.0)),
        ]);
        intervals.push(Interval { start, end, sampler });
    };
    push(0.0, offsets[0] as f64, first.color.clone(), first.color.clone());
    for (i, stops) in conic.stops.windows(2).enumerate() {
        push(
            offsets[i] as f64,
            offsets[i + 1] as f64,
            stops[0].color.clone(),
            stops[1].color.clone(),
        );
    }
    let last = conic.stops.last().expect("nonempty stops");
    push(
        *offsets.last().expect("nonempty offsets") as f64,
        1.0,
        last.color.clone(),
        last.color.clone(),
    );
    intervals
}

struct Encoder {
    geometry: Geometry,
    count: usize,
    bytes: Vec<u8>,
}

impl Encoder {
    fn segment(&mut self, interval: &Interval, a: f64, b: f64, depth: u8) {
        let sample = |u: f64| {
            components(interval.sampler.sample(u as f32), &interval.sampler.space)
        };
        let ca = sample(a);
        let cb = sample(b);
        let inaccurate = [0.25, 0.5, 0.75].into_iter().any(|f| {
            let c = sample(a + (b - a) * f);
            (0..self.count)
                .any(|i| (c[i] - (ca[i] + (cb[i] - ca[i]) * f)).abs() > 1.0 / 255.0)
        });
        if inaccurate && depth < 8 {
            let mid = (a + b) / 2.0;
            self.segment(interval, a, mid, depth + 1);
            self.segment(interval, mid, b, depth + 1);
            return;
        }
        let t0 = interval.start + (interval.end - interval.start) * a;
        let t1 = interval.start + (interval.end - interval.start) * b;
        self.bytes.push(0);
        for (x, y) in self.geometry.patch(t0, t1) {
            for (i, value) in [x, y].into_iter().enumerate() {
                let low = self.geometry.decode[2 * i];
                let high = self.geometry.decode[2 * i + 1];
                let encoded = (((value - low) / (high - low)).clamp(0.0, 1.0) * 65535.0)
                    .round() as u16;
                self.bytes.extend_from_slice(&encoded.to_be_bytes());
            }
        }
        for color in [ca, ca, cb, cb] {
            for component in &color[..self.count] {
                self.bytes.push((component.clamp(0.0, 1.0) * 255.0).round() as u8);
            }
        }
    }
}

/// One mesh/Decode contract for RGB, native CMYK and Separation tint samples.
pub(in crate::export) fn emit_conic_mesh(
    conic: &Conic,
    domain: ConicDomain,
) -> ConicMesh {
    let geometry = Geometry::new(conic, domain);
    let count = match &conic.space {
        ColorSpace::Cmyk => 4,
        ColorSpace::Spot(_) => 1,
        ColorSpace::Oklab
        | ColorSpace::Oklch
        | ColorSpace::Srgb
        | ColorSpace::Luma
        | ColorSpace::LinearRgb
        | ColorSpace::Hsl
        | ColorSpace::Hsv => 3,
    };
    if geometry.decode.iter().any(|v| !v.is_finite()) {
        return ConicMesh { bytes: Vec::new(), decode: [0.0, 1.0, 0.0, 1.0] };
    }
    let mut encoder = Encoder { geometry, count, bytes: Vec::new() };
    for interval in intervals(conic) {
        let n = ((interval.end - interval.start) * 128.0).ceil().max(1.0) as usize;
        for i in 0..n {
            encoder.segment(&interval, i as f64 / n as f64, (i + 1) as f64 / n as f64, 0);
        }
    }
    ConicMesh {
        bytes: encoder.bytes,
        decode: encoder.geometry.decode,
    }
}

#[cfg(test)]
pub(crate) fn emit_conic_coons_stream_rgb(conic: &Conic) -> Vec<u8> {
    emit_conic_mesh(conic, ConicDomain::default()).bytes
}
#[cfg(test)]
pub(crate) fn emit_conic_coons_stream_cmyk(conic: &Conic) -> Vec<u8> {
    emit_conic_mesh(conic, ConicDomain::default()).bytes
}

#[cfg(test)]
pub(crate) fn multispace_sample_stops_conic(
    conic: &typst_core::entities::gradient::Conic,
    n_samples: usize,
) -> Vec<(f32, f32, f32)> {
    let n = n_samples.max(2);
    (0..n)
        .map(|i| {
            let t = i as f32 / (n - 1) as f32;
            let c = conic.sample(t);
            let (r, g, b, _) = c.to_rgba_f32();
            (r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0))
        })
        .collect()
}

// ── P272 — Conic Type 6 Coons Patch Mesh L3 emit estratégia única ──
//
// **ADR-0090 REVOGADO P272**: Type 4 Gouraud descontinuado;
// `emit_conic_gouraud_stream` + `compute_adaptive_n_conic` +
// `oklab_delta_e` (P268+P268.2) removidos.
//
// **ADR-0092 expandida cumulativamente P272 (Cenário A revisado FINAL)**:
// Cristalino tem **estratégia única Coons** para 8/8 spaces — Type 6
// Coons via `emit_conic_coons_stream_rgb` (P272 N=stops*4; 7 spaces
// RGB-family + perceptual) e `emit_conic_coons_stream_cmyk` (P270.4
// N=stops; CMYK).

/// P270.3 — Matemática Bezier cúbico para arc círculo (Stanislaw Adaszewski).
///
/// Returns 2 control points entre start_angle e end_angle. Standard
/// approximation: `offset = radius * (4/3) * tan(angle/4)`. Erro máximo
/// ~0.0003 para quartos (90°); menor para N>4 patches angulares.
///
/// Reference: Stanislaw Adaszewski "Drawing a Circle with Bezier Curves".
#[cfg(test)]
pub(crate) fn bezier_control_points_for_arc(
    center: (f32, f32),
    radius: f32,
    start_angle: f32,
    end_angle: f32,
) -> [(f32, f32); 2] {
    let angle_delta = end_angle - start_angle;
    let offset = radius * (4.0 / 3.0) * libm::tanf(angle_delta / 4.0);

    let (sin_s, cos_s) = libm::sincosf(start_angle);
    let (sin_e, cos_e) = libm::sincosf(end_angle);

    // Control point 1: rotated 90° from start tangent.
    let cp1 = (
        center.0 + radius * cos_s - offset * sin_s,
        center.1 + radius * sin_s + offset * cos_s,
    );
    // Control point 2: rotated -90° from end tangent.
    let cp2 = (
        center.0 + radius * cos_e + offset * sin_e,
        center.1 + radius * sin_e - offset * cos_e,
    );

    [cp1, cp2]
}

/// Historical P270.3 count helper, kept only for the legacy helper tests.
///
/// N stops → N patches angulares. Cada patch cobre 360°/N graus.
#[cfg(test)]
pub(crate) fn compute_coons_patches_n_stops(
    conic: &typst_core::entities::gradient::Conic,
) -> usize {
    conic.stops.len()
}

/// Historical P272 count helper; the production mesh uses measured intervals.
///
/// Justificativa: qualidade visual angular superior; corner colors via
/// `Conic::sample(t)` dispatcher P270 (interpolate_in_space per
/// `conic.space`). Cap LOC accommodates extension.
#[cfg(test)]
pub(crate) fn compute_coons_patches_n_stops_extended(
    conic: &typst_core::entities::gradient::Conic,
) -> usize {
    conic.stops.len() * 4
}

#[cfg(test)]
mod geometry_tests {
    use super::*;
    use crate::export::{export_pdf, StreamMode};
    use std::sync::Arc;
    use typst_core::entities::axes::Axes;
    use typst_core::entities::content::Content;
    use typst_core::entities::frame_paint::FramePaint as Paint;
    use typst_core::entities::geometry::ShapeKind;
    use typst_core::entities::gradient::{Gradient, GradientStop};
    use typst_core::entities::layout_types::{
        Angle, ColorSpace, FrameItem, Page, PagedDocument, Point, Pt, Ratio,
    };

    struct Patch {
        points: Vec<(f64, f64)>,
        colors: Vec<Vec<u8>>,
    }

    fn conic(space: ColorSpace, stops: Vec<(Color, f64)>) -> Conic {
        Conic {
            stops: Arc::from(
                stops
                    .into_iter()
                    .map(|(c, t)| GradientStop::new(c, Ratio(t)))
                    .collect::<Vec<_>>(),
            ),
            center: Axes::new(Ratio(0.5), Ratio(0.5)),
            angle: Angle::deg(0.0),
            space,
            relative: None,
            anti_alias: true,
        }
    }

    fn rgb() -> Conic {
        conic(
            ColorSpace::Srgb,
            vec![(Color::rgb(255, 0, 0), 0.0), (Color::rgb(0, 0, 255), 1.0)],
        )
    }

    fn mesh(c: Conic, width: f64, height: f64) -> Vec<Patch> {
        mesh_on_page(c, width, height, width, height)
    }

    fn mesh_on_page(
        c: Conic,
        width: f64,
        height: f64,
        page_width: f64,
        page_height: f64,
    ) -> Vec<Patch> {
        let components = if c.space == ColorSpace::Cmyk { 4 } else { 3 };
        let doc = PagedDocument::new(vec![Page {
            width: page_width,
            height: page_height,
            numbering: None,
            logical_number: 1,
            supplement: Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items: vec![FrameItem::Shape {
                pos: Point { x: Pt(0.0), y: Pt(0.0) },
                kind: ShapeKind::Rect,
                width,
                height,
                fill: Some(Paint::Gradient(Gradient::Conic(Arc::new(c)))),
                stroke: None,
                fill_rule: Default::default(),
                parent_bbox_at_emit: None,
            }],
        }]);
        let pdf = export_pdf(&doc, StreamMode::Compact);
        let start = pdf
            .windows(b"/ShadingType 6 ".len())
            .position(|x| x == b"/ShadingType 6 ")
            .expect("Type 6");
        let end = start
            + pdf[start..]
                .windows(7)
                .position(|x| x == b"stream\n")
                .expect("stream");
        let header = std::str::from_utf8(&pdf[start..end]).unwrap();
        let integer = |key: &str| -> usize {
            header
                .split_once(key)
                .unwrap()
                .1
                .split_whitespace()
                .next()
                .unwrap()
                .parse()
                .unwrap()
        };
        let bits = integer("/BitsPerCoordinate ");
        assert!(bits == 8 || bits == 16);
        let decode: Vec<f64> = header
            .split_once("/Decode [")
            .unwrap()
            .1
            .split_once(']')
            .unwrap()
            .0
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let data = &pdf[end + 7..end + 7 + integer("/Length ")];
        let stride = 1 + 24 * (bits / 8) + 4 * components;
        assert_eq!(data.len() % stride, 0);
        data.chunks_exact(stride)
            .map(|p| {
                assert_eq!(p[0], 0);
                let coord = |i: usize| {
                    let off = 1 + i * (bits / 8);
                    let v = if bits == 8 {
                        p[off] as f64 / 255.0
                    } else {
                        u16::from_be_bytes([p[off], p[off + 1]]) as f64 / 65535.0
                    };
                    let j = 2 * (i % 2);
                    decode[j] + v * (decode[j + 1] - decode[j])
                };
                Patch {
                    points: (0..12).map(|i| (coord(2 * i), coord(2 * i + 1))).collect(),
                    colors: p[1 + 24 * (bits / 8)..]
                        .chunks_exact(components)
                        .map(|v| v.to_vec())
                        .collect(),
                }
            })
            .collect()
    }

    #[test]
    fn conic_geometry_author_center_is_not_discarded() {
        let mut c = rgb();
        c.center = Axes::new(Ratio(0.3), Ratio(0.7));
        for p in mesh(c, 96.0, 64.0) {
            assert!((p.points[0].0 - 0.3).abs() < 0.0001, "{:?}", p.points[0]);
            assert!((p.points[0].1 - 0.3).abs() < 0.0001, "{:?}", p.points[0]);
        }
    }

    #[test]
    fn conic_geometry_covers_farthest_corner_in_physical_space() {
        let mut c = rgb();
        c.center = Axes::new(Ratio(0.3), Ratio(0.7));
        let required = libm::hypot(0.7 * 96.0, 0.7 * 64.0);
        for p in mesh(c, 96.0, 64.0) {
            for i in [3, 6] {
                let radius = libm::hypot(
                    (p.points[i].0 - 0.3) * 96.0,
                    (p.points[i].1 - 0.3) * 64.0,
                );
                assert!(radius >= required, "radius {radius} below {required}");
            }
        }
    }

    #[test]
    fn conic_geometry_origin_and_rotation_follow_language() {
        for (angle, expected) in
            [(0.0, (-1.0, 0.0)), (90.0, (0.0, 1.0)), (180.0, (1.0, 0.0))]
        {
            let mut c = rgb();
            c.angle = Angle::deg(angle);
            let patches = mesh(c, 96.0, 64.0);
            let a = patches[0].points[3];
            let center = patches[0].points[0];
            let dx = (a.0 - center.0) * 96.0;
            let dy = (a.1 - center.1) * 64.0;
            let norm = libm::hypot(dx, dy);
            assert!(
                (dx / norm - expected.0).abs() < 0.001
                    && (dy / norm - expected.1).abs() < 0.001,
                "{angle}: {dx},{dy}"
            );
        }
    }

    #[test]
    fn conic_geometry_cmyk_half_turn_samples_half_mix() {
        let c = conic(
            ColorSpace::Cmyk,
            vec![
                (Color::cmyk(1.0, 0.0, 0.0, 0.0), 0.0),
                (Color::cmyk(0.0, 1.0, 0.0, 0.0), 1.0),
            ],
        );
        let patches = mesh(c, 96.0, 96.0);
        let p = patches
            .iter()
            .find(|p| p.points[3].0 > 0.5 && (p.points[3].1 - 0.5).abs() < 0.001)
            .expect("half turn boundary");
        assert!(
            p.colors[1][0].abs_diff(128) <= 1 && p.colors[1][1].abs_diff(128) <= 1,
            "{:?}",
            p.colors[1]
        );
        assert_eq!(
            patches.last().unwrap().colors[2],
            vec![0, 255, 0, 0],
            "last color must not wrap"
        );
    }

    #[test]
    fn conic_geometry_cmyk_explicit_stop_is_a_mesh_boundary() {
        let c = conic(
            ColorSpace::Cmyk,
            vec![
                (Color::cmyk(1.0, 0.0, 0.0, 0.0), 0.0),
                (Color::cmyk(0.0, 1.0, 0.0, 0.0), 0.25),
                (Color::cmyk(0.0, 0.0, 1.0, 0.0), 1.0),
            ],
        );
        let patches = mesh(c, 96.0, 96.0);
        let p = patches
            .iter()
            .find(|p| p.points[3].1 > 0.5 && (p.points[3].0 - 0.5).abs() < 0.001)
            .expect("explicit quarter-turn stop");
        assert_eq!(p.colors[1], vec![0, 255, 0, 0]);
    }

    #[test]
    fn conic_geometry_repeated_offsets_keep_both_sides() {
        let red = Color::rgb(255, 0, 0);
        let blue = Color::rgb(0, 0, 255);
        let c = conic(
            ColorSpace::Srgb,
            vec![(red.clone(), 0.0), (red, 0.25), (blue.clone(), 0.25), (blue, 1.0)],
        );
        let patches = mesh(c, 96.0, 96.0);
        let before = patches
            .iter()
            .find(|p| p.points[6].1 > 0.5 && (p.points[6].0 - 0.5).abs() < 0.001)
            .expect("left limit");
        let after = patches
            .iter()
            .find(|p| p.points[3].1 > 0.5 && (p.points[3].0 - 0.5).abs() < 0.001)
            .expect("right limit");
        assert_eq!(before.colors[2], vec![255, 0, 0]);
        assert_eq!(after.colors[1], vec![0, 0, 255]);
    }
    #[test]
    fn conic_geometry_center_quantization_does_not_shift_author_origin() {
        let mut c = rgb();
        c.center = Axes::new(Ratio(0.3), Ratio(0.7));
        let p = &mesh(c, 96.0, 64.0)[0];
        assert!((p.points[0].0 - 0.3).abs() < 1e-10, "{:?}", p.points[0]);
        assert!((p.points[0].1 - 0.3).abs() < 1e-10, "{:?}", p.points[0]);
    }

    #[test]
    fn conic_geometry_domain_covers_canvas_outside_nominal_shape() {
        let patches = mesh_on_page(rgb(), 10.0, 10.0, 96.0, 64.0);
        let required = libm::hypot(96.0 - 5.0, 64.0 - 5.0);
        for p in patches {
            for i in [3, 6] {
                let radius = libm::hypot(
                    (p.points[i].0 - 0.5) * 10.0,
                    (p.points[i].1 - 0.5) * 10.0,
                );
                assert!(radius >= required, "{radius} below {required}");
            }
        }
    }
}
