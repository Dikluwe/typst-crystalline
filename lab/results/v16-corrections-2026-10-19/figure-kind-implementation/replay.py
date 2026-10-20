"""Replay frozen Figure policies using semantic JSON and diagnostic observations."""
from pathlib import Path
import argparse
import datetime
import gzip
import hashlib
import importlib.util
import json
import subprocess

ROOT = Path.cwd()
HERE = Path(__file__).resolve().parent
parser = argparse.ArgumentParser()
parser.add_argument('--product', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
parser.add_argument('--phase', choices=['baseline', 'candidate'], required=True)
parser.add_argument('--build-receipt', type=Path)
args = parser.parse_args()
out = args.output.resolve()
assert not out.exists()
out.mkdir(parents=True)
(out / 'sources').mkdir()
sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
utc = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
product = args.product.resolve()
oracle = ROOT / 'lab/typst-original/target/release/typst'
assert sha(oracle) == '9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
cases = json.loads((HERE / 'cases.json').read_text())['cases']
spec = importlib.util.spec_from_file_location('validation', ROOT / 'lab/tools/ci/validate_layout_migration.py')
validation = importlib.util.module_from_spec(spec)
spec.loader.exec_module(validation)
inputs = validation.frozen_inputs() if args.phase == 'candidate' else None
if args.phase == 'candidate':
    assert args.build_receipt
    build = json.loads(args.build_receipt.read_text())
    assert build['product_sha256'] == sha(product)
    assert build['inputs'] == inputs
else:
    assert sha(product) == 'e114a4889a17292590a6a3c2ed0feeafd13dff970215740624fdf9aae0e8ef78'
record = {'utc_start': utc(), 'phase': args.phase,
          'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
          'diff_head_stat': subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True),
          'case_manifest_sha256': sha(HERE / 'cases.json'),
          'binaries': {'product': {'path': str(product), 'sha256': sha(product)},
                       'oracle': {'path': str(oracle), 'sha256': sha(oracle)}},
          'baseline_inputs_receipt_sha256': sha(HERE / 'baseline-inputs.json.gz'),
          'cases': []}

def observation(result):
    if result.returncode == 0:
        return {'returncode': 0, 'json': json.loads(result.stdout)}
    errors = [line for line in result.stderr.splitlines() if line.startswith('error:')]
    return {'returncode': result.returncode, 'error': errors[0] if errors else result.stderr.splitlines()[0]}

for case in cases:
    source = out / 'sources' / (case['name'] + '.typ')
    source.write_text(case['source'])
    assert sha(source) == case['source_sha256']
    command = [str(product), 'query', str(source), '<probe>', '--field', 'value']
    started = utc()
    run = subprocess.run(command, capture_output=True, text=True, timeout=60)
    observed = observation(run)
    expected = case['baseline_product'] if args.phase == 'baseline' else case['expected_at_closure']
    if 'policy' in expected:
        # This predeclared prior debt is characterized, never counted as oracle parity.
        passed = observed in [case['oracle'], {'returncode': 1, 'error': expected['known_diagnostic']}]
    else:
        passed = observed == expected
    record['cases'].append({'name': case['name'], 'disposition': case['disposition'],
                            'source_sha256': sha(source), 'argv': command,
                            'utc_start': started, 'utc_end': utc(), 'stdout': run.stdout,
                            'stderr': run.stderr, 'observed': observed,
                            'expected': expected, 'passed': passed})
    if not passed:
        print('FAIL', case['name'], observed, 'expected', expected, flush=True)
record.update(utc_end=utc(), passed=all(c['passed'] for c in record['cases']))
after = validation.frozen_inputs() if inputs is not None else None
record['changed_inputs'] = [] if inputs is None else [p for p in inputs.keys() | after.keys() if inputs.get(p) != after.get(p)]
assert sha(product) == record['binaries']['product']['sha256']
assert not record['changed_inputs']
(out / 'receipt.json').write_text(json.dumps(record, indent=2) + '\n')
print(args.phase, sum(c['passed'] for c in record['cases']), '/', len(cases), 'frozen policies')
raise SystemExit(0 if record['passed'] else 1)
