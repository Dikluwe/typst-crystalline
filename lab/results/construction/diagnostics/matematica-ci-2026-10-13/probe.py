#!/usr/bin/env python3
"""Read-only language probes; supply an output path that does not exist yet."""
import argparse
import datetime
import hashlib
import json
from pathlib import Path
import subprocess

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('output', type=Path)
args = parser.parse_args()
if args.output.exists():
    parser.error('output already exists; preserve the previous measurement')
root = Path(__file__).resolve().parents[3]
expressions = [
    '(calc.sin(1), calc.cos(1), calc.tan(1), calc.sin(30deg), calc.cos(60deg))',
    '(calc.asin(0.5), calc.acos(-0.5), calc.atan(1), calc.atan2(-1, 1), calc.atan2(-1, -1))',
    '(calc.sinh(1), calc.cosh(1), calc.tanh(1), calc.asinh(1), calc.acosh(2), calc.atanh(0.5))',
    '(calc.exp(1), calc.ln(2), calc.log(1000), calc.log(8, base: 2), calc.log(81, base: 3))',
    '(calc.sin(-0.0), calc.tan(-0.0), calc.sinh(-0.0), calc.tanh(-0.0), calc.asinh(-0.0), calc.atanh(-0.0))',
    'calc.asin(2)', 'calc.acosh(0)', 'calc.atanh(1)', 'calc.ln(0)',
]
def run(command):
    result = subprocess.run(command, cwd=root, capture_output=True, text=True)
    return dict(command=command, exit=result.returncode, stdout=result.stdout, stderr=result.stderr)

results = dict(
    utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
    head=run(['git', 'rev-parse', 'HEAD']),
    status=run(['git', 'status', '--porcelain=v1']),
    diffstat=run(['git', 'diff', 'HEAD', '--stat']),
    binaries={}, probes=[],
)
for binary in ['lab/typst-original/target/release/typst', 'target/debug/typst']:
    sha = hashlib.sha256((root / binary).read_bytes()).hexdigest()
    if binary.startswith('lab/') and sha != '7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea':
        raise SystemExit('oracle identity does not match the ratified binary')
    results['binaries'][binary] = sha
    for expression in expressions:
        results['probes'].append(run([str(root / binary), 'eval', expression, '--format', 'json']))
args.output.write_text(json.dumps(results, indent=2))
