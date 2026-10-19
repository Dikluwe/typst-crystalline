from pathlib import Path
import json,gzip,hashlib,re,subprocess,datetime,difflib
out=Path('00_nucleo/diagnosticos/clippy-aridade-interna-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));baseline=json.loads((out/'baseline-details.json').read_text());assert baseline['commands'][-1]['selected_all_observed'] and baseline['commands'][-1]['exit']==101
for source in before['owners']:assert hashlib.sha256(Path(source).read_bytes()).hexdigest()==before['files'][source]['sha256'],source
records=[]
def edit(path,old,new,count=1):
 p=Path(path);s=p.read_text();assert s.count(old)==count,(path,s.count(old),old[:100]);p.write_text(s.replace(old,new))
def function_span(s,marker):
 start=s.index(marker);opening=s.index('{',start);depth=1;at=opening+1
 while depth:
  depth+=int(s[at]=='{')-int(s[at]=='}');at+=1
 return start,opening,at
# Two truly unused private arguments; the authorial carrier stays intact.
for source,id,visibility,reason in [
('03_infra/src/export/render.rs','C047','','C047: Leaf raster usa oito inputs efetivos de pintura; texto autoral não é transportado para PNG'),
('03_infra/src/export/svg/glyphs.rs','C066','pub(super) ','C066: Leaf SVG usa oito inputs efetivos de contorno, face, escala e definições; texto HTML segue writer separado')]:
 edit(source,'    _text: &ecow::EcoString,\n','')
 marker=visibility+'fn render_text_shaped('
 attr=f'#[expect(clippy::too_many_arguments, reason = {json.dumps(reason,ensure_ascii=False)})]\n'
 edit(source,marker,attr+marker)
edit('03_infra/src/export/render.rs','pos, glyphs, style, text, units_per_em, font, ..','pos, glyphs, style, units_per_em, font, ..')
for source in ['03_infra/src/export/render.rs','03_infra/src/export/svg.rs']:
 edit(source,'                    style,\n                    text,\n                    *units_per_em,','                    style,\n                    *units_per_em,')
# Private typed request. Preserve the entire old implementation body after destructuring.
p='03_infra/src/pipeline/pdf.rs'
old='''fn compile_to_pdf_bytes_impl(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tagged_override: Option<bool>,
    features: typst_core::entities::compiler_features::Features,
    pretty_override: Option<bool>,
    pages_override: Option<PageRanges>,
    standard_override: Option<PdfStandards>,
    oracle: bool,
    timings: &mut Timings,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
'''
new='''struct PdfCompileRequest {
    full_error: bool,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tagged_override: Option<bool>,
    features: typst_core::entities::compiler_features::Features,
    pretty_override: Option<bool>,
    pages_override: Option<PageRanges>,
    standard_override: Option<PdfStandards>,
    oracle: bool,
}

fn compile_to_pdf_bytes_impl(
    world: &dyn World,
    source: &Source,
    request: PdfCompileRequest,
    timings: &mut Timings,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    let PdfCompileRequest {
        full_error,
        document_id,
        stream_mode,
        pdf_tagged_override,
        features,
        pretty_override,
        pages_override,
        standard_override,
        oracle,
    } = request;
'''
edit(p,old,new)
for flag,count in [('false',2),('true',1)]:
 oldcall='''compile_to_pdf_bytes_impl(
        world,
        source,
        full_error,
        document_id,
        stream_mode,
        pdf_tagged_override,
        features,
        pretty_override,
        pages_override,
        standard_override,
        '''+flag+''',
        &mut timings,
    )'''
 newcall='''compile_to_pdf_bytes_impl(
        world,
        source,
        PdfCompileRequest {
            full_error,
            document_id,
            stream_mode,
            pdf_tagged_override,
            features,
            pretty_override,
            pages_override,
            standard_override,
            oracle: '''+flag+''',
        },
        &mut timings,
    )'''
 edit(p,oldcall,newcall,count)
# One collector owns the synchronized state; geometric context stays per recursion.
p=Path('03_infra/src/export/gradients/mod.rs');s=p.read_text();scan_start=s.index('pub(super) fn scan_all_gradients(');walk_start=s.index('    fn walk(',scan_start);locals_start=s.index('    let mut ptr_to_idx:',walk_start);walk=s[walk_start:locals_start]
oldparams='''        ptr_to_idx: &mut HashMap<DedupKey, usize>,
        refs: &mut Vec<PatternRef>,
        grad_objs: &mut Vec<GradientObject>,
        next_id: &mut usize,
        counter: &mut usize,
'''
assert walk.count(oldparams)==1;walk=walk.replace(oldparams,'        collector: &mut Collector,\n')
pattern=r'(?m)^([ \t]*)ptr_to_idx,\n\1refs,\n\1grad_objs,\n\1next_id,\n\1counter,\n'
walk,n=re.subn(pattern,lambda m:m[1]+'collector,\n',walk);assert n==4,n
walk=walk.replace('*next_id','next_id').replace('*counter','counter');walk=re.sub(r'\b(ptr_to_idx|refs|grad_objs|next_id|counter)\b',lambda m:'collector.'+m[1],walk)
collector='''    struct Collector {
        ptr_to_idx: HashMap<DedupKey, usize>,
        refs: Vec<PatternRef>,
        grad_objs: Vec<GradientObject>,
        next_id: usize,
        counter: usize,
    }

'''
s=s[:walk_start]+collector+walk+s[locals_start:]
oldlocals='''    let mut ptr_to_idx: HashMap<DedupKey, usize> = HashMap::new();
    let mut refs: Vec<PatternRef> = Vec::new();
    let mut grad_objs: Vec<GradientObject> = Vec::new();
    let mut next_id = first_id;
    let mut counter = 1usize;'''
newlocals='''    let mut collector = Collector {
        ptr_to_idx: HashMap::new(),
        refs: Vec::new(),
        grad_objs: Vec::new(),
        next_id: first_id,
        counter: 1,
    };'''
assert s.count(oldlocals)==1;s=s.replace(oldlocals,newlocals)
oldroot='''                &mut ptr_to_idx,
                &mut refs,
                &mut grad_objs,
                &mut next_id,
                &mut counter,
'''
assert s.count(oldroot)==1;s=s.replace(oldroot,'                &mut collector,\n')
oldret='    (refs, ptr_to_idx, grad_objs)\n';assert s.count(oldret)==1;s=s.replace(oldret,'    (collector.refs, collector.ptr_to_idx, collector.grad_objs)\n');p.write_text(s)
# Common PageContext constructor consumes the already typed existing scenario.
p=Path('03_infra/src/export/stream.rs');s=p.read_text();start,opening,end=function_span(s,'    pub(crate) fn type1(');old_type1=s[start:end]
new_type1='''    pub(crate) fn type1(
        ptr_to_idx: &'a HashMap<usize, usize>,
        img_refs: &'a [ImageRef],
        pat_ptr_to_idx: &'a HashMap<DedupKey, usize>,
        pat_refs: &'a [PatternRef],
        mode: StreamMode,
    ) -> Self {
        Self::with_font_scenario(
            ptr_to_idx,
            img_refs,
            pat_ptr_to_idx,
            pat_refs,
            FontScenario::Type1,
            mode,
        )
    }'''
s=s[:start]+new_type1+s[end:]
for method in ['cidfont','multifont']:
 start,opening,end=function_span(s,'    pub(crate) fn '+method+'(');records.append(dict(removed_internal_constructor=method,old_source=s[start:end]));s=s[:start]+s[end:]
constructor='''    pub(crate) fn with_font_scenario(
        ptr_to_idx: &'a HashMap<usize, usize>,
        img_refs: &'a [ImageRef],
        pat_ptr_to_idx: &'a HashMap<DedupKey, usize>,
        pat_refs: &'a [PatternRef],
        font_scenario: FontScenario<'a>,
        mode: StreamMode,
    ) -> Self {
        Self {
            ptr_to_idx,
            img_refs,
            pat_ptr_to_idx,
            pat_refs,
            tiling_names: None,
            font_scenario,
            mode,
            tags: PdfTags::Disabled,
            semantic_mcids: HashMap::new(),
            ua1: false,
            oracle: false,
        }
    }

'''
anchor="impl<'a> PageContext<'a> {\n";assert s.count(anchor)==1;s=s.replace(anchor,anchor+constructor);p.write_text(s)
# Migrate the sole CIDFont fixture; all expected operators remain verbatim.
old='''PageContext::cidfont(
            &ptr,
            &imgs,
            &pats,
            &pat_refs,
            &char_to_gid,
            &glyph_mapping,
            &glyph_to_nominal,
            None,
            StreamMode::Verbose,
        )'''
new='''PageContext::with_font_scenario(
            &ptr,
            &imgs,
            &pats,
            &pat_refs,
            FontScenario::Cidfont {
                char_to_gid: &char_to_gid,
                glyph_mapping: &glyph_mapping,
                glyph_to_nominal: &glyph_to_nominal,
                bitmap: None,
            },
            StreamMode::Verbose,
        )'''
edit(str(p),old,new)
p='03_infra/src/export/builder/fonts.rs';edit(p,'use crate::export::PageContext;','use crate::export::PageContext;\nuse crate::export::stream::FontScenario;')
old='''PageContext::cidfont(
                &ptr_to_idx,
                &img_refs,
                &pat_ptr_to_idx,
                &pat_refs,
                &char_to_gid,
                &glyph_mapping,
                &glyph_to_nominal,
                if bitmap_only { Some(&bitmap_refs) } else { None },
                self.stream_mode,
            )'''
new='''PageContext::with_font_scenario(
                &ptr_to_idx,
                &img_refs,
                &pat_ptr_to_idx,
                &pat_refs,
                FontScenario::Cidfont {
                    char_to_gid: &char_to_gid,
                    glyph_mapping: &glyph_mapping,
                    glyph_to_nominal: &glyph_to_nominal,
                    bitmap: if bitmap_only { Some(&bitmap_refs) } else { None },
                },
                self.stream_mode,
            )'''
edit(p,old,new)
old='''PageContext::multifont(
                &ptr_to_idx,
                &img_refs,
                &pat_ptr_to_idx,
                &pat_refs,
                fonts,
                &per_font_char_to_gid,
                &per_font_glyph_mapping,
                &per_font_glyph_to_nominal,
                &per_font_glyph_reverse,
                &per_font_bitmap,
                self.stream_mode,
            )'''
new='''PageContext::with_font_scenario(
                &ptr_to_idx,
                &img_refs,
                &pat_ptr_to_idx,
                &pat_refs,
                FontScenario::Multifont {
                    fonts,
                    per_font_char_to_gid: &per_font_char_to_gid,
                    per_font_glyph_mapping: &per_font_glyph_mapping,
                    per_font_glyph_to_nominal: &per_font_glyph_to_nominal,
                    per_font_glyph_reverse: &per_font_glyph_reverse,
                    per_font_bitmap: &per_font_bitmap,
                    bindings: Some(crate::export::font_binding::LookupCache::default()),
                },
                self.stream_mode,
            )'''
edit(p,old,new)
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*before['owners']],check=True)
(out/'materialization.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),records=records),ensure_ascii=False,indent=2)+'\n')
(out/'stage-before-seal.diff').write_text(''.join(''.join(difflib.unified_diff(v['text'].splitlines(True),Path(p).read_text().splitlines(True),fromfile=p+' (before)',tofile=p)) for p,v in before['files'].items()))
print('Materializados cleanup de dois argumentos, pedido PDF, Collector e composição FontScenario.')
