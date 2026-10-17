from pathlib import Path
import json,gzip,subprocess,datetime,hashlib,re,sys
from PIL import Image
out=Path('00_nucleo/diagnosticos/place-fluxo-2026-10-14')/sys.argv[1];out.mkdir(parents=True,exist_ok=False);root=Path.cwd();sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
sources=['01_core/src/compiler/layout/'+n+'.rs' for n in ['place','placement','block','shape','line_commit','linebreak','parbreak','sub_frame','mod','checkpoint']];owners={n:re.search(r'^//! @prompt (.+)$',Path(n).read_text(),re.M).group(1) for n in sources};paths=sources+list(owners.values());r=dict(start=now(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={n:dict(text=Path(n).read_text(),sha256=sha(n)) for n in paths},rows=[])
placed='#place(dx:7pt,dy:5pt,rect(width:12pt,height:8pt,fill:blue,stroke:none))';red='#rect(width:20pt,height:12pt,fill:red,stroke:none)'
shape='rect(width:12pt,height:8pt,fill:blue,stroke:none)'
def placed_at(align=''):
 return '#place('+((align+',') if align else '')+'dx:7pt,dy:5pt,'+shape+')'
blocks={
 'inset-top':'#block(width:60pt,height:40pt,inset:3pt)['+placed_at()+']',
 'inset-bottom':'#block(width:60pt,height:40pt,inset:3pt)['+placed_at('bottom+right')+']',
 'inset-center':'#block(width:60pt,height:40pt,inset:3pt)['+placed_at('center+horizon')+']',
 'asym-top':'#block(width:60pt,height:40pt,inset:(left:4pt,top:6pt,right:8pt,bottom:10pt))['+placed_at()+']',
 'asym-bottom':'#block(width:60pt,height:40pt,inset:(left:4pt,top:6pt,right:8pt,bottom:10pt))['+placed_at('bottom+right')+']',
 'nested':'#block(width:80pt,height:60pt,inset:4pt)[#block(width:60pt,height:40pt,inset:3pt)['+placed_at()+']]',
 'text-top':'#block(width:60pt,height:40pt,inset:3pt)[A'+placed_at('top')+']',
 'text-omitted':'#block(width:60pt,height:40pt,inset:3pt)[A'+placed_at()+']',
 'clip-top':'#block(width:60pt,height:40pt,inset:3pt,clip:true)['+placed_at()+']',
 'clip-bottom':'#block(width:60pt,height:40pt,inset:3pt,clip:true)['+placed_at('bottom+right')+']',
 'auto-bottom':'#block(width:60pt,inset:3pt)['+placed_at('bottom+right')+']',
 'after-block':'#block(height:20pt)[]#block(width:60pt,height:40pt,inset:3pt)['+placed_at()+']',
}
cases={kind+'-'+name:('#box(width:90pt,height:110pt)['+body+']' if kind=='box' else body) for kind in ['root','box'] for name,body in blocks.items()}
for name,body in cases.items():
 src=out/(name+'.typ');src.write_text('#set page(width:120pt,height:140pt,margin:10pt)\n'+body)
 for label,exe in [('tekt',root/'target/debug/typst'),('vanilla',root/'lab/typst-original/target/release/typst')]:
  svg=out/(name+'-'+label+'.svg');cmd=[str(exe),'compile',str(src),str(svg)];v=subprocess.run(cmd,capture_output=True,text=True);row=dict(case=name,binary=label,binary_sha256=sha(exe),command=cmd,source=src.read_text(),exit=v.returncode,stdout=v.stdout,stderr=v.stderr)
  if not v.returncode:
   png=svg.with_suffix('.png');subprocess.run(['/home/linuxbrew/.linuxbrew/bin/rsvg-convert','--width','480','--height','560','--background-color','white','--output',str(png),str(svg)],check=True,capture_output=True);im=Image.open(png).convert('RGB');xy=[(x,y) for y in range(im.height) for x in range(im.width) if (lambda c:c[2]>c[0]+70 and c[2]>c[1]+30)(im.getpixel((x,y)))];row['blue_bbox']=None if not xy else [min(x for x,y in xy)/4,min(y for x,y in xy)/4,(max(x for x,y in xy)+1)/4,(max(y for x,y in xy)+1)/4];row['pixel_hash']=hashlib.sha256(im.tobytes()).hexdigest();row['svg_hash']=sha(svg)
  r['rows'].append(row);print(name,label,row.get('blue_bbox'),v.returncode,flush=True)
r['end']=now();assert all(sha(n)==v['sha256'] for n,v in r['files'].items())
with gzip.open(out/'results.json.gz','wt') as f:json.dump(r,f)
