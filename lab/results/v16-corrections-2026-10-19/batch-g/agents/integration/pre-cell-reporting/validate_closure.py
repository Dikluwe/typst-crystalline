"""Recheck final documentation paths and seals without repeating unchanged Rust tests."""
from pathlib import Path
import datetime
import gzip
import hashlib
import importlib.util
import json
import subprocess
import sys

R = Path.cwd()
P = Path(__file__).resolve().parent
out = P / 'closure-checks'
out.mkdir(exist_ok=False)
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
build = json.loads((P / 'candidate-build-final/receipt.json').read_text())
assert all(sha(R / name) == d for name, d in build['inputs'].items())
commands = [
    ('layout', ['python3', 'lab/tools/ci/check_mature_layout.py']),
    ('strict-lineage', ['lab/target/linter/debug/crystalline-lint', '.', '--checks', 'v1,v5,v15,v26', '--fail-on', 'warning']),
    ('reverse-lineage', ['python3', 'lab/tools/ci/check_reverse_lineage.py']),
    ('full-lint', ['python3', str(P / 'lint.py'), '--output', str(P / 'lint-closure-docs'), '--expected-v16', '0']),
    ('whitespace', ['git', 'diff', '--check']),
]
records = []
for name, argv in commands:
    start = datetime.datetime.now(datetime.timezone.utc).isoformat()
    with (out / (name + '.log')).open('w') as log:
        r = subprocess.run(argv, stdout=log, stderr=subprocess.STDOUT)
    records.append({'name': name, 'argv': argv, 'utc_start': start,
                    'utc_end': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'exit': r.returncode})
    (out / 'commands.json').write_text(json.dumps(records, indent=2) + '\n')
    assert r.returncode == 0, name
spec = importlib.util.spec_from_file_location('layout_closure', R / 'lab/tools/ci/check_mature_layout.py')
m = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = m
spec.loader.exec_module(m)
links = []
for source in (P / 'README.md', R / '00_nucleo/diagnostics/v16-correction-progress-2026-10-19.md'):
    active = m._active_markdown(source.read_text())
    for match in m.LINK_RE.finditer(active):
        raw = match.group(2)
        target = m._local_target(R, source, raw)
        if target is None:
            continue
        assert target.is_file(), (source, raw)
        links.append({'source': str(source.relative_to(R)), 'target': raw,
                      'resolved': str(target.resolve().relative_to(R)), 'sha256': sha(target)})
with gzip.open(P / 'lint-final-2/after.sarif.gz', 'rt') as f:
    prior = json.load(f)['runs'][0]['results']
with gzip.open(P / 'lint-closure-docs/after.sarif.gz', 'rt') as f:
    now = json.load(f)['runs'][0]['results']
assert sorted(map(lambda x: json.dumps(x, sort_keys=True), prior)) == sorted(map(lambda x: json.dumps(x, sort_keys=True), now))
assert all(sha(R / name) == d for name, d in build['inputs'].items())
scope = json.loads((P / 'scope.json').read_text())
changed = subprocess.check_output(['git', 'diff', 'HEAD', '--name-only'], text=True).splitlines()
expected = set(scope['sources'] + scope['owners'] + ['00_nucleo/diagnostics/v16-correction-progress-2026-10-19.md'])
assert set(changed) == expected, (set(changed) - expected, expected - set(changed))
subprocess.run(['git', 'merge-base', '--is-ancestor', scope['baseline_head'], 'HEAD'], check=True)
receipt = {'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
           'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
           'diff_head_stat': subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True),
           'input_count': len(build['inputs']), 'changed_inputs': [],
           'all_full_diagnostics_identical_to_executable_final': True,
           'active_changed_paths': changed, 'local_markdown_links': links,
           'ancestry_preserved': True, 'commands': records}
(out / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
print('PASS closure paths, forward/reverse seals, full diagnostics and unchanged executable inputs')
