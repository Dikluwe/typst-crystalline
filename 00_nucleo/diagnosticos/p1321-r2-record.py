"""P1321 R2 receipts: both owners, preserving the original R1 recorder."""
import importlib.util
import os
from pathlib import Path
import subprocess
import sys
import time

spec = importlib.util.spec_from_file_location('r1', Path(__file__).with_name('p1321-record.py'))
r1 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r1)
ROOT = r1.ROOT
PAIRS = [
    (r1.OWNER, r1.L0),
    ('01_core/src/compiler/eval/call_dispatch.rs',
     '00_nucleo/prompts/compiler/eval/call_dispatch.md'),
]
sha = r1.sha

def state():
    result = r1.state()
    result['files'] = {name: dict(sha256=sha(ROOT/name), text=(ROOT/name).read_text())
                       for pair in PAIRS for name in pair}
    return result

def save(name, data):
    r1.save('r2-' + name, data)

if __name__ == '__main__':
    name, argv = sys.argv[1], sys.argv[2:]
    before = state()
    start = time.monotonic()
    p = subprocess.run(argv, cwd=ROOT, capture_output=True, text=True, timeout=1500)
    save(name, dict(before=before, after=state(), argv=argv, cwd=str(ROOT),
        target=os.environ.get('CARGO_TARGET_DIR'), seconds=time.monotonic()-start,
        exit=p.returncode, stdout=p.stdout, stderr=p.stderr, recorder_sha256=sha(__file__),
        parent_recorder_sha256=sha(r1.__file__)))
    print(p.stdout[-1400:], p.stderr[-1400:])
    sys.exit(p.returncode)
