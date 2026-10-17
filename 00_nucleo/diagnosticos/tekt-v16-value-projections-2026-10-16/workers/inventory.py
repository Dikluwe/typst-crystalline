from pathlib import Path
import json
import subprocess
import tempfile
import hashlib
import gzip
import datetime
import collections
import re

root = Path.cwd()
out = root / '00_nucleo/diagnosticos/tekt-v16-value-projections-2026-10-16'
before = json.load(gzip.open(out / 'before.json.gz'))
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
assert all(sha(p) == h for p, h in before['hashes'].items())
cache = Path('/tmp/tekt-linter-update-target/debug/deps')
externs = {name: list(cache.glob('lib' + name + '-*.rlib')) for name in ['tree_sitter', 'tree_sitter_rust', 'serde_json']}
assert all(len(value) == 1 for value in externs.values())
command = ['rustc', '--edition=2021', '--crate-name', 'v16_current_contexts', '-L', 'dependency=' + str(cache)]
for name, paths in externs.items():
    command += ['--extern', name + '=' + str(paths[0])]
record = {'start': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'sarif': '00_nucleo/diagnosticos/tekt-v16-show-selectors-2026-10-16/validation-architecture.log'}
with tempfile.TemporaryDirectory(prefix='tekt-v16-current-contexts-') as temp:
    binary = Path(temp) / 'inventory'
    command += [str(out / 'workers/audit.rs'), '-o', str(binary)]
    with (out / 'inventory-compile.log').open('w') as log:
        result = subprocess.run(command, stdout=log, stderr=subprocess.STDOUT)
    assert result.returncode == 0
    record['compile'] = command
    command = [str(binary), str(root), record['sarif'],
               '00_nucleo/diagnosticos/tekt-v16-show-selectors-2026-10-16/latent-v16.json']
    with (out / 'contexts.json').open('w') as output:
        result = subprocess.run(command, stdout=output, stderr=subprocess.PIPE, text=True)
    assert result.returncode == 0, result.stderr
    record['parse'] = command
rows = json.loads((out / 'contexts.json').read_text())
assert sum(row['status'] == 'emitted-error' for row in rows) == 262
families = collections.defaultdict(list)
for row in rows:
    prefixes = collections.Counter(re.findall(r'\b([A-Z][A-Za-z0-9_]*)::', ' '.join(a['pattern'] for a in row['arms'])))
    for name in ['Some', 'None', 'Ok', 'Err', 'Self']:
        prefixes.pop(name, None)
    family = prefixes.most_common(1)[0][0] if prefixes else 'unresolved'
    families[family].append({k: row.get(k) for k in ['source', 'owner', 'line', 'function', 'pattern_source', 'status']})
(out / 'family-index.json').write_text(json.dumps(dict(families), indent=2) + '\n')
record.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(), rows=len(rows),
              contexts_sha256=sha(out / 'contexts.json'), sources_unchanged=all(sha(p) == h for p, h in before['hashes'].items()))
assert record['sources_unchanged']
(out / 'inventory-result.json').write_text(json.dumps(record, indent=2) + '\n')
print(json.dumps({'rows': len(rows), 'families': {k: len(v) for k,v in families.items()}, 'sources_unchanged': True}))
