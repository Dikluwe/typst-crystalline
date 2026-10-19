from pathlib import Path
import subprocess,json,gzip,datetime,hashlib,re
from PIL import Image
root=Path.cwd();out=root/'00_nucleo/diagnosticos/place-origem-local-2026-10-14/pdf-final';out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(out.parent/'candidate-final.json.gz','rt') as f:pre=json.load(f)
assert all(sha(n)==v['sha256'] for n,v in pre['files'].items())
r=dict(start=now(),head=pre['head'],files={n:v['sha256'] for n,v in pre['files'].items()},rows=[])
cases={name:(out.parent/'after'/f'{name}.typ').read_text() for name in ['root','box','box-inset','box-place','box-nested','box-text','root-text','cell','grid','box-bottom','box-horizon']}
cases['explicit-top']=(out.parent/'extra-after'/'root-text-top.typ').read_text()
cases['align-place']='#set page(width:120pt,height:100pt,margin:10pt)\n#align(left)[#place(dx:7pt,dy:5pt,rect(width:12pt,height:8pt,fill:blue,stroke:none))]'
for name,source in cases.items():
 f=out/(name+'.typ');f.write_text(source)
 for label,exe in [('tekt',root/'target/debug/typst'),('vanilla',root/'lab/typst-original/target/release/typst')]:
  pdf=out/(name+'-'+label+'.pdf');row=dict(case=name,binary=label,binary_sha256=sha(exe),source=source,commands=[])
  for cmd in [[str(exe),'compile',str(f),str(pdf)],['qpdf','--check',str(pdf)],['pdftoppm','-r','288','-singlefile','-png',str(pdf),str(pdf.with_suffix(''))]]:
   v=subprocess.run(cmd,capture_output=True,text=True);row['commands'].append(dict(command=cmd,exit=v.returncode,stdout=v.stdout,stderr=v.stderr));assert v.returncode==0,(name,label,cmd,v.stderr)
  im=Image.open(pdf.with_suffix('.png')).convert('RGB');coords=[(x,y) for y in range(im.height) for x in range(im.width) if (lambda p:p[2]>p[0]+70 and p[2]>p[1]+30)(im.getpixel((x,y)))];row['blue_bbox']=None if not coords else [min(x for x,y in coords)/4,min(y for x,y in coords)/4,(max(x for x,y in coords)+1)/4,(max(y for x,y in coords)+1)/4];row['size']=im.size;row['pixels_sha256']=hashlib.sha256(im.tobytes()).hexdigest();row['pdf_sha256']=sha(pdf);r['rows'].append(row);print(name,label,row['blue_bbox'],flush=True)
r['end']=now();assert all(sha(n)==v for n,v in r['files'].items())
with gzip.open(out/'results.json.gz','wt') as f:json.dump(r,f)
