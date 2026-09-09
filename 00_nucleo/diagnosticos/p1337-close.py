"""Close the scoped Bool/None/Auto correction using immutable independent evidence.

Read-only verification, then one new receipt. No repair, product execution,
staging, commit, cleanup, or claim of global parity.
"""
import collections, importlib.util, json, re, sys
from pathlib import Path
sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
def load(name):
    spec = importlib.util.spec_from_file_location(name, D / (name + '.py'))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module
r = load('p1337-record')
lineage = load('p1334-lineage-lib')
def pin(path, expected):
    p = Path(path)
    if not p.is_absolute():
        p = r.ROOT / p
    assert p.is_file() and r.sha(p) == expected, str(p)
def main():
    state = r.state()
    r.verify(state)
    baseline, manifest = r.read('baseline'), r.read('manifest-r1')
    manifest_sha = r.sha(D / 'p1337-manifest-r1.json')
    pin(D / 'p1337-baseline.json', manifest['baseline_sha256'])
    pin(D / 'p1337-measurement.json', manifest['measurement_sha256'])
    pin(D / 'p1336-closure.json', baseline['previous_closure_sha256'])
    for group in ['historical_preserved', 'retained_prior_artifacts']:
        for path, digest in baseline[group].items():
            pin(path, digest)
    previous = json.loads((D / 'p1336-closure.json').read_text())
    roots = ['00_nucleo/prompts', '01_core', '02_shell', '03_infra', '04_wiring',
             'Cargo.toml', 'Cargo.lock', 'benches', 'lab/surface-inventory', 'lab/typst-original']
    inventory = set(filter(None, r.git('ls-files', '--cached', '--others', '--exclude-standard', '-z', '--', *roots).split('\0')))
    assert inventory == set(baseline['state']['product_inventory']), 'new or missing productive files'
    candidate = r.read('candidate')
    pin(r.SOURCE, candidate['source_sha256'])
    assert candidate['manifest_sha256'] == manifest_sha
    hashes = lineage.hashes(r.L0, r.SOURCE)
    assert hashes['norm_sha256'] == manifest['prompt']['normative_sha256']
    assert hashes['recorded_a'] == hashes['effective_a'][:8]
    assert hashes['recorded_b'] == hashes['code_b'][:8]
    for role in ['tests_freeze', 'attacks_freeze']:
        assert candidate[role + '_path'].endswith('-r2.json')
        pin(candidate[role + '_path'], candidate[role + '_sha256'])
    frozen = r.read('tests-freeze-r2')
    assert frozen['manifest_sha256'] == manifest_sha
    for group in ['protected', 'protected_successors']:
        for name, digest in frozen[group].items():
            pin(D / name, digest)
    pin(D / 'p1337-tests-freeze-r1.json', frozen['predecessor_freeze_sha256'])
    pin(D / 'p1337-tests-baseline.json', frozen['baseline_runs_sha256'])
    pin(D / 'p1337-tests-expectations.json', frozen['expectations_sha256'])
    for name, digest in r.read('attacks-freeze-r2')['inputs'].items():
        pin(D / name, digest)
    pin(candidate['red_path'], candidate['red_sha256'])
    for name in ['red_review', 'red_mechanical_transport']:
        pin(candidate[name]['path'], candidate[name]['sha256'])
    assert r.read('unit-red-r1')['exit'] == 101
    # R1 execution is transported only by the independent canonical-format proof.
    # It is not represented as an execution of formatted R2 bytes.
    gates = ['unit-green', 'candidate-build', 'workspace-tests', 'fmt',
             'diff-check', 'lineage-final', 'lint-general']
    for name in gates:
        data = r.read(name)
        assert data['exit'] == 0 and data['manifest_sha256'] == manifest_sha, name
        for moment in ['before', 'after']:
            r.verify(data[moment])
            assert data[moment]['product_inventory'][r.SOURCE] == candidate['source_sha256'], name
    workspace = r.read('workspace-tests')
    summaries = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored', workspace['stdout'])
    assert summaries
    counts = dict(zip(['passed', 'failed', 'ignored'], [sum(int(row[i]) for row in summaries) for i in range(3)]))
    assert counts['passed'] > 0 and counts['failed'] == 0
    binary = r.read('candidate-build')['candidate_binary']
    binaries = dict(candidate=binary, vanilla=baseline['vanilla'], baseline=baseline['predecessor'])
    for item in binaries.values():
        pin(item['path'], item['sha256'])
    runs = r.read('tests-candidate')
    assert runs['products']['candidate'] == binary
    for control in runs['harness_tests']:
        assert control['actual'] == control['expected']
    cases = r.read('tests-cases')
    expectations = {(e['id'], e['profile']): e for e in r.read('tests-expectations')}
    declared = {c['id']: c['expr'] for c in cases}
    keys = set()
    for row in runs['rows']:
        key = (row['id'], row['profile'], row['order'])
        assert key not in keys
        keys.add(key)
        assert row['observation'] == 'Observed' and not row.get('failure')
        assert row['binary_sha256'] == binary['sha256'] and row['argv'][0] == binary['path']
        assert row['source'] == declared[row['id']]
        expected = expectations[row['id'], row['profile']]['expected']
        assert {k: row[k] for k in ['exit', 'stdout_base64', 'stderr_base64']} == expected
    required = {(c['id'], p, o) for c in cases for p in manifest['policy']['profiles'] for o in manifest['policy']['orders']}
    assert keys == required and len(keys) == len(runs['rows']) == 540
    comparison = r.read('tests-comparison')
    pin(D / 'p1337-tests-candidate.json', comparison['candidate_runs_sha256'])
    assert len(comparison['rows']) == 540
    assert all(row['verdict'] == 'Satisfied' for row in comparison['rows'])
    ab_counts = dict(collections.Counter(row['class'] for row in comparison['rows']))
    attacks = r.read('attacks-results')
    assert attacks['complete'] is True
    attacks_final = r.read('attacks-final')
    assert attacks_final['unknowns'] == 0 and attacks_final['control_passed']
    assert attacks_final['valid_mutant_families'] == attacks_final['semantic_rejections_observed'] == 5
    for name, digest in attacks_final['inputs'].items():
        pin(D / name, digest)
    assert [run['id'] for run in attacks['runs']] == ['C', 'M1', 'M2', 'M3', 'M4', 'M5']
    retained_mutant_artifacts = {}
    executable_hashes = set()
    for run in attacks['runs']:
        assert run == r.read('attacks-run-' + run['id'])
        assert run['valid_execution'] and run['compiled_typst_core'] and not run['execution_error']
        assert run['candidate_sha256'] == candidate['source_sha256']
        assert run['manifest_sha256'] == manifest_sha
        assert run['profile_config'] == 'profile.release.package.typst-core.opt-level=0'
        assert run['env']['CARGO_BUILD_JOBS'] == '2'
        assert run['exit'] == (0 if run['id'] == 'C' else 101)
        assert run['executed_tests'] == 4
        for path_key, hash_key in [('path', 'source_sha256'), ('patch_path', 'patch_sha256'),
                                   ('preserved_executable', 'executable_sha256'),
                                   ('stdout_path', 'stdout_sha256'), ('stderr_path', 'stderr_sha256')]:
            pin(run[path_key], run[hash_key])
            retained_mutant_artifacts[run[path_key]] = run[hash_key]
        executable_hashes.add(run['executable_sha256'])
        stderr = Path(run['stderr_path']).read_text()
        stdout = Path(run['stdout_path']).read_text()
        pattern = r'Compiling typst-core v[^\n]*\(' + re.escape(run['cwd'] + '/01_core') + r'\)'
        assert re.search(pattern, stderr), 'fresh build not evidenced: ' + run['id']
        assert 'Finished `release`' in stderr and 'Running unittests' in stderr
        if run['id'] != 'C':
            assert 'FAILED' in stdout and 'assertion `left == right` failed' in stdout + stderr
    assert len(executable_hashes) == 6
    final = r.read('review-final')
    assert final['verdict'] == 'PASS_SCOPED'
    assert not final['violations'] and not final['unknowns']
    for p in [D / 'p1337-final-report.md', Path(__file__).resolve(), D / 'p1337-attacks-results.json',
              D / 'p1337-attacks-final.json', D / 'p1337-tests-comparison.json',
              D / 'p1337-tests-summary-r1.json', D / 'p1337-review-final-audit.cjs']:
        expected = final['inputs'].get(str(p), final['inputs'].get(str(p.relative_to(r.ROOT))))
        assert expected == r.sha(p), 'missing final review pin: ' + p.name
    for path, digest in final['inputs'].items():
        pin(path, digest)
    artifacts = {str(p): r.sha(p) for p in D.glob('p1337-*') if p.is_file()}
    after = r.state()
    r.verify(after)
    assert all(after[k] == state[k] for k in ['head', 'branch', 'diff', 'staged', 'product_inventory'])
    step = r.ROOT / '00_nucleo/materialization/typst-passo-1337.md'
    r.save('closure', dict(at=r.now(), outcome='P1337_SCOPED_BOOL_NONE_AUTO_DIAGNOSTIC_CORRECTION',
        state=after, baseline_sha256=r.sha(D / 'p1337-baseline.json'), manifest_sha256=manifest_sha,
        previous_closure_sha256=baseline['previous_closure_sha256'],
        step=dict(path=str(step), sha256=r.sha(step)), binaries=binaries, lineage=hashes,
        ab_counts=ab_counts, workspace=counts, productive_mutants_rejected=5,
        artifacts=artifacts, historical_preserved=baseline['historical_preserved'],
        retained_mutant_artifacts=retained_mutant_artifacts,
        retained_prior_artifacts=baseline['retained_prior_artifacts'],
        predecessor_active_exports_preserved=previous['predecessor_active_exports_preserved'],
        changed_product_paths=sorted(p for p, h in after['product_inventory'].items() if baseline['state']['product_inventory'][p] != h),
        regime=manifest['regime'], normative_intention_changed_after_freeze=False,
        red_revision='R1 execution transported mechanically to formatted R2 by independent review',
        global_language_parity_claim=False, staged=False, committed=False,
        closure_algorithm_sha256=r.sha(__file__)))
if __name__ == '__main__':
    main()
