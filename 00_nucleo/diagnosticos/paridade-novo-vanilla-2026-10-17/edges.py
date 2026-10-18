from pathlib import Path
s=Path(__file__).with_name('measure.py').read_text();exec(compile(s[:s.index("\nflat_eval('tekt-v16")],str(Path(__file__).with_name('measure.py')),'exec'))
svg=work/'feature.svg';svg.write_text('<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"><rect width="10" height="10" fill="red"/></svg>');remember(svg)
pdf=work/'feature.pdf';m=load(out/'measurement.json.gz');r=next(c for c in m['cases'] if c['group']=='text-render' and c['label']=='named-abs')['results']['tekt'];pdf.write_bytes(Path(r['command'][3]).read_bytes())
paint='tiling(size:(4pt,4pt))[#rect(width:2pt,height:2pt,fill:red,stroke:none)]'
cases={
 'text-tiling-named':('#text(fill:'+paint+')[A]','pdf',None),
 'text-tiling-positional':('#text('+paint+')[A]','pdf',None),
 'svg-source':('#image("feature.svg")','pdf',None),
 'pdf-source':('#image("feature.pdf")','pdf',None),
 'html-mathml':('$x^2$','html',['html']),
 'html-span-string':('#html.elem("span","A")','html',['html']),
 'html-span-content-control':('#html.elem("span",[A])','html',['html']),
 'html-style-content':('#html.elem("style",[</abcd−])','html',['html']),
}
for name,(body,fmt,flags) in cases.items():
 p=work/(name+'.typ');p.write_text(body);add_compile('edges',name,p,fmt,flags,True)
records=[]
for i,spec in enumerate(specs):
 results={}
 for name in ['tekt','vanilla','vanilla_old']:
  args=spec['args'][:];folder=work/f'edge-{i:03d}-{name}';folder.mkdir(exist_ok=True);dest=folder/('result.'+spec['fmt']);args[args.index('OUTPUT')]=str(dest);r=execute([bins[name]['path'],*args]);observe_file(dest,r);results[name]=r
 records.append(dict(spec,results=results));print(spec['label'],{n:(r['exit'],r['errors']) for n,r in results.items()},flush=True)
with gzip.open(out/'edges.json.gz','wt') as f:json.dump({'utc':now(),'cases':records,'fixtures':fixtures,'pdf_source_sha256':sha(pdf)},f,ensure_ascii=False)
