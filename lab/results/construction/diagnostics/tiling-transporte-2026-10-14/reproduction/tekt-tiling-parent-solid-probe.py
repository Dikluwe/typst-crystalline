from pathlib import Path
import json,gzip,datetime,hashlib,subprocess,re,sys,xml.etree.ElementTree as ET
from PIL import Image
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tiling-transporte-2026-10-14'/sys.argv[1];out.mkdir(parents=True,exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
paths=['01_core/src/compiler/layout/tiling.rs','01_core/src/compiler/layout/shape.rs','01_core/src/compiler/layout/boxed.rs','01_core/src/compiler/layout/block.rs','01_core/src/entities/layout_types.rs','01_core/src/entities/geometry.rs','01_core/src/entities/paint.rs','01_core/src/entities/tiling.rs','03_infra/src/export/render.rs','03_infra/src/export/svg.rs','03_infra/src/export/stream/shape.rs']
paths=[p for p in paths if Path(p).exists()];paths += [re.search(r'^//! @prompt (.+)$',Path(p).read_text(),re.M).group(1) for p in list(paths)]
bins={'tekt':root/'target/debug/typst','vanilla':root/'lab/typst-original/target/release/typst'};assert sha(bins['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
r=dict(start=now(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),files={p:dict(sha256=sha(p),text=Path(p).read_text()) for p in paths},binaries={n:dict(path=str(p),sha256=sha(p)) for n,p in bins.items()},rows=[])
shape='rect(width:40pt,height:24pt,fill:p,stroke:none)'
variants={'direct':'#'+shape,'place':'#place(dx:17pt,dy:13pt,'+shape+')','box-fixed':'#place(dx:17pt,dy:13pt,box(width:60pt,height:40pt,inset:7pt)['+'#'+shape+'])','box-auto':'#place(dx:17pt,dy:13pt,box(inset:7pt)['+'#'+shape+'])','block-fixed':'#place(dx:17pt,dy:13pt,block(width:60pt,height:40pt,inset:7pt)['+'#'+shape+'])','block-auto':'#place(dx:17pt,dy:13pt,block(inset:7pt)['+'#'+shape+'])','box-place':'#place(dx:17pt,dy:13pt,box(width:60pt,height:40pt)[#place(dx:7pt,dy:5pt,'+shape+')])','rotated-box':'#place(dx:17pt,dy:13pt,rotate(25deg,origin:top+left,box(width:60pt,height:40pt,inset:7pt)['+'#'+shape+']))','rotated-soft':'#place(dx:17pt,dy:13pt,rotate(25deg,origin:top+left,'+shape+'))'}
for name,body in variants.items():
 for relative in ['self']:
  case=name+'-'+relative;source=out/(case+'.typ');source.write_text('#set page(width:120pt,height:96pt,margin:0pt)\n#let p=tiling(size:(8pt,6pt),offset:(3pt,1pt),angle:35deg,relative:"'+relative+'",rect(width:3pt,height:2pt,fill:red,stroke:none))\n'+body.replace('fill:p','fill:red'))
  for label,exe in bins.items():
   for fmt in ['svg','pdf']:
    output=out/(case+'-'+label+'.'+fmt);cmd=[str(exe),'compile',str(source),str(output)];v=subprocess.run(cmd,capture_output=True,text=True);row=dict(case=case,binary=label,format=fmt,source=source.read_text(),command=cmd,start=now(),exit=v.returncode,stdout=v.stdout,stderr=v.stderr)
    if v.returncode==0:
     row['sha256']=sha(output)
     if fmt=='svg':
      tree=ET.fromstring(output.read_text());row['patterns']=[dict(x.attrib) for x in tree.iter() if x.tag.endswith('pattern')];png=output.with_name(output.stem+'-svg.png');subprocess.run(['/home/linuxbrew/.linuxbrew/bin/rsvg-convert','--width','240','--height','192','--background-color','white','--output',str(png),str(output)],check=True,capture_output=True)
     else:
      q=subprocess.run(['qpdf','--check',str(output)],capture_output=True,text=True);row['qpdf']=dict(exit=q.returncode,stdout=q.stdout,stderr=q.stderr);png=output.with_suffix('.png');subprocess.run(['pdftoppm','-scale-to-x','240','-scale-to-y','192','-singlefile','-png',str(output),str(output.with_suffix(''))],check=True,capture_output=True)
     im=Image.open(png).convert('RGB');row['pixels_sha256']=hashlib.sha256(im.tobytes()).hexdigest()
    r['rows'].append(row);print(case,label,fmt,v.returncode,flush=True)
r['end']=now();assert all(sha(p)==v['sha256'] for p,v in r['files'].items());assert all(sha(bins[n])==v['sha256'] for n,v in r['binaries'].items())
with gzip.open(out/'results.json.gz','wt') as f:json.dump(r,f)
