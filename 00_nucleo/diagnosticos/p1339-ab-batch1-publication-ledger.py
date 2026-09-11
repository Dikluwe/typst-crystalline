"""Immutable factual component publication ledger; no semantic process run."""
import datetime, hashlib, json, subprocess
from pathlib import Path
D=Path(__file__).resolve().parent
def pin(p):return {'path':str(p),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
paths=sorted(D.glob('p1339-ab-batch1-*'))
paths += [D/'p1339-positive-oracles.json',D/'p1339-opaque-oracles.json']
records=[]
for p in paths:
 if p.is_file():
  entry=pin(p);entry.update({'bytes':p.stat().st_size,'filesystem_mtime_utc':datetime.datetime.fromtimestamp(p.stat().st_mtime,datetime.timezone.utc).isoformat(),'mtime_is_attested_publication_time':False})
  records.append(entry)
data={'schema':'p1339-batch1-component-ledger-v1','author':'/root/p1319_tests','regime':'executado sem atestacao de isolamento',
 'ledger_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
 'authority_manifest':pin(D/'p1339-authority-manifest-r2.json'),'contract':pin(D/'p1339-contract-r3.json'),
 'budget_redesign':pin(D/'p1339-budget-redesign-r1.json'),'budget_acceptance':pin(D/'p1339-verifier-budget-redesign-acceptance-r1.md'),
 'historical_ledger':pin(D/'p1339-ab-publication-budget-ledger-r1.json'),
 'scope':'All current oracle-author batch1 components and canonical wrappers; historical publications remain in predecessor ledger. Mechanical adversary components have independent receipts, later aggregate manifest pins them.',
 'publications':records,'process_accounting':{'semantic_processes_since_redesign':0,'aggregate_focal_batches_executed':0,'aggregate_focal_batches_limit':2,'full_preseal_matrices_executed':0,'full_preseal_matrices_limit':2,'canonical_RED':0,'future_core_api_executions':0,'future_pipeline_executions':0,'candidate_executions':0},
 'planned_next_semantic_slice':{'manifest_status':'NOT_PUBLISHED_NOT_ACCEPTED','case_ids':json.loads((D/'p1339-ab-batch1-cli-plan.json').read_text())['batch1_focal_ids'],'profiles':['default','html','a11y','html+a11y'],'products':['vanilla','baseline'],'orders':['normal','repeat','reverse'],'maximum_processes':384,'hypothesis':'Correct the two declared CLI causal policies and executable-specific opacity without changing R3; F fixtures/checkers/maps are frozen definitions, not preseal executions.'},
 'authoring_failures_and_corrections':[
  {'kind':'static authoring syntax defect','artifact':pin(D/'p1339-ab-batch1-prepare-coverage-map.py'),'correction':pin(D/'p1339-ab-batch1-prepare-coverage-map-r2.py'),'semantic_processes':0,'cause':'Invalid temporary source line detected before metadata script execution; corrected successor, predecessor retained.'},
  {'kind':'read-only metadata inspection failure','cause':'KeyError snapshot_projection while printing typed fixtures; complete dto_schema and structural witnesses were printed before the failure. Then exact per-case source/coordinates were read separately.','semantic_processes':0,'artifact_writes':0},
  {'kind':'metadata publication failure','artifact':pin(D/'p1339-ab-batch1-prepare-exact-lifecycle-map.py'),'correction':pin(D/'p1339-ab-batch1-finish-exact-lifecycle-map.py'),'cause':'List appended with += to existing dictionary after successful typed-r3 publication. Successor inserted witnesses keyed by id; no previous output modified.','wall_seconds':0.032817059,'semantic_processes':0},
  {'kind':'static provenance review successor','predecessor':pin(D/'p1339-ab-batch1-public-predicate.py'),'successor':pin(D/'p1339-ab-batch1-public-predicate-v2.py'),'cause':'Add strict binary/source/cwd/env/mode provenance wrapper before any evaluation of raw observations. No expectation or contract semantic change.','semantic_processes':0}],
 'observed_authoring_process_costs':[{'action':'finish exact lifecycle map metadata publication','wall_seconds':0.00062482},{'action':'publish canonical oracle metadata','wall_seconds':0.267068595}],
 'cost_limitations':'Metadata shell reads/hash computations and apply_patch costs are not aggregated as compiler execution; where tool wall time was not retained it is unavailable, not zero. No free semantic retry is claimed for draft labels.',
 'discriminatory_vector':'NOT_MEASURED_SINCE_REDESIGN; no corrected/regressed/killed counts claimed before accepted aggregate focal',
 'next_gate':'Independent review and single aggregate manifest accepted before any focal execution; known pending raw/derived receipt are not positive evidence.'}
p=D/'p1339-ab-batch1-publication-ledger-r1.json'
if p.exists():raise SystemExit('immutable ledger exists')
body=json.dumps(data,indent=2,ensure_ascii=True)+'\n'
patch='*** Begin Patch\n*** Add File: '+str(p)+'\n'+''.join('+'+x+'\n' for x in body.split('\n')[:-1])+'*** End Patch\n'
subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True)
print(pin(p))
