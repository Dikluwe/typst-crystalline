from pathlib import Path
p=Path('03_infra/src/export/stream/draw.rs');s=p.read_text();a=s.index('    fn solid_alpha_is_scoped_across_text_replacement_and_next_leaf()');part=s[a:];part=part.replace('''            for local in [false, true] {''','''            let opaque = FrameItem::Line { color: None, ..translucent.clone() };
            for local in [false, true] {''') # enums struct update invalid replace below
part=part.replace('let opaque = FrameItem::Line { color: None, ..translucent.clone() };','let mut opaque = translucent.clone();\n            if let FrameItem::Line { color, .. } = &mut opaque { *color = None; }')
part=part.replace('[&translucent, &replacement]','[&translucent, &replacement, &opaque]');part=part.replace('''                let mut depth = 0;
                for token in stream.split_whitespace() {
                    match token { "q" => depth += 1, "Q" => { depth -= 1; assert!(depth >= 0); }, _ => {} }
                }
                assert_eq!(depth, 0, "alpha and ActualText scopes balanced: {stream}");''','''                let mut state = (1.0f32, 1.0f32);
                let mut stack = Vec::new();
                let mut last = "";
                let mut strokes = Vec::new();
                let mut texts = Vec::new();
                for token in stream.split_whitespace() {
                    match token {
                        "q" => stack.push(state),
                        "Q" => state = stack.pop().expect("balanced graphics state"),
                        "gs" => {
                            let hex = last.strip_prefix("/A").unwrap();
                            state = (f32::from_bits(u32::from_str_radix(&hex[..8],16).unwrap()), f32::from_bits(u32::from_str_radix(&hex[8..],16).unwrap()));
                        }
                        "S" => strokes.push(state.1),
                        "Tj" => texts.push(state.0),
                        _ => {}
                    }
                    last = token;
                }
                assert!(stack.is_empty(), "alpha and ActualText scopes balanced: {stream}");
                assert_eq!(state, (1.0, 1.0));
                assert_eq!(strokes, vec![0.5, 1.0], "transparent stroke must not affect following leaf");
                assert_eq!(texts, vec![0.25]);''');s=s[:a]+part;p.write_text(s)
p=Path('03_infra/src/export/tiling.rs');s=p.read_text();a=s.rfind('\n}');s=s[:a]+'''
    #[test]
    fn overlap_contributors_match_half_open_cell_intersections() {
        let mut c = cell(RelativeTo::Self_);
        c.frame.size = Size { width: Pt(8.0), height: Pt(9.0) };
        c.spacing = Size { width: Pt(-4.0), height: Pt(-5.0) };
        let translations: Vec<_> = contributions(&c).unwrap().collect();
        assert_eq!(translations, vec![(-4.0,-8.0), (0.0,-8.0), (-4.0,-4.0), (0.0,-4.0), (-4.0,0.0), (0.0,0.0)]);
        c.spacing.width = Pt(-8.0);
        assert!(contributions(&c).is_none());
        c.spacing.width = Pt(f64::INFINITY);
        assert!(contributions(&c).is_none());
    }
'''+s[a:];p.write_text(s)
