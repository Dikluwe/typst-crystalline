#!/usr/bin/env python3
"""Read-only final checks over frozen source and actual recorded P1308 runs."""
import collections, datetime as dt, json, re, runpy, subprocess
from pathlib import Path
H=Path(__file__).resolve().parent;R=H.parent.parent
v=runpy.run_path(str(H/'p1308-verify.py'));sha,pin,read=v['sha'],v['pin'],v['read']
b=read(H/'p1308-baseline.json');m=read(H/'p1308-manifest.json')
s1=read(H/'p1308-manifest-successor-1.json');s2=read(H/'p1308-manifest-successor-2.json')
f=read(H/'p1308-verification-candidate-freeze-r2.json')
for name,digest in b['predecessors'].items():assert sha((H/name).read_bytes())==digest,name
for ref in m['inputs']+s2['inputs']:assert sha((R/ref['path']).read_bytes())==ref['sha256'],ref['path']
for name,digest in f['source_inventory'].items():assert sha((R/name).read_bytes())==digest,name
contracts={c['path']:c for c in m['contracts']}
contracts[s1['successor_L0']['path']]=s1['successor_L0']
contracts.update({c['path']:c for c in s2['contracts']})
for path,c in contracts.items():assert v['L0_semantic']((R/path).read_bytes())==c['semantic_sha256'],path
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=R).decode().strip()==b['state']['head'].strip()
assert subprocess.check_output(['git','diff','--cached','--binary'],cwd=R).decode()==b['state']['staged']
blocks={}
for name in ('p1308-tests.patch','p1308-tests-supplement.patch','p1308-r2-tests.patch'):
 for block in re.split(r'(?m)(?=^--- a/)',(H/name).read_text()):
  match=re.match(r'--- a/(.+)\n\+\+\+ b/(.+)\n',block)
  if match:
   assert match[1]==match[2];blocks.setdefault(match[1],[]).append(block)
for path,patches in blocks.items():
 data=v['baseline_bytes'](b,path)
 for patch in patches:data=v['apply_block'](data,patch,path)
 assert v['formatted_semantic'](data)==v['formatted_semantic']((R/path).read_bytes()),path
oracle=read(H/'p1308-r2-oracle.json');cases={c['id']:c for c in oracle['cases']}
helper=runpy.run_path(str(H/'p1307-r4-oracle.py'))
maps=[];gates={}
for name in ('public-matrix','public-reverse','public-focal','focal-p1293','workspace-tests','build','lint','fmt'):
 path=H/f'p1308-r2-{name}.json';receipt=read(path);assert receipt['exit']==0,name
 gates[name]=dict(receipt=pin(path),exit=0,at=receipt['at'],end=receipt['end'])
 if name.startswith('public-'):
  data=json.loads(receipt['stdout']);gates[name]['counts']=data['counts'];gates[name]['binary_sha256']=data['binary_sha256']
  assert data['counts']['Violated']==data['counts']['Unknown']==0
  if name in ('public-matrix','public-reverse'):
   seen={}
   for row in data['rows']:
    case=dict(cases[row['case']]);assert row['source_sha256']==case['source_sha256']
    if case['route']=='compile':case['fixture']=row['argv'][2]
    actual=helper['envelope'](row['returncode'],row['stdout'],row['stderr'],case)
    assert actual==row['observable']
    verdict=helper['classify'](case['observations'][row['profile']]['future_expected'],actual)
    assert verdict==row['verdict']=='Preserved'
    key=(row['case'],row['profile']);assert key not in seen;seen[key]=(actual,verdict)
   assert len(seen)==1982;maps.append(seen)
assert maps[0]==maps[1]
w=read(H/'p1308-r2-workspace-tests.json')
totals=[tuple(map(int,x)) for x in re.findall(r'test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',w['stdout'])]
assert tuple(map(sum,zip(*totals)))==(6620,0,3)
controls=[read(H/'p1308-mutation-control.json'),read(H/'p1308-mutation-cli-control.json')]
assert all(c['status']=='GREEN_CONTROL' for c in controls)
assert len(controls[0]['witnesses'])==18 and len(controls[1]['witnesses'])==24
reasons={
 'M01':'Removing Args+None returns cannot add arguments and none; both identity/origin witnesses fail, unsupported-add control survives.',
 'M02':'Removing only native_panic transport yields 5..14 instead of 0..14 and callback 31..40 instead of 26..40; message and named-debt control survive.',
 'M03':'Legacy type_name emits int/str instead of integer/string. Empty filter survives. The planned secondary string boundary also fails for this same vocabulary cause; it is not reported preserved.',
 'M04':'Detached callback origin loses ranges 20..25,17..24,17..32 and external FileId; vocabulary survives and synthetic-native detached control survives.',
 'M05':'Compiled CLI loses the required trace in 20 cells while primary stderr prefix, stdout and exit remain; panic.direct survives in all four profiles.',
 'M06':'Some Args formatter produces integral inline text where width-49 and Unicode-long require multiline; With repr control survives.'}
rows=[];build_seconds=0
for id in reasons:
 path=H/f'p1308-mutation-{id}.json';rec=read(path)
 assert rec['status']=='COMPILED_WITNESS_FAILURE_PENDING_CAUSAL_REVIEW'
 assert rec['commands'][0]['returncode']==0 and not rec['commands'][0]['timed_out']
 assert set(rec['witnesses'].values())=={'Preserved','Violated'}
 assert rec['source_after']==rec['source_before']
 changes={p for p,d in rec['source_before'].items() if d!=f['source_inventory'][p]}
 assert changes=={rec['spec']['changes'][0]['path']}
 assert sha(Path(rec['spec']['patch']['path']).read_bytes())==rec['spec']['patch']['sha256']
 build_seconds+=rec['commands'][0]['elapsed_seconds']
 rows.append(dict(id=id,receipt=pin(path),patch=rec['spec']['patch'],binary_sha256=rec['binary_sha256'],
  source_change=rec['spec']['changes'][0],build_seconds=rec['commands'][0]['elapsed_seconds'],
  witnesses=rec['witnesses'],verdict='KILLED_BY_COMPILED_CAUSAL_WITNESS',causal_review=reasons[id]))
m5=read(H/'p1308-mutation-M05.json')['public_observations']['rows']
for row in m5:
 a,e=row['observable'],row['expected']
 if row['verdict']=='Violated':
  assert a['exit']==e['exit'] and a['stdout']==e['stdout'] and e['stderr'].startswith(a['stderr'])
  assert e['stderr'][len(a['stderr']):].startswith('  while calling ')
assert sum(r['verdict']=='Violated' for r in m5)==20
ledger=dict(schema='p1308-six-real-mutants-ledger-v1',at=dt.datetime.now(dt.timezone.utc).isoformat(),
 executor='/root/p1306_oracle',regime=m['regime'],manifest=pin(H/'p1308-manifest-successor-2.json'),
 freeze=pin(H/'p1308-verification-candidate-freeze-r2.json'),runner=pin(H/'p1308-mutation-runner-v2.py'),
 controls=[pin(H/'p1308-mutation-control.json'),pin(H/'p1308-mutation-cli-control.json')],
 actual_valid_Rust_mutants=6,killed=6,survived=0,Unknown=0,compile_failures=0,mutation_score=1.0,
 rows=rows,mutant_build_seconds=build_seconds,control_build_seconds=sum(c['commands'][0]['elapsed_seconds'] for c in controls),
 scope_limit='Only the six predeclared P1308 families. Rust failing tests stop at first failed assertion/profile; no claim that every negative Rust witness ran in all profiles. M05 CLI covers four profiles. The 37 P1307 families remain unexecuted.',
 restoration=dict(at='2026-09-07T21:53:49.781914+00:00',inventory_files=995,copy_exactly_restored=True,live_candidate_unchanged=True,remaining_builds=0),
 full_preimplementation_discriminatory_seal=False,technical_isolation_attested=False)
final=dict(schema='p1308-r2-independent-final-verification-v1',at=ledger['at'],executor='/root/p1306_oracle',regime=m['regime'],
 verdict='PASS_BOUNDED_P1308_R2_ESSAY_READY_FOR_AUTHORIZED_COMMIT',checker=pin(__file__),
 manifest=pin(H/'p1308-manifest-successor-2.json'),freeze=pin(H/'p1308-verification-candidate-freeze-r2.json'),
 gates=gates,workspace=dict(passed=6620,failed=0,ignored=3),public_normal_reverse=dict(Preserved=1982,Violated=0,Unknown=0,exact_maps_equal=True),
 integrity=dict(source_files=995,source_matches_freeze=True,semantic_L0_pins_valid=True,predecessors_unchanged=True,HEAD_and_index_unchanged_before_commit=True,
  test_integration='Original independent three migrations plus 18 new tests and authorized six stderr literals; canonical application and rustfmt-normalized equality verified.'),
 actual_mutations=dict(valid=6,killed=6,Unknown=0,score=1.0,ledger='p1308-mutation-ledger.json'),
 decision='The authorized trace-only successor resolves the prior preservation conflict without modifying old oracles or making the classifier tolerant. Fresh global workspace and both full public orders pass.',
 source_review='Productive changes remain exactly the previously audited P1308 seven owners; R2 changes only two test owners and lineage metadata. Private Source overlay delegates current World services, native panic selection uses closed function identity, and trace containment is preserved.',
 limitations=['Executed without technical isolation attestation; earlier manifest creation followed coordinator interpretation of the successful pre-candidate checkpoint message as GO. No retroactive full seal.','Six source mutants were executed after candidate GREEN, not used to claim a preimplementation full discriminatory seal.','The 37 P1307 historical mutation families were not executed or discharged. No general P1307 certificate or equivalence claim.','Lint exits zero with existing warning/info output; no claim of zero informational findings.','Prior blocked/Violated records and the suspended first lease remain immutable. No new product process was run by this offline aggregate checker.'],
 commit_authority='User explicitly authorized commit; coordinator owns stage/commit after this bounded verification. No stage or commit performed by verifier.')
print(json.dumps(dict(ledger=ledger,verification=final),ensure_ascii=False,indent=2))
