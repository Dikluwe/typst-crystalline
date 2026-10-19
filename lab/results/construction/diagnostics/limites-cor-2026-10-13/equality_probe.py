#!/usr/bin/env python3
"""Repeat the measured language equality probes, retaining binary provenance."""
import argparse
import datetime
import gzip
import hashlib
import json
from pathlib import Path
import subprocess

root = Path(__file__).resolve().parents[3]
out = Path(__file__).resolve().parent
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('destination', type=Path)
args = parser.parse_args()
if args.destination.exists():
    parser.error('destination already exists')
with gzip.open(out / 'equality-before.json.gz', 'rt') as archive:
    before = json.load(archive)
expressions = before['expressions'] + [
    '(rgb(90.5 / 255 * 100%,0%,0%).to-hex(), rgb(90.5 / 255 * 100%,0%,0%) == rgb(91,0,0), rgb(90.5 / 255 * 100%,0%,0%) == rgb(90,0,0))'
]
def run(command):
    result = subprocess.run(command, cwd=root, capture_output=True, text=True)
    return dict(command=command, exit=result.returncode, stdout=result.stdout, stderr=result.stderr)
result = dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
    head=run(['git', 'rev-parse', 'HEAD']), status=run(['git', 'status', '--porcelain=v1']),
    diffstat=run(['git', 'diff', 'HEAD', '--stat']), expressions=expressions, binaries={}, probes=[])
for binary in before['binaries']:
    digest = hashlib.sha256((root / binary).read_bytes()).hexdigest()
    if binary.startswith('lab/') and digest != before['binaries'][binary]:
        raise SystemExit('oracle identity changed')
    result['binaries'][binary] = digest
    result['probes'].extend(run([str(root / binary), 'eval', expr, '--format', 'json']) for expr in expressions)
args.destination.write_text(json.dumps(result, indent=2))
n = len(expressions)
for i, expr in enumerate(expressions):
    a, b = result['probes'][i], result['probes'][i + n]
    print(a['exit'], b['exit'], a['stdout'] == b['stdout'], expr)
