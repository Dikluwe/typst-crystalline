from pathlib import Path
import subprocess,os,json,gzip,hashlib,datetime,collections,sys
out=Path('00_nucleo/diagnosticos/tekt-v16-none-parser-2026-10-16').resolve()
before=json.load(gzip.open(out/'before.json.gz'))
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
assert all(sha(p)==h for p,h in before['project_hashes'].items())
old=before['old_binary']['binary'];new='/tmp/tekt-linter-none-target/debug/crystalline-lint'
assert sha(old)==before['old_binary']['sha256']
record={'start':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),
        'commands':[],'binaries':{'old':{'path':old,'sha256':sha(old)},'new':{'path':new,'sha256':sha(new)}}}
env=dict(os.environ,PATH=str(Path(new).parent)+':'+os.environ['PATH'],CARGO_BUILD_JOBS='1')
for label,cmd,expected in [
('old',[old,'.','--format','sarif'],1),
('new',[new,'.','--format','sarif'],1),
('lineage',[new,'.','--checks','v1,v5,v15,v26','--fail-on','warning'],0),
('reverse',['python3','tools/ci/check_reverse_lineage.py'],0),
('python',['python3','-m','unittest','discover','-s','tools/ci'],0),
('integration',['cargo','test','--offline','--locked','-p','typst-wiring','--test','crystalline_lint','--','--test-threads=1'],0),
('diff',['git','diff','--check'],0)]:
 item={'label':label,'command':cmd,'start':now()};print(json.dumps(item),flush=True)
 with (out/('consumer-'+label+'.log')).open('w') as f:
  p=subprocess.run(cmd,env=env,stdout=f,stderr=subprocess.STDOUT)
 item.update(exit=p.returncode,end=now());record['commands'].append(item)
 (out/'consumer-result.json').write_text(json.dumps(record,indent=2)+'\n')
 assert p.returncode==expected,item
oldrows=json.loads((out/'consumer-old.log').read_text())['runs'][0]['results']
newrows=json.loads((out/'consumer-new.log').read_text())['runs'][0]['results']
key=lambda r:json.dumps(r,sort_keys=True)
prev=collections.Counter(map(key,oldrows));curr=collections.Counter(map(key,newrows))
removed=[json.loads(x) for x in (prev-curr).elements()];added=[json.loads(x) for x in (curr-prev).elements()]
observations=json.load(open(out.parent/'tekt-v16-value-projections-2026-10-16/none-parser-findings.json'))['observations']
expected={(x['source'],x['line']) for x in observations}
actual={(x['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./'),x['locations'][0]['physicalLocation']['region']['startLine']) for x in removed}
assert not added and len(removed)==9 and actual==expected
assert all(x['ruleId']=='V16' and x['level']=='error' for x in removed)
counts=lambda rows:dict(collections.Counter(x['ruleId']+':'+x['level'] for x in rows))
assert counts(oldrows)['V16:error']==253 and counts(newrows)['V16:error']==244
record.update(end=now(),old_counts=counts(oldrows),new_counts=counts(newrows),removed=removed,added=added,
              exact_other_diagnostics_preserved=True,project_sources_unchanged=all(sha(p)==h for p,h in before['project_hashes'].items()))
assert record['project_sources_unchanged']
(out/'consumer-result.json').write_text(json.dumps(record,indent=2)+'\n')
(out/'remaining-v16.json').write_text(json.dumps([x for x in newrows if x['ruleId']=='V16'],indent=2)+'\n')
print(json.dumps({'old':record['old_counts'],'new':record['new_counts'],'removed':9,'all_other_diagnostics_identical':True}),flush=True)
