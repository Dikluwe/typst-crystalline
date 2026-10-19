from pathlib import Path
import gzip,json,subprocess,hashlib,datetime,re,sys,collections
from PIL import Image
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tiling-transformacao-2026-10-14'/sys.argv[1];out.mkdir(parents=True,exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
files=['01_core/src/compiler/layout/tiling.rs','01_core/src/entities/tiling.rs','01_core/src/compiler/stdlib/visualize.rs','01_core/src/entities/layout_types.rs']
files += [re.search(r'^//! @prompt (.+)$',Path(p).read_text(),re.M).group(1) for p in list(files)]
files += ['lab/typst-original/crates/typst-library/src/visualize/tiling.rs','lab/typst-original/crates/typst-library/src/layout/transform.rs','lab/typst-original/crates/typst-svg/src/shape.rs','lab/typst-original/crates/typst-svg/src/paint.rs']
bins={'tekt':root/'target/debug/typst','vanilla':root/'lab/typst-original/target/release/typst'};assert sha(bins['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
r=dict(start=now(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:dict(text=Path(p).read_text(),sha256=sha(p)) for p in files},binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in bins.items()},cases=[])
for name,angle,offset,relative,placement in [
 ('plain',0,'(0pt,0pt)','self',True),
 ('offset',0,'(3pt,5pt)','self',True),
 ('quarter',90,'(0pt,0pt)','self',True),
 ('quarter-offset',90,'(3pt,5pt)','self',True),
 ('oblique',35,'(0pt,0pt)','self',True),
 ('oblique-offset',35,'(3pt,5pt)','self',True),
 ('oblique-ratio',35,'(50%,25%)','self',True),
 ('parent-offset',35,'(3pt,5pt)','parent',False),
 ('self-offset',35,'(3pt,5pt)','self',False),
 ('parent-place',35,'(3pt,5pt)','parent',True),
]:
 body=f'rect(width:64pt,height:48pt,stroke:none,fill:tiling(size:(12pt,16pt),spacing:(4pt,2pt),offset:{offset},angle:{angle}deg,relative:"{relative}")[#rect(width:3pt,height:7pt,fill:red,stroke:none)])'
 source='#set page(width:120pt,height:96pt,margin:(left:5pt,top:7pt,right:5pt,bottom:7pt))\n#'+('place(dx:15pt,dy:17pt,'+body+')' if placement else body)+'\n'
 for label,binary in bins.items():
  p=out/f'{name}-{label}.typ';p.write_text(source)
  for fmt in ['pdf','svg']:
   cmd=[str(binary),'compile','--format',fmt,str(p),str(p.with_suffix('.'+fmt))];x=subprocess.run(cmd,capture_output=True,text=True);row=dict(name=name,binary=label,format=fmt,source=source,command=cmd,exit=x.returncode,stdout=x.stdout,stderr=x.stderr)
   if x.returncode==0 and fmt=='pdf':
    subprocess.run(['pdftoppm','-r','144','-singlefile','-png',str(p.with_suffix('.pdf')),str(p.with_suffix(''))],check=True,capture_output=True);im=Image.open(p.with_suffix('.png')).convert('RGB');row.update(painted=sum(max(255-v for v in px)>32 for px in im.getdata()),pixels_sha256=hashlib.sha256(im.tobytes()).hexdigest())
   r['cases'].append(row);print(name,label,fmt,x.returncode,row.get('painted'),flush=True)
r['end']=now();assert all(sha(p)==v['sha256'] for p,v in r['files'].items());assert all(sha(bins[k])==v['sha256'] for k,v in r['binaries'].items())
with gzip.open(out/'results.json.gz','wt') as f:json.dump(r,f)
