//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/html/p1360_ab_tests.md
//! @prompt-hash ae6f9073
//! @layer L3
//! @updated 2026-09-26

use super::*;
use std::sync::Arc;
use typst_core::entities::html::{HtmlAttrs, HtmlBody, HtmlElem};

fn elem(tag: &str, body: Content) -> Content {
    Content::HtmlElem(Arc::new(HtmlElem::new(
        tag.into(),
        None,
        HtmlBody::Content(Box::new(body)),
    )))
}

fn fixture() -> Content {
    let title = elem("title", Content::text("A  > B"));
    let head = elem("head", title);
    let phrasing = Content::sequence(vec![
        elem("span", Content::text("left")),
        Content::Space,
        elem("em", Content::text("right")),
        Content::HtmlElem(Arc::new(HtmlElem::new("br".into(), None, HtmlBody::Unset))),
    ]);
    let mut attrs = HtmlAttrs::default();
    attrs.insert("data-guard".into(), "left>right".into());
    let article = Content::HtmlElem(Arc::new(HtmlElem::new(
        "article".into(),
        Some(attrs),
        HtmlBody::Content(Box::new(phrasing)),
    )));
    elem("html", Content::sequence(vec![head, elem("body", article)]))
}

#[test]
fn p1360_ab_html_pretty_preserves_phrasing_title_and_quoted_gt() {
    for (mode, attribute) in [
        (HtmlSerializationMode::Crystalline, "data-guard=\"left&gt;right\""),
        (HtmlSerializationMode::Vanilla, "data-guard=\"left>right\""),
    ] {
        let compact = export_html_with_serialization_and_pretty(&fixture(), mode, false)
            .expect("compact HTML");
        let pretty = export_html_with_serialization_and_pretty(&fixture(), mode, true)
            .expect("pretty HTML");

        assert!(!compact.ends_with('\n'));
        assert!(pretty.ends_with('\n'));
        assert!(pretty.contains("\n  <head>"));
        assert!(pretty.contains("<title>A  > B</title>"));
        assert!(!pretty.contains("<title>\n"));
        assert!(pretty.contains("<span>left</span> <em>right</em><br>"));
        assert!(pretty.contains(attribute));
        assert!(!pretty.contains("left>\nright"));
        assert_eq!(compact.contains(attribute), pretty.contains(attribute));
    }
}
