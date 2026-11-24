import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

experiment = Path(__file__).resolve().parent
root = experiment.parents[2]
source = Path(sys.argv[2])
out = root / 'lab/results/construction/diagnostics' / experiment.name
manifest = out.parent/'explicit-page-input-certificate-2026-11-21/candidate-source.sha256'
rust_name = '01_core/src/compiler/layout/paragraph_emit.rs'
l0_name = '00_nucleo/prompts/core/compiler/layout/paragraph_emit.md'
rust, owner = source/rust_name, source/l0_name
if sys.argv[1]=='red':
    for row in manifest.read_text().splitlines():
        expected,path=row.split(maxsplit=1)
        assert hashlib.sha256((source/path).read_bytes()).hexdigest()==expected,path
    (out/'before.rs').write_bytes(rust.read_bytes())
    (out/'before.md').write_bytes(owner.read_bytes())
    with owner.open('a') as stream:
        stream.write((experiment/'contract.md').read_text())
    check=subprocess.run(['crystalline-lint','--fix-hashes','--dry-run',str(source)],
                         stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True)
    (out/'dry-seal.log').write_text(check.stdout)
    line=next(line for line in check.stdout.splitlines() if f'/{rust_name} prompt={l0_name} ' in line)
    seal,code=re.search(r'hash-a=(\w+) hash-b=(\w+)',line).groups()
    sealed=re.sub(r'@prompt-hash \w+',f'@prompt-hash {seal}',rust.read_text(),count=1)
    owner.write_text(re.sub(r'Hash do Código: \w+',f'Hash do Código: {code}',owner.read_text(),count=1))
    rust.write_text(sealed+(experiment/'tests.rs').read_text()+(experiment/'stub.rs').read_text())
elif sys.argv[1]=='green':
    stub=(experiment/'stub.rs').read_text()
    candidate=rust.read_text()
    assert candidate.endswith(stub)
    candidate=candidate[:-len(stub)]
    header=re.search(r'@prompt-hash \w+',candidate).group()
    baseline=(out/'before.rs').read_text()
    start=baseline.index('pub(super) fn emit_product')
    end=baseline.index('fn emit_line',start)
    code=baseline[:start]+(experiment/'emitter.rs').read_text()+baseline[end:]
    code=re.sub(r'@prompt-hash \w+',header,code,count=1)
    rust.write_text(code+(experiment/'tests.rs').read_text())
    changes={}
    for row in manifest.read_text().splitlines():
        expected,path=row.split(maxsplit=1)
        actual=hashlib.sha256((source/path).read_bytes()).hexdigest()
        if expected!=actual:
            changes[path.removeprefix('./')]=actual
    assert set(changes)=={rust_name,l0_name},changes
    (out/'freeze.json').write_text(json.dumps(changes,indent=2)+'\n')
    patch=[]
    for ext, rel in [('rs',rust_name),('md',l0_name)]:
        target=out/f'candidate.{ext}'
        target.write_bytes((source/rel).read_bytes())
        diff=subprocess.run(['diff','-u','--label','a/'+rel,'--label','b/'+rel,
                             str(out/f'before.{ext}'),str(target)],capture_output=True)
        assert diff.returncode==1
        patch.append(diff.stdout)
    (out/'candidate.patch').write_bytes(b''.join(patch))
else:
    raise SystemExit('choose red or green')
