from pathlib import Path
import datetime,gzip,json,hashlib,subprocess,xml.etree.ElementTree as ET
out=Path(__file__).resolve().parents[1];prior=Path('00_nucleo/diagnosticos/text-size-length-2026-10-17');folder=out/'ts1-replay';folder.mkdir(exist_ok=False);sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();records=[];changed=[];allowed={'relative-vs-zero','affine-vs-absolute-part','relative-vs-resolved'}
for corpus in ['after-measurement','after-extended','morph-after-measurement']:
 data=json.load(gzip.open(prior/(corpus+'.json.gz')))
 for old in data['cases']:
  assert sha(old['source'])==old['source_sha256'];cmd=old['argv'].copy();cmd[0]=str(Path('target/debug/typst' if old['binary']=='tekt' else 'lab/typst-original/target/release/typst').resolve());dst=folder/(corpus+'-'+old['name']+'-'+old['binary']+'.pdf');cmd[3]=str(dst.resolve());r=subprocess.run(cmd,capture_output=True,text=True,timeout=40);assert (r.returncode,r.stdout,r.stderr)==(old['exit'],old['stdout'],old['stderr']),(corpus,old['name'],old['binary'],r.stderr);row=dict(corpus=corpus,name=old['name'],binary=old['binary'],utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),argv=cmd,binary_sha256=sha(cmd[0]),source_sha256=sha(cmd[2]),exit=r.returncode,stdout=r.stdout,stderr=r.stderr)
  if r.returncode==0:
   xml=dst.with_suffix('.bbox.html');z=subprocess.run(['pdftotext','-bbox',str(dst),str(xml)],capture_output=True);assert z.returncode==0;tree=ET.parse(xml);pages=[dict(width=float(p.get('width')),height=float(p.get('height')),words=[dict(text=w.text,x0=float(w.get('xMin')),y0=float(w.get('yMin')),x1=float(w.get('xMax')),y1=float(w.get('yMax'))) for w in p.iter('{http://www.w3.org/1999/xhtml}word')]) for p in tree.getroot().iter('{http://www.w3.org/1999/xhtml}page')];row.update(pages=pages,pdf_sha256=sha(dst));oldwords=[w for p in old['pages'] for w in p['words']];newwords=[w for p in pages for w in p['words']]
   if old['binary']=='tekt' and old['name'] in allowed:
    assert [w['text'] for w in newwords]==['true'],(old['name'],newwords);assert [w['text'] for w in oldwords]==['false'];changed.append(old['name'])
   else:
    assert len(old['pages'])==len(pages) and len(oldwords)==len(newwords),(old['name'],'page/word count')
    assert all(a['text']==b['text'] and max(abs(a[k]-b[k]) for k in ['x0','x1','y0','y1'])<.002 for a,b in zip(oldwords,newwords)),(old['name'],old['binary'],'geometry changed')
  records.append(row)
 print(corpus,flush=True)
assert len(records)==210 and set(changed)==allowed
(out/'ts1-replay.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),pairs=len(records)//2,unchanged_cases=102,intended_boolean_repairs=changed,all_other_measured_observables_preserved=True,records=records),indent=2)+'\n')
