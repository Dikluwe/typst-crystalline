from pathlib import Path
import datetime,gzip,json,hashlib,subprocess,xml.etree.ElementTree as ET
out=Path(__file__).resolve().parents[1];old=Path('00_nucleo/diagnosticos/par-box-fronteira-2026-10-17');folder=out/'parbox-regressions';folder.mkdir(exist_ok=False);sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();records=[]
for corpus in ['after','supplement-after','regions-after','finite-after']:
 data=json.load(gzip.open(old/(corpus+'-measurement.json.gz')))
 for item in data['cases']:
  if item['binary']!='tekt':continue
  source=Path(item['runs'][0]['command'][2] if 'runs' in item else item.get('source') or item['argv'][2]);assert source.is_file(),item
  label=corpus+'-'+item['name'];new=[]
  for kind,bin in [('tekt',Path('target/debug/typst').resolve()),('vanilla',Path('lab/typst-original/target/release/typst').resolve())]:
   dst=folder/(label+'-'+kind+'.pdf');cmd=[str(bin),'compile',str(source),str(dst),'--font-path','/usr/share/fonts/truetype/dejavu'];r=subprocess.run(cmd,capture_output=True,text=True,timeout=40);assert r.returncode==0,(cmd,r.stderr);xml=dst.with_suffix('.bbox.html');r2=subprocess.run(['pdftotext','-bbox',str(dst),str(xml)],capture_output=True);assert r2.returncode==0;tree=ET.parse(xml);pages=[dict(width=float(p.get('width')),height=float(p.get('height')),words=[dict(text=w.text,x0=float(w.get('xMin')),y0=float(w.get('yMin')),x1=float(w.get('xMax')),y1=float(w.get('yMax'))) for w in p.iter('{http://www.w3.org/1999/xhtml}word')]) for p in tree.getroot().iter('{http://www.w3.org/1999/xhtml}page')];new.append(dict(binary=kind,binary_sha256=sha(bin),utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),argv=cmd,exit=r.returncode,stderr=r.stderr,pages=pages,pdf_sha256=sha(dst)))
  before=item['pages'];after=new[0]['pages'];assert len(before)==len(after),(label,'page count');deltas=[]
  for a,b in zip(before,after):
   assert len(a['words'])==len(b['words']),(label,'words')
   for x,y in zip(a['words'],b['words']):
    assert x['text']==y['text'],(label,x,y)
    deltas.extend(abs(x[k]-y[k]) for k in ['x0','x1','y0','y1'])
  assert max(deltas,default=0)<0.002,(label,max(deltas));records.append(dict(label=label,source=str(source),source_sha256=sha(source),baseline=str(old/(corpus+'-measurement.json.gz')),baseline_unchanged=True,max_geometry_delta=max(deltas,default=0),observations=new));print(label,flush=True)
(out/'parbox-regressions.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),pairs=len(records),records=records),indent=2)+'\n');assert len(records)==50
