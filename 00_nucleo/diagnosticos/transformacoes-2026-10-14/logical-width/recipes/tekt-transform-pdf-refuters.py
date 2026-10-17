from pathlib import Path
import subprocess,hashlib,json,gzip,datetime,sys
from PIL import Image,ImageChops
root=Path.cwd();out=root/'00_nucleo/diagnosticos/transformacoes-2026-10-14'/sys.argv[1];out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
bins={'tekt':root/'target/debug/typst','vanilla':root/'lab/typst-original/target/release/typst'}
files={str(p):sha(p) for top in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'] for p in Path(top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']}
r=dict(start=now(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files=files,binaries={n:dict(path=str(p),sha256=sha(p)) for n,p in bins.items()},rows=[])
body='#rect(width:24pt,height:16pt,fill:red,stroke:none)'
cases={
 'box-text-rotate':'A #box(rotate(23deg)[TEXT]) B',
 'box-text-reflow':'A #box(rotate(23deg,reflow:true)[TEXT]) B',
 'box-text-scale':'A #box(scale(x:150%,y:80%)[TEXT]) B',
 'box-text-skew':'A #box(skew(ay:20deg,reflow:true)[TEXT]) B',
 'box-shape':'A #box(rotate(23deg)['+body+']) B',
 'shape-skew-y':'#skew(ay:20deg,reflow:true)['+body+']#rect(width:5pt,height:5pt,fill:blue,stroke:none)',
 'negative-x':'#scale(x:-100%,y:100%,origin:top+left)['+body+']',
 'negative-y':'#scale(x:100%,y:-100%,reflow:true)['+body+']',
 'zero':'#scale(0%,reflow:true)['+body+']AFTER',
 'empty':'#rotate(23deg,reflow:true)[]AFTER',
 'empty-size':'#scale(x:20pt,y:auto)[]AFTER',
 'spaces':'#rotate(23deg,origin:top+left)[#h(20pt)]AFTER',
 'relative-box':'#box(width:80pt,height:60pt)[#move(dx:10%,dy:20%)['+body+']]',
 'relative-block':'#block(width:80pt,height:60pt,inset:5pt)[#move(dx:10%,dy:20%)['+body+']]',
 'relative-nested':'#box(width:80pt,height:60pt)[#box(width:40pt,height:30pt)[#move(dx:10%,dy:20%)['+body+']]]',
 'relative-sibling':'#box(width:80pt,height:60pt)[#move(dx:10%,dy:20%)['+body+']]#move(dx:10%,dy:20%)['+body+']',
 'partial-origin':'#set rotate(origin:top+left)\n#rotate(23deg,origin:right)['+body+']',
 'style-fold':'#set rotate(origin:top+left)\n#set rotate(origin:right)\n#rotate(23deg)['+body+']',
 'rtl-start':'#set text(dir:rtl)\n#rotate(23deg,origin:start+top)['+body+']',
 'rtl-end':'#set text(dir:rtl)\n#rotate(23deg,origin:end+top)['+body+']',
 'reflow-wrap':'#block(width:50pt)[#scale(200%,reflow:true)[ONE TWO THREE FOUR]]',
 'reflow-wrap-rotate':'#block(width:50pt)[#rotate(23deg,reflow:true)[ONE TWO THREE FOUR]]',
 'block-spacing':'#set block(spacing:3pt)\nBEFORE #rotate(23deg)['+body+'] AFTER',
 'math':'#rotate(23deg,reflow:true)[$frac(a,b)$]AFTER',
 'clip':'#box(width:32pt,height:22pt,clip:true)[#rotate(23deg)['+body+']]',
 'parent-gradient':'#box(width:80pt,height:60pt)[#rotate(23deg)[#rect(width:24pt,height:16pt,fill:gradient.linear(red,blue,relative:"parent"),stroke:none)]]',
 'show-where':'#show rotate.where(angle:23deg): it => [MATCH]\n#rotate(23deg)[X]#rotate(0deg)[Y]',
 'set-alias':'#let r=rotate\n#set r(23deg,origin:top+left)\n#r['+body+']',
 'with-call':'#let r=rotate.with(23deg,origin:top+left)\n#r['+body+']',
 'set-lexical':'#let a=rotate['+body+']\n#set rotate(23deg)\n#a',
}
for name,body in cases.items():
 src=out/(name+'.typ');src.write_text('#set page(width:160pt,height:180pt,margin:10pt)\n'+body)
 for fmt in ['pdf']:
  pair={}
  for key,exe in bins.items():
   dest=out/(name+'-'+key+'.'+fmt);cmd=[str(exe),'compile',str(src),str(dest)]
   if fmt=='png':cmd+=['--ppi','144']
   t=now();q=subprocess.run(cmd,capture_output=True,text=True,timeout=45);row=dict(case=name,format=fmt,binary=key,command=cmd,start=t,end=now(),source=src.read_text(),exit=q.returncode,stdout=q.stdout,stderr=q.stderr)
   if q.returncode==0:
    row['output_sha256']=sha(dest)
    if fmt=='svg':
     raster=out/(name+'-'+key+'-svg.png');rc=['/home/linuxbrew/.linuxbrew/bin/rsvg-convert','--dpi-x','72','--dpi-y','72','--zoom','2','--background-color','white','--output',str(raster),str(dest)];subprocess.run(rc,check=True,capture_output=True);row['raster_sha256']=sha(raster)
    elif fmt=='pdf':
     raster=out/(name+'-'+key+'-pdf.png');rc=['pdftocairo','-png','-singlefile','-r','144',str(dest),str(raster.with_suffix(''))];subprocess.run(rc,check=True,capture_output=True);row['raster_sha256']=sha(raster);row['raster_command']=rc
    else:raster=dest
    pair[key]=Image.open(raster).convert('RGB')
   r['rows'].append(row)
  if len(pair)==2:
   a,b=pair['tekt'],pair['vanilla'];assert a.size==b.size
   ds=[max(v) for v in ImageChops.difference(a,b).getdata()];metric=dict(case=name,format=fmt,max_channel=max(ds),different_pixels=sum(v>0 for v in ds))
   r.setdefault('metrics',[]).append(metric);print(metric,flush=True)
  else:print('ERROR',name,fmt,[(x['binary'],x['stderr'].splitlines()[:1]) for x in r['rows'][-2:]],flush=True)
r['end']=now();r['source_guard']=all(sha(p)==h for p,h in files.items());assert r['source_guard'];assert all(sha(p)==r['binaries'][key]['sha256'] for key,p in bins.items())
with gzip.open(out/'results.json.gz','wt') as f:json.dump(r,f)
