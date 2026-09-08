"""P1318 command receipts; no historical artifact is overwritten."""
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / '00_nucleo/diagnosticos'
BASE = '/tmp/p1317-target.y5u9ah/release/typst'
OWNER = '01_core/src/compiler/stdlib/loading.rs'
L0 = '00_nucleo/prompts/compiler/stdlib/loading.md'

def sha(path):
    with open(path, 'rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()

def state():
    def git(*args):
        return subprocess.check_output(['git', *args], cwd=ROOT, text=True)
    return dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
        head=git('rev-parse', 'HEAD').strip(), diff_stat=git('diff', 'HEAD', '--stat'),
        diff=git('diff', 'HEAD'), staged=git('diff', '--cached'),
        status=git('status', '--short'), files={n:sha(ROOT/n) for n in (OWNER,L0)})

def save(name, data):
    path = D / ('p1318-' + name + '.json')
    assert not path.exists(), path
    payload = json.dumps(data, ensure_ascii=False, indent=2) + '\n'
    patch = '*** Begin Patch\n*** Add File: ' + str(path) + '\n'
    patch += ''.join('+' + line + '\n' for line in payload.splitlines())
    subprocess.run(['apply_patch'], input=patch+'*** End Patch\n', text=True,
                   capture_output=True, check=True)
    print(str(path), sha(path), flush=True)

if __name__ == '__main__':
    name, argv = sys.argv[1], sys.argv[2:]
    before = state()
    started = time.monotonic()
    p = subprocess.run(argv, cwd=ROOT, capture_output=True, text=True, timeout=1200)
    save(name, dict(before=before, after=state(), argv=argv, cwd=str(ROOT),
         target=os.environ.get('CARGO_TARGET_DIR'), seconds=time.monotonic()-started,
         exit=p.returncode, stdout=p.stdout, stderr=p.stderr, recorder_sha256=sha(__file__)))
    print(p.stdout[-2000:], p.stderr[-2000:])
    sys.exit(p.returncode)
