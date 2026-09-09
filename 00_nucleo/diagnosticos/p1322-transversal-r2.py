"""Successor audit adapter: actual feature flags on every executed command."""
import importlib.util
import json
from pathlib import Path
import sys
import tempfile
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('transversal_r1', D/'p1322-transversal.py')
t = importlib.util.module_from_spec(spec)
spec.loader.exec_module(t)
r = t.r

def args_for_profile(args, features):
    result = []
    i = 0
    while i < len(args):
        if args[i] == '--features':
            i += 2
            continue
        if args[i].startswith('--features='):
            i += 1
            continue
        result.append(args[i])
        i += 1
    if features:
        result += ['--features', ','.join(sorted(features))]
    assert t.matrix.feature_flags(result) == set(features)
    return result

def case_run(case, bins, directory, profile, reverse):
    gate = t.matrix.profile_gate(case, profile, t.PROFILES)
    if gate is not None:
        return t.matrix.run_case(case, bins['vanilla'], bins['crystalline'], directory, profile, t.PROFILES)
    source = t.matrix.HERE/case['fonte_typ'] if case['fonte_typ'] else None
    features = t.PROFILES[profile]['features']
    observed = {}
    pairs = [('oracle', 'vanilla', 'oraculo'), ('crystalline', 'crystalline', 'cristalino')]
    for key, binary, declaration in reversed(pairs) if reverse else pairs:
        config = case[declaration]
        observed[key] = t.matrix.invoke(bins[binary], args_for_profile(config['args'], features), source, directory/key, config.get('env'))
        assert t.matrix.feature_flags(observed[key]['command']) == set(features)
    oracle, crystal = observed['oracle'], observed['crystalline']
    state, inferred = t.matrix.classify_for_profile(case, profile, t.PROFILES, case['comparison'], oracle, crystal)
    try:
        values = t.matrix.comparison_observables(case['comparison'], oracle, crystal, directory)
    except Exception as error:
        values = {'harness_error': str(error)}
        state, inferred = 'UNKNOWN', 'UNKNOWN'
    return dict(id=case['id'], eixo=case['eixo'], estado=state, classe=inferred,
                oracle=oracle, crystalline=crystal, observed=values, nota=case['nota'])

def main():
    before = r.state()
    r.verify(before)
    prior = json.loads((D/'p1322-transversal.json').read_text())
    inputs = json.loads((D/'p1322-transversal-cases.json').read_text())
    external = Path(inputs['temp'])
    internal = t.functional.FIX/'matrix'
    tmp = Path(tempfile.mkdtemp(prefix='p1322-transversal-r2-', dir='/tmp'))
    pins = [Path(__file__), D/'p1322-transversal.py', D/'p1322-record.py', D/'p1322-transversal-cases.json', r.ROOT/'lab/parity/matrix/runner.py', r.ROOT/'lab/parity/matrix/svg_morphology.py']
    pins += [p for folder in [external/'fixtures', internal/'fixtures'] for p in folder.rglob('*') if p.is_file()]
    frozen = dict(at=r.now(), before=before, manifest_sha256=r.sha(D/'p1322-manifest.json'), inputs={str(p):r.sha(p) for p in pins},
        binaries=prior['binaries'], profiles=t.PROFILES, temp=str(tmp),
        supersedes={'transversal_matrix':r.sha(D/'p1322-transversal.json'), 'location_profiles':r.sha(D/'p1322-location-focal.json')},
        preserved='R1 extra/closures have actual feature flags and remain valid; R1 raw observations retained, not counted as four-profile evidence.',
        reason='Historical run_case only widened commands already declaring features; R2 injects exact active profile on every command, preserving gates and observables.')
    r.save('transversal-r2-freeze', frozen)
    bins = {side:Path(info['path']) for side,info in frozen['binaries'].items()}
    rows, location = [], []
    for phase in ['normal', 'repeat', 'reverse']:
        reverse = phase == 'reverse'
        cases = list(reversed(inputs['matrix']['cases'])) if reverse else inputs['matrix']['cases']
        for profile in t.PROFILES:
            t.matrix.HERE = external
            result = [case_run(c,bins,tmp/phase/profile/c['id'],profile,reverse) for c in cases]
            rows.append(dict(phase=phase,profile=profile,results=result,counts=t.matrix.closed_counts(result)))
            t.matrix.HERE = internal
            for c in cases:
                if c['id'] in ['P1138-S-001','P1138-S-002','P1138-S-003']:
                    row = case_run(c,bins,tmp/'location'/phase/profile/c['id'],profile,reverse)
                    row.update(profile=profile,phase=phase)
                    location.append(row)
            print(phase,profile,t.matrix.closed_counts(result),flush=True)
    t.functional.m.verify(frozen)
    r.verify(r.state())
    r.save('transversal-r2',dict(at=frozen['at'],end=r.now(),before=before,after=r.state(),
        manifest_sha256=r.sha(D/'p1322-manifest.json'),freeze_sha256=r.sha(D/'p1322-transversal-r2-freeze.json'),
        binaries=frozen['binaries'],matrix=rows,location=location,temp=str(tmp),
        artifacts={str(p):r.sha(p) for p in tmp.rglob('*') if p.is_file()}))

if __name__ == '__main__':
    main()
