from pathlib import Path
import datetime, gzip, hashlib, json, os, subprocess

out = Path(__file__).resolve().parents[1]
b = json.load(gzip.open(out / 'before.json.gz'))
scope = json.load(open(out / 'scope.json'))
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
now = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
hashes = {p: sha(p) for p in list(b['hashes']) + list(json.load(open(out / 'probe-fixtures-repair.json'))['fixtures'])}
linter = b['linter']['binary']
assert sha(linter) == b['linter']['sha256']
record = dict(start=now(), head=b['head'], source_hashes=hashes, status=subprocess.check_output(['git', 'status', '--porcelain=v1'], text=True), diffstat=subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True), commands=[])
commands = [
    ('guard', ['python3', str(out / 'workers/guard.py')], 0),
    ('controls', ['cargo','test','--offline','--locked','-p','typst-core','-p','typst-infra','--lib','--no-fail-fast','v16_whitespace_','--','--test-threads=1'], 0),
    ('probe-baseline', ['python3', str(out / 'workers/probe.py'), 'baseline'], 0),
    ('probe', ['python3', str(out / 'workers/probe.py'), 'green'], 0),
    ('build', ['cargo', 'build', '--offline', '--locked', '--workspace'], 0),
    ('public-probes', ['python3', str(out / 'workers/measure_after.py')], 0),
    ('fmt', ['cargo', 'fmt', '--all', '--check'], 0),
    ('clippy', ['python3', str(out / 'workers/strict.py')], 0),
    ('lineage', [linter, '.', '--checks', 'v1,v5,v15,v26', '--fail-on', 'warning'], 0),
    ('reverse', ['python3', 'tools/ci/check_reverse_lineage.py'], 0),
    ('architecture', [linter, '.', '--format', 'sarif'], 1),
    ('differential', ['python3', str(out / 'workers/differential.py')], 0),
    ('workspace', ['python3', str(out / 'workers/suite.py')], 0),
    ('rustdoc', ['cargo', 'doc', '--offline', '--locked', '--workspace', '--no-deps'], 0),
    ('diff', ['git', 'diff', '--check'], 0),
]
for label, cmd, expected in commands:
    item = dict(label=label, command=cmd, start=now())
    print(json.dumps(item), flush=True)
    env = dict(os.environ)
    if label == 'rustdoc':
        env['RUSTDOCFLAGS'] = '-D warnings'
    with (out / ('validation-' + label + '.log')).open('w') as f:
        r = subprocess.run(cmd, stdout=f, stderr=subprocess.STDOUT, env=env)
    item.update(exit=r.returncode, end=now())
    record['commands'].append(item)
    record.update(end=now(), sources_unchanged_during_validation=all(sha(p) == h for p, h in hashes.items()))
    (out / 'validation-result.json').write_text(json.dumps(record, indent=2) + '\n')
    print(json.dumps(item), flush=True)
    assert record['sources_unchanged_during_validation'] and r.returncode == expected, item
record['validation_complete'] = True
(out / 'validation-result.json').write_text(json.dumps(record, indent=2) + '\n')
print('VALIDATION PASS', flush=True)
