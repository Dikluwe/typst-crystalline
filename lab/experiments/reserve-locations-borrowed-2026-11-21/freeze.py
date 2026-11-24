from datetime import datetime, timezone
from pathlib import Path
import hashlib
import json
import subprocess
import sys

experiment = Path(__file__).resolve().parent
root = experiment.parents[2]
output = root / 'lab/results/construction/diagnostics' / experiment.name
source = Path(sys.argv[1])
changed = {
    '01_core/src/compiler/introspect/walk.rs': 'walk.rs',
    '00_nucleo/prompts/core/compiler/introspect/walk.md': 'walk.md',
}

def read_manifest(path):
    return {name.removeprefix('./'): digest for digest, name in
            (line.split(maxsplit=1) for line in path.read_text().splitlines())}

previous = read_manifest(output.parent / 'context-substitution-preservation-2026-11-21/candidate-source.sha256')
before = read_manifest(output / 'source-before.sha256')
known_metadata = '03_infra/src/pipeline.rs'
prior_seal = json.loads((output.parent / 'context-substitution-preservation-2026-11-21/header-seal.json').read_text())
assert before[known_metadata] == prior_seal['sealed_source_sha256']
assert previous[known_metadata] == prior_seal['compiled_source_sha256']
for path, digest in previous.items():
    if not path.startswith('.cargo/') and path != known_metadata:
        assert before[path] == digest, path

now = {path: hashlib.sha256((source / path).read_bytes()).hexdigest() for path in before}
delta = {path for path, digest in before.items() if now[path] != digest}
assert delta == set(changed), sorted(delta)
patch = []
for path, name in changed.items():
    candidate = output / ('candidate-' + name)
    candidate.write_bytes((source / path).read_bytes())
    diff = subprocess.run(['diff', '-u', '--label', 'a/' + path,
                           '--label', 'b/' + path,
                           str(output / ('baseline-' + name)), str(candidate)],
                          stdout=subprocess.PIPE, check=False)
    assert diff.returncode == 1
    patch.append(diff.stdout)
(output / 'candidate.patch').write_bytes(b''.join(patch))
(output / 'freeze.json').write_text(json.dumps({
    'utc': datetime.now(timezone.utc).isoformat(),
    'head': (output / 'head.txt').read_text().strip(),
    'source': str(source),
    'files_verified': len(before),
    'previous_control_metadata_only_difference': known_metadata,
    'changed_files': {path: now[path] for path in sorted(delta)},
}, indent=2) + '\n')
