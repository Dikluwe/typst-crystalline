"""Readonly original-location control for P1334 cross-source preservation."""
import importlib.util,json,sys
from pathlib import Path
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('s',D/'p1335-sentinels-r3.py');s=importlib.util.module_from_spec(spec);spec.loader.exec_module(s);r=s.r
if sys.argv[1]=='freeze':
    old=json.loads((D/'p1334-ab-cross-baseline.json').read_text());cases=[];seen=set()
    for row in old['cases']:
        if row['case'] in seen:continue
        seen.add(row['case']);cases.append(dict(id='p1334.'+row['case'],historical_id=row['case'],expression=row['expression'],format='repr',route='eval',cwd=str(r.ROOT),source_sha256=s.hashlib.sha256(row['expression'].encode()).hexdigest()))
    files=['p1335-location-control.py','p1335-sentinels-r3.py','p1335-sentinels-freeze-r3.json','p1334-ab-cross-baseline.json','p1334-ab-cross-expected.json','p1334-ab-fixtures/origins.typ','p1335-classification-preservation-freeze-r3.json']
    r.save('location-control-freeze',dict(at=r.now(),manifest_sha256=r.sha(D/'p1335-manifest.json'),inputs={str(D/n):r.sha(D/n) for n in files},binary=json.loads((D/'p1335-build.json').read_text())['candidate'],cases=cases,profiles=s.m.PROFILES,policy='Fresh current candidate at original immutable source path; exact exit/stdout/stderr versus final expected map, not relocated-source normalization. Original fixture only read.'))
else:
    f=json.loads((D/'p1335-location-control-freeze.json').read_text());assert all(r.sha(p)==h for p,h in f['inputs'].items());assert r.sha(f['binary']['path'])==f['binary']['sha256'];before=r.state();r.verify(before)
    expected={(x['case'],x['profile']):x['expected'] for x in json.loads((D/'p1334-ab-cross-expected.json').read_text())['expectations']};rows=[]
    for phase in ['normal','repeat','reverse']:
        for c in reversed(f['cases']) if phase=='reverse' else f['cases']:
            for p in s.m.PROFILES:
                o=s.observe(c,p,'crystalline',f['binary'],phase,Path('/tmp'));e=expected[(c['historical_id'],p)];actual=dict(exit=o['exit_code'],stdout=o['stdout'],stderr=o['stderr'])
                rows.append(dict(id=c['id'],profile=p,phase=phase,observation=o,expected=e,verdict='Unknown' if not o['complete'] else 'Preserved' if actual==e else 'Violated'))
    r.verify(r.state());r.save('location-control',dict(at=r.now(),manifest_sha256=r.sha(D/'p1335-manifest.json'),freeze_sha256=r.sha(D/'p1335-location-control-freeze.json'),before=before,after=r.state(),rows=rows,counts=dict(s.Counter(x['verdict'] for x in rows))))
