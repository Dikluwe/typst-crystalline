"""One grouped publication for the final aggregate batch; metadata only.
All generated Python is AST-checked before any successor artifact is written.
No compiler, recipe, semantic predicate or candidate is executed here.
"""
import ast, copy, datetime, hashlib, json, subprocess
from pathlib import Path
D=Path(__file__).resolve().parent
FILES={}
def hash_bytes(b):return hashlib.sha256(b).hexdigest()
def text(n):return FILES[n] if n in FILES else (D/n).read_text()
def pin(n,expected=None):
 p=Path(n);n=p.name
 if p.is_absolute() and p.parent!=D:raise ValueError('non-diagnostic input')
 h=hash_bytes(text(n).encode())
 if expected and h!=expected:raise ValueError(('pin drift',n))
 return {'path':str(D/n),'sha256':h}
def read(n):return json.loads(text(n))
def add(n,x):
 if (D/n).exists() or n in FILES:raise ValueError('immutable output exists '+n)
 FILES[n]=json.dumps(x,indent=2,ensure_ascii=True)+'\n' if not isinstance(x,str) else x.rstrip('\n')+'\n'
go=pin('p1339-verifier-batch1-focal-review-r1.md','d4b8b1c5dd764e479270ebd4eb31b5d510f176c46ca0996f66a17c9a68bcdcab')
oldplan=read('p1339-ab-batch1-cli-plan.json');oldpositive=read('p1339-positive-oracles.json');oldopaque=read('p1339-opaque-oracles.json')
show_ids=[o['id'] for o in oldpositive['oracles'] if o.get('show_effect')]
ids=sorted(set(show_ids+oldplan['batch1_focal_ids']))
assert len(show_ids)==21 and len(ids)==28
plan=copy.deepcopy(oldplan);plan['batch']=2;plan['status']='FINAL_BATCH_DEFINITIONS_FROZEN_PENDING_MANIFEST_ACCEPTANCE'
plan['predecessor']=pin('p1339-ab-batch1-cli-plan.json');plan['batch2_focal_ids']=ids
plan['batch1_focal_ids_historical_only']=plan.pop('batch1_focal_ids')
plan['input_sha256'].update({go['path']:go['sha256'],str(D/'p1339-ab-batch1-focal-runs.json'):pin('p1339-ab-batch1-focal-runs.json')['sha256']})
plan['source_invariance']='All724 case objects byte-equivalent as JSON values to batch1; no source, deadline, profile, invocation or historical reference altered.'
assert plan['cases']==oldplan['cases']
add('p1339-ab-batch2-cli-plan.json',plan)

# Primary-effect table is derived only from already pinned reference channels,
# not from the candidate, and checked for exact parser/marker consequences.
effect_table={}
for o in oldpositive['oracles']:
 if o['id'] not in show_ids:continue
 by={}
 for product,profiles in o['expected_observation_or_reference_with_sha256']['by_product'].items():
  by[product]={}
  for profile,ref in profiles.items():
   pin(ref['artifact'],ref['sha256']);r=read(Path(ref['artifact']).name)[ref['collection']][ref['row_index']]
   if ref.get('product_field'):r=r[ref['product_field']]
   raw={k:r[v] for k,v in ref['fields'].items()};first=raw['stderr'].split('\n',1)[0]
   panic='error: panicked with: '+o['show_marker']
   if first==panic:
    assert raw['exit']==1;effect='CallbackExecuted'
   elif raw['exit']==0:
    assert raw['stderr']=='' and raw['artifact_presence'] is True;effect='CompileSucceededNoCallback'
   else:
    assert raw['exit']==1 and first.startswith('error: ') and first!=panic;effect='OtherDiagnosticBeforeCallback'
   by[product][profile]={'effect':effect,'primary_diagnostic_line':first,'reference':ref}
 target=o['expected_observation_or_reference_with_sha256']['candidate_product']
 effect_table[o['id']]={'by_reference_product':by,'phase_reference_product':{'calibration':'actual_reference_product','candidate':target,'mutation_host':'vanilla'},'marker':o['show_marker'],'rule':'Exact complete first diagnostic line; no substring over source/frame/trace. Whole raw tuple comparison remains mandatory.'}
 assert all(x['effect']=='OtherDiagnosticBeforeCallback' for x in by['baseline'].values()) if '-direct-control' not in o['id'] else True
assert len(effect_table)==21
add('p1339-ab-batch2-show-effect-table.json',{'schema':'p1339-show-primary-effect-v1','predecessor':pin('p1339-positive-oracles.json'),'go':go,'cases':effect_table,'sources_changed':0,'candidate_read':False,'coverage':'All21show IDs, four profiles, C own product, F designated target, mutation_host vanilla independently of row product label.'})

# Preserve the strict spanful branch. The additional branch admits only the
# already measured exact two-channel-empty, one-message spanless tuple.
recipe=text('p1339-ab-batch1-causal-recipe.py')
recipe=recipe.replace("PLAN=D/'p1339-ab-batch1-cli-plan.json'","PLAN=D/'p1339-ab-batch2-cli-plan.json'")
recipe=recipe.replace("PLAN_SHA='55a46cf8390ea7914aba115afaf17e9ca9d07cedfb94e5cf7ce8ad3c0ce1d693'","PLAN_SHA='"+pin('p1339-ab-batch2-cli-plan.json')['sha256']+"'")
start=recipe.index('            positions=re.findall')
end=recipe.index("            _,allowed=row('causal-strong-empty-isolated'",start)
spanful=recipe[start:end]
newblock="""            spanless={'exit':1,'stdout':'','stderr':'error: only element functions can be used as selectors\\n\\n'}
            actual={k:r[k] for k in ['exit','stdout','stderr']}
            isolated_tuple={k:isolated[k] for k in ['exit','stdout','stderr']}
            if actual==spanless:
                assert isolated_tuple==spanless,'spanless padded/isolated full tuple mismatch'
                expected=spanless.copy()
                mapping={'kind':'strict_spanless_identity','replacements':0,'all_other_bytes':'identical full raw tuple; no source/frame/context exists'}
            else:
"""+''.join('    '+line+'\n' for line in spanful.rstrip('\n').split('\n'))+"""                mapping={'kind':'strict_spanful_source_line','old':old,'new':new,'replacements':1,'all_other_bytes':'identical'}
"""
recipe=recipe[:start]+newblock+recipe[end:]
recipe=recipe.replace("'byte_mapping':{'old':old,'new':new,'replacements':1,'all_other_bytes':'identical'}","'byte_mapping':mapping")
recipe=recipe.replace("'p1339-ab-batch1-focal-runs.json'","'p1339-ab-batch2-focal-runs.json'").replace("'p1339-ab-batch1-causal-derived-expectations.json'","'p1339-ab-batch2-causal-derived-expectations.json'")
needle="            assert heading['exit']==0 and heading['stdout']=='\"false\"\\n' and heading['stderr']==''"
assert needle in recipe
recipe=recipe.replace(needle,needle+"""
            for control_id,current in [('causal-strong-empty-isolated',allowed),('causal-heading-empty-vs-bare',heading)]:
                old_path=D/'p1339-ab-batch1-focal-runs.json'
                assert hashlib.sha256(old_path.read_bytes()).hexdigest()=='471b753ba19921b4d90e0393452ac251b5875dfc833b362d8d7146bfa171c7c8'
                prior_raw=json.loads(old_path.read_text())
                prior=[x for x in prior_raw['rows'] if x['id']==control_id and x['product']=='vanilla' and x['profile']==profile and x['order']==order]
                assert len(prior)==1
                assert {k:current[k] for k in ['exit','stdout','stderr']}=={k:prior[0][k] for k in ['exit','stdout','stderr']},'unchanged complete vanilla conjunction'
""")
add('p1339-ab-batch2-causal-recipe.py',recipe)

positive=copy.deepcopy(oldpositive);opaque=copy.deepcopy(oldopaque)
newraw=str(D/'p1339-ab-batch2-focal-runs.json');newderived=str(D/'p1339-ab-batch2-causal-derived-expectations.json')
def pending_recipe():
 r=copy.deepcopy(oldpositive['pending_causal_recipe']);r['artifact']=pin('p1339-ab-batch2-causal-recipe.py')
 r['command']=['python3',r['artifact']['path'],'--raw',newraw,'--output',newderived]
 r['raw']['path']=newraw;r['derived_receipt']['path']=newderived
 r['fail_closed']='Strict spanless branch requires exact measured padded/isolated full tuple in all12profile/order cells, no frame/source/context, zero substitutions. Strict spanful branch unchanged and separate. Both preserve byte offsets and complete vanilla conjunctions. Any other/partial/unstable form fails.'
 r['predecessor']=pin('p1339-ab-batch1-causal-recipe.py');r['authorized_cause']=go
 return r
for w,oldname in [(positive,'p1339-positive-oracles.json'),(opaque,'p1339-opaque-oracles.json')]:
 w['predecessor']=pin(oldname);w['batch']=2;w['status']='FINAL_BATCH_DEFINITIONS_FROZEN_PENDING_MANIFEST_ACCEPTANCE'
 w['published_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat();w['batch2_go']=go
 w['current_phase_counts']['aggregate_focal_batches_executed']=1
 w['batch1_failure_preserved']=pin('p1339-ab-batch1-focal-evaluation.json')
 w['execution_transport']=pin('p1339-ab-executor-v4.py')
 w['sources_and_future_invariance']='All724sources/deadlines unchanged; all119future definitions, adapters and assertions unchanged. Only the two authorized public-observation translation causes change.'
positive['plan']=pin('p1339-ab-batch2-cli-plan.json');positive['pending_causal_recipe']=pending_recipe();positive['show_effect_table']=pin('p1339-ab-batch2-show-effect-table.json')
positive['public_exact_comparison_recipe']['show_effect']='Use pinned batch2 table and exact primary line. C own product, F designated reference, mutation_host always vanilla regardless of raw product label. No callback evidence from source/frame/trace substring.'
positive['public_exact_comparison_recipe']['causal_composition']='Only original empty-vs-bare uses new strict spanless-or-spanful recipe for candidate expectation; all controls and full tuples remain conjunctive. Pending raw/derived block seal.'
for o in positive['oracles']:
 if o['id'] in effect_table:o['show_primary_effect_policy']=effect_table[o['id']]
 if 'pending_causal_recipe' in o:o['pending_causal_recipe']=pending_recipe()
 if 'pending_measurement_recipe' in o:o['pending_measurement_recipe']=pending_recipe()
 e=o['expected_observation_or_reference_with_sha256']
 if e.get('planned_raw'):e['planned_raw']['path']=newraw
assert positive['candidate_internal_oracles']==oldpositive['candidate_internal_oracles']
assert opaque['candidate_internal_oracles']==oldopaque['candidate_internal_oracles']
assert opaque['oracles']==oldopaque['oracles'] and opaque['conditional_angle_controls']==oldopaque['conditional_angle_controls']
add('p1339-ab-batch2-positive-oracles.json',positive);add('p1339-ab-batch2-opaque-oracles.json',opaque)

predicate='''"""Final-batch observation translation successor, no product execution.
Uses unchanged transport/provenance/tuple helpers from immutable predecessors;
replaces only frozen wrapper/plan lookup and causal show-effect evaluation.
"""
import argparse, hashlib, importlib.util, json
from pathlib import Path
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('p1339_public_v2',D/'p1339-ab-batch1-public-predicate-v2.py')
v2=importlib.util.module_from_spec(spec);spec.loader.exec_module(v2)
base=v2.prior;require=base.require
ALIASES=__ALIASES__
def root(name):
 target,sha=ALIASES[name]
 return base.pinned(D/target,sha)
base.root=root
for old,(new,sha) in ALIASES.items():base.PINS[old]=sha
old_evaluate=base.evaluate
def evaluate(row,purpose,positive,opaque,focal=None,derived=None):
 lookup={o['id']:o for o in positive['oracles']}
 o=lookup.get(row['id'])
 if o is None or not o.get('show_primary_effect_policy'):
  return old_evaluate(row,purpose,positive,opaque,focal,derived)
 if row['execution']!='Observed' or row['unknown_reason'] is not None:
  return {'id':row['id'],'raw_execution':row['execution'],'classification':'Unknown','reason':'mandatory_unknown','positive_credit':0}
 wanted=base.expected_for(o,row,purpose,focal,derived)
 actual=base.tuple_of(row,wanted)
 differences=[{'dimension':d,'expected':wanted[d],'actual':actual[d]} for d in wanted if actual[d]!=wanted[d]]
 policy=o['show_primary_effect_policy']
 reference_product=row['product'] if purpose=='calibration' else policy['phase_reference_product'][purpose]
 chosen=policy['by_reference_product'][reference_product][row['profile']]
 effect=chosen['effect'];primary=row['stderr'].split('\\n',1)[0]
 exact_panic='error: panicked with: '+o['show_marker']
 if effect=='CallbackExecuted':valid=primary==exact_panic and primary==chosen['primary_diagnostic_line'] and row['exit']==1
 elif effect=='CompileSucceededNoCallback':valid=row['exit']==0 and row['stderr']=='' and row['pdf_exists'] is True
 elif effect=='OtherDiagnosticBeforeCallback':valid=row['exit']==1 and primary==chosen['primary_diagnostic_line'] and primary!=exact_panic
 else:require(False,'unknown causal effect variant')
 if not valid:differences.append({'dimension':'causal_show_effect','expected':chosen,'actual':{'primary_diagnostic_line':primary,'exit':row['exit'],'pdf_exists':row['pdf_exists']}})
 return {'id':row['id'],'raw_execution':'Observed','classification':'Violated' if differences else 'Preserved','reason':'observable_difference' if differences else 'exact_designated_reference','differences':differences,'positive_credit':0 if differences else 1,'show_effect':{'reference_product':reference_product,'expected':effect,'actual_primary_line':primary,'matched':valid}}
base.evaluate=evaluate
def check(raw,purpose,ids,focal=None,derived=None):return v2.check(raw,purpose,ids,focal,derived)
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--raw',required=True);p.add_argument('--purpose',choices=['calibration','candidate','mutation_host'],required=True)
 p.add_argument('--ids',required=True);p.add_argument('--focal');p.add_argument('--focal-sha256');p.add_argument('--derived');p.add_argument('--derived-sha256');a=p.parse_args()
 raw=json.loads(Path(a.raw).read_text())
 focal=base.pinned(a.focal,a.focal_sha256) if a.focal else None
 derived=base.pinned(a.derived,a.derived_sha256) if a.derived else None
 if derived is not None:require(derived['raw']=={'path':str(Path(a.focal).resolve()),'sha256':a.focal_sha256},'derived raw identity')
 for result in check(raw,a.purpose,a.ids.split(','),focal,derived):print(json.dumps(result,ensure_ascii=True))
'''
aliases={old:(new,pin(new)['sha256']) for old,new in [('p1339-positive-oracles.json','p1339-ab-batch2-positive-oracles.json'),('p1339-opaque-oracles.json','p1339-ab-batch2-opaque-oracles.json'),('p1339-ab-batch1-cli-plan.json','p1339-ab-batch2-cli-plan.json')]}
predicate=predicate.replace('__ALIASES__',repr(aliases))
add('p1339-ab-batch2-public-predicate.py',predicate)

# Explicit preparation/delta ledger; historical failures are not recoded.
ledger={'schema':'p1339-final-batch-delta-ledger-v1','author':'/root/p1319_tests','regime':'executado sem atestacao de isolamento','batch':2,'go':go,
 'predecessor_manifest':pin('p1339-ab-batch1-manifest-r2.json'),'predecessor_raw':pin('p1339-ab-batch1-focal-runs.json'),'predecessor_evaluation':pin('p1339-ab-batch1-focal-evaluation.json'),
 'historical_cost':{'batch1_processes':384,'wall_seconds':12.819637848006096,'sum_child_seconds':49.92340499785496,'recipe_seconds':0.10872717201709747,'recipe_failed':True,'raw_unknown':12,'classification_counts_preserved_unmodified':{'Preserved':324,'Violated':48,'Unknown':12}},
 'historical_false_effects':'36baseline static-match cells were falsely tagged as callback evidence because marker occurred in source frame. Historical classifications remain byte-intact and are not positive causal proof.',
 'hypotheses':[{'cause':'show marker/source conflation and phase-target mismatch','affected_ids':show_ids,'delta':'Exact primary diagnostic panic line, full raw tuple unchanged; explicit product/profile/phase table for all21static/bound/direct forms. mutation_host resolves vanilla independently of raw label.','predicted_gain':'No source-embedded marker certifies callback. All baseline where forms report OtherDiagnosticBeforeCallback while vanilla match/empty executes and miss succeeds; directtext preserves legacy baseline error. Correcting former36false effects is not a new positive case.'},
 {'cause':'strict recipe assumed absent source frame','affected_ids':[i for i in ids if i.startswith('causal-') or i=='where-l0-empty-vs-bare'],'delta':'Add only exact measured spanless full tuple identity branch, zero substitutions, all12cells and complete vanilla conjunctions; preserve original strict spanful branch separately.','predicted_gain':'Derived expectation produced without invented location if all exact controls stable; any extra/partial context fails closed.'}],
 'unchanged_controls':['opaque-budget-finite','opaque-budget-workload'],'source_cases_changed':0,'future_definitions_changed':0,'R3_changed':False,'L0_changed':False,'candidate_read':False,
 'budget':{'batch1_consumed_rejected':True,'batch2_is_last':True,'batch2_executed':False,'automatic_retries':0,'third_batch_allowed':False,'full_C_used':0,'full_C_limit':2},
 'static_checks_before_publication':['Parse all generated Python with ast.parse only; no recipe/predicate invocation','Require all724case objects exactly equal to predecessor','Require119future definitions and opaque source/expectations exactly equal','Require21show policies,28focal IDs and672unique cells','Verify all pinned baseline/vanilla reference SHAs before extracting exact primary lines'],
 'no_runtime_credit':'Metadata publication/AST parsing is not compiler execution, mutant rejection or future F coverage.'}
add('p1339-ab-batch2-delta-ledger.json',ledger)

manifest=copy.deepcopy(read('p1339-ab-batch1-manifest-r2.json'))
manifest['predecessor']=pin('p1339-ab-batch1-manifest-r2.json');manifest['batch']=2
manifest['status']='FINAL_BATCH_FROZEN_PENDING_INDEPENDENT_MANIFEST_ACCEPTANCE_NO_EXECUTION_AUTHORITY'
manifest['published_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat()
manifest['operational_manifest']='Sole operational manifest for second and last aggregate batch; no third batch, no semantic retry.'
manifest['batch2_go']=go;manifest['delta_ledger']=pin('p1339-ab-batch2-delta-ledger.json')
manifest.pop('publication_successor',None)
manifest['active_public_wrappers']={'positive':pin('p1339-ab-batch2-positive-oracles.json'),'opaque':pin('p1339-ab-batch2-opaque-oracles.json')}
manifest['active_public_predicate']=pin('p1339-ab-batch2-public-predicate.py');manifest['active_show_effect_table']=pin('p1339-ab-batch2-show-effect-table.json')
manifest['hypotheses']=ledger['hypotheses']
manifest['existing_execution_slice']['ids']=ids
cells=[{'case_id':id,'product':product,'profile':profile,'order':order} for order in ['normal','repeat','reverse'] for profile in ['default','html','a11y','html+a11y'] for id in (list(reversed(ids)) if order=='reverse' else ids) for product in ['vanilla','baseline']]
assert len(cells)==672 and len({tuple(c.values()) for c in cells})==672
manifest['existing_execution_slice']['cells']=cells;manifest['existing_execution_slice']['processes']=672
manifest['existing_execution_slice']['argv']=['python3',str(D/'p1339-ab-executor-v4.py'),'--plan',str(D/'p1339-ab-batch2-cli-plan.json'),'--phase','calibration','--products','vanilla,baseline','--orders','normal,repeat,reverse','--ids',','.join(ids),'--output',newraw]
manifest['pending_outputs']={'raw':{'path':newraw,'status':'PENDING_MEASUREMENT','sha256':None,'credit':0},'derived':{'path':newderived,'status':'PENDING_MEASUREMENT','sha256':None,'credit':0},'recipe':pending_recipe(),'comparison':'Use batch2-public-predicate only; consume/judge all672classifications. No exit0 shortcut. Recipe failure or any mandatory Unknown/Violated blocks; no automatic retry.'}
manifest['budget']['batches_executed_before_manifest']=1;manifest['budget']['this_batch_planned_processes']=672
lookup={c['id']:c for c in plan['cases']}
manifest['budget']['per_process_timeout_seconds']={id:lookup[id].get('timeout_seconds',30) for id in ids}
manifest['budget']['maximum_sum_child_timeout_seconds']=sum(lookup[c['case_id']].get('timeout_seconds',30) for c in cells)
manifest['budget']['retry_policy']='This is last aggregate batch2. No semantic retry or third batch. Failure/no gain/new insufficiency stops for diagnosis and explicit redesign; no expectation tuning.'
manifest['predicate_classification_aggregation'].update({'exit_zero':'Parse every classification and require exact672cell bijection. Process exit0 never PASS.','ordinary_rows':'All648ordinary cells must Preserved, no differences/mandatory_unknown. Every21show policy must additionally report exact primary effect and matched=true.','opaque_rows':'Exactly12vanilla workload Unknown with declared timeout reason/control_satisfied/credit0;12baseline workload known-error Preserved/control_satisfied/credit0.','gain_vector':'Record all672classifications and exact show effects. Repaired48previous violations and36previous false callback effects remain distinct metrics; no historical recoding.'})
manifest['gain_ledger_recipe']='Report exact672cells: expected660Preserved including12baseline opaque-control known errors,12declared vanilla Unknown;648ordinary cells. FutureF unchanged and zero execution. Distinguish corrected translation from previously false callback credit and retain all batch1failures/costs.'
manifest['acceptance_required']='Independent verifier must accept this exact final-batch manifest before672processes. No fullC/RED/candidate from this publication; no third aggregate retry.'
manifest['recorder_successor_reason']='Single grouped publication of authorized batch2 translation changes; all generated scripts AST-checked before publication.'
manifest['provenance']={'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=D.parent.parent,text=True).strip(),'working_tree':'uncommitted','utc':manifest['published_utc'],'status':subprocess.check_output(['git','status','--short'],cwd=D.parent.parent,text=True),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=D.parent.parent,text=True)}
old_artifacts={x['path']:x for x in manifest['artifacts']}
for item in [go,pin('p1339-ab-batch1-focal-runs.json'),pin('p1339-ab-batch1-focal-evaluation.json'),pin(Path(__file__).name)]+[pin(n) for n in FILES]:old_artifacts[item['path']]=item
manifest['artifacts']=list(old_artifacts.values())
add('p1339-ab-batch2-manifest.json',manifest)

for n,s in FILES.items():
 if n.endswith('.py'):ast.parse(s,filename=n)
 else:json.loads(s)
assert read('p1339-ab-batch2-cli-plan.json')['cases']==oldplan['cases']
patch='*** Begin Patch\n'
for n,s in FILES.items():patch+='*** Add File: '+str(D/n)+'\n'+''.join('+'+line+'\n' for line in s.split('\n')[:-1])
patch+='*** End Patch\n'
subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True)
for n in FILES:print(n,pin(n)['sha256'])
