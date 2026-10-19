from pathlib import Path
import json,gzip,re,subprocess,tempfile,datetime
out=Path('00_nucleo/diagnosticos/tekt-frame-items-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));scope=json.loads((out/'scope.json').read_text());m=json.loads((out/'materialization.json').read_text());tests=json.loads((out/'tests-materialization.json').read_text());strip=lambda s:re.sub(r'^//! @prompt-hash [^\n]*\n','',s,count=1,flags=re.M)
for p,t in tests['before'].items():assert t==before['files'][p]
for p,t in m['before'].items():assert strip(t)==strip(tests['after'].get(p,before['files'][p])),p
replay={}
assert len(m['edits'])==71
assert sum(e['kind']=='pattern' for e in m['edits'])==69
assert sum(e['kind']=='dead-arm' for e in m['edits'])==1
for p,t in m['before'].items():
 data=t.encode()
 for e in sorted((e for e in m['edits'] if e['path']==p),key=lambda e:e['start'],reverse=True):
  assert data[e['start']:e['end']].decode()==e['old']
  if e['kind']=='pattern':assert e['old'] in ['_','_other'] and '=>' not in e['new']
  elif e['kind']=='dead-arm':assert e['name']=='F051' and e['new']=='' and 'debug_assert!' in e['old'] and '#[allow(unreachable_patterns)]' in e['old']
  elif e['kind']=='doc':assert e['name']=='visitor-doc' and e['new'].startswith('///')
  else:raise AssertionError(e)
  data=data[:e['start']]+e['new'].encode()+data[e['end']:]
 replay[p]=data.decode()
assert replay==m['unformatted']

def enums(s):
 found={}
 for match in re.finditer(r'^pub(?:\([^)]*\))? enum (\w+)\s*\{',s,re.M):
  start=match.start();pos=match.end();depth=1
  while depth:
   c=s[pos];depth+=int(c=='{')-int(c=='}');pos+=1
  found[match[1]]=s[start:pos]
 return found
with tempfile.TemporaryDirectory(prefix='tekt-v16-guard-') as temp:
 for i,(p,t) in enumerate(replay.items()):
  f=Path(temp)/f'consumer{i}.rs';f.write_text(t);subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',str(f)],check=True,capture_output=True);assert strip(f.read_text())==strip(Path(p).read_text()),p
  public=lambda s:[re.sub(r'\s+','',v) for v in re.findall(r'\bpub(?:\([^)]*\))?\s+(?:async\s+)?fn\s+\w+[^\{;]*',s)]
  assert public(before['files'][p])==public(Path(p).read_text()),p
  assert enums(before['files'][p])==enums(Path(p).read_text()),p
  attrs=lambda s:re.findall(r'^\s*#\[(?:expect|allow)\([^\n]*',s,re.M)
  oldattrs=attrs(before['files'][p]);newattrs=attrs(Path(p).read_text())
  if p=='01_core/src/entities/layout_types.rs':oldattrs=[v for v in oldattrs if v.strip()!='#[allow(unreachable_patterns)]']
  assert [v.strip() for v in oldattrs]==[v.strip() for v in newattrs],p
allowed=set(scope['sources']+scope['owners']);changed={p for p,t in before['files'].items() if Path(p).read_text()!=t};assert changed==allowed,changed^allowed
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),exact_edit_replay=True,only_scoped_formatting=True,all_function_signatures_and_enum_definitions_unchanged=True,existing_arm_bodies_preserved=True,no_lint_suppressions_added=True,unscoped_sources_and_build_configuration_unchanged=True,consumers=26,owners=26,matches=70)
(out/'code-guard.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record),flush=True)
