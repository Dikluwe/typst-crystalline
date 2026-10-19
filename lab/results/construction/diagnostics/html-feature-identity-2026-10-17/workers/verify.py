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
if len(sys.argv)>1 and sys.argv[1]=='probes':
 a=json.load(gzip.open(out/'before-probe.json.gz'));z=json.load(gzip.open(out/'after-probe.json.gz'));assert a['fixtures']==z['fixtures'];assert a['binaries']['vanilla']==z['binaries']['vanilla'];counts={};changed=[]
 def obs(r):return r['exit'],r['stdout'],r['stderr']
 def norm(r):return r['exit'],r['stdout'],r['stderr'].removesuffix('warning: ficheiro vazio: sem conteúdo\n\n')
 for old,new in zip(a['cases'],z['cases']):
  assert (old['profile'],old['label'],old['expression'])==(new['profile'],new['label'],new['expression']);t=new['results']['tekt'];v=new['results']['vanilla'];o=old['results']['tekt'];assert obs(old['results']['vanilla'])==obs(v)
  disabled=new['gated'] and not new['profile']&({'html':1,'bundle':4}[new['feature']])
  if disabled:
   category='disabled-correct-rejection';assert t['exit']==v['exit']==1;assert t['stderr'].removesuffix('warning: ficheiro vazio: sem conteúdo\n\n')==v['stderr'],(new['label'],t,v)
  elif new['label']=='control/{ import "bridge.typ" as bridge; bridge.root == std }':
   category='preexisting-cross-file-identity-divergence';assert obs(o)==obs(t);assert t['stdout']=='false\n' and v['stdout']=='true\n'
  else:
   category='active-or-ordinary-value';assert t['exit']==v['exit']==0;assert t['stdout']==v['stdout'],(new['label'],t,v);assert t['stderr']=='warning: ficheiro vazio: sem conteúdo\n\n' and v['stderr']==''
  counts[category]=counts.get(category,0)+1
  if obs(o)!=obs(t):changed.append(dict(profile=new['profile'],label=new['label']))
 result=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source_hashes=z['source_hashes'],binaries=z['binaries'],cases=len(z['cases']),categories=counts,changed=changed,feature_disabled_no_active_parity_credit=True);(out/'probe-verification.json').write_text(json.dumps(result,indent=2)+'\n');print('probe verified',counts,len(changed))
print('API and unrelated integrity verified',len(changes))
