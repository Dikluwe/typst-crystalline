from pathlib import Path
import gzip,json,collections,hashlib,os
D=Path(__file__).resolve().parent
avail=os.sched_getaffinity(0)
if len(avail)>1:os.sched_setaffinity(0,{sorted(avail)[1]})
source=D/('profiles-normalized.json.gz'if(D/'profiles-normalized.json.gz').exists()else'profiles-symbolized.json.gz');x=json.load(gzip.open(source,'rt'))
def has(fs,terms):return any(t in f for f in fs for t in terms)
def group(fs):
 catalog=has(fs,['world::fonts::fresh_catalog','world::fonts::prepared_catalog','world::fonts::combine_catalog','typst_infra::fontdb::','fonts::coverage::','font_info_from_bytes','fontdb::Database::load','typst_kit::fonts::FontSearcher::search','typst::fonts::discover_fonts','typst_kit::fonts::system'])
 if catalog:
  if has(fs,['fonts::coverage::','Coverage::from_ranges']):return 'catalog.coverage'
  if has(fs,['fontdb::Database::load','fontdb::Database::parse','fontdb::parse_face_info']):return 'catalog.discovery'
  if has(fs,['font_info_from_bytes','font_info_with_coverage','FontInfo::from_ttf','FontInfo::new','ttf_parser::Face::parse','ttf_parser::Face::from_raw']):return 'catalog.metadata_parser'
  if has(fs,['fontdb::selective::']):return 'catalog.selective_other'
  if has(fs,['world::fonts::combine_catalog']):return 'catalog.composition'
  return 'catalog.other'
 if has(fs,['typst_infra::export::','typst_pdf::','typst_pdf..']):return 'export'
 if has(fs,['typst_infra::shaper::','rustybuzz::','typst_library::text::shape']):return 'shape'
 if has(fs,['compiler::layout','compiler..layout','typst_layout::','typst_layout..']):return 'layout'
 if has(fs,['compiler::eval','compiler..eval','typst_eval::','typst_eval..']):return 'eval_residual'
 if has(fs,['typst_infra::pipeline::pdf::compile_to_pdf_bytes_impl']):return 'pdf_outer_residual'
 return 'other'
def symbols(row,sample):
 out=[]
 for idx,a in sample['addresses']:
  m=row['modules'].get(str(idx),{});path=m.get('path','');symbol=x['symbols'].get(path,{}).get(format(a,'x'),{});out.append({'module':path,'address':format(a,'x'),'function':symbol.get('function','??'),'location':symbol.get('location','??')})
 return out
anchors={'math_reverse':['build_math_glyph_reverse_map'],'style_collapse':['StyleChain::collapse'],'style_capture':['local_delta_since'],'map_content':['map_content_pre','::map_content','..map_content'],'font_selection':['FontBook::select_pattern','FontBook::select_fallback'],'hash_one':['BuildHasher::hash_one'],'clone':['clone..Clone','::clone::'],'font_binding':['FontGlyphCollector','font_binding','collect_glyphs_per_font'],'table_grid':['layout::grid','layout::table','typst_layout::grid']}
summary={}
for e in ['tekt','orig']:
 for case in sorted({r['fixture']for r in x['runs']}):
  rs=[r for r in x['runs']if r['engine']==e and r['fixture']==case];partition=collections.Counter();leaf=collections.Counter();first=collections.Counter();inclusive=collections.Counter();subleaf=collections.defaultdict(collections.Counter);quality=collections.Counter();per_run=[]
  for row in rs:
   local=collections.Counter()
   for s in row['samples']:
    sy=symbols(row,s);fs=[v['function']for v in sy];g=group(fs);partition[g]+=1;local[g]+=1;lp=sy[0];leaf[(lp['function'],lp['module'])]+=1;subleaf[g][(lp['function'],lp['module'])]+=1
    quality['truncated']+=s['depth']>=row['header']['depth'];quality['unknown_leaf']+=lp['function']=='??';quality['leaf_in_executable']+=lp['module'].endswith('/'+e+'/typst');quality['samples']+=1
    project=[v['function']for v in sy if v['module'].endswith('/'+e+'/typst')]
    if project:first[project[0]]+=1
    for name,terms in anchors.items():
     if has(fs,terms):inclusive[name]+=1
   per_run.append({'rep':row['rep'],'samples':len(row['samples']),'partition':dict(local),'delivered':row['header']['delivered'],'reserved':row['header']['reserved'],'truncated':sum(s['depth']>=row['header']['depth']for s in row['samples'])})
  total=sum(partition.values());summary[e+'/'+case]={'samples':total,'partition':dict(partition),'catalog_fraction':sum(n for k,n in partition.items()if k.startswith('catalog.'))/total,'inclusive_anchors':dict(inclusive),'interrupted_pc_top':[{'function':n,'module':m,'samples':c}for(n,m),c in leaf.most_common(25)],'first_executable_frame_top':first.most_common(25),'group_pc_top':{g:[{'function':n,'module':m,'samples':c}for(n,m),c in counter.most_common(15)]for g,counter in subleaf.items()},'quality':dict(quality),'per_run':per_run}
result={'input':source.name,'input_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'driver_text':Path(__file__).read_text(),'driver_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'priority':['catalog coverage/discovery/metadata-parser/selective-other/composition/other','export','shape','layout','eval_residual','pdf_outer_residual','other'],'scope':'Partitions are stack-presence ownership by explicit priority, not exclusive durations. Interrupted PC tops map the actual interrupted instruction; libc syscall PCs cannot certify exclusive kernel/I/O time. Unknown/inlined/truncated frames and sampler bias remain. Inclusive anchors overlap and must not be summed.','summary':summary}
(D/'classification.json').write_text(json.dumps(result,ensure_ascii=False,indent=2))
for n,s in summary.items():print(n,s['samples'],round(s['catalog_fraction']*100,1),s['partition'],s['quality'],flush=True)
