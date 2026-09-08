"""Repair artifact serialization only, validated by eight focal public executions."""
import importlib.util
from pathlib import Path
spec=importlib.util.spec_from_file_location('ab',str(Path(__file__).with_name('p1318-ab-runner.py')))
ab=importlib.util.module_from_spec(spec);spec.loader.exec_module(ab)
raw_path=ab.D/'p1318-ab-baseline-runs.json';raw=raw_path.read_text()
quoted=False;escape=False;count=0;parts=[]
for ch in raw:
    if quoted and ch=='\n':
        # splitlines discarded U+2028. This provisional repair is accepted only
        # after full affected-row observations match a fresh focal execution.
        parts.append('\\u2028');count+=1
    else:parts.append(ch)
    if ch=='"' and not escape:quoted=not quoted
    if ch=='\\' and not escape:escape=True
    else:escape=False
assert count==8 and not quoted
m=ab.json.loads(''.join(parts));f=ab.read(ab.D/'p1318-ab-writer-focal.json')
assert len(f['rows'])==8
old={(r['id'],r['profile']):r for r in m['rows'] if r['product']=='baseline'}
assert len(old)==513*4
for r in f['rows']:
    assert r['id'] in ('control-valid-unicode-quotes-array','control-valid-unicode-quotes-dictionary') and r['product']=='baseline'
    p=old[r['id'],r['profile']]
    assert ab.obs(p)==ab.obs(r) and p['argv']==r['argv'] and p['cwd']==r['cwd']
    assert '\u2028' in p['stdout']
assert m['runner_sha256']==ab.sha(ab.D/'p1318-ab-runner-r0.py')
assert f['runner_sha256']==ab.sha(ab.D/'p1318-ab-runner.py')
m['writer_revision']=dict(cause='Python splitlines() split raw U+2028 in JSON stdout; only serialization affected.',original_raw_sha256=ab.sha(raw_path),original_runner_sha256=m['runner_sha256'],current_runner_sha256=f['runner_sha256'],focal_sha256=ab.sha(ab.D/'p1318-ab-writer-focal.json'),focal_processes=f['processes'],focal_seconds=f['wall_seconds'],affected_observations=8,expectation_changes=0,full_baseline_reexecutions=0,validation='All eight repaired observations equal complete focal exit/stdout/stderr, argv and cwd. All other records retain original data. Original malformed artifact remains immutable.')
ab.save(ab.D/'p1318-ab-baseline-runs-r1.json',m)
assert ab.read(ab.D/'p1318-ab-baseline-runs-r1.json')==m
print('repaired observations',count,'validated focal',len(f['rows']),'sha256',ab.sha(ab.D/'p1318-ab-baseline-runs-r1.json'))
