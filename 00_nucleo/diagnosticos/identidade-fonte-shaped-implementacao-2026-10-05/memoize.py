from pathlib import Path
import re,json
p=Path('03_infra/src/export/font_binding.rs');s=p.read_text();at=s.index('fn bind_items(');s=s[:at]+'''/// Local positives, validated once against this export's append-only table.
#[derive(Default)]
pub(crate) struct LookupCache(std::cell::RefCell<Vec<(Font, typst_core::entities::font_book::FontVariant, typst_core::entities::font_variations::FontVariations, usize)>>);
impl LookupCache {
    pub(crate) fn index(&self, fonts: &[FontKey], style: &TextStyle, font: Option<&Font>) -> Option<usize> {
        let Some(font) = font else { return index(fonts, style, None) };
        let variant = text_style_to_font_variant(style);
        let vars = style.variations.clone().unwrap_or_default();
        if let Some((_, _, _, i)) = self.0.borrow().iter().find(|(old, v, axes, _)|
            std::ptr::eq(old.as_slice(), font.as_slice()) && *v == variant && *axes == vars) { return Some(*i); }
        let i = index(fonts, style, Some(font))?;
        self.0.borrow_mut().push((font.clone(), variant, vars, i));
        Some(i)
    }
}
pub(crate) fn lookup(cache: Option<&LookupCache>, fonts: &[FontKey], style: &TextStyle, font: Option<&Font>) -> Option<usize> {
    cache.map_or_else(|| index(fonts, style, font), |c| c.index(fonts, style, font))
}
pub(crate) fn same_font(a: &Option<Font>, b: &Option<Font>) -> bool {
    match (a, b) { (Some(a), Some(b)) => std::ptr::eq(a.as_slice(), b.as_slice()) || a == b,
        (None, None) => true, _ => false }
}

'''+s[at:]
s=s.replace("struct Binder<'a>(&'a mut Vec<FontKey>);","struct Binder<'a>(&'a mut Vec<FontKey>, LookupCache);")
s=s.replace('if index(self.0, style, Some(font)).is_some()','if self.1.index(self.0, style, Some(font)).is_some()')
s=s.replace('Binder(fonts), items','Binder(fonts, LookupCache::default()), items')
s=s.replace('    prepared: Vec<FontKey>,','    prepared: Vec<FontKey>,\n    cache: LookupCache,')
s=s.replace('Self { original, prepared }','Self { original, prepared, cache: LookupCache::default() }')
s=s.replace('if font.is_some() { index(&self.original, style, font)? }','if font.is_some() { self.cache.index(&self.original, style, font)? }')
s=s.replace("fonts: &'a [FontKey] }","fonts: &'a [FontKey], cache: LookupCache }")
s=s.replace('if let Some(i) = index(self.fonts, style, font)','if let Some(i) = self.cache.index(self.fonts, style, font)')
s=s.replace('Distributor { views: &mut views, fonts }','Distributor { views: &mut views, fonts, cache: LookupCache::default() }')
p.write_text(s)
# Optional private cache preserves direct legacy scenario constructors in tests.
for p in Path('03_infra/src').rglob('*.rs'):
 s=p.read_text();edits=[]
 for m in re.finditer(r'FontScenario::Multifont\s*\{',s):
  start=m.end();d=1;i=start
  while d:
   if s[i]=='{':d+=1
   elif s[i]=='}':d-=1
   i+=1
  body=s[start:i-1]
  if '..' in body or 'bindings:'in body or re.match(r'\s*=>',s[i:]):continue
  edits.append((i-1,(''if body.rstrip().endswith(',')else',')+' bindings: None, '))
 if edits:
  for i,e in reversed(edits):s=s[:i]+e+s[i:]
  p.write_text(s)
p=Path('03_infra/src/export/stream.rs');s=p.read_text();marker="fonts: &'a [((FontList, FontVariant, FontVariations), Vec<u8>)],";i=s.index(marker);i+=len(marker);s=s[:i]+'''\n        bindings: Option<crate::export::font_binding::LookupCache>,'''+s[i:]
# actual constructor at end of multifont function; independent test literals remain None.
start=s.index('    pub(crate) fn multifont(');end=s.index('\n}\n',start)
part=s[start:end].replace('bindings: None','bindings: Some(crate::export::font_binding::LookupCache::default())');s=s[:start]+part+s[end:];p.write_text(s)
p=Path('03_infra/src/export/stream/text.rs');s=p.read_text();s=s.replace('''            let Some(fi) = crate::export::font_binding::index(fonts, style, font)''','''            let Some(fi) = crate::export::font_binding::lookup(bindings.as_ref(), fonts, style, font)''')
# Add cache binding only to the arms that use lookup.
for name in ['fn emit_shaped_pdf_inner','fn emit_shaped_pdf_verbose_inner']:
 start=s.index(name);end=s.index('\n}',start);part=s[start:end];part=part.replace('FontScenario::Multifont {\n            fonts,','FontScenario::Multifont {\n            fonts, bindings,');s=s[:start]+part+s[end:]
p.write_text(s)
p=Path('03_infra/src/export/stream/text/runs.rs');s=p.read_text().replace('fa != fb','!crate::export::font_binding::same_font(fa, fb)');s=s.replace('FontScenario::Multifont { fonts, per_font_bitmap, .. }','FontScenario::Multifont { fonts, bindings, per_font_bitmap, .. }');s=s.replace('''FontScenario::Multifont {
            fonts,''','''FontScenario::Multifont {
            fonts, bindings,''');s=s.replace('crate::export::font_binding::index(fonts, style, font.as_ref())','crate::export::font_binding::lookup(bindings.as_ref(), fonts, style, font.as_ref())');p.write_text(s)
p=Path('03_infra/src/export/builder/fonts.rs');s=p.read_text().replace("sets: &'a mut [BTreeSet<u16>],","sets: &'a mut [BTreeSet<u16>],\n        cache: crate::export::font_binding::LookupCache,");s=s.replace('crate::export::font_binding::index(self.fonts, style, font.as_ref())','self.cache.index(self.fonts, style, font.as_ref())');s=s.replace('FontGlyphCollector { fonts, faces, sets: &mut sets }','FontGlyphCollector { fonts, faces, sets: &mut sets, cache: Default::default() }');p.write_text(s)
