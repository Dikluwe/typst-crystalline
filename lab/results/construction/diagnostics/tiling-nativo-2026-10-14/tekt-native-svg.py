from pathlib import Path
p=Path('03_infra/src/export/svg/paints.rs');s=p.read_text()
s=s.replace('use typst_core::entities::paint::Paint;','use typst_core::entities::frame_paint::FramePaint as Paint;\nuse typst_core::entities::layout_types::{Point, Pt, TransformMatrix};\nuse std::sync::Arc;')
s=s.replace('use typst_core::entities::tiling::{TilingBody, TilingRelative};\n','')
s=s.replace('    paints: Vec<SvgPaintDef>,','    paints: Vec<SvgPaintDef>,\n    pub(super) local_to_root: TransformMatrix,\n    pub(super) decorative: bool,')
s=s.replace('struct SvgPaintDef {','#[derive(Clone)]\nstruct SvgPaintDef {\n    matrix: TransformMatrix,')
s=s.replace('value: color_to_css(paint.to_color()),','value: color_to_css(match paint { Paint::Gradient(g) => typst_core::entities::paint::Paint::Gradient(g.clone()).to_color(), _ => unreachable!() }),')
s=s.replace('        let (paint_x, paint_y, paint_width, paint_height) =','''        let matrix = if let Paint::Tiling(cell) = paint {
            let Some(matrix) = crate::export::tiling::placement(cell, Point { x: Pt(x), y: Pt(y) }, self.local_to_root) else {
                return PaintReference { value: "none".into(), fallback: Some("noninvertible-tiling-transform") };
            };
            matrix
        } else { TransformMatrix::identity() };
        let (paint_x, paint_y, paint_width, paint_height) =''')
s=s.replace('candidate.paint == *paint','same_paint(&candidate.paint, paint) && candidate.matrix == matrix')
s=s.replace('                    paint: paint.clone(),','                    paint: paint.clone(),\n                    matrix,')
a=s.index('    pub(super) fn write(&self, xml: &mut XmlWriter)');b=s.index('\nfn unsupported_paint_reason',a)
s=s[:a]+'''    pub(super) fn write(
        &mut self, xml: &mut XmlWriter,
        fonts: Option<&crate::export::font_binding::PreparedFonts>,
        glyph_defs: &mut super::GlyphDefs, clip_next_id: &mut usize,
        glyph_fonts: &super::SvgGlyphFontContext,
    ) {
        if self.paints.is_empty() { return; }
        xml.start_element("defs");
        let mut index = 0;
        while index < self.paints.len() {
            let definition = self.paints[index].clone();
            let id = format!("p{index}");
            if let Paint::Tiling(cell) = &definition.paint {
                let size = cell.frame.size;
                xml.start_element("pattern");
                xml.write_attribute("id", &id);
                xml.write_attribute("patternUnits", "userSpaceOnUse");
                xml.write_attribute("patternContentUnits", "userSpaceOnUse");
                xml.write_attribute("width", &fmt_num(size.width.0 + cell.spacing.width.0));
                xml.write_attribute("height", &fmt_num(size.height.0 + cell.spacing.height.0));
                xml.write_attribute("overflow", "visible");
                let m = definition.matrix;
                xml.write_attribute("patternTransform", &format!("matrix({} {} {} {} {} {})", fmt_num(m.a), fmt_num(m.b), fmt_num(m.c), fmt_num(m.d), fmt_num(m.tx), fmt_num(m.ty)));
                let clip = format!("c{}", *clip_next_id); *clip_next_id += 1;
                xml.start_element("clipPath");
                xml.write_attribute("id", &clip);
                xml.write_attribute("clipPathUnits", "userSpaceOnUse");
                xml.start_element("rect");
                xml.write_attribute("width", &fmt_num(size.width.0));
                xml.write_attribute("height", &fmt_num(size.height.0));
                xml.end_element(); xml.end_element();
                xml.start_element("g");
                xml.write_attribute("clip-path", &format!("url(#{clip})"));
                let previous = (self.local_to_root, self.decorative);
                self.local_to_root = TransformMatrix::identity(); self.decorative = true;
                for item in &cell.frame.items {
                    super::render_item(xml, item, fonts, glyph_defs, self, clip_next_id, &super::SvgDestinationContext::empty(), glyph_fonts, false);
                }
                (self.local_to_root, self.decorative) = previous;
                xml.end_element(); xml.end_element();
            } else {
                write_paint_def(xml, &id, &definition.paint, definition.x, definition.y, definition.width, definition.height);
            }
            index += 1;
        }
        xml.end_element();
    }
}

fn same_paint(a: &Paint, b: &Paint) -> bool {
    match (a,b) {
        (Paint::Solid(a), Paint::Solid(b)) => a == b,
        (Paint::Gradient(a), Paint::Gradient(b)) => a == b,
        (Paint::Tiling(a), Paint::Tiling(b)) => Arc::ptr_eq(a,b),
        _ => false,
    }
}
''' +s[b:]
s=s.replace('Paint::Tiling(_) => "tiling-content-or-size",','Paint::Tiling(_) => unreachable!("prepared tiling is native"),')
s=s.replace('Paint::Tiling(tiling) => {\n            tiling.size.is_some() && matches!(tiling.body, TilingBody::Color(_))\n        }','Paint::Tiling(_) => true,')
a=s.index('        Paint::Tiling(tiling) => {');b=s.index('        Paint::Solid(_) => {}',a)
s=s[:a]+'        Paint::Tiling(_) => unreachable!("prepared cell emitted by PaintDefs"),\n'+s[b:]
p.write_text(s)
p=Path('03_infra/src/export/svg/geometry.rs');s=p.read_text().replace('::{PathItem, ShapeKind, Stroke}', '::{PathItem, ShapeKind}').replace('use typst_core::entities::paint::Paint;', 'use typst_core::entities::frame_paint::{FramePaint as Paint, FrameStroke as Stroke};')
s=s.replace('        let outset = value.thickness / 2.0;', '''        if matches!(value.paint, Paint::Tiling(_)) {
            return paint_defs.reference(&value.paint, pos.x.0, pos.y.0, width, height);
        }
        let outset = value.thickness / 2.0;''',1)
# The path optimization adds a local transform, which shifts a user-space paint.
s=s.replace('if let [PathItem::MoveTo(start), PathItem::LineTo(end)] = items.as_slice() {','if let [PathItem::MoveTo(start), PathItem::LineTo(end)] = items.as_slice()\n                && !matches!(fill, Some(Paint::Tiling(_)))\n                && !stroke.is_some_and(|s| matches!(s.paint, Paint::Tiling(_))) {')
p.write_text(s)
p=Path('03_infra/src/export/svg.rs');s=p.read_text().replace('::{FrameItem, Page}', '::{FrameItem, Page, TransformMatrix}')
s=s.replace('paint_defs.write(&mut xml);', 'paint_defs.write(&mut xml, fonts, &mut glyph_defs, &mut clip_next_id, glyph_fonts);')
s=s.replace('            paint_root: _,','            paint_root,')
s=s.replace('            render_group(\n','''            let previous_root = paint_defs.local_to_root;
            paint_defs.local_to_root = if *paint_root { TransformMatrix::identity() }
                else { previous_root.concat(&TransformMatrix::translate(pos.x.0, pos.y.0)).concat(matrix) };
            render_group(
''')
s=s.replace('        FrameItem::Link { target, items, pos, size } => {','''        FrameItem::Link { items, .. } if paint_defs.decorative => {
            for item in items { render_item(xml, item, fonts, glyph_defs, paint_defs, clip_next_id, destinations, glyph_fonts, false); }
        }
        FrameItem::Link { target, items, pos, size } => {''')
a=s.index('            render_group(');b=s.index('\n        }',a);s=s[:b]+'\n            paint_defs.local_to_root = previous_root;'+s[b:];p.write_text(s)
