"""Phase-D selection transport only: JSON array -> frozen execute/checked_inputs.
Never changes the plan, case IDs, source, expectation, compiler or frozen runner.
"""
import argparse, concurrent.futures, hashlib, importlib.util, json, tempfile, time
from pathlib import Path
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('p1339_frozen_executor',D/'p1339-ab-executor-v4.py')
frozen=importlib.util.module_from_spec(spec);spec.loader.exec_module(frozen)
SEAL_SHA='35f00c4b9e15a010692017f5083ea4f451f4104a3730022136972e151ac0a8ee'
PLAN_SHA='e36f89070d5de5c9ac9e340f443255797aea109a42b8cfe71e448bc34dcb380b'
def sha_stream(path):
 h=hashlib.sha256()
 with Path(path).open('rb') as f:
  for b in iter(lambda:f.read(1024*1024),b''):h.update(b)
 return h.hexdigest()
def verify_sealed(seal):
 start=frozen.utc();tick=time.monotonic()
 def one(p):
  actual=sha_stream(p['path'])
  return {'path':p['path'],'expected':p['sha256'],'actual':actual,'valid':actual==p['sha256']}
 with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:checks=list(pool.map(one,seal['immutable_inputs']))
 return {'start_utc':start,'end_utc':frozen.utc(),'seconds':time.monotonic()-tick,'checks':checks,'valid':all(x['valid'] for x in checks),'read_boundary':'Exact sealed byte hashes only; no source interpreted or emitted.'}
def main():
 p=argparse.ArgumentParser();p.add_argument('--selection',required=True);p.add_argument('--selection-sha256',required=True);p.add_argument('--output',required=True);a=p.parse_args()
 assert frozen.sha(a.selection)==a.selection_sha256,'selection identity'
 adapter_sha=frozen.sha(__file__)
 selection=frozen.read(a.selection);ids=selection['ids'];assert isinstance(ids,list) and len(ids)==124 and len(set(ids))==124 and all(isinstance(x,str) for x in ids)
 sealpath=D/'p1339-seal.json';assert frozen.sha(sealpath)==SEAL_SHA
 seal=frozen.read(sealpath);assert seal['verdict']=='SEALED_R3_C_SCOPED'
 assert selection['seal']=={'path':str(sealpath),'sha256':SEAL_SHA}
 planpath=D/'p1339-ab-batch2-cli-plan.json';assert frozen.sha(planpath)==PLAN_SHA
 assert selection['plan']=={'path':str(planpath),'sha256':PLAN_SHA}
 assert frozen.sha(frozen.__file__)==selection['frozen_executor']['sha256'],'frozen executor identity'
 output=Path(a.output).resolve();assert output.parent==D and output.name=='p1339-ab-red-lossless-runs.json' and not output.exists()
 plan=frozen.read(planpath);frozen.checked_inputs(plan)
 own_oracles=frozen.read(seal['active_oracles']['positive']['path'])['oracles']
 discovery=sorted(x['id'] for x in own_oracles if x['id'].startswith('historical-discovery-'))
 controls=sorted(x['id'] for x in own_oracles if x['reference_policy']=='baseline')
 assert ids==sorted(set(discovery+controls)) and discovery==selection['discovery_ids'] and controls==selection['control_ids'],'exact previously approved subset'
 wanted=set(ids);assert wanted<={c['id'] for c in plan['cases']}
 cases=[c for c in plan['cases'] if c['id'] in wanted]
 assert len(cases)==124
 binaries={product:tuple([value['path'],value['sha256']]) for product,value in seal['binaries'].items()}
 assert set(binaries)=={'vanilla','baseline'} and binaries==frozen.BINARIES
 assert all(frozen.sha(path)==h for path,h in binaries.values()),'binary identity drift'
 print('Lossless D: verifying sealed inputs; no compiler started yet',flush=True)
 integrity_before=verify_sealed(seal);assert integrity_before['valid'],'sealed drift'
 before=frozen.state();tick=time.monotonic();outdir=Path(tempfile.mkdtemp(prefix='p1339-ab-red-'));tasks=[]
 for order in ['normal','repeat','reverse']:
  for profile in frozen.PROFILES:
   for c in reversed(cases) if order=='reverse' else cases:
    if profile not in c.get('profiles',list(frozen.PROFILES)):continue
    for product,(binary,digest) in binaries.items():tasks.append((c,binary,product,profile,order,outdir,len(tasks),digest))
 cells=[{'case_id':t[0]['id'],'product':t[2],'profile':t[3],'order':t[4]} for t in tasks]
 assert len(tasks)==2706 and cells==selection['cells'],'exact cell sequence'
 print('Lossless D: starting 2706 cells through unchanged frozen execute',flush=True)
 with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:rows=list(pool.map(lambda task:frozen.execute(*task),tasks))
 execution_seconds=time.monotonic()-tick
 frozen.checked_inputs(plan);assert all(frozen.sha(path)==h for path,h in binaries.values())
 print('Lossless D: Typst completed; rechecking sealed inputs',flush=True)
 integrity_after=verify_sealed(seal);assert integrity_after['valid'] and frozen.sha(sealpath)==SEAL_SHA
 assert frozen.sha(a.selection)==a.selection_sha256 and frozen.sha(__file__)==adapter_sha,'D transport drift'
 data={'schema':'p1339-ab-execution-v4-lossless-selection','authority_manifest_sha256':frozen.MANIFEST_SHA,'l0_freeze_sha256':frozen.L0_FREEZE_SHA,'plan_sha256':PLAN_SHA,
  'runner_sha256':frozen.sha(frozen.__file__),'adapter':{'path':str(Path(__file__).resolve()),'sha256':frozen.sha(__file__)},'selection':{'path':str(Path(a.selection).resolve()),'sha256':a.selection_sha256},
  'phase':'red','seal':{'path':str(sealpath),'sha256':SEAL_SHA},'regime':'executado sem atestacao de isolamento','executor':'/root/p1319_tests',
  'context':'Inherited P1319 independent tests; no P1339 productive/candidate source interpreted. Exact byte-hash validation of sealed inputs only.',
  'before':before,'after':frozen.state(),'binaries':{p:{'path':b,'sha256':h} for p,(b,h) in binaries.items()},'output_dir':str(outdir),'processes':len(rows),'wall_seconds':execution_seconds,'rows':rows,
  'integrity_before':integrity_before,'integrity_after':integrity_after,
  'mechanical_scope':'JSON ID array selects unchanged plan case objects; same frozen checked_inputs and execute functions, four workers, phases/profiles/orders/environment/cwd/sources/error/Unknown behavior. No run() override, fake IDs or string.split monkeypatch.'}
 frozen.save_new(output,data)
 print(json.dumps({'raw':{'path':str(output),'sha256':frozen.sha(output)},'processes':len(rows),'unknown':sum(r['execution']!='Observed' for r in rows),'wall_seconds':execution_seconds}),flush=True)
if __name__=='__main__':main()
