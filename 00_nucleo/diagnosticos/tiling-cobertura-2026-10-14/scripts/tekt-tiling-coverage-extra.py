from pathlib import Path
import subprocess,gzip,json,hashlib,datetime
from PIL import Image
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tiling-cobertura-2026-10-14/extra';out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
bins={'tekt':root/'target/debug/typst','vanilla':root/'lab/typst-original/target/release/typst'}
r=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in bins.items()},source_snapshot='../candidate-final.json.gz',cases=[])
for name,body in {
 'over-limit':'rect(width:200pt,height:100pt,fill:tiling(size:(1pt,1pt))[#rect(width:0.5pt,height:0.5pt,fill:red,stroke:none)],stroke:none)',
 'ellipse':'ellipse(width:40pt,height:24pt,fill:tiling(size:(8pt,8pt))[#rect(width:4pt,height:4pt,fill:red,stroke:none)],stroke:none)',
 'stroke':'rect(width:40pt,height:24pt,stroke:stroke(paint:tiling(size:(8pt,8pt))[#rect(width:4pt,height:4pt,fill:red,stroke:none)],thickness:8pt))',
 'text-auto':'rect(width:40pt,height:24pt,fill:tiling()[#text(fill:red)[A]],stroke:none)',
}.items():
 for label,binary in bins.items():
  source='#set page(width:240pt,height:140pt,margin:0pt)\n#place(dx:12pt,dy:12pt,'+body+')\n';p=out/f'{name}-{label}.typ';p.write_text(source);cmd=[str(binary),'compile',str(p),str(p.with_suffix('.pdf'))];x=subprocess.run(cmd,capture_output=True,text=True);row=dict(name=name,binary=label,source=source,command=cmd,exit=x.returncode,stdout=x.stdout,stderr=x.stderr)
  if x.returncode==0:
   subprocess.run(['pdftoppm','-r','144','-singlefile','-png',str(p.with_suffix('.pdf')),str(p.with_suffix(''))],check=True,capture_output=True);im=Image.open(p.with_suffix('.png')).convert('RGB');row.update(pixels_sha256=hashlib.sha256(im.tobytes()).hexdigest(),painted=sum(max(255-v for v in px)>32 for px in im.getdata()))
  r['cases'].append(row);print(name,label,x.returncode,row.get('painted'),flush=True)
r['end']=datetime.datetime.now(datetime.timezone.utc).isoformat();assert all(sha(bins[k])==v['sha256'] for k,v in r['binaries'].items())
with gzip.open(out/'results.json.gz','wt') as f:json.dump(r,f)
