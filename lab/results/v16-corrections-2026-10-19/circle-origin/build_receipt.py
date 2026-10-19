"""Bind a successfully built executable to its frozen working-tree inputs."""
from pathlib import Path
import datetime
import gzip
import hashlib
import json
import subprocess
import sys

root = Path.cwd()
out = Path(__file__).resolve().parent
phase = sys.argv[1]
assert phase in ('candidate', 'accepted')
state = out / ('candidate-run/state.json.gz' if phase == 'candidate' else 'validation/validation-state.json.gz')
commands = out / ('candidate-run/commands.json' if phase == 'candidate' else 'validation/validation-commands.json')
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
with gzip.open(state, 'rt') as f:
    frozen = json.load(f)
assert all(sha(root / p) == value for p, value in frozen['inputs'].items())
results = json.loads(commands.read_text())
assert results and all(r['exit'] == 0 for r in results), results
assert any(r['name'] in ('product-build', 'build') for r in results)
if phase == 'candidate':
    # The orchestration log will grow after this job; preserve its build prefix.
    commands = out / 'candidate-build-commands.json'
    assert not commands.exists()
    commands.write_text(json.dumps(results, indent=2) + '\n')
if phase == 'accepted':
    assert len(results) == 12
    assert not json.loads((out / 'validation/validation-freeze.json').read_text())['changed_inputs']
product = root / 'lab/target/product/debug/typst'
record = {
    'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
    'state_note': 'uncommitted working tree; exact changes and frozen inputs in state',
    'state': {'path': str(state), 'sha256': sha(state)},
    'commands': {'path': str(commands), 'sha256': sha(commands)},
    'product': {'path': str(product), 'sha256': sha(product)},
    'config_inputs': {p: sha(root / p) for p in ('Cargo.toml', 'Cargo.lock', 'crystalline.toml', '.cargo/config.toml', 'rust-toolchain.toml')},
}
target = out / (phase + '-build-receipt.json')
assert not target.exists(), 'preserve previous attempts'
target.write_text(json.dumps(record, indent=2) + '\n')
print(phase, record['product']['sha256'])
