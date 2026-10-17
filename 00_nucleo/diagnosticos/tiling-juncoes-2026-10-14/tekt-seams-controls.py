from pathlib import Path
import subprocess,json,gzip,hashlib,datetime,collections
from PIL import Image
out=Path('00_nucleo/diagnosticos/tiling-juncoes-2026-10-14');qdf=(out/'baseline.qdf.pdf').read_bytes();variants={}
# Equivalent finite drawing, only a diagnostic control, never production layout.
ops='q 1 0 0 -1 20 120 cm 0 0 40 30 re W n\n'
for y in range(-4,30,4):
 for x in range(-4,40,4):ops+=f'q 1 0 0 1 {x} {y} cm /Cell Do Q\n'
ops+='Q\n'
a=qdf.index(b'stream\n',qdf.index(b'10 0 obj'))+7;b=qdf.index(b'\nendstream',a)
manual=qdf[:a]+ops.encode()+qdf[b:];manual=manual.replace(b'/T0 17 0 R\n    >>',b'/T0 17 0 R\n    >>\n /XObject << /Cell 19 0 R >>',1);variants['manual-grid']=manual
# Constant-color four-layer source-over control, with no cell or pattern.
ops='q 1 0 0 -1 20 120 cm /A3f0000003f800000 gs\n'
for color in ['1 1 0','0 1 0','0 0 1','1 0 0']:ops+=f'{color} rg 0 0 40 30 re f\n'
ops+='Q\n';variants['flat-layers']=qdf[:a]+ops.encode()+qdf[b:]
rows=[];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
for name,data in variants.items():
 (out/(name+'.qdf.pdf')).write_bytes(data);pdf=out/(name+'.pdf');pdf.write_bytes(subprocess.run(['fix-qdf'],input=data,capture_output=True,check=True).stdout)
for name in ['baseline','manual-grid','flat-layers']:
 for observer in ['poppler-aa','poppler-no-aa','mupdf','cairo']:
  pdf=out/(name+'.pdf');png=out/(name+'-'+observer+'.png')
  if observer.startswith('poppler'):
   cmd=['pdftoppm','-r','144','-aaVector','yes' if observer=='poppler-aa' else 'no','-png','-singlefile',str(pdf),str(png.with_suffix(''))]
  elif observer=='mupdf':cmd=['mutool','draw','-q','-r','144','-o',str(png),str(pdf)]
  else:cmd=['pdftocairo','-r','144','-png','-singlefile',str(pdf),str(png.with_suffix(''))]
  p=subprocess.run(cmd,capture_output=True,text=True);assert p.returncode==0,(cmd,p.stderr)
  im=Image.open(png).convert('RGB');counts=collections.Counter(im.getpixel((x,y)) for y in range(40,100) for x in range(40,120));expected=(159.375,63.75,79.6875);errors=sorted(max(abs(v-e) for v,e in zip(c,expected)) for c,n in counts.items() for _ in range(n))
  row=dict(variant=name,observer=observer,command=cmd,pdf_sha256=sha(pdf),png_sha256=sha(png),maximum=errors[-1],p95=errors[int(.95*(len(errors)-1))],colors=counts.most_common(8));rows.append(row);print(name,observer,row['maximum'],row['p95'],row['colors'][:4])
(out/'controls.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source_provenance='probe.json.gz and scripts',rows=rows),indent=2))
