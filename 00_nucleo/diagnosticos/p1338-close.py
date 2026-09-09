"""Read-only closing checks followed by one immutable P1338 receipt."""
import collections, importlib.util, json, re, sys
from pathlib import Path
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
def load(name):
    spec = importlib.util.spec_from_file_location(name, D / (name + '.py'))
    module = importlib.util.module_from_spec(spec); spec.loader.exec_module(module)
    return module
r = load('p1338-record')
lineage = load('p1334-lineage-lib')
def pin(path, expected):
    p = Path(path)
    if not p.is_absolute(): p = r.ROOT / p
    assert p.is_file() and r.sha(p) == expected, str(p)
def main():
    before = r.state(); r.verify(before)
    b, m, c = r.read('baseline'), r.read('manifest-r1'), r.read('candidate')
    mh = r.sha(D / 'p1338-manifest-r1.json')
    pin(D / 'p1338-baseline.json', m['baseline_sha256'])
    pin(D / 'p1338-measurement.json', m['measurement_sha256'])
    pin(D / 'p1338-manifest.json', m['supersedes_manifest_sha256'])
    pin(D / 'p1337-closure.json', b['previous_closure_sha256'])
    pin(D / 'p1337-review-postclosure.json', b['previous_review_sha256'])
    for group in ['historical_preserved', 'retained_prior_artifacts']:
        for path, digest in b[group].items(): pin(path, digest)
    roots = ['00_nucleo/prompts', '01_core', '02_shell', '03_infra', '04_wiring',
             'Cargo.toml', 'Cargo.lock', 'benches', 'lab/surface-inventory', 'lab/typst-original']
    files = set(filter(None, r.git('ls-files', '--cached', '--others', '--exclude-standard', '-z', '--', *roots).split('\0')))
    assert files == set(b['state']['product_inventory']), 'new or missing productive paths'
    pin(r.SOURCE, c['source_sha256']); assert c['manifest_sha256'] == mh
    h = lineage.hashes(r.L0, r.SOURCE)
    assert h['norm_sha256'] == m['prompt']['normative_sha256']
    assert h['recorded_a'] == h['effective_a'][:8] and h['recorded_b'] == h['code_b'][:8]
    for role in ['tests_freeze', 'attacks_freeze']:
        pin(c[role + '_path'], c[role + '_sha256'])
    for role in ['red_review', 'attacks_tests_freeze']:
        pin(c[role]['path'], c[role]['sha256'])
    pin(c['red_path'], c['red_sha256']); assert r.read('unit-red')['exit'] == 101
    frozen = r.read('tests-freeze')
    for name, digest in frozen['protected'].items(): pin(D / name, digest)
    pin(D / 'p1338-tests-baseline.json', frozen['baseline_runs_sha256'])
    pin(D / 'p1338-tests-expectations.json', frozen['expectations_sha256'])
    gates = ['unit-green', 'candidate-build', 'workspace-tests', 'fmt', 'diff-check', 'lineage-final', 'lint-general']
    for name in gates:
        data = r.read(name)
        assert data['exit'] == 0 and data['manifest_sha256'] == mh, name
        for moment in ['before', 'after']:
            r.verify(data[moment])
            assert data[moment]['product_inventory'][r.SOURCE] == c['source_sha256'], name
    w = r.read('workspace-tests')
    summaries = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored', w['stdout'])
    assert summaries
    counts = dict(zip(['passed', 'failed', 'ignored'], [sum(int(row[i]) for row in summaries) for i in range(3)]))
    assert counts['passed'] > 0 and counts['failed'] == 0
    binary = r.read('candidate-build')['candidate_binary']
    binaries = dict(candidate=binary, vanilla=b['vanilla'], baseline=b['predecessor'])
    for item in binaries.values(): pin(item['path'], item['sha256'])
    runs, comparison = r.read('tests-candidate'), r.read('tests-comparison')
    assert runs['products']['candidate'] == binary
    assert all(x['actual'] == x['expected'] for x in runs['harness_tests'])
    cases = r.read('tests-cases')
    expected = {(x['id'], x['profile']): x for x in r.read('tests-expectations')}
    declared = {x['id']: x['expr'] for x in cases}
    keys = set()
    for row in runs['rows']:
        key = (row['id'], row['profile'], row['order'])
        assert key not in keys; keys.add(key)
        assert row['observation'] == 'Observed' and not row.get('failure')
        assert row['source'] == declared[row['id']] and row['binary_sha256'] == binary['sha256']
        assert {k: row[k] for k in ['exit', 'stdout_base64', 'stderr_base64']} == expected[row['id'], row['profile']]['expected']
    required = {(x['id'], p, o) for x in cases for p in m['policy']['profiles'] for o in m['policy']['orders']}
    assert keys == required and len(keys) == len(runs['rows']) == len(comparison['rows'])
    pin(D / 'p1338-tests-candidate.json', comparison['candidate_runs_sha256'])
    assert all(x['verdict'] == 'Satisfied' for x in comparison['rows'])
    ab_counts = dict(collections.Counter(x['class'] for x in comparison['rows']))
    attacks = r.read('attacks-results')
    assert attacks['complete'] is True
    assert [x['id'] for x in attacks['runs']] == ['C', 'M1', 'M2', 'M3', 'M4']
    retained, executables = {}, set()
    control_count = attacks['runs'][0]['executed_tests']; assert control_count > 0
    for run in attacks['runs']:
        assert run == r.read('attacks-run-' + run['id'])
        assert run['valid_execution'] and run['compiled_typst_core'] and not run['execution_error']
        assert run['candidate_sha256'] == c['source_sha256'] and run['manifest_sha256'] == mh
        assert run['profile_config'] == 'profile.release.package.typst-core.opt-level=0'
        assert run['env']['CARGO_BUILD_JOBS'] == '2' and run['executed_tests'] == control_count
        assert run['exit'] == (0 if run['id'] == 'C' else 101)
        for pk, hk in [('path', 'source_sha256'), ('patch_path', 'patch_sha256'), ('preserved_executable', 'executable_sha256'), ('stdout_path', 'stdout_sha256'), ('stderr_path', 'stderr_sha256')]:
            pin(run[pk], run[hk]); retained[run[pk]] = run[hk]
        executables.add(run['executable_sha256'])
        sout, serr = Path(run['stdout_path']).read_text(), Path(run['stderr_path']).read_text()
        assert re.search(r'Compiling typst-core v[^\n]*\(' + re.escape(run['cwd'] + '/01_core') + r'\)', serr)
        assert 'Finished `release`' in serr and 'Running unittests' in serr
        if run['id'] != 'C': assert 'FAILED' in sout and 'assertion `left == right` failed' in sout + serr
    assert len(executables) == 5
    final = r.read('review-final')
    assert final['verdict'] == 'PASS_SCOPED' and not final['violations'] and not final['unknowns']
    for p in [D / 'p1338-final-report.md', Path(__file__).resolve(), D / 'p1338-attacks-results.json', D / 'p1338-tests-comparison.json']:
        expected_hash = final['inputs'].get(str(p), final['inputs'].get(str(p.relative_to(r.ROOT))))
        assert expected_hash == r.sha(p), 'missing final review pin: ' + p.name
    for path, digest in final['inputs'].items(): pin(path, digest)
    artifacts = {str(p): r.sha(p) for p in D.glob('p1338-*') if p.is_file()}
    after = r.state(); r.verify(after)
    assert all(before[k] == after[k] for k in ['head', 'branch', 'diff', 'staged', 'product_inventory'])
    previous = json.loads((D / 'p1337-closure.json').read_text())
    step = r.ROOT / '00_nucleo/materialization/typst-passo-1338.md'
    r.save('closure', dict(at=r.now(), outcome='P1338_SCOPED_ARRAY_MISSING_FIELD_DIAGNOSTIC_CORRECTION',
        state=after, baseline_sha256=r.sha(D/'p1338-baseline.json'), manifest_sha256=mh,
        previous_closure_sha256=b['previous_closure_sha256'], step=dict(path=str(step), sha256=r.sha(step)),
        binaries=binaries, lineage=h, ab_counts=ab_counts, workspace=counts, productive_mutants_rejected=4,
        artifacts=artifacts, historical_preserved=b['historical_preserved'], retained_prior_artifacts=b['retained_prior_artifacts'],
        retained_mutant_artifacts=retained, predecessor_active_exports_preserved=previous['predecessor_active_exports_preserved'],
        changed_product_paths=sorted(p for p, digest in after['product_inventory'].items() if b['state']['product_inventory'][p] != digest),
        regime=m['regime'], normative_intention_changed_after_effective_freeze=False,
        global_language_parity_claim=False, staged=False, committed=False, closure_algorithm_sha256=r.sha(__file__)))
if __name__ == '__main__': main()
