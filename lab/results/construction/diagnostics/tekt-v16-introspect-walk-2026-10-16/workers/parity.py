from pathlib import Path
import json,re
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-walk-2026-10-16');m=json.load(open(out/'measurement-after.json'));rows={}
for c in m['cases']:
 assert c['exit']==0 and c['query']['exit']==0,c
 rows.setdefault(Path(c['fixture']).stem,{})['vanilla' if c['binary'].startswith('lab/') else 'tekt']=c
checks=[]
for name,pair in rows.items():
 a,b=pair['vanilla'],pair['tekt'];qa,qb=map(lambda x:json.loads(x['query']['stdout']),[a,b]);assert qa==qb,(name,qa,qb)
 # Compare the visible semantic probe tuple and heading count, independent of PDF bytes or line placement.
 def payload(text):
  text=' '.join(text.split());match=re.search(r'(\(\(1,.*?\)|\(\)) HEADINGS (\d+)',text);assert match,text;return match.groups()
 if name in ['order','state','counter']:
  marker={'order':'BARE','state':'STATE','counter':'COUNTER'}[name]
  def semantic(text):
   text=' '.join(text.split());assert marker in text,text;return text.split(marker,1)[1].strip()
  assert semantic(a['text'])==semantic(b['text']),(name,semantic(a['text']),semantic(b['text']))
  observed=semantic(b['text'])
 else:
  assert payload(a['text'])==payload(b['text']),(name,payload(a['text']),payload(b['text']))
  observed=payload(b['text'])
 checks.append({'fixture':name,'levels':qa,'semantic_probe':observed})
assert len(checks)==9;(out/'parity-result.json').write_text(json.dumps({'checks':checks,'pin':m.get('parity_pin','9f2b6e8715237cb086899a42873660fe744622e8'),'language_scope':'nine direct/synthesized/mixed/order/state/counter fixtures, body/fields, native query levels/order/body/heading count; not global parity or PDF byte equivalence'},indent=2)+'\n');print('Nine bilateral language probes PASS',flush=True)
