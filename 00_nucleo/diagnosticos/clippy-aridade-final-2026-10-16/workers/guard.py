from pathlib import Path
import json,gzip,re,subprocess,tempfile,hashlib,datetime
out=Path('00_nucleo/diagnosticos/clippy-aridade-final-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));material=json.loads((out/'materialization.json').read_text())
strip=lambda s:re.sub(r'^//! @prompt-hash [^\n]*\n','',s,count=1,flags=re.M)
# Exact edit transcript recovers the original source, no unrelated code was overwritten.
replay={p:t for p,t in before['files'].items() if p.endswith('.rs')}
for e in material['edits']:
 assert replay[e['path']].count(e['old'])==1,e['path'];replay[e['path']]=replay[e['path']].replace(e['old'],e['new'],1)
assert replay==material['unformatted']
with tempfile.TemporaryDirectory(prefix='tekt-final-guard-') as temp:
 for i,(p,t) in enumerate(replay.items()):
  f=Path(temp)/f'consumer{i}.rs';f.write_text(t)
  subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',str(f)],check=True,capture_output=True)
  assert strip(f.read_text())==strip(Path(p).read_text()),p
  old=before['files'][p];new=Path(p).read_text()
  public=lambda s:re.findall(r'\bpub\s+(?:async\s+)?(?:fn\s+\w+[^\{;]*|struct\s+\w+|enum\s+\w+|type\s+\w+[^;]*;)',s)
  compact=lambda v:[re.sub(r'\s+','',x) for x in v]
  assert compact(public(old))==compact(public(new)),p
  tests=lambda s:re.findall(r'#\[test\]\s*fn\s+(\w+)',s)
  assert tests(old)==tests(new),p
# Algorithms after the single unpack and all table logic outside projection remain identical.
p='01_core/src/compiler/layout/grid.rs';mark='        // P232 — save/restore';assert before['files'][p].split(mark,1)[1]==Path(p).read_text().split(mark,1)[1]
p='01_core/src/compiler/stdlib/structural/outline.rs';mark='    let body = match (prefix, prefix_width_pt, hanging_pt)';end='/// Função contextual qualificada `outline.entry.indented'
assert before['files'][p].split(mark,1)[1].split(end,1)[0]==Path(p).read_text().split(mark,1)[1].split(end,1)[0]
p='03_infra/src/export/svg/groups.rs';mark='fn write_clip_definition(';assert before['files'][p].split(mark,1)[1]==Path(p).read_text().split(mark,1)[1]
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),exact_edit_replay=True,scoped_format_only=True,external_signatures_unchanged=True,test_names_unchanged=True,grid_engine_body_unchanged=True,outline_body_unchanged=True,clip_helpers_unchanged=True,call_maps=material['call_maps'])
(out/'code-guard.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record),flush=True)
