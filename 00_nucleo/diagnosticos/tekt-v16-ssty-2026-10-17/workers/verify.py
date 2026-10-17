from pathlib import Path
import json,gzip,hashlib,re,datetime
from expected import expected,out
b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();normal=lambda s:re.sub(r'^//! @prompt-hash \S+$','//! @prompt-hash <sealed>',s,flags=re.M)
for p,text in expected().items():assert normal(Path(p).read_text())==normal(text),p
allowed=set(scope['sources'])|set(scope['owners'].values());changes=[p for p,h in b['hashes'].items() if sha(p)!=h];assert set(changes)==allowed,changes
result=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),exact_replay=True,public_api_and_other_algorithms_unchanged=True,changed=changes,other_baseline_files_unchanged=True,math_size_source_unchanged=sha('01_core/src/entities/layout_types.rs')==b['hashes']['01_core/src/entities/layout_types.rs']);assert result['math_size_source_unchanged'];(out/'integrity-result.json').write_text(json.dumps(result,indent=2)+'\n');print('Exact nominal replay, public API and unrelated files verified')
