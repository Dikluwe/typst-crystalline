from pathlib import Path
import subprocess,hashlib,json,gzip,datetime,sys
from PIL import Image,ImageChops
root=Path.cwd();out=root/'00_nucleo/diagnosticos/transformacoes-2026-10-14'/sys.argv[1];out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
bins={'tekt':root/'target/debug/typst','vanilla':root/'lab/typst-original/target/release/typst'}
files={str(p):sha(p) for top in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'] for p in Path(top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']}
r=dict(start=now(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files=files,binaries={n:dict(path=str(p),sha256=sha(p)) for n,p in bins.items()},rows=[])
body='#rect(width:24pt,height:16pt,fill:red,stroke:none)'
cases={'shape': '#rect(width:20pt,height:10pt,fill:red,stroke:none)', 'two-shapes': '#rect(width:20pt,height:10pt,fill:red,stroke:none)\n#rect(width:20pt,height:10pt,fill:red,stroke:none)', 'shape-v': '#rect(width:20pt,height:10pt,fill:red,stroke:none)\n#v(7pt)', 'shape-weak-v': '#rect(width:20pt,height:10pt,fill:red,stroke:none)\n#v(7pt,weak:true)', 'v-shape': '#v(7pt)\n#rect(width:20pt,height:10pt,fill:red,stroke:none)', 'shape-negative-v': '#rect(width:20pt,height:10pt,fill:red,stroke:none)\n#v(-7pt)', 'shape-linebreak': '#rect(width:20pt,height:10pt,fill:red,stroke:none)#linebreak()', 'shape-parbreak': '#rect(width:20pt,height:10pt,fill:red,stroke:none)#parbreak()', 'shape-text': '#rect(width:20pt,height:10pt,fill:red,stroke:none)\nX', 'text-shape': 'X\n#rect(width:20pt,height:10pt,fill:red,stroke:none)', 'block-shape': '#block[#rect(width:20pt,height:10pt,fill:red,stroke:none)]', 'transform-shape': '#rotate(0deg)[#rect(width:20pt,height:10pt,fill:red,stroke:none)]', 'shape-transform': '#rect(width:20pt,height:10pt,fill:red,stroke:none)\n#rotate(0deg)[#rect(width:20pt,height:10pt,fill:red,stroke:none)]', 'transform-shape-tail': '#rotate(0deg)[#rect(width:20pt,height:10pt,fill:red,stroke:none)]\n#rect(width:20pt,height:10pt,fill:red,stroke:none)', 'shape-pagebreak': '#rect(width:20pt,height:10pt,fill:red,stroke:none)#pagebreak()\n#rect(width:20pt,height:10pt,fill:red,stroke:none)', 'ellipse': '#ellipse(width:20pt,height:10pt,fill:red,stroke:none)', 'shape-inset': '#block(inset:5pt)[#rect(width:20pt,height:10pt,fill:red,stroke:none)]', 'shape-label': '#rect(width:20pt,height:10pt,fill:red,stroke:none) <a>', 'shape-explicit-page': '#set page(height:200pt)\n#rect(width:20pt,height:10pt,fill:red,stroke:none)', 'between-shapes-7pt': '#rect(width:20pt,height:10pt,fill:red,stroke:none)#v(7pt)#rect(width:20pt,height:10pt,fill:red,stroke:none)', 'between-text-7pt': '#rect(width:20pt,height:10pt,fill:red,stroke:none)#v(7pt)X', 'transform-v-7pt': '#rotate(0deg)[#rect(width:20pt,height:10pt,fill:red,stroke:none)]#v(7pt)', 'between-shapes-30pt': '#rect(width:20pt,height:10pt,fill:red,stroke:none)#v(30pt)#rect(width:20pt,height:10pt,fill:red,stroke:none)', 'between-text-30pt': '#rect(width:20pt,height:10pt,fill:red,stroke:none)#v(30pt)X', 'transform-v-30pt': '#rotate(0deg)[#rect(width:20pt,height:10pt,fill:red,stroke:none)]#v(30pt)', 'between-shapes-7pt-weak': '#rect(width:20pt,height:10pt,fill:red,stroke:none)#v(7pt,weak:true)#rect(width:20pt,height:10pt,fill:red,stroke:none)', 'between-text-7pt-weak': '#rect(width:20pt,height:10pt,fill:red,stroke:none)#v(7pt,weak:true)X', 'transform-v-7pt-weak': '#rotate(0deg)[#rect(width:20pt,height:10pt,fill:red,stroke:none)]#v(7pt,weak:true)', 'between-shapes-30pt-weak': '#rect(width:20pt,height:10pt,fill:red,stroke:none)#v(30pt,weak:true)#rect(width:20pt,height:10pt,fill:red,stroke:none)', 'between-text-30pt-weak': '#rect(width:20pt,height:10pt,fill:red,stroke:none)#v(30pt,weak:true)X', 'transform-v-30pt-weak': '#rotate(0deg)[#rect(width:20pt,height:10pt,fill:red,stroke:none)]#v(30pt,weak:true)', 'break-exact': '#set page(height:30pt)\n#rect(width:20pt,height:10pt,fill:red,stroke:none)\n#rect(width:20pt,height:10pt,fill:red,stroke:none)'}

for name,body in cases.items():
 src=out/(name+'.typ');src.write_text('#set page(width:100pt,height:auto,margin:10pt)\n'+body)
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
   a,b=pair['tekt'],pair['vanilla'];
   if a.size!=b.size:
    r.setdefault("size_differences",[]).append(dict(case=name,format=fmt,tekt=a.size,vanilla=b.size));print("SIZE",name,fmt,a.size,b.size,flush=True);continue
   ds=[max(v) for v in ImageChops.difference(a,b).getdata()];metric=dict(case=name,format=fmt,max_channel=max(ds),different_pixels=sum(v>0 for v in ds))
   r.setdefault('metrics',[]).append(metric);print(metric,flush=True)
  else:print('ERROR',name,fmt,[(x['binary'],x['stderr'].splitlines()[:1]) for x in r['rows'][-2:]],flush=True)
r['end']=now();r['source_guard']=all(sha(p)==h for p,h in files.items());assert r['source_guard'];assert all(sha(p)==r['binaries'][key]['sha256'] for key,p in bins.items())
with gzip.open(out/'results.json.gz','wt') as f:json.dump(r,f)
