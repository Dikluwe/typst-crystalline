from pathlib import Path
import json,datetime
out=Path(__file__).resolve().parents[1];a={(r['name'],r['binary']):r for r in json.load(open(out/'caption-before.json'))['rows']};z={(r['name'],r['binary']):r for r in json.load(open(out/'caption-after.json'))['rows']};repaired=[];pending=[]
for n in ['omitted','none','empty','content','explicit-caption','body']:
 t=z[n,'tekt'];v=z[n,'vanilla'];assert all(a[n,'vanilla'][f]==v[f] for f in ['exit','stdout','stderr'])
 if n in ['omitted','none']:
  assert all(a[n,'tekt'][f]==t[f] for f in ['exit','stdout','stderr']);pending.append(dict(name=n,tekt=t,vanilla=v));continue
 assert t['exit']==v['exit']==0
 assert json.loads(t['stdout'])==json.loads(v['stdout']),(n,t['stdout'],v['stdout'])
 if n=='body':assert all(a[n,'tekt'][f]==t[f] for f in ['exit','stdout','stderr'])
 else:assert a[n,'tekt']['exit']==1;repaired.append(n)
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),pairs=6,repaired=repaired,body_preserved=True,pending_presence_and_diagnostic_gaps=pending,criterion='Present caption values compare complete JSON language values, empty/body-only caption; defaults/synthesis/presence remain explicit limits.');(out/'caption-observables-result.json').write_text(json.dumps(r,indent=2)+'\n');print(dict(repaired=repaired,pending=2))
