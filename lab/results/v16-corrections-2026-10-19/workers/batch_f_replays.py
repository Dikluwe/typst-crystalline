"""Bind accepted language replays to the completed, frozen validation build."""
from pathlib import Path
import datetime, gzip, hashlib, json, subprocess

ROOT = Path.cwd()
OUT = ROOT / 'lab/results/v16-corrections-2026-10-19/batch-f'
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
state_path = OUT / 'validation/validation-state.json.gz'
with gzip.open(state_path, 'rt') as f:
    state = json.load(f)
assert all(sha(ROOT / p) == h for p, h in state['inputs'].items())
commands_path = OUT / 'validation/validation-commands.json'
commands = json.loads(commands_path.read_text())
assert len(commands) == 12 and all(r['exit'] == 0 for r in commands)
assert json.loads((OUT / 'validation/validation-freeze.json').read_text())['changed_inputs'] == []
product = ROOT / 'lab/target/product/debug/typst'
receipt_path = OUT / 'accepted-build-receipt.json'
assert not receipt_path.exists()
receipt_path.write_text(json.dumps({
    'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
    'state': {'path': str(state_path), 'sha256': sha(state_path)},
    'commands': {'path': str(commands_path), 'sha256': sha(commands_path)},
    'product': {'path': str(product), 'sha256': sha(product)},
}, indent=2) + '\n')
jobs = [
    ['python3', str(OUT/'agents/adapters/replay.py'), '--output', str(OUT/'agents/adapters/accepted-replay'), '--validation-state', str(state_path), '--build-receipt', str(receipt_path)],
    ['python3', str(OUT/'agents/geometry/replay.py'), '--output', str(OUT/'agents/geometry/accepted-replay'), '--validation-state', str(state_path)],
    ['python3', str(OUT/'agents/encoding/replay.py'), 'accepted-replay.json', str(state_path), str(receipt_path), sha(product)],
]
results = []
for command in jobs:
    result = subprocess.run(command)
    results.append({'command': command, 'exit': result.returncode})
(OUT/'accepted-replay-commands.json').write_text(json.dumps(results, indent=2)+'\n')
assert all(r['exit'] == 0 for r in results), results
assert sha(product) == json.loads(receipt_path.read_text())['product']['sha256']
assert all(sha(ROOT / p) == h for p, h in state['inputs'].items())
