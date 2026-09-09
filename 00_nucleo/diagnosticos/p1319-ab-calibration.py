"""Preserve initial measurements, append only focal evidence before freeze."""
import importlib.util
from pathlib import Path

D=Path('/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos')
s=importlib.util.spec_from_file_location('ab',D/'p1319-ab-runner.py')
ab=importlib.util.module_from_spec(s);s.loader.exec_module(ab)
m=ab.read(D/'p1319-ab-baseline-runs.json');f=ab.read(D/'p1319-ab-focal-runs.json')
old=ab.read(D/'p1319-ab-cases-r0.json')['cases'];new=ab.read(ab.CASES)['cases']
by={c['id']:c for c in new}
changes=[]
for c in old:
    n=by[c['id']]
    assert c['expr']==n['expr'] and c['cwd']==n['cwd']
    if c!=n:
        assert c['id'].startswith('new-cross-file-') and c['kind']=='file' and n['kind']=='literal'
        for r in m['rows']:
            if r['id']==c['id'] and r['product']=='baseline':
                assert ab.obs(r)==dict(exit=1,stdout='',stderr='error: current file is outside its sandbox root\n\n')
        changes.append(c['id'])
assert len(changes)==4
assert m['runner_sha256']==ab.sha(D/'p1319-ab-runner-r0.py')
assert m['cases_sha256']==ab.sha(D/'p1319-ab-cases-r0.json')
assert f['runner_sha256']==ab.sha(D/'p1319-ab-runner.py') and f['cases_sha256']==ab.sha(ab.CASES)
assert m['l0_normative_sha256']==f['l0_normative_sha256']==ab.normative()
assert m['binaries']==f['binaries']
newids=set(by)-{c['id'] for c in old}
assert len(newids)==4 and all(i.startswith('new-subdirectory-') for i in newids)
assert {r['id'] for r in f['rows']}==newids
assert len(f['rows'])==32
assert all(r['stderr'].startswith('error: failed to parse CSV (') for r in f['rows'])
m['rows']+=f['rows']
m['calibration_revision']=dict(revision=1,utc=ab.state()['utc'],reason_code='EVAL_IMPORT_SANDBOX_BEFORE_CSV',
    hypothesis='Four exploratory eval imports never reach parsing; preserve the measured unrelated sandbox failure. Four direct subdirectory Path/Str probes test directory-qualified vpath without relying on import.',
    changed_cases=changes,new_cases=sorted(newids),initial_unknown_observations=16,resolved_as_preservation=16,
    new_parsing_red_observations=16,regressions=0,coverage_limit='Cross-file captured base remains unobserved by this eval corpus; requires local validation by another authority.',
    runner_sha256=ab.sha(D/'p1319-ab-runner.py'),cases_sha256=ab.sha(ab.CASES),
    source_measurements={str(D/'p1319-ab-baseline-runs.json'):ab.sha(D/'p1319-ab-baseline-runs.json'),str(D/'p1319-ab-focal-runs.json'):ab.sha(D/'p1319-ab-focal-runs.json')},
    focal_processes=f['processes'],focal_wall_seconds=f['wall_seconds'],focal_before=f['before'],focal_after=f['after'])
m['processes']+=f['processes'];m['wall_seconds']+=f['wall_seconds']
ab.save(D/'p1319-ab-baseline-runs-r1.json',m)
print('revision 1; 16 known sandbox preservation observations; 16 new parsing RED observations; no full replay')
