from pathlib import Path
p=Path('01_core/src/compiler/layout/tests.rs');s=p.read_text();s+=r'''

#[cfg(test)]
mod place_origin_regressions {
    use super::*;
    use crate::entities::layout_types::TransformMatrix;

    fn blue_shape_positions(items: &[FrameItem], transform: TransformMatrix, out: &mut Vec<(f64, f64)>) {
        for item in items {
            match item {
                FrameItem::Shape { pos, width: 12.0, height: 8.0, .. } => {
                    out.push((transform.a * pos.x.0 + transform.c * pos.y.0 + transform.tx,
                              transform.b * pos.x.0 + transform.d * pos.y.0 + transform.ty));
                }
                FrameItem::Group { pos, matrix, items, .. } => {
                    let local = TransformMatrix::translate(pos.x.0, pos.y.0).concat(matrix);
                    blue_shape_positions(items, transform.concat(&local), out);
                }
                FrameItem::Link { items, .. } | FrameItem::Semantic { items, .. }
                | FrameItem::TextReplacement { items, .. } => blue_shape_positions(items, transform, out),
                _ => {}
            }
        }
    }

    fn expect_position(body: &str, expected: (f64, f64)) {
        let source = format!("#set page(width:120pt,height:100pt,margin:10pt)\n{body}");
        let doc = layout_test(&source);
        assert!(doc.layout_errors.is_empty(), "{source}: {:?}", doc.layout_errors);
        let mut points = Vec::new();
        for page in &doc.pages { blue_shape_positions(&page.items, TransformMatrix::identity(), &mut points); }
        assert_eq!(points.len(), 1, "{source}: {points:?}");
        assert!((points[0].0 - expected.0).abs() < 1e-6 && (points[0].1 - expected.1).abs() < 1e-6,
            "{source}: actual {:?}, expected {expected:?}", points[0]);
    }

    const PLACED: &str = "#place(dx:7pt,dy:5pt,rect(width:12pt,height:8pt,fill:blue,stroke:none))";

    #[test]
    fn place_origin_box_has_no_font_strut() {
        for size in [11, 20, 30] {
            for (wrapper, expected) in [
                (format!("#box(width:60pt,height:40pt)[{PLACED}]"), (17.0,15.0)),
                (format!("#box(width:60pt,height:40pt,inset:3pt)[{PLACED}]"), (20.0,18.0)),
                (format!("#box()[{PLACED}]"), (17.0,15.0)),
                (format!("#box(width:80pt,height:60pt,inset:4pt)[#box(width:60pt,height:40pt,inset:3pt)[{PLACED}]]"), (24.0,22.0)),
                (format!("#place(dx:17pt,dy:13pt,box(width:60pt,height:40pt)[{PLACED}])"), (34.0,28.0)),
            ] { expect_position(&format!("#set text(size:{size}pt)\n{wrapper}"),expected); }
        }
    }

    #[test]
    fn place_origin_vertical_alignment_uses_body_extent_once() {
        for (align, expected) in [("bottom+right",(65.0,47.0)),("center+horizon",(41.0,31.0))] {
            expect_position(&format!("#box(width:60pt,height:40pt)[#place({align},dx:7pt,dy:5pt,rect(width:12pt,height:8pt,fill:blue,stroke:none))]"),expected);
        }
    }

    #[test]
    fn place_origin_pending_line_and_explicit_top_are_distinct() {
        let edge = FixedMetrics.text_edges(Pt(11.0), &TextStyle::default()).0.0;
        expect_position(&format!("A{PLACED}"),(17.0,15.0+edge));
        expect_position(&format!("#box(width:60pt,height:40pt)[A{PLACED}]"),(17.0,15.0+edge));
        for wrapper in ["A#place(top,dx:7pt,dy:5pt,rect(width:12pt,height:8pt,fill:blue,stroke:none))",
            "#box(width:60pt,height:40pt)[A#place(top,dx:7pt,dy:5pt,rect(width:12pt,height:8pt,fill:blue,stroke:none))]"] {
            expect_position(wrapper,(17.0,15.0));
        }
    }

    #[test]
    fn place_origin_preserves_legacy_align_and_cell_controls() {
        for source in [
            format!("#align(left)[#box(width:60pt,height:40pt)[{PLACED}]]"),
            format!("#box(width:60pt,height:40pt)[#align(left)[{PLACED}]]"),
            format!("#grid(columns:(60pt,),rows:(40pt,))[{PLACED}]"),
            format!("#table(columns:(60pt,),rows:(40pt,),inset:0pt,stroke:none)[{PLACED}]"),
        ] { expect_position(&source,(17.0,15.0)); }
    }
}
''';p.write_text(s)
