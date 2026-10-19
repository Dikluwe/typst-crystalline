//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/builder/resources.md
//! @prompt-hash be45f3a4
//! @layer L3
//! @updated 2026-09-28

use super::PdfBuilder;
use crate::export::adaptive_n_for_stops;
use crate::export::apply_parent_transform;
use crate::export::build_jpeg_xobject;
use crate::export::build_png_rgb_xobject;
use crate::export::build_png_smask_xobject;
use crate::export::compute_axial_coords;
use crate::export::compute_radial_coords;
use crate::export::detect_image_format;
use crate::export::emit_conic_mesh;
use crate::export::emit_function_dict;
use crate::export::emit_function_dict_cmyk;
use crate::export::emit_function_dict_spot;
use crate::export::jpeg_color_space;
use crate::export::jpeg_is_rgb;
use crate::export::multispace_sample_stops;
use crate::export::multispace_sample_stops_linear_cmyk;
use crate::export::multispace_sample_stops_radial;
use crate::export::multispace_sample_stops_radial_cmyk;
use crate::export::pdf_defaults;
use crate::export::resolve_relative;
use crate::export::GradientObject;
use crate::export::GradientObjectKind;
use crate::export::ImageFormat;
use crate::export::ImageXObject;
use typst_core::entities::frame_visitor::walk_frame_items;
use typst_core::entities::frame_visitor::FrameVisitor;
use typst_core::entities::layout_types::FrameItem;
use typst_core::entities::layout_types::PagedDocument;

/// **P956** — entrada `/ColorSpace << /c0 [/ICCBased <icc_id> 0 R] >>` para
/// os recursos de página em modo verbose (o `/c0` dos blocos de texto aponta
/// para o perfil ICC sRGB sempre embutido nesse modo). A forma array
/// `[/ICCBased n 0 R]` é a mesma já usada pelos XObjects de imagem (P777) —
/// um colour space ICCBased é, por definição PDF, um array com o nome
/// `/ICCBased` seguido da referência do stream do perfil. Compact → string
/// vazia (recursos byte-inalterados).
fn sampled_spot_tint(color: typst_core::entities::color::Color) -> f32 {
    if let typst_core::entities::color::Color::Spot(spot) = color {
        spot.tint.get().clamp(0.0, 1.0) as f32
    } else {
        0.0
    }
}

fn pdf_spot_space(
    colorant: &typst_core::entities::color::SpotColorant,
    icc_profile_id: Option<usize>,
) -> String {
    use typst_core::entities::color::Color;
    let name = match colorant.name.as_deref() {
        None => "/None".to_string(),
        Some("all") => "/All".to_string(),
        Some(name) => {
            let mut out = String::from("/");
            for byte in name.bytes() {
                if (33..=126).contains(&byte) && !b"#%()/<>[]{}".contains(&byte) {
                    out.push(byte as char);
                } else {
                    out.push_str(&format!("#{byte:02X}"));
                }
            }
            out
        }
    };
    let (alternate, c0, c1, range) = match colorant.fallback {
        Color::Cmyk { c, m, y, k } => (
            "/DeviceCMYK".to_string(),
            "0 0 0 0".to_string(),
            format!("{c:.8} {m:.8} {y:.8} {k:.8}"),
            "0 1 0 1 0 1 0 1",
        ),
        _ => {
            let (r, g, b, _) = colorant.fallback.to_rgba_f32();
            let space = icc_profile_id
                .map(|id| format!("[/ICCBased {id} 0 R]"))
                .unwrap_or_else(|| "/DeviceRGB".to_string());
            (space, "1 1 1".to_string(), format!("{r:.8} {g:.8} {b:.8}"), "0 1 0 1 0 1")
        }
    };
    format!(
        "[/Separation {name} {alternate} << /FunctionType 2 /Domain [0 1] /C0 [{c0}] /C1 [{c1}] /N 1 /Range [{range}] >>]"
    )
}

fn page_spot_colorants(
    page: &typst_core::entities::layout_types::Page,
) -> Vec<typst_core::entities::color::SpotColorant> {
    use typst_core::entities::color::{Color, SpotColorant};
    struct Collector(Vec<SpotColorant>);
    impl Collector {
        fn color(&mut self, color: &Color) {
            if let Color::Spot(spot) = color {
                if !self.0.contains(&spot.colorant) {
                    self.0.push((*spot.colorant).clone());
                }
            }
        }
        fn prepared(&mut self, paint: &typst_core::entities::frame_paint::FramePaint) {
            use typst_core::entities::frame_paint::FramePaint;
            match paint {
                FramePaint::Solid(c) => self.color(c),
                FramePaint::Gradient(g) => {
                    self.paint(&typst_core::entities::paint::Paint::Gradient(g.clone()))
                }
                FramePaint::Tiling(c) => walk_frame_items(self, &c.frame.items),
            }
        }
        fn paint(&mut self, paint: &typst_core::entities::paint::Paint) {
            use typst_core::entities::gradient::Gradient;
            use typst_core::entities::paint::Paint;
            match paint {
                Paint::Solid(color) => self.color(color),
                Paint::Gradient(gradient) => {
                    let stops = match gradient {
                        Gradient::Linear(g) => &g.stops,
                        Gradient::Radial(g) => &g.stops,
                        Gradient::Conic(g) => &g.stops,
                    };
                    for stop in stops.iter() {
                        self.color(&stop.color);
                    }
                }
                Paint::Tiling(_) => {}
            }
        }
    }
    impl FrameVisitor for Collector {
        fn visit_text(&mut self, item: &FrameItem) {
            if let FrameItem::TextSeparator { style, .. }
            | FrameItem::Text { style, .. } = item
            {
                if let Some(c) = &style.fill {
                    self.color(c);
                }
            }
        }
        fn visit_text_shaped(&mut self, item: &FrameItem) {
            if let FrameItem::TextShaped { style, .. } = item {
                if let Some(c) = &style.fill {
                    self.color(c);
                }
            }
        }
        fn visit_glyph(&mut self, item: &FrameItem) {
            if let FrameItem::Glyph { style, .. } = item {
                if let Some(c) = &style.fill {
                    self.color(c);
                }
            }
        }
        fn visit_line(&mut self, item: &FrameItem) {
            if let FrameItem::Line { color: Some(c), .. } = item {
                self.color(c);
            }
        }
        fn visit_shape(&mut self, item: &FrameItem) {
            if let FrameItem::Shape { fill, stroke, .. } = item {
                if let Some(fill) = fill {
                    self.prepared(fill);
                }
                if let Some(stroke) = stroke {
                    self.prepared(&stroke.paint);
                }
            }
        }
    }
    let mut collector = Collector(Vec::new());
    if let typst_core::entities::page_canvas::PageFill::Paint(paint) = &page.fill {
        collector.paint(paint);
    }
    for items in [&page.background, &page.items, &page.foreground] {
        walk_frame_items(&mut collector, items);
    }
    collector.0
}

fn opacity_resources(page: &typst_core::entities::layout_types::Page) -> String {
    use crate::export::stream::{opacity_name, solid_opacity};
    struct Collector(std::collections::BTreeMap<String, (f32, f32)>);
    impl Collector {
        fn add(&mut self, fill: f32, stroke: f32) {
            if fill != 1.0 || stroke != 1.0 {
                self.0.insert(opacity_name(fill, stroke), (fill, stroke));
            }
        }
    }
    impl FrameVisitor for Collector {
        fn visit_item(&mut self, item: &FrameItem) {
            let (f, s) = solid_opacity(item);
            self.add(f, s);
        }
        fn visit_shape(&mut self, item: &FrameItem) {
            self.visit_item(item);
        }
    }
    let mut collector = Collector(Default::default());
    if let typst_core::entities::page_canvas::PageFill::Paint(
        typst_core::entities::paint::Paint::Solid(c),
    ) = &page.fill
    {
        collector.add(c.to_rgba_f32().3, 1.0);
    }
    for items in [&page.background, &page.items, &page.foreground] {
        crate::export::tiling::walk_resources(&mut collector, items);
    }
    if collector.0.is_empty() {
        return String::new();
    }
    let mut out = String::from(" /ExtGState << ");
    for (name, (fill, stroke)) in collector.0 {
        out.push_str(&format!("/{name} << /Type /ExtGState /ca {fill} /CA {stroke} >> "));
    }
    out.push_str(">>");
    out
}

pub(super) fn colorspace_resource_entry(
    verbose: bool,
    icc_profile_id: Option<usize>,
    page: &typst_core::entities::layout_types::Page,
) -> String {
    let mut entries = Vec::new();
    if let (true, Some(id)) = (verbose, icc_profile_id) {
        entries.push(format!("/c0 [/ICCBased {id} 0 R]"));
    }
    for colorant in page_spot_colorants(page) {
        entries.push(format!(
            "/{} {}",
            crate::export::stream::spot_resource_name(&colorant),
            pdf_spot_space(&colorant, icc_profile_id)
        ));
    }
    let opacity = opacity_resources(page);
    if entries.is_empty() {
        opacity
    } else {
        format!(" /ColorSpace << {} >>{opacity}", entries.join(" "))
    }
}

/// **P777** — verdadeiro se o documento contiver pelo menos um JPEG RGB.
pub(super) fn has_rgb_jpeg(doc: &PagedDocument) -> bool {
    struct RgbJpegDetector(bool);
    impl FrameVisitor for RgbJpegDetector {
        fn visit_image(&mut self, item: &FrameItem) {
            if let FrameItem::Image { data, .. } = item {
                if detect_image_format(data) == ImageFormat::Jpeg && jpeg_is_rgb(data) {
                    self.0 = true;
                }
            }
        }
    }
    let mut detector = RgbJpegDetector(false);
    for p in &doc.pages {
        for items in [&p.background, &p.items, &p.foreground] {
            crate::export::tiling::walk_resources(&mut detector, items);
        }
        if detector.0 {
            return true;
        }
    }
    false
}

/// **P941** — funde as entradas `/ImN id 0 R` dos XObjects de glifos bitmap no
/// bloco `/XObject << ... >>` das imagens do documento. Sem isto, as entradas
/// bitmap ficavam fora do dicionário e os leitores PDF não resolviam a
/// referência (`XObject 'Im3' is unknown` no poppler).
pub(super) fn merge_bitmap_xobjects(
    base_xobject_res: String,
    bitmap_entries: &[String],
) -> String {
    if bitmap_entries.is_empty() {
        return base_xobject_res;
    }
    let joined = bitmap_entries.join(" ");
    if base_xobject_res.is_empty() {
        return format!("/XObject << {joined} >>");
    }
    let inner = base_xobject_res
        .strip_prefix("/XObject << ")
        .and_then(|s| s.strip_suffix(" >>"))
        .unwrap_or("");
    if inner.is_empty() {
        format!("/XObject << {joined} >>")
    } else {
        format!("/XObject << {inner} {joined} >>")
    }
}

impl PdfBuilder {
    /// Emite todos os XObjects de imagem pré-processados para o builder.
    ///
    /// Para PNG com alpha: emite /SMask (canal alpha) antes do XObject principal
    /// (canal RGB), para que o SMask apareça antes no ficheiro PDF — o dicionário
    /// do XObject principal referencia o ID do SMask por forward reference.
    /// P263 — Emite objects Function + Shading + Pattern para cada
    /// gradient único pré-processado por `scan_all_gradients`.
    ///
    /// `next_sub_id`: contador de IDs allocaveis para sub-Functions
    /// (Type 3 stitching). Os IDs alocados por gradient (3×N) **não
    /// incluem** as sub-Functions; estas são alocadas em `next_sub_id`
    /// (que deve apontar para zone de IDs livre pós-todos os outros).
    pub(super) fn emit_gradient_objects(
        &mut self,
        grad_objs: Vec<GradientObject>,
        page_dimensions: &[(f64, f64)],
        next_sub_id: &mut usize,
        icc_profile_id: Option<usize>,
    ) {
        for go in grad_objs {
            let GradientObject {
                kind,
                function_id,
                shading_id,
                pattern_id,
                parent_bbox_at_emit,
                paint_bbox_at_emit,
                page_height_at_emit,
                conic_matrix,
                conic_domain,
            } = go;
            let spot_pdf_bbox = paint_bbox_at_emit.map(|rect| {
                (rect.x.0, page_height_at_emit - rect.y.0 - rect.h.0, rect.w.0, rect.h.0)
            });
            let (page_w, page_h) = page_dimensions.first().copied().unwrap_or((
                pdf_defaults::A4_FALLBACK_WIDTH_ROUNDED,
                pdf_defaults::A4_FALLBACK_HEIGHT_ROUNDED,
            ));
            // P273.6 — bbox real do Layouter substitui page_bbox 3γ.1 quando
            // disponível; fallback page_bbox preserved P273.5.
            let effective_parent_bbox: (f32, f32, f32, f32) =
                if let Some(rect) = parent_bbox_at_emit {
                    (
                        rect.x.0 as f32,
                        rect.y.0 as f32,
                        (rect.x.0 + rect.w.0) as f32,
                        (rect.y.0 + rect.h.0) as f32,
                    )
                } else {
                    (0.0, 0.0, page_w as f32, page_h as f32)
                };
            // P265 + P268 — branching Linear / Radial / Conic.
            match &kind {
                GradientObjectKind::Linear(linear) => {
                    use typst_core::entities::layout_types::ColorSpace;
                    let (x0, y0, x1, y1) = compute_axial_coords(
                        linear.angle.to_rad(),
                        0.0,
                        0.0,
                        page_w,
                        page_h,
                    );
                    // P273.5 + P273.6 — quando relative=Parent, exercita
                    // apply_parent_transform com effective_parent_bbox:
                    // - P273.6: bbox real do Layouter (Block save/restore) quando disponível.
                    // - P273.5 fallback: page_bbox identity (gradient top-level).
                    let relative = resolve_relative(linear.relative);
                    let (x0, y0, x1, y1) = if relative
                        == typst_core::entities::gradient::RelativeTo::Parent
                    {
                        let local = (
                            (x0 / page_w) as f32,
                            (y0 / page_h) as f32,
                            (x1 / page_w) as f32,
                            (y1 / page_h) as f32,
                        );
                        let (tx0, ty0, tx1, ty1) =
                            apply_parent_transform(local, Some(effective_parent_bbox));
                        (tx0 as f64, ty0 as f64, tx1 as f64, ty1 as f64)
                    } else {
                        (x0, y0, x1, y1)
                    };
                    // P1483 — /Separation mantém a tinta como único canal da função.
                    if let ColorSpace::Spot(colorant) = &linear.space {
                        let endpoints = linear.stops.len() == 2
                            && linear.stops[0]
                                .offset
                                .as_ref()
                                .map(|r| r.get())
                                .unwrap_or(0.0)
                                .abs()
                                < 1e-9
                            && (linear.stops[1]
                                .offset
                                .as_ref()
                                .map(|r| r.get())
                                .unwrap_or(1.0)
                                - 1.0)
                                .abs()
                                < 1e-9;
                        let count = if endpoints { 2 } else { 16 };
                        let stops: Vec<f32> = (0..count)
                            .map(|i| {
                                sampled_spot_tint(
                                    linear.sample(i as f32 / (count - 1) as f32),
                                )
                            })
                            .collect();
                        let (function, sub_objects) =
                            emit_function_dict_spot(&stops, function_id, next_sub_id);
                        for (id, object) in sub_objects {
                            self.add(id, object);
                        }
                        self.add(function_id, function);
                        let (x0, y0, x1, y1) = spot_pdf_bbox
                            .map(|(bx, by, bw, bh)| {
                                compute_axial_coords(
                                    linear.angle.to_rad(),
                                    bx,
                                    by,
                                    bw,
                                    bh,
                                )
                            })
                            .unwrap_or((x0, y0, x1, y1));
                        self.add(shading_id, format!(
                            "<< /ShadingType 2 /ColorSpace {} /Coords [{x0:.3} {y0:.3} {x1:.3} {y1:.3}] /Function {function_id} 0 R /Extend [true true] >>",
                            pdf_spot_space(colorant, icc_profile_id)
                        ));
                    } else if linear.space == ColorSpace::Cmyk {
                        let stops_cmyk = multispace_sample_stops_linear_cmyk(linear, 16);
                        let shading_dict = format!(
                            "<< /ShadingType 2 /ColorSpace /DeviceCMYK \
                               /Coords [{:.3} {:.3} {:.3} {:.3}] \
                               /Function {} 0 R /Extend [false false] >>",
                            x0, y0, x1, y1, function_id,
                        );
                        let (func_dict, sub_objs) = emit_function_dict_cmyk(
                            &stops_cmyk,
                            function_id,
                            next_sub_id,
                        );
                        for (sub_id, sub_dict) in sub_objs {
                            self.add(sub_id, sub_dict);
                        }
                        self.add(function_id, func_dict);
                        self.add(shading_id, shading_dict);
                    } else {
                        // P270.1 pipeline + P274 adaptive N (N=16 baseline
                        // preservado para low-contrast; N=32/64 para
                        // moderate/high contrast — banding suppression).
                        let n = adaptive_n_for_stops(&linear.stops, linear.space.clone());
                        let stops = multispace_sample_stops(linear, n);
                        let shading_dict = format!(
                            "<< /ShadingType 2 /ColorSpace /DeviceRGB \
                               /Coords [{:.3} {:.3} {:.3} {:.3}] \
                               /Function {} 0 R /Extend [false false] >>",
                            x0, y0, x1, y1, function_id,
                        );
                        let (func_dict, sub_objs) =
                            emit_function_dict(&stops, function_id, next_sub_id);
                        for (sub_id, sub_dict) in sub_objs {
                            self.add(sub_id, sub_dict);
                        }
                        self.add(function_id, func_dict);
                        self.add(shading_id, shading_dict);
                    }
                }
                GradientObjectKind::Radial(radial) => {
                    use typst_core::entities::layout_types::ColorSpace;
                    // P269 — passa focal_center/focal_radius reais.
                    // Defaults (focal=center, fr=0) preservam comportamento P265.
                    let (x0, y0, r0, x1, y1, r1) = compute_radial_coords(
                        radial.center,
                        radial.radius,
                        radial.focal_center,
                        radial.focal_radius,
                        page_w,
                        page_h,
                    );
                    // P273.5 + P273.6 — paridade Linear; usa effective_parent_bbox.
                    let relative = resolve_relative(radial.relative);
                    let (x0, y0, x1, y1) = if relative
                        == typst_core::entities::gradient::RelativeTo::Parent
                    {
                        let local = (
                            (x0 / page_w) as f32,
                            (y0 / page_h) as f32,
                            (x1 / page_w) as f32,
                            (y1 / page_h) as f32,
                        );
                        let (tx0, ty0, tx1, ty1) =
                            apply_parent_transform(local, Some(effective_parent_bbox));
                        (tx0 as f64, ty0 as f64, tx1 as f64, ty1 as f64)
                    } else {
                        (x0, y0, x1, y1)
                    };
                    // P1483 — função radial spot, com uma única componente.
                    if let ColorSpace::Spot(colorant) = &radial.space {
                        let stops: Vec<f32> = (0..16)
                            .map(|i| sampled_spot_tint(radial.sample(i as f32 / 15.0)))
                            .collect();
                        let (function, sub_objects) =
                            emit_function_dict_spot(&stops, function_id, next_sub_id);
                        for (id, object) in sub_objects {
                            self.add(id, object);
                        }
                        self.add(function_id, function);
                        let (x0, y0, r0, x1, y1, r1) = spot_pdf_bbox
                            .map(|(bx, by, bw, bh)| {
                                let (a, b, c, d, e, f) = compute_radial_coords(
                                    radial.center,
                                    radial.radius,
                                    radial.focal_center,
                                    radial.focal_radius,
                                    bw,
                                    bh,
                                );
                                (a + bx, b + by, c, d + bx, e + by, f)
                            })
                            .unwrap_or((x0, y0, r0, x1, y1, r1));
                        self.add(shading_id, format!(
                            "<< /ShadingType 3 /ColorSpace {} /Coords [{x0:.3} {y0:.3} {r0:.3} {x1:.3} {y1:.3} {r1:.3}] /Function {function_id} 0 R /Extend [true true] >>",
                            pdf_spot_space(colorant, icc_profile_id)
                        ));
                    } else if radial.space == ColorSpace::Cmyk {
                        let stops_cmyk = multispace_sample_stops_radial_cmyk(radial, 16);
                        let shading_dict = format!(
                            "<< /ShadingType 3 /ColorSpace /DeviceCMYK \
                               /Coords [{:.3} {:.3} {:.3} {:.3} {:.3} {:.3}] \
                               /Function {} 0 R /Extend [true true] >>",
                            x0, y0, r0, x1, y1, r1, function_id,
                        );
                        let (func_dict, sub_objs) = emit_function_dict_cmyk(
                            &stops_cmyk,
                            function_id,
                            next_sub_id,
                        );
                        for (sub_id, sub_dict) in sub_objs {
                            self.add(sub_id, sub_dict);
                        }
                        self.add(function_id, func_dict);
                        self.add(shading_id, shading_dict);
                    } else {
                        // P270.1 pipeline + P274 adaptive N (paridade Linear).
                        let n = adaptive_n_for_stops(&radial.stops, radial.space.clone());
                        let stops = multispace_sample_stops_radial(radial, n);
                        let shading_dict = format!(
                            "<< /ShadingType 3 /ColorSpace /DeviceRGB \
                               /Coords [{:.3} {:.3} {:.3} {:.3} {:.3} {:.3}] \
                               /Function {} 0 R /Extend [true true] >>",
                            x0, y0, r0, x1, y1, r1, function_id,
                        );
                        let (func_dict, sub_objs) =
                            emit_function_dict(&stops, function_id, next_sub_id);
                        for (sub_id, sub_dict) in sub_objs {
                            self.add(sub_id, sub_dict);
                        }
                        self.add(function_id, func_dict);
                        self.add(shading_id, shading_dict);
                    }
                }
                GradientObjectKind::Conic(conic) => {
                    use typst_core::entities::layout_types::ColorSpace;
                    let mesh = emit_conic_mesh(conic, conic_domain);
                    let (colorspace, components, c0, c1) =
                        if let ColorSpace::Spot(colorant) = &conic.space {
                            (pdf_spot_space(colorant, icc_profile_id), 1, "[0]", "[1]")
                        } else if conic.space == ColorSpace::Cmyk {
                            ("/DeviceCMYK".to_string(), 4, "[0 0 0 0]", "[1 1 1 1]")
                        } else {
                            ("/DeviceRGB".to_string(), 3, "[0 0 0]", "[1 1 1]")
                        };
                    let decode_array = format!(
                        "[{:.12} {:.12} {:.12} {:.12}{}]",
                        mesh.decode[0],
                        mesh.decode[1],
                        mesh.decode[2],
                        mesh.decode[3],
                        " 0 1".repeat(components)
                    );
                    let stream = mesh.bytes;
                    let len = stream.len();
                    let header = format!(
                        "<< /ShadingType 6 /ColorSpace {} \
                           /BitsPerCoordinate 16 /BitsPerComponent 8 \
                           /BitsPerFlag 8 \
                           /Decode {} \
                           /Length {} >>\nstream\n",
                        colorspace, decode_array, len,
                    );
                    let mut shading_bytes = header.into_bytes();
                    shading_bytes.extend_from_slice(&stream);
                    shading_bytes.extend_from_slice(b"\nendstream");
                    // Type 6 Coons não usa Function dict (cores nos corner colors
                    // do stream). Function vazio preserva numbering.
                    self.add(
                        function_id,
                        format!(
                            "<< /FunctionType 2 /Domain [0 1] /C0 {} /C1 {} /N 1 >>",
                            c0, c1,
                        ),
                    );
                    self.add_bytes(shading_id, shading_bytes);
                }
            };
            // Emit Pattern dict.
            let matrix = if let Some(m) = conic_matrix {
                if m.b == 0.0 && m.c == 0.0 {
                    format!("[{:.3} 0 0 {:.3} {:.3} {:.3}]", m.a, m.d, m.tx, m.ty)
                } else {
                    format!(
                        "[{:.6} {:.6} {:.6} {:.6} {:.6} {:.6}]",
                        m.a, m.b, m.c, m.d, m.tx, m.ty
                    )
                }
            } else {
                "[1 0 0 1 0 0]".to_string()
            };
            let pattern_dict = format!(
                "<< /PatternType 2 /Shading {} 0 R /Matrix {} >>",
                shading_id, matrix,
            );
            self.add(pattern_id, pattern_dict);
        }
    }

    pub(super) fn emit_image_xobjects(&mut self, xobjects: Vec<ImageXObject>) {
        for xobj in xobjects {
            match xobj {
                ImageXObject::Jpeg { data, main_obj_id, iw, ih, icc_profile_id } => {
                    let cs = jpeg_color_space(&data);
                    self.add_bytes(
                        main_obj_id,
                        build_jpeg_xobject(&data, iw, ih, cs, icc_profile_id),
                    );
                }
                ImageXObject::Png { payload, main_obj_id, smask_obj_id } => {
                    // Emitir /SMask antes do XObject principal.
                    if let (Some(smask_id), Some(alpha)) =
                        (smask_obj_id, &payload.alpha_data_compressed)
                    {
                        self.add_bytes(
                            smask_id,
                            build_png_smask_xobject(payload.width, payload.height, alpha),
                        );
                    }
                    self.add_bytes(
                        main_obj_id,
                        build_png_rgb_xobject(&payload, smask_obj_id),
                    );
                }
            }
        }
    }
}
