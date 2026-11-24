"""Bind reported numbers to exact source, binary and measurement windows."""
import hashlib
import json
from pathlib import Path

lab = Path(__file__).resolve().parent
root = lab.parents[2]
results = root / 'lab/results/construction/diagnostics/physical-order-2026-11-21'
record = {'base_commit': '7c912bded6e58a24801486dec539714b79f18085',
          'variants': {}}
for label in ('control', 'reuse', 'strict'):
    out = results / label
    source_manifest = lab / f'{label}-source-manifest.json'
    binary = root / 'lab/target/physical-order-2026-11-21/runs' / label / 'candidate'
    variant = {
        'source_manifest_sha256': hashlib.sha256(source_manifest.read_bytes()).hexdigest(),
        'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest(),
    }
    for name in ('nominal.jsonl', 'paired-published.jsonl', 'paired-reuse.jsonl'):
        path = out / name
        if not path.exists():
            continue
        rows = [json.loads(line) for line in path.read_text().splitlines()]
        variant[name] = {'rows': len(rows),
                         'blocks': len(set(row['block'] for row in rows)),
                         'first_utc': min(row['utc'] for row in rows),
                         'last_utc': max(row['utc'] for row in rows)}
    record['variants'][label] = variant
(results/'measurement-windows.json').write_text(json.dumps(record, indent=2) + '\n')
