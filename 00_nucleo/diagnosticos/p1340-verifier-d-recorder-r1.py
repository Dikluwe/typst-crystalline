"""Narrow current-baseline confirmation; immutable inherited sources and predicates."""
import hashlib, importlib.util, json, subprocess, tempfile, time
from pathlib import Path
ROOT=Path('/repos/Antigravity/typst-crystalline'); D=ROOT/'00_nucleo/diagnosticos'
def sha(p): return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def read(n): return json.loads((D/n).read_text())
PINS={
'p1340-baseline.json':'0869202e774bd7b76278365aac7292d45a1c930be42cf83270845c985cb5000a',
'p1339-ab-executor-v4.py':'0f4353a9b2b6ebb66b2ff918cae7580681b7d83308d4801b2d4924a6d6759c97',
'p1339-ab-batch2-cli-plan.json':'e36f89070d5de5c9ac9e340f443255797aea109a42b8cfe71e448bc34dcb380b',
'p1339-ab-batch2-public-predicate.py':'1a7f3ba6759080ef59f6987f5418c3518eb918efaa1ce7e1423d7ff78960b08d',
'p1339-ab-batch2-positive-oracles.json':'2d33aac0f559d377b4275b07a1cdc77997cc90b9d887fdabf5ff9bccf28ab55f',
'p1339-ab-batch2-opaque-oracles.json':'dc612c50cf80e7eedd6072d8956ed3534f09c0c0eba5a16edbce7b95256db182',
}
binary='/tmp/p1339-target.UD8gh7/release/typst'; bh='ec13271aa5984c2eda3115c6e6988d210eeb08432e09f94c0e5343258c4d2dc9'
def check():
    assert all(sha(D/n)==h for n,h in PINS.items())
    assert sha(binary)==bh
    b=read('p1340-baseline.json')['before']
    for p,h in (b['modified']|b['new_sources']).items():
        if p.startswith(('01_core/','02_shell/','03_infra/','04_wiring/')): assert sha(ROOT/p)==h,p
def module(name,file):
    s=importlib.util.spec_from_file_location(name,D/file); m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
check(); transport=module('p1340_transport','p1339-ab-executor-v4.py'); predicate=module('p1340_predicate','p1339-ab-batch2-public-predicate.py')
ids=['context-dependency-context_filtered_set','context-dependency-plain_set_control','ordinary-unrelated-panic']
cases={c['id']:c for c in read('p1339-ab-batch2-cli-plan.json')['cases']}
positive=read('p1339-ab-batch2-positive-oracles.json');opaque=read('p1339-ab-batch2-opaque-oracles.json')
before=transport.state();td=Path(tempfile.mkdtemp(prefix='p1340-verifier-d-'));rows=[];judgments=[];tick=time.monotonic()
for i,name in enumerate(ids):
    c=cases[name];assert sha(c['source_path'])==c['source_sha256'];check()
    row=transport.execute(c,binary,'baseline','default','normal',td,i,bh);rows.append(row)
    judgments.append(predicate.evaluate(row,'candidate',positive,opaque))
check();after=transport.state()
value=dict(schema='p1340-verifier-d-confirmation-r1',regime='executado sem atestação de isolamento',executor='/root/p1311_review',pins=PINS,recorder_sha256=sha(__file__),before=before,after=after,binary=dict(path=binary,sha256=bh),rows=rows,judgments=judgments,processes=3,wall_seconds=time.monotonic()-tick,phase='narrow current-baseline RED evidence; not global P1339 D',instrumental_predecessor=dict(attempted_processes=3,receipt_persisted=False,reason='Tool output exceeded budget while transporting full source inventory; JSON parsing failed before persistence. No result or exact duration credited. These identical cases are a direct confirmation, not oracle recalibration.'),preflight='Explicit P1340 baseline/product pins and inherited input/source pins replace ancestral L0 preflight only; frozen execute/evaluate functions unchanged.')
path=D/'p1340-verifier-d-confirmation-r1.json';assert not path.exists()
body=json.dumps(value,ensure_ascii=True,indent=2)+'\n'
patch='*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+x+'\n' for x in body.splitlines())+'*** End Patch\n'
subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True)
print(json.dumps(dict(path=str(path),sha256=sha(path),processes=3,seconds=value['wall_seconds'],judgments=judgments)))
