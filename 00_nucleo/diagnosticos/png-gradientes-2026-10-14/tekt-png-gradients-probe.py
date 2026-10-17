from pathlib import Path
import subprocess,json,gzip,hashlib,datetime,sys
from PIL import Image,ImageChops
root=Path.cwd(); stage=root/'00_nucleo/diagnosticos/png-gradientes-2026-10-14'; stage.mkdir(exist_ok=True)
phase=sys.argv[1]; out=stage/phase;out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
paths=[p for top in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'] for p in Path(top).rglob('*') if p.is_file() and p.suffix in ('.rs','.md','.toml')]
record=dict(start=now(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={str(p):sha(p) for p in paths},rows=[])
record['sources']={str(p):p.read_text() for p in [Path('03_infra/src/export/render.rs'),Path('00_nucleo/prompts/infra/export/render.md')]}
bins={'tekt':root/'target/debug/typst','vanilla':root/'lab/typst-original/target/release/typst'}
record['binaries']={k:{'path':str(p),'sha256':sha(p)} for k,p in bins.items()}
assert sha(bins['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
if phase.startswith('before'):
 with gzip.open(root/'00_nucleo/diagnosticos/tiling-overlap-2026-10-14/final-renders/results.json.gz','rt') as f:prior=json.load(f)
 assert record['binaries']==prior['binaries']
 record['binary_source_provenance']='tiling-overlap-2026-10-14/candidate-final.json.gz'
def rect(g,extra=''):return '#rect(width:40pt,height:30pt,fill:'+g+',stroke:none'+extra+')'
linear='gradient.linear(red,blue,space:rgb)'
cases={
 'solid':rect('red'),
 'linear':rect(linear),
 'linear-angle':rect('gradient.linear(red,blue,angle:37deg,space:rgb)'),
 'linear-negative-angle':rect('gradient.linear(red,blue,angle:-123deg,space:rgb)'),
 'linear-oklab':rect('gradient.linear(red,blue)'),
 'linear-alpha':rect('gradient.linear(rgb(255,0,0,20%),rgb(0,0,255,80%),space:rgb)'),
 'linear-stops':rect('gradient.linear((red,0%),(green,45%),(blue,100%),space:rgb)'),
 'sharp':rect('gradient.linear(red,blue,angle:37deg).sharp(3)'),
 'radial':rect('gradient.radial(red,blue,space:rgb)'),
 'radial-focal':rect('gradient.radial(red,blue,center:(60%,40%),radius:60%,focal-center:(40%,30%),focal-radius:10%,space:rgb)'),
 'conic':rect('gradient.conic(red,blue,space:rgb)'),
 'conic-offset':rect('gradient.conic(red,green,blue,center:(30%,70%),angle:37deg,space:rgb)'),
 'parent-page':rect('gradient.linear(red,blue,relative:"parent",space:rgb)'),
 'parent-box':'#box(width:60pt,height:50pt,inset:5pt)['+rect('gradient.linear(red,blue,relative:"parent",space:rgb)')+']',
 'parent-rotate':'#rotate(23deg,origin:top+left)['+rect('gradient.linear(red,blue,relative:"parent",space:rgb)')+']',
 'parent-scale':'#scale(x:140%,y:60%,origin:top+left)['+rect('gradient.linear(red,blue,relative:"parent",space:rgb)')+']',
 'self-rotate':'#rotate(23deg,origin:top+left)['+rect(linear)+']',
 'self-scale':'#scale(x:140%,y:60%,origin:top+left)['+rect(linear)+']',
 'stroke-rect':'#rect(width:40pt,height:30pt,fill:none,stroke:(paint:'+linear+',thickness:6pt))',
 'stroke-ellipse':'#ellipse(width:40pt,height:30pt,fill:none,stroke:(paint:'+linear+',thickness:6pt))',
 'stroke-round':'#rect(width:40pt,height:30pt,radius:8pt,fill:none,stroke:(paint:'+linear+',thickness:6pt))',
 'stroke-dash':'#rect(width:40pt,height:30pt,fill:none,stroke:(paint:'+linear+',thickness:6pt,dash:"dashed"))',
 'stroke-parent':'#rect(width:40pt,height:30pt,fill:none,stroke:(paint:gradient.linear(red,blue,relative:"parent",space:rgb),thickness:6pt))',
 'cell-parent':rect('tiling(size:(8pt,6pt))[#rect(width:8pt,height:6pt,fill:gradient.linear(red,blue,relative:"parent",space:rgb),stroke:none)]'),
 'cell-self':rect('tiling(size:(8pt,6pt))[#rect(width:8pt,height:6pt,fill:gradient.radial(red,blue,space:rgb),stroke:none)]'),
 'cell-alpha':rect('tiling(size:(8pt,6pt),spacing:(-4pt,-3pt))[#rect(width:8pt,height:6pt,fill:gradient.linear(rgb(255,0,0,20%),rgb(0,0,255,80%),space:rgb),stroke:none)]'),
 'clip':'#box(width:30pt,height:20pt,clip:true)['+rect(linear)+']',
 'page-gradient':'#set page(fill:gradient.linear(red,blue,space:rgb))\n'+rect('none'),
}
prefix='#set page(width:120pt,height:100pt,margin:0pt)\n'
for name,body in cases.items():
 src=out/(name+'.typ');src.write_text(prefix+(body if name=='page-gradient' else '#place(dx:20pt,dy:20pt)['+body+']'))
 for key,exe in bins.items():
  dest=out/(name+'-'+key+'.png');cmd=[str(exe),'compile',str(src),str(dest),'--ppi','144']
  start=now();r=subprocess.run(cmd,capture_output=True,text=True,timeout=60)
  row=dict(case=name,binary=key,command=cmd,start=start,end=now(),exit=r.returncode,stderr=r.stderr,source=src.read_text())
  if r.returncode==0:
   im=Image.open(dest).convert('RGBA');row.update(sha256=sha(dest),size=im.size,pixels_sha256=hashlib.sha256(im.tobytes()).hexdigest())
  record['rows'].append(row)
 a=out/(name+'-tekt.png');b=out/(name+'-vanilla.png')
 if a.exists() and b.exists():
  ia=Image.open(a).convert('RGBA');ib=Image.open(b).convert('RGBA')
  if ia.size==ib.size:
   ds=[max(v) for v in ImageChops.difference(ia,ib).getdata()];print(name,max(ds),sum(v>16 for v in ds),flush=True)
 else:print(name,'compile error',flush=True)
record['end']=now();assert all(sha(p)==h for p,h in record['files'].items());assert all(sha(x['path'])==x['sha256'] for x in record['binaries'].values())
with gzip.open(out/'results.json.gz','wt') as f:json.dump(record,f)
