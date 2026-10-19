from pathlib import Path
import subprocess,xml.etree.ElementTree as ET,json,gzip,hashlib,datetime
b=Path('00_nucleo/diagnosticos/transformacoes-2026-10-14/logical-pdf-final');prior=json.load(gzip.open(b/'results.json.gz'))
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
rows=[]
for case in sorted({x['case'] for x in prior['metrics'] if x['different_pixels']}):
 pair={}
 for binary in ['tekt','vanilla']:
  p=b/(case+'-'+binary+'.pdf');cmd=['pdftotext','-bbox',str(p),'-'];q=subprocess.run(cmd,capture_output=True,text=True);assert q.returncode==0
  dest=b/(case+'-'+binary+'-bbox.xml');dest.write_text(q.stdout)
  pair[binary]=[dict(text=x.text,**{k:float(v) for k,v in x.attrib.items()}) for x in ET.fromstring(q.stdout).iter() if x.tag.endswith('}word')]
  rows.append(dict(case=case,binary=binary,command=cmd,utc=now(),pdf_sha256=sha(p),xml_sha256=sha(dest),words=pair[binary]))
 a=pair['tekt'];c=pair['vanilla'];same=[x['text'] for x in a]==[x['text'] for x in c]
 delta=max((abs(x[k]-y[k]) for x,y in zip(a,c) for k in ['xMin','xMax','yMin','yMax']),default=0) if same else None
 print(case,same,delta,flush=True);rows.append(dict(case=case,text_equal=same,max_bbox_delta_pt=delta))
with gzip.open(b/'text-geometry.json.gz','wt') as f:json.dump(dict(utc=now(),source_measurement_sha256=sha(b/'results.json.gz'),head=prior['head'],rows=rows),f)
