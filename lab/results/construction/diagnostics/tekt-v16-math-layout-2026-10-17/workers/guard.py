from pathlib import Path
import json,gzip,hashlib,re,subprocess,tempfile
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));t=json.load(open(out/'tests-before-code.json'));m=json.load(open(out/'materialization.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();normcode=lambda s:re.sub(r'^//! @prompt-hash [^\n]*\n','',s,count=1,flags=re.M);norml0=lambda s:re.sub(r'^Hash do Código: [^\n]*\n','',s,count=1,flags=re.M);allowed=scope['sources']+scope['owners'];assert all(sha(p)==h for p,h in b['hashes'].items() if p not in allowed)
refinement=json.load(open(out/'clippy-refinement.json')) if (out/'clippy-refinement.json').exists() else None
for p,s in json.load(open(out/'l0-before-code.json'))['owners'].items():
 if refinement and p==refinement['owner']:s=refinement['after_owner'];assert refinement['l0_written_before_code']
 assert norml0(Path(p).read_text())==norml0(s),p
replay={p:t['expected_files'].get(p,b['files'].get(p,'')) for p in scope['sources']}
assert replay[t['new_source']]==t['new_source_text'];assert replay[t['parent']]==b['files'][t['parent']]+t['declaration'];assert replay[t['accent']]==b['files'][t['accent']]+t['accent_tests']
for source in scope['existing_sources']:
 raw=replay[source].encode();edits=[]
 for r in scope['targets']:
  if r['source']!=source:continue
  if r['optional_content']:
   old=r['match_source'].encode();assert raw.count(old)==1;a=raw.index(old);z=a+len(old);new='let c = c?;\n            match c.source_body() {\n                Content::MathIdent(s) | Content::MathText(s) => s.chars().next(),\n                '+r['replacement']+' => None,\n            }'
  else:a,z=r['pattern_start_byte'],r['pattern_end_byte'];assert raw[a:z]==b'_';new=r['replacement']
  edits.append((a,z,new.encode()))
 for a,z,new in sorted(edits,reverse=True):raw=raw[:a]+new+raw[z:]
 replay[source]=raw.decode()
for edit in m['functional_edits']:
 source=edit['source'];assert replay[source].count(edit['old'])==1;replay[source]=replay[source].replace(edit['old'],edit['new'],1)
with tempfile.TemporaryDirectory(prefix='tekt-math-replay-') as directory:
 for i,(source,s) in enumerate(replay.items()):
  s=re.sub(r'^//! @updated [0-9-]+$','//! @updated 2026-10-17',s,count=1,flags=re.M);assert s==m['files'][source]['unformatted'];p=Path(directory)/(str(i)+'.rs');p.write_text(s);subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',str(p)],check=True);assert p.read_text()==m['files'][source]['expected_after']
  expected=p.read_text()
  if refinement and source==refinement['source']:
   assert normcode(expected)==normcode(refinement['before_source']);assert expected.count(refinement['old'])==1;expected=expected.replace(refinement['old'],refinement['new'])
  assert normcode(Path(source).read_text())==normcode(expected),source
public=lambda s:re.findall(r'^\s*pub(?!\()\s+[^\n]+',s,re.M)
for source in scope['existing_sources']:assert public(b['files'][source])==public(Path(source).read_text()),source
new_tests=sum(Path(p).read_text().count('#[test]')-b['files'].get(p,'').count('#[test]') for p in scope['sources']);# Macro expands 23 concrete tests from one annotated definition.
assert new_tests==7,new_tests
record={'original_fallbacks':36,'partition_forwarding_auxiliary':True,'replay_all_authorized_edits':True,'public_declarations_preserved':True,'unrelated_sources_L0_configuration_unchanged':True,'new_concrete_tests':29,'new_annotated_definitions':new_tests,'source_transformations_preserve_origin_by_existing_helpers':True};(out/'guard-result.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record),flush=True)
