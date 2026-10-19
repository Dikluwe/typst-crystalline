#!/usr/bin/env python3
"""Repeat the stored color probes against the ratified oracle and current debug build."""
import datetime
import hashlib
import gzip
import json
from pathlib import Path
import subprocess

root = Path(__file__).resolve().parents[3]
out = Path(__file__).resolve().parent

def run(command):
    result = subprocess.run(command, cwd=root, capture_output=True, text=True)
    return dict(command=command, exit=result.returncode, stdout=result.stdout, stderr=result.stderr)

expressions = []
for name in ['before-probes.json', 'constructor-probes-before.json']:
    path = out / name
    if path.exists():
        prior = json.loads(path.read_text())
    else:
        with gzip.open(out / 'evidence.json.gz', 'rt') as archive:
            prior = json.loads(json.load(archive)['entries'][name]['text'])
    expressions.extend(prior['expressions'])
expressions.extend([
    'color.hsl(color.spot("A", red).tint(50%)).components()',
    '(color.oklab(rgb(0, 0, 0, 40%)).components(), color.oklch(rgb(0, 0, 0, 40%)).components())',
    'oklch(0, red)',
])
results = dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
    head=run(['git', 'rev-parse', 'HEAD']),
    status=run(['git', 'status', '--porcelain=v1']),
    diffstat=run(['git', 'diff', 'HEAD', '--stat']), expressions=expressions, binaries={}, probes=[])
for binary in ['lab/typst-original/target/release/typst', 'target/debug/typst']:
    digest = hashlib.sha256((root / binary).read_bytes()).hexdigest()
    if binary.startswith('lab/') and digest != '7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea':
        raise SystemExit('unratified oracle')
    results['binaries'][binary] = digest
    for expression in expressions:
        results['probes'].append(run([str(root / binary), 'eval', expression, '--format', 'json']))
path = out / 'after-probes.json'
with path.open('x') as destination:
    json.dump(results, destination, indent=2)
count = len(expressions)
for i, expression in enumerate(expressions):
    oracle, candidate = results['probes'][i], results['probes'][i + count]
    print(i, oracle['exit'], candidate['exit'], oracle['stdout'] == candidate['stdout'], expression)
    if oracle['stdout'] != candidate['stdout']:
        print('  oracle:', oracle['stdout'].strip(), 'candidate:', candidate['stdout'].strip())
