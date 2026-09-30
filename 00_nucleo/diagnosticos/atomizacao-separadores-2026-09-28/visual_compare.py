from pathlib import Path
import json,subprocess,hashlib,datetime
from PIL import Image,ImageChops
root=Path.cwd();before=root/'00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28';after=root/'00_nucleo/diagnosticos/atomizacao-separadores-2026-09-28/after';out=after/'visual';out.mkdir(exist_ok=True)
names=[p['case'] for p in json.loads((before/'resultados.json').read_text())]+['11-single-spaces','12-explicit-text','13-adjacent-inline','14-long-separated'];mapping={'12-explicit-text':'13-text-literal','13-adjacent-inline':'12-adjacent-control','14-long-separated':'14-long-controls'}
rows=[]
for name in names:
 rendered=[]
 for phase,folder,case in [('before',before,name),('after',after,mapping.get(name,name))]:
  pdf=folder/(case+'-tekt.pdf');prefix=out/(name+'-'+phase);r=subprocess.run(['pdftoppm','-r','144','-png','-singlefile',str(pdf),str(prefix)],capture_output=True,text=True);assert r.returncode==0,r.stderr;rendered.append(Image.open(str(prefix)+'.png').convert('RGB'))
 a,b=rendered;box=ImageChops.difference(a,b).getbbox() if a.size==b.size else 'different-size';rows.append({'case':name,'dpi':144,'page':1,'before_pdf_sha256':hashlib.sha256((before/(name+'-tekt.pdf')).read_bytes()).hexdigest(),'after_pdf_sha256':hashlib.sha256((after/(mapping.get(name,name)+'-tekt.pdf')).read_bytes()).hexdigest(),'pixel_identical':box is None,'difference_bbox':box})
(out/'comparison.json').write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'rows':rows},indent=2)+'\n')
assert all(row['pixel_identical'] for row in rows)
print('14/14 PDFs visualmente idênticos na primeira página a 144 dpi')
