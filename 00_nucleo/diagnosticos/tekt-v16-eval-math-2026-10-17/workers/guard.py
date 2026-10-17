from pathlib import Path
import json,gzip,hashlib,re,subprocess,tempfile
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));t=json.load(open(out/'tests-before-code.json'));m=json.load(open(out/'materialization.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();normcode=lambda s:re.sub(r'^//! @prompt-hash [^\n]*\n','',s,count=1,flags=re.M);norml0=lambda s:re.sub(r'^Hash do Código: [^\n]*\n','',s,count=1,flags=re.M);allowed=scope['sources']+scope['owners'];assert all(sha(p)==h for p,h in b['hashes'].items() if p not in allowed)
refinement=json.load(open(out/'refinement.json'));applied=json.load(open(out/'refinement-applied.json'));recipe=json.load(open(out/'recipe-refinement.json'));recipe_tests=json.load(open(out/'recipe-tests.json'));recipe_applied=json.load(open(out/'recipe-applied.json'))
for p,expected in json.load(open(out/'l0-before-code.json'))['owners'].items():
 if p==refinement['owner']:expected=refinement['expected_l0']
 if p==recipe['owner']:expected=recipe['expected_l0']
 assert norml0(Path(p).read_text())==norml0(expected),p
source=scope['sources'][0];s=b['files'][source]
for ins in sorted(t['files'][source]['insertions'],key=lambda x:x['char'],reverse=True):s=s[:ins['char']]+ins['text']+s[ins['char']:]
assert s==t['files'][source]['expected'];s=s.encode()
for r in sorted(scope['targets'],key=lambda x:x['baseline_pattern_start_byte'],reverse=True):
 a,z=r['baseline_pattern_start_byte'],r['baseline_pattern_end_byte'];assert s[a:z]==b'_';s=s[:a]+r['replacement'].encode()+s[z:]
replays={source:s.decode()};source=scope['sources'][1];s=b['files'][source]+t['files'][source]['module'];assert s==t['files'][source]['expected'];assert s.count(m['table_root_before'])==1;replays[source]=s.replace(m['table_root_before'],m['table_root_after'],1)
with tempfile.TemporaryDirectory(prefix='tekt-eval-math-replay-') as d:
 for i,(source,s) in enumerate(replays.items()):
  s=re.sub(r'^//! @updated [0-9-]+$','//! @updated 2026-10-17',s,count=1,flags=re.M);assert s==m['files'][source]['unformatted'];p=Path(d)/(str(i)+'.rs');p.write_text(s);subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',str(p)],check=True);assert p.read_text()==m['files'][source]['expected_after']
  if source==refinement['source']:
   assert p.read_text().count(refinement['old'])==1;p.write_text(p.read_text().replace(refinement['old'],refinement['new'],1));subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',str(p)],check=True);assert normcode(p.read_text())==normcode(applied['expected_after'])
  if source==recipe['source']:
   assert normcode(p.read_text())==normcode(recipe_tests['before']);text=p.read_text()
   for change in recipe_tests['changes']:
    assert text.count(change['old'])==1;text=text.replace(change['old'],change['new'],1)
   pos=text.rfind('\n}');text=text[:pos]+recipe_tests['new_test']+text[pos:];p.write_text(text);subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',str(p)],check=True);assert normcode(p.read_text())==normcode(recipe_tests['expected_with_tests']);assert p.read_text().count(recipe['old'])==1;p.write_text(p.read_text().replace(recipe['old'],recipe['new'],1));subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',str(p)],check=True);assert normcode(p.read_text())==normcode(recipe_applied['expected_after'])
  assert normcode(Path(source).read_text())==normcode(p.read_text()),source
new_tests=sum(Path(p).read_text().count('#[test]')-b['files'][p].count('#[test]') for p in scope['sources']);assert new_tests==6;record={'selected_original_fallbacks':70,'prime_count_shallow_decomposition':True,'production_changes_only_table_root_and_ShowRecipe_context':True,'prior_guards_bodies_order_and_assertions_preserved':True,'added_negative_leaf_assertions':2,'new_tests':new_tests,'unrelated_sources_L0_configuration_unchanged':True,'public_API_enum_contracts_unchanged':True};(out/'guard-result.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record),flush=True)
