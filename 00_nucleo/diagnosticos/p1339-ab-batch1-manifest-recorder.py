"""Publish the single aggregate batch1 manifest after final adapter pins arrive.
This program only validates hashes and writes metadata. It never starts a test.
"""
import argparse, datetime, hashlib, json, re, subprocess
from pathlib import Path
R=Path('/repos/Antigravity/typst-crystalline');D=R/'00_nucleo/diagnosticos'
def digest(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def pin(name,expected=None):
 p=Path(name);p=p if p.is_absolute() else D/p
 if p.parent!=D:raise ValueError('non-diagnostic component '+str(p))
 actual=digest(p)
 if expected and actual!=expected:raise ValueError('component drift '+str(p))
 return {'path':str(p),'sha256':actual}
def read(name):return json.loads((D/name).read_text())
def git(*args):return subprocess.check_output(['git',*args],cwd=R,text=True).rstrip('\n')
def main():
 ap=argparse.ArgumentParser()
 for k in ['driver','driver-sha256','adapter-receipt','adapter-receipt-sha256','integration','integration-sha256']:
  ap.add_argument('--'+k,required=True)
 a=ap.parse_args()
 driver=pin(a.driver,a.driver_sha256);receipt=pin(a.adapter_receipt,a.adapter_receipt_sha256);integration=pin(a.integration,a.integration_sha256)
 # The receipt itself must bind the latest checker, rather than merely being
 # another unchanged filename/hash. Its schema is kept opaque here; independent
 # review validates semantics and actual implementation bindings.
 checker=pin('p1339-ab-batch1-lifecycle-predicate-v3.py','715adff3a42c6be12aaedf582c4c7846d30c226c7be45afbfaa432889dfbf60f')
 if checker['sha256'] not in Path(receipt['path']).read_text():raise ValueError('final adapter receipt does not pin checker-v3')
 authorities=[
  pin('p1339-authority-manifest-r2.json','842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b'),
  pin('p1339-l0-freeze.json','397c136fc8710d44b2f7537193fe5b9c44ab89d40a99bf6b994296e05e7c4d04'),
  pin('p1339-contract-r3.json','c0cd1826679ddaf77e937a677839c5cbe64fdae6ec71bfa34e635d584d9c3e17'),
  pin('p1339-budget-redesign-r1.json','4bdd984d7b9d482d68ebb787e98eeed8333a1d4447c637615d4ceb9c78ed6c19'),
  pin('p1339-verifier-budget-redesign-acceptance-r1.md','0552cdbfb02e18496ac3c07121e0640fad2a772aa32e678a0b7062344286bafd'),
  pin('p1339-implementation-observer-authority.json'),pin('p1339-closed-harness-authority.json'),
  pin('p1339-implementation-authorities.json'),pin('p1339-completion-authorization.md')]
 contract=read('p1339-contract-r3.json');freeze=read('p1339-l0-freeze.json');l0=[]
 for path,expected in contract['l0_normative_sha256'].items():
  p=R/path;data=p.read_bytes();lines=data.splitlines(keepends=True)
  boundary=next((i for i,l in enumerate(lines) if l in [b'\n',b'\r\n']),len(lines))
  matches=[i for i,l in enumerate(lines[:boundary]) if re.fullmatch(rb'Hash do C\xc3\xb3digo: [0-9a-f]{8}\r?\n',l)]
  if len(matches)!=1:raise ValueError('noncanonical L0 hash metadata '+path)
  normative=hashlib.sha256(b''.join(l for i,l in enumerate(lines) if i!=matches[0])).hexdigest()
  if normative!=expected:raise ValueError('normative L0 drift '+path)
  l0.append({'path':str(p),'raw_sha256':hashlib.sha256(data).hexdigest(),'normative_sha256':normative,'freeze_raw_sha256':freeze['l0_sha256'][path],'normative_policy':'remove only the one canonical Hash do Codigo preamble line; no other byte normalized'})
 if len(l0)!=31:raise ValueError('L0 cardinality')
 plan=read('p1339-ab-batch1-cli-plan.json');positive=read('p1339-positive-oracles.json');opaque=read('p1339-opaque-oracles.json')
 active=[
  'p1339-positive-oracles.json','p1339-opaque-oracles.json','p1339-ab-batch1-cli-plan.json','p1339-ab-batch1-cli-oracles.json',
  'p1339-ab-executor-v4.py','p1339-ab-batch1-public-predicate.py','p1339-ab-batch1-public-predicate-v2.py','p1339-ab-batch1-causal-recipe.py',
  'p1339-ab-closed-api-fixtures-r1.json','p1339-ab-private-relation-fixtures-r1.json','p1339-ab-batch1-projection-fixtures-r2.json',
  'p1339-ab-batch1-public-opaque-fixture-r2.json','p1339-ab-batch1-opaque-predicate.py','p1339-ab-batch1-same-context-style-fixture.json',
  'p1339-ab-batch1-retention-lifecycle-typed-r3.json','p1339-ab-batch1-lifecycle-predicate.py','p1339-ab-batch1-lifecycle-predicate-v2.py','p1339-ab-batch1-lifecycle-predicate-v3.py',
  'p1339-ab-batch1-coverage-map-r2.json','p1339-ab-publication-budget-ledger-r1.json','p1339-ab-batch1-publication-ledger-r1.json',
  'p1339-mutant-closed-state-interface.md','p1339-mutant-closed-state-private-interface.md','p1339-mutant-closed-state-style-signatures.md',
  'p1339-mutant-closed-state-inventory.json','p1339-mutant-closed-state-inventory-supplement.json','p1339-mutant-closed-state-inventory-closure.json','p1339-mutant-closed-state-inventory-resolution.json',
  'p1339-mutant-closed-state-api-harness.rs','p1339-mutant-closed-state-relation-harness.rs','p1339-mutant-closed-state-public-opaque-harness-r2.rs',
  'p1339-mutant-closed-state-projection-harness.rs','p1339-mutant-closed-state-style-harness.rs','p1339-mutant-closed-state-core-wrapper.rs','p1339-mutant-closed-state-lifecycle-collector.rs',
  'p1339-verifier-opacity-design-review-r1.md','p1339-ab-batch1-manifest-recorder.py']
 artifacts={pin(n)['path']:pin(n) for n in active}
 for p,expected in plan['input_sha256'].items():artifacts[p]=pin(p,expected)
 # Pin all source fixtures, public expectation reference files, and publication
 # ancestors already designated by the independent oracle; never walk a source
 # directory or arbitrary implementation receipt.
 for c in plan['cases']:
  if c.get('source_path'):artifacts[c['source_path']]=pin(c['source_path'],c['source_sha256'])
 for c in positive['oracles']+opaque['conditional_angle_controls']:
  e=c['expected_observation_or_reference_with_sha256']
  for prod,profiles in e.get('by_product',{}).items():
   for ref in profiles.values():artifacts[ref['artifact']]=pin(ref['artifact'],ref['sha256'])
 for p in D.glob('p1339-ab-batch1-*'):
  if p.is_file():artifacts[str(p)]=pin(p)
 for p in [driver,receipt,integration]+authorities:artifacts[p['path']]=p
 frozen_by_id={c['id']:c for c in plan['cases']};ids=plan['batch1_focal_ids']
 cells=[{'case_id':id,'product':product,'profile':profile,'order':order} for order in ['normal','repeat','reverse'] for profile in ['default','html','a11y','html+a11y'] for id in (list(reversed(ids)) if order=='reverse' else ids) for product in ['vanilla','baseline']]
 if len(ids)!=16 or len(cells)!=384:raise ValueError('focal scope cardinality')
 binaries={'vanilla':{'path':'/usr/local/bin/typst','sha256':'7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'},'baseline':{'path':'/tmp/p1338-target.vlNAmp/release/typst','sha256':'f7c8085f8453ecb6b10841b698092d41e7fbec64d9d46f9341b9b9b538bfefa1'}}
 for b in binaries.values():
  if digest(b['path'])!=b['sha256']:raise ValueError('binary drift')
 raw=str(D/'p1339-ab-batch1-focal-runs.json');derived=str(D/'p1339-ab-batch1-causal-derived-expectations.json')
 output=D/'p1339-ab-batch1-manifest.json'
 if any(Path(p).exists() for p in [str(output),raw,derived]):raise ValueError('immutable manifest or future raw path already exists')
 data={'schema':'p1339-aggregate-batch-manifest-v1','batch':1,'author':'/root/p1319_tests','regime':'executado sem atestacao de isolamento',
  'status':'FROZEN_PENDING_INDEPENDENT_MANIFEST_ACCEPTANCE_NO_EXECUTION_AUTHORITY',
  'published_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'provenance':{'head':git('rev-parse','HEAD'),'branch':git('branch','--show-current'),'working_tree':'uncommitted','status':git('status','--short'),'diff_stat':git('diff','HEAD','--stat')},
  'authority_manifest':authorities[0],'contract':authorities[2],'authorities':authorities,'l0':l0,
  'l0_read_limit':'Only frozen normative L0 and exact authorized step were read; no productive source/hash-line diff bodies. Productive state is independently verified by verifier, not self-attested by oracle author.',
  'binaries':binaries,'artifacts':list(artifacts.values()),'driver':driver,'adapter_receipt':receipt,'integration_recipe':integration,
  'oracle_author_role':'Inherited P1319 A/B context; candidate nonexistent and unread. Source read restrictions unchanged. No isolation attestation, no contract/solution/verdict write authority.',
  'hypotheses':[
   {'cause':'direct-show target incorrectly vanilla','delta':'Three direct-show candidate controls use baseline raw/effect while vanilla remains mutation host. Six static match/miss boundary controls unchanged.','gain_prediction':'All own-product C direct controls and six boundary controls stable; future candidate preserves declared baseline result, including bare text debt.','focal_ids':[id for id in ids if id.startswith('show-')]},
   {'cause':'where-l0-empty-vs-bare composed causal diagnostic','delta':'Freeze none+same-byte-padding control preserving later bare selector span, exact one-source-line derivation and three isolated conjunctive controls.','gain_prediction':'Baseline padded and isolated bare fail at same legacy error; vanilla allowed where/heading controls succeed; unique derivation preserves all diagnostic bytes except declared source line. No raw normalization.','focal_ids':[id for id in ids if id.startswith('causal-') or id=='where-l0-empty-vs-bare']},
   {'cause':'opacity was incorrectly assumed bilateral','delta':'Pin same1second workload to raw vanilla timeout Unknown and baseline exact known guard error; finite3second control separately positive.','gain_prediction':'Twelve vanilla Unknown workload cells, twelve baseline known error cells, twenty-four finite positive cells; stable normal/repeat/reverse. Opaque earns zero positive credit.','focal_ids':[id for id in ids if id.startswith('opaque-')]},
   {'cause':'future closed field/retention/lifecycle definitions incomplete','delta':'Joint immutable qualified inventory closure,71relation pairs,26public API cases,8projection cases,public opaque/styled request scenario,12typed lifecycle scenarios and exhaustive field/clause map with50exact propositions.','gain_prediction':'Independent structural review can identify concrete fixtures/asserts/ports for every required F clause; no actual C runtime coverage claimed from templates or inventory.','focal_ids':[],'phase':'F_NOT_EXECUTED_PRESEAL_ZERO_CREDIT'}],
  'existing_execution_slice':{'ids':ids,'cells':cells,'processes':384,'executor':pin('p1339-ab-executor-v4.py'),'argv':['python3',str(D/'p1339-ab-executor-v4.py'),'--plan',str(D/'p1339-ab-batch1-cli-plan.json'),'--phase','calibration','--products','vanilla,baseline','--orders','normal,repeat,reverse','--ids',','.join(ids),'--output',raw],
    'profiles':['default','html','a11y','html+a11y'],'env_recipe':'executor-v4 explicit allowlist and NO_COLOR=1 TERM=dumb PYTHONDONTWRITEBYTECODE=1; actual full env/cwd/argv recorded per row','source_policy':'same frozen named compile fixtures; eval literal bytes; no stdin compile workaround','artifact_observable':'PDF presence only; hashes provenance, never semantic equality'},
  'future_definition_slice':{'core_definitions':107,'core_required_F_raw_cells':1284,'pipeline_definitions':12,'pipeline_required_F_raw_cells':144,'execution_C':0,'credit_C':0,'status':'NOT_EXECUTED_PRESEAL',
    'fixtures':[pin(n) for n in ['p1339-ab-closed-api-fixtures-r1.json','p1339-ab-private-relation-fixtures-r1.json','p1339-ab-batch1-projection-fixtures-r2.json','p1339-ab-batch1-public-opaque-fixture-r2.json','p1339-ab-batch1-same-context-style-fixture.json','p1339-ab-batch1-retention-lifecycle-typed-r3.json']],
    'coverage_map':pin('p1339-ab-batch1-coverage-map-r2.json'),'rule':'Final actual owner-local relation discriminant/retention/attempt transcript plus both real core+pipeline test binaries and all checkers required. No dummy comparator, alternative orchestrator or L1-to-L3 import. No final NotDue/Unknown waiver.'},
  'pending_outputs':{'raw':{'path':raw,'status':'PENDING_MEASUREMENT','sha256':None,'credit':0},'derived':{'path':derived,'status':'PENDING_MEASUREMENT','sha256':None,'credit':0},'recipe':positive['pending_causal_recipe'],
    'comparison':'Use pinned public-predicate-v2 after raw and derived are newly pinned; any required Unknown, causal recipe failure, extra/missing cell or unstable observation blocks focal gate. Write new comparison/cost receipt, never edit existing expectations.'},
  'budget':{'redesign_batch_limit':2,'batches_executed_before_manifest':0,'this_batch_planned_processes':384,'automatic_semantic_retries':0,'complete_preseal_limit':2,'complete_preseal_used_all_roles':0,
    'per_process_timeout_seconds':{id:frozen_by_id[id].get('timeout_seconds',30) for id in ids},'maximum_sum_child_timeout_seconds':sum(frozen_by_id[c['case_id']].get('timeout_seconds',30) for c in cells),'worker_limit':4,
    'observed_cost':'PENDING_MEASUREMENT; preserve every technical failure and actual child/wall cost. Preparation metadata ledger is separate from compiler runtime.',
    'retry_policy':'Any failed process/publication is recorded, never free because draft/transport. No automatic retry. Second aggregate batch only on independently reviewed residual within closed list plus new causal hypothesis; unchanged failure/no gain stops.'},
  'gain_ledger_recipe':'Record actual positives preserved/violated, required opaque Unknown, known baseline guard error, all20mutants independently judged, cases corrected/regressed and dominant cause. Future definition acceptance is separate from runtime passes. Preserve old32opacity and all1472historical processes unchanged.',
  'remaining_gates':'This focal is not full discrimination, seal, RED or implementation permission. Full matrices still0/2; all20mutants,10absences,18scopeouts,seven supplements and every W/F obligation remain. No canonical RED before seal.',
  'acceptance_required':'Only independent verifier may accept this exact manifest SHA before the384process run. Later seal must pin raw+derived+comparison+all protected definitions; observer implementation authority remains dormant until seal plus genuine RED.'}
 body=json.dumps(data,indent=2,ensure_ascii=True)+'\n';patch='*** Begin Patch\n*** Add File: '+str(output)+'\n'+''.join('+'+x+'\n' for x in body.split('\n')[:-1])+'*** End Patch\n'
 subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True)
 print(pin(output))
if __name__=='__main__':main()
