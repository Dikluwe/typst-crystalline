//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/svg/tests.md
//! @prompt-hash 8857f6ce
//! @layer L3
//! @updated 2026-09-26

use super::super::gradients::svg_adaptive_stops;
use super::geometry::rounded_rect_path_data;
use super::glyphs::SvgGlyphPathBuilder;
use super::paints::{
    adaptive_offsets_use_ratio_repr, fmt_ratio_repr, write_svg_gradient_stops,
};
use crate::font_variant::text_style_to_font_variant;
use base64::Engine;
use std::sync::Arc;
use ttf_parser::{Face, GlyphId};
use typst_core::entities::frame_paint::FramePaint as Paint;
use typst_core::entities::frame_paint::FrameStroke as Stroke;
use typst_core::entities::geometry::PathItem;
use typst_core::entities::layout_types::{Color, LinkTarget, Size, TransformMatrix};

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
        logical_number: 1,
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
        logical_number: 1,
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
        logical_number: 1,
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
fn p1454_html_frame_preserva_paints_sem_reescrita() {
    let page = p1227_page(vec![
        p1227_rect(Some(Paint::Solid(Color::rgb(255, 65, 54))), None),
        FrameItem::Shape {
            pos: Point { x: Pt(50.0), y: Pt(7.0) },
            kind: ShapeKind::Rect,
            width: 40.0,
            height: 20.0,
            fill: Some(Paint::Solid(Color::rgb(255, 0, 0))),
            stroke: None,
            fill_rule: Default::default(),
            parent_bbox_at_emit: None,
        },
    ]);

    let svg = export_svg_html_frame_with_fonts(
        &page,
        &SvgOptions::default(),
        &[],
        &SvgGlyphFontContext::empty(),
    );

    assert!(svg.contains("fill=\"#ff4136\""), "paint #ff4136 foi reescrito: {svg}");
    assert!(svg.contains("fill=\"#ff0000\""), "paint #ff0000 foi perdido: {svg}");
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
    use typst_core::entities::frame_paint::FramePaint as Paint;
    use typst_core::entities::frame_paint::FrameStroke as Stroke;
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
    use typst_core::entities::frame_paint::FramePaint as Paint;
    use typst_core::entities::frame_paint::FrameStroke as Stroke;
    use typst_core::entities::geometry::{DashLength, DashPattern, LineCap, LineJoin};
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
        logical_number: 1,
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
            Gradient::linear_with_space(stops(), Angle::deg(0.0), space.clone())
        } else {
            Gradient::radial_with_space(
                stops(),
                Axes::new(Ratio(0.5), Ratio(0.5)),
                Ratio(0.5),
                space.clone(),
            )
        };
        let sampled = svg_adaptive_stops(&gradient);
        assert_eq!(sampled.len(), expected_count, "{kind}/{space:?}");
        let penultimate = &sampled[sampled.len() - 2];
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
            Gradient::linear_with_space(stops(), Angle::deg(25.0), ColorSpace::LinearRgb)
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
        let gradient =
            Gradient::linear_with_space(stops(), Angle::deg(25.0), space.clone());
        assert!(
            adaptive_offsets_use_ratio_repr(&gradient),
            "Linear/{space:?} deve serializar todos os offsets via Ratio::repr"
        );
    }

    let luma = Gradient::linear_with_space(stops(), Angle::deg(25.0), ColorSpace::Luma);
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
                Gradient::linear_with_space(stops(), Angle::deg(25.0), space.clone()),
            ),
            (
                "radial",
                "radialGradient",
                Gradient::radial_with_space(
                    stops(),
                    Axes::new(Ratio(0.4), Ratio(0.6)),
                    Ratio(0.7),
                    space.clone(),
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
            Gradient::linear_with_space(stops(), Angle::deg(25.0), space.clone())
        } else {
            let mut gradient =
                Gradient::radial(stops(), Axes::new(Ratio(0.4), Ratio(0.6)), Ratio(0.7));
            if let Gradient::Radial(value) = &mut gradient {
                Arc::make_mut(value).space = space.clone();
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
    let linear = Gradient::linear_with_space(stops(), Angle::deg(0.0), ColorSpace::Cmyk);
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
        assert!(svg.contains("data-crystalline-fill-fallback=\"gradient-color-space\""));
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
    use typst_core::entities::frame_paint::TilingFrame;
    use typst_core::entities::layout_types::{Angle, Frame};
    let value = TilingFrame {
        frame: Frame {
            size: Size { width: Pt(4.0), height: Pt(6.0) },
            items: vec![FrameItem::Shape {
                pos: Point::ZERO,
                kind: ShapeKind::Rect,
                width: 4.0,
                height: 6.0,
                fill: Some(Paint::Solid(Color::rgb(0, 128, 255))),
                stroke: None,
                parent_bbox_at_emit: None,
                fill_rule: Default::default(),
            }],
        },
        spacing: Size { width: Pt(1.0), height: Pt(2.0) },
        offset: Point::ZERO,
        angle: Angle::deg(0.0),
        relative: typst_core::entities::gradient::RelativeTo::Self_,
    };
    let svg = export_svg(
        &p1227_page(vec![p1227_rect(
            Some(Paint::Tiling(std::sync::Arc::new(value))),
            None,
        )]),
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
    // Origem/licença: 03_infra/fixtures/fonts/LICENSE-NotoSans.md.
    let font_data =
        include_bytes!("../../../fixtures/fonts/NotoSans-Regular.ttf") as &[u8];
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
        logical_number: 1,
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
            bidi: None,
            font: None,
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
        paint_root: false,
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
    let svg = export_svg(&p1246_group(ShapeKind::Rect, 8.0, 6.0), &SvgOptions::default());
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
    let ellipse =
        export_svg(&p1246_group(ShapeKind::Ellipse, 20.0, 10.0), &SvgOptions::default());
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
        paint_root: false,
        pos: Point { x: Pt(2.0), y: Pt(3.0) },
        matrix: TransformMatrix::default(),
        clip_mask: Some(ShapeKind::Ellipse),
        inner_width: 8.0,
        inner_height: 8.0,
        items: vec![p1227_rect(Some(Paint::Solid(Color::rgb(0, 0, 255))), None)],
    };
    let page = p1227_page(vec![FrameItem::Group {
        paint_root: false,
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
        alt: None,
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
        DynamicImage, GenericImageView, ImageBuffer, ImageFormat as RasterFormat, Rgba,
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
        assert!(svg.contains(&encoded), "payload/alpha deve permanecer byte-preservado");
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
    assert!(right[3] >= 120 && right[3] <= 136, "alpha deve permanecer 0.5: {right:?}");
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
        alt: None,
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
    assert!(top_left[2] > top_left[0], "azul deve ocupar o topo esquerdo: {top_left:?}");
    assert!(
        top_right[0] > top_right[2],
        "vermelho deve ocupar o topo direito: {top_right:?}"
    );
}

#[test]
fn p1248_imagens_preservam_payload_geometria_clip_orientacao_e_fallback() {
    use image::{DynamicImage, ImageBuffer, ImageFormat as RasterFormat, Rgba};
    use std::io::Cursor;
    let source =
        DynamicImage::ImageRgba8(ImageBuffer::from_pixel(2, 2, Rgba([9, 80, 170, 64])));
    let mut cursor = Cursor::new(Vec::new());
    source.write_to(&mut cursor, RasterFormat::Png).unwrap();
    let png = cursor.into_inner();
    let payload = base64::engine::general_purpose::STANDARD.encode(&png);
    let raster =
        export_svg(&p1227_page(vec![p1248_image(&png, 1, None)]), &SvgOptions::default());
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
    let oriented =
        export_svg(&p1227_page(vec![p1248_image(&png, 6, None)]), &SvgOptions::default());
    let nested_bytes = br##"<svg xmlns="http://www.w3.org/2000/svg"><rect fill="#ff0000"/><circle fill="#0000ff" fill-opacity="0.5"/></svg>"##;
    let nested_payload = base64::engine::general_purpose::STANDARD.encode(nested_bytes);
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
                && unknown.contains("data-crystalline-image-fallback=\"unknown-format\"")
                && !unknown.contains("<image")
        };
    assert!(valid(&raster, &clipped, &oriented, &nested, &unknown));
}

#[test]
fn p1254_svg_executa_grafo_repetido_e_recortado_do_layout() {
    use std::sync::Arc;
    use typst_core::entities::layout_types::{Length, Size};
    use typst_core::entities::paint::Paint;
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
    assert_eq!(first.matches("fill=\"#ff0000\"").count(), 1);
    assert!(first.contains("patternUnits=\"userSpaceOnUse\""));
    assert_eq!(first.matches("fill=\"#0000ff\"").count(), 1);
    assert!(first.find("fill=\"#ff0000\"") < first.find("fill=\"#0000ff\""));
}

fn p1249_fixture() -> (FrameItem, FontKey, GlyphFontRequest) {
    let bytes = include_bytes!("../../../fixtures/fonts/NewCMMath-Book.otf").to_vec();
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
        include_bytes!("../../../fixtures/fonts/NewCMMath-Book.otf").to_vec();
    let beta_bytes =
        include_bytes!("../../../fixtures/fonts/NimbusSans-Regular.otf").to_vec();
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
        paint_root: false,
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
        include_bytes!("../../../fixtures/fonts/NewCMMath-Book.otf").to_vec();
    let sans_bytes =
        include_bytes!("../../../fixtures/fonts/NimbusSans-Regular.otf").to_vec();
    let math = Face::parse(&math_bytes, 0).unwrap();
    let gid_a = math.glyph_index('A').unwrap().0;
    let gid_sum = math.glyph_index('∑').unwrap().0;
    assert_ne!(gid_a, gid_sum);
    let family = FontList::single(EcoString::from("variant witness"));
    let normal = (family.clone(), FontVariant::default(), FontVariations::default());
    let mut bold_style = typst_core::entities::layout_types::TextStyle::regular(Pt(20.0));
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
        let mut builder = SvgGlyphPathBuilder::new(size / f64::from(math.units_per_em()));
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

#[test]
fn prepared_nested_cells_keep_resources_and_drop_decorative_links() {
    use std::sync::Arc;
    use typst_core::entities::frame_paint::TilingFrame;
    use typst_core::entities::gradient::RelativeTo;
    use typst_core::entities::layout_types::{Angle, Frame, LinkTarget};
    let size = Size { width: Pt(8.0), height: Pt(9.0) };
    let cell = |items| {
        Paint::Tiling(Arc::new(TilingFrame {
            frame: Frame { size, items },
            spacing: Size { width: Pt(2.0), height: Pt(3.0) },
            offset: Point::ZERO,
            angle: Angle::deg(0.0),
            relative: RelativeTo::Self_,
        }))
    };
    let nested = cell(vec![p1227_rect(Some(Paint::Solid(Color::rgb(255, 0, 0))), None)]);
    let outer = cell(vec![FrameItem::Link {
        target: LinkTarget::Url("https://example.invalid/decorative".into()),
        pos: Point::ZERO,
        size,
        items: vec![p1227_rect(Some(nested), None)],
    }]);
    let page = p1227_page(vec![p1227_rect(
        Some(outer.clone()),
        Some(Stroke { paint: outer, thickness: 3.0, ..Stroke::default() }),
    )]);
    let svg = export_svg(&page, &SvgOptions::default());
    assert_eq!(svg.matches("<pattern ").count(), 2);
    assert_eq!(svg.matches("<clipPath ").count(), 2);
    assert!(svg.contains("fill=\"url(#p0)\"") && svg.contains("stroke=\"url(#p0)\""));
    assert!(svg.contains("fill=\"url(#p1)\""));
    assert!(!svg.contains("<a ") && !svg.contains("example.invalid"));
    assert!(!svg.contains("data-crystalline-fill-fallback"));
}

#[test]
fn svg_link_overflow_keeps_visual_ink_outside_nominal_anchor() {
    let visual = FrameItem::Group {
        paint_root: false,
        pos: Point { x: Pt(8.0), y: Pt(8.0) },
        matrix: TransformMatrix::translate(20.0, 0.0),
        clip_mask: None,
        inner_width: 5.0,
        inner_height: 5.0,
        items: vec![FrameItem::Shape {
            pos: Point::ZERO,
            kind: ShapeKind::Rect,
            width: 5.0,
            height: 5.0,
            fill: Some(Paint::Solid(Color::rgb(255, 0, 0))),
            stroke: None,
            fill_rule: Default::default(),
            parent_bbox_at_emit: None,
        }],
    };
    let page = p1227_page(vec![FrameItem::Link {
        target: LinkTarget::Url("https://example.org/outer".into()),
        items: vec![visual],
        pos: Point { x: Pt(8.0), y: Pt(8.0) },
        size: Size { width: Pt(10.0), height: Pt(10.0) },
    }]);
    let svg = export_svg(&page, &SvgOptions::default());
    assert_eq!(svg.matches("<a ").count(), 1);
    let start = svg.find("<a ").expect("the nominal hyperlink must remain");
    let anchor = svg[start..].split("</a>").next().expect("closed anchor");
    assert!(anchor.contains("xlink:href=\"https://example.org/outer\""));
    assert!(anchor.contains("x=\"8\" y=\"8\" width=\"10\" height=\"10\""));
    assert!(anchor.contains("data-crystalline-link-hit-area=\"true\""));
    assert!(
        !anchor.contains("fill=\"#ff0000\""),
        "opaque overflow must not activate the outer URL through SVG ancestry: {svg}"
    );
    assert!(
        svg[..start].contains("fill=\"#ff0000\""),
        "overflow paint must survive outside the anchor"
    );
}

#[cfg(test)]
mod readiness_variable_contours {
    use super::*;
    use typst_core::entities::world_types::Font;

    const VARIABLE: &[u8] =
        include_bytes!("../../../fixtures/fonts/UbuntuSans-Variable.ttf");

    fn original_contour(data: &[u8], weight: Option<f32>) -> String {
        let mut face = Face::parse(data, 0).expect("portable original face");
        if let Some(weight) = weight {
            assert!(face
                .set_variation(ttf_parser::Tag::from_bytes(b"wght"), weight)
                .is_some());
        }
        let gid = face.glyph_index('A').unwrap();
        let mut path = SvgGlyphPathBuilder::new(17.0 / f64::from(face.units_per_em()));
        face.outline_glyph(gid, &mut path).expect("actual A outline");
        path.finish()
    }

    fn input(
        data: &[u8],
        weight: u16,
        explicit: Option<f32>,
        x: f64,
    ) -> (FrameItem, FontKey) {
        let family = FontList::single(
            crate::fonts::font_info_from_bytes(data, 0)
                .expect("portable face metadata")
                .family
                .into(),
        );
        let mut style = TextStyle::regular(Pt(17.0));
        style.font = Some(family.clone());
        style.weight = Some(weight);
        style.variations = explicit.map(|value| FontVariations(vec![(*b"wght", value)]));
        let key = (
            (
                family,
                text_style_to_font_variant(&style),
                style.variations.clone().unwrap_or_default(),
            ),
            data.to_vec(),
        );
        let face = Face::parse(data, 0).unwrap();
        let gid = face.glyph_index('A').unwrap();
        let item = FrameItem::TextShaped {
            pos: Point { x: Pt(x), y: Pt(40.0) },
            glyphs: vec![ShapedGlyph {
                glyph_id: gid.0,
                x_advance: i32::from(face.glyph_hor_advance(gid).unwrap()),
                x_offset: 0,
                y_offset: 0,
                cluster: 0,
                char_code: 'A',
            }],
            style,
            text: "A".into(),
            units_per_em: face.units_per_em(),
            font: Some(Font::from_data(data.to_vec())),
            bidi: None,
        };
        (item, key)
    }

    fn attr<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
        let marker = format!(" {name}=\"");
        let tail = tag.split_once(&marker)?.1;
        Some(tail.split_once('"')?.0)
    }

    fn used_contours(svg: &str) -> Vec<String> {
        let mut symbols = std::collections::HashMap::new();
        for symbol in svg.split("<symbol").skip(1) {
            let (head, body) = symbol.split_once('>').unwrap();
            let id = attr(head, "id").unwrap();
            let path = body.split_once("<path").unwrap().1.split_once('>').unwrap().0;
            symbols.insert(id.to_owned(), attr(path, "d").unwrap().to_owned());
        }
        svg.split("<use")
            .skip(1)
            .map(|tail| {
                let tag = tail.split_once('>').unwrap().0;
                let reference =
                    attr(tag, "xlink:href").or_else(|| attr(tag, "href")).unwrap();
                symbols
                    .get(reference.strip_prefix('#').unwrap())
                    .expect("referenced outline")
                    .clone()
            })
            .collect()
    }

    #[test]
    fn readiness_svg_variable_shaped_contour_preserves_original_instance() {
        let (item, key) = input(VARIABLE, 700, None, 20.0);
        let svg = export_svg_with_fonts(
            &p1227_page(vec![item]),
            &SvgOptions::default(),
            &[key],
        );
        // The reference configures the original font directly. It does not use
        // resource materialization, binding lookup, or an exporter instance helper.
        assert_eq!(used_contours(&svg), vec![original_contour(VARIABLE, Some(700.0))]);
    }

    #[test]
    fn readiness_svg_variable_instances_and_explicit_override_do_not_alias() {
        let (a, ka) = input(VARIABLE, 400, None, 10.0);
        let (b, kb) = input(VARIABLE, 700, None, 35.0);
        let (c, kc) = input(VARIABLE, 400, Some(700.0), 60.0);
        let svg = export_svg_with_fonts(
            &p1227_page(vec![a, b, c]),
            &SvgOptions::default(),
            &[ka, kb, kc],
        );
        let regular = original_contour(VARIABLE, Some(400.0));
        let bold = original_contour(VARIABLE, Some(700.0));
        assert_ne!(regular, bold, "fixture has distinct actual contours");
        assert_eq!(used_contours(&svg), vec![regular, bold.clone(), bold]);
    }

    #[test]
    fn readiness_svg_variable_direct_glyph_preserves_original_instance() {
        let (item, key) = input(VARIABLE, 700, None, 20.0);
        let FrameItem::TextShaped { pos, glyphs, style, .. } = item else {
            unreachable!()
        };
        let request = GlyphFontRequest::new('A', &style);
        let context = SvgGlyphFontContext::from_resolutions([(request, key.0.clone())]);
        let item = FrameItem::Glyph {
            pos,
            glyph_id: glyphs[0].glyph_id,
            x_advance: Pt(12.0),
            size: style.size,
            style,
            base_char: 'A',
        };
        let svg = export_svg_with_fonts_and_contexts(
            &p1227_page(vec![item]),
            &SvgOptions::default(),
            &[key],
            &SvgDestinationContext::empty(),
            &context,
        );
        assert_eq!(used_contours(&svg), vec![original_contour(VARIABLE, Some(700.0))]);
    }

    #[test]
    fn readiness_svg_static_face_ignores_inapplicable_variation() {
        let data = include_bytes!("../../../fixtures/fonts/DejaVuSans.ttf");
        let (item, key) = input(data, 400, Some(700.0), 20.0);
        let svg = export_svg_with_fonts(
            &p1227_page(vec![item]),
            &SvgOptions::default(),
            &[key],
        );
        assert_eq!(used_contours(&svg), vec![original_contour(data, None)]);
    }
}
