"""Hash the exact isolated source tree used for a laboratory executable."""
import hashlib
import json
import sys
from datetime import datetime, timezone
from pathlib import Path

root = Path(__file__).resolve().parent
variant = sys.argv[1]
source = root / {'control': 'atomized-source', 'reuse': 'reuse-atomized-source',
                 'strict': 'strict-atomized-source'}[variant]
if not source.is_dir():
    raise SystemExit(source)
files = {}
for path in sorted(source.rglob('*')):
    if path.is_file():
        files[str(path.relative_to(source))] = hashlib.sha256(path.read_bytes()).hexdigest()
record = {'variant': variant, 'base_commit': '7c912bded6e58a24801486dec539714b79f18085',
          'utc': datetime.now(timezone.utc).isoformat(), 'files': files}
out = root / f'{variant}-source-manifest.json'
out.write_text(json.dumps(record, indent=2) + '\n')
print(out, hashlib.sha256(out.read_bytes()).hexdigest(), len(files))
