"""Recheck paths and lineage after publishing the diagnostic summary."""
from pathlib import Path
import datetime
import gzip
import hashlib
import json
import subprocess

root = Path.cwd()
out = Path(__file__).resolve().parent
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
utc = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(out / 'validation/validation-state.json.gz', 'rt') as f:
    state = json.load(f)
commands = [
    ('paths', ['python3', 'lab/tools/ci/check_mature_layout.py']),
    ('strict-lineage', [str(root / 'lab/target/linter/debug/crystalline-lint'), '.', '--checks', 'v1,v5,v15,v26', '--fail-on', 'warning']),
    ('reverse-lineage', ['python3', 'lab/tools/ci/check_reverse_lineage.py']),
    ('authored-diff', ['git', 'diff', '--check']),
]
target = out / 'post-document-validation.json'
assert not target.exists()
record = {'utc_start': utc(), 'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
          'diff_head_stat': subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True), 'commands': []}
for name, argv in commands:
    result = subprocess.run(argv, capture_output=True, text=True)
    path = out / ('post-document-' + name + '.log')
    path.write_text(result.stdout + result.stderr)
    record['commands'].append({'name': name, 'command': argv, 'exit': result.returncode, 'utc_end': utc(), 'log_sha256': sha(path)})
    assert result.returncode == 0, result.stdout + result.stderr
record['changed_validation_inputs'] = [p for p, digest in state['inputs'].items() if sha(root / p) != digest]
assert record['changed_validation_inputs'] == []
record['utc_end'] = utc()
record['current_diagnostic_sha256'] = sha(root / '00_nucleo/diagnostics/v16-correction-progress-2026-10-19.md')
target.write_text(json.dumps(record, indent=2) + '\n')
print('PASS post-document paths, strict/reverse lineage and unchanged validated inputs')
