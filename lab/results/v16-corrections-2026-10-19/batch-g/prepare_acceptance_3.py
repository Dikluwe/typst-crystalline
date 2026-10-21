"""Freeze the exact actual-build acceptance workers before their final execution."""
from pathlib import Path
import datetime
import hashlib
import json
import subprocess

P = Path(__file__).resolve().parent
R = Path.cwd()
F = P.parent / 'figure-kind-implementation'
sha = lambda x: hashlib.sha256(Path(x).read_bytes()).hexdigest()
build = json.loads((P / 'candidate-build-3/receipt.json').read_text())
freeze = json.loads((P / 'validation-final-3/validation-freeze.json').read_text())
commands = json.loads((P / 'validation-final-3/validation-commands.json').read_text())
assert freeze['completed_jobs'] == len(commands) == 12 and all(c['exit'] == 0 for c in commands)
assert not freeze['changed_inputs'] and build['exit'] == 0 and not build['changed_inputs']
assert all(sha(R / name) == digest for name, digest in build['inputs'].items())
names = [
    Path(__file__).resolve(), P / 'accept_3.py', P / 'hash_measurement_candidate_3.py',
    P / 'agents/core/run_public_hash_probe.py', P / 'agents/core/public_hash_probe.rs',
    P / 'agents/core/replay_equality.py', P / 'agents/core/equality-probes/receipt.json',
    P / 'agents/core/context-intent-probes/receipt.json', P / 'agents/html/replay_cli.py',
    P / 'agents/html/cli-policies.json', F / 'replay.py', F / 'cases.json',
    F / 'run_reference_acceptance.py', F / 'agents/entities/reference-acceptance/run_product.py',
    F / 'target_controls.py', F / 'target-controls-oracle/receipt.json',
    R / 'lab/target/product/debug/typst', R / 'lab/typst-original/target/release/typst',
]
for folder in (P / 'agents/core/equality-probes', P / 'agents/core/context-intent-probes'):
    names.extend(x for x in folder.rglob('*') if x.is_file() and x.suffix in {'.expr', '.typ'})
state = {'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
         'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
         'state': 'Uncommitted working tree; exact candidate3 build and frozen workers before final replay',
         'build_receipt_sha256': sha(P / 'candidate-build-3/receipt.json'),
         'files': {str(n.relative_to(R)): sha(n) for n in sorted(set(names))}}
path = P / 'acceptance-worker-state-3.json'
assert not path.exists()
path.write_text(json.dumps(state, indent=2) + '\n')
print('Frozen acceptance files', len(state['files']), 'product', build['product_sha256'])
