"""Successor of three extra probes: actual Typst escapes, not literal slashes."""
import importlib.util
import hashlib
import json
from pathlib import Path
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('t',D/'p1322-transversal.py')
t=importlib.util.module_from_spec(spec);spec.loader.exec_module(t)
r=t.r
cases=[dict(id='boundary-'+str(i),expression=e) for i,e in enumerate([
    r'csv(bytes("\"a\nb\",c\n1"))',
    r'csv(bytes("a,b\r\n1"))',
    r'csv(bytes("á,😀\r\n1"))',
])]
before=r.state();r.verify(before)
bins=json.loads((D/'p1322-transversal.json').read_text())['binaries']
frozen=dict(at=r.now(),before=before,binaries=bins,cases=cases,profiles=t.PROFILES,
    manifest_sha256=r.sha(D/'p1322-manifest.json'),runner_sha256=r.sha(__file__),
    inputs={str(D/n):r.sha(D/n) for n in ['p1322-transversal.py','p1322-sentinels.py','p1322-record.py']},
    supersedes=dict(receipt_sha256=r.sha(D/'p1322-transversal.json'),ids=[c['id'] for c in cases]),
    reason='R1 boundary0-2 double-escaped slash sequences, so did not test claimed multiline/CRLF boundaries. R2 retains actual single Typst escapes.')
r.save('boundary-r2-freeze',frozen)
rows=[]
for phase in ['normal','repeat','reverse']:
    for profile in t.PROFILES:
        for c in reversed(cases) if phase=='reverse' else cases:
            row=dict(id=c['id'],expression=c['expression'],profile=profile,phase=phase,universe='supplement')
            sides=list(bins.items())
            for side,binary in reversed(sides) if phase=='reverse' else sides:
                o=t.functional.modules.execute(Path(binary['path']),c,profile,phase,side,t.functional.FIX)
                reason=None if o['complete'] and o['exit_code'] in (0,1) else 'EXECUTION_UNKNOWN'
                o.update(binary_path=binary['path'],binary_sha256=binary['sha256'],features=t.PROFILES[profile]['features'],
                    reason_code=reason,complete=reason is None,source_sha256=hashlib.sha256(c['expression'].encode()).hexdigest())
                row[side]=o
            row['runtime_class']=t.functional.m.classify(row['vanilla'],row['crystalline']);rows.append(row)
t.functional.m.verify(frozen)
r.save('boundary-r2',dict(at=frozen['at'],end=r.now(),before=before,after=r.state(),
    manifest_sha256=r.sha(D/'p1322-manifest.json'),freeze_sha256=r.sha(D/'p1322-boundary-r2-freeze.json'),rows=rows))
print([(x['id'],x['profile'],x['runtime_class']) for x in rows])
