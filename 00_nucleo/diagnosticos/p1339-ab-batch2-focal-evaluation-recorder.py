"""Record frozen post-focal recipe and classifier calls exactly once.
Does not execute compilers or alter any oracle, recipe or raw observation.
"""
import collections, datetime, hashlib, json, subprocess, time
from pathlib import Path
D=Path(__file__).resolve().parent
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def pin(n):return {'path':str(D/n),'sha256':sha(D/n)}
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def run(argv):
 start=now();tick=time.monotonic();p=subprocess.run(argv,cwd=D.parent.parent,text=True,capture_output=True)
 return {'argv':argv,'cwd':str(D.parent.parent),'start_utc':start,'end_utc':now(),'seconds':time.monotonic()-tick,'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr}
rawpath=D/'p1339-ab-batch2-focal-runs.json';raw=json.loads(rawpath.read_text())
manifest=json.loads((D/'p1339-ab-batch2-manifest.json').read_text())
accept=json.loads((D/'p1339-verifier-batch2-manifest-acceptance-r1.json').read_text())
assert accept['manifest']==pin('p1339-ab-batch2-manifest.json')
assert accept['verdict']=='ACCEPT_FOCAL_BATCH2_LAST_ONLY'
assert raw['processes']==672 and len(raw['rows'])==672
derivedpath=D/'p1339-ab-batch2-causal-derived-expectations.json'
out=D/'p1339-ab-batch2-focal-evaluation.json'
assert not out.exists() and not derivedpath.exists(),'do not rerun recipe or overwrite receipt'
recipe=manifest['pending_outputs']['recipe'];assert sha(recipe['artifact']['path'])==recipe['artifact']['sha256']
recipe_call=run(recipe['command'])
argv=['python3',str(D/'p1339-ab-batch2-public-predicate.py'),'--raw',str(rawpath),'--purpose','calibration','--ids',','.join(manifest['existing_execution_slice']['ids']),'--focal',str(rawpath),'--focal-sha256',sha(rawpath)]
derived=None
if recipe_call['exit']==0 and derivedpath.exists():
 derived=pin(derivedpath.name);argv+=['--derived',str(derivedpath),'--derived-sha256',derived['sha256']]
predicate_call=run(argv)
classifications=[];parse_errors=[]
for i,line in enumerate(predicate_call['stdout'].split('\n')):
 if not line:continue
 try:classifications.append(json.loads(line))
 except Exception as e:parse_errors.append({'line_index':i,'line':line,'error':str(e)})
actual_cells=[(x.get('id'),x.get('product'),x.get('profile'),x.get('order')) for x in classifications]
expected_cells={(r['id'],r['product'],r['profile'],r['order']) for r in raw['rows']}
failures=[]
if recipe_call['exit']!=0 or derived is None:failures.append({'kind':'causal_recipe_failed','exit':recipe_call['exit'],'stderr':recipe_call['stderr']})
if predicate_call['exit']!=0:failures.append({'kind':'predicate_execution_failed','exit':predicate_call['exit'],'stderr':predicate_call['stderr']})
if parse_errors:failures.append({'kind':'predicate_output_parse','errors':parse_errors})
if len(actual_cells)!=672 or len(actual_cells)!=len(set(actual_cells)) or set(actual_cells)!=expected_cells:failures.append({'kind':'classification_cell_bijection','actual_count':len(actual_cells),'unique_count':len(set(actual_cells))})
for c in classifications:
 if c['id']=='opaque-budget-workload' and c['product']=='vanilla':
  valid=c['classification']=='Unknown' and c['reason']=='declared_observation_budget_opacity' and c.get('control_satisfied') is True and c['positive_credit']==0
 elif c['id']=='opaque-budget-workload':
  valid=c['classification']=='Preserved' and c['reason']=='asymmetric_known_error_control' and c.get('control_satisfied') is True and c['positive_credit']==0
 else:valid=c['classification']=='Preserved' and not c.get('differences')
 if c['id'].startswith('show-'):valid=valid and c.get('show_effect',{}).get('matched') is True
 if not valid:failures.append({'kind':'classification_rejected','row':c})
data={'schema':'p1339-focal-evaluation-receipt-v1','author':'/root/p1319_tests','regime':'executado sem atestacao de isolamento','at':now(),
 'authority_manifest':pin('p1339-authority-manifest-r2.json'),'contract':pin('p1339-contract-r3.json'),'batch_manifest':pin('p1339-ab-batch2-manifest.json'),'independent_authorization':pin('p1339-verifier-batch2-manifest-acceptance-r1.json'),
 'raw':pin(rawpath.name),'raw_processes':raw['processes'],'raw_execution_counts':dict(collections.Counter(r['execution'] for r in raw['rows'])),'raw_unknown_causes':dict(collections.Counter(r['unknown_reason'] for r in raw['rows'] if r['execution']!='Observed')),
 'raw_wall_seconds':raw['wall_seconds'],'sum_child_seconds':sum(r['seconds'] for r in raw['rows']),'raw_provenance_selectors':['before','after','binaries','runner_sha256','rows[].argv/env/cwd/source_sha256/start_utc/end_utc'],
 'recipe':recipe['artifact'],'recipe_call':recipe_call,'derived_receipt':derived,'predicate':pin('p1339-ab-batch2-public-predicate.py'),'predicate_predecessor':pin('p1339-ab-batch1-public-predicate.py'),'predicate_call':predicate_call,
 'classifications':classifications,'classification_counts':dict(collections.Counter(c.get('classification') for c in classifications)),'reason_counts':dict(collections.Counter(c.get('reason') for c in classifications)),
 'failures':failures,'AUTHOR_FOCAL_CHECKS_SATISFIED':not failures,
 'independent_verdict':'PENDING; author receipt is not verifier seal or acceptance','no_exit_zero_shortcut':'Every classification parsed and checked against exact672cell set and declared control policy. Process exit alone never grants pass.',
 'budget':{'aggregate_batches_executed_since_redesign':2,'aggregate_batches_limit':2,'full_preseal_used':0,'full_preseal_limit':2,'automatic_retries':0,'canonical_RED':0,'candidate':0,'future_internal_executions':0},
 'cost_delta':'One authorized672process focal; one frozen recipe invocation and one classifier invocation. Any failures retained; no modified expectations or automatic retry.',
 'recorder':pin(Path(__file__).name),
 'prelaunch_metadata_abort':{'error':"KeyError: 'authorized_execution'",'stderr':"Traceback (most recent call last):\n  File \"<stdin>\", line 8, in <module>\nKeyError: 'authorized_execution'\n",'wall_seconds':0.000008726,'semantic_processes':0,'cause':'Acceptance2 names field authorized_argv, not predecessor authorized_execution. Abort happened before subprocess.run(argv). Then launched exact pinned argv directly, once.','product_retry':False,'inputs_changed':False},
 'gain_evidence':{'old_classifications_immutable':pin('p1339-ab-batch1-focal-evaluation.json'),'old_raw_immutable':pin('p1339-ab-batch1-focal-runs.json'),'show_effect_counts':dict(collections.Counter(c.get('show_effect',{}).get('expected') for c in classifications if c['id'].startswith('show-'))),'all_show_cells':sum(c['id'].startswith('show-') for c in classifications),'scope':'All21static/bound/direct forms; old36false callback effects are not positive proof or retroactively recoded.'}}
body=json.dumps(data,indent=2,ensure_ascii=True)+'\n';patch='*** Begin Patch\n*** Add File: '+str(out)+'\n'+''.join('+'+s+'\n' for s in body.split('\n')[:-1])+'*** End Patch\n'
subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True)
print(json.dumps({'receipt':pin(out.name),'raw':pin(rawpath.name),'derived':derived,'classification_counts':data['classification_counts'],'failures':failures,'AUTHOR_FOCAL_CHECKS_SATISFIED':not failures},ensure_ascii=True))
