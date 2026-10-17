from pathlib import Path
import json,gzip,datetime
out=Path(__file__).resolve().parents[1];before=json.load(gzip.open(out/'before-corrected-measurement.json.gz'));after=json.load(gzip.open(out/'after-measurement.json.gz'));names=list(dict.fromkeys(x['name'] for x in after['cases']));assert len(names)==24;changed=[];controls=[]
for name in names:
 b={x['binary']:x for x in before['cases'] if x['name']==name};a={x['binary']:x for x in after['cases'] if x['name']==name};assert a['vanilla']['stdout']==b['vanilla']['stdout'];assert all(x['exit']==0 for x in a.values());assert a['tekt']['stdout']==a['vanilla']['stdout'],name
 if b['tekt']['stdout']!=b['vanilla']['stdout']:changed.append(name)
 else:controls.append(name);assert a['tekt']['stdout']==b['tekt']['stdout']
assert len(changed)==17 and len(controls)==7
(out/'observables-result.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),pairs=24,repaired=changed,preserved_controls=controls,no_measured_regressions=True),indent=2)+'\n');print('24 pairs agree; 17 repaired, 7 controls preserved')
