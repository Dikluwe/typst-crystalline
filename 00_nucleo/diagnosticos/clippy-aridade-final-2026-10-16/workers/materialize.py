from pathlib import Path
import gzip,json,re,hashlib,datetime,subprocess
out=Path('00_nucleo/diagnosticos/clippy-aridade-final-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'))
assert json.loads((out/'baseline-result.json').read_text())['exit']==0
receipt=json.loads((out/'l0-before-code.json').read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
assert all(sha(p)==h for p,h in receipt['owners'].items())
paths=[p for p in before['files'] if p.endswith('.rs')];texts={p:Path(p).read_text() for p in paths};assert all(texts[p]==before['files'][p] for p in paths)
edits=[]
def edit(p,old,new):
 assert texts[p].count(old)==1,(p,old,texts[p].count(old))
 texts[p]=texts[p].replace(old,new,1);edits.append(dict(path=p,old=old,new=new))
# Shared grid engine: exact twelve inputs, borrowed and named.
p=paths[0];names=['columns','rows','cells','hlines','vlines','gutter','align','inset','header','footer','stroke','fill']
types=["&'b [TrackSizing]","&'b [TrackSizing]","&'b [Content]","&'b [H]","&'b [V]",'Option<Length>','Option<Align2D>','Sides<Length>',"Option<&'b Content>","Option<&'b Content>","Option<&'b Stroke>","Option<&'b Color>"]
struct="pub(super) struct GridLayoutInput<'b, H: LayoutHLine, V: LayoutVLine> {\n"+''.join(f'    pub(super) {n}: {t},\n' for n,t in zip(names,types))+'}\n\n'
edit(p,'/// Layout de `grid(...)`',struct+'/// Layout de `grid(...)`')
call_maps=[]
def call_block(path,start,fields,carrier,keep=0,offset=0):
 t=texts[path];a=t.index(start,offset);opening=a+len(start)-1;depth=1;i=opening+1
 while depth:
  if t[i]=='(':depth+=1
  elif t[i]==')':depth-=1
  i+=1
 old=t[a:i];s=t[opening+1:i-1];args=[];st=0;stack=0
 for j,c in enumerate(s):
  if c in '([{':stack+=1
  elif c in ')]}':stack-=1
  elif c==',' and stack==0:
   if s[st:j].strip():args.append(s[st:j].strip())
   st=j+1
 if s[st:].strip():args.append(s[st:].strip())
 assert len(args)==len(fields)+keep,(path,args)
 new=start+'\n'+''.join('    '+v+',\n' for v in args[:keep])+carrier+' {\n'+''.join('    '+n+': '+v+',\n' for n,v in zip(fields,args[keep:]))+'},\n)'
 edit(path,old,new)
 call_maps.append(dict(path=path,callee=start,fields=dict(zip(fields,args[keep:])),positional=args[:keep]))
 return a+len(new)
call_block(p,'layouter.layout_grid(',names,'GridLayoutInput')
a=texts[p].index('    pub(super) fn layout_grid<');b=texts[p].index('        // P232 — save/restore',a)
edit(p,texts[p][a:b],'''    pub(super) fn layout_grid<H: LayoutHLine, V: LayoutVLine>(
        &mut self,
        input: GridLayoutInput<'_, H, V>,
    ) {
        let GridLayoutInput {
            columns, rows, cells, hlines, vlines, gutter, align, inset,
            header, footer, stroke, fill,
        } = input;
''')
call_block(paths[1],'layouter.layout_grid(',names,'super::grid::GridLayoutInput')
# Outline plan: same arguments in the productive call and both fixtures.
p=paths[2];names=['prefix_width_pt','base','hanging_pt','auto','parent_key','level','rtl'];types=['Option<f64>','crate::entities::layout_types::Length','Option<f64>','bool','i64','usize','bool']
struct='struct OutlineIndentPlan {\n'+''.join(f'    {n}: {t},\n' for n,t in zip(names,types))+'}\n\n'
edit(p,'fn compose_outline_entry_indented(',struct+'fn compose_outline_entry_indented(')
a=texts[p].index('fn compose_outline_entry_indented(');b=texts[p].index('    use crate::entities::layout_types::Length;',a)
edit(p,texts[p][a:b],'''fn compose_outline_entry_indented(
    prefix: Option<Content>,
    inner: Content,
    plan: OutlineIndentPlan,
) -> Content {
    let OutlineIndentPlan {
        prefix_width_pt, base, hanging_pt, auto, parent_key, level, rtl,
    } = plan;
''')
a=texts[p].index('let result = Value::Content(')
for _ in range(3):a=call_block(p,'compose_outline_entry_indented(',names,'OutlineIndentPlan',2,a)
# Root SVG session, borrowed by descendant renderers.
p=paths[3]
edit(p,'::{FrameItem, Page, TransformMatrix}', '::{FrameItem, Page, Pt, Size, TransformMatrix}')
struct='''struct SvgRenderSession<'a> {
    fonts: Option<&'a super::font_binding::PreparedFonts>,
    glyph_defs: &'a mut GlyphDefs,
    paint_defs: &'a mut PaintDefs,
    clip_next_id: &'a mut usize,
    destinations: &'a SvgDestinationContext,
    glyph_fonts: &'a SvgGlyphFontContext,
    html_frame: bool,
}

'''
edit(p,'fn render_item(',struct+'fn render_item(')
edit(p,'    let mut clip_next_id = 0usize;','''    let mut clip_next_id = 0usize;
    let mut session = SvgRenderSession {
        fonts,
        glyph_defs: &mut glyph_defs,
        paint_defs: &mut paint_defs,
        clip_next_id: &mut clip_next_id,
        destinations,
        glyph_fonts,
        html_frame,
    };''')
a=texts[p].index('        render_item(');b=texts[p].index('        );',a)+len('        );')
edit(p,texts[p][a:b],'        render_item(&mut xml, item, &mut session);')
edit(p,'    paint_defs.write(&mut xml, fonts, &mut glyph_defs, &mut clip_next_id, glyph_fonts);\n    glyph_defs.write(&mut xml);','''    session.paint_defs.write(
        &mut xml, session.fonts, session.glyph_defs,
        session.clip_next_id, session.glyph_fonts,
    );
    session.glyph_defs.write(&mut xml);''')
a=texts[p].index('fn render_item(');b=texts[p].index('    match item {',a)
edit(p,texts[p][a:b],'''fn render_item(
    xml: &mut XmlWriter,
    item: &FrameItem,
    session: &mut SvgRenderSession<'_>,
) {
''')
a=texts[p].index('    match item {',texts[p].index('fn render_item('));body=texts[p][a:]
old='''                render_item(
                    xml,
                    item,
                    fonts,
                    glyph_defs,
                    paint_defs,
                    clip_next_id,
                    destinations,
                    glyph_fonts,
                    false,
                );'''
assert body.count(old)==1;body=body.replace(old,'                render_item(xml, item, session);')
old=old.replace('                    false,','                    html_frame,');assert body.count(old)==1;body=body.replace(old,'                render_item(xml, item, session);')
a2=body.index('            render_group(');b2=body.index('            );',a2)+len('            );')
body=body[:a2]+'''            render_group(
                xml, pos, matrix, clip_mask.as_ref(),
                Size { width: Pt(*inner_width), height: Pt(*inner_height) },
                items, session,
            );'''+body[b2:]
a2=body.index('            render_link(');b2=body.index('            );',a2)+len('            );')
body=body[:a2]+'            render_link(xml, target, items, pos, size, session);'+body[b2:]
for name in ['glyph_defs','paint_defs','clip_next_id','glyph_fonts','html_frame']:body=re.sub(r'\b'+name+r'\b','session.'+name,body)
body=body.replace('= fonts {','= session.fonts {')
anchor='''        FrameItem::Link { items, .. } if session.paint_defs.decorative => {
            for item in items {
                render_item(xml, item, session);
            }
        }'''
assert body.count(anchor)==1;body=body.replace(anchor,'''        FrameItem::Link { items, .. } if session.paint_defs.decorative => {
            let previous_html_frame = session.html_frame;
            session.html_frame = false;
            for item in items {
                render_item(xml, item, session);
            }
            session.html_frame = previous_html_frame;
        }''')
edit(p,texts[p][a:],body)
# Group/navigation functions, unchanged private geometry helpers / destination API.
for p in paths[4:6]:
 edit(p,'use super::glyphs::GlyphDefs;\n','');edit(p,'use super::paints::PaintDefs;\n','')
p=paths[4]
edit(p,'use super::{render_item, SvgDestinationContext, SvgGlyphFontContext};','use super::{render_item, SvgRenderSession};')
edit(p,'::{FrameItem, Point, Pt, TransformMatrix}', '::{FrameItem, Point, Pt, Size, TransformMatrix}')
a=texts[p].index('    inner_width: f64,');b=texts[p].index('    let clip =',a)
edit(p,texts[p][a:b],'''    size: Size,
    items: &[FrameItem],
    session: &mut SvgRenderSession<'_>,
) {
    let inner_width = size.width.0;
    let inner_height = size.height.0;
''')
edit(p,'write_clip_definition(xml, kind, inner_width, inner_height, clip_next_id)','write_clip_definition(xml, kind, inner_width, inner_height, session.clip_next_id)')
for p in paths[4:6]:
 a=texts[p].index('        render_item(');b=texts[p].index('        );',a)+len('        );')
 edit(p,texts[p][a:b],'        render_item(xml, item, session);')
p=paths[5]
edit(p,'use super::{render_item, SvgGlyphFontContext};','use super::{render_item, SvgRenderSession};')
a=texts[p].index('    fonts: Option<',texts[p].index('pub(super) fn render_link('));b=texts[p].index('    xml.start_element("a");',a)
edit(p,texts[p][a:b],'''    session: &mut SvgRenderSession<'_>,
) {
''')
edit(p,'if let Some(id) = destinations.id_for(label)', 'if let Some(id) = session.destinations.id_for(label)')
# Tiling view reborrows the same underlying mutable state.
p=paths[6];a=texts[p].index('                for item in &cell.frame.items {');b=texts[p].index('                (self.local_to_root, self.decorative) = previous;',a)
edit(p,texts[p][a:b],'''                {
                    let destinations = super::SvgDestinationContext::empty();
                    let mut session = super::SvgRenderSession {
                        fonts,
                        glyph_defs: &mut *glyph_defs,
                        paint_defs: &mut *self,
                        clip_next_id: &mut *clip_next_id,
                        destinations: &destinations,
                        glyph_fonts,
                        html_frame: false,
                    };
                    for item in &cell.frame.items {
                        super::render_item(xml, item, &mut session);
                    }
                }
''')
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),edits=edits,call_maps=call_maps,unformatted=texts)
(out/'materialization.json').write_text(json.dumps(record,indent=2)+'\n')
for p in paths:Path(p).write_text(texts[p])
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*paths],check=True)
print(json.dumps(dict(sources=paths,call_maps=call_maps)),flush=True)
