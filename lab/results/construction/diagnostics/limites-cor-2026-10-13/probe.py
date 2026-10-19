#!/usr/bin/env python3
"""Repeat color limit probes without replacing a previous measurement."""
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

def read(name):
    if (out / name).exists():
        return json.loads((out / name).read_text())
    with gzip.open(out / 'evidence.json.gz', 'rt') as archive:
        return json.loads(json.load(archive)['entries'][name]['text'])

def run(command):
    result = subprocess.run(command, cwd=root, capture_output=True, text=True)
    return dict(command=command, exit=result.returncode, stdout=result.stdout, stderr=result.stderr)

expressions = read('before-probes.json')['expressions']
for name in ['precision-before.json', 'polar-before.json']:
    rows = read(name)['results']
    expressions.extend(row['command'][2] for row in rows[:len(rows)//2])
expressions.append('gradient.linear(red, blue, space: oklch).sample(25%).components()')
results = dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
    head=run(['git', 'rev-parse', 'HEAD']), status=run(['git', 'status', '--porcelain=v1']),
    diffstat=run(['git', 'diff', 'HEAD', '--stat']), expressions=expressions,
    binaries={}, probes=[])
for binary in ['lab/typst-original/target/release/typst', 'target/debug/typst']:
    digest = hashlib.sha256((root / binary).read_bytes()).hexdigest()
    if binary.startswith('lab/') and digest != '7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea':
        raise SystemExit('unratified oracle')
    results['binaries'][binary] = digest
    for expression in expressions:
        results['probes'].append(run([str(root / binary), 'eval', expression, '--format', 'json']))
args.destination.write_text(json.dumps(results, indent=2))
count = len(expressions)
for i, expression in enumerate(expressions):
    a, b = results['probes'][i], results['probes'][i + count]
    print(i, a['exit'], b['exit'], a['stdout'] == b['stdout'], expression)
    if a['stdout'] != b['stdout']:
        print('oracle:', a['stdout'].strip(), 'candidate:', b['stdout'].strip())
