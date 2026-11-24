import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

exp = Path(__file__).resolve().parent
root = exp.parents[2]
source = Path(sys.argv[2])
out = root/'lab/results/construction/diagnostics'/exp.name
old = out.parent/'page-three-emitter-continuation-2026-11-21/candidate-source.sha256'
paths = {'emit':'paragraph_emit','plan':'paragraph_plan','sequence':'sequence'}
rust = {k:source/f'01_core/src/compiler/layout/{n}.rs' for k,n in paths.items()}
l0 = {k:source/f'00_nucleo/prompts/core/compiler/layout/{n}.md' for k,n in paths.items()}
def change(s,a,b):
    assert s.count(a)==1,a
    return s.replace(a,b,1)
mode=sys.argv[1]
if mode=='red':
    for row in old.read_text().splitlines():
        digest,path=row.split(maxsplit=1)
        assert hashlib.sha256((source/path).read_bytes()).hexdigest()==digest,path
    for k in paths:
        (out/f'before-{k}.rs').write_bytes(rust[k].read_bytes())
        (out/f'before-{k}.md').write_bytes(l0[k].read_bytes())
        with l0[k].open('a') as f:
            f.write((exp/f'contract-{k}.md').read_text())
    check=subprocess.run(['crystalline-lint','--fix-hashes','--dry-run',str(source)],
                         stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True)
    (out/'dry-seal.log').write_text(check.stdout)
    for k,n in paths.items():
        row=next(row for row in check.stdout.splitlines() if f'/layout/{n}.rs prompt=00_nucleo/prompts/core/compiler/layout/{n}.md ' in row)
        a,b=re.search(r'hash-a=(\w+) hash-b=(\w+)',row).groups()
        rust[k].write_text(re.sub(r'@prompt-hash \w+',f'@prompt-hash {a}',rust[k].read_text(),count=1))
        l0[k].write_text(re.sub(r'Hash do Código: \w+',f'Hash do Código: {b}',l0[k].read_text(),count=1))
    p=rust['sequence']
    p.write_text(p.read_text()+(exp/'tests.rs').read_text() +
                 '\n#[cfg(test)]\nfn lab_sequence_boundary(_: &[Content], _: usize, _: usize) -> bool { false }\n')
elif mode=='green':
    extra={'session':'paragraph_session','memo':'physical_memo'}
    paths.update(extra)
    for k,n in extra.items():
        rust[k]=source/f'01_core/src/compiler/layout/{n}.rs'
        l0[k]=source/f'00_nucleo/prompts/core/compiler/layout/{n}.md'
        (out/f'before-{k}.rs').write_bytes(rust[k].read_bytes())
        (out/f'before-{k}.md').write_bytes(l0[k].read_bytes())
        with l0[k].open('a') as f:
            f.write((exp/f'contract-{k}.md').read_text())
    check=subprocess.run(['crystalline-lint','--fix-hashes','--dry-run',str(source)],
                         stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True)
    (out/'dry-seal-retention.log').write_text(check.stdout)
    for k,n in extra.items():
        row=next(row for row in check.stdout.splitlines() if f'/layout/{n}.rs prompt=00_nucleo/prompts/core/compiler/layout/{n}.md ' in row)
        a,b=re.search(r'hash-a=(\w+) hash-b=(\w+)',row).groups()
        rust[k].write_text(re.sub(r'@prompt-hash \w+',f'@prompt-hash {a}',rust[k].read_text(),count=1))
        l0[k].write_text(re.sub(r'Hash do Código: \w+',f'Hash do Código: {b}',l0[k].read_text(),count=1))
    exec((exp/'implement.py').read_text())
    changed={}
    for row in old.read_text().splitlines():
        expected,path=row.split(maxsplit=1)
        digest=hashlib.sha256((source/path).read_bytes()).hexdigest()
        if expected!=digest:
            changed[path.removeprefix('./')]=digest
    assert set(changed)=={str(p.relative_to(source)) for p in [*rust.values(),*l0.values()]},changed
    (out/'freeze.json').write_text(json.dumps(changed,indent=2)+'\n')
    patch=[]
    for k in paths:
        for ext,p in [('rs',rust[k]),('md',l0[k])]:
            frozen=out/f'candidate-{k}.{ext}'
            frozen.write_bytes(p.read_bytes())
            name=str(p.relative_to(source))
            diff=subprocess.run(['diff','-u','--label','a/'+name,'--label','b/'+name,
                                 str(out/f'before-{k}.{ext}'),str(frozen)],capture_output=True)
            assert diff.returncode==1
            patch.append(diff.stdout)
    (out/'candidate.patch').write_bytes(b''.join(patch))
else:
    raise SystemExit('choose red or green')
