from pathlib import Path
import datetime, gzip, hashlib, json, re, subprocess

root = Path.cwd()
out = Path(__file__).resolve().parents[1]
b = json.load(gzip.open(out / 'before.json.gz'))
scope = json.load(open(out / 'scope.json'))
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
hashes = {p: sha(p) for p in list(b['hashes']) + scope['new_sources'] + scope['new_owners']}
results = []
for name, rust, before in [
    ('attach', 'source_attach.rs', 'source-probe-before.json'),
    ('family', 'source_family.rs', 'source-family-before.json'),
    ('family-v2', 'source_family_v2.rs', 'source-family-supplement.json'),
    ('lines', 'source_lines.rs', 'source-lines-before.json'),
    ('markers', 'source_markers.rs', 'source-markers-before.json'),
]:
    src = out / 'workers' / rust
    previous = json.load(open(out / before))
    assert previous['source_sha256'] == sha(src), name
    lib = root / 'target/debug/libtypst_core.rlib'
    binary = Path('/tmp') / ('tekt-math-after-' + name)
    cmd = ['rustc', '--edition=2021', str(src), '-L', 'dependency=' + str(root / 'target/debug/deps'), '--extern', 'typst_core=' + str(lib), '-o', str(binary)]
    record = dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(), head=b['head'], source_hashes=hashes, source_sha256=sha(src), library_sha256=sha(lib), command=cmd)
    r = subprocess.run(cmd, capture_output=True, text=True)
    record.update(build_exit=r.returncode, build_stderr=r.stderr)
    assert r.returncode == 0, r.stderr
    r = subprocess.run([str(binary)], capture_output=True, text=True)
    record.update(exit=r.returncode, stdout=r.stdout, stderr=r.stderr, binary_sha256=sha(binary), sources_unchanged=all(sha(p) == h for p, h in hashes.items()))
    assert r.returncode == 0 and record['sources_unchanged'], record
    # Preserve both original measurements and exact post-build stdout. Comparisons
    # differ only in the intentionally printed presence bit of Source wrappers.
    lines = r.stdout.splitlines()
    if name.startswith('family'):
        count = 12 if name == 'family' else 23
        assert len(lines) == count * 2
        pairs = [{'plain': a, 'source': z, 'same': a.replace('source=false', 'source=*') == z.replace('source=true', 'source=*')} for a, z in zip(lines[::2], lines[1::2])]
        record['pairs'] = pairs
        assert all(p['same'] for p in pairs), pairs
    record['end'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
    (out / ('source-after-' + name + '.json')).write_text(json.dumps(record, indent=2) + '\n')
    results.append({'probe': name, 'exit': r.returncode, 'stdout': r.stdout})
print(json.dumps(results, indent=2), flush=True)
