"""Vanilla-only stabilization boundaries, prior to the concrete L0 design."""
import datetime
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
BIN = '/usr/local/bin/typst'
PIN = '7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'
CASES = {
    'oscillating_set': '#let c = counter(heading.where())\n#context c.update(if c.final().first() == 0 { 1 } else { 0 })\n#context metadata(c.final())',
    'increasing_set': '#let c = counter(heading.where())\n#context c.update(c.final().first() + 1)\n#context metadata(c.final())',
    'nan_key_stable_zero': '#let c = counter(heading.where(level: float.nan))\n#context c.update(12)\n#context { assert(c.get() == (0,)); metadata(c.get()) }',
    'fresh_callback_stable': '#let c = counter(heading.where())\n#set heading(numbering: "1")\n= A\n#context c.update(n => n + 10)\n= B\n#context { assert(c.get() == (12,)); metadata(c.get()) }',
    'nested_producer': '#let c = counter(heading.where())\n#context [#context c.update(12)]\n#context { assert(c.get() == (12,)); metadata(c.get()) }',
    'unrelated_error': '#let c = counter(heading.where())\n#context panic("unrelated-error")\n#context c.update(12)\n#context { assert(c.get() == (12,)); metadata(c.get()) }',
}

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def sha(path):
    with open(path, 'rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()

def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, text=True)

assert sha(BIN) == PIN
result = dict(start=now(), head=git('rev-parse', 'HEAD').strip(),
              status=git('status', '--short'), diff_stat=git('diff', 'HEAD', '--stat'),
              baseline=dict(path=BIN, sha256=PIN, upstream='a51e02804'),
              runner_sha256=sha(__file__), cases=CASES, runs=[],
              role='root, exploratory measurement after reading source; not an independent oracle',
              budget='six cases, two orders, no adaptive revision; no full corpus')
for order, names in [('normal', list(CASES)), ('reverse', list(reversed(CASES)))]:
    for case in names:
        source = CASES[case]
        argv = [BIN, 'query', '-', 'metadata', '--field', 'value', '--format', 'json']
        start = now()
        proc = subprocess.run(argv, cwd=ROOT, input=source, text=True, capture_output=True, timeout=30)
        result['runs'].append(dict(case=case, order=order, argv=argv, cwd=str(ROOT),
                                  source_sha256=hashlib.sha256(source.encode()).hexdigest(),
                                  start=start, end=now(), exit=proc.returncode,
                                  stdout=proc.stdout, stderr=proc.stderr))
result.update(end=now(), status_after=git('status', '--short'), diff_stat_after=git('diff', 'HEAD', '--stat'))
print(json.dumps(result, ensure_ascii=False, indent=2))
