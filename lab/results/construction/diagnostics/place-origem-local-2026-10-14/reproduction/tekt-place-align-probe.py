from pathlib import Path
import subprocess,json,gzip,datetime,hashlib,re,sys
from PIL import Image
root=Path.cwd();out=root/'00_nucleo/diagnosticos/place-origem-local-2026-10-14'/sys.argv[1];out.mkdir(parents=True,exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
owners=json.loads((out.parent/'owners.json').read_text());paths=list(owners)+list(owners.values());r=dict(start=now(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),files={p:dict(text=Path(p).read_text(),sha256=sha(p)) for p in paths},rows=[])
text='text(fill:blue)[MARKER]';place='place(bottom,dy:30pt)['+'#'+text+']';rect='rect(width:12pt,height:8pt,fill:blue,stroke:none)'
cases={}
for key,body in [('root','#'+place),('align','#align(left)[#'+place+']'),('align-double','#align(left)[#align(left)[#'+place+']]'),('rotate','#rotate(0deg)[#'+place+']')]:cases['auto-'+key]='#set page(width:120pt,height:auto,margin:20pt)\n#block(height:200pt)[TALL]\n'+body
for key,body in [('text','#'+text),('shape','#'+rect),('box','#box(width:40pt,height:30pt,fill:blue)[]'),('mixed','A#'+rect)]:
 for align in ['left','center','right','top','bottom','horizon']:
  cases[key+'-'+align]='#set page(width:120pt,height:100pt,margin:10pt)\n#align('+align+')['+body+']'
for name,source in cases.items():
 f=out/(name+'.typ');f.write_text(source)
 for label,exe in [('tekt',root/'target/debug/typst'),('vanilla',root/'lab/typst-original/target/release/typst')]:
  svg=out/(name+'-'+label+'.svg');cmd=[str(exe),'compile',str(f),str(svg)];v=subprocess.run(cmd,capture_output=True,text=True);row=dict(case=name,binary=label,binary_sha256=sha(exe),command=cmd,source=source,exit=v.returncode,stdout=v.stdout,stderr=v.stderr)
  if not v.returncode:
   png=svg.with_suffix('.png');subprocess.run(['/home/linuxbrew/.linuxbrew/bin/rsvg-convert','--zoom','4','--background-color','white','--output',str(png),str(svg)],check=True,capture_output=True);im=Image.open(png).convert('RGB');coords=[(x,y) for y in range(im.height) for x in range(im.width) if (lambda p:p[2]>p[0]+70 and p[2]>p[1]+30)(im.getpixel((x,y)))];row['blue_bbox']=None if not coords else [min(x for x,y in coords)/4,min(y for x,y in coords)/4,(max(x for x,y in coords)+1)/4,(max(y for x,y in coords)+1)/4];row['size']=im.size;row['pixels_sha256']=hashlib.sha256(im.tobytes()).hexdigest();row['svg_sha256']=sha(svg)
  r['rows'].append(row);print(name,label,row.get('blue_bbox'),row.get('size'),v.returncode,flush=True)
r['end']=now();assert all(sha(p)==v['sha256'] for p,v in r['files'].items())
with gzip.open(out/'results.json.gz','wt') as f:json.dump(r,f)
