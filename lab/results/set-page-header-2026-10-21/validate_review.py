"""Validate the review package without altering productive inputs or seals."""
from pathlib import Path
from collections import Counter
import datetime
import gzip
import hashlib
import importlib.util
import json
import re
import subprocess

ROOT = Path.cwd()
HERE = Path(__file__).resolve().parent
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
utc = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
spec = importlib.util.spec_from_file_location('validation', ROOT / 'lab/tools/ci/validate_layout_migration.py')
validation = importlib.util.module_from_spec(spec)
spec.loader.exec_module(validation)
before = validation.frozen_inputs()
build = json.loads((HERE / 'baseline-build/receipt.json').read_text())
assert before == build['inputs'], 'review must not change productive sources or L0'
jobs = json.loads((HERE / 'validation-baseline/validation-commands.json').read_text())
freeze = json.loads((HERE / 'validation-baseline/validation-freeze.json').read_text())
assert len(jobs) == 12 and all(j['exit'] == 0 for j in jobs)
assert freeze['changed_inputs'] == [] and freeze['completed_jobs'] == freeze['expected_jobs'] == 12
history = json.loads((HERE / 'history-baseline.json').read_text())
assert all(sha(ROOT / p) == h for p, h in history['paths'].items())
record = {'utc_start': utc(), 'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
          'diff_head_stat': subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True),
          'status': subprocess.check_output(['git', 'status', '--short'], text=True),
          'productive_inputs_unchanged': True, 'full_validation_jobs': len(jobs),
          'history_unchanged': True, 'commands': [], 'links': []}
out = HERE / 'review-checks'
assert not out.exists()
out.mkdir()
commands = [
    ('layout', ['python3', 'lab/tools/ci/check_mature_layout.py']),
    ('strict-lineage', ['lab/target/linter/debug/crystalline-lint', '.', '--checks', 'v1,v5,v15,v26', '--fail-on', 'warning']),
    ('reverse-lineage', ['python3', 'lab/tools/ci/check_reverse_lineage.py']),
    ('full-lint', ['python3', str(HERE / 'lint.py'), '--output', str(HERE / 'lint-review'), '--expected-v16', '0']),
    ('whitespace', ['git', 'diff', '--check']),
]
for name, argv in commands:
    started = utc()
    run = subprocess.run(argv, capture_output=True)
    (out / (name + '.stdout')).write_bytes(run.stdout)
    (out / (name + '.stderr')).write_bytes(run.stderr)
    record['commands'].append({'name': name, 'argv': argv, 'utc_start': started, 'utc_end': utc(), 'exit': run.returncode})
    assert run.returncode == 0, name
def results(path):
    return Counter(json.dumps(r, sort_keys=True) for r in json.loads(gzip.decompress(path.read_bytes()))['runs'][0]['results'])
assert results(HERE / 'lint-review/after.sarif.gz') == results(HERE / 'lint-baseline/after.sarif.gz')
record['complete_lint_multiset_unchanged'] = True
for doc in [HERE / 'README.md', HERE / 'review.md', ROOT / '00_nucleo/diagnostics/set-page-header-2026-10-21.md']:
    for link in re.findall(r'\]\(([^)]+)\)', doc.read_text()):
        if link.startswith(('https:', 'http:', '#')):
            continue
        dest = (doc.parent / link.split('#')[0]).resolve()
        assert dest.is_file(), (doc, link)
        record['links'].append({'source': str(doc.relative_to(ROOT)), 'target': link,
                                'resolved': str(dest.relative_to(ROOT)), 'sha256': sha(dest)})
after = validation.frozen_inputs()
assert before == after
record.update(utc_end=utc(), input_count=len(before), changed_inputs=[],
              branch=subprocess.check_output(['git', 'branch', '--show-current'], text=True).strip())
(out / 'receipt.json').write_text(json.dumps(record, indent=2) + '\n')
print('Review paths, lineage, unchanged productive inputs/history and complete lint verified.')
