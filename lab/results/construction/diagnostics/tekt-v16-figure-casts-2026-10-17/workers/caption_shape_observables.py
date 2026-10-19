from pathlib import Path
import json,datetime
out=Path(__file__).resolve().parents[1];z={(r['name'],r['binary']):r for r in json.load(open(out/'caption-shape-after.json'))['rows']};names={n for n,k in z};assert len(names)==15;exact=[];partial=[]
for n in sorted(names):
 t=z[n,'tekt'];v=z[n,'vanilla']
 if n=='separator-auto':
  assert t['exit']==1 and v['exit']==0 and 'expected content, found auto' in t['stderr'];partial.append(dict(name=n,reason='FG1: auto vs omitted carrier',tekt=t,vanilla=v));continue
 assert t['exit']==v['exit']==0,(n,t,v)
 a=json.loads(t['stdout']);b=json.loads(v['stdout'])
 if n=='kind-element':partial.append(dict(name=n,reason='Admission corrected; typed kind identity/synthesis remains FG1',tekt=a,vanilla=b));continue
 if n=='body-symbol':
  assert a['text']==b['text']=='α';assert a['func']=='text' and b['func']=='symbol';partial.append(dict(name=n,reason='SY1: symbol morphology',tekt=a,vanilla=b));continue
 if n=='fields-explicit' and a!=b:
  assert a.keys()==b.keys() and a['position']==b['position'] and a['body']==b['body'];assert a['separator']['text']==b['separator']['text'] and a['separator']['func']=='text' and b['separator']['func']=='symbol';partial.append(dict(name=n,reason='SY1: identical separator grapheme with Text versus Symbol morphology',tekt=a,vanilla=b));continue
 assert a==b,(n,a,b);exact.append(n)
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),pairs=len(names),exact_language_values=exact,partial_and_pending=partial,criterion='Compare complete JSON language values, including the full encoded string for json.encode; expose carrier/morphology gaps without treating them as passing parity.');(out/'caption-shape-observables-result.json').write_text(json.dumps(r,indent=2)+'\n');print(dict(pairs=len(names),exact=len(exact),partial=len(partial)))
