"""Verify that each measured source tree still matches its frozen manifest."""
import hashlib
import json
from pathlib import Path

lab = Path(__file__).resolve().parent
names = {'control': 'atomized-source', 'reuse': 'reuse-atomized-source',
         'strict': 'strict-atomized-source'}
for label, directory in names.items():
    manifest = json.loads((lab/f'{label}-source-manifest.json').read_text())['files']
    source = lab/directory
    actual = {str(path.relative_to(source)): hashlib.sha256(path.read_bytes()).hexdigest()
              for path in sorted(source.rglob('*')) if path.is_file()}
    assert actual == manifest, label
    print(label, len(actual), 'verified')
