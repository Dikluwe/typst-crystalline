from pathlib import Path
import subprocess,hashlib,json,gzip,datetime,sys
from PIL import Image,ImageChops
root=Path.cwd();out=root/'00_nucleo/diagnosticos/transformacoes-2026-10-14'/sys.argv[1];out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
bins={'tekt':root/'target/debug/typst','vanilla':root/'lab/typst-original/target/release/typst'}
files={str(p):sha(p) for top in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'] for p in Path(top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']}
r=dict(start=now(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files=files,binaries={n:dict(path=str(p),sha256=sha(p)) for n,p in bins.items()},rows=[])
body='#rect(width:24pt,height:16pt,fill:red,stroke:none)'
cases={'ltr-start': '#set text(dir:ltr)\n#rotate(23deg,origin:start+top)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]', 'ltr-end': '#set text(dir:ltr)\n#rotate(23deg,origin:end+top)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]', 'ltr-left': '#set text(dir:ltr)\n#rotate(23deg,origin:left+top)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]', 'ltr-right': '#set text(dir:ltr)\n#rotate(23deg,origin:right+top)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]', 'ltr-align-left': '#set text(dir:ltr)\n#align(left)[#rotate(23deg,origin:left+top)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]]', 'ltr-align-center': '#set text(dir:ltr)\n#align(center)[#rotate(23deg,origin:left+top)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]]', 'ltr-align-right': '#set text(dir:ltr)\n#align(right)[#rotate(23deg,origin:left+top)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]]', 'ltr-align-start': '#set text(dir:ltr)\n#align(start)[#rotate(23deg,origin:left+top)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]]', 'ltr-align-end': '#set text(dir:ltr)\n#align(end)[#rotate(23deg,origin:left+top)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]]', 'ltr-text': '#set text(dir:ltr)\n#rotate(0deg)[ONE TWO]', 'ltr-reflow': '#set text(dir:ltr)\n#rotate(23deg,reflow:true)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]', 'ltr-block': '#set text(dir:ltr)\n#block(width:80pt,inset:5pt)[#rotate(0deg)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]]', 'ltr-nested': '#set text(dir:ltr)\n#rotate(0deg)[#rotate(0deg)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]]', 'ltr-siblings': '#set text(dir:ltr)\n#align(center)[#rotate(0deg)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]#rotate(0deg)[#rect(width:50pt,height:16pt,fill:blue,stroke:none)]]', 'ltr-auto': '#set page(width:auto)\n#set text(dir:ltr)\n#rotate(0deg)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]#rect(width:50pt,height:16pt,fill:blue,stroke:none)', 'rtl-start': '#set text(dir:rtl)\n#rotate(23deg,origin:start+top)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]', 'rtl-end': '#set text(dir:rtl)\n#rotate(23deg,origin:end+top)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]', 'rtl-left': '#set text(dir:rtl)\n#rotate(23deg,origin:left+top)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]', 'rtl-right': '#set text(dir:rtl)\n#rotate(23deg,origin:right+top)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]', 'rtl-align-left': '#set text(dir:rtl)\n#align(left)[#rotate(23deg,origin:left+top)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]]', 'rtl-align-center': '#set text(dir:rtl)\n#align(center)[#rotate(23deg,origin:left+top)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]]', 'rtl-align-right': '#set text(dir:rtl)\n#align(right)[#rotate(23deg,origin:left+top)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]]', 'rtl-align-start': '#set text(dir:rtl)\n#align(start)[#rotate(23deg,origin:left+top)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]]', 'rtl-align-end': '#set text(dir:rtl)\n#align(end)[#rotate(23deg,origin:left+top)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]]', 'rtl-text': '#set text(dir:rtl)\n#rotate(0deg)[ONE TWO]', 'rtl-reflow': '#set text(dir:rtl)\n#rotate(23deg,reflow:true)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]', 'rtl-block': '#set text(dir:rtl)\n#block(width:80pt,inset:5pt)[#rotate(0deg)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]]', 'rtl-nested': '#set text(dir:rtl)\n#rotate(0deg)[#rotate(0deg)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]]', 'rtl-siblings': '#set text(dir:rtl)\n#align(center)[#rotate(0deg)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]#rotate(0deg)[#rect(width:50pt,height:16pt,fill:blue,stroke:none)]]', 'rtl-auto': '#set page(width:auto)\n#set text(dir:rtl)\n#rotate(0deg)[#rect(width:24pt,height:16pt,fill:red,stroke:none)]#rect(width:50pt,height:16pt,fill:blue,stroke:none)'}

for name,body in cases.items():
 src=out/(name+'.typ');src.write_text('#set page(width:300pt,height:300pt,margin:70pt)\n'+body)
 for fmt in ['svg','png']:
  pair={}
  for key,exe in bins.items():
   dest=out/(name+'-'+key+'.'+fmt);cmd=[str(exe),'compile',str(src),str(dest)]
   if fmt=='png':cmd+=['--ppi','144']
   t=now();q=subprocess.run(cmd,capture_output=True,text=True,timeout=45);row=dict(case=name,format=fmt,binary=key,command=cmd,start=t,end=now(),source=src.read_text(),exit=q.returncode,stdout=q.stdout,stderr=q.stderr)
   if q.returncode==0:
    row['output_sha256']=sha(dest)
    if fmt=='svg':
     raster=out/(name+'-'+key+'-svg.png');rc=['/home/linuxbrew/.linuxbrew/bin/rsvg-convert','--dpi-x','72','--dpi-y','72','--zoom','2','--background-color','white','--output',str(raster),str(dest)];subprocess.run(rc,check=True,capture_output=True);row['raster_sha256']=sha(raster)
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
