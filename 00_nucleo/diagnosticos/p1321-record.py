"""Immutable local receipts for P1321; no writes to earlier evidence."""
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
OWNER = '01_core/src/compiler/stdlib/loading.rs'
L0 = '00_nucleo/prompts/compiler/stdlib/loading.md'
BASE = '/tmp/p1319-target.VqXtmj/release/typst'
VANILLA = '/usr/local/bin/typst'

def sha(path):
    with open(path, 'rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()

def state():
    def git(*args):
        return subprocess.check_output(['git', *args], cwd=ROOT, text=True)
    return dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
        head=git('rev-parse','HEAD').strip(), diff_stat=git('diff','HEAD','--stat'),
        diff=git('diff','HEAD'), staged=git('diff','--cached'), status=git('status','--short'),
        files={n:dict(sha256=sha(ROOT/n),text=(ROOT/n).read_text()) for n in (OWNER,L0)})

def save(name, data):
    path = D / ('p1321-' + name + '.json')
    if path.exists():
        raise FileExistsError(path)
    payload = json.dumps(data, ensure_ascii=True, indent=2) + '\n'
    patch = '*** Begin Patch\n*** Add File: ' + str(path) + '\n'
    patch += ''.join('+' + line + '\n' for line in payload.rstrip('\n').split('\n'))
    subprocess.run(['apply_patch'], input=patch+'*** End Patch\n', text=True,
                   capture_output=True, check=True)
    print(path, sha(path), flush=True)

if __name__ == '__main__':
    name, argv = sys.argv[1], sys.argv[2:]
    before = state()
    start = time.monotonic()
    p = subprocess.run(argv, cwd=ROOT, capture_output=True, text=True, timeout=1500)
    save(name, dict(before=before, after=state(), argv=argv, cwd=str(ROOT),
        target=os.environ.get('CARGO_TARGET_DIR'), seconds=time.monotonic()-start,
        exit=p.returncode, stdout=p.stdout, stderr=p.stderr, recorder_sha256=sha(__file__)))
    print(p.stdout[-1400:], p.stderr[-1400:])
    sys.exit(p.returncode)
