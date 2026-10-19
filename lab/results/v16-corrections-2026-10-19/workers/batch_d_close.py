"""Close batch D only after independently checking the completed frozen receipts.

Run from the repository root after final validation/replays finish. Archived
candidate/structural/replay directory labels can be supplied explicitly; missing
or incomplete evidence fails closed before any closure artifact is written.
"""
import argparse
from collections import Counter
import datetime
import gzip
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path.cwd()
OUT = ROOT / 'lab/results/v16-corrections-2026-10-19/batch-d'
ORACLE_SHA = '9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
ORACLE_COMMIT = 'bd5ebec285505889e8ff1d4f68804ab147a699f3'
ORACLE_TREE = 'cac836c07e4dec2a89219370428292393a3216b5'
SENTINEL = 'TektV16EvolutionProbe'
EXPECTED_IDS = {f'V16-{n:03d}' for n in range(29, 48)} | {'V16-077'}
SUPPLEMENT_IDS = {'V16-045-selector', 'V16-045-kind', 'V16-040-certificate'}
EVIDENCE = {}


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def read(path):
    path = OUT / path
    EVIDENCE[str(path.relative_to(ROOT))] = sha(path)
    raw = gzip.decompress(path.read_bytes()) if path.suffix == '.gz' else path.read_bytes()
    return json.loads(raw)


def checked_text(path):
    path = OUT / path
    EVIDENCE[str(path.relative_to(ROOT))] = sha(path)
    return path.read_text()


def same_files(hashes, expected=None):
    assert hashes, 'empty input freeze'
    if expected is not None:
        assert set(hashes) == set(expected), 'input freeze inventory differs'
    for path, digest in hashes.items():
        assert sha(ROOT / path) == digest, f'input changed: {path}'


def jobs(path, expected_names, plan=None):
    rows = read(path)
    assert [r['name'] for r in rows] == expected_names, (path, 'incomplete or different jobs')
    assert all(r['exit'] == 0 and r.get('utc_end') for r in rows), path
    if plan is not None:
        assert [(r['name'], r['command']) for r in rows] == [(r['name'], r['command']) for r in plan]
    for row in rows:
        memory = row.get('memory')
        if memory:
            assert memory['memory.max'] == str(12 * 1024 ** 3)
            assert memory['memory.swap.max'] == '0'
    return rows


def outcomes(text, names):
    found = {}
    for name in names:
        matches = re.findall(r'^test [^\n]*::' + re.escape(name) + r' \.\.\. (ok|FAILED|ignored)\s*$', text, re.M)
        assert len(matches) == 1, (name, 'missing/duplicate test observation', matches)
        found[name] = matches[0]
    return found


def artifact(path, digest):
    path = Path(path)
    if not path.is_absolute():
        path = ROOT / path
    assert sha(path) == digest, f'replay artifact changed: {path}'
    EVIDENCE[str(path.relative_to(ROOT))] = digest
    return path


def build_binding(path, digest, validation_state_path):
    actual = artifact(path, digest)
    assert actual.resolve() == validation_state_path.resolve(), 'accepted replay is not tied to final build state'


def structural_record(record, expected_ids, expected_count):
    assert record['exit'] == 101 and not record['other_errors']
    assert record['selected_e0004'] == expected_count
    assert len(record['targets']) == expected_count
    assert {t['id'] for t in record['targets']} == expected_ids
    for target in record['targets']:
        assert target['diagnostics'], target['id']
        for diagnostic in target['diagnostics']:
            assert (diagnostic.get('code') or {}).get('code') == 'E0004'
            assert SENTINEL in diagnostic['message'], target['id']
            assert any(
                span['is_primary']
                and span['file_name'].removeprefix(record['cwd'] + '/') == target['file']
                and span['line_start'] == target['line']
                and target['byte_start'] <= span['byte_start'] < target['byte_end']
                for span in diagnostic['spans']
            ), target['id']


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--candidate', default='candidate-run')
    parser.add_argument('--structural', default='structural-green')
    parser.add_argument('--grid', default='agents/grid/accepted-replay')
    parser.add_argument('--measurement', default='agents/measurement/accepted-replay')
    args = parser.parse_args()
    for label in vars(args).values():
        assert not Path(label).is_absolute() and '..' not in Path(label).parts

    scope = read('scope.json')
    assert len(scope['ids']) == 20 and set(scope['ids']) == EXPECTED_IDS
    assert len(scope['sources']) == len(set(scope['sources'])) == 15
    assert len(scope['owners']) == len(set(scope['owners'])) == 15
    owned = scope['sources'] + scope['owners']

    gate_names = ['python-ci-tests', 'layout', 'strict-lineage', 'reverse-lineage', 'build',
                  'tests-domain-infra', 'tests-wiring', 'doctests', 'build-all-targets',
                  'clippy', 'fmt', 'rustdoc']
    validation = jobs('validation/validation-commands.json', gate_names)
    cargo = ['cargo']
    locked = ['--offline', '--locked']
    serial = ['--', '--test-threads=1']
    expected_commands = {
        'python-ci-tests': ['python3', '-m', 'unittest', 'discover', '-s', 'lab/tools/ci'],
        'layout': ['python3', 'lab/tools/ci/check_mature_layout.py'],
        'strict-lineage': [str(ROOT / 'lab/target/linter/debug/crystalline-lint'), '.', '--checks', 'v1,v5,v15,v26', '--fail-on', 'warning'],
        'reverse-lineage': ['python3', 'lab/tools/ci/check_reverse_lineage.py'],
        'build': cargo + ['build'] + locked + ['--workspace'],
        'tests-domain-infra': cargo + ['test'] + locked + ['--workspace', '--exclude', 'typst-wiring', '--lib', '--bins', '--tests', '--no-fail-fast'] + serial,
        'tests-wiring': cargo + ['test'] + locked + ['-p', 'typst-wiring', '--bins', '--tests', '--no-fail-fast'] + serial,
        'doctests': cargo + ['test'] + locked + ['--workspace', '--doc', '--no-fail-fast'] + serial,
        'build-all-targets': cargo + ['build'] + locked + ['--workspace', '--all-targets'],
        'clippy': cargo + ['clippy'] + locked + ['--workspace', '--all-targets', '--', '-D', 'warnings', '-D', 'unfulfilled_lint_expectations'],
        'fmt': cargo + ['fmt', '--check', '--all'],
        'rustdoc': cargo + ['doc'] + locked + ['--workspace', '--no-deps'],
    }
    assert all(r['command'] == expected_commands[r['name']] for r in validation)
    freeze = read('validation/validation-freeze.json')
    assert freeze['changed_inputs'] == []
    assert freeze['completed_jobs'] == freeze['expected_jobs'] == len(validation) == 12
    assert freeze['input_count'] == 2131
    state = read('validation/validation-state.json.gz')
    assert len(state['inputs']) == 2131
    same_files(state['inputs'])
    assert state['oracle_sha256'] == ORACLE_SHA
    assert state['linter_commit'] == '12797073972e375d9ab078cf6750c8b30b71e2b9'
    assert state['checkpoint'] == subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
    for row in validation:
        assert row['environment']['CARGO_BUILD_JOBS'] == '1'
        checked_text('validation/' + row['name'] + '.log')
    by_gate = {r['name']: r for r in validation}
    assert by_gate['clippy']['command'][-4:] == ['-D', 'warnings', '-D', 'unfulfilled_lint_expectations']
    assert by_gate['rustdoc']['overrides'].get('RUSTDOCFLAGS') == '-D warnings'
    assert by_gate['fmt']['command'] == ['cargo', 'fmt', '--check', '--all']
    assert by_gate['strict-lineage']['command'][-4:] == ['--checks', 'v1,v5,v15,v26', '--fail-on', 'warning']

    outer_names = ['clean-product-packages', 'full-validation', 'accepted-grid-replay', 'accepted-measurement-replay']
    outer = jobs('validation-run/commands.json', outer_names, read('validation-jobs.json'))
    candidate_names = ['core-tests', 'build', 'grid-replay', 'measurement-replay', 'structural-green']
    candidate = jobs(args.candidate + '/commands.json', candidate_names, read('candidate-jobs.json'))
    for directory in [args.candidate, args.structural, 'validation-run']:
        assert read(directory + '/freeze.json')['changed'] == [], directory
    for directory in [args.candidate, 'validation-run']:
        run_state = read(directory + '/state.json.gz')
        same_files(run_state['inputs'])
        assert all(run_state['inputs'][p] == state['inputs'][p] for p in owned)

    tests = read('new-tests.json')
    names = [t['name'] for t in tests]
    assert len(names) == len(set(names)) == 27 and all(n.startswith('v16_d_') for n in names)
    assert all(t['file'] in scope['sources'] for t in tests)
    final_log = checked_text('validation/tests-domain-infra.log')
    assert set(outcomes(final_log, names).values()) == {'ok'}
    assert set(outcomes(checked_text(args.candidate + '/core-tests.log'), names).values()) == {'ok'}
    assert '/repos/Antigravity/typst-crystalline/01_core' in final_log
    assert '/repos/Antigravity/typst-crystalline/03_infra' in final_log
    classification = read('baseline-test-classification.json')
    baseline_path = classification['log']
    assert sha(OUT / baseline_path) == classification['sha256']
    baseline = outcomes(checked_text(baseline_path), names)
    assert Counter(baseline.values()) == Counter({'ok': 24, 'FAILED': 3})
    assert {r['name'] for r in classification['tests']} == set(names)
    assert classification['counts'] == {'semantic RED': 3, 'preservation': 24}
    red_names = set()
    for row in classification['tests']:
        assert row['baseline_outcome'] == baseline[row['name']]
        assert row['classification'] == ('semantic RED' if baseline[row['name']] == 'FAILED' else 'preservation')
        if row['classification'] == 'semantic RED':
            assert row['file'] == '01_core/src/compiler/layout/measurement.rs'
            red_names.add(row['name'])
    assert read('baseline-final/freeze.json')['changed'] == []
    baseline_jobs = read('baseline-final/commands.json')
    assert len(baseline_jobs) == 1 and baseline_jobs[0]['exit'] == 101
    assert re.search(r'test result: FAILED\. 24 passed; 3 failed; 0 ignored;', checked_text(baseline_path))

    lint = read('lint-delta.json')
    assert set(lint['closed_ids']) == EXPECTED_IDS
    assert lint['remaining_v16'] == 44 and lint['new_errors_or_warnings'] == 0 and lint['outside_scope_exact']
    lint_state = read('lint-state.json.gz')
    same_files(lint_state['inputs'])
    assert all(lint_state['inputs'][p] == state['inputs'][p] for p in owned)
    before = read('baseline.sarif.gz')['runs'][0]['results']
    after = read('after.sarif.gz')['runs'][0]['results']
    location = lambda r: r['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./')
    finding = lambda r: json.dumps(r, sort_keys=True)
    cases = read('cases.json')
    assert {r['id'] for r in cases} == EXPECTED_IDS
    selected_sites = {(r['analysis']['file'], r['analysis']['line']) for r in cases}
    selected = [r for r in before if r['ruleId'] == 'V16' and
                (location(r), r['locations'][0]['physicalLocation']['region']['startLine']) in selected_sites]
    assert len(selected) == 20
    assert sum(r['ruleId'] == 'V16' for r in before) == 64
    assert sum(r['ruleId'] == 'V16' for r in after) == 44
    errors = lambda rows: Counter(finding(r) for r in rows if r['level'] in {'error', 'warning'})
    assert errors(before) - errors(after) == Counter(map(finding, selected))
    assert not errors(after) - errors(before)
    assert Counter(finding(r) for r in before if location(r) not in scope['sources']) == Counter(finding(r) for r in after if location(r) not in scope['sources'])

    experiments = read(args.structural + '/results.json')
    assert [r['name'] for r in experiments] == ['all-domains', 'new-old', 'old-new', 'new-new']
    structural_record(experiments[0], EXPECTED_IDS | SUPPLEMENT_IDS, 23)
    targets = read('structural-targets.json')
    assert {t['id'] for t in targets} == EXPECTED_IDS | SUPPLEMENT_IDS
    target_specs = {t['id']: t for t in targets}
    for target in experiments[0]['targets']:
        spec = target_specs[target['id']]
        assert target['file'] == spec['file'] and target['green'] == spec['green']
        source = (ROOT / target['file']).read_text()
        offset = source.index(spec['green']['anchor'], source.index(spec['green']['after']))
        assert source.count('\n', 0, offset) + 1 == target['line']
        assert len(source[:offset].encode()) == target['byte_start']
    for record in experiments[1:]:
        structural_record(record, {'V16-043'}, 1)
        assert record['isolated_axis'] == record['name']
    proof = read(args.structural + '/inputs.json.gz')
    same_files(proof['frozen_owners'], owned)
    assert all(proof['frozen_owners'][p] == state['inputs'][p] for p in owned)
    original = read(args.structural + '/original-sources.json.gz')
    assert all(hashlib.sha256(original[p].encode()).hexdigest() == state['inputs'][p] for p in scope['sources'])
    structural_baseline = read('structural-baseline/results.json')
    assert [r['name'] for r in structural_baseline] == ['all-domains', 'new-old', 'old-new', 'new-new']
    assert len(structural_baseline[0]['targets']) == 20
    assert {t['id'] for t in structural_baseline[0]['targets']} == EXPECTED_IDS
    assert all(r['exit'] == 101 and r['selected_e0004'] == 0 and not r['other_errors'] for r in structural_baseline)
    assert read('structural-baseline/freeze.json')['changed'] == []

    independent = read('structural-witness-review.json')['conclusion']
    assert independent['status'] == 'PASS'
    assert independent['green_primary_targets'] == 23 and independent['green_tuple_classes'] == 3
    assert independent['baseline_target_absence'] == 20 and independent['baseline_tuple_absence'] == 3
    assert not independent['suppression_or_production_changes_in_mutations']
    replay_review = read('agents/measurement/candidate-replay-review.json')
    assert replay_review['status'] == 'PASS_SCOPED_REPLAY_AUDIT' and not replay_review['errors']
    note_review = read('agents/grid/note-review.json')
    assert note_review['same_linter'] and not note_review['new_suppression_lines']
    obligations = read('open-obligations.json')
    assert len(obligations['obligations']) == 8
    for obligation in obligations['obligations']:
        assert obligation['status'].startswith('open_') and obligation['required_behavior']
        assert obligation['proposed_refuter_or_resolution_test'] and obligation['limits_of_current_evidence']
        for evidence in obligation['evidence']:
            artifact(evidence['path'], evidence['sha256'])

    validation_state_path = OUT / 'validation/validation-state.json.gz'
    grid = read(args.grid + '/receipt.json')
    grid_before = read('agents/grid/cli-baseline/receipt.json')
    assert grid['pass'] and grid['failures'] == []
    assert grid['source_unchanged'] and grid['product_unchanged'] and grid['oracle_unchanged']
    same_files(grid['source_sha256'])
    assert all(grid['source_sha256'][p] == state['inputs'][p] for p in grid['source_sha256'])
    assert len(grid['cases']) == 18 and set(grid['cases']) == set(grid_before['cases'])
    assert set(grid['preservation']) == set(grid['cases']) and all(grid['preservation'].values())
    for name, entry in grid['cases'].items():
        assert entry['source'] == grid_before['cases'][name]['source']
        assert entry['sha256'] == hashlib.sha256(entry['source'].encode()).hexdigest()
        assert set(entry['runs']) == {'product', 'oracle'}
        for who, run in entry['runs'].items():
            assert run['exit'] == 0 and not run.get('extraction_error')
            assert set(run['extraction']) == {'text', 'bbox'}
            assert all(x['exit'] == 0 for x in run['extraction'].values())
            assert 'observables' in run and run['observables']['pages']
        baseline_run = grid_before['cases'][name]['runs']['product']
        assert baseline_run['exit'] == 0 and entry['runs']['product']['observables'] == baseline_run['observables']
    for who, binary in grid['binaries'].items():
        artifact(binary['path'], binary['sha256'])
    assert grid['binaries']['oracle']['sha256'] == ORACLE_SHA
    assert grid['binaries']['oracle']['commit'] == ORACLE_COMMIT
    assert grid['binaries']['oracle']['tree'] == ORACLE_TREE
    build_binding(grid['binaries']['product']['build_receipt'], grid['binaries']['product']['receipt_sha256'], validation_state_path)
    artifact(OUT / 'agents/grid/replay.py', grid['runner_sha256'])

    measurement = read(args.measurement + '/receipt.json')
    base = read('agents/measurement/probes/png-before.json')
    expected_cases = {r['name'] for r in base['runs']} | {'effect-image', 'effect-shape', 'effect-equation', 'effect-figure'}
    assert len(expected_cases) == 27
    assert measurement['frozen'] and measurement['scope_before'] == measurement['scope_after']
    same_files(measurement['scope_before'])
    assert all(measurement['scope_before'][p] == state['inputs'][p] for p in measurement['scope_before'])
    assert measurement['oracle_commit'] == ORACLE_COMMIT and measurement['oracle_tree'] == ORACLE_TREE
    assert measurement['binaries']['oracle']['sha256'] == ORACLE_SHA
    assert measurement['build_receipt_hash_after'] == measurement['build_receipt']['sha256']
    build_binding(measurement['build_receipt']['path'], measurement['build_receipt']['sha256'], validation_state_path)
    artifact(measurement['runner']['path'], measurement['runner']['sha256'])
    artifact(OUT / args.measurement / 'asset.png', measurement['asset_sha256'])
    for who, binary in measurement['binaries'].items():
        assert binary['sha256'] == measurement['binary_hashes_after'][who]
        artifact(binary['path'], binary['sha256'])
        assert binary['sha256'] == grid['binaries'][who]['sha256']
    checks = {r['name']: r for r in measurement['checks']}
    assert len(measurement['checks']) == len(checks) == 27 and set(checks) == expected_cases
    assert len(measurement['runs']) == 54
    for name in expected_cases:
        pair = {r['binary']: r for r in measurement['runs'] if r['name'] == name}
        assert set(pair) == {'product', 'oracle'}
        assert pair['product']['source'] == pair['oracle']['source']
        for run in pair.values():
            assert run['exit'] == 0 and not run['timed_out'] and not run.get('geometry_error')
            assert run['source_sha256'] == hashlib.sha256(run['source'].encode()).hexdigest()
            artifact(OUT / args.measurement / (name + '.typ'), run['source_sha256'])
            artifact(run['output'], run['output_sha256'])
        check = checks[name]
        assert check['pass']
        color = '#2ecc40' if name.startswith('transform-') else '#ff0000'
        left, right = [pair[who]['colored_boxes'].get(color, []) for who in ['product', 'oracle']]
        if name.startswith('transform-'):
            assert len(left) == len(right) == (1 if name == 'transform-shape' else 0)
            assert all(abs(x-y) < 1e-6 for a,b in zip(left,right) for x,y in zip(a,b))
            assert check['product_green'] == left and check['oracle_green'] == right
        else:
            assert len(left) == len(right) == 1 and abs(left[0][0] - right[0][0]) < 1e-6
            assert check['product_marker'] == left and check['oracle_marker'] == right

    diff = subprocess.check_output(['git', 'diff', 'HEAD', '--', '01_core', '02_shell', '03_infra', '04_wiring'], text=True)
    added = '\n'.join(line[1:] for line in diff.splitlines() if line.startswith('+') and not line.startswith('+++'))
    attributes = re.findall(r'#!?\s*\[[^\]]*\]', added, re.S)
    assert not any(re.search(r'\b(?:allow|expect|ignore)\b', attr) for attr in attributes), 'added suppression or ignored test'

    # Only now can closure artifacts be emitted; every count above was witnessed.
    now = datetime.datetime.now(datetime.timezone.utc).isoformat()
    closed = []
    all_targets = {t['id']: t for t in experiments[0]['targets']}
    for identifier in scope['ids']:
        target = all_targets[identifier]
        source = target['file']
        body = (ROOT / source).read_text()
        owner = re.search(r'^//! @prompt (.+)$', body, re.M)[1]
        assert owner in scope['owners']
        closed.append({'id': identifier, 'source': source, 'owner': owner,
                       'source_sha256': sha(ROOT / source), 'owner_sha256': sha(ROOT / owner),
                       'prompt_seal': re.search(r'^//! @prompt-hash (\w+)$', body, re.M)[1],
                       'exact_structural_target': target,
                       'status': 'closed after observed baseline/final controls, enum evolution, exact lint delta, accepted replays and integrated gates; broader owner debt remains open'})
    assert {r['source'] for r in closed} == set(scope['sources'])
    assert {r['owner'] for r in closed} == set(scope['owners'])
    execution = {'utc': now, 'log': 'validation/tests-domain-infra.log',
                 'log_sha256': sha(OUT / 'validation/tests-domain-infra.log'),
                 'new_tests_observed_passing': tests, 'baseline_semantic_red': sorted(red_names),
                 'baseline_preservation_count': 24}
    ledger = {'utc': now, 'checkpoint': state['checkpoint'],
              'branch': subprocess.check_output(['git', 'branch', '--show-current'], text=True).strip(),
              'measurement_state': 'uncommitted working tree; exact inputs/diffstat/status in validation-state.json.gz',
              'closed': closed, 'source_owner_pairs': 15, 'remaining_v16': 44,
              'new_tests': len(tests), 'integrated_gates': len(validation), 'frozen_inputs': len(state['inputs']),
              'outer_jobs': len(outer), 'candidate_jobs': len(candidate),
              'structural_witnesses': len(experiments[0]['targets']), 'isolated_tuple_runs': len(experiments)-1,
              'final_bilateral_cases': {'grid': len(grid['cases']), 'measurement': len(checks)},
              'completed_at': outer[-1]['utc_end'], 'evidence_sha256': EVIDENCE,
              'worker_sha256': sha(Path(__file__)),
              'open_obligations': 'open-obligations.json',
              'closure_limit': 'Captioned/block/contextual measurement and other owner limits remain explicit debt; no full language parity or global V16-zero claim.'}
    for name, data in [('new-test-execution.json', execution), ('closure-ledger.json', ledger)]:
        (OUT / name).write_text(json.dumps(data, indent=2) + '\n')
    print(json.dumps({k: ledger[k] for k in ['new_tests', 'integrated_gates', 'frozen_inputs', 'outer_jobs', 'candidate_jobs', 'structural_witnesses', 'isolated_tuple_runs', 'remaining_v16', 'completed_at']}, indent=2))


if __name__ == '__main__':
    main()
