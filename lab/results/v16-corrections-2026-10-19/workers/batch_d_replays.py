"""Invoke the measured language corpora against the coordinator's frozen build."""
from pathlib import Path
import hashlib
import subprocess
import sys

kind, label, receipt = sys.argv[1:]
base = Path('lab/results/v16-corrections-2026-10-19/batch-d/agents')
assert Path(receipt).is_file()
if kind == 'grid':
    product = Path('lab/target/product/debug/typst')
    command = ['python3', str(base/'grid/replay.py'), '--product', str(product),
               '--sha', hashlib.sha256(product.read_bytes()).hexdigest(),
               '--receipt', receipt, '--output', label,
               '--baseline', 'cli-baseline/receipt.json']
elif kind == 'measurement':
    command = ['python3', str(base/'measurement/replay.py'), label,
               '--build-receipt', receipt]
else:
    raise ValueError(kind)
raise SystemExit(subprocess.call(command))
