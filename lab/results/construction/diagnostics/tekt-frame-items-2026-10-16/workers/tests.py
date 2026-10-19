from pathlib import Path
import subprocess,json
out=Path('00_nucleo/diagnosticos/tekt-frame-items-2026-10-16');p=Path('01_core/src/entities/layout_types.rs');old=p.read_text();anchor='    #[test]\n    fn pt_add_pt()';assert old.count(anchor)==1
new='''    #[test]
    fn v16_frame_visitor_keeps_all_variants_callback_order_and_wrappers() {
        #[derive(Default)]
        struct Recorder { events: Vec<&'static str> }
        impl FrameVisitor for Recorder {
            fn visit_text(&mut self, _: &FrameItem) { self.events.push("text"); }
            fn visit_text_shaped(&mut self, _: &FrameItem) { self.events.push("shaped"); }
            fn visit_line(&mut self, _: &FrameItem) { self.events.push("line"); }
            fn visit_glyph(&mut self, _: &FrameItem) { self.events.push("glyph"); }
            fn visit_image(&mut self, _: &FrameItem) { self.events.push("image"); }
            fn visit_shape(&mut self, _: &FrameItem) { self.events.push("shape"); }
            fn visit_group(&mut self, _: &FrameItem) { self.events.push("group"); }
            fn visit_link(&mut self, _: &FrameItem) { self.events.push("link"); }
            fn visit_semantic(&mut self, _: &FrameItem) { self.events.push("semantic"); }
        }
        let style = TextStyle::regular(Pt(12.0));
        let text = FrameItem::Text { pos: Point::ZERO, text: "x".into(), style: style.clone(), bidi: None };
        let line = FrameItem::Line { start: Point::ZERO, end: Point { x: Pt(1.0), y: Pt::ZERO }, thickness: 1.0, color: None, cap: crate::entities::geometry::LineCap::Butt };
        let glyph = FrameItem::Glyph { pos: Point::ZERO, glyph_id: 1, x_advance: Pt(1.0), size: Pt(12.0), style: style.clone(), base_char: 'x' };
        let items = vec![
            text.clone(),
            FrameItem::TextSeparator { pos: Point::ZERO, advance: Pt(1.0), style: style.clone(), origin: TextSeparatorOrigin::Literal, bidi: None },
            FrameItem::TextShaped { pos: Point::ZERO, glyphs: vec![], style, text: "shaped".into(), units_per_em: 1000, font: None, bidi: None },
            line.clone(), glyph.clone(),
            FrameItem::Image { pos: Point::ZERO, data: std::sync::Arc::new(vec![]), alt: None, width: Pt(1.0), height: Pt(1.0), intrinsic_width: 1, intrinsic_height: 1, clip_rect: None, orientation: 1 },
            FrameItem::Shape { pos: Point::ZERO, kind: crate::entities::geometry::ShapeKind::Rect, width: 1.0, height: 1.0, fill: None, stroke: None, fill_rule: crate::entities::geometry::FillRule::NonZero, parent_bbox_at_emit: None },
            FrameItem::TextReplacement { text: "logical".into(), items: vec![text.clone()] },
            FrameItem::Semantic { kind: SemanticKind::Formula, placement: SemanticPlacement::Inline, alt: None, items: vec![text] },
            FrameItem::Group { paint_root: false, pos: Point::ZERO, matrix: TransformMatrix::identity(), clip_mask: None, inner_width: 1.0, inner_height: 1.0, items: vec![line] },
            FrameItem::Link { target: LinkTarget::Url("https://example.test".into()), items: vec![glyph], pos: Point::ZERO, size: Size { width: Pt(1.0), height: Pt(1.0) } },
        ];
        let mut visitor = Recorder::default();
        walk_frame_items(&mut visitor, &items);
        assert_eq!(visitor.events, ["text", "text", "shaped", "line", "glyph", "image", "shape", "text", "semantic", "text", "group", "line", "link", "glyph"]);
    }

'''
p.write_text(old.replace(anchor,new+anchor));subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',str(p)],check=True)
(out/'tests-materialization.json').write_text(json.dumps(dict(before={str(p):old},after={str(p):p.read_text()}),ensure_ascii=False,indent=2)+'\n');print('One visitor regression added before implementation')
