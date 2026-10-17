from pathlib import Path
import subprocess,json,gzip,datetime,hashlib
from PIL import Image,ImageChops
root=Path.cwd(); stage=root/'00_nucleo/diagnosticos/png-gradientes-2026-10-14';out=stage/'svg-controls';out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(stage/'candidate-final.json.gz','rt') as f:candidate=json.load(f)
assert all(sha(p)==v['sha256'] for p,v in candidate['files'].items())
record=dict(start=now(),candidate='candidate-final.json.gz',candidate_sha256=sha(stage/'candidate-final.json.gz'),head=candidate['head'],rows=[])
for name in ['rotate-solid','scale-solid','round-solid','ellipse-solid']:
 src=stage/'isolation'/f'{name}.typ'
 for key,exe in [('tekt',root/'target/debug/typst'),('vanilla',root/'lab/typst-original/target/release/typst')]:
  dest=out/f'{name}-{key}.svg';cmd=[str(exe),'compile',str(src),str(dest)]
  r=subprocess.run(cmd,capture_output=True,text=True,timeout=60);row=dict(case=name,binary=key,exe_sha256=sha(exe),command=cmd,source_sha256=sha(src),exit=r.returncode,stderr=r.stderr)
  if r.returncode==0:
   raster=dest.with_suffix('.png');cmd=['/home/linuxbrew/.linuxbrew/bin/rsvg-convert','--dpi-x','72','--dpi-y','72','--zoom','2','--background-color','white','--output',str(raster),str(dest)];q=subprocess.run(cmd,capture_output=True,text=True);row.update(svg_sha256=sha(dest),raster_command=cmd,raster_exit=q.returncode,raster_stderr=q.stderr)
  record['rows'].append(row)
record['end']=now();assert all(sha(p)==v['sha256'] for p,v in candidate['files'].items());(out/'results.json').write_text(json.dumps(record,indent=2));print('SVG geometry controls saved')
