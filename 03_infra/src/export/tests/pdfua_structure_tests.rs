//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/tests/pdfua_structure_tests.md
//! @prompt-hash 198067d9
//! @layer L3
//! @updated 2026-09-23

use super::*;
use typst_core::entities::document_format_options::{PdfStandard, PdfStandards};

fn export(doc: &typst_core::entities::layout_types::PagedDocument) -> Vec<u8> {
    let standards = PdfStandards::new([PdfStandard::Ua1]).unwrap();
    export_pdf_with_document_id_and_tags_and_serialization_and_pages_and_standard(
        doc,
        None,
        StreamMode::Verbose,
        PdfTags::Enabled,
        PdfSerializationMode::Compact,
        None,
        standards,
    )
    .output
}

#[test]
fn p1481_ua1_lingua_documental_e_texto_real_marcado() {
    let doc = layout(&Content::text("Accessible text"));
    let pdf = export(&doc);
    let raw = String::from_utf8_lossy(&pdf);
    assert!(raw.contains("/Lang (en)"), "catalog language missing");
    assert!(raw.contains("/S /P"), "real paragraph missing from structure");
    let stream = extract_page_content_streams_text(&pdf);
    assert!(stream.contains("/MCID 0"), "real text missing marked content: {stream}");
}

#[test]
fn p1481_ua1_preserva_idioma_local() {
    use typst_core::entities::lang::Lang;
    let mut doc = layout(&Content::text("Titre"));
    doc.document_language = Some("fr".parse::<Lang>().unwrap());
    let pdf = export(&doc);
    let raw = String::from_utf8_lossy(&pdf);
    assert!(raw.contains("/Lang (fr)"));
    assert!(raw.contains("<rdf:li>fr</rdf:li>"));
    assert!(
        raw.contains("/Lang (en)"),
        "texto sem override após primeiro francês deve conservar en"
    );
}

#[test]
fn p1481_common_pdf_nao_recebe_lang_ua1() {
    let doc = layout(&Content::text("Plain"));
    let pdf = export_pdf(&doc, StreamMode::Verbose);
    let raw = String::from_utf8_lossy(&pdf);
    assert!(!raw.contains("/Lang (en)"));
}

#[test]
fn p1481_fragmentos_formam_um_heading_e_um_paragrafo() {
    use typst_core::entities::layout_types::{
        FrameItem, Page, Point, Pt, SemanticKind, SemanticPlacement, TextStyle,
    };
    let mut heading = TextStyle::regular(Pt(11.0));
    heading.heading_level = Some(1);
    let plain = TextStyle::regular(Pt(11.0));
    let mut strong = plain.clone();
    strong.semantic_strong = true;
    let text = |value: &str, style: TextStyle| FrameItem::Text {
        pos: Point::ZERO,
        text: value.into(),
        style,
        bidi: None,
    };
    let boundary = || FrameItem::Semantic {
        kind: SemanticKind::ParbreakBoundary,
        placement: SemanticPlacement::Block,
        alt: None,
        items: vec![text("", plain.clone())],
    };
    let doc = typst_core::entities::layout_types::PagedDocument::new(vec![Page {
        width: 200.0,
        height: 200.0,
        numbering: None,
        logical_number: 1,
        supplement: Content::Empty,
        bleed: Default::default(),
        fill: Default::default(),
        background: vec![],
        foreground: vec![],
        items: vec![
            text("First", heading.clone()),
            text(" heading", heading),
            boundary(),
            text("a ", plain.clone()),
            text("strong", strong),
            text(" end", plain.clone()),
            boundary(),
        ],
    }]);
    let pdf = export(&doc);
    let raw = String::from_utf8_lossy(&pdf);
    assert_eq!(raw.matches("/S /H1").count(), 1, "heading fragmentado: {raw}");
    assert_eq!(raw.matches("/S /P").count(), 1, "parágrafo fragmentado: {raw}");
    assert_eq!(raw.matches("/S /Strong").count(), 1);
}
