"""Bind closure to completed frozen validation, never to a requested plan."""
from pathlib import Path
import datetime, gzip, hashlib, json, re, subprocess
root=Path.cwd(); out=root/'lab/results/v16-corrections-2026-10-19/batch-c'
def read(name): return json.loads((out/name).read_text())
def gz(name): return json.loads(gzip.decompress((out/name).read_bytes()))
def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()
def save(name,data): (out/name).write_text(json.dumps(data,indent=2)+'\n')
now=datetime.datetime.now(datetime.timezone.utc).isoformat()
validation=read('validation/validation-commands.json')
freeze=read('validation/validation-freeze.json')
assert len(validation)==12 and all(r['exit']==0 for r in validation)
assert not freeze['changed_inputs'] and freeze['completed_jobs']==freeze['expected_jobs']==12
for path in ['candidate-final/freeze.json','structural-green-final/freeze.json','validation-run/freeze.json']:
 assert not read(path)['changed'],path
state=gz('validation/validation-state.json.gz')
assert all(sha(root/p)==digest for p,digest in state['inputs'].items())
lint=read('lint-delta.json'); assert lint['remaining_v16']==64 and lint['new_errors_or_warnings']==0 and lint['outside_scope_exact']
assert all(sha(root/p)==digest for p,digest in gz('lint-state.json.gz')['inputs'].items())
scope=read('scope.json'); experiments=read('structural-green-final/results.json')
assert len(experiments)==7 and all(r['exit']==101 and not r['other_errors'] and r['selected_e0004']==len(r['targets']) for r in experiments)
assert len(experiments[0]['targets'])==25
proof=gz('structural-green-final/inputs.json.gz')
assert all(sha(root/p)==digest for p,digest in proof['frozen_owners'].items())
selector=read('agents/selectors/accepted-replay.json');assert selector['pass'] and len(selector['cases'])==60 and not selector['changed_sources']
effects=read('agents/effects/accepted-replay/receipt.json');assert effects['source_unchanged'] and effects['product_binary_unchanged'] and effects['oracle_binary_unchanged']
local_cases=[r for r in effects['results'] if r['scope']=='local-acceptance']
debts=[r for r in effects['results'] if r['scope']=='open-producer-dependency']
assert len(local_cases)==23
assert {r['name'] for r in debts}=={'figure-supplement','equation-supplement'}
for case in local_cases+debts:
 assert set(case['runs'])=={'oracle','product'}
 for who,run in case['runs'].items():
  observed=run['observables']
  assert run.get('exit')==1 and not run.get('timeout')
  assert re.findall(r'^error: (.+)$',run['stderr'],re.M)==['cannot return outside of function']
  assert observed['invalid_return'] and observed['discarded_warning'] and not observed['panic_error']
  expected=case['expected_update_hint'] if case['scope']=='local-acceptance' or who=='oracle' else False
  assert observed['update_hint']==expected,(case['name'],who)
rich=[r for r in effects['results'] if r['scope']=='real-query-producer']
assert len(rich)==1 and set(rich[0]['runs'])=={'oracle','product'}
for run in rich[0]['runs'].values():
 assert run.get('exit')==0
 routes=run['rich_body_routes']
 assert [r['route'] for r in routes]==['markup','code-tail','math']
 assert all(r['has_heading'] and r['has_strong'] and r['has_text'] for r in routes)
outer=read('validation-run/commands.json');assert len(outer)==4 and all(r['exit']==0 for r in outer)
log=out/'validation/tests-domain-infra.log';text=log.read_text();tests=read('new-tests.json')
assert len(tests)==52 and all(re.search(r'::'+re.escape(t['name'])+r' \.\.\. ok$',text,re.M) for t in tests)
assert '/repos/Antigravity/typst-crystalline/01_core' in text
assert '/repos/Antigravity/typst-crystalline/03_infra' in text
assert not re.search(r'\+.*#\[(?:allow|expect|ignore)',subprocess.check_output(['git','diff','HEAD','--','01_core'],text=True),re.M)
heading_control=bool(re.search(r'::a1_heading_constructor_identity_presence_and_external_styled \.\.\. ok$',text,re.M))
assert heading_control,'Existing public Heading reflection control must pass'
save('new-test-execution.json',{'utc':now,'log':'validation/tests-domain-infra.log','log_sha256':sha(log),'new_tests_observed_passing':tests,'existing_heading_control_passed':heading_control})
closed=[]
for id in scope['ids']:
 target=next(t for t in experiments[0]['targets'] if t['id']==id)
 source=target['file'];body=(root/source).read_text();owner=re.search(r'^//! @prompt (.+)$',body,re.M)[1];assert owner in scope['owners']
 closed.append({'id':id,'source':source,'owner':owner,'source_sha256':sha(root/source),'owner_sha256':sha(root/owner),'prompt_seal':re.search(r'^//! @prompt-hash (\w+)$',body,re.M)[1],'exact_structural_target':target,'status':'closed after semantic/preservation controls, actual enum evolution, full lint delta and integrated gates; broader obligations remain open'})
save('closure-ledger.json',{'utc':now,'checkpoint':state['checkpoint'],'branch':subprocess.check_output(['git','branch','--show-current'],text=True).strip(),'measurement_state':'uncommitted working tree; exact diffstat/status/full inputs in validation-state.json.gz','closed':closed,'source_owner_pairs':len(scope['sources']),'shared_nuclei':[{'file':p,'raw_sha256':sha(root/p)} for p in scope['shared_nuclei']],'remaining_v16':64,'new_tests':len(tests),'integrated_gates':12,'frozen_inputs':freeze['input_count'],'completed_at':validation[-1]['utc_end'],'final_bilateral_cases':{'selector':60,'warning':23,'rich_query_producer':1,'rich_insertion_routes':3,'producer_dependencies_observed_open':2},'open_obligations':'open-obligations.json'})
print(json.dumps({'closed':len(closed),'tests':len(tests),'gates':len(validation),'frozen_inputs':freeze['input_count'],'completed_at':validation[-1]['utc_end']},indent=2))
