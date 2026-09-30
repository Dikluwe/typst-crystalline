//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/tests/p1483_pdf_spot_tests.md
//! @prompt-hash 93cc6bfb
//! @layer L3
//! @updated 2026-09-23

use super::*;
use std::sync::Arc;
use typst_core::entities::axes::Axes;
use typst_core::entities::color::{SpotColor, SpotColorant};
use typst_core::entities::geometry::{ShapeKind, Stroke};
use typst_core::entities::gradient::{Gradient, GradientStop};
use typst_core::entities::layout_types::{
    Angle, ColorSpace, FrameItem, Page, PagedDocument, Point, Pt, Ratio,
};
use typst_core::entities::paint::Paint;

fn colorant(name: Option<&str>, fallback: Color) -> Arc<SpotColorant> {
    Arc::new(SpotColorant { name: name.map(Into::into), fallback })
}

fn tint(colorant: &Arc<SpotColorant>, value: f64) -> Color {
    Color::Spot(SpotColor { colorant: colorant.clone(), tint: Ratio(value) })
}

fn rect(y: f64, fill: Option<Paint>, stroke: Option<Paint>) -> FrameItem {
    FrameItem::Shape {
        pos: Point { x: Pt(10.0), y: Pt(y) },
        kind: ShapeKind::Rect,
        width: 60.0,
        height: 20.0,
        fill,
        stroke: stroke.map(|paint| Stroke { paint, thickness: 2.0, ..Stroke::default() }),
        fill_rule: Default::default(),
        parent_bbox_at_emit: None,
    }
}

fn page(items: Vec<FrameItem>) -> Page {
    Page {
        width: 100.0,
        height: 120.0,
        numbering: None,
        logical_number: 1,
        supplement: Content::Empty,
        bleed: Default::default(),
        fill: Default::default(),
        background: vec![],
        foreground: vec![],
        items,
    }
}

#[test]
fn p1483_pdf_solid_fill_stroke_tints_and_reuse() {
    let brand = colorant(Some("Brand"), Color::rgb(255, 65, 54));
    let doc = PagedDocument::new(vec![page(vec![
        rect(10.0, Some(Paint::Solid(tint(&brand, 0.0))), None),
        rect(35.0, Some(Paint::Solid(tint(&brand, 0.5))), None),
        rect(60.0, Some(Paint::Solid(tint(&brand, 1.0))), None),
        rect(85.0, None, Some(Paint::Solid(tint(&brand, 0.5)))),
    ])]);
    let pdf = export_pdf(&doc, StreamMode::Verbose);
    let raw = String::from_utf8_lossy(&pdf);
    let stream = extract_page_content_streams_text(&pdf);
    assert!(raw.contains("/Separation") && raw.contains("/Brand"), "{raw}");
    assert_eq!(raw.matches("/Separation").count(), 1, "reuse same colorant resource");
    assert!(stream.contains(" scn") && stream.contains(" SCN"), "{stream}");
    assert!(stream.contains("0 scn") && stream.contains("1 scn"), "{stream}");
    assert!(raw.contains("/FunctionType 2") && raw.contains("/C0 ["), "{raw}");
}

#[test]
fn p1483_pdf_special_names_and_cmyk_alternate() {
    let none = colorant(None, Color::rgb(0, 68, 255));
    let all = colorant(Some("all"), Color::rgb(0, 204, 0));
    let cmyk = colorant(Some("Other"), Color::cmyk(0.0, 1.0, 0.0, 0.0));
    let doc = PagedDocument::new(vec![page(vec![
        rect(10.0, Some(Paint::Solid(tint(&none, 0.5))), None),
        rect(35.0, Some(Paint::Solid(tint(&all, 0.5))), None),
        rect(60.0, Some(Paint::Solid(tint(&cmyk, 0.5))), None),
    ])]);
    let raw =
        String::from_utf8_lossy(&export_pdf(&doc, StreamMode::Verbose)).into_owned();
    assert!(raw.contains("/Separation /None"), "{raw}");
    assert!(raw.contains("/Separation /All"), "{raw}");
    assert!(raw.contains("/Separation /Other /DeviceCMYK"), "{raw}");
}

#[test]
fn p1483_pdf_spot_gradient_fill_keeps_separation() {
    let brand = colorant(Some("Brand"), Color::rgb(255, 65, 54));
    let stops = || {
        vec![
            GradientStop::new(tint(&brand, 0.0), Ratio(0.0)),
            GradientStop::new(tint(&brand, 1.0), Ratio(1.0)),
        ]
    };
    let space = || ColorSpace::Spot((*brand).clone());
    let doc = PagedDocument::new(vec![page(vec![
        rect(
            10.0,
            Some(Paint::Gradient(Gradient::linear_with_space(
                stops(),
                Angle::deg(0.0),
                space(),
            ))),
            None,
        ),
        rect(
            35.0,
            Some(Paint::Gradient(Gradient::radial_with_space(
                stops(),
                Axes::new(Ratio(0.5), Ratio(0.5)),
                Ratio(0.5),
                space(),
            ))),
            None,
        ),
        rect(
            60.0,
            Some(Paint::Gradient(Gradient::conic_with_space(
                stops(),
                Axes::new(Ratio(0.5), Ratio(0.5)),
                Angle::deg(0.0),
                space(),
            ))),
            None,
        ),
    ])]);
    let pdf = export_pdf(&doc, StreamMode::Verbose);
    let raw = String::from_utf8_lossy(&pdf);
    let stream = extract_page_content_streams_text(&pdf);
    assert!(raw.contains("/Separation") && raw.contains("/ShadingType 2"), "{raw}");
    assert!(raw.contains("/ShadingType 3") && raw.contains("/ShadingType 6"), "{raw}");
    let axial = raw.find("/ShadingType 2 /ColorSpace").expect("axial shading");
    let end = axial + raw[axial..].find("/ShadingType 3").expect("radial shading");
    assert!(
        raw[axial..end].contains("/Extend [true true]"),
        "axial spot must extend as vanilla"
    );
    assert!(
        raw.contains("/Coords [10.000 100.000 70.000 100.000]"),
        "axial spot must follow its shape"
    );
    assert!(
        raw.contains("/Matrix [60.000 0 0 20.000 10.000 40.000]"),
        "conic mesh must follow its shape"
    );
    assert!(stream.matches("/Pattern cs").count() >= 3, "{stream}");
    assert!(
        raw.contains("0000000000 00000 f"),
        "unused reserved IDs must be free in xref"
    );
}

#[test]
fn p1483_pdf_process_control_has_no_separation() {
    let doc = PagedDocument::new(vec![page(vec![
        rect(10.0, Some(Paint::Solid(Color::rgb(255, 65, 54))), None),
        rect(35.0, None, Some(Paint::Solid(Color::rgb(0, 68, 255)))),
    ])]);
    let pdf = export_pdf(&doc, StreamMode::Verbose);
    let raw = String::from_utf8_lossy(&pdf);
    let stream = extract_page_content_streams_text(&pdf);
    assert!(!raw.contains("/Separation"), "{raw}");
    assert!(stream.contains(" rg") && stream.contains(" RG"), "{stream}");
}

#[test]
fn p1483_pdf_background_gradient_registers_pattern() {
    let brand = colorant(Some("Backdrop"), Color::rgb(255, 65, 54));
    let stops = vec![
        GradientStop::new(tint(&brand, 0.0), Ratio(0.0)),
        GradientStop::new(tint(&brand, 1.0), Ratio(1.0)),
    ];
    let mut background_page = page(vec![]);
    background_page.background.push(rect(
        10.0,
        Some(Paint::Gradient(Gradient::linear_with_space(
            stops,
            Angle::deg(0.0),
            ColorSpace::Spot((*brand).clone()),
        ))),
        None,
    ));
    let pdf = export_pdf(&PagedDocument::new(vec![background_page]), StreamMode::Verbose);
    let raw = String::from_utf8_lossy(&pdf);
    let stream = extract_page_content_streams_text(&pdf);
    assert!(
        raw.contains("/Pattern <<") && raw.contains("/Separation /Backdrop"),
        "{raw}"
    );
    assert!(stream.contains("/Pattern cs"), "{stream}");
}

#[test]
fn p1483_pdf_compact_distinguishes_colorants_and_escapes_names() {
    let first = colorant(Some("Brand Ink"), Color::rgb(255, 65, 54));
    let second = colorant(Some("Brand Ink"), Color::rgb(0, 68, 255));
    let doc = PagedDocument::new(vec![page(vec![
        rect(10.0, Some(Paint::Solid(tint(&first, 0.5))), None),
        rect(35.0, Some(Paint::Solid(tint(&second, 0.5))), None),
    ])]);
    let pdf = export_pdf(&doc, StreamMode::Compact);
    let raw = String::from_utf8_lossy(&pdf);
    let stream = extract_page_content_streams_text(&pdf);
    assert_eq!(raw.matches("/Separation /Brand#20Ink").count(), 2, "{raw}");
    assert_eq!(stream.matches(" cs ").count(), 2, "{stream}");
    assert!(!raw.contains("/c0 [/ICCBased"), "{raw}");
}
#[test]
fn p1483_pdf_reused_spot_gradient_uses_distinct_shape_patterns() {
    let brand = colorant(Some("Shared"), Color::rgb(255, 65, 54));
    let gradient = Gradient::linear_with_space(
        vec![
            GradientStop::new(tint(&brand, 0.0), Ratio(0.0)),
            GradientStop::new(tint(&brand, 1.0), Ratio(1.0)),
        ],
        Angle::deg(0.0),
        ColorSpace::Spot((*brand).clone()),
    );
    let doc = PagedDocument::new(vec![page(vec![
        rect(10.0, Some(Paint::Gradient(gradient.clone())), None),
        rect(35.0, Some(Paint::Gradient(gradient)), None),
    ])]);
    let pdf = export_pdf(&doc, StreamMode::Verbose);
    let raw = String::from_utf8_lossy(&pdf);
    let stream = extract_page_content_streams_text(&pdf);
    assert_eq!(raw.matches("/PatternType 2").count(), 2, "{raw}");
    assert_eq!(stream.matches("/Pattern cs").count(), 2, "{stream}");
}
