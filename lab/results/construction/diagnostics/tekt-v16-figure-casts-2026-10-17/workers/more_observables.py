from pathlib import Path
import json,re,datetime
out=Path(__file__).resolve().parents[1];a={ (r['name'],r['binary']):r for r in json.load(open(out/'more-cast-probe.json'))['rows']};z={(r['name'],r['binary']):r for r in json.load(open(out/'more-cast-after.json'))['rows']};assert a.keys()==z.keys();names={n for n,k in z}
def diagnostic(r):
 lines=r['stderr'].splitlines();return r['exit'],next((x for x in lines if x.startswith('error:')),None),next((m.group(1) for x in lines if (m:=re.search(r'<input-expression>:(\d+:\d+)',x))),None)
errors=[];texts=[];preserved=[]
for n in sorted(names):
 before=a[n,'tekt'];after=z[n,'tekt'];oracle=z[n,'vanilla'];assert all(a[n,'vanilla'][f]==oracle[f] for f in ['exit','stdout','stderr'])
 if n.endswith('-ab'):
  assert all(before[f]==after[f] for f in ['exit','stdout','stderr']);preserved.append(n);continue
 if n.startswith('type-') or n.startswith('symbol-kind-'):
  assert diagnostic(after)==diagnostic(oracle),(n,diagnostic(after),diagnostic(oracle));errors.append(dict(name=n,tekt=diagnostic(after),vanilla=diagnostic(oracle)));continue
 assert after['exit']==oracle['exit']==0
 t=json.loads(after['stdout']);v=json.loads(oracle['stdout']);t=t['body'] if n.startswith('symbol-caption-') else t;v=v['body'] if n.startswith('symbol-caption-') else v
 assert t['text']==v['text'],(n,t,v);assert t['func']=='text' and v['func']=='symbol'
 texts.append(dict(name=n,text=t['text'],tekt_kind=t['func'],vanilla_kind=v['func'],status='Text preserved, SymbolElem morphology still pending SY1'))
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),pairs=len(names),correct_type_or_symbol_diagnostics=errors,symbol_text_preserved=texts,invalid_symbol_constructor_controls_preserved=preserved,oracle_unchanged=True,morphology_gap_explicit=True)
(out/'more-observables-result.json').write_text(json.dumps(r,indent=2)+'\n');print(dict(pairs=len(names),diagnostics=len(errors),text=len(texts),preserved_invalid=len(preserved)))
