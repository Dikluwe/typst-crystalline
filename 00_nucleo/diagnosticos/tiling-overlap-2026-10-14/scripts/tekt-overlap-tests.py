from pathlib import Path
p=Path('03_infra/src/export/pdf_tiling.rs');s=p.read_text();idx=s.rfind('\n}')
s=s[:idx]+'''
    #[test]
    fn period_overlap_uses_four_clipped_contributions_and_one_cell_body() {
        let mut paint = cell(vec![shape(FramePaint::Solid(Color::srgb_f32(1.0, 0.0, 0.0, 0.5)))]);
        if let FramePaint::Tiling(c) = &mut paint {
            Arc::make_mut(c).spacing = Size { width: Pt(-4.0), height: Pt(-4.0) };
        }
        let page = Page {
            width: 40.0, height: 40.0, numbering: None, logical_number: 1,
            supplement: typst_core::entities::content::Content::Empty,
            bleed: Default::default(), fill: Default::default(), background: vec![],
            foreground: vec![], items: vec![shape(paint)],
        };
        let doc = PagedDocument::new(vec![page]);
        let patterns = Patterns::scan(&doc, &mut 10);
        let map = HashMap::new();
        let mut ctx = super::super::stream::PageContext::type1(&map, &[], &map, &[], super::super::StreamMode::Compact);
        ctx.tiling_names = Some(&patterns.names[0]);
        let objects = patterns.emit(0, &mut ctx, &patterns.resources("", 0));
        let forms: Vec<_> = objects.iter().map(|(_, b)| String::from_utf8_lossy(b)).filter(|s| s.contains("/Subtype /Form")).collect();
        assert_eq!(forms.len(), 1, "body stored once, referenced for overlap");
        assert!(forms[0].contains("/BBox [0 0 8 8]"));
        let pattern = objects.iter().map(|(_, b)| String::from_utf8_lossy(b)).find(|s| s.contains("/PatternType 1")).unwrap();
        assert!(pattern.contains("/BBox [0 0 4 4]"));
        assert_eq!(pattern.matches(" Do").count(), 4);
        for translation in ["-4 -4 cm", "0 -4 cm", "-4 0 cm", "0 0 cm"] { assert!(pattern.contains(translation), "{translation}"); }
        let svg = crate::export::svg::export_svg(&doc.pages[0], &Default::default());
        assert_eq!(svg.matches("<use ").count(), 4);
        assert_eq!(svg.matches("fill-opacity").count(), 1, "store body alpha once");
    }
''' +s[idx:];p.write_text(s)
p=Path('03_infra/src/export/stream/draw.rs');s=p.read_text();idx=s.rfind('\n}')
s=s[:idx]+'''
    #[test]
    fn solid_alpha_is_scoped_across_text_replacement_and_next_leaf() {
        use typst_core::entities::layout_types::{Color, TextStyle};
        let map = std::collections::HashMap::new();
        for mode in [StreamMode::Compact, StreamMode::Verbose] {
            let ctx = PageContext::type1(&map, &[], &map, &[], mode);
            let translucent = FrameItem::Line { start: Point::ZERO, end: Point { x: Pt(10.0), y: Pt(0.0) }, thickness: 1.0, color: Some(Color::srgb_f32(1.0, 0.0, 0.0, 0.5)), cap: Default::default() };
            let text = FrameItem::Text { pos: Point { x: Pt(3.0), y: Pt(12.0) }, text: "A".into(), style: TextStyle { fill: Some(Color::srgb_f32(0.0, 0.0, 1.0, 0.25)), ..Default::default() }, bidi: None };
            let replacement = FrameItem::TextReplacement { text: "A".into(), items: vec![text] };
            for local in [false, true] {
                let mut stream = String::new();
                for item in [&translucent, &replacement] {
                    if local { draw_item_local(&mut stream, item, None, &ctx); }
                    else { stream = draw_item_top(stream, item, 40.0, &ctx); }
                }
                assert!(stream.contains(" gs"), "alpha must be emitted");
                let mut depth = 0;
                for token in stream.split_whitespace() {
                    match token { "q" => depth += 1, "Q" => { depth -= 1; assert!(depth >= 0); }, _ => {} }
                }
                assert_eq!(depth, 0, "alpha and ActualText scopes balanced: {stream}");
                assert!(stream.rfind("EMC").unwrap() < stream.rfind("Q").unwrap());
            }
        }
    }
''' +s[idx:];p.write_text(s)
