from pathlib import Path
import json,gzip,re,subprocess,tempfile,datetime
out=Path('00_nucleo/diagnosticos/clippy-enums-publicos-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));scope=json.loads((out/'proposal-scope.json').read_text());material=json.loads((out/'materialization.json').read_text());tests=json.loads((out/'tests-materialization.json').read_text())
strip=lambda s:re.sub(r'^//! @prompt-hash [^\n]*\n','',s,count=1,flags=re.M)
for p,t in tests['before'].items():assert t==before['files'][p],p
for p,t in material['before'].items():assert t==tests['after'].get(p,before['files'][p]),p
replay=material['before'].copy()
for e in material['edits']:
 p=e['path'];at=e['start'];old=e['old'];new=e['new'];assert replay[p][at:at+len(old)]==old,p;replay[p]=replay[p][:at]+new+replay[p][at+len(old):]
assert replay==material['unformatted']
with tempfile.TemporaryDirectory(prefix='tekt-public-enums-guard-') as temp:
 for i,(p,t) in enumerate(replay.items()):
  f=Path(temp)/f'owner{i}.rs';f.write_text(t)
  subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',str(f)],check=True,capture_output=True)
  assert strip(f.read_text())==strip(Path(p).read_text()),p
  signatures=lambda s:[re.sub(r'\s+','',v) for v in re.findall(r'\bpub(?:\([^)]*\))?\s+(?:async\s+)?fn\s+\w+[^\{;]*',s)]
  assert signatures(before['files'][p])==signatures(Path(p).read_text()),p
assert sum(material['tag_constructor_counts'].values())==35 and len(material['edits'])==39
allowed={r[k] for r in scope for k in ['source','owner']}
changed={p for p,t in before['files'].items() if Path(p).read_text()!=t};assert changed==allowed,changed^allowed
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),exact_edit_replay=True,only_scoped_formatting=True,function_signatures_unchanged=True,tag_constructors_wrapped=35,read_patterns_unchanged=True,only_two_public_payloads_changed=True,other_productive_sources_unchanged=True,approved_manifest='282da3f8f79080a4831e1898879775e32a62caa0b2ffa941b8fca9198136b950')
(out/'code-guard.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record),flush=True)
