from pathlib import Path
import json,gzip,re,hashlib,datetime,sys
out=Path(__file__).resolve().parents[1];before=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
def contract(t):
 result=[]
 for m in re.finditer(r'^\s*pub fn \w+',t,re.M):result.append(re.sub(r'\s+','',t[m.start():t.index('{',m.start())]))
 return dict(functions=result,structs=[re.sub(r'\s+','',m[0]) for m in re.finditer(r'^pub struct Module\([^;]+;',t,re.M)],exports=[re.sub(r'\s+','',m[0]) for m in re.finditer(r'^pub use [\s\S]*?;',t,re.M)])
values={p:contract(Path(p).read_text()) for p in scope['sources']};assert values==json.load(open(out/'public-contract-before.json'));(out/'public-contract-after.json').write_text(json.dumps(values,indent=2)+'\n')
allowed=set(scope['sources'])|set(scope['owners'].values());changes=[p for p,h in before['hashes'].items() if sha(p)!=h];assert set(changes)<=allowed,changes
(out/'integrity-result.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),unrelated_sources_preserved=True,changed=changes,new_nuclei={p:sha(p) for p in scope['nuclei']},public_api_preserved=True),indent=2)+'\n')
print('API and unrelated integrity verified',len(changes))
