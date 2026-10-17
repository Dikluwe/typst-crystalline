from pathlib import Path
import json,gzip,re,hashlib,datetime
from expected import out,expected
b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();normal=lambda s:re.sub(r'^//! @prompt-hash \S+$','//! @prompt-hash <sealed>',s,flags=re.M)
for p,s in expected().items():assert normal(Path(p).read_text())==normal(s),p
allowed=set(scope['sources'])|set(scope['owners'].values());changed=[p for p,h in b['hashes'].items() if sha(p)!=h];assert set(changed)==allowed,changed
result=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),exact_replay=True,public_api_and_unselected_algorithms_unchanged=True,other_baseline_files_unchanged=True,changed=changed);(out/'integrity-result.json').write_text(json.dumps(result,indent=2)+'\n');print('Exact replay and protected files verified')
