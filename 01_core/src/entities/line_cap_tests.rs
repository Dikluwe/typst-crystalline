//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/line_cap_tests.md
//! @prompt-hash ece73ab9
//! @layer L1
#![cfg(test)]

use crate::compiler::layout::layout;
use crate::entities::content::Content;
use crate::entities::layout_types::FrameItem;

fn line_debugs(items: &[FrameItem], out: &mut Vec<String>) {
    for item in items {
        match item {
            FrameItem::Line { .. } => out.push(format!("{item:?}")),
            FrameItem::Group { items, .. }
            | FrameItem::Link { items, .. }
            | FrameItem::Semantic { items, .. } => line_debugs(items, out),
            FrameItem::TextReplacement { .. }
            | FrameItem::TextSeparator { .. }
            | FrameItem::Text { .. }
            | FrameItem::TextShaped { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Image { .. }
            | FrameItem::Shape { .. } => {}
        }
    }
}

fn rendered_lines(content: Content) -> Vec<String> {
    let document = layout(&Content::equation(content, false));
    let mut out = Vec::new();
    for page in &document.pages {
        line_debugs(&page.background, &mut out);
        line_debugs(&page.items, &mut out);
        line_debugs(&page.foreground, &mut out);
    }
    out
}

#[test]
fn p1431_auto_square_e_produtor_legado_butt_chegam_ao_frame() {
    let matrix = Content::math_matrix_full(
        vec![
            vec![Content::text("a"), Content::text("b")],
            vec![Content::text("c"), Content::text("d")],
        ],
        ('(', ')'),
        None,
        None,
        None,
        Some(1),
    );
    let auto = rendered_lines(matrix);
    assert_eq!(auto.len(), 1, "P34 matrix auto line identity: {auto:?}");
    assert!(auto[0].contains("Square"), "P34 matrix auto cap: {auto:?}");

    let legacy =
        rendered_lines(Content::math_frac(Content::text("x"), Content::text("y")));
    assert_eq!(legacy.len(), 1, "P34 legacy fraction line identity: {legacy:?}");
    assert!(legacy[0].contains("Butt"), "P34 legacy cap: {legacy:?}");
}

#[test]
fn p1431_cap_opaco_falha_fechado() {
    let lines =
        rendered_lines(Content::math_frac(Content::text("x"), Content::text("y")));
    let truncated = lines.first().and_then(|line| line.get(..line.len().min(4)));
    let verdict = match truncated {
        Some(value) if value.contains("cap:") => {
            ("PRESERVED_LINE_CAP_CARRIER", true, false)
        }
        _ => ("UNKNOWN_STROKE_OBSERVATION", false, false),
    };
    assert_eq!(verdict, ("UNKNOWN_STROKE_OBSERVATION", false, false));
}
