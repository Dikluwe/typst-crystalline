"""Publish canonical independent definitions before aggregate focal calibration.
No product execution. Pending raw receipts remain explicit and cannot pass a gate.
"""
import copy, datetime, hashlib, json, subprocess
from pathlib import Path
D=Path(__file__).resolve().parent
PROFILES=['default','html','a11y','html+a11y'];ORDERS=['normal','repeat','reverse']
def pin(n):
 p=D/n
 return {'path':str(p),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
def load(n):return json.loads((D/n).read_text())
def publish(n,a):
 p=D/n
 if p.exists():raise SystemExit('immutable output exists '+str(p))
 body=json.dumps(a,indent=2,ensure_ascii=True)+'\n'
 patch='*** Begin Patch\n*** Add File: '+str(p)+'\n'+''.join('+'+s+'\n' for s in body.split('\n')[:-1])+'*** End Patch\n'
 subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True)
 print(n,pin(n)['sha256'])
contract=pin('p1339-contract-r3.json');manifest=pin('p1339-authority-manifest-r2.json')
plan=load('p1339-ab-batch1-cli-plan.json');defs=load('p1339-ab-batch1-cli-oracles.json')
plan_pin=pin('p1339-ab-batch1-cli-plan.json');defs_pin=pin('p1339-ab-batch1-cli-oracles.json')
by_plan={c['id']:c for c in plan['cases']}
common={'schema':'p1339-oracle-v2','author':'/root/p1319_tests','regime':'executado sem atestacao de isolamento',
 'published_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
 'status':'DEFINITIONS_FROZEN_PENDING_AGGREGATE_BATCH1_MEASUREMENT_NOT_SEALED',
 'contract':contract,'authority_manifest':manifest,'l0_freeze':pin('p1339-l0-freeze.json'),
 'budget_redesign':pin('p1339-budget-redesign-r1.json'),'budget_acceptance':pin('p1339-verifier-budget-redesign-acceptance-r1.md'),
 'candidate_access':'No candidate exists or has been read. No productive source or implementation tests/diff bodies read. Inherited P1319 context disclosed; no technical isolation attestation.',
 'read_boundary':'L0, pinned contract/public baseline/vanilla receipts and nominal mechanical declarations/signatures/harness artifacts only; materialization access limited to exact authorized typst-passo-1339.md.',
 'unknown_policy':'Raw Unknown remains Unknown. Every due mandatory Unknown blocks, except deliberate declared opacity and conditional Angle applicability. NotDue is scheduling metadata only; final mandatory entries cannot be NotDue.',
 'publication_immutability':'Do not rewrite definitions to fill measured values. Planned raw and recipe-derived expectation receipts are new immutable outputs pinned by later verifier seal. Recipe failure blocks and must not trigger silent normalization/adaptation.',
 'historical_budget':pin('p1339-ab-publication-budget-ledger-r1.json'),
 'current_phase_counts':{'aggregate_focal_batches_executed':0,'full_preseal_matrices_executed':0,'canonical_RED_executed':0,'candidate_executions':0,'future_internal_executions':0},
 'classification':'Exact dimensions only, no PDF-byte comparison, no diagnostic normalization; source paths/spans/trace are observable. Closed relations use real discriminant, never public boolfalse as Unproven.',
 'no_claims':['No seal or verifier verdict','No positive credit for opaque or future definitions','No global parity claim','No implementation permission from these author artifacts']}

def complete_public(c):
 c=copy.deepcopy(c)
 c['phase_class']=c.get('phase',{}).get('kind','existing_public')
 c['freeze_phase']='C';c['execution_phases']=['C','D_when_baseline_RED','F']
 c['binding_contract_if_needed']=None
 c['definition_ancestry']=defs_pin
 if c['reference_policy']=='composed_causal':c['reference_policy']='causal_successor'
 return c
public=[complete_public(c) for c in defs['oracles']]
pending={'status':'PENDING_MEASUREMENT','path':str(D/'p1339-ab-batch1-focal-runs.json'),'sha256':None,'credit':0,
 'selection':'Unique rows by id/product/profile/order, all required cells observed and controls stable in normal/repeat/reverse; actual SHA must be pinned by seal.'}
recipe={'artifact':pin('p1339-ab-batch1-causal-recipe.py'),'callable':'derive(raw)',
 'command':['python3',str(D/'p1339-ab-batch1-causal-recipe.py'),'--raw',pending['path'],'--output',str(D/'p1339-ab-batch1-causal-derived-expectations.json')],
 'raw':pending,'derived_receipt':{'status':'PENDING_MEASUREMENT','path':str(D/'p1339-ab-batch1-causal-derived-expectations.json'),'sha256':None,'credit':0},
 'fail_closed':'Exact binary/plan hashes, source byte length and preserved bare-call offset; exactly one complete source line; all remaining bytes identical. No adjustment if diagnostic/spans/context differ or isolated controls fail.',
 'conjunctive_controls':['causal-bare-strong-isolated','causal-strong-empty-isolated','causal-heading-empty-vs-bare']}
next(c for c in public if c['id']=='where-l0-empty-vs-bare')['pending_causal_recipe']=recipe
for c in defs['batch1_new_case_policies']:
 if c['role']=='opaque' if 'role' in c else False:continue
 src=c['expression'];n={'id':c['id'],'contract_sha256':contract['sha256'],'authority_manifest_sha256':manifest['sha256'],
 'obligation_ids':c['obligation_ids'],'mandatory':True,'role':c.get('role','preservation'),
 'source_ref_or_literal_with_sha256':{'literal':src,'sha256':hashlib.sha256(src.encode()).hexdigest()},
 'adapter':c['adapter'],'profiles':PROFILES,'orders':ORDERS,'reference_policy':c['reference_policy'],
 'expected_observation_or_reference_with_sha256':{'definition':defs_pin,'selector':'batch1_new_case_policies[id='+c['id']+']',
    'candidate_reference_product':c['reference_policy'],'baseline_calibration_reference_product':'baseline','vanilla_calibration_reference_product':'vanilla',
    'dimensions':['exit','stdout','stderr'],'planned_raw':pending if c['id'].startswith('causal-') else None},
 'raw_input_provenance':{'plan':plan_pin,'case_id':c['id'],'previous_reference':c.get('reference')},
 'phase':{'kind':'existing_public','freeze':'C','execute':['C','D_when_baseline_RED','F']},
 'invocation':{'mode':'eval','cwd':str(D.parent.parent),'timeout_seconds':c.get('timeout_seconds',30)}}
 if c['id'].startswith('causal-'):n['pending_measurement_recipe']=recipe
 public.append(complete_public(n))

def future_entries(name, ids=None, opaque=False):
 data=load(name);cases=data.get('cases',[data]);out=[]
 for c in cases:
  if ids is not None and c['id'] not in ids:continue
  ph={'freeze':'C','execute':['F'],'preseal_state':'NOT_EXECUTED_PRESEAL','credit':0}
  out.append({'id':c['id'],'contract_sha256':contract['sha256'],'authority_manifest_sha256':manifest['sha256'],
   'obligation_ids':c.get('obligation_ids',['W06','W07','W08','W09']),'matrix_ids':c.get('matrix_ids',[]),
   'mandatory':True,'role':'opaque' if opaque else c.get('role','positive'),
   'source_ref_or_literal_with_sha256':{'artifact':pin(name),'selector':'cases[id='+c['id']+']' if 'cases' in data else '$'},
   'adapter':'actual_owner_local_compiled_test_only_binding','profiles':c.get('profiles',PROFILES),'orders':c.get('orders',ORDERS),
   'reference_policy':'normative_l0','expected_observation_or_reference_with_sha256':{'artifact':pin(name),'selector':'cases[id='+c['id']+'].predicates|expected|typed_predicates' if 'cases' in data else '$.typed_predicates','conjunction':'All assertions and exact construction steps in selected fixture, not only a success boolean.'},
   'raw_input_provenance':{'fixture':pin(name),'candidate_access':False},
   'phase_class':'candidate_internal','freeze_phase':'C','execution_phases':['F'],
   'binding_contract_if_needed':{'public_interface':pin('p1339-mutant-closed-state-interface.md'),'private_interface':pin('p1339-mutant-closed-state-private-interface.md'),'authority':pin('p1339-closed-harness-authority.json'),'contract_selector':'candidate_test_binding','actual_graph_audit':'mandatory independent acceptance before interpreting runtime DTO'},
   'phase':ph,'credit':{'C':0,'D':0,'positive':0 if opaque else 'only after actual F pass'}})
 return out
relations=load('p1339-ab-private-relation-fixtures-r1.json')['cases']
opaque_rel={c['id'] for c in relations if c['expected']['discriminant']=='Unproven'}
future=[]
for n in ['p1339-ab-closed-api-fixtures-r1.json','p1339-ab-batch1-projection-fixtures-r2.json','p1339-ab-batch1-same-context-style-fixture.json','p1339-ab-batch1-retention-lifecycle-typed-r3.json']:
 future+=future_entries(n)
future+=future_entries('p1339-ab-private-relation-fixtures-r1.json',{c['id'] for c in relations}-opaque_rel)
opaque_future=future_entries('p1339-ab-private-relation-fixtures-r1.json',opaque_rel,True)+future_entries('p1339-ab-batch1-public-opaque-fixture-r2.json',opaque=True)
positive=copy.deepcopy(common);positive.update({'kind':'positive_expected_error_preservation','plan':plan_pin,'public_definition_ancestry':defs_pin,
 'oracles':public,'candidate_internal_oracles':future,'coverage_map':pin('p1339-ab-batch1-coverage-map-r2.json'),
 'pending_causal_recipe':recipe,'execution_transport':pin('p1339-ab-executor-v4.py'),
 'public_exact_comparison_recipe':{'observed_requirement':'execution == Observed and unknown_reason == null; actual input and executable provenance match frozen plan/phase',
  'calibration_reference':'For each product and profile compare to its own pinned by_product reference. Candidate compares to candidate_product; mutation host to mutation_host_product. No common vanilla target for baseline preservation.',
  'reference_load':'Verify SHA before parsing referenced artifact; select exact collection/row_index and optional product_field, then map each named dimension to its pinned raw field. Missing/ambiguous reference fails Unknown/mandatory_unknown.',
  'comparison':'For every listed dimension compare full decoded raw string/int/bool exactly; dimensions artifact_presence maps actual pdf_exists only. No line/path/span/newline/Unicode/trace/diagnostic normalization.',
  'causal_composition':'Only where-l0-empty-vs-bare candidate expectation uses pending_causal_recipe. Preserve original vanilla/baseline raw reference for C. New isolated controls use own-product C raw; F uses declared baseline/vanilla target. Pending references block seal until measured and pinned.',
  'show_effect':'Conjoin exact marker-causal effect: required CallbackExecuted has marker diagnostic, CallbackNotExecuted lacks marker and succeeds with artifact; OtherDiagnosticBeforeCallback lacks marker and is exact designated error. Use show_effect_by_product where present, especially direct-show baseline candidate vs vanilla mutation host.',
  'matrix':'Every required case/profile/order/product exactly once. Unknown, duplicates, missing cells, wrong binary/source hash or unstable comparison dimensions block. PDF hashes recorded as provenance only.'},
 'counts':{'existing_public_definitions':len(public),'candidate_internal_nonopaque_definitions':len(future),'all_future_core_raw_cells_including_opaque':1284,'all_future_pipeline_raw_cells':144,'future_runtime_executed':0},
 'final_future_gate':'Require both actual L1 core and L3 pipeline test binaries and all1284+144 raw cells plus corresponding fixture predicates, lifecycle-v3, all-channel opaque predicate and independent exact SW/field map. Collector exit0 is not pass.'})
opaque=copy.deepcopy(common);workload=next(c for c in plan['cases'] if c['id']=='opaque-budget-workload')
work=complete_public({'id':workload['id'],'contract_sha256':contract['sha256'],'authority_manifest_sha256':manifest['sha256'],'obligation_ids':['protocol-opacity'],
 'mandatory':False,'required_control':True,'role':'opaque','source_ref_or_literal_with_sha256':{'literal':workload['expression'],'sha256':hashlib.sha256(workload['expression'].encode()).hexdigest()},
 'adapter':'eval_exact','profiles':PROFILES,'orders':ORDERS,'reference_policy':'normative_l0',
 'expected_observation_or_reference_with_sha256':{'definition':defs_pin,'selector':'batch1_new_case_policies[id=opaque-budget-workload]','expected_by_product':workload['expected_by_product']},
 'raw_input_provenance':{'plan':plan_pin,'case_id':workload['id'],'review':pin('p1339-verifier-opacity-design-review-r1.md')},
 'invocation':{'mode':'eval','timeout_seconds':1,'cwd':str(D.parent.parent)},'positive_credit':0})
opaque.update({'kind':'declared_opacity_and_conditional_applicability','oracles':[work],
 'conditional_angle_controls':[complete_public(c) for c in defs['conditional_opaque_controls']],
 'candidate_internal_oracles':opaque_future,
 'all_channel_public_opaque_predicate':pin('p1339-ab-batch1-opaque-predicate.py'),
 'asymmetric_opacity_policy':{'vanilla':'Same frozen 1-second workload must retain raw Unknown(timeout), exit null, empty stdout/stderr; observation-budget opacity only.',
  'baseline':'Same workload must retain exact known guard error tuple from pinned r2 raw per profile, never Unknown.',
  'candidate':'Preserve baseline known error tuple under same1-second deadline, not vanilla timeout parity.',
  'finite_control':'opaque-budget-finite separately required positive, both products exact3 newline within3seconds.',
  'determinism':'Repeat/reverse every executable-specific cell; drift blocks. Never relabel earlier32processes; no guard removal, deadline tuning or fourth contract revision.',
  'credit':'Zero positive credit for this workload, four conditional Angle controls and actual F06 Unproven/public noncertification; none substitutes for C mutation families.'},
 'counts':{'existing_required_asymmetric_workload':1,'conditional_angle_unactivated':4,'future_opaque_definitions':len(opaque_future),'future_runtime_executed':0},
 'candidate_internal_raw_rule':'Actual private discriminant Unproven is mandatory for frozen opaque relation; public validation must not Ok(true), with exact allowed Err/Okfalse and all-channel warning veto. It remains NotDue only before F, never from an observed failed execution.'})
required=set(load('p1339-contract-r3.json')['oracle_interface']['required_fields'])
for x in public+future+[work]+opaque['conditional_angle_controls']+opaque_future:
 if not required<=set(x):raise ValueError((x['id'],required-set(x)))
assert len(public)==719 and len(public)+len(opaque['conditional_angle_controls'])+1==len(plan['cases'])==724
assert len(future)+len(opaque_future)==119
publish('p1339-positive-oracles.json',positive)
publish('p1339-opaque-oracles.json',opaque)
