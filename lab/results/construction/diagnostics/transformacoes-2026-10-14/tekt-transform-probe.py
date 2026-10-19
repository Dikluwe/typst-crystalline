from pathlib import Path
import subprocess,json,gzip,hashlib,datetime,re
root=Path.cwd();out=root/'00_nucleo/diagnosticos/transformacoes-2026-10-14';out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def git(*args):return subprocess.check_output(['git',*args],text=True)
files={str(p):sha(p) for top in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'] for p in Path(top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']}
sources=['01_core/src/entities/elements/transform.rs','01_core/src/compiler/stdlib/transforms.rs','01_core/src/compiler/layout/transform.rs','01_core/src/entities/content/construction/layout.rs','01_core/src/compiler/eval/observation/elements.rs','01_core/src/entities/content/identity.rs','01_core/src/entities/content/fields.rs','lab/typst-original/crates/typst-library/src/layout/transform.rs','lab/typst-original/crates/typst-layout/src/transforms.rs']
for p in sources.copy():
 m=re.search(r'^//! @prompt (.+)$',Path(p).read_text(),re.M)
 if m:sources.append(m[1])
bins={'tekt':root/'target/debug/typst','vanilla':root/'lab/typst-original/target/release/typst'}
assert sha(bins['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
with gzip.open(root/'00_nucleo/diagnosticos/png-gradientes-2026-10-14/final-v2/results.json.gz','rt') as f:previous=json.load(f)
assert all(sha(p)==previous['binaries'][k]['sha256'] for k,p in bins.items())
record=dict(start=now(),head=git('rev-parse','HEAD').strip(),status=git('status','--porcelain=v1'),diffstat=git('diff','HEAD','--stat'),files=files,sources={p:Path(p).read_text() for p in sources},binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in bins.items()},binary_source_provenance='png-gradientes-2026-10-14/candidate-final.json.gz',rows=[])
with gzip.open(out/'before.json.gz','wt') as f:json.dump(record,f)
queries={
 'rotate-fields-empty':'rotate[X].fields()',
 'rotate-fields-explicit':'rotate(23deg,origin:top+left,reflow:false)[X].fields()',
 'rotate-field-angle':'rotate[X].angle',
 'rotate-field-origin':'rotate[X].origin',
 'rotate-func':'rotate(0deg)[X].func()==rotate',
 'rotate-eq-explicit':'rotate[X]==rotate(0deg)[X]',
 'rotate-eq-reflow':'rotate(0deg)[X]==rotate(0deg,reflow:false)[X]',
 'rotate-eq-origin':'rotate(0deg)[X]==rotate(0deg,origin:center+horizon)[X]',
 'different-functions':'rotate(0deg)[X]==scale(100%)[X]',
 'move-fields':'move(dx:50%+1em,dy:2pt)[X].fields()',
 'move-func':'move[X].func()==move',
 'scale-fields-empty':'scale[X].fields()',
 'scale-fields-factor':'scale(150%)[X].fields()',
 'scale-fields-x':'scale(x:150%)[X].fields()',
 'scale-fields-auto':'scale(x:80pt,y:auto)[X].fields()',
 'scale-func':'scale[X].func()==scale',
 'scale-repr':'scale(x:80pt,y:auto,origin:bottom+left,reflow:true)[X]',
 'skew-fields':'skew(ax:12deg,ay:-3deg,origin:top+right,reflow:true)[X].fields()',
 'skew-func':'skew[X].func()==skew',
 'rotate-repr':'rotate(23deg,origin:top+left,reflow:true)[X]',
 'move-repr':'move(dx:20%,dy:1em)[X]',
 'rotate-angle-int':'rotate(1)[X]',
 'rotate-angle-float':'rotate(1.0)[X]',
 'rotate-angle-string':'rotate("bad")[X]',
 'rotate-origin-string':'rotate(23deg,origin:"top-left")[X]',
 'rotate-origin-axis':'rotate(23deg,origin:top)[X].fields()',
 'rotate-reflow-string':'rotate(23deg,reflow:"yes")[X]',
 'rotate-unknown':'rotate(23deg,unknown:true)[X]',
 'rotate-extra':'rotate(23deg,4deg)[X]',
 'rotate-body-string':'rotate(23deg,"X").fields()',
 'rotate-body-named':'rotate(23deg,body:[X]).fields()',
 'scale-int':'scale(2)[X]',
 'scale-float':'scale(1.5)[X]',
 'scale-two-factors':'scale(150%,50%)[X]',
 'scale-factor-and-x':'scale(150%,x:50%)[X].fields()',
 'scale-relative':'scale(x:50%+1pt)[X]',
 'scale-both-auto':'scale(x:auto,y:auto)[X].fields()',
 'scale-unknown':'scale(150%,unknown:true)[X]',
 'move-int':'move(dx:2)[X]',
 'move-string':'move(dx:"bad")[X]',
 'move-unknown':'move(dx:2pt,unknown:true)[X]',
 'move-body-string':'move("X").fields()',
 'skew-positional':'skew(12deg)[X]',
 'skew-float':'skew(ax:0.2)[X]',
 'skew-near90':'skew(ax:89.99deg)[X].fields()',
 'skew-90':'skew(ax:90deg)[X].fields()',
 'skew-large':'skew(ax:100deg)[X].fields()',
}
qdir=out/'queries';qdir.mkdir()
for name,expr in queries.items():
 src=qdir/(name+'.typ');src.write_text('#metadata(repr('+expr+')) <probe>\n')
 for key,exe in bins.items():
  cmd=[str(exe),'query',str(src),'<probe>','--field','value','--one'];start=now();r=subprocess.run(cmd,capture_output=True,text=True,timeout=60)
  row=dict(case=name,kind='query',binary=key,command=cmd,start=start,end=now(),exit=r.returncode,stdout=r.stdout,stderr=r.stderr,source=src.read_text());record['rows'].append(row)
 print('query',name,flush=True)
rdir=out/'renders';rdir.mkdir();body='rect(width:40pt,height:30pt,fill:red,stroke:none)'
cases={
 'plain':'#'+body,
 'rotate-default':'#rotate(23deg)['+'#'+body+']',
 'rotate-top-left':'#rotate(23deg,origin:top+left)['+'#'+body+']',
 'rotate-bottom-right':'#rotate(23deg,origin:bottom+right)['+'#'+body+']',
 'rotate-reflow':'#rotate(23deg,reflow:true)['+'#'+body+']#rect(width:5pt,height:5pt,fill:blue,stroke:none)',
 'scale-factor':'#scale(150%)['+'#'+body+']',
 'scale-x':'#scale(x:150%)['+'#'+body+']',
 'scale-top-left':'#scale(x:140%,y:60%,origin:top+left)['+'#'+body+']',
 'scale-size-auto':'#scale(x:80pt,y:auto,origin:top+left)['+'#'+body+']',
 'scale-reflow':'#scale(150%,reflow:true)['+'#'+body+']#rect(width:5pt,height:5pt,fill:blue,stroke:none)',
 'scale-both-auto':'#scale(x:auto,y:auto)['+'#'+body+']',
 'move-absolute':'#move(dx:5pt,dy:7pt)['+'#'+body+']',
 'move-relative':'#move(dx:10%,dy:20%)['+'#'+body+']',
 'move-em':'#set text(size:20pt)\n#move(dx:1em,dy:0.5em)['+'#'+body+']',
 'skew-default':'#skew(ax:12deg)['+'#'+body+']',
 'skew-origin':'#skew(ax:12deg,origin:top+left)['+'#'+body+']',
 'skew-reflow':'#skew(ax:12deg,reflow:true)['+'#'+body+']#rect(width:5pt,height:5pt,fill:blue,stroke:none)',
 'rotate-set':'#set rotate(origin:top+left,reflow:true)\n#rotate(23deg)['+'#'+body+']',
 'scale-set':'#set scale(x:150%)\n#scale['+'#'+body+']',
 'move-set':'#set move(dx:12pt)\n#move['+'#'+body+']',
 'skew-set':'#set skew(ax:12deg)\n#skew['+'#'+body+']',
 'show-rotate':'#show rotate: it => it.body\n#rotate(23deg)['+'#'+body+']',
 'inline-default':'A #rotate(23deg)[TEST] B\n\nNEXT',
 'inline-reflow':'A #rotate(23deg,reflow:true)[TEST] B\n\nNEXT',
 'nested':'#box(width:80pt,height:60pt)[#rotate(23deg,origin:top+left)[#scale(150%)['+'#'+body+']]]',
}
for name,content in cases.items():
 src=rdir/(name+'.typ');src.write_text('#set page(width:160pt,height:120pt,margin:0pt)\n#place(dx:20pt,dy:20pt)['+content+']')
 for key,exe in bins.items():
  dest=rdir/(name+'-'+key+'.svg');cmd=[str(exe),'compile',str(src),str(dest)];start=now();r=subprocess.run(cmd,capture_output=True,text=True,timeout=60)
  row=dict(case=name,kind='render',binary=key,command=cmd,start=start,end=now(),exit=r.returncode,stderr=r.stderr,source=src.read_text())
  if r.returncode==0:
   raster=dest.with_suffix('.png');cmd=['/home/linuxbrew/.linuxbrew/bin/rsvg-convert','--dpi-x','72','--dpi-y','72','--zoom','2','--background-color','white','--output',str(raster),str(dest)];q=subprocess.run(cmd,capture_output=True,text=True);row.update(svg_sha256=sha(dest),raster_command=cmd,raster_exit=q.returncode,raster_stderr=q.stderr)
  record['rows'].append(row)
 print('render',name,flush=True)
record['end']=now();assert all(sha(p)==h for p,h in files.items());assert all(sha(p)==record['binaries'][k]['sha256'] for k,p in bins.items())
with gzip.open(out/'results.json.gz','wt') as f:json.dump(record,f)
print('Source guards passed',flush=True)
