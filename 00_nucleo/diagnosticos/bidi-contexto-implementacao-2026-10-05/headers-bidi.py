import json,re
from pathlib import Path
H=Path('00_nucleo/diagnosticos/bidi-contexto-implementacao-2026-10-05');scope={x['source'] for x in json.loads((H/'scope.json').read_text())}
j=json.loads(Path('/tmp/tekt-bidi-materializacao/lint-unsealed.sarif').read_text());seals=[]
for r in j['runs'][0]['results']:
 if r['ruleId']!='V5':continue
 loc=r['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./')
 if loc not in scope:continue
 m=re.search(r'Hash L0: ([a-f0-9]{8})',r['message']['text']);assert m,r
 p=Path(loc);s=p.read_text();s=re.sub(r'^//! @prompt-hash [a-f0-9]+',f'//! @prompt-hash {m[1]}',s,flags=re.M);p.write_text(s);seals.append({'source':loc,'prompt_hash':m[1]})
(H/'seals.json').write_text(json.dumps(seals,indent=2)+'\n');print('headers updated',len(seals))
