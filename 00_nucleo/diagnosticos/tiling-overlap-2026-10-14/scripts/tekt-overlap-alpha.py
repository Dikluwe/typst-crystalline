from pathlib import Path
p=Path('03_infra/src/export/stream/shape.rs');s=p.read_text();a=s.index('// ── Helpers')
s=s[:a]+'''/// A resource name shared by collection and operators, preserving f32 alpha.
pub(in crate::export) fn opacity_name(fill: f32, stroke: f32) -> String {
    format!("A{:08x}{:08x}", fill.to_bits(), stroke.to_bits())
}

pub(in crate::export) fn opacity_operator(fill: f32, stroke: f32) -> String {
    if fill == 1.0 && stroke == 1.0 { return String::new(); }
    format!("/{} gs\\n", opacity_name(fill, stroke))
}

pub(in crate::export) fn solid_opacity(
    item: &typst_core::entities::layout_types::FrameItem,
) -> (f32, f32) {
    use typst_core::entities::{frame_paint::FramePaint, layout_types::FrameItem};
    let color_alpha = |c: &typst_core::entities::color::Color| c.to_rgba_f32().3;
    let paint_alpha = |p: &FramePaint| match p {
        FramePaint::Solid(c) => color_alpha(c),
        FramePaint::Gradient(_) | FramePaint::Tiling(_) => 1.0,
    };
    match item {
        FrameItem::Shape { fill, stroke, .. } => (
            fill.as_ref().map(paint_alpha).unwrap_or(1.0),
            stroke.as_ref().map(|s| paint_alpha(&s.paint)).unwrap_or(1.0),
        ),
        FrameItem::Text { style, .. } | FrameItem::TextSeparator { style, .. }
        | FrameItem::TextShaped { style, .. } | FrameItem::Glyph { style, .. } => {
            let a = style.fill.as_ref().map(color_alpha).unwrap_or(1.0);
            (a, a)
        }
        FrameItem::Line { color, .. } => (1.0, color.as_ref().map(color_alpha).unwrap_or(1.0)),
        FrameItem::Group { .. } | FrameItem::Link { .. } | FrameItem::Semantic { .. }
        | FrameItem::TextReplacement { .. } | FrameItem::Image { .. } => (1.0, 1.0),
    }
}

'''+s[a:];p.write_text(s)
p=Path('03_infra/src/export/stream.rs');s=p.read_text().replace('    line_rg_prefix, spot_resource_name,','    line_rg_prefix, opacity_name, opacity_operator, solid_opacity, spot_resource_name,');p.write_text(s)
p=Path('03_infra/src/export/builder/resources.rs');s=p.read_text();a=s.index('pub(super) fn colorspace_resource_entry(')
s=s[:a]+'''fn opacity_resources(page: &typst_core::entities::layout_types::Page) -> String {
    use crate::export::stream::{opacity_name, solid_opacity};
    struct Collector(std::collections::BTreeMap<String, (f32, f32)>);
    impl Collector {
        fn add(&mut self, fill: f32, stroke: f32) {
            if fill != 1.0 || stroke != 1.0 {
                self.0.insert(opacity_name(fill, stroke), (fill, stroke));
            }
        }
    }
    impl FrameVisitor for Collector {
        fn visit_item(&mut self, item: &FrameItem) {
            let (f, s) = solid_opacity(item);
            self.add(f, s);
        }
        fn visit_shape(&mut self, item: &FrameItem) { self.visit_item(item); }
    }
    let mut collector = Collector(Default::default());
    if let typst_core::entities::page_canvas::PageFill::Paint(
        typst_core::entities::paint::Paint::Solid(c)
    ) = &page.fill { collector.add(c.to_rgba_f32().3, 1.0); }
    for items in [&page.background, &page.items, &page.foreground] {
        crate::export::tiling::walk_resources(&mut collector, items);
    }
    if collector.0.is_empty() { return String::new(); }
    let mut out = String::from(" /ExtGState << ");
    for (name, (fill, stroke)) in collector.0 {
        out.push_str(&format!("/{name} << /Type /ExtGState /ca {fill} /CA {stroke} >> "));
    }
    out.push_str(">>");
    out
}

'''+s[a:];old='''    if entries.is_empty() {
        String::new()
    } else {
        format!(" /ColorSpace << {} >>", entries.join(" "))
    }
}''';new='''    let opacity = opacity_resources(page);
    if entries.is_empty() {
        opacity
    } else {
        format!(" /ColorSpace << {} >>{opacity}", entries.join(" "))
    }
}''';assert old in s;s=s.replace(old,new,1);p.write_text(s)
p=Path('03_infra/src/export/stream/draw.rs');s=p.read_text();s=s.replace('    pdf_defaults, PageContext, PdfTags, StreamMode,','    opacity_operator, solid_opacity, pdf_defaults, PageContext, PdfTags, StreamMode,')
s=s.replace(') -> (String, bool) {',') -> (String, usize) {',1)
s=s.replace('let mut tail = false;', 'let mut tail = 0;',1).replace('                if tail {\n                    ops.push_str("Q\\n");\n                }','                for _ in 0..tail {\n                    ops.push_str("Q\\n");\n                }',1).replace('return (ops, false);','return (ops, 0);')
# Both dispatchers scope only nonopaque leaves, with no extra operators for opaque ones.
opening='''    let (fill_alpha, stroke_alpha) = solid_opacity(item);
    let opacity = opacity_operator(fill_alpha, stroke_alpha);
    if !opacity.is_empty() { ops.push_str("q\\n"); ops.push_str(&opacity); }
'''
needle='''    if ua1_decoration {
        ops.push_str("/Artifact << /Type /Layout >> BDC\\n");
    }
    match item {'''
assert s.count(needle)==2;s=s.replace(needle,needle.replace('    match item {',opening+'    match item {'))
s=s.replace('    (ops, deferred)\n}', '''    let mut restores = usize::from(deferred);
    if !opacity.is_empty() {
        if defer_restore { restores += 1; } else { ops.push_str("Q\\n"); }
    }
    (ops, restores)
}''',1)
s=s.replace('    restores\n}\n\n#[cfg(test)]', '''    if !opacity.is_empty() {
        if defer_restore { restores += 1; } else { ops.push_str("Q\\n"); }
    }
    restores
}

#[cfg(test)]''',1)
# Direct marked-glyph paths bypass the ordinary leaf dispatcher.
needle='''                    let close = index + 1 == items.len();'''
assert s.count(needle)==2;s=s.replace(needle,needle+'''
                    let (f, s) = solid_opacity(child);
                    let glyph_opacity = opacity_operator(f, s);
                    if !glyph_opacity.is_empty() { ops.push_str("q\\n"); ops.push_str(&glyph_opacity); }''')
needle='''                    }
                }
            } else {
                super::text_replacement::opening(&mut ops, text);'''
assert needle in s;s=s.replace(needle,'''                    }
                    if !glyph_opacity.is_empty() { ops.push_str("Q\\n"); }
                }
            } else {
                super::text_replacement::opening(&mut ops, text);''',1)
needle='''                    group_text_flip_close(ops);
                }
            } else {''';assert needle in s;s=s.replace(needle,'''                    group_text_flip_close(ops);
                    if !glyph_opacity.is_empty() { ops.push_str("Q\\n"); }
                }
            } else {''',1);p.write_text(s)
p=Path('03_infra/src/export/stream/text.rs');s=p.read_text();a=s.index('pub(super) fn verbose_block_prefix(');b=s.index('/// **P956** — operador',a);part=s[a:b];needle='''    if let Some(op) = fill.as_ref().and_then(spot_fill_operator) {''';part=part.replace(needle,'''    let alpha = fill.as_ref().map(|c| c.to_rgba_f32().3).unwrap_or(1.0);
    let opacity = super::opacity_operator(alpha, alpha);
'''+needle);part=part.replace('{base_y:.5} cm\\n{op}BT','{base_y:.5} cm\\n{opacity}{op}BT');part=part.replace('{base_y:.5} cm\\n/c0','{base_y:.5} cm\\n{opacity}/c0');s=s[:a]+part+s[b:];p.write_text(s)
p=Path('03_infra/src/export/stream/page.rs');s=p.read_text();needle='''        if let typst_core::entities::paint::Paint::Solid(color) = paint {''';s=s.replace(needle,'''        let opacity = if let typst_core::entities::paint::Paint::Solid(c) = paint {
            super::opacity_operator(c.to_rgba_f32().3, 1.0)
        } else { String::new() };
        if !opacity.is_empty() { ops.push_str("q\\n"); ops.push_str(&opacity); }
'''+needle,1);needle='''        if ctx.ua1 {
            ops.push_str("EMC\\n");
        }''';s=s.replace(needle,'''        if !opacity.is_empty() { ops.push_str("Q\\n"); }
'''+needle,1);p.write_text(s)
