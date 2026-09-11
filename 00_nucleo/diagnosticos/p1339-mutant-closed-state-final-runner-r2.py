"""Frozen phase-F runner r2 (same batch1; lifecycle v3 successor). Actual test binaries only; no preseal runtime credit.

Runs separate L1/L3 test bindings, preserves raw channels, rejects missing rows,
and invokes the independent lifecycle and opaque predicate programs verbatim.
"""
import argparse, base64, collections, datetime, hashlib, importlib.util
import json, os, pathlib, resource, subprocess

ROOT=pathlib.Path(__file__).resolve().parents[2]
D=ROOT/'00_nucleo/diagnosticos'
PINS={
 'p1339-mutant-closed-state-final-runner.py':'529f6ee7c22e847f8035bdd3896714f6c91fa7ef303987358f1656716aca3440',
 'p1339-ab-batch1-retention-lifecycle-typed-r2.json':'a1f5cf0e93723dde9c3bb317929e2e50fcdbc839475ec1d961fc661567a422bb',
 'p1339-ab-batch1-lifecycle-predicate-v2.py':'b7115b78a61d4b07391f46d50d9b0ffba0244a9571f7b7eb48f617f8c6517209',
 'p1339-ab-batch1-coverage-map-r2.json':'ad95dd42ce102810a75986511c67a996286ec78a486c11e87faf50cd54aa51ea',
 'p1339-ab-closed-api-fixtures-r1.json':'dda8618c9ede3640fab05d6cd4278f6c766c3410e4a646561f4bd32436f20dd5',
 'p1339-ab-private-relation-fixtures-r1.json':'2e57f6141bd90395cdf8989ab308f7017e29fc2ad3ddd48ec5bf3e3938a78845',
 'p1339-ab-batch1-projection-fixtures-r2.json':'a1645ed5b13e604456d38bede6fce98463fa98d9312434bc0a71534c3686716b',
 'p1339-ab-batch1-public-opaque-fixture-r2.json':'fe1ffadc4d74dc21a95298171e606ad69ada8ac2495e554bb0dafeb2e43c36d3',
 'p1339-ab-batch1-same-context-style-fixture.json':'70d572c86aee6499c0f5ebf20849187d4113ebc6016888f1e0b4a0ea43d572aa',
 'p1339-ab-batch1-retention-lifecycle-typed-r3.json':'6d8c257997631133fb1a7a55816daaa50e1633c129fafd17f6d1ddeb378ca541',
 'p1339-ab-batch1-lifecycle-predicate.py':'8cae4f1719a9310d4e627bc85bfe5d2589bb411af1f1527d8a872e2a7a94ed14',
 'p1339-ab-batch1-lifecycle-predicate-v3.py':'715adff3a42c6be12aaedf582c4c7846d30c226c7be45afbfaa432889dfbf60f',
 'p1339-ab-batch1-opaque-predicate.py':'2fd325da895f0af22d307eabfa7d805311fd40d49cad7a24ea9d5984f2fa4c6c',
 'p1339-mutant-closed-state-api-harness.rs':'09c467410cb0f4e37d11dca47c15f4913271f74e7b1400457513d76cd9c373c0',
 'p1339-mutant-closed-state-relation-harness.rs':'320299b464c2c8ab47a206a7ff15e806afd595d7bdb1dc7488432d856ab6b289',
 'p1339-mutant-closed-state-public-opaque-harness-r2.rs':'6838a1c00c0004bb3fb0cd282efa196c3c58c85b91c4ea64fb345de10558e8d7',
 'p1339-mutant-closed-state-style-harness.rs':'2f01811e5ae26ea7ed5650adcf359c47b00afdbdd22089bd0775cd6d6f7d8f21',
 'p1339-mutant-closed-state-projection-harness.rs':'27c1e4d8ee3f0bbfe689eb2af82dd0451d7e34cfdaf900b80f37462dae6bfcc2',
 'p1339-mutant-closed-state-lifecycle-collector.rs':'d8e07c060162976895e54ebf55dc12e5b51e5234aa173f09f234f82a2cc38f1b',
 'p1339-mutant-closed-state-core-wrapper.rs':'1b316c6a1d00feca89d5d2c1894caac6a2ba2acae212b3d2c4820b95cbadfa5c',
 'p1339-contract-r3.json':'c0cd1826679ddaf77e937a677839c5cbe64fdae6ec71bfa34e635d584d9c3e17',
 'p1339-authority-manifest-r2.json':'842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b',
 'p1339-l0-freeze.json':'397c136fc8710d44b2f7537193fe5b9c44ab89d40a99bf6b994296e05e7c4d04',
 'p1339-budget-redesign-r1.json':'4bdd984d7b9d482d68ebb787e98eeed8333a1d4447c637615d4ceb9c78ed6c19',
 'p1339-verifier-budget-redesign-acceptance-r1.md':'0552cdbfb02e18496ac3c07121e0640fad2a772aa32e678a0b7062344286bafd',
}
PROFILES=['default','html','a11y','html+a11y']
ORDERS=['normal','repeat','reverse']
GROUPS={
 'P1339_CLOSED_API':('p1339-ab-closed-api-fixtures-r1.json','id',312),
 'P1339_PRIVATE_RELATION':('p1339-ab-private-relation-fixtures-r1.json','case',852),
 'P1339_PUBLIC_OPAQUE':('p1339-ab-batch1-public-opaque-fixture-r2.json','id',12),
 'P1339_SAME_CONTEXT_STYLE':('p1339-ab-batch1-same-context-style-fixture.json','case_id',12),
 'P1339_EXTRA_PROJECTION':('p1339-ab-batch1-projection-fixtures-r2.json','id',96),
 'P1339_LIFECYCLE':('p1339-ab-batch1-retention-lifecycle-typed-r3.json','case_id',144),
}

def sha(p):return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def pin(p):return {'path':str(pathlib.Path(p).resolve()),'sha256':sha(p)}
def read(p):return json.loads(pathlib.Path(p).read_text())
def write_new(p,x):
 with pathlib.Path(p).open('x') as f:json.dump(x,f,ensure_ascii=False,indent=2);f.write('\n')
def load_module(name,file):
 spec=importlib.util.spec_from_file_location(name,D/file)
 module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module);return module
def disable_core():resource.setrlimit(resource.RLIMIT_CORE,(0,0))

def run(binary,filter_name,config,audit,timeout):
 binary=pathlib.Path(binary).resolve();identity=pin(binary)
 delta={'P1339_TEST_BINARY_SHA256':identity['sha256'],
        'P1339_COMPILED_CONFIGURATION_JSON':json.dumps(config,separators=(',',':')),
        'P1339_COMPILED_CONFIGURATION_SHA256':hashlib.sha256(json.dumps(config,separators=(',',':')).encode()).hexdigest(),
        'P1339_BINDING_AUDIT_JSON':json.dumps(audit,separators=(',',':'))}
 argv=[str(binary),filter_name,'--nocapture','--test-threads=1']
 started=utc()
 try:
  p=subprocess.run(argv,cwd=ROOT,env={**os.environ,**delta},capture_output=True,timeout=timeout,preexec_fn=disable_core)
  out,err,code=p.stdout,p.stderr,p.returncode;transport='Completed' if code>=0 else 'Signal'
 except subprocess.TimeoutExpired as e:
  out,err,code=e.stdout or b'',e.stderr or b'',None;transport='Timeout'
 return {'start':started,'end':utc(),'argv':argv,'cwd':str(ROOT),'binary':identity,'environment_delta':delta,
         'core_limit':0,'timeout_seconds':timeout,'transport':transport,'exit':code,
         'signal':-code if code is not None and code<0 else None,
         'stdout':out.decode('utf-8','replace'),'stderr':err.decode('utf-8','replace'),
         'stdout_base64':base64.b64encode(out).decode(),'stderr_base64':base64.b64encode(err).decode(),
         'stdout_sha256':hashlib.sha256(out).hexdigest(),'stderr_sha256':hashlib.sha256(err).hexdigest()}

def extract(raw,prefix):
 rows=[];marker=prefix+' '
 text=base64.b64decode(raw['stdout_base64']).decode('utf-8',errors='strict')
 for line in text.splitlines():
  if marker not in line:continue
  start=line.index(marker)
  leading=line[:start]
  if leading and not (leading.startswith('test ') and leading.endswith(' ... ')):continue
  rows.append(json.loads(line[start+len(marker):]))
 return rows

def exact_set(prefix,rows):
 file,key,count=GROUPS[prefix];fixture=read(D/file)
 cases=fixture.get('cases',[fixture]);ids=[x['id'] for x in cases]
 assert len(ids)==len(set(ids)),(prefix,'duplicate frozen IDs')
 expected={(i,p,o) for i in ids for p in PROFILES for o in ORDERS}
 actual=[(x[key],x['profile'],x['order']) for x in rows]
 assert len(rows)==count and len(actual)==len(set(actual)) and set(actual)==expected,(prefix,'missing/duplicate/unexpected raw cells')

def main():
 p=argparse.ArgumentParser()
 p.add_argument('--core-test-bin',required=True);p.add_argument('--pipeline-test-bin',required=True)
 p.add_argument('--compiled-config',required=True);p.add_argument('--binding-audit',required=True)
 p.add_argument('--output-dir',required=True);p.add_argument('--timeout',type=int,default=1200)
 a=p.parse_args();assert a.timeout>0
 for name,h in PINS.items():assert sha(D/name)==h,('frozen pin drift',name)
 config=read(a.compiled_config);audit=read(a.binding_audit)
 assert isinstance(config,dict) and config,'actual build configuration required'
 assert audit['status']=='accepted_real_product_path','independent final binding audit required before execution'
 assert audit['port_source_pins'],'all real private/pipeline hook source pins required'
 for item in audit['port_source_pins']:assert sha(item['path'])==item['sha256'],('audited binding source drift',item['path'])
 output=pathlib.Path(a.output_dir).resolve();output.mkdir(exist_ok=False)
 core=run(a.core_test_bin,'p1339_frozen_core_bound_matrix',config,audit,a.timeout)
 write_new(output/'core-raw.json',core)
 pipeline=run(a.pipeline_test_bin,'p1339_frozen_pipeline_bound_matrix',config,audit,a.timeout)
 write_new(output/'pipeline-raw.json',pipeline)
 failures=[];groups={};checks=[]
 for label,raw in [('core',core),('pipeline',pipeline)]:
  if raw['transport']!='Completed' or raw['exit']!=0:failures.append([label,'test process failed',raw['transport'],raw['exit']])
 for prefix in GROUPS:
  try:
   groups[prefix]=extract(pipeline if prefix=='P1339_LIFECYCLE' else core,prefix)
   exact_set(prefix,groups[prefix])
  except Exception as e:failures.append([prefix,type(e).__name__,str(e)])
 lifecycle=load_module('p1339_frozen_lifecycle','p1339-ab-batch1-lifecycle-predicate-v3.py')
 opaque=load_module('p1339_frozen_opaque','p1339-ab-batch1-opaque-predicate.py')
 cases={c['id']:c for c in read(D/'p1339-ab-batch1-retention-lifecycle-typed-r3.json')['cases']}
 for row in groups.get('P1339_LIFECYCLE',[]):
  try:
   assert row['binary_sha256']==pipeline['binary']['sha256'],'wrong actual test binary identity'
   assert row['compiled_configuration']==config,'wrong actual compiled configuration'
   assert row['port_source_pins']==audit['port_source_pins'],'wrong audited port identity'
   checks.append(lifecycle.check(cases[row['case_id']],row))
  except Exception as e:failures.append(['lifecycle',row.get('case_id'),type(e).__name__,str(e)])
 for row in groups.get('P1339_PUBLIC_OPAQUE',[]):
  try:assert opaque.assert_opaque(row) is True
  except Exception as e:failures.append(['opaque',row.get('id'),type(e).__name__,str(e)])
 for prefix in ['P1339_PRIVATE_RELATION','P1339_PUBLIC_OPAQUE']:
  for row in groups.get(prefix,[]):
   if row['actual_operation_identity']['binary_sha256']!=core['binary']['sha256']:
    failures.append([prefix,'wrong actual core binary identity'])
 aggregate={'schema':'p1339-frozen-closed-state-execution-v1','at':utc(),
   'regime':'executado sem atestação de isolamento','phase':'F','runner':pin(__file__),
   'pins':PINS,'compiled_configuration':pin(a.compiled_config),'binding_audit':pin(a.binding_audit),
   'raw_receipts':[pin(output/'core-raw.json'),pin(output/'pipeline-raw.json')],
   'counts':{k:len(v) for k,v in groups.items()},'expected_counts':{k:v[2] for k,v in GROUPS.items()},
   'lifecycle_checks':checks,'failures':failures,'PASS_CLOSED_TESTS_ONLY':not failures,
   'limits':['Not a seal, not independent PASS_SCOPED, no architectural-witness/CLI/workspace closure inferred.',
             'L1/L3 binding is test-only and must be independently audited on actual productive source graph.',
             'Raw abnormal/missing outputs are failures, never preservation or positive opacity credit.']}
 write_new(output/'aggregate.json',aggregate)
 print(json.dumps({'aggregate':pin(output/'aggregate.json'),'PASS_CLOSED_TESTS_ONLY':not failures,'failures':len(failures),'counts':aggregate['counts']}))
 raise SystemExit(0 if not failures else 1)

if __name__=='__main__':main()
