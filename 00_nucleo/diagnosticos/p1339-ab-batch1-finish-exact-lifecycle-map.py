"""Finish metadata publication after predecessor dict/list authoring error.
No semantic execution; the already published typed-r3 is never rewritten.
"""
import hashlib, json, subprocess
from pathlib import Path
D=Path(__file__).resolve().parent
def pin(name):
 p=D/name
 return {'path':str(p),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
typed=json.loads((D/'p1339-ab-batch1-retention-lifecycle-typed-r3.json').read_text())
coverage=json.loads((D/'p1339-ab-batch1-coverage-map.json').read_text())
coverage['predecessor']=pin('p1339-ab-batch1-coverage-map.json')
coverage['original_lifecycle_predicate_map']=typed['original_lifecycle_predicate_map']
for sw in typed['structural_witnesses']:
 if sw['id'].startswith('SW-LC-'):coverage['structural_witness_definitions'][sw['id']]=sw
coverage['exact_lifecycle_fixture']=pin('p1339-ab-batch1-retention-lifecycle-typed-r3.json')
coverage['authoring_delta']='Replace generic 50x check()+SW mapping with exact line/predicate pins and 14 narrow residual witnesses; all final-only, zero semantic process.'
coverage['authoring_failure']={'predecessor':pin('p1339-ab-batch1-prepare-exact-lifecycle-map.py'),'cause':'metadata dict/list += TypeError after typed-r3 successful publication','failed_process_wall_seconds':0.032817059,'semantic_processes':0,'correction':'Insert SW by explicit id into existing dictionary, preserve all predecessor entries and typed-r3 bytes.'}
p=D/'p1339-ab-batch1-coverage-map-r2.json'
if p.exists():raise SystemExit('immutable output exists')
body=json.dumps(coverage,indent=2,ensure_ascii=True)+'\n'
patch='*** Begin Patch\n*** Add File: '+str(p)+'\n'+''.join('+'+line+'\n' for line in body.split('\n')[:-1])+'*** End Patch\n'
subprocess.run(['apply_patch'],input=patch,text=True,check=True,capture_output=True)
print(pin(p.name))
print(pin('p1339-ab-batch1-lifecycle-predicate-v3.py'))
