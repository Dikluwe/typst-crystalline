from pathlib import Path
import json,gzip,hashlib,datetime,subprocess
from PIL import Image
root=Path.cwd();out=root/'00_nucleo/diagnosticos/block-regiao-2026-10-14/open-pdf';out.mkdir(exist_ok=False);sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(out.parent/'candidate-final.json.gz','rt') as f:pre=json.load(f)
assert all(sha(n)==v['sha256'] for n,v in pre['files'].items())
r=dict(start=now(),head=pre['head'],files={n:v['sha256'] for n,v in pre['files'].items()},rows=[])
for name in ['root-body-overflow','box-body-overflow','root-outset','box-outset','root-after-outset','box-after-outset']:
 source=(out.parent/'extra-final'/(name+'.typ')).read_text();f=out/(name+'.typ');f.write_text(source)
 for label,exe in [('tekt',root/'target/debug/typst'),('vanilla',root/'lab/typst-original/target/release/typst')]:
  pdf=out/(name+'-'+label+'.pdf');row=dict(case=name,binary=label,binary_sha256=sha(exe),source=source,commands=[],pages=[])
  for cmd in [[str(exe),'compile',str(f),str(pdf)],['qpdf','--check',str(pdf)],['pdfinfo',str(pdf)],['pdftoppm','-r','144','-png',str(pdf),str(pdf.with_suffix(''))]]:
   v=subprocess.run(cmd,capture_output=True,text=True);row['commands'].append(dict(command=cmd,exit=v.returncode,stdout=v.stdout,stderr=v.stderr));assert v.returncode==0,(name,label,v.stderr)
  for png in sorted(out.glob(name+'-'+label+'-*.png')):
   im=Image.open(png).convert('RGB');coords=[(x,y) for y in range(im.height) for x in range(im.width) if (lambda c:c[2]>c[0]+70 and c[2]>c[1]+30)(im.getpixel((x,y)))];row['pages'].append(dict(path=str(png),sha256=sha(png),size=im.size,blue_bbox=None if not coords else [min(x for x,y in coords)/2,min(y for x,y in coords)/2,(max(x for x,y in coords)+1)/2,(max(y for x,y in coords)+1)/2]))
  row['pdf_sha256']=sha(pdf);r['rows'].append(row);print(name,label,[x['blue_bbox'] for x in row['pages']],flush=True)
r['end']=now();assert all(sha(n)==v for n,v in r['files'].items())
with gzip.open(out/'results.json.gz','wt') as f:json.dump(r,f)
