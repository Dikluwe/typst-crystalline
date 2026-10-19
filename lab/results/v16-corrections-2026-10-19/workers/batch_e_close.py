"""Fail-closed audit of batch E receipts, actual test logs and compiler witnesses."""
from pathlib import Path
from collections import Counter
import datetime,gzip,hashlib,json,os,re,subprocess,sys
import yaml
ROOT=Path.cwd();OUT=ROOT/'lab/results/v16-corrections-2026-10-19/batch-e';EVIDENCE={}
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def read(p):
 p=OUT/p;EVIDENCE[str(p.relative_to(ROOT))]=sha(p)
 return json.loads(gzip.decompress(p.read_bytes()) if p.suffix=='.gz' else p.read_bytes())
def text(p):
 p=OUT/p;EVIDENCE[str(p.relative_to(ROOT))]=sha(p);return p.read_text()
def same(inputs):
 assert inputs
 for p,h in inputs.items():assert sha(ROOT/p)==h,p
scope=read('scope.json');ids={f'V16-{n:03d}' for n in [1,2,3,4,83,84,85]}
expected_sources={
 '01_core/src/compiler/eval/bindings/field_access/content_methods.rs',
 '02_shell/src/cli.rs',
 '01_core/src/compiler/stdlib/structural/math/equation.rs',
 '01_core/src/entities/content/fields.rs',
 '01_core/src/entities/content/identity.rs',
 '01_core/src/compiler/stdlib/loading/encoding.rs',
 '01_core/src/compiler/eval/expression_dispatch.rs',
 '01_core/src/compiler/eval/selector_matching.rs',
 '01_core/src/compiler/introspect/from_tags.rs',
 '01_core/src/compiler/introspect/from_tags/equations.rs',
}
assert set(scope['ids'])==ids and len(scope['ids'])==len(ids)
assert set(scope['sources'])==expected_sources and len(scope['sources'])==10
expected_owners={p.replace('01_core/src/','00_nucleo/prompts/core/').replace('02_shell/src/','00_nucleo/prompts/shell/').removesuffix('.rs')+'.md' for p in expected_sources}
assert set(scope['owners'])==expected_owners and len(scope['owners'])==10
assert len(scope['shared_nuclei'])==1
state=read('validation/validation-state.json.gz');same(state['inputs'])
assert state['checkpoint']==subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
assert state['oracle_sha256']=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
assert state['linter_sha256']=='421d0797e7ae1a5102e9c5bcd87b488fea70a1997394c38765a03239fe579121'
gates=['python-ci-tests','layout','strict-lineage','reverse-lineage','build','tests-domain-infra','tests-wiring','doctests','build-all-targets','clippy','fmt','rustdoc']
jobs=read('validation/validation-commands.json');assert [r['name'] for r in jobs]==gates and all(r['exit']==0 for r in jobs)
expected_commands={
 'python-ci-tests':['python3','-m','unittest','discover','-s','lab/tools/ci'],
 'layout':['python3','lab/tools/ci/check_mature_layout.py'],
 'strict-lineage':[str(ROOT/'lab/target/linter/debug/crystalline-lint'),'.','--checks','v1,v5,v15,v26','--fail-on','warning'],
 'reverse-lineage':['python3','lab/tools/ci/check_reverse_lineage.py'],
 'build':['cargo','build','--offline','--locked','--workspace'],
 'tests-domain-infra':['cargo','test','--offline','--locked','--workspace','--exclude','typst-wiring','--lib','--bins','--tests','--no-fail-fast','--','--test-threads=1'],
 'tests-wiring':['cargo','test','--offline','--locked','-p','typst-wiring','--bins','--tests','--no-fail-fast','--','--test-threads=1'],
 'doctests':['cargo','test','--offline','--locked','--workspace','--doc','--no-fail-fast','--','--test-threads=1'],
 'build-all-targets':['cargo','build','--offline','--locked','--workspace','--all-targets'],
 'clippy':['cargo','clippy','--offline','--locked','--workspace','--all-targets','--','-D','warnings','-D','unfulfilled_lint_expectations'],
 'fmt':['cargo','fmt','--check','--all'],
 'rustdoc':['cargo','doc','--offline','--locked','--workspace','--no-deps'],
}
for row in jobs:
 assert row['command']==expected_commands[row['name']],row['name']
 for key,value in {'CARGO_BUILD_JOBS':'1','CARGO_PROFILE_DEV_DEBUG':'0','CARGO_PROFILE_TEST_DEBUG':'0','CARGO_INCREMENTAL':'0','TYPST_VANILLA':str(ROOT/'lab/typst-original/target/release/typst')}.items():
  assert row['environment'][key]==value,(row['name'],key)
 expected_overrides={'CRYSTALLINE_PDF_FIXED_EPOCH':'1792281600'} if row['name']=='tests-domain-infra' else {'RUSTDOCFLAGS':'-D warnings'} if row['name']=='rustdoc' else {}
 assert row['overrides']==expected_overrides,row['name']
 text('validation/'+row['name']+'.log')
by={r['name']:r for r in jobs};assert by['clippy']['command'][-4:]==['-D','warnings','-D','unfulfilled_lint_expectations']
assert by['rustdoc']['overrides']=={'RUSTDOCFLAGS':'-D warnings'}
assert by['strict-lineage']['command'][-4:]==['--checks','v1,v5,v15,v26','--fail-on','warning']
freeze=read('validation/validation-freeze.json');assert freeze['changed_inputs']==[] and freeze['completed_jobs']==freeze['expected_jobs']==12
assert freeze['input_count']==len(state['inputs'])==2132
outer=read('validation-run/commands.json');plan=read('validation-jobs.json')
assert [r['name'] for r in outer]==[r['name'] for r in plan] and all(r['exit']==r.get('expected_exit',0) for r in outer)
assert outer[0]['command']==['cargo','clean','-p','typst-core','-p','typst-shell','-p','typst-infra','-p','typst-wiring']
for row,planned in zip(outer,plan):
 for key in ['name','command','cwd','environment','expected_exit']:
  assert row.get(key,0 if key=='expected_exit' else None)==planned.get(key,0 if key=='expected_exit' else None),(row['name'],key)
 assert row['memory']['memory.max']==str(12*1024**3) and row['memory']['memory.high']==str(8*1024**3) and row['memory']['memory.swap.max']=='0'
 assert all(int(line.split()[1])==0 for line in row['memory']['memory.events'].splitlines() if line.split()[0] in ['oom','oom_kill','oom_group_kill'])
assert read('validation-run/freeze.json')['changed']==[];same(read('validation-run/state.json.gz')['inputs'])
assert read('baseline-run/freeze.json')['changed']==[]
baseline_jobs=read('baseline-run/commands.json');assert [r['exit'] for r in baseline_jobs]==[0,0,101,101]
assert '2 passed; 0 failed; 0 ignored;' in text('baseline-run/getter-preservation.log')
assert read('label-baseline-run/commands.json')[0]['exit']==0
assert read('label-baseline-run/freeze.json')['changed']==[]
assert '1 passed; 0 failed; 0 ignored;' in text('label-baseline-run/label-baseline.log')
# Every selected test is observed, not inferred from command success.
tests=read('new-tests.json');assert len(tests)==34 and len({t['name'] for t in tests})==34
final=text('validation/tests-domain-infra.log');classification=read('baseline-test-classification.json');observed=[]
followup_classification=read('followup-test-classification.json')
query_classification=read('query-test-classification.json')
all_classifications=classification['tests']+followup_classification['tests']+query_classification['tests']
assert len({r['name'] for r in all_classifications})==len(all_classifications)
assert {t['name'] for t in tests}<={r['name'] for r in all_classifications}
for record in [classification,followup_classification,query_classification]:
 assert Counter(r['classification'] for r in record['tests'])==record['counts']
for t in tests:
 pattern=r'^test [^\n]*::'+re.escape(t['name'])+r' \.\.\. (ok|FAILED|ignored)\s*$'
 actual=re.findall(pattern,final,re.M);assert actual==['ok'],(t,actual)
 row=next(r for r in all_classifications if r['name']==t['name'])
 assert row['file']==t['file']
 assert row['baseline_outcome']==('ok' if row['classification']=='preservation' else 'FAILED')
 assert re.findall(pattern,text(row['log']),re.M)==[row['baseline_outcome']]
 if 'log_sha256' in row:assert row['log_sha256']==sha(OUT/row['log'])
 if row['classification']=='test expectation defect':
  assert re.findall(pattern,text('label-baseline-run/label-baseline.log'),re.M)==['ok']
 observed.append({**t,'baseline':row['classification'],'final':'ok'})
assert classification['counts']=={'semantic RED':12,'preservation':13,'test expectation defect':1}
assert 'test result: FAILED' not in final
observed_counts=Counter(row['baseline'] for row in observed)
assert sum(observed_counts.values())==34 and set(observed_counts)=={'semantic RED','preservation','test expectation defect'}
assert observed_counts['test expectation defect']==1
semantic_red=observed_counts['semantic RED']
preservation=observed_counts['preservation']+observed_counts['test expectation defect']
followup_jobs=read('followup-red-run/commands.json')
followup_plan=read('followup-red-jobs.json')
assert len(followup_jobs)==len(followup_plan)
for actual,planned in zip(followup_jobs,followup_plan):
 assert actual['command']==planned['command'] and actual['name']==planned['name']
 assert actual['exit']==planned.get('expected_exit',0)
assert read('followup-red-run/freeze.json')['changed']==[]
query_jobs=read('query-red-run/commands.json');query_plan=read('query-red-jobs.json')
assert len(query_jobs)==len(query_plan)==2
for actual,planned in zip(query_jobs,query_plan):
 for key in ['name','command','cwd','environment','expected_exit']:
  assert actual.get(key,0 if key=='expected_exit' else None)==planned.get(key,0 if key=='expected_exit' else None)
 assert actual['exit']==planned.get('expected_exit',0)==101
assert read('query-red-run/freeze.json')['changed']==[]
query_red_log=text('query-red-run/query-red.log')
query_failed_names=re.findall(r'^test [^\n]*::([A-Za-z0-9_]+) \.\.\. FAILED\s*$',query_red_log,re.M)
assert set(query_failed_names)=={r['name'] for r in query_classification['tests'] if r['classification']=='semantic RED'}
assert len(query_failed_names)==query_classification['counts']['semantic RED']
assert 'error[E' not in query_red_log and 'could not compile' not in query_red_log
read('query-red-run/state.json.gz')
old_selector='equation_where_filters_match_identity_body_block_and_captured_fields'
old_pattern=r'^test [^\n]*::'+re.escape(old_selector)+r' \.\.\. (ok|FAILED|ignored)\s*$'
assert re.findall(old_pattern,text('query-red-run/existing-selector-red.log'),re.M)==['FAILED']
assert re.findall(old_pattern,final,re.M)==['ok']
# Preserve the failed full-suite evidence; it must never stand in for final gates.
failed_jobs=read('validation-selector-failure/validation-commands.json')
assert [r['name'] for r in failed_jobs]==gates[:6]
assert [r['exit'] for r in failed_jobs]==[0,0,0,0,0,101]
for row in failed_jobs:assert row['command']==expected_commands[row['name']]
failed_freeze=read('validation-selector-failure/validation-freeze.json')
assert failed_freeze['changed_inputs']==[] and failed_freeze['completed_jobs']==6 and failed_freeze['expected_jobs']==12
failed_state=read('validation-selector-failure/validation-state.json.gz')
assert failed_state['checkpoint']==state['checkpoint']
failed_log=text('validation-selector-failure/tests-domain-infra.log')
assert re.findall(old_pattern,failed_log,re.M)==['FAILED']
assert len(re.findall(r'^test [^\n]* \.\.\. FAILED\s*$',failed_log,re.M))==1
assert 'test result: FAILED. 7532 passed; 1 failed; 5 ignored;' in failed_log
assert read('validation-selector-failure-run/freeze.json')['changed']==[]
assert [r['exit'] for r in read('validation-selector-failure-run/commands.json')]==[0,1]
(OUT/'new-test-execution.json').write_text(json.dumps({'utc':now(),'tests':observed,'semantic_red':semantic_red,'preservation_including_corrected_yaml_control':preservation,'existing_selector_regression':'actual RED and final PASS; excluded from new-test count'},indent=2)+'\n')
# Independently reparse compiler JSON and match exact local primary spans.
struct=read('structural-query-green/results.json');baseline=read('structural-baseline/results.json')
assert [r['name'] for r in struct]==['core-extension','shell-omissions']
assert [(r['selected_e0004'],r['exit']) for r in baseline]==[(0,101),(0,0)]
assert [len(r['targets']) for r in struct]==[8,4]
assert read('structural-query-green/freeze.json')['changed']==[]
original=read('structural-query-green/original-sources.json.gz');witnesses=[]
structural_inputs=read('structural-query-green/inputs.json.gz')
for path,content in original.items():
 digest=hashlib.sha256(content.encode()).hexdigest()
 assert digest==structural_inputs['inputs'][path]==state['inputs'][path],path
for path,digest in structural_inputs['inputs'].items():
 if path in state['inputs']:assert digest==state['inputs'][path],path
for path,digest in structural_inputs['frozen_owners'].items():assert digest==state['inputs'][path],path
target_plan=read('structural-targets.json')
expected_witness_ids={t['id'] for t in target_plan if 'green' in t}
assert len(expected_witness_ids)==12
for rec in struct:
 assert rec['exit']==101 and not rec['other_errors'] and rec['selected_e0004']==len(rec['targets'])
 changes=read('structural-query-green/'+rec['name']+'-mutations.json.gz');errors=[];build_finished=[]
 planned_targets=[t for t in target_plan if t['group']==rec['name'] and 'green' in t]
 assert [t['id'] for t in rec['targets']]==[t['id'] for t in planned_targets]
 expected_changes={}
 if rec['name']=='core-extension':
  path='01_core/src/entities/content.rs';content=original[path]
  pos=content.index('\n}',content.index('pub enum Content {'))
  expected_changes[path]=content[:pos]+'\n    TektV16EvolutionProbe,'+content[pos:]
 else:
  for target in planned_targets:
   path=target['file'];content=expected_changes.get(path,original[path])
   assert content.count(target['remove_exact'])==1,target['id']
   expected_changes[path]=content.replace(target['remove_exact'],'')
 assert changes==expected_changes,(rec['name'],'mutations differ from the exact planned witness')
 for actual,planned in zip(rec['targets'],planned_targets):
  for key,value in planned.items():assert actual[key]==value,(actual['id'],key)
 for line in text('structural-query-green/'+rec['name']+'.log').splitlines():
  try:item=json.loads(line)
  except ValueError:continue
  if item.get('reason')=='compiler-message' and item['message']['level']=='error':errors.append(item['message'])
  if item.get('reason')=='build-finished':build_finished.append(item.get('success'))
 assert build_finished==[False],(rec['name'],build_finished)
 assert all((e.get('code') or {}).get('code')=='E0004' for e in errors)
 for t in rec['targets']:
  content=changes.get(t['file'],original[t['file']]);spec=t['green'];pos=content.index(spec['anchor'],content.index(spec['after']));line=content[:pos].count('\n')+1
  assert line==t['line'] and len(content[:pos].encode())==t['byte_start']
  assert t['byte_end']==len(content[:pos+len(spec['anchor'])].encode())
  matched=[e for e in errors if any(s['is_primary'] and s['file_name'].removeprefix(rec['cwd']+'/')==t['file'] and s['line_start']==line and t['byte_start']<=s['byte_start']<t['byte_end'] for s in e['spans'])]
  assert len(matched)==1 and matched==t['diagnostics'],t
  if rec['name']=='core-extension':assert all('TektV16EvolutionProbe' in e['message'] for e in matched)
  witnesses.append({'id':t['id'],'file':t['file'],'line':line,'method':rec['name'],'independent_primary_span_verified':True})
assert ids<={w['id'] for w in witnesses}
assert {w['id'] for w in witnesses}==expected_witness_ids and len(witnesses)==len(expected_witness_ids)
(OUT/'structural-witness-review.json').write_text(json.dumps({'utc':now(),'witnesses':witnesses,'limits':'Core uses real Content enum extension. Shell uses positive-arm omission with real dependencies; it does not claim a successfully compiled cross-crate future enum. Additional helper witnesses are green-only; no baseline outcome is asserted for them.'},indent=2)+'\n')
# Scan comparator also verifies retained error movement and outside-scope notes.
lint=read('lint-delta.json');assert lint['remaining_v16']==37 and lint['new_errors_or_warnings']==0 and lint['outside_scope_exact'] and set(lint['closed_ids'])==ids
same(read('lint-state.json.gz')['inputs']);after=read('after.sarif.gz')['runs'][0]['results'];assert sum(r['ruleId']=='V16' for r in after)==37
assert all(r['ruleId']=='V16' for r in after if r['level'] in ['error','warning'])
replay=read('agents/language/accepted-replay/receipt.json');assert not replay['passed'] and not replay['changed_build_inputs']
assessment=read('agents/language/accepted-assessment.json')
replay_path=OUT/'agents/language/accepted-replay/receipt.json'
assessor_path=OUT/'agents/language/assess.py'
review_path=Path(assessment['review'])
if not review_path.is_absolute():review_path=ROOT/review_path
review_path=review_path.resolve()
review_path.relative_to(ROOT)
assert review_path.is_file() and assessment['review_sha256']==sha(review_path)
assert assessment['receipt_sha256']==sha(replay_path)
assert assessment['candidate_binary_sha256']==replay['binaries']['product']['sha256']
assert assessment['assessor_sha256']==sha(assessor_path)
EVIDENCE[str(assessor_path.relative_to(ROOT))]=sha(assessor_path)
EVIDENCE[str(review_path.relative_to(ROOT))]=sha(review_path)
independent_path=OUT/('closure-language-assessment-'+datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ')+'.json')
assessor_command=[sys.executable,str(assessor_path),'--receipt',str(replay_path),'--review',str(review_path),'--output',str(independent_path)]
subprocess.run(assessor_command,cwd=ROOT,check=True)
independent=read(independent_path.relative_to(OUT))
assert independent['scoped_acceptance'] and not independent['full_parity']
for key in ['receipt_sha256','review_sha256','assessor_sha256','candidate_binary_sha256','counts','cases','required_corrections','errors','scoped_acceptance','full_parity']:
 assert assessment[key]==independent[key],key
assert replay['validation_state']['path']==str(OUT/'validation/validation-state.json.gz')
assert replay['validation_state']['sha256']==sha(OUT/'validation/validation-state.json.gz')
assert replay['binaries']['product']['sha256']==sha(ROOT/'lab/target/product/debug/typst')
assert len(replay['cases'])==sum(r['case_count'] for r in read('agents/language/baseline-manifest.json')['receipts'])==173
assert all(c['oracle_preserved'] and not c['private_marker_leak'] for c in replay['cases'])
# The realized query corpus has strict all-case acceptance, without partial exceptions.
def canonical_json(value):
 return json.dumps(value,sort_keys=True,ensure_ascii=False,separators=(',',':'),allow_nan=False)
def language_equal(left,right):
 if isinstance(left,bool) or isinstance(right,bool):return type(left) is type(right) and left==right
 if isinstance(left,(int,float)) and isinstance(right,(int,float)):return left==right
 if isinstance(left,dict) and isinstance(right,dict):
  return left.keys()==right.keys() and all(language_equal(left[key],right[key]) for key in left)
 if isinstance(left,list) and isinstance(right,list):
  return len(left)==len(right) and all(language_equal(a,b) for a,b in zip(left,right))
 return type(left) is type(right) and left==right
assert not language_equal({'value':True},{'value':1})
assert language_equal({'value':[1,2.0]},{'value':[1.0,2]})
def raw_observation(result,case):
 assert not result.get('timeout')
 def normalize(value):
  if case.get('command')=='query':
   source_path=Path(result['argv'][2])
   for spelling in [str(source_path),os.path.relpath(source_path,ROOT)]:value=value.replace(spelling,'<probe-document>')
  return value
 stdout,stderr=normalize(result['stdout']),normalize(result['stderr'])
 if result['returncode']!=0:return {'returncode':result['returncode'],'stdout':stdout,'stderr':stderr}
 value=(yaml.safe_load if case.get('format')=='yaml' else json.loads)(stdout)
 return {'returncode':0,'value':value,'stderr':stderr}
# Independently audit original raw classification with booleans distinct from numbers.
# Preserve historical receipt labels; discrepancies block closure rather than rewriting them.
frozen_legacy={}
for item in read('agents/language/baseline-manifest.json')['receipts']:
 assert sha(OUT/'agents/language'/item['file'])==item['sha256']
 for case in read('agents/language/'+item['file'])['cases']:
  key=item['file']+'::'+case['name'];assert key not in frozen_legacy
  frozen_legacy[key]=case
assert len(frozen_legacy)==len(replay['cases'])==173
legacy_counts=Counter()
for case in replay['cases']:
 original=frozen_legacy[case['id']]
 computed={
  'baseline_product':raw_observation(original['results']['product'],original),
  'baseline_oracle':raw_observation(original['results']['oracle'],original),
  'candidate_product':raw_observation(case['results']['product'],case),
  'fresh_oracle':raw_observation(case['results']['oracle'],case),
 }
 assert canonical_json(case['observations'])==canonical_json(computed),case['id']
 assert language_equal(computed['fresh_oracle'],computed['baseline_oracle']),case['id']
 new,old,oracle=computed['candidate_product'],computed['baseline_product'],computed['fresh_oracle']
 classification='PRESERVED' if language_equal(new,old) else 'REPAIRED' if language_equal(new,oracle) else 'REGRESSION'
 assert classification==case['classification'],(case['id'],'boolean/numeric classification discrepancy')
 legacy_counts[classification]+=1
 assessed=next(row for row in independent['cases'] if row['id']==case['id'])
 assert assessed['matches_oracle']==language_equal(new,oracle),case['id']
assert replay['counts']=={key:legacy_counts[key] for key in ['PRESERVED','REPAIRED','REGRESSION']}
def query_observation(result):
 assert not result.get('timeout') and result['returncode']==0,result
 assert result['cwd']==str(ROOT)
 assert result['argv'][1]=='query'
 source_path=Path(result['argv'][2])
 assert source_path.is_absolute()
 def normalize(value):
  for spelling in [str(source_path),os.path.relpath(source_path,ROOT)]:value=value.replace(spelling,'<probe-document>')
  return value
 def reject_constant(value):raise ValueError('non-JSON numeric constant: '+value)
 value=json.loads(normalize(result['stdout']),parse_constant=reject_constant)
 return {'returncode':0,'value':value,'stderr':normalize(result['stderr'])}
query_root=OUT/'agents/language'
query_manifest=read('agents/language/realized-query-manifest.json')
assert query_manifest['cases']==39 and len(query_manifest['receipts'])==3
assert [entry['file'] for entry in query_manifest['receipts']]==['realized-query/receipt.json','realized-query-controls/receipt.json','realized-query-defaults/receipt.json']
expected_query_cases=[]
for manifest_entry in query_manifest['receipts']:
 baseline_path=query_root/manifest_entry['file']
 assert sha(baseline_path)==manifest_entry['sha256']
 baseline_receipt=read('agents/language/'+manifest_entry['file'])
 assert len(baseline_receipt['cases'])==manifest_entry['cases']
 assert baseline_receipt['binaries']['product']['sha256']==manifest_entry['product_sha256']
 assert baseline_receipt['binaries']['oracle']['sha256']==state['oracle_sha256']
 for case in baseline_receipt['cases']:
  assert case['source_sha256']==hashlib.sha256(case['source'].encode()).hexdigest()
  expected_query_cases.append((manifest_entry['file']+'::'+case['name'],manifest_entry['file'],case))
assert len(expected_query_cases)==len({case_id for case_id,_,_ in expected_query_cases})==39
query_replay=read('agents/language/accepted-realized-query-replay/receipt.json')
query_runner=query_root/'realized-query-replay.py'
assert query_replay.get('utc_end')
assert query_replay['runner_sha256']==sha(query_runner)
EVIDENCE[str(query_runner.relative_to(ROOT))]=sha(query_runner)
assert query_replay['baseline_manifest']=={'path':str(query_root/'realized-query-manifest.json'),'sha256':sha(query_root/'realized-query-manifest.json')}
assert query_replay['validation_state']['path']==str(OUT/'validation/validation-state.json.gz')
assert query_replay['validation_state']['sha256']==sha(OUT/'validation/validation-state.json.gz')==query_replay['validation_state_sha256_after']
assert query_replay['validation_state']['receipt']==state
assert query_replay['head']==query_replay['head_after']==state['checkpoint']
assert not query_replay['changed_build_inputs']
assert query_replay['oracle_pin']['commit']=='bd5ebec285505889e8ff1d4f68804ab147a699f3'
assert query_replay['oracle_pin']['tree']=='cac836c07e4dec2a89219370428292393a3216b5'
assert query_replay['oracle_pin']['recipe']=='lab/results/construction/diagnostics/vanilla-resync-2026-10-17/relatorio.md'
for label,path in [('product',ROOT/'lab/target/product/debug/typst'),('oracle',ROOT/'lab/typst-original/target/release/typst')]:
 assert query_replay['binaries'][label]=={'path':str(path),'sha256':sha(path)}
 assert query_replay['binary_sha256_after'][label]==sha(path)==replay['binaries'][label]['sha256']
actual_query_cases=query_replay['cases']
assert [case['id'] for case in actual_query_cases]==[case_id for case_id,_,_ in expected_query_cases]
query_counts=Counter()
for index,(actual,(case_id,receipt_name,baseline_case)) in enumerate(zip(actual_query_cases,expected_query_cases)):
 assert actual['baseline_receipt']==receipt_name and actual['command']=='query' and actual['format']=='json'
 for key,value in baseline_case.items():
  if key not in ['results','parsed_equal','utc_start','head','diff_head_stat']:
   assert actual[key]==value,(case_id,key)
 source_path=query_root/'accepted-realized-query-replay/sources'/f'{index:03d}-{baseline_case["name"]}.typ'
 assert source_path.read_text()==baseline_case['source']
 assert sha(source_path)==actual['source_sha256']==baseline_case['source_sha256']
 EVIDENCE[str(source_path.relative_to(ROOT))]=sha(source_path)
 for label in ['product','oracle']:
  expected_argv=list(baseline_case['results'][label]['argv'])
  assert expected_argv[1]=='query'
  flag_shapes=[['--format','json']]
  for field in ['block','body','numbering','alt']:
   flag_shapes.extend([['--format','json','--field',field],['--field',field,'--format','json']])
  assert expected_argv[4:] in flag_shapes
  expected_argv[0]=query_replay['binaries'][label]['path'];expected_argv[2]=str(source_path)
  assert actual['results'][label]['argv']==expected_argv,(case_id,label)
 observations={'baseline_product':query_observation(baseline_case['results']['product']),
               'baseline_oracle':query_observation(baseline_case['results']['oracle']),
               'candidate_product':query_observation(actual['results']['product']),
               'fresh_oracle':query_observation(actual['results']['oracle'])}
 assert canonical_json(actual['observations'])==canonical_json(observations),case_id
 assert language_equal(observations['candidate_product'],observations['fresh_oracle']) and language_equal(observations['fresh_oracle'],observations['baseline_oracle']),case_id
 expected_classification='PRESERVED' if language_equal(observations['candidate_product'],observations['baseline_product']) else 'REPAIRED'
 assert actual['classification']==expected_classification
 query_counts[expected_classification]+=1
 assert actual['candidate_observation_sha256']==hashlib.sha256(canonical_json(observations['candidate_product']).encode()).hexdigest()
 assert 'equation.constructor' not in actual['results']['product']['stdout']+actual['results']['product']['stderr']
 assert actual['pass'] is True and actual['matches_oracle'] is True and actual['oracle_preserved'] is True and actual['private_marker_leak'] is False
assert query_replay['counts']=={key:query_counts[key] for key in ['PRESERVED','REPAIRED','REGRESSION']}
assert query_replay['passed'] is True and sum(query_counts.values())==39 and query_counts['REGRESSION']==0
read('open-obligations.json')
closure={'utc':now(),'baseline_commit':state['checkpoint'],'branch':subprocess.check_output(['git','branch','--show-current'],text=True).strip(),'validated_input_count':len(state['inputs']),'full_linter_exit':1,'remaining_v16':37,'selected_v16_closed':7,'owner_pairs':len(expected_sources),'new_tests':len(tests),'semantic_red':semantic_red,'preservation':preservation,'structural_witnesses':len(witnesses),'language_cases':len(replay['cases']),'language_classifications':replay['counts'],'realized_query_cases':len(actual_query_cases),'realized_query_classifications':query_replay['counts'],'sources':{p:sha(ROOT/p) for p in scope['sources']+scope['owners']+scope['shared_nuclei']},'closed_ids':scope['ids'],'validation_completed':freeze['utc'],'evidence':EVIDENCE}
(OUT/'closure-ledger.json').write_text(json.dumps(closure,indent=2)+'\n')
print(json.dumps({k:v for k,v in closure.items() if k not in ['sources','evidence']},indent=2))
