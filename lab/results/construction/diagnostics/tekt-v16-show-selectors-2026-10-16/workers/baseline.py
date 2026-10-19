from pathlib import Path
import subprocess
import json
import datetime
import hashlib
import gzip

out = Path('00_nucleo/diagnosticos/tekt-v16-show-selectors-2026-10-16')
scope = json.loads((out / 'scope.json').read_text())
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
now = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
record = {'start': now(), 'commands': []}
commands = [
    ('format', ['rustfmt', '--edition', '2021', '--config', 'skip_children=true', *scope['sources']]),
    ('seal', ['python3', str(out / 'workers/seal_owned.py'), *scope['sources']]),
    ('tests', ['cargo', 'test', '--offline', '--locked', '--workspace', '--lib', 'v16_show_selector_', '--', '--test-threads=1']),
    ('architecture', ['/tmp/tekt-linter-update-target/debug/crystalline-lint', '.', '--format', 'sarif']),
    ('probe', ['python3', str(out / 'workers/probe.py'), 'baseline']),
]
for label, command in commands:
    item = {'label': label, 'command': command, 'start': now()}
    print(json.dumps(item), flush=True)
    with (out / ('baseline-' + label + '.log')).open('w') as log:
        r = subprocess.run(command, stdout=log, stderr=subprocess.STDOUT)
    item.update(exit=r.returncode, end=now())
    record['commands'].append(item)
    if label == 'architecture':
        results = json.loads((out / 'baseline-architecture.log').read_text())['runs'][0]['results']
        v16 = [x for x in results if x['ruleId'] == 'V16']
        assert len(v16) == 267 and all(x['level'] == 'error' for x in v16)
        rows = json.loads((out / 'classification.json').read_text())
        selected = [(x['source'], x['line']) for x in rows if x['kind'] == 'emitted-v16']
        actual = {(x['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./'),
                   x['locations'][0]['physicalLocation']['region']['startLine']) for x in v16}
        assert set(selected) <= actual
        record['selected_v16'] = len(selected)
        record['global_v16'] = len(v16)
    (out / 'baseline-result.json').write_text(json.dumps(record, indent=2) + '\n')
    print(json.dumps(item), flush=True)
    assert r.returncode == (1 if label == 'architecture' else 0)
before = json.load(gzip.open(out / 'before.json.gz'))
record['end'] = now()
record['source_hashes'] = {p: sha(p) for p in before['hashes']}
record['files'] = {p: Path(p).read_text() for p in [*scope['sources'], *scope['owners']]}
(out / 'baseline-result.json').write_text(json.dumps(record, indent=2) + '\n')
