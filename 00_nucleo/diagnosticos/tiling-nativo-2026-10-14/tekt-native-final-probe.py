from pathlib import Path
import subprocess,json,gzip,hashlib,datetime,sys
from PIL import Image,ImageChops
phase=sys.argv[1];root=Path.cwd();out=root/'00_nucleo/diagnosticos/tiling-nativo-2026-10-14'/phase;out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
binaries={'tekt':root/'target/debug/typst','vanilla':root/'lab/typst-original/target/release/typst'}
assert sha(binaries['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
if phase=='before-renders':
 with gzip.open(root/'00_nucleo/diagnosticos/raster-mascaras-2026-10-14/after-final/results.json.gz','rt') as f:old=json.load(f)
 assert sha(binaries['tekt'])==old['binaries']['tekt']['sha256']
body='#rect(width:3pt,height:4pt,fill:red,stroke:none)'
pat='tiling(size:(8pt,6pt))['+body+']'
cases={
 'solid':'#rect(width:40pt,height:30pt,fill:red,stroke:none)',
 'fill':'#rect(width:40pt,height:30pt,fill:'+pat+',stroke:none)',
 'empty':'#rect(width:40pt,height:30pt,fill:tiling(size:(8pt,6pt))[],stroke:none)',
 'auto':'#rect(width:40pt,height:30pt,fill:tiling['+body+'],stroke:none)',
 'rotate-offset':'#rect(width:40pt,height:30pt,fill:tiling(size:(8pt,6pt),angle:37deg,offset:(3pt,5pt),spacing:(-2pt,4pt))['+body+'],stroke:none)',
 'cell-parent-gradient':'#rect(width:40pt,height:30pt,fill:tiling(size:(8pt,6pt))[#rect(width:8pt,height:6pt,fill:gradient.linear(red,blue,relative:"parent"),stroke:none)],stroke:none)',
 'stroke-open':'#rect(width:40pt,height:30pt,fill:none,stroke:(paint:'+pat+',thickness:6pt))',
 'dense-open':'#rect(width:200pt,height:100pt,fill:tiling(size:(1pt,1pt))[#rect(width:0.5pt,height:0.5pt,fill:red,stroke:none)],stroke:none)',
 'auto-empty-error':'#rect(width:40pt,height:30pt,fill:tiling[],stroke:none)',
 'pitch-error':'#rect(width:40pt,height:30pt,fill:tiling(size:(8pt,6pt),spacing:(-8pt,0pt))['+body+'],stroke:none)',
 'body-error':'#rect(width:40pt,height:30pt,fill:tiling(size:(8pt,6pt))[@missing-tile-label],stroke:none)',
}
cases.update({
 'nested': '#rect(width:40pt,height:30pt,fill:tiling(size:(12pt,12pt))[#rect(width:8pt,height:8pt,fill:'+pat+',stroke:none)],stroke:none)',
 'text-cell': '#rect(width:40pt,height:30pt,fill:tiling(size:(12pt,12pt))[#text(size:8pt,fill:blue)[A]],stroke:none)',
 'image-cell': '#rect(width:40pt,height:30pt,fill:tiling(size:(8pt,8pt))[#image("../../../../03_infra/fixtures/p307b/sources/tiny.jpg",width:4pt,height:4pt)],stroke:none)',
 'overlap-alpha': '#rect(width:40pt,height:30pt,fill:tiling(size:(8pt,8pt),spacing:(-4pt,-4pt))[#rect(width:8pt,height:8pt,fill:rgb(255,0,0,50%),stroke:none)],stroke:none)',
 'ellipse-stroke': '#ellipse(width:40pt,height:30pt,fill:none,stroke:(paint:'+pat+',thickness:6pt))',
 'parent-place': '#place(dx:8pt,dy:7pt)[#rect(width:40pt,height:30pt,fill:tiling(size:(8pt,6pt),relative:"parent")['+body+'],stroke:none)]',
 'parent-box': '#box(width:60pt,height:50pt,inset:5pt)[#rect(width:40pt,height:30pt,fill:tiling(size:(8pt,6pt),relative:"parent")['+body+'],stroke:none)]',
 'offset-large': '#rect(width:40pt,height:30pt,fill:tiling(size:(8pt,6pt),offset:(800003pt,600005pt))['+body+'],stroke:none)',
})
cases['subpixel'] = '#rect(width:40pt,height:30pt,fill:tiling(size:(0.1pt,0.1pt))[#rect(width:0.05pt,height:0.05pt,fill:red,stroke:none)],stroke:none)'
record=dict(start=now(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),binaries={k:{'path':str(v),'sha256':sha(v)} for k,v in binaries.items()},binary_source_provenance='raster-mascaras-2026-10-14/after-final/results.json.gz' if phase=='before-renders' else 'candidate-final.json.gz',rows=[])
for name,body in cases.items():
 src=out/(name+'.typ');src.write_text('#set page(width:240pt,height:140pt,margin:0pt)\n#place(dx:20pt,dy:20pt)['+body+']')
 for binary,exe in binaries.items():
  for fmt in ('svg','pdf','png'):
   dest=out/(name+'-'+binary+'.'+fmt);cmd=[str(exe),'compile',str(src),str(dest),'--root',str(root)]+(['--ppi','144'] if fmt=='png' else [])
   r=subprocess.run(cmd,capture_output=True,text=True)
   row=dict(case=name,binary=binary,format=fmt,command=cmd,exit=r.returncode,stderr=r.stderr,source=src.read_text())
   if r.returncode==0:
    row['sha256']=sha(dest)
    raster=dest.with_suffix('.'+fmt+'.png')
    if fmt=='svg':
     rendered=subprocess.run(['/home/linuxbrew/.linuxbrew/bin/rsvg-convert','--dpi-x','72','--dpi-y','72','--zoom','2','--background-color','white','--output',str(raster),str(dest)],capture_output=True)
     if rendered.returncode:
      row['render_error']=rendered.stderr.decode();record['rows'].append(row);continue
    elif fmt=='pdf':
     q=subprocess.run(['qpdf','--check',str(dest)],capture_output=True,text=True);row['qpdf_exit']=q.returncode;row['qpdf_stdout']=q.stdout;row['qpdf_stderr']=q.stderr
     subprocess.run(['pdftoppm','-r','144','-png','-singlefile',str(dest),str(raster.with_suffix(''))],check=True,capture_output=True)
    else:raster=dest
    im=Image.open(raster).convert('RGBA');bg=Image.new('RGBA',im.size,'white');bg.alpha_composite(im);im=bg.convert('RGB');row['size']=im.size;row['pixels_sha256']=hashlib.sha256(im.tobytes()).hexdigest()
   record['rows'].append(row)
 print(name,flush=True)
record['end']=now();assert all(sha(v['path'])==v['sha256'] for v in record['binaries'].values())
with gzip.open(out/'results.json.gz','wt') as f:json.dump(record,f)
