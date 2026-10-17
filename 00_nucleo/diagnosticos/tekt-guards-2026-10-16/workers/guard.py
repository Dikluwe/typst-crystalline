from pathlib import Path
import json,gzip,re,subprocess,tempfile,datetime
out=Path('00_nucleo/diagnosticos/tekt-guards-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));scope=json.loads((out/'scope.json').read_text());m=json.loads((out/'materialization.json').read_text());tests=json.loads((out/'tests-materialization.json').read_text());strip=lambda s:re.sub(r'^//! @prompt-hash [^\n]*\n','',s,count=1,flags=re.M)
for p,t in tests['before'].items():assert t==before['files'][p]
for p,t in m['before'].items():assert strip(t)==strip(tests['after'].get(p,before['files'][p])),p
replay=m['before'].copy()
for e in m['edits']:
 p=e['path'];assert replay[p].count(e['old'])==e['count'];replay[p]=replay[p].replace(e['old'],e['new'])
assert replay==m['unformatted'] and len(m['edits'])==30
with tempfile.TemporaryDirectory(prefix='tekt-v17-guard-') as temp:
 for i,(p,t) in enumerate(replay.items()):
  f=Path(temp)/f'consumer{i}.rs';f.write_text(t);subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',str(f)],check=True,capture_output=True);assert strip(f.read_text())==strip(Path(p).read_text()),p
  public=lambda s:[re.sub(r'\s+','',v) for v in re.findall(r'\bpub(?:\([^)]*\))?\s+(?:async\s+)?fn\s+\w+[^\{;]*',s)]
  assert public(before['files'][p])==public(Path(p).read_text()),p
  attrs=lambda s:re.findall(r'^\s*#\[(?:expect|allow)\([^\n]*',s,re.M)
  assert attrs(before['files'][p])==attrs(Path(p).read_text()),p
allowed=set(scope['sources']+scope['owners']);changed={p for p,t in before['files'].items() if Path(p).read_text()!=t};assert changed==allowed,changed^allowed
for p in scope['owners']:
 norm=lambda s:re.sub(r'^Hash do Código: [^\n]*\n','',s,count=1,flags=re.M)
 assert norm(Path(p).read_text()).startswith(norm(before['files'][p])),p
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),exact_edit_replay=True,only_scoped_formatting=True,all_function_signatures_unchanged=True,unscoped_sources_and_build_configuration_unchanged=True,old_l0_clauses_preserved=True,no_lint_suppressions_added=True,consumers=len(scope['sources']),owners=len(scope['owners']),replacements=len(m['edits']))
(out/'code-guard.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record),flush=True)
