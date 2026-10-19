"""Run every pinned linter check and retain the complete diagnostic stream."""
from pathlib import Path
import datetime
import gzip
import hashlib
import importlib.util
import json
import subprocess
from collections import Counter

root = Path.cwd()
out = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('validation', root / 'lab/tools/ci/validate_layout_migration.py')
validation = importlib.util.module_from_spec(spec)
spec.loader.exec_module(validation)
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
utc = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
linter = root / 'lab/target/linter/debug/crystalline-lint'
assert sha(linter) == '421d0797e7ae1a5102e9c5bcd87b488fea70a1997394c38765a03239fe579121'
assert not (out / 'after.sarif.gz').exists()
before = validation.frozen_inputs()
record = {'utc_start': utc(), 'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
          'state': 'uncommitted working tree', 'diff_head_stat': subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True),
          'inputs': before, 'linter_sha256': sha(linter), 'command': [str(linter), '.', '--format', 'sarif']}
run = subprocess.run(record['command'], capture_output=True)
with gzip.open(out / 'after.sarif.gz', 'wb') as f:
    f.write(run.stdout)
(out / 'after.stderr.log').write_bytes(run.stderr)
results = json.loads(run.stdout)['runs'][0]['results']
after = validation.frozen_inputs()
record.update(utc_end=utc(), exit=run.returncode,
              counts=dict(Counter(r['ruleId'] + ':' + r.get('level', 'warning') for r in results)),
              changed_inputs=[p for p, v in before.items() if after.get(p) != v])
with gzip.open(out / 'lint-state.json.gz', 'wt') as f:
    json.dump(record, f, indent=2)
print(json.dumps({k: record[k] for k in ('utc_end', 'exit', 'counts', 'changed_inputs')}))
assert not record['changed_inputs']
assert run.returncode == 1
assert record['counts'].get('V16:error') == 20
assert all(r.get('level') == 'note' or r['ruleId'] == 'V16' for r in results)
