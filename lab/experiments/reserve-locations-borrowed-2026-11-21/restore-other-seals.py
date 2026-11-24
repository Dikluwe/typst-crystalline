"""Restore exact pre-experiment bytes for prompts outside the owning L0."""
from pathlib import Path
import hashlib
import sys

experiment = Path(__file__).resolve().parent
root = experiment.parents[2]
copy = Path(sys.argv[1])
output = root / 'lab/results/construction/diagnostics' / experiment.name
for row in (output / 'source-before.sha256').read_text().splitlines():
    expected, filename = row.split(maxsplit=1)
    if not filename.startswith('00_nucleo/prompts/') or filename.endswith('/introspect/walk.md'):
        continue
    target = copy / filename
    if hashlib.sha256(target.read_bytes()).hexdigest() == expected:
        continue
    original = root / filename
    if filename == '00_nucleo/prompts/infra/pipeline.md':
        original = experiment.parent / 'context-substitution-preservation-2026-11-21/candidate-pipeline.md'
    body = original.read_bytes()
    assert hashlib.sha256(body).hexdigest() == expected, filename
    target.write_bytes(body)
    print(filename)
