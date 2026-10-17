from pathlib import Path
import json,gzip,datetime,hashlib,subprocess,sys,re,xml.etree.ElementTree as ET
from PIL import Image,ImageChops
root=Path.cwd();out=root/'00_nucleo/diagnosticos/block-auto-width-2026-10-14'/sys.argv[1];out.mkdir(parents=True,exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
paths=list(json.loads((out.parent/'owners.json').read_text()));paths += [re.search(r'^//! @prompt (.+)$',Path(p).read_text(),re.M).group(1) for p in list(paths)]
bins={'tekt':root/'target/debug/typst','vanilla':root/'lab/typst-original/target/release/typst'};assert sha(bins['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
r=dict(start=now(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),files={p:dict(sha256=sha(p),text=Path(p).read_text()) for p in paths},binaries={n:dict(path=str(p),sha256=sha(p)) for n,p in bins.items()},rows=[])
shape='#rect(width:40pt,height:24pt,fill:red,stroke:none)'
bodies={'shape':shape,'empty':'','overlay-only':'#place(dx:90pt,dy:2pt,rect(width:8pt,height:6pt,fill:red,stroke:none))','shape-overlay':shape+'#place(dx:90pt,dy:2pt,rect(width:8pt,height:6pt,fill:red,stroke:none))','text':'ABC','literal':'#"ABC "','markup':'ABC ','two-lines':'ABC\\ A','nested':'#block(inset:3pt)['+shape+']','fixed-child':'#block(width:70pt,inset:3pt)['+shape+']','box':'#box(width:40pt,height:24pt,fill:red)[]','grid':'#grid(columns:(20pt,30pt),gutter:4pt)[A][B]','stack':'#stack(dir:ltr,spacing:4pt,box(width:20pt,height:10pt,fill:red),box(width:30pt,height:10pt,fill:red))','align':'#align(right)['+shape+']','math':'$ x^2 $'}
bodies.update({'grid-fr':'#grid(columns:(1fr,1fr),gutter:4pt)[A][B]','grid-mixed':'#grid(columns:(20pt,1fr),gutter:4pt)[A][B]','grid-single':'#grid(columns:(20pt,),gutter:4pt)[A]','grid-negative':'#grid(columns:(20pt,30pt),gutter:-4pt)[A][B]','grid-rows':'#grid(columns:(20pt,30pt),gutter:4pt)[A][B][C][D]'})
for case,body in bodies.items():
 for clip in [False,True]:
  name=case+('-clip' if clip else '')
  source=out/(name+'.typ');source.write_text('#set page(width:160pt,height:120pt,margin:0pt)\n#set text(size:10pt)\n#place(dx:17pt,dy:13pt,block(fill:blue,inset:7pt,clip:'+str(clip).lower()+')['+body+'])')
  for label,exe in bins.items():
   output=out/(name+'-'+label+'.svg');cmd=[str(exe),'compile',str(source),str(output)];v=subprocess.run(cmd,capture_output=True,text=True);row=dict(case=name,binary=label,source=source.read_text(),command=cmd,exit=v.returncode,stdout=v.stdout,stderr=v.stderr)
   if v.returncode==0:
    tree=ET.fromstring(output.read_text());row['blue_shapes']=[dict(e.attrib) for e in tree.iter() if e.attrib.get('fill','').lower()=='#0074d9'];png=output.with_suffix('.png');subprocess.run(['/home/linuxbrew/.linuxbrew/bin/rsvg-convert','--dpi-x','72','--dpi-y','72','--zoom','2','--background-color','white','--output',str(png),str(output)],check=True,capture_output=True)
    im=Image.open(png).convert('RGB');row['pixels_sha256']=hashlib.sha256(im.tobytes()).hexdigest();mask=Image.new('L',im.size);mask.putdata([255 if b>180 and r<80 and g<160 else 0 for r,g,b in im.getdata()]);row['blue_bbox']=mask.getbbox()
   r['rows'].append(row)
  print(name,flush=True)
r['end']=now();assert all(sha(p)==v['sha256'] for p,v in r['files'].items());assert all(sha(bins[n])==v['sha256'] for n,v in r['binaries'].items())
with gzip.open(out/'results.json.gz','wt') as f:json.dump(r,f)
