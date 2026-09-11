"""Publish batch-one CLI policy corrections without executing products."""
import copy
import hashlib
import json
from pathlib import Path
import subprocess

D = Path(__file__).resolve().parent
def read(n): return json.loads((D/n).read_text())
def sha(p): return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def pin(n): return {'path': str(D/n), 'sha256': sha(D/n)}
def write(n,d):
 p=D/n; assert not p.exists()
 s=json.dumps(d,ensure_ascii=True,indent=2)+'\n'
 patch='*** Begin Patch\n*** Add File: '+str(p)+'\n'+'\n'.join('+'+x for x in s.split('\n')[:-1])+'\n*** End Patch\n'
 subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True)
 print(json.dumps(pin(n)))

plan=read('p1339-ab-cli-plan-r4.json')
oracles=read('p1339-ab-cli-oracles-r1.json')
plan['batch']=1
plan['status']='COMPONENT_OF_BATCH_1_NOT_EXECUTION_AUTHORITY'
for name in ['p1339-budget-redesign-r1.json','p1339-verifier-budget-redesign-acceptance-r1.md','p1339-verifier-opacity-design-review-r1.md','p1339-verifier-cli-plan-r2-findings.md']:
 plan['input_sha256'][str(D/name)]=sha(D/name)
oracles['predecessor']=pin('p1339-ab-cli-oracles-r1.json')
oracles['batch']=1
oracles['status']='BATCH_1_COMPONENT; causal-control observations remain to be measured in the aggregated focal'

direct={c['id'] for c in plan['cases'] if c['id'].endswith('-direct-control')}
for c in plan['cases']+oracles['oracles']:
 if c['id'] in direct:
  c['reference_policy']='baseline'
  c['show_effect_by_product']={'candidate':'OtherDiagnosticBeforeCallback' if c['id']=='show-text-direct-control' else 'CallbackExecuted', 'baseline':'OtherDiagnosticBeforeCallback' if c['id']=='show-text-direct-control' else 'CallbackExecuted', 'vanilla':'CallbackExecuted'}
  c['show_effect']=c['show_effect_by_product']['candidate']
  if 'expected_observation_or_reference_with_sha256' in c:
   c['expected_observation_or_reference_with_sha256']['candidate_product']='baseline'
   c['expected_observation_or_reference_with_sha256']['mutation_host_product']='vanilla'

original=next(c for c in plan['cases'] if c['id']=='where-l0-empty-vs-bare')
source=original['expression']; needle='strong.where()'
assert source.count(needle)==1
calibration=source.replace(needle,'none'+' '*(len(needle)-4),1)
assert len(source.encode())==len(calibration.encode())
assert source.index('selector(strong)')==calibration.index('selector(strong)')
composed={
 'kind':'composed_diagnostic_from_declared_calibration',
 'original_expression':source,'calibration_expression':calibration,
 'calibration_case_id':'causal-empty-vs-bare-padded-legacy-control',
 'expected_baseline_calibration':{'execution':'Observed','exit':1,'stdout':'','primary_failure':'legacy bare selector(strong), not where construction',
   'diagnostic_primary_span_within_utf8_byte_range':[source.index('selector(strong)'),source.index('selector(strong)')+len('selector(strong)')]},
 'candidate_expected':{'execution':'Observed','exit':1,'stdout':'',
   'stderr_derivation':{'from_product':'baseline','from_case':'causal-empty-vs-bare-padded-legacy-control','same_profile':True,
      'operation':'replace_exactly_one_literal','old':'1 │ '+calibration,'new':'1 │ '+source,
      'all_other_bytes':'unchanged; compare entire actual candidate stderr, do not normalize actual output'}},
 'mutation_host_expected':{'product':'vanilla','reference_case':'where-l0-empty-vs-bare','source':'unchanged historical/focal vanilla full tuple'},
 'conjunction_controls':['causal-bare-strong-isolated','causal-strong-empty-isolated','causal-heading-empty-vs-bare'],
 'causal_limit':'New diagnostic calibration source, not samegraph bridge or repaired baseline claim. Later bare call and all its byte offsets are identical. Separate positive confirms allowed where constructor; legacy bare remains unavailable.',
 'binding_before_seal':'Pin actual aggregated-focal baseline calibration receipt and row indices before canonical seal; no expected text or span adapted to a candidate.',
}
for c in plan['cases']+oracles['oracles']:
 if c['id']=='where-l0-empty-vs-bare':
  c['reference_policy']='composed_causal'
  c['composed_predicate']=copy.deepcopy(composed)
  if 'expected_observation_or_reference_with_sha256' in c:
   c['expected_observation_or_reference_with_sha256']['candidate_product']='composed_causal'

def newcase(id,expr,policy):
 return {'id':id,'expression':expr,'mode':'eval','adapter':'eval_exact','reference_policy':policy,
  'obligation_ids':['W01'],'mandatory':True,'reference':{'kind':'aggregate_batch1_focal_before_seal'},'profiles':['default','html','a11y','html+a11y']}
plan['cases'].extend([
 newcase('causal-empty-vs-bare-padded-legacy-control',calibration,'baseline'),
 newcase('causal-bare-strong-isolated','selector(strong)','baseline'),
 newcase('causal-strong-empty-isolated','repr(strong.where())','vanilla'),
 newcase('causal-heading-empty-vs-bare','repr(heading.where()==selector(heading))','vanilla'),
])

opaque=next(c for c in plan['cases'] if c['id']=='opaque-budget-workload')
opaque.pop('expected_execution',None)
opaque['reference_policy']='executable_specific_control'
opaque['opacity']='Observation-budget opacity only. No workload parity or loop-guard repair; zero positive credit.'
opaque['expected_by_product']={
 'vanilla':{'binary_sha256':'7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8','execution':'Unknown','unknown_reason':'timeout','exit':None,'timeout_seconds':1,'stdout':'','stderr':''},
 'baseline':{'binary_sha256':'f7c8085f8453ecb6b10841b698092d41e7fbec64d9d46f9341b9b9b538bfefa1','execution':'Observed','reference':pin('p1339-ab-opaque-focal-r2-runs.json'),'case_id':'opaque-budget-workload','same_profile':True,'fields':['exit','stdout','stderr'],'timeout_seconds':1},
 'candidate':{'execution':'Observed','reference_product':'baseline','reference':pin('p1339-ab-opaque-focal-r2-runs.json'),'case_id':'opaque-budget-workload','same_profile':True,'fields':['exit','stdout','stderr'],'timeout_seconds':1},
}
opaque['classification_credit']={'positive':0,'opaque_vanilla_only':True,'baseline_known_error_is_not_unknown':True,'required_control':True}
opaque['determinism']='Same executable-specific cells in normal/repeat/reverse; any drift blocks the control. Do not tune deadline after observing candidate.'
finite=next(c for c in plan['cases'] if c['id']=='opaque-budget-finite')
finite['reference']={'kind':'pinned_raw','artifact':pin('p1339-ab-opaque-focal-r2-runs.json'),'case_id':'opaque-budget-finite','fields':['exit','stdout','stderr'],'same_product_and_profile':True}

plan['batch1_focal_ids']=sorted(direct|{'where-l0-empty-vs-bare','causal-empty-vs-bare-padded-legacy-control','causal-bare-strong-isolated','causal-strong-empty-isolated','causal-heading-empty-vs-bare','opaque-budget-workload','opaque-budget-finite','show-strong-static-match','show-strong-static-miss','show-emph-static-match','show-emph-static-miss','show-text-static-match','show-text-static-miss'})
assert set(plan['batch1_focal_ids']) <= {c['id'] for c in plan['cases']}
oracles['batch1_new_case_policies']=[copy.deepcopy(c) for c in plan['cases'] if c['id'].startswith('causal-') or c['id'].startswith('opaque-budget-')]
oracles['causal_mapping_review_required_before_focal']=True
oracles['no_new_execution']='Only metadata/oracle publication; no semantic process has run under this successor.'
write('p1339-ab-batch1-cli-plan.json',plan)
oracles['plan']=pin('p1339-ab-batch1-cli-plan.json')
write('p1339-ab-batch1-cli-oracles.json',oracles)
