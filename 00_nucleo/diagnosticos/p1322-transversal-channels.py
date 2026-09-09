"""Expose raw deltas hidden by intentionally narrower transversal projections."""
import importlib.util
import json
from pathlib import Path
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('r',D/'p1322-record.py')
r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
before=r.state();r.verify(before)
source=D/'p1322-transversal-r2.json'
data=json.loads(source.read_text())
rows=[]
for group in data['matrix']:
    for row in group['results']:
        if 'oracle' not in row:continue
        deltas=[key for key in ['exit_code','stdout','stderr'] if row['oracle'][key]!=row['crystalline'][key]]
        rows.append(dict(id=row['id'],profile=group['profile'],phase=group['phase'],
            projection_state=row['estado'],channel_state='RAW_DIFFERENCE' if deltas else 'RAW_EQUAL',
            differing_channels=deltas,oracle=row['oracle'],crystalline=row['crystalline']))
reference={(x['id'],x['profile']):x for x in rows if x['phase']=='normal'}
unstable=[]
for row in rows:
    ref=reference[(row['id'],row['profile'])]
    # Compilation output destinations vary by phase. Only actual channels,
    # not command/output paths, define this additional comparison.
    if any(row[side][key]!=ref[side][key] for side in ['oracle','crystalline'] for key in ['exit_code','stdout','stderr']):
        unstable.append(dict(id=row['id'],profile=row['profile'],phase=row['phase']))
projected_matches=[x for x in rows if x['projection_state']=='MATCH' and x['channel_state']=='RAW_DIFFERENCE']
r.save('transversal-channels',dict(at=r.now(),before=before,after=r.state(),
    manifest_sha256=r.sha(D/'p1322-manifest.json'),source=dict(path=str(source),sha256=r.sha(source)),
    runner_sha256=r.sha(__file__),rows=rows,unstable_channels=unstable,
    projected_match_raw_difference=[dict(id=x['id'],profile=x['profile'],phase=x['phase'],differing_channels=x['differing_channels']) for x in projected_matches],
    interpretation='R2 MATCH means only the manifest-selected projection. This overlay retains raw warnings/hints/help/version/path differences; no channel suppression, new execution, or principal-denominator credit. Not every raw delta is language debt; causal addendum decides separately.'))
print('projected MATCH with raw delta',len(projected_matches),'unstable',unstable)
