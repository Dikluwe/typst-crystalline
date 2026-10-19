"""Losslessly compress bulky fingerprint records; create a dataset SHA-256 manifest."""
import argparse, gzip, hashlib, json
from pathlib import Path
OUT=Path(__file__).resolve().parent
h=lambda data:hashlib.sha256(data).hexdigest()
p=argparse.ArgumentParser();p.add_argument('action',choices=['compress','manifest']);a=p.parse_args()
if a.action=='compress':
    records=[]
    for path in sorted((OUT/'state').glob('*.json')):
        if path.name in {'initial.json','built.json','final.json'} or path.stat().st_size<262144:continue
        data=path.read_bytes();packed=gzip.compress(data,mtime=0);assert gzip.decompress(packed)==data
        target=path.with_suffix('.json.gz');target.write_bytes(packed)
        records.append({'original':str(path.relative_to(OUT)),'original_sha256':h(data),'stored':str(target.relative_to(OUT)),'stored_sha256':h(packed),'encoding':'gzip; decompress to recover exact original bytes'})
        path.unlink()
    (OUT/'packed-records.json').write_text(json.dumps(records,indent=2)+'\n')
else:
    records=[]
    for path in sorted(OUT.rglob('*')):
        if path.is_file() and path.name!='MANIFEST.sha256':records.append(h(path.read_bytes())+'  '+str(path.relative_to(OUT)))
    report=OUT.parent/'performance-baseline.md';records.append(h(report.read_bytes())+'  ../performance-baseline.md')
    (OUT/'MANIFEST.sha256').write_text('\n'.join(records)+'\n')
    print('Dataset manifest created; verify from this directory with sha256sum --check MANIFEST.sha256')
