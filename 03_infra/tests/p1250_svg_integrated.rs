//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/p1250_svg_integrated.md
//! @prompt-hash 478b85ee
//! @layer L3
//! @updated 2026-08-28
//!
//! Campanha integrada P1250. Não é consumer produtivo.

use std::sync::Arc;

use ecow::EcoString;
use typst_core::entities::corners::Corners;
use typst_core::entities::font_book::FontVariant;
use typst_core::entities::font_list::FontList;
use typst_core::entities::font_variations::FontVariations;
use typst_core::entities::geometry::{
    DashLength, DashPattern, LineCap, LineJoin, ShapeKind, Stroke,
};
use typst_core::entities::label::Label;
use typst_core::entities::layout_types::{
    Color, FrameItem, LinkTarget, Page, Point, Pt, Rect, Size, TextStyle, TransformMatrix,
};
use typst_core::entities::paint::Paint;
use typst_infra::export::{
    export_svg, export_svg_with_context, export_svg_with_fonts_and_contexts, FontKey,
    GlyphFontRequest, SvgDestinationContext, SvgGlyphFontContext, SvgOptions,
};

fn page(items: Vec<FrameItem>) -> Page {
    Page {
        width: 160.0,
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

fn rect(fill: Option<Paint>, stroke: Option<Stroke>) -> FrameItem {
    FrameItem::Shape {
        pos: Point::ZERO,
        kind: ShapeKind::Rect,
        width: 40.0,
        height: 30.0,
        fill,
        stroke,
        fill_rule: Default::default(),
        parent_bbox_at_emit: None,
    }
}

fn link(label: &str, items: Vec<FrameItem>) -> FrameItem {
    FrameItem::Link {
        target: LinkTarget::Destination(Label(label.into())),
        items,
        pos: Point { x: Pt(1.0), y: Pt(2.0) },
        size: Size { width: Pt(30.0), height: Pt(20.0) },
    }
}

fn tiling() -> Paint {
    use typst_core::entities::tiling::{Tiling, TilingBody};
    let mut value = Tiling::new(TilingBody::Color(Color::rgba(0, 128, 255, 128)));
    value.size = Some(Size { width: Pt(4.0), height: Pt(6.0) });
    value.spacing = Some(Size { width: Pt(1.0), height: Pt(2.0) });
    Paint::Tiling(value)
}

fn gradient() -> Paint {
    use typst_core::entities::color::ColorSpace;
    use typst_core::entities::gradient::{Gradient, GradientStop};
    use typst_core::entities::layout_types::{Angle, Ratio};
    Paint::Gradient(Gradient::linear_with_space(
        vec![
            GradientStop::new(Color::rgba(255, 0, 0, 128), Ratio(0.0)),
            GradientStop::new(Color::rgba(0, 255, 0, 128), Ratio(1.0)),
        ],
        Angle::deg(0.0),
        ColorSpace::Srgb,
    ))
}

fn image(data: &[u8], orientation: u32, clip_rect: Option<Rect>) -> FrameItem {
    FrameItem::Image {
        pos: Point { x: Pt(7.0), y: Pt(11.0) },
        data: Arc::new(data.to_vec()),
        width: Pt(6.0),
        height: Pt(2.0),
        intrinsic_width: 3,
        intrinsic_height: 2,
        clip_rect,
        orientation,
    }
}

fn glyph_fixture() -> (FrameItem, FontKey, GlyphFontRequest) {
    let bytes = include_bytes!("../fixtures/fonts/NewCMMath-Book.otf").to_vec();
    let face = ttf_parser::Face::parse(&bytes, 0).unwrap();
    let glyph_id = face.glyph_index('∑').unwrap().0;
    let family = FontList::single(EcoString::from("New Computer Modern Math"));
    let mut style = TextStyle::regular(Pt(20.0));
    style.font = Some(family.clone());
    style.math = true;
    style.fill = Some(Color::rgba(255, 0, 0, 64));
    let request = GlyphFontRequest::new('∑', &style);
    let key = ((family, FontVariant::default(), FontVariations::default()), bytes);
    (
        FrameItem::Glyph {
            pos: Point { x: Pt(31.0), y: Pt(47.0) },
            glyph_id,
            x_advance: Pt(17.0),
            size: Pt(20.0),
            style,
            base_char: '∑',
        },
        key,
        request,
    )
}

fn refs_closed(svg: &str) -> bool {
    let values = |needle: &str| -> Vec<String> {
        svg.split(needle)
            .skip(1)
            .filter_map(|tail| tail.split('"').next())
            .map(str::to_owned)
            .collect()
    };
    let ids = values(" id=\"");
    if ids
        .iter()
        .any(|id| ids.iter().filter(|other| *other == id).count() != 1)
    {
        return false;
    }
    let mut refs = values("xlink:href=\"#");
    refs.extend(
        svg.split("url(#")
            .skip(1)
            .filter_map(|tail| tail.split(')').next())
            .map(str::to_owned),
    );
    refs.iter()
        .all(|target| ids.iter().filter(|id| *id == target).count() == 1)
}

fn corpus() -> [String; 7] {
    use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba};
    use std::io::Cursor;
    let mut png = Cursor::new(Vec::new());
    DynamicImage::ImageRgba8(ImageBuffer::from_fn(3, 2, |x, y| {
        Rgba([x as u8 * 100, y as u8 * 120, 40, 255])
    }))
    .write_to(&mut png, ImageFormat::Png)
    .unwrap();
    let raster = image(
        png.get_ref(),
        6,
        Some(Rect { x: Pt(8.0), y: Pt(11.0), w: Pt(4.0), h: Pt(2.0) }),
    );
    let destinations = SvgDestinationContext::from_destinations([(
        Label("local".into()),
        Point { x: Pt(90.0), y: Pt(70.0) },
    )]);
    let c01 = export_svg_with_context(
        &page(vec![FrameItem::Group {
            pos: Point { x: Pt(5.0), y: Pt(7.0) },
            matrix: TransformMatrix { a: 0.0, b: 1.0, c: -1.0, d: 0.0, tx: 3.0, ty: 4.0 },
            clip_mask: Some(ShapeKind::RoundedRect { radii: Corners::uniform(Pt(2.0)) }),
            inner_width: 30.0,
            inner_height: 20.0,
            items: vec![link("local", vec![rect(Some(tiling()), None)])],
        }]),
        &SvgOptions::default(),
        &destinations,
    );
    let c02 = export_svg(
        &page(vec![rect(Some(gradient()), None), raster.clone()]),
        &SvgOptions::default(),
    );
    let (glyph, font, request) = glyph_fixture();
    let glyph_ctx = SvgGlyphFontContext::from_resolutions([(request, font.0.clone())]);
    let c03 = export_svg_with_fonts_and_contexts(
        &page(vec![FrameItem::Group {
            pos: Point { x: Pt(2.0), y: Pt(4.0) },
            matrix: TransformMatrix { a: 0.0, b: 1.0, c: -1.0, d: 0.0, tx: 6.0, ty: 8.0 },
            clip_mask: None,
            inner_width: 100.0,
            inner_height: 100.0,
            items: vec![link("local", vec![glyph])],
        }]),
        &SvgOptions::default(),
        &[font],
        &destinations,
        &glyph_ctx,
    );
    let nested = br#"<svg xmlns="http://www.w3.org/2000/svg"><rect width="4" height="4" fill="red"/><circle cx="3" cy="2" r="1" fill="blue" fill-opacity=".5"/></svg>"#;
    let stroke = Stroke {
        paint: Paint::Solid(Color::rgb(0, 0, 0)),
        thickness: 4.0,
        dash: Some(DashPattern {
            array: vec![DashLength::Length(6.0), DashLength::Length(2.0)],
            phase: 1.0,
        }),
        cap: LineCap::Round,
        join: LineJoin::Bevel,
        ..Stroke::default()
    };
    let c04 = export_svg(
        &page(vec![image(nested, 1, None), rect(Some(tiling()), Some(stroke))]),
        &SvgOptions::default(),
    );
    let c05_local = export_svg_with_context(
        &page(vec![
            link("local", vec![rect(Some(Paint::Solid(Color::rgb(0, 0, 255))), None)]),
            raster,
        ]),
        &SvgOptions::default(),
        &destinations,
    );
    let c05_remote = export_svg_with_context(
        &page(vec![link(
            "remote",
            vec![rect(Some(Paint::Solid(Color::rgb(0, 0, 255))), None)],
        )]),
        &SvgOptions::default(),
        &destinations,
    );
    let (glyph, font, request) = glyph_fixture();
    let glyph_ctx = SvgGlyphFontContext::from_resolutions([(request, font.0.clone())]);
    let c06 = export_svg_with_fonts_and_contexts(
        &page(vec![
            glyph,
            rect(Some(gradient()), None),
            rect(Some(tiling()), None),
            FrameItem::Group {
                pos: Point::ZERO,
                matrix: TransformMatrix::default(),
                clip_mask: Some(ShapeKind::Ellipse),
                inner_width: 20.0,
                inner_height: 10.0,
                items: vec![link(
                    "local",
                    vec![rect(Some(Paint::Solid(Color::rgb(1, 2, 3))), None)],
                )],
            },
        ]),
        &SvgOptions::default(),
        &[font],
        &destinations,
        &glyph_ctx,
    );
    [c01, c02, c03, c04, c05_local, c05_remote, c06]
}

fn valid(v: &[String; 7]) -> bool {
    v[0].contains("<pattern")
        && v[0].contains("<clipPath")
        && v[0].contains("xlink:href=\"#crystalline-destination-0\"")
        && v[0].contains("matrix(0 1 -1 0 3 4)")
        && refs_closed(&v[0])
        && v[1].contains("stop-opacity=\"0.501960784\"")
        && v[1].contains("clip-path=\"url(#c0)\"")
        && v[1].contains("matrix(0 2 -6 0 13 11)")
        && refs_closed(&v[1])
        && v[2].contains("<symbol")
        && v[2].contains("rgba(255,0,0,0.251)")
        && v[2].contains("matrix(1 0 0 -1 31 47)")
        && refs_closed(&v[2])
        && v[3].contains("data:image/svg+xml")
        && v[3].contains("<pattern")
        && v[3].contains("stroke-linecap=\"round\"")
        && v[3].contains("stroke-dasharray=\"6 2\"")
        && refs_closed(&v[3])
        && refs_closed(&v[4])
        && v[5].contains("data-crystalline-link-fallback=\"destination-unknown\"")
        && !v[5].contains("xlink:href=\"#")
        && v[6].contains("<symbol")
        && v[6].contains("<linearGradient")
        && v[6].contains("<pattern")
        && v[6].contains("<clipPath")
        && v[6].contains("data-crystalline-destination")
        && refs_closed(&v[6])
}

#[test]
fn p1250_seis_corpus_e_mutacoes_integradas() {
    let baseline = corpus();
    assert!(valid(&baseline));
    assert_eq!(baseline, corpus(), "duas execuções devem ser byte-idênticas");
    for mutation in 1..=11 {
        let mut v = baseline.clone();
        match mutation {
            1 => v[6] = v[6].replacen("id=\"g0\"", "id=\"p0\"", 1),
            2 => v[0] = v[0].replacen("id=\"p0\"", "id=\"captured\"", 1),
            3 => v[0] = v[0].replace("xlink:href=\"#crystalline-destination-0\"", ""),
            4 => v[0] = v[0].replace("matrix(0 1 -1 0 3 4)", "matrix(1 0 0 1 0 0)"),
            5 => {
                v[3] = v[3].replace("stroke-linecap=\"round\"", "stroke-linecap=\"butt\"")
            }
            6 => {
                v[1] = v[1].replace("stop-opacity=\"0.501960784\"", "stop-opacity=\"1\"")
            }
            7 => v[1] = v[1].replace("clip-path=\"url(#c0)\"", ""),
            8 => v[2] = v[2].replacen("<symbol", "<discarded-symbol", 1),
            9 => v[3] = v[3].replace("data:image/svg+xml", "data:application/omitted"),
            10 => {
                v[5] = v[5].replace(
                    "data-crystalline-link-fallback=\"destination-unknown\"",
                    "xlink:href=\"#invented-cross-page\"",
                )
            }
            11 => v[2] = v[2].replace("rgba(255,0,0,0.251)", "rgba(255,0,0,1)"),
            _ => unreachable!(),
        }
        assert!(!valid(&v), "mutação integrada M{mutation:02} sobreviveu");
    }
}
