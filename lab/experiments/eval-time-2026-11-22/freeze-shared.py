"""Record or verify the isolated source tree used by the extra atomization."""
import hashlib
import json
import sys
from datetime import datetime, timezone
from pathlib import Path

here = Path(__file__).resolve().parent
source = here / 'shared-source'
manifest = here / 'shared-source-manifest.json'
files = {str(path.relative_to(source)): hashlib.sha256(path.read_bytes()).hexdigest()
         for path in sorted(source.rglob('*')) if path.is_file()}
if sys.argv[1:] == ['verify']:
    old = json.loads(manifest.read_text())['files']
    assert old == files, (set(old) ^ set(files),
                          [key for key in old.keys() & files.keys() if old[key] != files[key]])
else:
    record = {'base_commit': '7c912bded6e58a24801486dec539714b79f18085',
              'utc': datetime.now(timezone.utc).isoformat(), 'files': files}
    manifest.write_text(json.dumps(record, indent=2) + '\n')
print(len(files), hashlib.sha256(manifest.read_bytes()).hexdigest())
