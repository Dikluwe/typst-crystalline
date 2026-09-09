"""Fresh P1335 observations; retain the P1309 raw-channel projection."""
import argparse
from collections import Counter
from concurrent.futures import ThreadPoolExecutor
import importlib.util
import json
from pathlib import Path
import sys
import time
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
def load(name,path):
    spec=importlib.util.spec_from_file_location(name,path)
    module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module);return module
r=load('p1335_record',D/'p1335-record.py')
old=load('p1309_matrix_readonly',D/'p1309-matrix.py')
PROFILES=old.PROFILES
observe=old.observe
classify=old.classify
stable_key=old.stable_key

def freeze():
    build=json.loads((D/'p1335-build.json').read_text())
    catalog=json.loads((D/'p1335-probe-catalog.json').read_text())
    historical=json.loads((D/'p1322-probe-catalog.json').read_text())
    assert build['exit']==0 and catalog['profiles']==PROFILES
    lookup={p['id']:p for p in catalog['probes']}
    assert len(lookup)==len(catalog['probes']) and all(lookup.get(p['id'])==p for p in historical['probes'])
    inputs=['p1335-probe-catalog.json','p1322-probe-catalog.json','p1335-build.json',
        'p1335-manifest.json','p1335-record.py','p1335-matrix.py','p1309-matrix.py','p1309-record-r2.py']
    r.save('runtime-freeze',dict(at=r.now(),manifest_sha256=r.sha(D/'p1335-manifest.json'),
        inputs={str(D/p):r.sha(D/p) for p in inputs},
        binaries=dict(vanilla=r.baseline()['vanilla'],crystalline=build['candidate']),
        profiles=PROFILES,projection='exact stdout/stderr bytes; successful JSON must parse; no normalization',
        orders=['normal','repeat','reverse'],timeout=30,workers=8))

def verify(frozen):
    for path,digest in frozen['inputs'].items():assert r.sha(path)==digest,path
    for binary in frozen['binaries'].values():assert r.sha(binary['path'])==binary['sha256']
    r.verify(r.state())

def run(phase):
    frozen=json.loads((D/'p1335-runtime-freeze.json').read_text());verify(frozen)
    catalog=json.loads((D/'p1335-probe-catalog.json').read_text())
    jobs=[(p,profile) for p in sorted(catalog['probes'],key=lambda p:p['id']) for profile in PROFILES]
    if phase=='reverse':jobs.reverse()
    before=r.state();tick=time.monotonic();start=r.now()
    def pair(job):
        probe,profile=job
        row=dict(id=probe['id'],path=probe['path'],expression=probe['expression'],profile=profile,phase=phase,universe='principal')
        for side in (('crystalline','vanilla') if phase=='reverse' else ('vanilla','crystalline')):
            row[side]=observe(frozen['binaries'][side],probe,profile,side,phase)
        row['runtime_class']=classify(row['vanilla'],row['crystalline']);return row
    rows=[]
    with ThreadPoolExecutor(max_workers=8) as pool:
        for row in pool.map(pair,jobs):
            rows.append(row)
            if len(rows)%1000==0:print(phase,len(rows),len(jobs),flush=True)
    verify(frozen);counts=dict(Counter(row['runtime_class'] for row in rows))
    r.save('matrix-'+phase,dict(schema='p1335-global-matrix-v1',at=start,end=r.now(),seconds=time.monotonic()-tick,
        manifest_sha256=r.sha(D/'p1335-manifest.json'),freeze_sha256=r.sha(D/'p1335-runtime-freeze.json'),
        catalog_sha256=r.sha(D/'p1335-probe-catalog.json'),runner_sha256=r.sha(__file__),
        before=before,after=r.state(),binaries=frozen['binaries'],profiles=PROFILES,workers=8,
        probes=len(catalog['probes']),pairs=len(rows),counts=counts,results=rows))
    print(counts,flush=True)

if __name__=='__main__':
    mode=sys.argv[1]
    if mode=='freeze':freeze()
    else:
        assert mode in ('normal','repeat','reverse');run(mode)
