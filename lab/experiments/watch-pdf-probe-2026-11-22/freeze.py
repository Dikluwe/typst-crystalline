"""Freeze or verify the isolated final candidate source."""
import hashlib
import json
from pathlib import Path
import sys

here = Path(__file__).resolve().parent
source = here / 'source'
manifest = here / 'source-manifest.json'
files = {str(path.relative_to(source)): hashlib.sha256(path.read_bytes()).hexdigest()
         for path in sorted(source.rglob('*')) if path.is_file()}
if sys.argv[1:] == ['verify']:
    assert json.loads(manifest.read_text())['files'] == files
else:
    manifest.write_text(json.dumps({'base_commit': '7c912bded6e58a24801486dec539714b79f18085',
                                    'files': files}, indent=2) + '\n')
print(len(files), hashlib.sha256(manifest.read_bytes()).hexdigest())
