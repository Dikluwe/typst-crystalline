"""Mechanical phase-D execution and receipt using already sealed expectations.
No candidate, source interpretation, future API execution or new oracle policy.
"""
import collections, concurrent.futures, datetime, hashlib, importlib.util
import json, re, subprocess, time
from pathlib import Path
R=Path('/repos/Antigravity/typst-crystalline');D=R/'00_nucleo/diagnosticos'
SEAL=D/'p1339-seal.json';SEAL_SHA='35f00c4b9e15a010692017f5083ea4f451f4104a3730022136972e151ac0a8ee'
def sha(p):
 h=hashlib.sha256()
 with Path(p).open('rb') as f:
  for chunk in iter(lambda:f.read(1024*1024),b''):h.update(chunk)
 return h.hexdigest()
def pin(p):return {'path':str(Path(p).resolve()),'sha256':sha(p)}
def read(p):return json.loads(Path(p).read_text())
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def git(*args):return subprocess.check_output(['git',*args],cwd=R,text=True).rstrip('\n')
def state():return {'utc':utc(),'head':git('rev-parse','HEAD'),'status':git('status','--short'),'diff_stat':git('diff','HEAD','--stat')}
def save(p,data):
 assert p.parent==D and (p.name.startswith('p1339-ab-') or p.name=='p1339-red.json') and not p.exists()
 body=json.dumps(data,indent=2,ensure_ascii=True)+'\n'
 patch='*** Begin Patch\n*** Add File: '+str(p)+'\n'+''.join('+'+s+'\n' for s in body.split('\n')[:-1])+'*** End Patch\n'
 subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True)
def verify(seal):
 start=utc();tick=time.monotonic()
 def one(p):
  try:
   actual=sha(p['path']);return {'path':p['path'],'expected':p['sha256'],'actual':actual,'valid':actual==p['sha256']}
  except Exception as e:return {'path':p['path'],'expected':p['sha256'],'valid':False,'error':str(e)}
 with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:rows=list(pool.map(one,seal['immutable_inputs']))
 l0=[]
 for item in seal['l0']:
  b=Path(item['path']).read_bytes();lines=b.splitlines(keepends=True);boundary=next((i for i,l in enumerate(lines) if l in [b'\n',b'\r\n']),len(lines))
  matches=[i for i,l in enumerate(lines[:boundary]) if re.fullmatch(rb'Hash do C\xc3\xb3digo: [0-9a-f]{8}\r?\n',l)]
  normative=hashlib.sha256(b''.join(l for i,l in enumerate(lines) if i not in matches)).hexdigest()
  l0.append({'path':item['path'],'raw_sha256':hashlib.sha256(b).hexdigest(),'normative_sha256':normative,'valid':len(matches)==1 and hashlib.sha256(b).hexdigest()==item['raw_sha256'] and normative==item['normative_sha256']})
 return {'start_utc':start,'end_utc':utc(),'seconds':time.monotonic()-tick,'immutable_inputs':rows,'l0':l0,'seal_unchanged':sha(SEAL)==SEAL_SHA,'valid':all(x['valid'] for x in rows+l0) and sha(SEAL)==SEAL_SHA,
  'read_boundary':'Mechanical byte-stream SHA verification only for exact sealed paths, including mutant/reference sources; no source contents emitted or interpreted, no productive candidate inspected.'}

assert not (D/'p1339-red.json').exists() and not (D/'p1339-ab-red-runs.json').exists()
assert sha(SEAL)==SEAL_SHA
seal=read(SEAL);assert seal['verdict']=='SEALED_R3_C_SCOPED' and seal['mutation_score']==1.0
assert seal['discrimination']['sha256']=='4e7f14744dcb62f91c7d0c89a1f56edc397987208d97307f63cb217a7871c995'
before=state();print('D: verifying all sealed pins before execution',flush=True)
integrity_before=verify(seal)
save(D/'p1339-ab-red-integrity-before.json',{'seal':pin(SEAL),'state':before,'check':integrity_before})
assert integrity_before['valid'],'sealed input drift before RED'
positive=read(seal['active_oracles']['positive']['path']);opaque=read(seal['active_oracles']['opaque']['path'])
planpath=D/'p1339-ab-batch2-cli-plan.json';plan=read(planpath)
lookup={c['id']:c for c in plan['cases']};oracles={c['id']:c for c in positive['oracles']}
discovery=sorted(c['id'] for c in positive['oracles'] if c['id'].startswith('historical-discovery-'))
controls=sorted(c['id'] for c in positive['oracles'] if c['reference_policy']=='baseline')
ids=sorted(set(discovery+controls));assert len(discovery)==10 and len(controls)==114 and len(ids)==124
assert sum(len(lookup[i].get('profiles',['default','html','a11y','html+a11y']))*6 for i in ids)==2706
rawpath=D/'p1339-ab-red-runs.json'
argv=['python3',str(D/'p1339-ab-executor-v4.py'),'--plan',str(planpath),'--phase','red','--products','vanilla,baseline','--orders','normal,repeat,reverse','--ids',','.join(ids),'--output',str(rawpath),'--seal',str(SEAL),'--seal-sha256',SEAL_SHA]
invocation={'seal':pin(SEAL),'authority_manifest':seal['authority_manifest'],'contract':seal['contract'],'before':before,'argv':argv,'discovery_ids':discovery,'control_ids':controls,'processes':2706,'frozen_selection':'10 historical-discovery IDs plus every114reference_policy=baseline control. Verifier/coordination approved this conditional D subset; no724full-D obligation or opacity workload required.','future_runtime_credit':0,'recorder':pin(__file__)}
save(D/'p1339-ab-red-invocation.json',invocation)
print('D: starting124IDs /2706real baseline+vanilla cells',flush=True)
tick=time.monotonic();started=utc();p=subprocess.run(argv,cwd=R,text=True,capture_output=True)
call={'argv':argv,'cwd':str(R),'start_utc':started,'end_utc':utc(),'seconds':time.monotonic()-tick,'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr}
save(D/'p1339-ab-red-executor-call.json',call)
print('D: executor finished',p.returncode,p.stdout.strip(),flush=True)
assert p.returncode==0 and rawpath.exists(),'RED transport did not complete'
raw=read(rawpath);assert raw['phase']=='red' and raw['seal']==pin(SEAL)
spec=importlib.util.spec_from_file_location('p1339_sealed_public',seal['active_public_predicate']['path'])
predicate=importlib.util.module_from_spec(spec);spec.loader.exec_module(predicate)
focalpath=D/'p1339-ab-batch2-focal-runs.json';focal=read(focalpath);derived=read(seal['active_causal_derived']['path'])
assert pin(focalpath)==derived['raw']
print('D: judging all2706own-product cells and120baseline discovery failures',flush=True)
judged=predicate.check(raw,'calibration',ids,focal,derived)
failures=[{'kind':'own_product_or_control_difference','row':x} for x in judged if x['classification']!='Preserved' or x.get('differences')]
red_rows=[];vanilla_discovery=[]
for r in raw['rows']:
 if r['id'] not in discovery:continue
 x=predicate.evaluate(r,'candidate',positive,opaque,focal,derived)
 x.update({k:r[k] for k in ['product','profile','order']});x['obligation_ids']=oracles[r['id']]['obligation_ids'];x['raw_row_index']=raw['rows'].index(r)
 if r['product']=='baseline':
  red_rows.append(x)
  if not (r['execution']=='Observed' and r['unknown_reason'] is None and r['exit']==1 and x['classification']=='Violated' and x.get('differences')):failures.append({'kind':'not_genuine_absence_RED','row':x})
 else:
  vanilla_discovery.append(x)
  if not (r['execution']=='Observed' and r['exit']==0 and x['classification']=='Preserved'):failures.append({'kind':'vanilla_discovery_not_green','row':x})
assert len(red_rows)==120 and len(vanilla_discovery)==120
control_rows=[x for x in judged if x['id'] in controls];assert len(control_rows)==2466
after=state();print('D: verifying all sealed pins after execution',flush=True)
integrity_after=verify(seal)
save(D/'p1339-ab-red-integrity-after.json',{'seal':pin(SEAL),'state':after,'check':integrity_after})
if not integrity_after['valid']:failures.append({'kind':'sealed_input_drift_after_RED'})
data={'schema':'p1339-independent-D-RED-v1','author':'/root/p1319_tests','regime':'executado sem atestacao de isolamento','at':utc(),'seal':pin(SEAL),'authority_manifest':seal['authority_manifest'],'contract':seal['contract'],'discrimination':seal['discrimination'],
 'status':'GENUINE_BASELINE_RED_WITH_GREEN_CONTROLS' if not failures else 'RED_GATE_FAILED','independent_verifier_status':'PENDING_REVIEW','before':before,'after':after,
 'invocation':pin(D/'p1339-ab-red-invocation.json'),'executor_call':pin(D/'p1339-ab-red-executor-call.json'),'raw':pin(rawpath),'predicate':seal['active_public_predicate'],'derived':seal['active_causal_derived'],
 'integrity_before':pin(D/'p1339-ab-red-integrity-before.json'),'integrity_after':pin(D/'p1339-ab-red-integrity-after.json'),
 'processes':raw['processes'],'raw_wall_seconds':raw['wall_seconds'],'sum_child_seconds':sum(r['seconds'] for r in raw['rows']),'raw_execution_counts':dict(collections.Counter(r['execution'] for r in raw['rows'])),
 'discovery_ids':discovery,'control_ids':controls,'own_product_classifications':judged,'baseline_RED_against_sealed_F_reference':red_rows,'vanilla_discovery_green':vanilla_discovery,
 'counts':{'IDs':124,'baseline_RED_observed_Violated':sum(x['classification']=='Violated' for x in red_rows),'vanilla_discovery_Preserved':sum(x['classification']=='Preserved' for x in vanilla_discovery),'control_cells':len(control_rows),'controls_Preserved':sum(x['classification']=='Preserved' for x in control_rows),'mandatory_Unknown':sum(r['execution']!='Observed' for r in raw['rows'])},
 'failures':failures,'genuine_RED':not failures,'future_ledger':{'state':'NotDue_in_D','definitions':seal['phase_F_NotDue_ledger'],'execution_D':0,'credit_D':0,'not_RED':'No missing private API compile error is used as evidence.'},
 'no_new_expectations':'All actual source/invocation/profile/order/reference and full tuple/effect predicates are sealed. D merely selects ten discovery failures plus all114preexisting baseline controls; no oracle rewritten after seal.',
 'implementation_permission':'This author receipt still requires independent verifier review; author does not implement or self-certify final closure.','recorder':pin(__file__)}
save(D/'p1339-red.json',data)
print(json.dumps({'receipt':pin(D/'p1339-red.json'),'raw':pin(rawpath),'status':data['status'],'counts':data['counts'],'failures':len(failures)},ensure_ascii=True),flush=True)
