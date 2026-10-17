from pathlib import Path
import json,re
H=Path('00_nucleo/diagnosticos/identidade-fonte-shaped-implementacao-2026-10-05');scope=json.loads((H/'scope.json').read_text())
for src in ['03_infra/src/export/svg/groups.rs','03_infra/src/export/svg/navigation.rs']:
 p=Path(src);owner=next(l.split('@prompt ',1)[1]for l in p.read_text().splitlines()if '@prompt 'in l)
 q=Path(owner);q.write_text(q.read_text()+ '\n\n## Binding da face ratificada — 2026-10-05\n\nMedição: `00_nucleo/diagnosticos/identidade-fonte-shaped-2026-10-05/probe.txt`. Transportar o contexto privado PreparedFonts pelos wrappers SVG; preservar geometria/semântica/API pública. Contexto discrimina face original/eixos e fornece bytes preparados, sem resolução nova por nome. O campo TextShaped.font foi confirmado pelo humano.\n')
 scope.append({'source':src,'owner':owner})
scope.append({'source':'03_infra/src/export/font_binding.rs','owner':'00_nucleo/prompts/infra/export/font_binding.md'});(H/'scope.json').write_text(json.dumps(scope,indent=2)+'\n')
Path('03_infra/src/export/font_binding.rs').write_bytes(Path('/tmp/tekt-font-bound/font_binding.rs').read_bytes())
p=Path('03_infra/src/shaper/shaping.rs');s=p.read_text();s=s.replace('font: None,','font: Some(world.font(candidate.slot_idx)?),');p.write_text(s)
p=Path('03_infra/src/export/mod.rs');s=p.read_text();s+='\nmod font_binding;\n';# bind at three concrete multifont entrypoints before parse/fallback
s=s.replace('''    if fonts.is_empty() {''','''    let bound_fonts = font_binding::bind_document(doc, fonts);
    let fonts = bound_fonts.as_ref();
    if fonts.is_empty() {''');p.write_text(s)
p=Path('03_infra/src/export/builder.rs');s=p.read_text();marker='''        if let Some(data) = font_data {''';assert s.count(marker)==1;s=s.replace(marker,'''        let bound = super::font_binding::bind_document(doc, &[]);
        if !bound.is_empty() {
            let faces: Vec<_> = bound.iter().filter_map(|(_, data)| Face::parse(data, 0).ok()).collect();
            if faces.len() == bound.len() { return self.build_multifont(doc, &bound, &faces); }
        }
'''+marker);p.write_text(s)
p=Path('03_infra/src/export/builder/fonts.rs');s=p.read_text();s=s.replace('TextShaped { glyphs, style, .. }','TextShaped { glyphs, style, font, .. }');s=s.replace('''let fi = crate::export::stream::font_index_for_style(self.fonts, style);
                for g''','''let Some(fi) = crate::export::font_binding::index(self.fonts, style, font.as_ref()) else { return };
                for g''');marker='''        for face in faces {''';# occurrence in multifont only
assert s.count(marker)==1
s=s.replace(marker,'''        let bound_views = crate::export::font_binding::partition(doc, fonts);
        for face in faces {
            let view = &bound_views[per_font_mappings.len()];
            let mut chars = collect_codepoints(view);
            let text_chars = collect_text_codepoints(view);
            chars.extend(text_chars.iter().copied());
            let glyph_ids = collect_glyph_ids(view);
            let shaped_mappings = collect_shaped_glyph_mappings(view);
            let shaped_cluster_texts = collect_shaped_cluster_texts(view);''');p.write_text(s)
# append a private font argument to each shaped emitter definition/call.
names=['emit_shaped_pdf','emit_shaped_pdf_deferred','emit_shaped_pdf_inner','emit_shaped_pdf_verbose','emit_shaped_pdf_verbose_deferred','emit_shaped_pdf_verbose_inner']
for src in ['03_infra/src/export/stream/text.rs','03_infra/src/export/stream/draw.rs']:
 p=Path(src);s=p.read_text();edits=[]
 if src.endswith('draw.rs'):s=s.replace('text, units_per_em , .. }','text, units_per_em, font, .. }')
 for m in re.finditer(r'\b('+ '|'.join(names)+r')\s*\(',s):
  depth=1;i=m.end()
  while depth:
   if s[i]=='(':depth+=1
   elif s[i]==')':depth-=1
   i+=1
  content=s[m.end():i-1];isdef=s[max(0,m.start()-4):m.start()].strip()=='fn'
  arg='font: Option<&typst_core::entities::world_types::Font>' if isdef else ('font.as_ref()'if src.endswith('draw.rs') else ('font' if m.start()<s.find('#[cfg(test)]') or '#[cfg(test)]'not in s else 'None'))
  # Existing unit tests can occur after inline test module, detect reference variable scope conservatively.
  if not isdef and src.endswith('text.rs') and m.start()>s.find('#[cfg(test)]'):arg='None'
  sep=''if content.rstrip().endswith(',') else ','
  edits.append((i-1,sep+' '+arg+','))
 for i,e in reversed(edits):s=s[:i]+e+s[i:]
 if src.endswith('text.rs'):
  for fn,end in [('fn emit_shaped_pdf_inner','/// Para um'),('fn emit_shaped_pdf_verbose_inner','#[cfg(test)]')]:
   start=s.index(fn);stop=s.find('\n}',start)+2
   part=s[start:stop].replace('let fi = font_index_for_style(fonts, style);','let Some(fi) = crate::export::font_binding::index(fonts, style, font) else { return false };')
   s=s[:start]+part+s[stop:]
 p.write_text(s)
p=Path('03_infra/src/export/stream/text/runs.rs');s=p.read_text();s=s.replace('glyphs: ga, ..','glyphs: ga, font: fa, ..').replace('glyphs: gb, ..','glyphs: gb, font: fb, ..');s=s.replace('if pa.y != pb.y || ua != ub {','if pa.y != pb.y || ua != ub || fa != fb {');s=s.replace('TextShaped { style, .. } = &items[start]','TextShaped { style, font, .. } = &items[start]');s=s.replace('let fi = font_index_for_style(fonts, style);','let Some(fi) = crate::export::font_binding::index(fonts, style, font.as_ref()) else { return start + 1 };',1);s=s.replace('TextShaped { pos: pos0, style, .. }','TextShaped { pos: pos0, style, font, .. }');s=s.replace('let fi = font_index_for_style(fonts, style);','let Some(fi) = crate::export::font_binding::index(fonts, style, font.as_ref()) else { return };');p.write_text(s)
# PNG/SVG context: public params remain legacy FontKey; private traversal receives prepared bindings.
for src in ['03_infra/src/export/render.rs','03_infra/src/export/svg.rs']:
 p=Path(src);s=p.read_text();s=s.replace('text, units_per_em , .. }','text, units_per_em, font, .. }')
 privstart=s.index('fn render_item(');s=s[:privstart]+s[privstart:].replace('fonts: Option<&[FontKey]>','fonts: Option<&super::font_binding::PreparedFonts>')
 marker='''    let pixel_per_pt = opts.pixel_per_pt.max(0.01);''' if src.endswith('render.rs') else '''    let mut xml = XmlWriter::new'''
 start=s.index('fn render_page_to_png_with_fonts_inner' if src.endswith('render.rs') else 'fn export_svg_with_fonts_inner');i=s.index(marker,start)
 s=s[:i]+'''    let prepared = fonts.map(|f| super::font_binding::PreparedFonts::new(page, f));
    let fonts = prepared.as_ref();
'''+s[i:]
 if src.endswith('render.rs'):
  start=s.index('fn render_text_shaped(');stop=s.index('    let face =',start);part=s[start:stop];part=part.replace('fonts: &[FontKey],','fonts: &super::font_binding::PreparedFonts,\n    font: Option<&typst_core::entities::world_types::Font>,')
  pos=part.index('    let Some(font_list)') if '    let Some(font_list)'in part else part.index('    let font_list')
  part=part[:pos]+'''    let Some((_, font_bytes)) = fonts.get(style, font) else { return };
''';s=s[:start]+part+s[stop:]
  # add arg only the dispatcher call, not definition
  s=s.replace('''                    fonts,
                );''','''                    fonts,
                    font.as_ref(),
                );''',1)
 else:
  s=s.replace('''                    fonts,
                    glyph_defs,
                );''','''                    fonts,
                    glyph_defs,
                    font.as_ref(),
                );''',1)
  s=s.replace('''                    *base_char,
                    fonts,''','''                    *base_char,
                    fonts.fonts(),''')
 p.write_text(s)
for src in ['03_infra/src/export/svg/groups.rs','03_infra/src/export/svg/navigation.rs']:
 p=Path(src);s=p.read_text().replace('fonts: Option<&[FontKey]>','fonts: Option<&super::super::font_binding::PreparedFonts>');p.write_text(s)
p=Path('03_infra/src/export/svg/glyphs.rs');s=p.read_text();start=s.index('pub(super) fn render_text_shaped(');stop=s.index('    if units_per_em == 0',start);part=s[start:stop].replace('fonts: &[FontKey]','fonts: &super::super::font_binding::PreparedFonts').replace('glyph_defs: &mut GlyphDefs,','glyph_defs: &mut GlyphDefs,\n    font: Option<&typst_core::entities::world_types::Font>,');pos=part.index('    let Some(font_list)');part=part[:pos]+'''    let Some((font_idx, font_bytes)) = fonts.get(style, font) else { return };

''';s=s[:start]+part+s[stop:];p.write_text(s)
