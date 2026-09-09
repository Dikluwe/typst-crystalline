"""Check behavior-preserving mechanical preparation against pre-L0 measurement."""
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('r',D/'p1323-record.py')
r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
b=json.loads((D/'p1323-baseline.json').read_text());before=r.state();r.verify(before)
binary=Path(r.TARGET)/'release/typst';rows=[]
for old in b['rows']:
    if old['side']!='crystalline':continue
    argv=list(old['argv']);argv[0]=str(binary)
    output=Path(r.TARGET)/('seam-'+old['profile']+'.html');argv[4]=str(output)
    at=r.now();p=subprocess.run(argv,cwd=r.ROOT,capture_output=True,text=True,timeout=30)
    row=dict(profile=old['profile'],at=at,argv=argv,exit=p.returncode,stdout=p.stdout,stderr=p.stderr,
        artifact_sha256=r.sha(output) if output.exists() else None)
    row['preserved']=all(row[k]==old[k] for k in ['exit','stdout','stderr','artifact_sha256'])
    rows.append(row)
r.save('seam-preservation',dict(at=r.now(),before=before,after=r.state(),manifest_sha256=r.sha(D/'p1323-manifest.json'),
    baseline_sha256=r.sha(D/'p1323-baseline.json'),binary=dict(path=str(binary),sha256=r.sha(binary)),rows=rows,
    preserved=all(x['preserved'] for x in rows),meaning='Same-product preparation preservation, not cross-product artifact byte parity.'))
assert all(x['preserved'] for x in rows)
