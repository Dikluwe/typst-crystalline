"""Independent complete C matrix, one budgeted run, gated by focal acceptance.

Reuses frozen transport/predicates without editing them. Never runs future APIs.
Raw receipts precede classification; no automatic semantic retry or seal.
"""
import argparse, base64, concurrent.futures, datetime, hashlib, importlib.util
import itertools, json, os, pathlib, re, resource, subprocess, sys, tempfile, time
sys.dont_write_bytecode = True
ROOT = pathlib.Path(__file__).resolve().parents[2]
D = ROOT / '00_nucleo/diagnosticos'
PINS = {
 'p1339-ab-batch1-manifest-r2.json':'86742a540fc58ac9e4096aadf32541f520a7953af3e209df490f48f69cee3d08',
 'p1339-verifier-batch1-manifest-acceptance-r1.json':'2a5285c99c9c0d64570aeb01c66f2bf4ed8f9de772b8a24c6b949fe2aa376419',
 'p1339-positive-oracles.json':'482babb923e306357f1aa70490e4fef8a5a16f772a494819d50feaee6f2c6972',
 'p1339-opaque-oracles.json':'4f33c33353caee89f509fe81ed76a2f631510684645f1c9f6b82341587472ed0',
 'p1339-ab-batch1-cli-plan.json':'55a46cf8390ea7914aba115afaf17e9ca9d07cedfb94e5cf7ce8ad3c0ce1d693',
 'p1339-ab-executor-v4.py':'0f4353a9b2b6ebb66b2ff918cae7580681b7d83308d4801b2d4924a6d6759c97',
 'p1339-ab-batch1-public-predicate-v2.py':'5a0802c79948420744ebcbb972679d6ab9d2a039f06bb8b0fd83d5560e9abb3e',
 'p1339-ab-batch1-public-predicate.py':'9fa26cecc4d6bf266f72016a619b7f5b283cec801f5a61bea6028d943bdb5294',
 'p1339-mutation-registry.json':'04d5578d720b4c7fff47954c29d20392b7a221f8b0cb80094a3859409add3677',
 'p1339-verifier-mutant-focal-review-r1.md':'08a6841630810fcb523f546ac15b5ad638925e80d76e1679063a0060e94b179a',
}
PROFILES={'default':[],'html':['--features','html'],'a11y':['--features','a11y-extras'],'html+a11y':['--features','html,a11y-extras']}
ORDERS=['normal','repeat','reverse']

def sha(p): return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def now(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def read(p): return json.loads(pathlib.Path(p).read_text())
def pin(p): return {'path':str(pathlib.Path(p).resolve()),'sha256':sha(p)}
def checked(ref):
 assert sha(ref['path'])==ref['sha256'],ref
 return pathlib.Path(ref['path'])
def write_new(name,data):
 path=D/name
 assert name.startswith('p1339-verifier-') and not path.exists(),str(path)
 body=json.dumps(data,ensure_ascii=True,indent=2)+'\n'
 patch='*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+l+'\n' for l in body.splitlines())+'*** End Patch\n'
 subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True,cwd=ROOT)
 return pin(path)
def module(name,file):
 spec=importlib.util.spec_from_file_location(name,D/file)
 mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod);return mod
def state():
 def git(*args):return subprocess.check_output(['git',*args],cwd=ROOT,text=True)
 diff=git('diff','--unified=0','HEAD','--','01_core','02_shell','03_infra','04_wiring','tests')
 changes=[l for l in diff.splitlines() if l[:1] in ['+','-'] and not l.startswith(('+++','---'))]
 assert all(re.fullmatch(r'[+-]//! @prompt-hash [a-f0-9]{8}',l) for l in changes),'candidate/body change before seal'
 untracked=git('ls-files','--others','--exclude-standard','--','01_core','02_shell','03_infra','04_wiring','tests')
 assert not untracked,'untracked product before seal'
 return {'utc':now(),'head':git('rev-parse','HEAD').strip(),'status':git('status','--short'),'diff_stat':git('diff','HEAD','--stat'),'product_nonheader_changes':0,'untracked_product':[]}
def validate_inputs(plan,transport):
 for name,h in PINS.items():assert sha(D/name)==h,name
 manifest=read(D/'p1339-ab-batch1-manifest-r2.json')
 for ref in manifest['artifacts']+manifest['authorities']:checked(ref)
 transport.checked_inputs(plan)
 for ref in manifest['binaries'].values():checked(ref)
 registry=read(D/'p1339-mutation-registry.json')
 inventories=[]
 for host in [registry['multiplex'],registry['architecture_witness']['control'],registry['architecture_witness']['negative']]:
  checked(host['binary']);root=pathlib.Path(host['source_root'])
  actual={str(p.relative_to(root)):sha(p) for p in sorted(root.rglob('*')) if p.is_file() and (p.suffix=='.rs' or p.name in ('Cargo.toml','Cargo.lock'))}
  assert actual==host['source_sha256'],str(root)
  inventories.append({'source_root':str(root),'files':len(actual),'exact':True})
 for mutant in registry['mutants']:
  for field in ['binary','source','build','patch']:checked(mutant[field])
  assert read(mutant['build']['path'])['exit']==0
 return inventories

def public_matrix(plan,transport):
 output=pathlib.Path(tempfile.mkdtemp(prefix='p1339-verifier-full-public-'))
 binaries=transport.BINARIES;tasks=[]
 for order in ORDERS:
  for profile in PROFILES:
   for case in reversed(plan['cases']) if order=='reverse' else plan['cases']:
    if profile not in case.get('profiles',list(PROFILES)):continue
    for product,(binary,digest) in binaries.items():tasks.append((case,binary,product,profile,order,output,len(tasks),digest))
 assert len(tasks)==17106
 started=now();tick=time.monotonic()
 with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:rows=list(pool.map(lambda t:transport.execute(*t),tasks))
 return {'schema':'p1339-independent-public-C-v1','authority_manifest_sha256':'842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b','plan_sha256':sha(D/'p1339-ab-batch1-cli-plan.json'),'phase':'calibration','executor':'/root/p1311_review','regime':'executado sem atestacao de isolamento','start_utc':started,'end_utc':now(),'wall_seconds':time.monotonic()-tick,'processes':len(rows),'binaries':{k:{'path':p,'sha256':h} for k,(p,h) in binaries.items()},'output_directory':str(output),'transport':pin(D/'p1339-ab-executor-v4.py'),'rows':rows}

def mutant_matrix():
 registry=read(D/'p1339-mutation-registry.json');negative=read(checked(registry['negative_oracles']))
 reference=checked(registry['pinned_reference']);pure=checked(registry['architecture_witness']['control']['binary']);multi=checked(registry['multiplex']['binary'])
 env={k:os.environ[k] for k in ['PATH','HOME','LANG','TZ','FONTCONFIG_FILE','FONTCONFIG_PATH','TYPST_FONT_PATHS'] if k in os.environ}
 env.update(LC_ALL='C.UTF-8',NO_COLOR='1',TERM='dumb');jobs=[]
 pairs=list(zip(negative['cases'],registry['mutants']))
 for order in ORDERS:
  for profile in PROFILES:
   for case,mutant in reversed(pairs) if order=='reverse' else pairs:
    assert mutant['id']==f"M{case['family']:02d}"
    for role,binary,mode in [('pinned',reference,0),('pure_control',pure,0),('mode0',multi,0),('mutant',pathlib.Path(mutant['binary']['path']),mutant['mode'])]:
     jobs.append((case['id'],mutant['id'],case['expr'],role,str(binary),mode,profile,order))
   jobs.append(('invalid-mode',None,'1','invalid_mode',str(multi),99,profile,order))
 assert len(jobs)==972
 def run(job):
  case,family,expression,role,binary,mode,profile,order=job;run_env={**env,'P1339_MUTANT':str(mode)}
  argv=[binary,'--color=never','eval',expression,'--format','json',*PROFILES[profile]]
  row={'case_id':case,'family':family,'role':role,'profile':profile,'order':order,'source':expression,'source_sha256':hashlib.sha256(expression.encode()).hexdigest(),'binary_sha256':sha(binary),'argv':argv,'env':run_env,'cwd':str(ROOT),'start_utc':now()};tick=time.monotonic()
  try:
   p=subprocess.run(argv,cwd=ROOT,env=run_env,capture_output=True,timeout=30);out,err,code=p.stdout,p.stderr,p.returncode
   row.update(exit=code,signal=-code if code<0 else None,execution='Observed' if code in (0,1) else 'Unknown',unknown_reason=None if code in (0,1) else 'abnormal_exit')
  except subprocess.TimeoutExpired as e:
   out,err=e.stdout or b'',e.stderr or b'';row.update(exit=None,signal=None,execution='Unknown',unknown_reason='timeout')
  except OSError as e:
   out,err=b'',str(e).encode();row.update(exit=None,signal=None,execution='Unknown',unknown_reason=type(e).__name__)
  row.update(stdout=out.decode('utf-8','surrogateescape'),stderr=err.decode('utf-8','surrogateescape'),stdout_base64=base64.b64encode(out).decode(),stderr_base64=base64.b64encode(err).decode(),end_utc=now(),seconds=time.monotonic()-tick)
  return row
 started=now();tick=time.monotonic()
 with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:rows=list(pool.map(run,jobs))
 return {'schema':'p1339-independent-mutant-C-v1','start_utc':started,'end_utc':now(),'wall_seconds':time.monotonic()-tick,'processes':len(rows),'registry':pin(D/'p1339-mutation-registry.json'),'rows':rows}

def judge_mutants(raw):
 rows=raw['rows'];comparisons=[];fail=[]
 def obs(r):return tuple(r[k] for k in ['execution','exit','stdout_base64','stderr_base64'])
 for family in [f'M{i:02d}' for i in range(1,21)]:
  for profile in PROFILES:
   previous={}
   for order in ORDERS:
    group={r['role']:r for r in rows if (r['family'],r['profile'],r['order'])==(family,profile,order)}
    assert set(group)=={'pinned','pure_control','mode0','mutant'}
    ref,pure,zero,mutant=[group[k] for k in ['pinned','pure_control','mode0','mutant']]
    controls=ref['execution']=='Observed' and obs(ref)==obs(pure)==obs(zero)
    if not controls:outcome='invalid_control'
    elif family=='M20':outcome='mandatory_unknown' if mutant['execution']=='Unknown' and mutant.get('signal')==6 else 'M20_not_discriminated'
    elif family=='M12':outcome='equal_runtime_structural_rejection' if obs(ref)==obs(mutant) else 'M12_unexpected_delta'
    elif mutant['execution']!='Observed':outcome='unknown_not_a_semantic_kill'
    else:outcome='observable_violation' if obs(ref)!=obs(mutant) else 'survived'
    valid=outcome in ['mandatory_unknown','equal_runtime_structural_rejection','observable_violation']
    if not valid:fail.append([family,profile,order,outcome])
    for role,r in group.items():
     if role in previous and previous[role]!=obs(r):fail.append([family,profile,order,role,'nondeterministic'])
     previous[role]=obs(r)
    comparisons.append({'family':family,'profile':profile,'order':order,'controls_exact':controls,'raw_mutant_execution':mutant['execution'],'gate':'Violated' if valid else 'NotRejected','reason':outcome})
 invalid=[r for r in rows if r['role']=='invalid_mode']
 assert len(invalid)==12
 for r in invalid:
  if r['exit'] in [None,0] or not r['stderr']:fail.append(['invalid_mode',r['profile'],r['order'],'not_fail_closed'])
 return comparisons,fail

def main():
 parser=argparse.ArgumentParser();parser.add_argument('--focal-acceptance',required=True);parser.add_argument('--focal-acceptance-sha256',required=True);parser.add_argument('--execute',action='store_true');args=parser.parse_args()
 assert args.execute,'explicit execution required; no default run'
 acceptance=read(checked({'path':args.focal_acceptance,'sha256':args.focal_acceptance_sha256}))
 assert acceptance['verdict']=='PASS_FOCAL_BATCH1'
 focal=read(checked(acceptance['raw']));derived=read(checked(acceptance['derived']))
 transport=module('p1339_frozen_transport','p1339-ab-executor-v4.py');predicate=module('p1339_frozen_predicate','p1339-ab-batch1-public-predicate-v2.py')
 plan=read(D/'p1339-ab-batch1-cli-plan.json');inventories=validate_inputs(plan,transport);before=state()
 for name in ['p1339-verifier-full-c-start-r1.json','p1339-verifier-full-c-public-r1.json','p1339-verifier-full-c-mutants-r1.json','p1339-verifier-full-c-comparison-r1.json']:assert not (D/name).exists(),name
 start=write_new('p1339-verifier-full-c-start-r1.json',{'schema':'p1339-complete-C-budget-start-v1','utc':now(),'runner':pin(__file__),'focal_acceptance':pin(args.focal_acceptance),'before':before,'inventories':inventories,'full_preseal_run_number':1,'full_preseal_limit':2,'public_planned_processes':17106,'mutant_planned_processes':972,'future_executed':0,'automatic_retries':0})
 resource.setrlimit(resource.RLIMIT_CORE,(0,0))
 public=public_matrix(plan,transport);public_pin=write_new('p1339-verifier-full-c-public-r1.json',public)
 mutants=mutant_matrix();mutant_pin=write_new('p1339-verifier-full-c-mutants-r1.json',mutants)
 failures=[];results=[]
 try:results=predicate.check(public,'calibration',[c['id'] for c in plan['cases']],focal,derived)
 except Exception as e:failures.append(['public_checker',type(e).__name__,str(e)])
 for r in results:
  allowed=r['classification']=='Preserved' or (r['classification']=='Unknown' and r['reason']=='conditional_angle_receiver_unactivated') or (r['classification']=='Unknown' and r['reason']=='declared_observation_budget_opacity' and r.get('control_satisfied') is True)
  if not allowed:failures.append(['public',r])
 if len(results)!=17106:failures.append(['public_result_count',len(results)])
 comparisons,negative_failures=judge_mutants(mutants);failures.extend(negative_failures)
 validate_inputs(plan,transport);after=state()
 record={'schema':'p1339-independent-complete-C-comparison-v1','utc':now(),'regime':'executado sem atestacao de isolamento','start':start,'runner':pin(__file__),'raw_public':public_pin,'raw_mutants':mutant_pin,'focal_acceptance':pin(args.focal_acceptance),'before':before,'after':after,'public_results':results,'mutant_comparisons':comparisons,'failures':failures,'full_preseal_runs_used':1,'full_preseal_limit':2,'future_ledger':{'definitions':119,'raw_cells_due_F':1428,'execution_C':0,'credit_C':0,'state':'NOT_EXECUTED_PRESEAL'},'M12_structural_basis':pin(D/'p1339-verifier-mutant-focal-review-r1.md'),'eligible_negative_families':20,'mutation_score':None if negative_failures else 1.0,'PASS_COMPLETE_C_COMPONENTS':not failures,'seal':None,'limits':['No future runtime credited.','No general equivalence or isolation attestation.','M12 graph proof conjunctive; runtime equality alone never rejection.','M20 raw SIGABRT Unknown gate-rejected, never preservation.','Independent final seal decision still required.']}
 final=write_new('p1339-verifier-full-c-comparison-r1.json',record)
 print(json.dumps({'receipt':final,'public_processes':len(public['rows']),'mutant_processes':len(mutants['rows']),'failures':len(failures),'PASS_COMPLETE_C_COMPONENTS':not failures}),flush=True)
 raise SystemExit(1 if failures else 0)

if __name__=='__main__':main()
