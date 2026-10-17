from pathlib import Path
import subprocess,json,gzip,hashlib,datetime,collections
out=Path('00_nucleo/diagnosticos/tekt-v16-none-parser-2026-10-16').resolve();before=json.load(gzip.open(out/'before.json.gz'));package=json.load(open(out/'package-result.json'));publication=json.load(open(out/'publication-result.json'));proposal=json.load(open(out/'proposed-pin.json'))
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
assert publication['pin_applied'] and publication['remote_exact_commit_verified']
changed=sorted(p for p,h in before['project_hashes'].items() if sha(p)!=h);assert changed==sorted(proposal),changed
for path,item in proposal.items():assert Path(path).read_text()==item['after'],path
binary=package['binary']['path'];assert sha(binary)==package['binary']['sha256']
record={'start':now(),'commit':package['commit'],'project_head':before['project_head'],'changed_project_files':changed,'commands':[],'source_hashes':{p:sha(p) for p in before['project_hashes']}}
for label,cmd,expected in [
 ('architecture',[binary,'.','--format','sarif'],1),
 ('lineage',[binary,'.','--checks','v1,v5,v15,v26','--fail-on','warning'],0),
 ('reverse',['python3','tools/ci/check_reverse_lineage.py'],0),
 ('python',['python3','-m','unittest','discover','-s','tools/ci'],0),
 ('diff',['git','diff','--check'],0)]:
 item={'label':label,'command':cmd,'start':now()};print(json.dumps(item),flush=True)
 with (out/('post-pin-'+label+'.log')).open('w') as log:p=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT)
 item.update(exit=p.returncode,end=now());record['commands'].append(item)
 (out/'post-pin-result.json').write_text(json.dumps(record,indent=2)+'\n');assert p.returncode==expected,item
old=json.loads((out/'consumer-new.log').read_text())['runs'][0]['results'];new=json.loads((out/'post-pin-architecture.log').read_text())['runs'][0]['results']
assert collections.Counter(json.dumps(x,sort_keys=True) for x in old)==collections.Counter(json.dumps(x,sort_keys=True) for x in new)
assert all(sha(p)==h for p,h in record['source_hashes'].items())
record.update(end=now(),diagnostics_identical_to_review=True,counts=dict(collections.Counter(x['ruleId']+':'+x['level'] for x in new)),files_unchanged_during_validation=True)
(out/'post-pin-result.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps({'post_pin_validation':'PASS','counts':record['counts']}),flush=True)
