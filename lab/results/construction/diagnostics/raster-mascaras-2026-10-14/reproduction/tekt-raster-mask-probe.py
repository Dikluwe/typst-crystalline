from pathlib import Path
import sys,subprocess,hashlib,datetime,gzip,json,time
from PIL import Image,ImageChops
root=Path.cwd();out=root/'00_nucleo/diagnosticos/raster-mascaras-2026-10-14'/sys.argv[1];out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
bins={'tekt':root/'target/debug/typst','vanilla':root/'lab/typst-original/target/release/typst'}
assert sha(bins['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
paths=['03_infra/src/export/render.rs','00_nucleo/prompts/infra/export/render.md']
r=dict(start=now(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={n:dict(text=Path(n).read_text(),sha256=sha(n)) for n in paths},binaries={n:dict(path=str(p),sha256=sha(p)) for n,p in bins.items()},rows=[])
asset=out/'red.png';Image.new('RGBA',(8,4),(255,0,0,255)).save(asset)
shape='#rect(width:80pt,height:60pt,fill:red,stroke:none)'
rect='block(width:40pt,height:30pt,inset:4pt,clip:true)['+shape+']'
curve='curve(fill:red,stroke:none,fill-rule:"even-odd",curve.move((0pt,0pt)),curve.line((40pt,0pt)),curve.line((40pt,40pt)),curve.line((0pt,40pt)),curve.close(),curve.move((10pt,10pt)),curve.line((30pt,10pt)),curve.line((30pt,30pt)),curve.line((10pt,30pt)),curve.close())'
bodies={'clip':'#'+rect,'no-clip':'#'+rect.replace('clip:true','clip:false'),'rounded':'#'+rect.replace('clip:true','clip:true,radius:10pt'),'nested':'#block(width:50pt,height:40pt,inset:5pt,clip:true)[#place(dx:20pt,dy:10pt,'+rect+')]','rotate':'#rotate(30deg,origin:top+left,'+rect+')','text':'#block(width:40pt,height:8pt,clip:true)[#text(size:20pt)[ABCD]]','image-cover':'#image("red.png",width:12pt,height:16pt,fit:"cover")','even-odd':'#'+curve,'non-zero':'#'+curve.replace('even-odd','non-zero'),'dash':'#line(length:50pt,stroke:(paint:red,thickness:6pt,dash:(4pt,4pt),cap:"round"))','solid':'#rect(width:40pt,height:30pt,fill:red,stroke:none)','tiling':'#rect(width:40pt,height:30pt,fill:tiling(size:(10pt,10pt))[#rect(width:15pt,height:15pt,fill:red,stroke:none)],stroke:none)'}
bodies.update({'even-odd-zero': '#'+curve.replace('stroke:none','stroke:0pt'), 'non-zero-zero':'#'+curve.replace('stroke:none','stroke:0pt').replace('even-odd','non-zero')})
bodies.update({'even-odd-stroke': '#'+curve.replace('stroke:none','stroke:1pt'), 'non-zero-stroke':'#'+curve.replace('stroke:none','stroke:1pt').replace('even-odd','non-zero')})
bodies['alpha']='#rect(width:40pt,height:30pt,fill:rgb("#ff000080"),stroke:none)'
if len(sys.argv)>2:bodies={k:v for k,v in bodies.items() if k in sys.argv[2].split(',')}
for case,body in bodies.items():
 source=out/(case+'.typ');source.write_text('#set page(width:120pt,height:100pt,margin:0pt)\n'+('#set page(fill:none)\n' if case=='alpha' else '')+'#place(dx:20pt,dy:20pt)['+body+']')
 for name,exe in bins.items():
  for fmt in ['png','svg']:
   dest=out/(case+'-'+name+'.'+fmt);cmd=[str(exe),'compile',str(source),str(dest)]+(['--ppi','144'] if fmt=='png' else []);start=time.monotonic();v=subprocess.run(cmd,capture_output=True,text=True);row=dict(case=case,binary=name,format=fmt,source=source.read_text(),command=cmd,exit=v.returncode,stderr=v.stderr,seconds=time.monotonic()-start)
   if v.returncode==0:
    if fmt=='svg':
     dest=dest.with_name(dest.stem+'-svg.png');subprocess.run(['/home/linuxbrew/.linuxbrew/bin/rsvg-convert','--dpi-x','72','--dpi-y','72','--zoom','2','--background-color','white','--output',str(dest),str(out/(case+'-'+name+'.svg'))],check=True,capture_output=True)
    im=Image.open(dest).convert('RGBA');bg=Image.new('RGBA',im.size,'white');bg.alpha_composite(im);im=bg.convert('RGB');mask=Image.new('L',im.size);mask.putdata([255 if r>180 and g<100 and b<100 else 0 for r,g,b in im.getdata()]);row.update(red_bbox=mask.getbbox(),red_pixels=sum(bool(v) for v in mask.getdata()),pixels_sha256=hashlib.sha256(im.tobytes()).hexdigest())
   r['rows'].append(row)
 print(case,flush=True)
r['end']=now();assert all(sha(n)==v['sha256'] for n,v in r['files'].items());assert all(sha(v['path'])==v['sha256'] for v in r['binaries'].values())
with gzip.open(out/'results.json.gz','wt') as f:json.dump(r,f)
