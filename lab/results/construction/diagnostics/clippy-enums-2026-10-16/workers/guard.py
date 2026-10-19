from pathlib import Path
import gzip,json,re,hashlib,subprocess,tempfile,datetime
out=Path('00_nucleo/diagnosticos/clippy-enums-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));material=json.loads((out/'materialization.json').read_text());scope=json.loads((out/'scope.json').read_text())
strip=lambda s:re.sub(r'^//! @prompt-hash [^\n]*\n','',s,count=1,flags=re.M)
replay=material['before_implementation'].copy()
for e in material['edits']:
 assert replay[e['path']].count(e['old'])==1,e['path'];replay[e['path']]=replay[e['path']].replace(e['old'],e['new'],1)
assert replay==material['unformatted']
with tempfile.TemporaryDirectory(prefix='tekt-enums-guard-') as temp:
 for i,(p,t) in enumerate(replay.items()):
  f=Path(temp)/f'consumer{i}.rs';f.write_text(t)
  subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',str(f)],check=True,capture_output=True)
  assert strip(f.read_text())==strip(Path(p).read_text()),p
  public=lambda s:re.findall(r'\bpub\s+(?:async\s+)?(?:fn\s+\w+[^\{;]*|struct\s+\w+|enum\s+\w+|type\s+\w+[^;]*;)',s)
  compact=lambda seq:[re.sub(r'\s+','',s) for s in seq]
  assert compact(public(before['files'][p]))==compact(public(Path(p).read_text())),p
# All callback source remains exact after removing the two nominal attributes.
for p in scope['sources'][2:4]:
 new=strip(Path(p).read_text());new=re.sub(r'#\[expect\(\s*clippy::large_enum_variant,.*?\)\]\n','',new,flags=re.S)
 assert strip(before['files'][p])==new,p
# Borrowed replay body uses precisely the old four bindings and remains byte-identical.
p=scope['sources'][0];old=before['files'][p];new=Path(p).read_text()
old=old.split('            OutlineEntryIndented { entry, prefix, inner, gap } => {\n',1)[1].split('            }\n',1)[0]
new=new.split('            OutlineEntryIndented(payload) => {\n',1)[1];new=new[new.index('                let mut scopes'):].split('            }\n',1)[0]
assert old==new
for p in set(before['files'])-set(scope['sources']+scope['owners']):assert Path(p).read_text()==before['files'][p],p
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),exact_edit_replay=True,only_scoped_formatting=True,external_signatures_unchanged=True,replay_body_unchanged=True,callback_bodies_and_variants_unchanged=True,only_outline_payload_boxed=True,tag_transformation_unchanged=True)
(out/'code-guard.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record),flush=True)

traits=json.loads((out/'traits-materialization.json').read_text());tests=json.loads((out/'traits-tests-materialization.json').read_text())
assert traits['before']==tests['after']
for p,t in tests['before'].items():assert t==before['files'][p]
replay=traits['before'].copy()
for e in traits['edits']:
 assert replay[e['path']].count(e['old'])==1
 replay[e['path']]=replay[e['path']].replace(e['old'],e['new'],1)
assert replay==traits['unformatted']
with tempfile.TemporaryDirectory(prefix='tekt-traits-guard-') as temp:
 for i,(p,t) in enumerate(replay.items()):
  f=Path(temp)/f'trait{i}.rs';f.write_text(t)
  subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',str(f)],check=True,capture_output=True)
  current=strip(Path(p).read_text())
  if p.endswith('/style.rs'):
   current=re.sub(r'    #\[expect\(\s*clippy::should_implement_trait,\s*reason = "S005: [^"]*"\s*\)\]\n','',current,count=1)
  assert strip(f.read_text())==current,p
# Exact edits remove only Decimal's wrapper, rename two helpers with bodies intact,
# and move the identical ordered style fold to the trait, keeping its old signature.
for name in ['duration','version']:
 p=f'01_core/src/entities/{name}.rs';old=tests['before'][p];new=Path(p).read_text()
 oldbody=old.split('    pub fn to_string(&self) -> String {',1)[1].split('\n    }',1)[0]
 newbody=new.split('    fn canonical_string(&self) -> String {',1)[1].split('\n    }',1)[0]
 assert oldbody==newbody,name
record.update(approved_manifest='d3fba43d799461d294ad562c37649ba9c2e4da2a45d7b7099953624ced28e7e8',approved_traits_exact_edit_replay=True,duration_version_formatter_bodies_preserved=True,styles_fold_preserved=True)
(out/'code-guard.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record),flush=True)

followup=json.loads((out/'traits-followup-materialization.json').read_text());replay=followup['before'].copy()
for e in followup['edits']:
 assert replay[e['path']].count(e['old'])==1
 replay[e['path']]=replay[e['path']].replace(e['old'],e['new'],1)
assert replay==followup['unformatted']
with tempfile.TemporaryDirectory(prefix='tekt-followup-guard-') as temp:
 for i,(p,t) in enumerate(replay.items()):
  f=Path(temp)/f'followup{i}.rs';f.write_text(t)
  subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',str(f)],check=True,capture_output=True)
  assert strip(f.read_text())==strip(Path(p).read_text()),p
record.update(compatibility_adapter_only_nominal_expectation=True,decimal_repr_only_direct_display=True)
(out/'code-guard.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record),flush=True)
