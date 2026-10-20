"""Compare rendered language text, target boundaries and callback diagnostics."""
from pathlib import Path
import argparse
import datetime
import hashlib
import html.parser
import importlib.util
import json
import subprocess

ROOT = Path.cwd()
HERE = Path(__file__).resolve().parent
parser = argparse.ArgumentParser()
parser.add_argument('--product', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
parser.add_argument('--build-receipt', type=Path)
parser.add_argument('--baseline', action='store_true')
args = parser.parse_args()
assert not args.output.exists()
args.output.mkdir(parents=True)
sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
utc = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
spec = importlib.util.spec_from_file_location('validation', ROOT / 'lab/tools/ci/validate_layout_migration.py')
validation = importlib.util.module_from_spec(spec)
spec.loader.exec_module(validation)
inputs = validation.frozen_inputs() if not args.baseline else None
if inputs is not None:
    build = json.loads(args.build_receipt.read_text())
    assert build['inputs'] == inputs and build['product_sha256'] == sha(args.product)
source = HERE / 'target-controls-oracle/receipt.json'
oracle = json.loads(source.read_text())

class Text(html.parser.HTMLParser):
    def __init__(self):
        super().__init__()
        self.text = []
    def handle_data(self, data):
        self.text.append(data)

def warnings(stderr):
    return [line for line in stderr.splitlines() if line.startswith('warning:')]

record = {'utc_start': utc(), 'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
          'diff_head_stat': subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True),
          'product': {'path': str(args.product), 'sha256': sha(args.product)},
          'oracle_receipt_sha256': sha(source), 'baseline': args.baseline, 'cases': []}
for case in oracle['cases']:
    path = args.output / (case['name'] + '.typ')
    path.write_text(case['source'])
    assert sha(path) == case['source_sha256']
    output = path.with_suffix('.' + case['format'])
    command = [str(args.product), 'compile', str(path), str(output)]
    if case['format'] == 'html':
        command += ['--features', 'html']
    started = utc()
    run = subprocess.run(command, capture_output=True, text=True, timeout=60)
    text = None
    if run.returncode == 0:
        if case['format'] == 'pdf':
            text = subprocess.check_output(['pdftotext', str(output), '-'], text=True)
        else:
            observer = Text()
            observer.feed(output.read_text())
            text = ''.join(observer.text)
        text = ' '.join(text.split())
    passed = run.returncode == case['returncode'] and text == case['text'] and warnings(run.stderr) == warnings(case['stderr'])
    record['cases'].append({'name': case['name'], 'source_sha256': sha(path),
                            'argv': command, 'utc_start': started, 'utc_end': utc(),
                            'returncode': run.returncode, 'text': text, 'stderr': run.stderr,
                            'expected_returncode': case['returncode'], 'expected_text': case['text'],
                            'expected_warnings': warnings(case['stderr']), 'passed': passed})
    if not passed:
        print('DIFF', case['name'], run.returncode, repr(text), warnings(run.stderr), flush=True)
record.update(utc_end=utc(), passed=all(c['passed'] for c in record['cases']))
after = validation.frozen_inputs() if inputs is not None else None
record['changed_inputs'] = [] if inputs is None else [p for p in inputs.keys() | after.keys() if inputs.get(p) != after.get(p)]
assert not record['changed_inputs'] and sha(args.product) == record['product']['sha256']
(args.output / 'receipt.json').write_text(json.dumps(record, indent=2) + '\n')
print(sum(c['passed'] for c in record['cases']), '/', len(record['cases']), 'rendered target/diagnostic controls')
raise SystemExit(0 if args.baseline or record['passed'] else 1)
