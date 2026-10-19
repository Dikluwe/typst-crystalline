from pathlib import Path
import json,gzip,subprocess,hashlib,datetime
from PIL import Image,ImageChops
root=Path.cwd();out=root/'00_nucleo/diagnosticos/block-auto-alignment-2026-10-14';dest=out/'pdf-after';dest.mkdir(exist_ok=False)
with gzip.open(out/'after/results.json.gz','rt') as f:pre=json.load(f)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
assert all(sha(n)==v['sha256'] for n,v in pre['files'].items())
r=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),provenance=pre,rows=[])
for case in ['right-clip','center-clip','wider-right-clip','nested-clip','fixed-clip','link-clip','underline-outer-clip']:
 for name,bin in pre['binaries'].items():
  assert sha(bin['path'])==bin['sha256'];source=out/'after'/(case+'.typ');pdf=dest/(case+'-'+name+'.pdf');cmd=[bin['path'],'compile',str(source),str(pdf)];p=subprocess.run(cmd,capture_output=True,text=True);assert p.returncode==0,p.stderr
  q=subprocess.run(['qpdf','--check',str(pdf)],capture_output=True,text=True);assert q.returncode==0,q.stderr
  subprocess.run(['pdftoppm','-r','144','-singlefile','-png',str(pdf),str(pdf.with_suffix(''))],check=True,capture_output=True)
  r['rows'].append(dict(case=case,binary=name,command=cmd,exit=p.returncode,qpdf=q.stdout,sha256=sha(pdf)))
 a=Image.open(dest/(case+'-tekt.png')).convert('RGB');b=Image.open(dest/(case+'-vanilla.png')).convert('RGB');d=list(ImageChops.difference(a,b).getdata());r['rows'][-1]['pixel_comparison']=dict(max=max(max(x) for x in d),pixels_gt16=sum(max(x)>16 for x in d));print(case,r['rows'][-1]['pixel_comparison'],flush=True)
r['end']=datetime.datetime.now(datetime.timezone.utc).isoformat();assert all(sha(n)==v['sha256'] for n,v in pre['files'].items())
with gzip.open(dest/'results.json.gz','wt') as f:json.dump(r,f)
