from pathlib import Path
import json,gzip
p=Path(__file__).resolve().parent
d=json.loads(gzip.decompress((p/'after.json.gz').read_bytes()))
fixed={'empty-fields','empty-func','empty-children','empty-has','empty-unknown','empty-at-unknown','sequence-fields','sequence-children','sequence-single','sequence-for'}
controls={'single-text','opaque-styled'}
rows=[]
for c in d['cases']:
 if c['label'] not in fixed|controls:continue
 a=c['results']['tekt'];b=c['results']['vanilla']
 assert a['exit']==b['exit'],c
 if a['exit']:
  assert a['stderr'].splitlines()[0]==b['stderr'].splitlines()[0],c
 else:assert a['text']==b['text'],c
 rows.append(c['label'])
assert set(rows)==fixed|controls
print(json.dumps({'pass':rows,'open':['children-method','empty-call','sequence-call','empty-selector','sequence-selector','empty-counter'],'note':'JSON samples are retained as existing controls, not byte/layout equality assertions.'}))
