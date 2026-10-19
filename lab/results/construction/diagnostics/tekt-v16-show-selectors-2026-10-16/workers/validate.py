from pathlib import Path
import json
import gzip
import subprocess
import hashlib
import datetime
import os
import collections

out = Path('00_nucleo/diagnosticos/tekt-v16-show-selectors-2026-10-16')
before = json.load(gzip.open(out / 'before.json.gz'))
scope = json.loads((out / 'scope.json').read_text())
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
now = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
hashes = {p: sha(p) for p in before['hashes']}
linter = '/tmp/tekt-linter-update-target/debug/crystalline-lint'
assert sha(linter) == before['linter']['sha256']
record = {'start': now(), 'head': before['head'], 'source_hashes': hashes, 'commands': [],
          'status': subprocess.check_output(['git', 'status', '--porcelain=v1'], text=True),
          'diffstat': subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True)}
commands = [
    ('guard', ['python3', str(out / 'workers/guard.py')]),
    ('probe', ['python3', str(out / 'workers/probe.py'), 'green']),
    ('build', ['cargo', 'build', '--offline', '--locked', '--workspace']),
    ('fmt', ['cargo', 'fmt', '--check', '--all']),
    ('clippy', ['python3', str(out / 'workers/strict.py')]),
    ('lineage', [linter, '.', '--checks', 'v1,v5,v15,v26', '--fail-on', 'warning']),
    ('reverse', ['python3', 'tools/ci/check_reverse_lineage.py']),
    ('architecture', [linter, '.', '--format', 'sarif']),
    ('workspace', ['python3', str(out / 'workers/suite.py')]),
    ('rustdoc', ['cargo', 'doc', '--offline', '--locked', '--workspace', '--no-deps']),
    ('diff', ['git', 'diff', '--check']),
]
for label, command in commands:
    item = {'label': label, 'command': command, 'start': now()}
    print(json.dumps(item), flush=True)
    env = dict(os.environ)
    if label == 'rustdoc':
        env['RUSTDOCFLAGS'] = '-D warnings'
    with (out / ('validation-' + label + '.log')).open('w') as log:
        r = subprocess.run(command, env=env, stdout=log, stderr=subprocess.STDOUT)
    item.update(exit=r.returncode, end=now())
    record['commands'].append(item)
    record['sources_unchanged_during_validation'] = all(sha(p) == h for p, h in hashes.items())
    record['end'] = now()
    (out / 'validation-result.json').write_text(json.dumps(record, indent=2) + '\n')
    print(json.dumps(item), flush=True)
    assert record['sources_unchanged_during_validation']
    assert r.returncode == (1 if label == 'architecture' else 0)
    if label == 'architecture':
        baseline = json.loads((out / 'baseline-architecture.log').read_text())['runs'][0]['results']
        current = json.loads((out / 'validation-architecture.log').read_text())['runs'][0]['results']
        rows = json.loads((out / 'classification.json').read_text())
        expected_removed = []
        for row in rows:
            if row['kind'] != 'emitted-v16':
                continue
            matched = [x for x in baseline if x['ruleId'] == 'V16'
                       and x['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./') == row['source']
                       and x['locations'][0]['physicalLocation']['region']['startLine'] == row['line']]
            assert len(matched) == 1, row
            expected_removed.extend(matched)
        def key(item):
            return json.dumps([item['ruleId'], item['level'],
                item['locations'][0]['physicalLocation']['artifactLocation']['uri'], item['message']], sort_keys=True)
        old = collections.Counter(key(x) for x in baseline if x['ruleId'] != 'V19')
        new = collections.Counter(key(x) for x in current if x['ruleId'] != 'V19')
        removed = collections.Counter(key(x) for x in expected_removed)
        assert old - removed == new, {'added': list((new-old).elements()), 'removed': list((old-new).elements())}
        v16 = [x for x in current if x['ruleId'] == 'V16']
        assert len(v16) == 262 and all(x['level'] == 'error' for x in v16)
        old_notes = collections.Counter(key(x) for x in baseline if x['ruleId'] == 'V19')
        new_notes = collections.Counter(key(x) for x in current if x['ruleId'] == 'V19')
        added_notes = new_notes - old_notes
        assert not old_notes - new_notes
        assert all(x['level'] == 'note' for x in current if x['ruleId'] == 'V19')
        delta = {'old_v16': 267, 'new_v16': 262, 'removed_selected': expected_removed,
                 'added_non_v19': [], 'removed_other_non_v19': [],
                 'new_v19_groups': [json.loads(x) for x in added_notes.elements()],
                 'old_levels': dict(collections.Counter(x['level'] for x in baseline)),
                 'new_levels': dict(collections.Counter(x['level'] for x in current)),
                 'global_gate_still_fails': True}
        (out / 'architecture-delta.json').write_text(json.dumps(delta, indent=2) + '\n')
        (out / 'remaining-v16.json').write_text(json.dumps(v16, indent=2) + '\n')
print(json.dumps({'stage_validation': 'PASS', 'global_architecture_gate': 'FAIL: remaining V16'}), flush=True)
