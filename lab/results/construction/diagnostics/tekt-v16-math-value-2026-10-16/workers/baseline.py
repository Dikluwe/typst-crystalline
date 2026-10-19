from pathlib import Path
import subprocess,json,gzip,hashlib,datetime
out=Path('00_nucleo/diagnosticos/tekt-v16-math-value-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'))
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
record={'start':now(),'head':before['head'],'commands':[]}
for label,cmd,expected in [
('fmt',['rustfmt','--edition','2021','--config','skip_children=true',*scope['sources']],0),
('seal',['python3',str(out/'workers/seal_owned.py'),*scope['sources']],0),
('controls',['cargo','test','--offline','--locked','-p','typst-core','v16_math_value_','--','--test-threads=1'],0),
('existing-math',['cargo','test','--offline','--locked','-p','typst-core','p1431','--','--test-threads=1'],0),
('architecture',[before['linter']['binary'],'.','--format','sarif'],1),
('probe',['python3',str(out/'workers/probe.py'),'baseline'],0)]:
 item={'label':label,'command':cmd,'start':now()};print(json.dumps(item),flush=True)
 with (out/('baseline-'+label+'.log')).open('w') as log:r=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT)
 item.update(exit=r.returncode,end=now());record['commands'].append(item)
 (out/'baseline-result.json').write_text(json.dumps(record,indent=2)+'\n')
 assert r.returncode==expected,item
 if label=='controls':assert '7 passed; 0 failed;' in (out/'baseline-controls.log').read_text()
 if label=='architecture':
  rows=json.loads((out/'baseline-architecture.log').read_text())['runs'][0]['results']
  v16=[x for x in rows if x['ruleId']=='V16'];assert len(v16)==244
  for target in scope['targets']:
   assert len([x for x in v16 if x['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./')==target['source'] and x['locations'][0]['physicalLocation']['region']['startLine']==target['line']])==1,target
record.update(end=now(),files={p:Path(p).read_text() for p in scope['sources']+scope['owners']},source_hashes={p:sha(p) for p in before['hashes']},structural_red=True)
(out/'baseline-result.json').write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps({'baseline':'PASS','functional_controls':7,'selected_evolution_protections_missing':4,'v16':244}),flush=True)
