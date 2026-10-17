from pathlib import Path
p=Path('01_core/src/compiler/layout/tests.rs');s=p.read_text();assert s.rstrip().endswith('}')
new=r'''
    #[test]
    fn block_region_place_respects_insets_and_explicit_alignment() {
        for (args, align, expected) in [
            ("inset:3pt", "", (20.0,18.0)),
            ("inset:3pt", "bottom+right,", (62.0,44.0)),
            ("inset:3pt", "center+horizon,", (41.0,31.0)),
            ("inset:(left:4pt,top:6pt,right:8pt,bottom:10pt)", "", (21.0,21.0)),
            ("inset:(left:4pt,top:6pt,right:8pt,bottom:10pt)", "bottom+right,", (57.0,37.0)),
        ] {
            let body = format!("#block(width:60pt,height:40pt,{args})[#place({align}dx:7pt,dy:5pt,rect(width:12pt,height:8pt,fill:blue,stroke:none))]");
            expect_position(&body,expected);
            expect_position(&format!("#box(width:90pt,height:70pt)[{body}]"),expected);
        }
        expect_position(&format!("#block(width:80pt,height:60pt,inset:4pt)[#block(width:60pt,height:40pt,inset:3pt)[{PLACED}]]"),(24.0,22.0));
    }

    #[test]
    fn block_region_place_auto_height_and_pending_text_are_local() {
        let edge = FixedMetrics.text_edges(Pt(11.0), &TextStyle::default()).0 .0;
        expect_position("#block(width:60pt,inset:3pt)[#place(bottom+right,dx:7pt,dy:5pt,rect(width:12pt,height:8pt,fill:blue,stroke:none))]",(62.0,10.0));
        expect_position(&format!("#block(width:60pt,height:40pt,inset:3pt)[A{PLACED}]"),(20.0,18.0+edge));
        expect_position("#block(width:60pt,height:40pt,inset:3pt)[A#place(top,dx:7pt,dy:5pt,rect(width:12pt,height:8pt,fill:blue,stroke:none))]",(20.0,18.0));
        expect_position(&format!("#block(width:60pt,height:20pt,inset:3pt)[]{PLACED}"),(17.0,35.0));
    }

    #[test]
    fn block_region_clip_uses_allocated_frame_and_local_children() {
        let src = format!("#set page(width:120pt,height:100pt,margin:10pt)\n#block(width:60pt,height:40pt,inset:3pt,clip:true)[{PLACED}]");
        let doc=layout_test(&src);
        let (pos,w,h,items)=doc.pages[0].items.iter().find_map(|i| {
            if let FrameItem::Group{pos,inner_width,inner_height,clip_mask:Some(_),items,..}=i {
                Some((pos,*inner_width,*inner_height,items))
            } else {None}
        }).expect("clip group");
        assert_eq!((pos.x.0,pos.y.0,w,h),(10.0,10.0,60.0,40.0));
        let mut positions=Vec::new();
        blue_shape_positions(items,TransformMatrix::identity(),&mut positions);
        assert_eq!(positions,vec![(10.0,8.0)]);
        expect_position("#block(width:60pt,height:40pt,inset:3pt,clip:true)[#place(bottom+right,dx:7pt,dy:5pt,rect(width:12pt,height:8pt,fill:blue,stroke:none))]",(62.0,44.0));
    }
'''
s=s.rstrip()[:-1]+new+'}\n';p.write_text(s)
