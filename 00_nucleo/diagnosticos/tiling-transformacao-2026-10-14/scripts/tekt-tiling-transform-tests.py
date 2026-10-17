from pathlib import Path
p=Path('01_core/src/compiler/layout/tiling.rs');s=p.read_text();s=s.replace('fn p1254_offset_e_aplicado_antes_da_rotacao_clockwise()', 'fn tiling_transform_offset_is_the_fixed_rotation_origin()');s=s.replace('assert!((matrix.tx + 4.0).abs() < 1e-9);\n        assert!((matrix.ty - 3.0).abs() < 1e-9);','assert!((matrix.tx - 3.0).abs() < 1e-9);\n        assert!((matrix.ty - 4.0).abs() < 1e-9);')
s=s.rsplit('\n}',1)[0]+'''

    fn ink_at(group: &FrameItem, x: f64, y: f64) -> bool {
        let (m, cells) = cells(group);
        let det = m.a * m.d - m.b * m.c;
        let dx = x - m.tx;
        let dy = y - m.ty;
        let (x, y) = ((m.d * dx - m.c * dy) / det, (m.a * dy - m.b * dx) / det);
        cells.iter().any(|cell| {
            let FrameItem::Group { pos: cell_pos, items, .. } = cell else { panic!("expected cell") };
            let FrameItem::Shape { pos, width, height, .. } = &items[0] else { panic!("expected shape") };
            let left = cell_pos.x.0 + pos.x.0;
            let top = cell_pos.y.0 + pos.y.0;
            x >= left && x < left + width && y >= top && y < top + height
        })
    }

    #[test]
    fn tiling_transform_offset_is_rotation_pivot_in_rendered_cells() {
        let mut t = tile(3.0);
        t.size = Some(Size { width: Pt(8.0), height: Pt(12.0) });
        t.angle = Angle::rad(std::f64::consts::FRAC_PI_2);
        t.offset.x.abs = Length::pt(3.0);
        t.offset.y.abs = Length::pt(5.0);
        let group = materialize(&t, Point::ZERO, 40.0, 24.0).unwrap();
        assert!(ink_at(&group, 2.5, 6.5), "offset (3,5) anchors the rotated red cell");
        assert!(!ink_at(&group, 3.5, 4.5), "outside the anchored cell");
    }

    #[test]
    fn tiling_transform_period_shift_uses_rotated_axes() {
        let mut t = tile(3.0);
        t.size = Some(Size { width: Pt(8.0), height: Pt(12.0) });
        t.angle = Angle::rad(std::f64::consts::FRAC_PI_2);
        let plain = materialize(&t, Point::ZERO, 40.0, 24.0).unwrap();
        // At 90 degrees, a 12pt horizontal shift is one full vertical
        // lattice pitch; an 8pt horizontal modulo would change the pattern.
        t.offset.x.abs = Length::pt(12.0);
        let shifted = materialize(&t, Point::ZERO, 40.0, 24.0).unwrap();
        for x in 0..40 {
            for y in 0..24 {
                let (x, y) = (f64::from(x) + 0.37, f64::from(y) + 0.41);
                assert_eq!(ink_at(&plain, x, y), ink_at(&shifted, x, y), "paint differs at ({x},{y})");
            }
        }
    }

    #[test]
    fn tiling_transform_parent_translation_is_not_rotated() {
        let origin = Point { x: Pt(20.0), y: Pt(30.0) };
        let parent = placement_transform(TilingRelative::Parent, origin, 2.0, 5.0, 0.61);
        let reference = placement_transform(TilingRelative::Itself, Point::ZERO, 2.0, 5.0, 0.61);
        for q in [(0.0, 0.0), (3.0, 7.0), (18.0, 24.0)] {
            let actual = parent.apply(q.0, q.1);
            let expected = reference.apply(q.0, q.1);
            assert!((actual.0 + origin.x.0 - expected.0).abs() < 1e-9);
            assert!((actual.1 + origin.y.0 - expected.1).abs() < 1e-9);
        }
    }
}
''';p.write_text(s)
