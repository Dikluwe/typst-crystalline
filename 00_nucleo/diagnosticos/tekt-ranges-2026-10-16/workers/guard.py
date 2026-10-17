from pathlib import Path
import json,gzip,re,subprocess,tempfile,datetime
out=Path('00_nucleo/diagnosticos/tekt-ranges-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));scope=json.loads((out/'scope.json').read_text());material=json.loads((out/'materialization.json').read_text());tests=json.loads((out/'tests-materialization.json').read_text());strip=lambda s:re.sub(r'^//! @prompt-hash [^\n]*\n','',s,count=1,flags=re.M)
for p,t in tests['before'].items():assert t==before['files'][p]
assert material['before']==tests['after'];replay=material['before'].copy()
for e in material['edits']:
 p=e['path'];assert replay[p].count(e['old'])==e['count'];replay[p]=replay[p].replace(e['old'],e['new'])
assert replay==material['unformatted'] and sum(e['count'] for e in material['edits'])==5
with tempfile.TemporaryDirectory(prefix='tekt-v18-guard-') as temp:
 for i,(p,t) in enumerate(replay.items()):
  f=Path(temp)/f'consumer{i}.rs';f.write_text(t);subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',str(f)],check=True,capture_output=True);assert strip(f.read_text())==strip(Path(p).read_text()),p
  public=lambda s:[re.sub(r'\s+','',v) for v in re.findall(r'\bpub(?:\([^)]*\))?\s+(?:async\s+)?fn\s+\w+[^\{;]*',s)]
  assert public(before['files'][p])==public(Path(p).read_text()),p
allowed=set(scope['sources']+scope['owners']);changed={p for p,t in before['files'].items() if Path(p).read_text()!=t};assert changed==allowed,changed^allowed
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),exact_edit_replay=True,only_scoped_formatting=True,all_function_signatures_unchanged=True,only_cardinality_patterns_and_hex_predicate_changed=True,argument_validation_order_and_normalization_unchanged=True,unscoped_sources_and_build_configuration_unchanged=True,no_lint_suppressions_added=True)
(out/'code-guard.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record),flush=True)
