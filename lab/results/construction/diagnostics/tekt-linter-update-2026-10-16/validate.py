from pathlib import Path
import os
import subprocess
import json
import datetime
import hashlib
import gzip
import collections
import sys

root = Path.cwd()
out = root / '00_nucleo/diagnosticos/tekt-linter-update-2026-10-16'
source = Path('/tmp/tekt-linter-update-src-e53de6f')
target = Path('/tmp/tekt-linter-update-target')
old = '/tmp/tekt-linter-ci-target/debug/crystalline-lint'
new = str(target / 'debug/crystalline-lint')
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
now = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
before = json.load(gzip.open(out / 'before.json.gz'))
assert all(sha(root / p) == h for p, h in before['hashes'].items())
record = {'start': now(), 'project_head': before['head'], 'commands': [],
          'old_binary_sha256': sha(old), 'frozen_source_hashes':
          {str(p.relative_to(source)): sha(p) for p in sorted(source.rglob('*')) if p.is_file()}}
env = dict(os.environ, CARGO_BUILD_JOBS='1', CARGO_TARGET_DIR=str(target), NO_COLOR='1')

def save():
    record['end'] = now()
    record['project_sources_unchanged'] = all(sha(root / p) == h for p, h in before['hashes'].items())
    (out / 'validation.json').write_text(json.dumps(record, indent=2) + '\n')
    assert record['project_sources_unchanged']

def run(label, command, cwd=root, override=None, expected_exit=0):
    item = {'label': label, 'command': command, 'cwd': str(cwd), 'start': now()}
    print(json.dumps(item), flush=True)
    local_env = dict(env)
    if override:
        local_env.update(override)
    with (out / (label + '.log')).open('w') as log:
        result = subprocess.run(command, cwd=cwd, env=local_env, stdout=log, stderr=subprocess.STDOUT)
    item.update(end=now(), exit=result.returncode)
    record['commands'].append(item)
    save()
    print(json.dumps(item), flush=True)
    if result.returncode != expected_exit:
        raise SystemExit(result.returncode)

if '--resume-policy-check' in sys.argv:
    saved = json.loads((out / 'initial-validation/validation.json').read_text())
    assert all(x['exit'] == 0 for x in saved['commands'][:-1])
    assert saved['commands'][-1]['label'] == 'new-architecture'
    assert saved['commands'][-1]['exit'] == 1
    assert saved['new_binary_sha256'] == sha(new)
    assert saved['frozen_source_hashes'] == record['frozen_source_hashes']
    record = saved
    record['resumed_after_observing_existing_v16_error_policy'] = now()
else:
    run('build-linter', ['cargo', 'build', '--offline', '--locked', '--bin', 'crystalline-lint'], source)
    record['new_binary_sha256'] = sha(new)
    run('v19-unit', ['cargo', 'test', '--offline', '--locked', '--lib', 'v19', '--', '--test-threads=1'], source)
    run('v19-report-cli', ['cargo', 'test', '--offline', '--locked', '--test', 'v19_report_cli', '--', '--test-threads=1'], source)
    run('ci-python', ['python3', '-m', 'unittest', 'discover', '-s', 'tools/ci'])
    run('old-architecture', [old, '.', '--format', 'sarif'])
    run('new-architecture', [new, '.', '--format', 'sarif'], expected_exit=1)
run('lineage', [new, '.', '--checks', 'v1,v5,v15,v26', '--fail-on', 'warning'])
run('reverse-lineage', ['python3', 'tools/ci/check_reverse_lineage.py'])
# Use the workspace target already built by the preceding stages. The linter on
# PATH is the freshly built frozen revision, rather than the installed binary.
run('project-integration', ['cargo', 'test', '--offline', '--locked', '-p', 'typst-wiring',
    '--test', 'crystalline_lint', '--', '--test-threads=1'], override={
    'CARGO_TARGET_DIR': str(root / 'target'), 'PATH': str(target / 'debug') + os.pathsep + env['PATH']})

def results(label):
    return json.loads((out / (label + '.log')).read_text())['runs'][0]['results']
def key(item, with_level=True):
    loc = item.get('locations', [])
    return json.dumps([item['ruleId'], item['level'] if with_level else None, item['message'], loc], sort_keys=True)
old_results = results('old-architecture')
new_results = results('new-architecture')
old_other = collections.Counter(key(x) for x in old_results if x['ruleId'] != 'V19')
new_other = collections.Counter(key(x) for x in new_results if x['ruleId'] != 'V19')
old_content = collections.Counter(key(x, False) for x in old_results if x['ruleId'] != 'V19')
new_content = collections.Counter(key(x, False) for x in new_results if x['ruleId'] != 'V19')
comparison = {
    'old_levels': dict(collections.Counter((x['level'] for x in old_results))),
    'new_levels': dict(collections.Counter((x['level'] for x in new_results))),
    'old_rules': dict(collections.Counter(x['ruleId'] for x in old_results)),
    'new_rules': dict(collections.Counter(x['ruleId'] for x in new_results)),
    'added_non_v19': [json.loads(k) for k in (new_other - old_other).elements()],
    'removed_non_v19': [json.loads(k) for k in (old_other - new_other).elements()],
    'added_non_v19_ignoring_severity': [json.loads(k) for k in (new_content - old_content).elements()],
    'removed_non_v19_ignoring_severity': [json.loads(k) for k in (old_content - new_content).elements()],
    'existing_policy': 'crystalline.toml: V16 = { level = "error" }',
    'global_architecture_gate_passes': False,
}
(out / 'architecture-comparison.json').write_text(json.dumps(comparison, indent=2) + '\n')
record['comparison'] = comparison
assert collections.Counter(x['ruleId'] for x in new_results if x['level'] == 'error') == {'V16':267}
assert not comparison['added_non_v19_ignoring_severity'] and not comparison['removed_non_v19_ignoring_severity'], comparison
assert all(x['ruleId'] == 'V16' or x['level'] == 'note' for x in new_results)
assert all(x['level'] == 'note' and 'subestima' not in x['message']['text']
           for x in new_results if x['ruleId'] == 'V19')
assert record['frozen_source_hashes'] == {
    str(p.relative_to(source)): sha(p) for p in sorted(source.rglob('*')) if p.is_file()
    and 'target' not in p.relative_to(source).parts
}, 'frozen source changed'
save()
print(json.dumps({'targeted_validation': 'PASS', 'global_architecture_gate_passes': False,
                  'new_rules': comparison['new_rules'], 'new_levels': comparison['new_levels']}), flush=True)
