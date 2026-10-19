from pathlib import Path
import subprocess,gzip,json,hashlib,datetime,re,sys
from PIL import Image
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tiling-cobertura-2026-10-14'/sys.argv[1];out.mkdir(parents=True,exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
files=['01_core/src/compiler/layout/tiling.rs','01_core/src/compiler/layout/shape.rs','03_infra/src/export/stream/shape.rs','01_core/src/entities/tiling.rs','01_core/src/entities/layout_types.rs']
files +=[re.search(r'^//! @prompt (.+)$',Path(p).read_text(),re.M).group(1) for p in list(files)]
files += ['lab/typst-original/crates/typst-library/src/visualize/tiling.rs']
bins={'tekt':root/'target/debug/typst','vanilla':root/'lab/typst-original/target/release/typst'};assert sha(bins['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
r=dict(start=now(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:dict(text=Path(p).read_text(),sha256=sha(p)) for p in files},binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in bins.items()},cases=[])
for name,dx,dy,w,h,params,body in [
 ('auto-shape',12,12,40,24,'size:auto','#rect(width:4pt,height:4pt,fill:red,stroke:none)'),
 ('empty',12,12,40,24,'size:(8pt,8pt)',''),
 ('empty-dense',12,12,100,100,'size:(1pt,1pt)',''),
 ('dense',12,12,100,100,'size:(2pt,2pt)','#rect(width:1pt,height:1pt,fill:red,stroke:none)'),
 ('self',180,60,40,24,'size:(8pt,8pt)','#rect(width:4pt,height:4pt,fill:red,stroke:none)'),
 ('parent',180,60,40,24,'size:(8pt,8pt),relative:"parent"','#rect(width:4pt,height:4pt,fill:red,stroke:none)'),
 ('self-rotated',180,60,40,24,'size:(8pt,8pt),angle:35deg','#rect(width:4pt,height:4pt,fill:red,stroke:none)'),
 ('parent-rotated',180,60,40,24,'size:(8pt,8pt),angle:35deg,relative:"parent"','#rect(width:4pt,height:4pt,fill:red,stroke:none)'),
 ('large-offset',12,12,40,24,'size:(8pt,8pt),offset:(1000000000000pt,1000000000000pt)','#rect(width:4pt,height:4pt,fill:red,stroke:none)'),
 ('offset-control',12,12,40,24,'size:(8pt,8pt),offset:(0pt,0pt)','#rect(width:4pt,height:4pt,fill:red,stroke:none)'),
 ('spacing',12,12,40,24,'size:(8pt,8pt),spacing:(2pt,3pt)','#rect(width:4pt,height:4pt,fill:red,stroke:none)'),
]:
 for label,binary in bins.items():
  source=f'#set page(width:240pt,height:140pt,margin:0pt)\n#let t=tiling({params})[{body}]\n#place(dx:{dx}pt,dy:{dy}pt,rect(width:{w}pt,height:{h}pt,fill:t,stroke:none))\n';p=out/f'{name}-{label}.typ';p.write_text(source);cmd=[str(binary),'compile',str(p),str(p.with_suffix('.pdf'))];x=subprocess.run(cmd,capture_output=True,text=True);row=dict(name=name,binary=label,source=source,command=cmd,exit=x.returncode,stdout=x.stdout,stderr=x.stderr)
  if x.returncode==0:
   subprocess.run(['pdftoppm','-r','144','-singlefile','-png',str(p.with_suffix('.pdf')),str(p.with_suffix(''))],check=True,capture_output=True);im=Image.open(p.with_suffix('.png')).convert('RGB');row.update(pixels_sha256=hashlib.sha256(im.tobytes()).hexdigest(),painted=sum(max(255-v for v in px)>32 for px in im.getdata()))
  r['cases'].append(row);print(name,label,x.returncode,row.get('painted'),x.stderr.splitlines()[:1],flush=True)
r['end']=now();assert all(sha(bins[k])==v['sha256'] for k,v in r['binaries'].items());assert all(sha(p)==v['sha256'] for p,v in r['files'].items())
with gzip.open(out/'results.json.gz','wt') as f:json.dump(r,f)
