from pathlib import Path
import subprocess,hashlib,json,gzip,datetime,sys
from PIL import Image,ImageChops
root=Path.cwd();out=root/'00_nucleo/diagnosticos/transformacoes-2026-10-14'/sys.argv[1];out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
bins={'tekt':root/'target/debug/typst','vanilla':root/'lab/typst-original/target/release/typst'}
files={str(p):sha(p) for top in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'] for p in Path(top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']}
r=dict(start=now(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files=files,binaries={n:dict(path=str(p),sha256=sha(p)) for n,p in bins.items()},rows=[])
cases={'shape-auto': '#block()[#rect(width:20pt,height:10pt,fill:red,stroke:none)]', 'shape-inset-5pt': '#block(inset:5pt)[#rect(width:20pt,height:10pt,fill:red,stroke:none)]', 'shape-height-40pt': '#block(height:40pt)[#rect(width:20pt,height:10pt,fill:red,stroke:none)]', 'shape-height-40pt-inset-5pt': '#block(height:40pt,inset:5pt)[#rect(width:20pt,height:10pt,fill:red,stroke:none)]', 'shape-fill-blue-inset-5pt': '#block(fill:blue,inset:5pt)[#rect(width:20pt,height:10pt,fill:red,stroke:none)]', 'text-auto': '#block()[X]', 'text-inset-5pt': '#block(inset:5pt)[X]', 'text-height-40pt': '#block(height:40pt)[X]', 'text-height-40pt-inset-5pt': '#block(height:40pt,inset:5pt)[X]', 'text-fill-blue-inset-5pt': '#block(fill:blue,inset:5pt)[X]', 'empty-auto': '#block()[]', 'empty-inset-5pt': '#block(inset:5pt)[]', 'empty-height-40pt': '#block(height:40pt)[]', 'empty-height-40pt-inset-5pt': '#block(height:40pt,inset:5pt)[]', 'empty-fill-blue-inset-5pt': '#block(fill:blue,inset:5pt)[]', 'transform-auto': '#block()[#rotate(0deg)[#rect(width:20pt,height:10pt,fill:red,stroke:none)]]', 'transform-inset-5pt': '#block(inset:5pt)[#rotate(0deg)[#rect(width:20pt,height:10pt,fill:red,stroke:none)]]', 'transform-height-40pt': '#block(height:40pt)[#rotate(0deg)[#rect(width:20pt,height:10pt,fill:red,stroke:none)]]', 'transform-height-40pt-inset-5pt': '#block(height:40pt,inset:5pt)[#rotate(0deg)[#rect(width:20pt,height:10pt,fill:red,stroke:none)]]', 'transform-fill-blue-inset-5pt': '#block(fill:blue,inset:5pt)[#rotate(0deg)[#rect(width:20pt,height:10pt,fill:red,stroke:none)]]'}

for name,body in cases.items():
 src=out/(name+'.typ');src.write_text('#set page(width:100pt,height:auto,margin:10pt)\n'+body)
 for fmt in ['svg','png']:
  pair={}
  for key,exe in bins.items():
   dest=out/(name+'-'+key+'.'+fmt);cmd=[str(exe),'compile',str(src),str(dest)]
   if fmt=='png':cmd+=['--ppi','144']
   t=now();q=subprocess.run(cmd,capture_output=True,text=True,timeout=45);row=dict(case=name,format=fmt,binary=key,command=cmd,start=t,end=now(),source=src.read_text(),exit=q.returncode,stdout=q.stdout,stderr=q.stderr)
   if q.returncode==0:
    row['output_sha256']=sha(dest)
    if fmt=='svg':
     raster=out/(name+'-'+key+'-svg.png');rc=['/home/linuxbrew/.linuxbrew/bin/rsvg-convert','--dpi-x','72','--dpi-y','72','--zoom','2','--background-color','white','--output',str(raster),str(dest)];rr=subprocess.run(rc,capture_output=True,text=True);row['raster_command']=rc;row['raster_exit']=rr.returncode;row['raster_stderr']=rr.stderr
     if rr.returncode:
      r['rows'].append(row);print('RASTER ERROR',name,key,rr.stderr.strip(),flush=True);continue
     row['raster_sha256']=sha(raster)
    else:raster=dest
    pair[key]=Image.open(raster).convert('RGB')
   r['rows'].append(row)
  if len(pair)==2:
   a,b=pair['tekt'],pair['vanilla'];
   if a.size!=b.size:
    r.setdefault("size_differences",[]).append(dict(case=name,format=fmt,tekt=a.size,vanilla=b.size));print("SIZE",name,fmt,a.size,b.size,flush=True);continue
   ds=[max(v) for v in ImageChops.difference(a,b).getdata()];metric=dict(case=name,format=fmt,max_channel=max(ds),different_pixels=sum(v>0 for v in ds))
   r.setdefault('metrics',[]).append(metric);print(metric,flush=True)
  else:print('ERROR',name,fmt,[(x['binary'],x['stderr'].splitlines()[:1]) for x in r['rows'][-2:]],flush=True)
r['end']=now();r['source_guard']=all(sha(p)==h for p,h in files.items());assert r['source_guard'];assert all(sha(p)==r['binaries'][key]['sha256'] for key,p in bins.items())
with gzip.open(out/'results.json.gz','wt') as f:json.dump(r,f)
