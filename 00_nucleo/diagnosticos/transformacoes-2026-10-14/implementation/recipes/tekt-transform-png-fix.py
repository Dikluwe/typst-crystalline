from pathlib import Path
p=Path('03_infra/src/export/render.rs');s=p.read_text();a=s.index('    let Some(glyph) = pixglyph::Glyph::load(face, id) else {',s.index('fn render_glyph('));b=s.index('    let bitmap = glyph.rasterize',a)
s=s[:a]+'''    let ts = state.transform;
    let ppem = size * ts.sy;
    if ts.kx != 0.0 || ts.ky != 0.0 || ts.sx != ts.sy || ppem < 0.0 || ppem > 100.0 {
        struct Outline(sk::PathBuilder);
        impl ttf_parser::OutlineBuilder for Outline {
            fn move_to(&mut self, x: f32, y: f32) {
                self.0.close();
                self.0.move_to(x, y);
            }
            fn line_to(&mut self, x: f32, y: f32) { self.0.line_to(x, y); }
            fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
                self.0.quad_to(x1, y1, x, y);
            }
            fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
                self.0.cubic_to(x1, y1, x2, y2, x, y);
            }
            fn close(&mut self) { self.0.close(); }
        }
        let mut outline = Outline(sk::PathBuilder::new());
        if face.outline_glyph(id, &mut outline).is_none() { return; }
        // CFF2 outlines can omit the final close command.
        outline.0.close();
        let Some(path) = outline.0.finish() else { return; };
        let scale = size / f32::from(face.units_per_em());
        let transform = ts.pre_translate(x, y).pre_scale(scale, -scale);
        let mut paint = sk::Paint::default();
        paint.set_color(color);
        canvas.fill_path(&path, &paint, sk::FillRule::Winding, transform, state.mask);
        return;
    }
    let Some(glyph) = pixglyph::Glyph::load(face, id) else {
        return;
    };

'''+s[b:];p.write_text(s)
