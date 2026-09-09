"""Close only the scoped Int/Str correction after all independently judged gates.

Read-only checks followed by one immutable JSON receipt. No product execution,
repair, cleanup, staging, commit, or claim of global language parity.
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
r = load('p1336-record')
lineage = load('p1334-lineage-lib')
def pin(path, expected):
    p = Path(path)
    if not p.is_absolute():
        p = r.ROOT / p
    assert p.is_file() and r.sha(p) == expected, str(p)
def main():
    state = r.state()
    r.verify(state)
    baseline, manifest = r.read('baseline'), r.read('manifest')
    pin(D / 'p1336-baseline.json', manifest['baseline_sha256'])
    pin(D / 'p1336-measurement.json', manifest['measurement_sha256'])
    for path, digest in baseline['historical_preserved'].items():
        pin(path, digest)
    previous = json.loads((D / 'p1335-closure.json').read_text())
    pin(D / 'p1335-closure.json', baseline['prior_closure_sha256'])
    # These are retained active/successor exports, not the lost P1335 R0 bytes.
    for path, digest in previous['export_artifacts'].items():
        pin(path, digest)
    roots = ['00_nucleo/prompts', '01_core', '02_shell', '03_infra', '04_wiring',
             'Cargo.toml', 'Cargo.lock', 'benches', 'lab/surface-inventory', 'lab/typst-original']
    inventory = set(filter(None, r.git('ls-files', '--cached', '--others', '--exclude-standard', '-z', '--', *roots).split('\0')))
    assert inventory == set(baseline['state']['product_inventory']), 'new or missing productive files'
    candidate = r.read('candidate')
    pin(r.SOURCE, candidate['source_sha256'])
    hashes = lineage.hashes(r.L0, r.SOURCE)
    assert hashes['norm_sha256'] == manifest['prompt']['normative_sha256']
    assert hashes['recorded_a'] == hashes['effective_a'][:8]
    assert hashes['recorded_b'] == hashes['code_b'][:8]
    frozen = r.read('tests-freeze')
    pin(D / 'p1336-tests-freeze.json', candidate['tests_freeze_sha256'])
    for name, digest in frozen['protected'].items():
        pin(D / name, digest)
    pin(D / 'p1336-tests-baseline.json', frozen['baseline_runs_sha256'])
    pin(D / 'p1336-tests-expectations.json', frozen['expectations_sha256'])
    pin(D / 'p1336-attacks-plan.md', candidate['attacks_plan_sha256'])
    pin(D / 'p1336-unit-red.json', candidate['red_sha256'])
    pin(D / 'p1336-review-red.md', candidate['red_review_sha256'])
    assert r.read('unit-red')['exit'] == 101
    gates = ['unit-green', 'candidate-build', 'workspace-tests', 'located-control',
             'warnings-control', 'fmt', 'diff-check', 'lineage-final', 'lint-final']
    for name in gates:
        data = r.read(name)
        assert data['exit'] == 0, name
        assert data['manifest_sha256'] == r.sha(D / 'p1336-manifest.json'), name
        for moment in ['before', 'after']:
            r.verify(data[moment])
            assert data[moment]['product_inventory'][r.SOURCE] == candidate['source_sha256'], name
    workspace = r.read('workspace-tests')
    summaries = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored', workspace['stdout'])
    assert summaries
    counts = dict(zip(['passed', 'failed', 'ignored'], [sum(int(row[i]) for row in summaries) for i in range(3)]))
    assert counts['passed'] > 0 and counts['failed'] == 0
    binary = r.read('candidate-build')['binary']
    binaries = dict(candidate=binary, vanilla=baseline['vanilla'], baseline=baseline['predecessor'])
    for item in binaries.values():
        pin(item['path'], item['sha256'])
    runs = r.read('tests-candidate')
    assert runs['products']['candidate'] == binary
    for control in runs['harness_tests']:
        assert control['actual'] == control['expected']
    assert len(runs['rows']) == 540
    for row in runs['rows']:
        assert row['observation'] == 'Observed'
        assert row['binary_sha256'] == binary['sha256']
    comparison = r.read('tests-comparison')
    pin(D / 'p1336-tests-candidate.json', comparison['candidate_runs_sha256'])
    assert len(comparison['rows']) == 540
    assert all(row['verdict'] == 'Satisfied' for row in comparison['rows'])
    ab_counts = dict(collections.Counter(row['class'] for row in comparison['rows']))
    review_channels = r.read('review-channels')
    assert not review_channels['errors']
    pin(D / 'p1336-tests-candidate.json', review_channels['candidate_sha256'])
    attacks = r.read('attacks-final')
    assert attacks['candidate_sha256'] == candidate['source_sha256']
    assert attacks['mandatory_unknown_effective'] == 0
    assert attacks['valid_mutants'] == attacks['rejected_mutants'] == 6
    assert len(attacks['historical_invalid_executions']) == 5
    assert all(row['status'] == 'Unknown' and row['credit'] is False for row in attacks['historical_invalid_executions'])
    for name, digest in attacks['pins'].items():
        pin(D / name, digest)
    retained_mutant_artifacts = {}
    executables = r.read('attacks-executable-preservation')['executables']
    assert [item['id'] for item in executables] == ['C', 'M1', 'M2', 'M3', 'M4', 'M5', 'M6']
    for item in executables:
        pin(item['preserved'], item['sha256'])
        retained_mutant_artifacts[item['preserved']] = item['sha256']
    assert [run['id'] for run in attacks['runs']] == ['C', 'M1', 'M2', 'M3', 'M4', 'M5', 'M6']
    for run in attacks['runs']:
        assert run['valid_execution']
        assert run['status'] == ('passed' if run['id'] == 'C' else 'failed_test')
        assert run['exit'] == (0 if run['id'] == 'C' else 101)
        for channel in ['stdout', 'stderr']:
            pin(run[channel + '_path'], run[channel + '_sha256'])
            retained_mutant_artifacts[run[channel + '_path']] = run[channel + '_sha256']
        if run['id'] != 'C':
            stderr = Path(run['stderr_path']).read_text()
            pattern = r'Compiling typst-core v[^\n]*\(' + re.escape(run['cwd'] + '/01_core') + r'\)'
            assert re.search(pattern, stderr), 'mutant rebuild not evidenced: ' + run['id']
    for mutant in attacks['mutants']:
        pin(mutant['path'], mutant['source_sha256'])
        retained_mutant_artifacts[mutant['path']] = mutant['source_sha256']
    final = r.read('review-final')
    assert final['verdict'] == 'PASS_SCOPED'
    assert not final['violations'] and not final['unknowns']
    # The reviewer must pin this algorithm and the exact substantive report.
    for p in [D / 'p1336-final-report.md', Path(__file__).resolve(), D / 'p1336-attacks-final.json',
              D / 'p1336-tests-comparison.json', D / 'p1336-review-gates.json']:
        expected = final['inputs'].get(str(p), final['inputs'].get(str(p.relative_to(r.ROOT))))
        assert expected == r.sha(p), 'missing final review pin: ' + p.name
    for path, digest in final['inputs'].items():
        pin(path, digest)
    artifacts = {}
    for p in D.glob('p1336-*'):
        if p.is_file():
            artifacts[str(p)] = r.sha(p)
    after = r.state()
    r.verify(after)
    assert all(after[k] == state[k] for k in ['head', 'branch', 'diff', 'staged', 'product_inventory'])
    step = r.ROOT / '00_nucleo/materialization/typst-passo-1336.md'
    r.save('closure', dict(at=r.now(), outcome='P1336_SCOPED_INT_STR_DIAGNOSTIC_CORRECTION',
        state=after, baseline_sha256=r.sha(D / 'p1336-baseline.json'),
        manifest_sha256=r.sha(D / 'p1336-manifest.json'), previous_closure_sha256=baseline['prior_closure_sha256'],
        step=dict(path=str(step), sha256=r.sha(step)), binaries=binaries, lineage=hashes,
        ab_counts=ab_counts, workspace=counts, productive_mutants_rejected=6,
        artifacts=artifacts, historical_preserved=baseline['historical_preserved'],
        retained_mutant_artifacts=retained_mutant_artifacts,
        predecessor_active_exports_preserved=previous['export_artifacts'],
        changed_product_paths=sorted(p for p, h in after['product_inventory'].items() if baseline['state']['product_inventory'][p] != h),
        regime=manifest['regime'], normative_intention_changed_after_freeze=False,
        global_language_parity_claim=False, staged=False, committed=False,
        closure_algorithm_sha256=r.sha(__file__)))
if __name__ == '__main__':
    main()
