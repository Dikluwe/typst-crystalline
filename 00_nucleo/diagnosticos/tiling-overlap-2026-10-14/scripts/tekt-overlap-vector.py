from pathlib import Path
p=Path('03_infra/src/export/tiling.rs');s=p.read_text();i=s.index('/// Resource-only traversal')
s=s[:i]+'''/// Cells whose clipped extent intersects the first period, in painter order.
pub(super) fn contributions(
    cell: &TilingFrame,
) -> Option<impl Iterator<Item = (f64, f64)>> {
    let w = cell.frame.size.width.0;
    let h = cell.frame.size.height.0;
    let px = w + cell.spacing.width.0;
    let py = h + cell.spacing.height.0;
    if [w, h, px, py].into_iter().any(|n| !n.is_finite() || n <= 0.0) {
        return None;
    }
    let nx = (w / px).ceil();
    let ny = (h / py).ceil();
    if !nx.is_finite() || !ny.is_finite()
        || nx >= i64::MAX as f64 || ny >= i64::MAX as f64 {
        return None;
    }
    Some((1 - ny as i64..=0).flat_map(move |y| {
        (1 - nx as i64..=0).map(move |x| (x as f64 * px, y as f64 * py))
    }))
}

'''+s[i:];p.write_text(s)
p=Path('03_infra/src/export/pdf_tiling.rs');s=p.read_text().replace('    id: usize,\n','    id: usize,\n    form_id: usize,\n',1).replace('*next += 1;','*next += 2;',1).replace('                                id,\n','                                id,\n                                form_id: id + 1,\n',1)
a=s.index('            let mut object = format!("<< /Type /Pattern');b=s.index('            objects.push((pattern.id, object));',a)+len('            objects.push((pattern.id, object));')
s=s[:a]+'''            let mut form = format!("<< /Type /XObject /Subtype /Form /BBox [0 0 {} {}] /Resources << {} >> /Length {} >>\\nstream\\n", size.width.0, size.height.0, cell_resources, bytes.len()).into_bytes();
            form.extend_from_slice(&bytes);
            form.extend_from_slice(b"\\nendstream");
            objects.push((pattern.form_id, form));
            let px = size.width.0 + pattern.cell.spacing.width.0;
            let py = size.height.0 + pattern.cell.spacing.height.0;
            let mut period = String::new();
            for (x, y) in super::tiling::contributions(pattern.cell)
                .expect("prepared tiling contribution range") {
                period.push_str(&format!("q 1 0 0 1 {x} {y} cm /Cell Do Q\\n"));
            }
            let mut object = format!("<< /Type /Pattern /PatternType 1 /PaintType 1 /TilingType 1 /BBox [0 0 {px} {py}] /XStep {px} /YStep {py} /Matrix [{} {} {} {} {} {}] /Resources << /XObject << /Cell {} 0 R >> >> /Length {} >>\\nstream\\n",m.a,m.b,m.c,m.d,m.tx,m.ty,pattern.form_id,period.len()).into_bytes();
            object.extend_from_slice(period.as_bytes());
            object.extend_from_slice(b"\\nendstream");
            objects.push((pattern.id, object));'''+s[b:];p.write_text(s)
p=Path('03_infra/src/export/svg/paints.rs');s=p.read_text();a=s.index('                xml.start_element("pattern");');b=s.index('                let clip = format!',a);pattern=s[a:b].replace('"overflow", "visible"','"overflow", "hidden"');s=s[:a]+s[b:];a=s.index('                xml.start_element("g");',a);s=s[:a]+s[a:].replace('                xml.start_element("g");','                xml.start_element("g");\n                xml.write_attribute("id", &format!("{id}cell"));',1);a=s.index('                (self.local_to_root, self.decorative) = previous;');b=s.index('            } else {',a)
s=s[:a]+'''                (self.local_to_root, self.decorative) = previous;
                xml.end_element();
'''+pattern+'''                for (x, y) in crate::export::tiling::contributions(cell)
                    .expect("prepared tiling contribution range") {
                    xml.start_element("use");
                    xml.write_attribute("href", &format!("#{id}cell"));
                    xml.write_attribute("xlink:href", &format!("#{id}cell"));
                    xml.write_attribute("transform", &format!("translate({} {})", fmt_num(x), fmt_num(y)));
                    xml.end_element();
                }
                xml.end_element();
'''+s[b:];p.write_text(s)
