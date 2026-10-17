from pathlib import Path
import gzip,json,re,hashlib,subprocess,tempfile,datetime
out=Path('00_nucleo/diagnosticos/clippy-retencao-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));material=json.loads((out/'materialization.json').read_text())
strip=lambda s:re.sub(r'^//! @prompt-hash [^\n]*\n','',s,count=1,flags=re.M)
replay=material['before_implementation'].copy()
for e in material['edits']:
 assert replay[e['path']].count(e['old'])==1,e['path'];replay[e['path']]=replay[e['path']].replace(e['old'],e['new'],1)
assert replay==material['unformatted']
with tempfile.TemporaryDirectory(prefix='tekt-retention-guard-') as temp:
 for i,(p,t) in enumerate(replay.items()):
  f=Path(temp)/f'consumer{i}.rs';f.write_text(t)
  subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',str(f)],check=True,capture_output=True)
  assert strip(f.read_text())==strip(Path(p).read_text()),p
# Restore the declared maintenance deltas; the rest of each consumer must be exact.
for p in replay:
 restored=Path(p).read_text()
 # The formatted template and replay above prove changes are limited to the transcript.
 # Function signatures other than the unreachable wrapper are preserved.
 public=lambda s:re.findall(r'\bpub\s+(?:async\s+)?(?:fn\s+\w+[^\{;]*|struct\s+\w+|enum\s+\w+|type\s+\w+[^;]*;)',s)
 old=before['files'][p];a=public(old);b=public(restored)
 if p.endswith('/subset.rs'):a=[s for s in a if not re.match(r'pub\s+fn\s+subset_font\(',s)]
 compact=lambda seq:[re.sub(r'\s+','',s) for s in seq]
 assert compact(a)==compact(b),p
 tests=lambda s:re.findall(r'#\[test\]\s*fn\s+(\w+)',s)
 if p.endswith('context_stabilization.rs'):
  assert set(tests(restored))-set(tests(old))=={'execution_retains_style_values_through_retirement_and_vector_growth'}
 else:assert tests(old)==tests(restored)
p='03_infra/src/pipeline/context_stabilization.rs';old=before['files'][p];new=Path(p).read_text()
add=lambda t:t[t.index('    fn add('):t.index('    fn execute(')]
assert add(old).replace('            parent,\n','')==add(new)
assert 'retired: Vec<Execution>' in new and new.count('self.retired.push(*')==3
assert '_retained_styles: styles,' in new and 'current: Option<Box<Execution>>' in new
assert 'mod subset;' in Path('03_infra/src/export/mod.rs').read_text()
uses=subprocess.run(['rg','-n',r'\bsubset_font\b','01_core','02_shell','03_infra','04_wiring','benches'],capture_output=True,text=True);assert uses.returncode==1,uses.stdout
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),exact_edit_replay=True,only_scoped_formatting=True,productive_signatures_preserved=True,parent_argument_and_target_calculation_preserved=True,all_retirement_transitions_preserve_record=True,retention_field_preserved=True,no_legacy_subset_references=True)
(out/'code-guard.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record),flush=True)
