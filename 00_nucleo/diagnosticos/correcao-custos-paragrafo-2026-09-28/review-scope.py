from pathlib import Path
import json,hashlib,difflib
P=Path('00_nucleo/diagnosticos/correcao-custos-paragrafo-2026-09-28');load=lambda f:json.loads((P/f).read_text());sha=lambda f:hashlib.sha256(f.read_bytes()).hexdigest();scope=load('scope.json');old=load('initial.json')['source_sha256'];current={}
for root in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts']:
 for f in Path(root).rglob('*'):
  if f.is_file() and f.suffix in ['.rs','.md','.toml'] and 'target' not in f.parts:current[str(f)]=sha(f)
changed=sorted(f for f in current.keys()|old.keys() if current.get(f)!=old.get(f));allowed={f for pair in scope.items() for f in pair}|{'00_nucleo/prompts/_nuclei/paragraph-costs.toml'};result={'changed':changed,'unexpected':[f for f in changed if f not in allowed],'source_sha256':current};(P/'scope-review.json').write_text(json.dumps(result,indent=2)+'\n');diff=[]
for f in changed:
 q=P/'before'/(f+'.txt');before=q.read_text().splitlines(True) if q.exists() else [];diff+=list(difflib.unified_diff(before,Path(f).read_text().splitlines(True),fromfile='before/'+f,tofile='after/'+f))
(P/'review.diff').write_text(''.join(diff));print('changed',len(changed),'unexpected',result['unexpected']);assert not result['unexpected']
