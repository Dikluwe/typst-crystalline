from pathlib import Path
import json,gzip,subprocess,hashlib,datetime,shutil,collections
from PIL import Image
out=Path('00_nucleo/diagnosticos/tiling-juncoes-2026-10-14');out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
paths=['03_infra/src/export/pdf_tiling.rs','00_nucleo/prompts/infra/export/pdf_tiling.md','03_infra/src/export/stream/draw.rs','00_nucleo/prompts/infra/export/stream/draw.md']
record=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:dict(text=Path(p).read_text(),sha256=sha(p)) for p in paths},rows=[])
source=Path('00_nucleo/diagnosticos/tiling-overlap-2026-10-14/final-renders/overlap-order-tekt.pdf');shutil.copyfile(source,out/'source.pdf');record['source_pdf_sha256']=sha(source);record['source_provenance']='tiling-overlap-2026-10-14/final-renders/results.json.gz'
qdf=subprocess.check_output(['qpdf','--qdf','--object-streams=disable','--stream-data=uncompress',str(source),'-'])
variants={'baseline':qdf}
for isolate in ['true','false']:
 variants['cell-group-'+isolate]=qdf.replace(b'/Subtype /Form',b'/Group << /S /Transparency /CS /DeviceRGB /I '+isolate.encode()+b' /K false >>\n/Subtype /Form')
variants['cell-group-inherit']=qdf.replace(b'/Subtype /Form',b'/Group << /S /Transparency /I true /K false >>\n/Subtype /Form')
variants['no-aa']=qdf.replace(b'/CA 1',b'/CA 1 /SA false')
# Eliminate XObject boundary but keep equivalent cell clip, using the same body bytes.
body=qdf.split(b'19 0 obj')[1].split(b'stream\n',1)[1].split(b'\nendstream')[0]
inline=qdf
for x,y in [(-4,-4),(0,-4),(-4,0),(0,0)]:
 op=f'q 1 0 0 1 {x} {y} cm /Cell Do Q'.encode()
 inline=inline.replace(op,f'q 1 0 0 1 {x} {y} cm 0 0 8 8 re W n\n'.encode()+body+b'\nQ')
# The inline body needs its own existing opacity resource in the pattern.
inline=inline.replace(b'/Cell 19 0 R\n    >>',b'/Cell 19 0 R\n    >>\n /ExtGState << /A3f0000003f800000 << /ca 0.5 /CA 1 >> >>')
variants['inline-cell']=inline
for name,data in variants.items():
 raw=out/(name+'.qdf.pdf');raw.write_bytes(data);fixed=subprocess.run(['fix-qdf'],input=data,capture_output=True,check=True).stdout;pdf=out/(name+'.pdf');pdf.write_bytes(fixed)
 q=subprocess.run(['qpdf','--check',str(pdf)],capture_output=True,text=True);assert q.returncode==0,(name,q.stderr)
 for engine in ['poppler','mupdf']:
  png=out/(name+'-'+engine+'.png')
  cmd=['pdftoppm','-r','144','-png','-singlefile',str(pdf),str(png.with_suffix(''))] if engine=='poppler' else ['mutool','draw','-q','-r','144','-o',str(png),str(pdf)]
  r=subprocess.run(cmd,capture_output=True,text=True);assert r.returncode==0,(name,engine,r.stderr)
  im=Image.open(png).convert('RGB');colors=collections.Counter(im.getpixel((x,y)) for y in range(40,100) for x in range(40,120));expected=(159.375,63.75,79.6875);errors=sorted(max(abs(v-e) for v,e in zip(c,expected)) for c,count in colors.items() for _ in range(count));row=dict(variant=name,engine=engine,command=cmd,pdf_sha256=sha(pdf),png_sha256=sha(png),colors=colors.most_common(8),maximum=errors[-1],p95=errors[int(.95*(len(errors)-1))]);record['rows'].append(row);print(name,engine,row['maximum'],row['p95'],row['colors'][:4],flush=True)
record['end']=datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(out/'probe.json.gz','wt') as f:json.dump(record,f)
