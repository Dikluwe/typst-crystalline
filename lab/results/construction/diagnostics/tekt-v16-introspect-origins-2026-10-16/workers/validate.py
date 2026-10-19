from pathlib import Path
import subprocess,json,gzip,hashlib,datetime,os
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-origins-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();hashes={p:sha(p) for p in before['hashes']};linter=before['linter']['binary'];assert sha(linter)==before['linter']['sha256']
record={'start':now(),'head':before['head'],'source_hashes':hashes,'status':subprocess.check_output(['git','status','--porcelain=v1'],text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'commands':[]}
commands=[
('guard',['python3',str(out/'workers/guard.py')],0),
('controls',['cargo','test','--offline','--locked','-p','typst-core','v16_origins_','--','--test-threads=1'],0),
('single-eval',['cargo','test','--offline','--locked','-p','typst-core','single_evaluation_','--','--test-threads=1'],0),
('probe-value',['python3',str(out/'workers/probe.py'),'green','Value'],0),
('probe-content',['python3',str(out/'workers/probe.py'),'green','Content'],0),
('build',['cargo','build','--offline','--locked','--workspace'],0),
('fmt',['cargo','fmt','--check','--all'],0),
('clippy',['python3',str(out/'workers/strict.py')],0),
('lineage',[linter,'.','--checks','v1,v5,v15,v26','--fail-on','warning'],0),
('reverse',['python3','tools/ci/check_reverse_lineage.py'],0),
('architecture',[linter,'.','--format','sarif'],1),
('differential',['python3',str(out/'workers/differential.py')],0),
('workspace',['python3',str(out/'workers/suite.py')],0),
('rustdoc',['cargo','doc','--offline','--locked','--workspace','--no-deps'],0),
('diff',['git','diff','--check'],0)]
for label,cmd,expected in commands:
 item={'label':label,'command':cmd,'start':now()};print(json.dumps(item),flush=True);env=dict(os.environ)
 if label=='rustdoc':env['RUSTDOCFLAGS']='-D warnings'
 with (out/('validation-'+label+'.log')).open('w') as log:r=subprocess.run(cmd,env=env,stdout=log,stderr=subprocess.STDOUT)
 item.update(exit=r.returncode,end=now());record['commands'].append(item);record['sources_unchanged_during_validation']=all(sha(p)==h for p,h in hashes.items());record['end']=now()
 (out/'validation-result.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(item),flush=True)
 assert record['sources_unchanged_during_validation']
 assert r.returncode==expected,item
 if label=='controls':assert '11 passed; 0 failed;' in (out/'validation-controls.log').read_text()
record['validation_complete']=True;(out/'validation-result.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps({'stage_validation':'PASS','remaining_v16':239}),flush=True)
