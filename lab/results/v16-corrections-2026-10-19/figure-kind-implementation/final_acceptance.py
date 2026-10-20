"""Record final CLI acceptance without relabeling the open SetPage failure."""
from pathlib import Path
import datetime
import hashlib
import importlib.util
import json
import subprocess

ROOT = Path.cwd()
HERE = Path(__file__).resolve().parent
PRODUCT = Path('lab/target/product/debug/typst')
BUILD = HERE / 'candidate-build-4/receipt.json'
sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
utc = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
commands = [
    ['python3', str(HERE / 'build.py'), '--output', str(BUILD.parent)],
    ['python3', str(HERE / 'replay.py'), '--product', str(PRODUCT), '--output', str(HERE / 'candidate-replay-4'), '--phase', 'candidate', '--build-receipt', str(BUILD)],
    ['python3', str(HERE / 'target_controls.py'), '--product', str(PRODUCT), '--output', str(HERE / 'candidate-target-controls-4'), '--build-receipt', str(BUILD)],
    ['python3', str(HERE / 'ownership_controls.py'), '--product', str(PRODUCT), '--output', str(HERE / 'ownership-candidate-4')],
]
records = []
for command in commands:
    start = utc()
    run = subprocess.run(command)
    records.append({'argv': command, 'utc_start': start, 'utc_end': utc(), 'exit': run.returncode})
(HERE / 'final-cli-commands.json').write_text(json.dumps(records, indent=2) + '\n')
assert [r['exit'] for r in records] == [0, 0, 1, 0]
controls = json.loads((HERE / 'candidate-target-controls-4/receipt.json').read_text())
assert [r['name'] for r in controls['cases'] if not r['passed']] == ['header-figure-boundary']

spec = importlib.util.spec_from_file_location('validation', ROOT / 'lab/tools/ci/validate_layout_migration.py')
validation = importlib.util.module_from_spec(spec)
spec.loader.exec_module(validation)
build = json.loads(BUILD.read_text())
assert build['inputs'] == validation.frozen_inputs() and build['product_sha256'] == sha(PRODUCT)
out = HERE / 'native-page-running-final'
out.mkdir()
previous = json.loads((HERE / 'native-page-running/receipt.json').read_text())
expected = next(r for r in previous['cases'] if r['name'] == 'oracle')
source = out / 'candidate.typ'
source.write_bytes((HERE / 'native-page-running/oracle.typ').read_bytes())
assert sha(source) == expected['source_sha256']
command = [str(PRODUCT), 'compile', str(source), str(source.with_suffix('.pdf'))]
record = {'utc_start': utc(), 'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
          'diff_head_stat': subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True),
          'argv': command, 'source_sha256': sha(source), 'product_sha256': sha(PRODUCT),
          'build_receipt_sha256': sha(BUILD), 'oracle_receipt_sha256': sha(HERE / 'native-page-running/receipt.json')}
run = subprocess.run(command, capture_output=True, text=True)
text = None if run.returncode else ' '.join(subprocess.check_output(['pdftotext', str(source.with_suffix('.pdf')), '-'], text=True).split())
record.update(utc_end=utc(), exit=run.returncode, text=text, stderr=run.stderr, expected_text=expected['text'])
record['passed'] = run.returncode == 0 and text == expected['text'] and run.stderr == expected['stderr']
(out / 'receipt.json').write_text(json.dumps(record, indent=2) + '\n')
assert record['passed'] and build['inputs'] == validation.frozen_inputs() and build['product_sha256'] == sha(PRODUCT)
print('Final CLI policies84/84; ownership4/4; rendered10/11 with explicit unresolved SetPage; native PageRun passed')
