"""Replay frozen G language and prior F policies against one exact G build."""
from pathlib import Path
import datetime,json,subprocess,hashlib
from collections import Counter
P=Path(__file__).resolve().parent;F=P.parent/'figure-kind-implementation';product=Path('lab/target/product/debug/typst');build=P/'candidate-build-3/receipt.json'
out=P/'acceptance-3';out.mkdir(exist_ok=False);records=[]
sha=lambda path:hashlib.sha256(Path(path).read_bytes()).hexdigest()
REFERENCE_GROUPS=[('', 'render-order-policies-v2.json'),('additional-', 'additional-render-order-policies-v2.json'),('targeted-', 'targeted-render-order-policies-v2.json'),('origin-', 'origin-policies.json'),('origin-edge-', 'origin-edge-policies.json'),('show-origin-selected-', 'show-origin-selected-policies.json')]
def require_rows(rows,names):
 assert [row['name'] for row in rows]==names, 'missing, duplicate, reordered or extra case rows'
def require_distribution(rows,field,expected):
 assert Counter(row[field] for row in rows)==Counter(expected), 'case-class distribution differs'
def verify_f_policy_receipt(receipt,manifest,build_receipt):
 require_rows(receipt['cases'],[c['name'] for c in manifest])
 require_distribution(manifest,'disposition',{'required-closure':77,'preserved-known-debt':5,'negative-control-only':2})
 require_distribution(receipt['cases'],'disposition',{'required-closure':77,'preserved-known-debt':5,'negative-control-only':2})
 assert receipt['passed'] and not receipt['changed_inputs']
 assert receipt['binaries']['product']['sha256']==build_receipt['product_sha256']
 assert receipt['binaries']['oracle']['sha256']=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
 for actual,expected in zip(receipt['cases'],manifest):
  assert actual['passed'] is True
  assert actual['source_sha256']==expected['source_sha256']
  assert actual['expected']==expected['expected_at_closure']
  if expected['disposition']=='required-closure':assert actual['observed']==expected['oracle']
  elif 'policy' in expected['expected_at_closure']:
   assert actual['observed'] in [expected['oracle'],{'returncode':1,'error':expected['expected_at_closure']['known_diagnostic']}]
  else:assert actual['observed']==expected['expected_at_closure']
def verify_reference_receipts(build_receipt):
 base=F/'agents/entities/reference-acceptance';folder=P/'reference-corpora-3'
 aggregate=json.loads((folder/'receipt.json').read_text())
 assert len(aggregate['commands'])==6 and not aggregate['changed_inputs'] and aggregate['binary_unchanged']
 assert aggregate['build_sha256']==sha(build) and aggregate['product_sha256']==build_receipt['product_sha256']
 assert aggregate['inputs']==build_receipt['inputs']
 classes=[]
 for index,(prefix,policy_name) in enumerate(REFERENCE_GROUPS):
  matrix_path=base/(prefix+'matrix.json');oracle_path=base/(prefix+'oracle/receipt.json');policy_path=base/policy_name
  matrix=json.loads(matrix_path.read_text());oracle=json.loads(oracle_path.read_text());policy=json.loads(policy_path.read_text())
  rows=json.loads((folder/(prefix.rstrip('-') or 'main')/'receipt.json').read_text())
  names=[c['name'] for c in matrix['cases']]
  require_rows(oracle['cases'],names);require_rows(rows['cases'],names)
  assert oracle['matrix_sha256']==sha(matrix_path)
  assert rows['matrix_sha256']==sha(matrix_path) and rows['oracle_receipt_sha256']==sha(oracle_path) and rows['policies_sha256']==sha(policy_path)
  assert rows['build_receipt_sha256']==sha(build) and rows['product_sha256']==build_receipt['product_sha256']
  assert rows['required_failures']==[]
  command=aggregate['commands'][index]
  assert command['exit']==0 and command['required_failures']==[]
  assert command['argv'][command['argv'].index('--matrix')+1]==prefix+'matrix.json'
  for actual,case,expected in zip(rows['cases'],matrix['cases'],oracle['cases']):
   assert actual['policy']==policy['cases'][case['name']]
   assert sha(base/(case['name']+'.typ'))==expected['source_sha256']
   assert all(sha(base/name)==digest for name,digest in expected['extra_sha256'].items())
   classes.append({'class':actual['policy']['class']})
   if actual['policy']['class'].startswith('required-'):
    assert actual['pass'] is True and actual['checks'] and all(actual['checks'].values())
    if actual['policy']['class']=='required-language':
     assert actual['runs'][0]['exit']==0
     if case['query']:assert actual['runs'][1]['exit']==0
 require_distribution(classes,'class',{'required-language':27,'required-error':12,'required-termination':1,'open-bilateral-gap':2,'refuted-fixture':1})
 assert sum(c['class'].startswith('required-') for c in classes)==40
def verify_header_debt(actual,preserved):
 assert actual['name']==preserved['name']=='header-figure-boundary'
 assert actual['source_sha256']==preserved['source_sha256']
 assert actual['passed'] is False and preserved['passed'] is False
 assert all(actual[field]==preserved[field] for field in ('returncode','text','stderr')), 'known header debt changed observable behavior'
def run(name,argv,expected_exit=0):
 start=datetime.datetime.now(datetime.timezone.utc).isoformat();print(start,'START',name,flush=True)
 with (out/(name+'.log')).open('w') as f:r=subprocess.run(argv,stdout=f,stderr=subprocess.STDOUT)
 records.append({'name':name,'utc_start':start,'utc_end':datetime.datetime.now(datetime.timezone.utc).isoformat(),'argv':argv,'exit':r.returncode,'expected_exit':expected_exit});(out/'commands.json').write_text(json.dumps(records,indent=2)+'\n');print(name,r.returncode,flush=True)
 if r.returncode!=expected_exit:print((out/(name+'.log')).read_text()[-7000:],flush=True);raise SystemExit(1)
build_receipt=json.loads(build.read_text());assert build_receipt['exit']==0 and not build_receipt['changed_inputs'] and sha(product)==build_receipt['product_sha256']
run('g-hash',['python3',str(P/'hash_measurement_candidate_3.py')])
run('g-equality',['python3',str(P/'agents/core/replay_equality.py'),'--product',str(product),'--build-receipt',str(build),'--out',str(P/'equality-candidate-3')])
r=json.loads((P/'equality-candidate-3/receipt.json').read_text());assert not r['required_failures']
run('g-html',['python3',str(P/'agents/html/replay_cli.py'),'--product',str(product),'--build-receipt',str(build),'--output',str(P/'html-candidate-3')])
run('f-policies',['python3',str(F/'replay.py'),'--product',str(product),'--build-receipt',str(build),'--output',str(P/'figure-policy-replay-3'),'--phase','candidate'])
f_receipt=json.loads((P/'figure-policy-replay-3/receipt.json').read_text());assert f_receipt['case_manifest_sha256']==sha(F/'cases.json')
verify_f_policy_receipt(f_receipt,json.loads((F/'cases.json').read_text())['cases'],build_receipt)
run('f-references',['python3',str(F/'run_reference_acceptance.py'),'--build',str(build),'--output',str(P/'reference-corpora-3')])
verify_reference_receipts(build_receipt)
run('f-targets',['python3',str(F/'target_controls.py'),'--product',str(product),'--build-receipt',str(build),'--output',str(P/'figure-target-controls-3')],expected_exit=1)
r=json.loads((P/'figure-target-controls-3/receipt.json').read_text());assert len(r['cases'])==11 and [c['name'] for c in r['cases'] if not c['passed']]==['header-figure-boundary']
target_oracle=F/'target-controls-oracle/receipt.json';target_cases=json.loads(target_oracle.read_text())['cases']
require_rows(r['cases'],[c['name'] for c in target_cases])
assert r['oracle_receipt_sha256']==sha(target_oracle) and r['product']['sha256']==build_receipt['product_sha256'] and not r['changed_inputs']
assert all(actual['source_sha256']==expected['source_sha256'] for actual,expected in zip(r['cases'],target_cases))
header_history=F/'candidate-target-controls-4/receipt.json';assert sha(header_history)=='f79d6f40107bdbceec8df9ec4da7391a393431fd41569f6179652f6f52d2d922'
verify_header_debt(next(c for c in r['cases'] if c['name']=='header-figure-boundary'),next(c for c in json.loads(header_history.read_text())['cases'] if c['name']=='header-figure-boundary'))
print('PASS required G/F acceptance; known header-figure-boundary remains a strict failure.',flush=True)
