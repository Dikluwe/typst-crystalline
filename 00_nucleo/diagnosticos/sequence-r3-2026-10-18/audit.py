from pathlib import Path
import json,gzip,collections
p=Path(__file__).resolve().parent
load=lambda f:json.loads(gzip.decompress((p/f).read_bytes()))
corpora=[('main','transport-after-main.json.gz','before.json.gz',{
'markup-label':'open-label-reflection','literal-identical':'open-textual-precedence','literal-same-prefix':'open-textual-precedence','literal-local':'open-textual-precedence','regex-literal':'open-textual-precedence','styled-child':'open-styled-reflection','styled-equality':'open-styled-fields','symbol-cast':'open-symbol-morphology'}),
('transport','transport-after.json.gz','transport-before.json.gz',{'style-tail':'open-textual-group-shape-and-styled-reflection','style-head':'open-styled-reflection','rule-tail':'open-show-tail-shape-and-styled-reflection'}),
('textual','textual-after-transport.json.gz','textual-before.json.gz',{'constant-chain':'open-textual-output-not-revisited','function-chain-inverse':'open-textual-output-revisited-twice','alias-chain':'open-textual-output-not-revisited','revoked-cross':'open-textual-revocation-boundary','local-cross':'open-textual-revocation-boundary','unicode-overlap':'open-textual-precedence','show-set':'open-textual-show-set-admission'})]
# These snapshots retain explicitly defective outcomes for a differential check;
# they are never requirements for accepted parity.
known=load('known-open.json.gz')
result={}
def observation(v):return (v['exit'],v['text'],v['stderr'].splitlines()[0] if v['stderr'] else '')
for name,after,before,open_labels in corpora:
 data=load(after);old={x['label']:x for x in load(before)['cases']};rows=[]
 for row in data['cases']:
  label=row['label'];a,b=[row['results'][k] for k in ['tekt','vanilla']]
  assert row['source_sha256']==old[label]['source_sha256'],(name,label,'source changed')
  if label in open_labels:
   assert observation(old[label]['results']['tekt'])!=observation(old[label]['results']['vanilla']),(name,label,'new divergence')
   previous=known[name][label]
   assert observation(a)==tuple(previous['tekt']) and observation(b)==tuple(previous['vanilla']),(name,label,row)
   status=open_labels[label]
  else:
   assert a['exit']==b['exit'] and a['text']==b['text'],(name,label,row)
   if a['exit']:assert a['stderr'].splitlines()[0]==b['stderr'].splitlines()[0],(name,label,row)
   status='accepted-language-value-and-primary-error'
  rows.append({'label':label,'status':status})
 result[name]=rows
geo=load('markup-geometry.json.gz');assert all(v['accepted'] for c in geo['cases'] for v in c['results'].values())
result['geometry']={'comparisons':sum(len(v['checks']) for c in geo['cases'] for v in c['results'].values()),'method':'Within each compiler, exact page/word identities and PDF coordinates within 1e-6pt; not cross-compiler geometry or PDF byte equality.'}
result['method']='Accepted cases compare the pinned oracle; open cases are exact differential controls of measured defects, never accepted parity. No newly divergent input is admitted.'
(p/'audit.json.gz').write_bytes(gzip.compress(json.dumps(result,indent=2).encode()))
print(json.dumps({k:dict(collections.Counter(x['status'] for x in v)) for k,v in result.items() if isinstance(v,list)},indent=2))
