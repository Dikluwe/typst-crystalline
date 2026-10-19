from pathlib import Path
import json,gzip,datetime,hashlib,subprocess,xml.etree.ElementTree as ET
p=Path(__file__).resolve().parent;old=p.parent/'mixed-style-metrics-2026-10-18';binary=Path('target/debug/typst').resolve();out=p/'outputs/mixed-regressions';out.mkdir(parents=True,exist_ok=True)
sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest()
files=['after.json.gz','bib-after.json.gz','regions-after.json.gz','corpus-after.json.gz','corpus-extra-after.json.gz','corpus-boundary-after.json.gz','corpus-repeat-after.json.gz','controls-after.json.gz','controls-extra-after.json.gz','deco-after.json.gz','deco-empty-after.json.gz']
record=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),binary_sha256=sha(binary),source_state='candidate-sources.json.gz',cases=[])
index=0
for name in files:
 d=json.loads(gzip.decompress((old/name).read_bytes()))
 for row in d['cases']:
  original=row['results']['tekt'];originals=original if 'command' in original else original
  formats={Path(original['command'][3]).suffix[1:]:original} if 'command' in original else original
  item=dict(baseline=name,label=row['label'],source=row['source'],source_sha256=row['source_sha256'],results={})
  for fmt,expected in formats.items():
   index+=1;cmd=expected['command'].copy();source=Path(cmd[2]);assert sha(source)==row['source_sha256'],(name,row['label'],'source mutated');target=out/(str(index)+'.'+fmt);cmd[0]=str(binary);cmd[3]=str(target)
   z=subprocess.run(cmd,text=True,capture_output=True,timeout=60);result=dict(command=cmd,exit=z.returncode,stderr=z.stderr)
   same=z.returncode==expected['exit']
   if z.returncode==0:
    result['sha256']=sha(target)
    if fmt=='pdf':
     bbox=target.with_suffix('.bbox');subprocess.run(['pdftotext','-bbox',str(target),str(bbox)],check=True,capture_output=True);root=ET.parse(bbox).getroot();ns={'x':'http://www.w3.org/1999/xhtml'};result['pages']=[dict(width=float(pg.attrib['width']),height=float(pg.attrib['height']),words=[dict(text=w.text,bounds=[float(w.attrib[k]) for k in ['xMin','yMin','xMax','yMax']]) for w in pg.findall('.//x:word',ns)]) for pg in root.findall('.//x:page',ns)]
     result['text']=' '.join(subprocess.check_output(['pdftotext',str(target),'-'],text=True).split())
     same=same and result['pages']==expected['pages'] and ('text' not in expected or result['text']==expected['text'])
    else:same=same and result['sha256']==expected.get('sha256',expected.get('artifact_sha256'))
   elif not same:result['baseline_stderr']=expected['stderr']
   result['same_as_predecessor']=same;item['results'][fmt]=result
  record['cases'].append(item);print(name,row['label'],[(f,v['same_as_predecessor']) for f,v in item['results'].items()],flush=True)
record['end']=datetime.datetime.now(datetime.timezone.utc).isoformat();assert sha(binary)==record['binary_sha256'];record['unchanged']=sum(v['same_as_predecessor'] for row in record['cases'] for v in row['results'].values());record['artifacts']=index
(p/'mixed-regressions.json.gz').write_bytes(gzip.compress(json.dumps(record,indent=2,ensure_ascii=False).encode()));print('UNCHANGED',record['unchanged'],'/',index)
assert record['unchanged']==index,'Inspect differences before deciding regression versus correction'
