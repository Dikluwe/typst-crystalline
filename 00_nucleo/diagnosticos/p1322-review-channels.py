"""Independent read-only reconciliation of the late raw-channel overlay."""
import collections
import datetime
import hashlib
import json
from pathlib import Path
import subprocess
import sys
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
inputs = {}
def read(name):
    p = D / name
    b = p.read_bytes()
    inputs[str(p)] = hashlib.sha256(b).hexdigest()
    return json.loads(b)
raw = read('p1322-transversal-r2.json')
overlay = read('p1322-transversal-channels.json')
expected = {}
for group in raw['matrix']:
    for row in group['results']:
        if 'oracle' not in row:
            continue
        key = row['id'], group['profile'], group['phase']
        diffs = [k for k in ('exit_code','stdout','stderr') if row['oracle'][k] != row['crystalline'][k]]
        expected[key] = dict(id=key[0], profile=key[1], phase=key[2], projection_state=row['estado'],
            channel_state='RAW_DIFFERENCE' if diffs else 'RAW_EQUAL', differing_channels=diffs,
            oracle=row['oracle'], crystalline=row['crystalline'])
actual = {(r['id'],r['profile'],r['phase']):r for r in overlay['rows']}
errors = []
if len(actual) != len(overlay['rows']) or actual != expected:
    errors.append('OVERLAY_NOT_EXACT_RAW_RECONSTRUCTION')
if overlay['source']['sha256'] != inputs[str(D/'p1322-transversal-r2.json')]:
    errors.append('SOURCE_PIN')
projected = [r for r in expected.values() if r['projection_state']=='MATCH' and r['channel_state']=='RAW_DIFFERENCE']
summarized = [dict(id=r['id'],profile=r['profile'],phase=r['phase'],differing_channels=r['differing_channels']) for r in projected]
if summarized != overlay['projected_match_raw_difference']:
    errors.append('PROJECTED_MATCH_RAW_DELTA_OMITTED')
unstable = []
for key,row in expected.items():
    normal = expected[key[0],key[1],'normal']
    if any(row[s][k] != normal[s][k] for s in ('oracle','crystalline') for k in ('exit_code','stdout','stderr')):
        unstable.append(dict(id=key[0],profile=key[1],phase=key[2]))
if unstable != overlay['unstable_channels'] or unstable:
    errors.append('CHANNEL_INSTABILITY')
# Additional discriminating checks do not add to the 13 attacks frozen in the plan.
omitted = dict(actual)
omitted.pop(('P1137-I-001','default','normal'))
control = actual == expected
reject_omission = omitted != expected
counts = collections.Counter(r['phase']+'/'+r['profile'] for r in projected)
out = dict(at=datetime.datetime.now(datetime.timezone.utc).isoformat(), role='D',
    checker_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(), inputs=inputs,
    verdict='Violated' if errors else 'Preserved', violations=errors, unknowns=[],
    reconstructed_rows=len(expected), projected_match_raw_differences=len(projected),
    counts=dict(counts), unstable=unstable,
    extra_controls=dict(genuine=control, omitted_query_warning_rejected=reject_omission),
    scope='Exact channels and overlay reconciliation; causal language/mechanics and ranking reviewed separately. No denominator credit or new execution.')
dest=D/'p1322-review-channels.json'
assert not dest.exists()
payload=json.dumps(out,indent=2,ensure_ascii=False)+'\n'
subprocess.run(['apply_patch'],input='*** Begin Patch\n*** Add File: '+str(dest)+'\n'+''.join('+'+x+'\n' for x in payload.splitlines())+'*** End Patch\n',text=True,check=True,capture_output=True)
print(payload)
