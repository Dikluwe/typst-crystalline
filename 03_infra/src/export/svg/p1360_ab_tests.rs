//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/svg/p1360_ab_tests.md
//! @prompt-hash bd652ffe
//! @layer L3
//! @updated 2026-09-26

use super::*;

fn empty_page() -> Page {
    Page {
        width: 100.0,
        height: 80.0,
        numbering: None,
        logical_number: 1,
        supplement: typst_core::entities::content::Content::Empty,
        bleed: Default::default(),
        fill: Default::default(),
        background: vec![],
        foreground: vec![],
        items: vec![],
    }
}

#[test]
fn p1360_ab_svg_options_default_compact_and_true_indents() {
    assert!(!SvgOptions::default().pretty);
    let compact =
        export_svg(&empty_page(), &SvgOptions { pretty: false, ..Default::default() });
    let pretty =
        export_svg(&empty_page(), &SvgOptions { pretty: true, ..Default::default() });
    assert!(!compact.contains("\n  <"));
    assert!(pretty.contains("\n  <"));
    let compact_lines = compact.lines().map(str::trim).collect::<String>();
    let pretty_lines = pretty.lines().map(str::trim).collect::<String>();
    assert_eq!(
        compact_lines, pretty_lines,
        "pretty must change only structural XML whitespace"
    );
}
