from pathlib import Path
import json,gzip,hashlib,datetime,subprocess,sys
from PIL import Image
root=Path.cwd();out=root/'00_nucleo/diagnosticos/pdf-caminho-vazio-2026-10-14'/sys.argv[1];out.mkdir(parents=True,exist_ok=False);sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
paths=['03_infra/src/export/stream/draw.rs','00_nucleo/prompts/infra/export/stream/draw.md'];record=dict(start=now(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={n:dict(text=Path(n).read_text(),sha256=sha(n)) for n in paths},rows=[])
empty=lambda shape:'#place(dx:10pt,dy:10pt,'+shape+')'
blank=empty('rect(width:25pt,height:18pt,fill:none,stroke:none)')
blue='#place(dx:60pt,dy:50pt,rect(width:12pt,height:8pt,fill:blue,stroke:none))'
cases={
'rect':blank+blue,
'ellipse':empty('ellipse(width:25pt,height:18pt,fill:none,stroke:none)')+blue,
'rounded':empty('rect(width:25pt,height:18pt,radius:4pt,fill:none,stroke:none)')+blue,
'polygon':empty('polygon((0pt,0pt),(25pt,0pt),(0pt,18pt),fill:none,stroke:none)')+blue,
'nested':'#box(width:100pt,height:70pt)[#box(width:100pt,height:70pt)['+blank+blue+']]',
'group-boundary':'#place(box(width:40pt,height:30pt)['+blank+'])'+blue,
'clip':blank+'#place(box(width:12pt,height:8pt,clip:true)[#rect(width:80pt,height:70pt,fill:blue,stroke:none)])',
'stroke':blank+'#place(dx:60pt,dy:50pt,rect(width:12pt,height:8pt,fill:none,stroke:blue+1pt))',
'only':blank,
'visible-control':'#place(dx:10pt,dy:10pt,rect(width:25pt,height:18pt,fill:red,stroke:none))'+blue,
}
with gzip.open(out/'initial.json.gz','wt') as initial:json.dump(record,initial)
for name,body in cases.items():
 source='#set page(width:120pt,height:100pt,margin:10pt)\n'+body;f=out/(name+'.typ');f.write_text(source)
 for label,exe,fmt,flags in [('tekt',root/'target/debug/typst','pdf',[]),('compact',root/'target/debug/typst','pdf',['--compact']),('vanilla',root/'lab/typst-original/target/release/typst','pdf',[]),('tekt-svg',root/'target/debug/typst','svg',[]),('vanilla-svg',root/'lab/typst-original/target/release/typst','svg',[])]:
  artifact=out/(name+'-'+label+'.'+fmt);png=artifact.with_suffix('.png');row=dict(case=name,binary=label,binary_sha256=sha(exe),source=source,commands=[]);cmds=[[str(exe),'compile',str(f),str(artifact)]+flags]
  if fmt=='pdf':cmds += [['qpdf','--check',str(artifact)],['pdftoppm','-r','144','-singlefile','-png',str(artifact),str(artifact.with_suffix(''))]]
  else:cmds += [['/home/linuxbrew/.linuxbrew/bin/rsvg-convert','--dpi-x','72','--dpi-y','72','--zoom','2','--background-color','white','--output',str(png),str(artifact)]]
  for cmd in cmds:
   v=subprocess.run(cmd,capture_output=True,text=True);row['commands'].append(dict(command=cmd,exit=v.returncode,stdout=v.stdout,stderr=v.stderr));row['exit']=v.returncode
   if v.returncode:break
  if row['exit']:
   record['rows'].append(row);print(name,label,'ERROR',row['commands'][-1]['stderr'],flush=True);continue
  im=Image.open(png).convert('RGB');row['size']=im.size;row['pixel_sha256']=hashlib.sha256(im.tobytes()).hexdigest();row['artifact_sha256']=sha(artifact)
  for color,select in [('blue',lambda c:c[2]>c[0]+70 and c[2]>c[1]+30),('red',lambda c:c[0]>c[1]+70 and c[0]>c[2]+30)]:
   coords=[(x,y) for y in range(im.height) for x in range(im.width) if select(im.getpixel((x,y)))];row[color+'_bbox']=None if not coords else [min(x for x,y in coords)/2,min(y for x,y in coords)/2,(max(x for x,y in coords)+1)/2,(max(y for x,y in coords)+1)/2]
  record['rows'].append(row);print(name,label,row['blue_bbox'],row['red_bbox'],flush=True)
record['end']=now();assert all(sha(n)==v['sha256'] for n,v in record['files'].items())
with gzip.open(out/'results.json.gz','wt') as f:json.dump(record,f)
