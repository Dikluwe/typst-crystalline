"""Focal language evidence for the proposed public selector data; no candidate."""
import datetime
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / '00_nucleo/diagnosticos'
def sha(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()
def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, text=True)
def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()
def save(name, data):
    path = D / ('p1339-where-l0-' + name + '.json')
    assert not path.exists(), path
    body = json.dumps(data, ensure_ascii=False, indent=2) + '\n'
    subprocess.run(['apply_patch'], cwd=ROOT, text=True, input='*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+line+'\n' for line in body.splitlines())+'*** End Patch\n', check=True, capture_output=True)
    return sha(path)
def state():
    return dict(at=now(), head=git('rev-parse', 'HEAD').strip(), status=git('status','--short'), diff_stat=git('diff','HEAD','--stat'))
cases = [
    ('order-repr', 'repr((heading.where(level:2,outlined:false),heading.where(outlined:false,level:2)))'),
    ('order-equality', 'repr(heading.where(level:2,outlined:false)==heading.where(outlined:false,level:2))'),
    ('empty-vs-bare', 'repr((strong.where()==selector(strong),heading.where()==selector(heading)))'),
    ('empty-identity', '{ let f=strong; repr((f.where()==strong.where(),f.where())) }'),
    ('numeric-equality', 'repr(heading.where(level:1)==heading.where(level:1.0))'),
    ('spread-order', 'repr(heading.where(..(level:1,outlined:false),..(level:2)))'),
    ('different-functions', 'repr(strong.where(body:[Hi])==emph.where(body:[Hi]))'),
    ('nan-field', '{ let s=heading.where(level:float("nan")); repr((s,s==s)) }'),
]
inputs = [
    '00_nucleo/prompts/entities/selector.md', '00_nucleo/prompts/entities/show.md',
    '00_nucleo/prompts/compiler/eval/bindings/value_methods.md',
    '00_nucleo/prompts/compiler/eval/selector_matching.md',
    '01_core/src/entities/selector.rs', '01_core/src/entities/show.rs',
    '01_core/src/entities/func.rs', '01_core/src/compiler/eval/bindings/value_methods.rs',
    '01_core/src/compiler/eval/selector_matching.rs',
    'lab/typst-original/crates/typst-library/src/foundations/func.rs',
    'lab/typst-original/crates/typst-library/src/foundations/selector.rs',
]
manifest = save('manifest', dict(state=state(), cases=cases, input_hashes={p:sha(ROOT/p) for p in inputs}, script_sha256=sha(__file__), role='root: L0 author, not independent contract/verdict', budget='one focal normal/reverse default-profile run, no full corpus', purpose='measure ordering and equality before choosing carrier'))
for name,binary in [('vanilla','/usr/local/bin/typst'),('crystalline','/tmp/p1338-target.vlNAmp/release/typst')]:
    start = state()
    rows = []
    for order, sequence in [('normal',cases),('reverse',list(reversed(cases)))]:
        for id, expr in sequence:
            argv=[binary,'eval',expr]
            at=now()
            result=subprocess.run(argv,cwd=ROOT,text=True,capture_output=True,timeout=20)
            rows.append(dict(id=id,order=order,argv=argv,cwd=str(ROOT),start=at,end=now(),exit=result.returncode,stdout=result.stdout,stderr=result.stderr))
    print(name,save(name,dict(state=start,end_state=state(),binary=dict(path=binary,sha256=sha(binary)),manifest_sha256=manifest,rows=rows)),flush=True)
