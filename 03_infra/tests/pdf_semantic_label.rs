//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/tests/pdf_semantic_label.md
//! @prompt-hash 015a5ad0
//! @layer L3
//! @updated 2026-09-15
//!
//! Harness focal do carrier semântico PDF `/Lbl` de P1405.

use typst_core::entities::content::Content;
use typst_core::entities::layout_types::{
    FrameItem, Page, PagedDocument, SemanticKind, SemanticPlacement,
};
use typst_infra::export::{export_pdf, StreamMode};

#[test]
fn semantic_label_emits_lbl_structure_and_marked_content() {
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
        items: vec![FrameItem::Semantic {
            kind: SemanticKind::Label,
            placement: SemanticPlacement::Inline,
            alt: None,
            items: vec![],
        }],
    };
    let pdf = export_pdf(&PagedDocument::new(vec![page]), StreamMode::Verbose).output;
    let raw = String::from_utf8_lossy(&pdf);
    assert!(raw.contains("/S /Lbl"));
    assert!(raw.contains("/Lbl << /MCID 0 >> BDC"));
}
