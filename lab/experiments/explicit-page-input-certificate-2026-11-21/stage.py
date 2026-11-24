import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

experiment = Path(__file__).resolve().parent
root = experiment.parents[2]
source = Path(sys.argv[2])
output = root / 'lab/results/construction/diagnostics' / experiment.name
prev = output.parent / 'physical-paragraph-coverage-2026-11-21/candidate-source.sha256'
files = dict(paged=source/'03_infra/src/pipeline/paged.rs', pdf=source/'03_infra/src/pipeline/pdf.rs')
owners = {name: source/f'00_nucleo/prompts/infra/pipeline/{name}.md' for name in files}
mode = sys.argv[1]
if mode == 'red':
    for line in prev.read_text().splitlines():
        digest, filename = line.split(maxsplit=1)
        assert hashlib.sha256((source/filename).read_bytes()).hexdigest()==digest, filename
    for name, l0 in owners.items():
        (output/f'before-{name}.rs').write_bytes(files[name].read_bytes())
        (output/f'before-{name}.md').write_bytes(l0.read_bytes())
        with l0.open('a') as stream:
            stream.write((experiment/f'{name}.md').read_text())
    check = subprocess.run(['crystalline-lint','--fix-hashes','--dry-run',str(source)],
                           stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    (output/'dry-seal.log').write_text(check.stdout)
    for name, rust in files.items():
        l0 = owners[name]
        line = next(line for line in check.stdout.splitlines() if f'/pipeline/{name}.rs prompt=00_nucleo/prompts/infra/pipeline/{name}.md ' in line)
        seal, code = re.search(r'hash-a=(\w+) hash-b=(\w+)',line).groups()
        rust.write_text(re.sub(r'@prompt-hash \w+',f'@prompt-hash {seal}',rust.read_text(),count=1))
        l0.write_text(re.sub(r'Hash do Código: \w+',f'Hash do Código: {code}',l0.read_text(),count=1))
    path=files['paged']
    path.write_text(path.read_text()+(experiment/'tests.rs').read_text()+(experiment/'stub.rs').read_text())
elif mode == 'green':
    path=files['paged']; orig=path.read_text()
    stub=(experiment/'stub.rs').read_text()
    assert orig.endswith(stub)
    s=orig[:-len(stub)]
    s=s.replace('pub(super) fn compile_paged_in_session(', 'fn compile_paged_native(',1)
    s+= (experiment/'pages.rs').read_text()
    path.write_text(s)
    path=files['pdf']; s=path.read_text()
    needle='    pub shaping: crate::shaper::ShapeSession,'
    assert s.count(needle)==1
    path.write_text(s.replace(needle,needle+'\n    pub(super) lab_pages: std::sync::Mutex<Option<super::paged::LabPageMemo>>,',1))
    changed={str(p.relative_to(source)) for p in [*files.values(),*owners.values()]}
    delta={}
    for line in prev.read_text().splitlines():
        digest, filename=line.split(maxsplit=1); filename=filename.removeprefix('./')
        actual=hashlib.sha256((source/filename).read_bytes()).hexdigest()
        if actual!=digest:
            delta[filename]=actual
    assert set(delta)==changed,sorted(delta)
    patch=[]
    for name in files:
        for ext,path in [('rs',files[name]),('md',owners[name])]:
            candidate=output/f'candidate-{name}.{ext}'
            candidate.write_bytes(path.read_bytes())
            relative=str(path.relative_to(source))
            diff=subprocess.run(['diff','-u','--label','a/'+relative,'--label','b/'+relative,
                                 str(output/f'before-{name}.{ext}'),str(candidate)],capture_output=True)
            assert diff.returncode==1
            patch.append(diff.stdout)
    (output/'candidate.patch').write_bytes(b''.join(patch))
    (output/'freeze.json').write_text(json.dumps(delta,indent=2)+'\n')
else:
    raise SystemExit('choose red or green')
