import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

exp=Path(__file__).resolve().parent
out=exp.parents[2]/'lab/results/construction/diagnostics'/exp.name
prior=out.parent/'references-500-source-plan-block-2026-11-21'
source=Path(sys.argv[2])
manifest={p.removeprefix('./'): h for h,p in (line.split(maxsplit=1) for line in (prior/'candidate-source.sha256').read_text().splitlines())}
owners=json.loads((exp/'owners.json').read_text())
paths={}
for owner in owners:
    layer,path=owner.split('/',1)
    prefix={'core':'01_core','infra':'03_infra'}[layer]
    paths[owner]=(source/f'{prefix}/src/{path}.rs', source/f'00_nucleo/prompts/{owner}.md')

def replace(s, old, new):
    assert s.count(old)==1, old
    return s.replace(old,new,1)

def edit(k, f):
    p=paths[k][0]
    p.write_text(f(p.read_text()))

if sys.argv[1]=='seal':
    for p,h in manifest.items():
        assert hashlib.sha256((source/p).read_bytes()).hexdigest()==h,p
    # Each affected consumer has a current, exact L0 before appending the experiment.
    for owner,(rust,prompt) in paths.items():
        with prompt.open('a') as f:
            f.write('\n\n## Experimento isolado de reserva em bloco\n\n'
              'Evidência: lab/results/construction/diagnostics/references-500-source-plan-block-2026-11-21/'
              'report.md mede a montagem em bloco e identifica a reserva integral após o corte. O censo deste ensaio consta em lab/results/construction/diagnostics/references-500-origin-reservations-2026-11-21/red.json.\n\n'+owners[owner]+'\n')
    seal=subprocess.run(['crystalline-lint','--fix-hashes','--dry-run',str(source)],capture_output=True,text=True)
    (out/'dry-seal.log').write_text(seal.stdout+seal.stderr)
    for rust,prompt in paths.values():
        row=next(row for row in seal.stdout.splitlines() if f'{rust} prompt={prompt.relative_to(source)} ' in row)
        a,b=re.search(r'hash-a=(\w+) hash-b=(\w+)',row).groups()
        rust.write_text(re.sub(r'@prompt-hash \w+',f'@prompt-hash {a}',rust.read_text(),count=1))
        prompt.write_text(re.sub(r'Hash do Código: \w+',f'Hash do Código: {b}',prompt.read_text(),count=1))
elif sys.argv[1]=='implement':
    exec((exp/'implement.py').read_text())
    changed={p:h for p in manifest if (h:=hashlib.sha256((source/p).read_bytes()).hexdigest())!=manifest[p]}
    assert set(changed)=={str(p.relative_to(source)) for pair in paths.values() for p in pair},changed
    (out/'freeze.json').write_text(json.dumps(changed,indent=2)+'\n')
    patch=[]
    for owner,pair in paths.items():
        for p in pair:
            name=str(p.relative_to(source))
            baseline=out/('before-'+owner.replace('/','-')+p.suffix)
            candidate=out/('candidate-'+owner.replace('/','-')+p.suffix)
            candidate.write_bytes(p.read_bytes())
            diff=subprocess.run(['diff','-u','--label','a/'+name,'--label','b/'+name,str(baseline),str(candidate)],capture_output=True)
            assert diff.returncode==1, name
            patch.append(diff.stdout)
    (out/'candidate.patch').write_bytes(b''.join(patch))
elif sys.argv[1]=='snapshot':
    for owner,pair in paths.items():
        for p in pair:
            (out/('before-'+owner.replace('/','-')+p.suffix)).write_bytes(p.read_bytes())
else:
    raise SystemExit('use snapshot, seal or implement')
