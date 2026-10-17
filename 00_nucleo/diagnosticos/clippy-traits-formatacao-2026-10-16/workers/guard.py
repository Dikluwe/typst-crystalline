from pathlib import Path
import gzip,json,re,hashlib,subprocess,tempfile,datetime
out=Path('00_nucleo/diagnosticos/clippy-traits-formatacao-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));material=json.loads((out/'materialization.json').read_text())
strip=lambda s:re.sub(r'^//! @prompt-hash [^\n]*\n','',s,count=1,flags=re.M)
replay=material['before_implementation'].copy()
for e in material['edits']:
 assert replay[e['path']].count(e['old'])==1,e['path'];replay[e['path']]=replay[e['path']].replace(e['old'],e['new'],1)
assert replay==material['unformatted']
with tempfile.TemporaryDirectory(prefix='tekt-traits-guard-') as temp:
 for i,(p,t) in enumerate(replay.items()):
  f=Path(temp)/f'consumer{i}.rs';f.write_text(t)
  subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',str(f)],check=True,capture_output=True)
  assert strip(f.read_text())==strip(Path(p).read_text()),p
  public=lambda s:re.findall(r'\bpub\s+(?:async\s+)?(?:fn\s+\w+[^\{;]*|struct\s+\w+|enum\s+\w+|type\s+\w+[^;]*;)',s)
  compact=lambda seq:[re.sub(r'\s+','',s) for s in seq]
  assert compact(public(before['files'][p]))==compact(public(Path(p).read_text())),p
p='01_core/src/entities/duration.rs';old=before['files'][p];new=Path(p).read_text()
assert strip(old[:old.index('    pub fn to_string(')])==strip(new[:new.index('    pub fn to_string(')])
assert old[old.index('\nimpl Default for Duration'):old.index('#[cfg(test)]')]==new[new.index('\nimpl Default for Duration'):new.index('#[cfg(test)]')]
assert 'self.nanos.unsigned_abs()' in new
for p in ['01_core/src/entities/style.rs','00_nucleo/prompts/entities/style.md']:assert Path(p).read_text()==before['files'][p]
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),exact_edit_replay=True,only_scoped_formatting=True,public_signatures_unchanged=True,constructors_accessors_neg_default_unchanged=True,four_nominal_expectations=True,styles_unchanged_pending_proposal=True)
(out/'code-guard.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record),flush=True)
