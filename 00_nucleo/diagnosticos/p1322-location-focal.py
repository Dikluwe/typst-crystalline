"""Contrast fixture location without suppressing external-path diagnostics."""
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('transversal',D/'p1322-transversal.py')
t=importlib.util.module_from_spec(spec);spec.loader.exec_module(t)
r=t.r
before=r.state();r.verify(before)
prior=json.loads((D/'p1322-transversal.json').read_text())
inputs=json.loads((D/'p1322-transversal-cases.json').read_text())
folder=t.functional.FIX/'matrix'
folder.mkdir()
subprocess.run(['cp','-a',str(Path(inputs['temp'])/'fixtures'),str(folder/'fixtures')],check=True)
cases=[c for c in inputs['matrix']['cases'] if c['id'] in ['P1138-S-001','P1138-S-002','P1138-S-003']]
pins={str(p):r.sha(p) for p in (folder/'fixtures').rglob('*') if p.is_file()}
bins=prior['binaries']
r.save('location-focal-freeze',dict(at=r.now(),manifest_sha256=r.sha(D/'p1322-manifest.json'),before=before,
    previous_sha256=r.sha(D/'p1322-transversal.json'),runner_sha256=r.sha(__file__),cases=cases,profiles=t.PROFILES,
    inputs=pins,binaries=bins,hypothesis='Location is the only changed input; internal fixture diagnostics should retain historical equality. External-path difference remains measured, not normalized away.'))
t.matrix.HERE=folder
rows=[]
for phase in ['normal','repeat','reverse']:
    for profile in t.PROFILES:
        for c in (reversed(cases) if phase=='reverse' else cases):
            row=t.matrix.run_case(c,Path(bins['vanilla']['path']),Path(bins['crystalline']['path']),
                Path(inputs['temp'])/'location-focal'/phase/profile/c['id'],profile,t.PROFILES)
            row.update(profile=profile,phase=phase);rows.append(row)
r.verify(r.state())
assert all(r.sha(p)==v for p,v in pins.items())
r.save('location-focal',dict(at=r.now(),before=before,after=r.state(),manifest_sha256=r.sha(D/'p1322-manifest.json'),
    freeze_sha256=r.sha(D/'p1322-location-focal-freeze.json'),rows=rows,
    interpretation='Contrasting location, not deleting or overriding the external-path observations'))
qa=[str(Path(inputs['temp'])/('qa-'+side+'.png')) for side in ['vanilla','crystalline']]
r.save('visual-qa',dict(at=r.now(),state=r.state(),manifest_sha256=r.sha(D/'p1322-manifest.json'),
    source_receipt_sha256=r.sha(D/'p1322-transversal.json'),renders={p:r.sha(p) for p in qa},
    commands=[['pdftoppm','-scale-to','1400','-png','-singlefile',str(Path(inputs['temp'])/'normal/default/P1138-X-003'/name),str(Path(inputs['temp'])/('qa-'+side))]
        for side,name in [('vanilla','oracle.pdf'),('crystalline','crystalline.pdf')]],
    review='Root inspected both entire single-page renderings: same visible Hello, parity. placement, legible text, no visible clipping/overlap. This is a tiny sample, not global visual parity.',
    limits='Internal PDF font naming alone is mechanical; raster 11-pixel/channel-delta-1 observation remains in raw receipt, not proof of semantic mismatch.'))
print([(x['id'],x['profile'],x['estado']) for x in rows])
