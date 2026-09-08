"""Read-only independent P1311 evidence audit; writes only reviewer receipt."""
import argparse
import base64
from collections import Counter
import datetime
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / '00_nucleo/diagnosticos'
inputs = {}


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def read(name):
    path = D / name
    inputs[str(path.relative_to(ROOT))] = sha(path)
    return json.loads(path.read_text())


def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, text=True)


def obs(row):
    return {k: row[k] for k in ('exit', 'stdout', 'stderr')}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--ab-runs', required=True)
    args = parser.parse_args()
    baseline = read('p1311-baseline.json')
    frozen = read('p1311-ab-freeze.json')
    assert inputs['00_nucleo/diagnosticos/p1311-ab-freeze.json'] == '37db1eee5ae27d597d11df9534574008b46a31fae56bbdd77b7135c772663b3a'
    for path, digest in frozen['inputs'].items():
        assert sha(ROOT / path) == digest, path
    l0 = ROOT / frozen['l0']['path']
    normalized, count = re.subn(rb'(?m)^Hash do C\xc3\xb3digo: [0-9a-f]+\r?\n', b'', l0.read_bytes())
    assert count == 1 and hashlib.sha256(normalized).hexdigest() == frozen['l0']['normative_sha256']
    source_path = ROOT / '01_core/src/compiler/eval/bindings/field_access.rs'
    source = source_path.read_bytes()
    code_hash = hashlib.sha256(b''.join(line for line in source.splitlines(keepends=True)
                                      if not line.startswith(b'//! @prompt-hash '))).hexdigest()[:8]
    assert re.search(r'(?m)^Hash do Código: ([0-9a-f]{8})$', l0.read_text()).group(1) == code_hash
    cases = read('p1311-ab-cases.json')['cases']
    measured = read('p1311-ab-baseline-final.json')
    bycase = {c['id']: c for c in cases}
    measured_rows = {(r['id'], r['profile'], r['product']): r for r in measured['runs']}
    assert len(measured_rows) == len(measured['runs']) == len(cases) * 8
    expected = {(e['id'], e['profile']): e for e in frozen['expected']}
    assert len(expected) == len(frozen['expected']) == len(cases) * 4
    for key, e in expected.items():
        oracle = 'vanilla' if bycase[key[0]]['kind'] == 'target' else 'baseline'
        assert e['oracle_product'] == oracle
        assert e['expected'] == obs(measured_rows[(*key, oracle)])
    for row in measured['runs']:
        assert row['argv'][2] == bycase[row['id']]['expr']
    candidate = read(args.ab_runs)
    actual = {(r['id'], r['profile'], r['order']): r for r in candidate['runs']}
    assert len(actual) == len(candidate['runs']) == len(expected) * 3
    assert actual.keys() == {(*k, order) for k in expected for order in ('normal', 'repeat', 'reverse')}
    for key, row in actual.items():
        assert row['product'] == 'candidate'
        assert row['argv'][2] == bycase[key[0]]['expr']
        assert obs(row) == expected[key[:2]]['expected'], key
        assert obs(row) == obs(actual[(*key[:2], 'normal')])
    build = read('p1311-build.json')
    candidate_identity = build['candidate']
    assert sha(candidate_identity['path']) == candidate_identity['sha256']
    assert candidate['binaries']['candidate'] == candidate_identity
    assert sha(baseline['baseline_binary']['path']) == baseline['baseline_binary']['sha256']
    assert sha(baseline['vanilla']['path']) == baseline['vanilla']['sha256']
    red = read('p1311-unit-red.json')
    green = read('p1311-unit-green.json')
    assert red['exit'] == 101 and '2 passed; 3 failed;' in red['stdout']
    assert 'assertion `left == right` failed' in red['stderr']
    assert green['exit'] == 0 and '5 passed; 0 failed;' in green['stdout']
    assert frozen['utc'] < green['before']['at']
    for name in ('build', 'workspace-tests', 'lint', 'fmt', 'diff-check', 'lineage-preflight'):
        receipt = read('p1311-' + name + '.json')
        assert receipt['exit'] == 0
        assert receipt['baseline_sha256'] == inputs['00_nucleo/diagnosticos/p1311-baseline.json']
        assert receipt['before']['head'] == receipt['after']['head'] == baseline['state']['head']
    workspace = read('p1311-workspace-tests.json')
    rows = [tuple(map(int, m)) for m in re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;', workspace['stdout'])]
    totals = [sum(r[i] for r in rows) for i in range(3)]
    assert totals[1] == 0 and totals[0] > 0
    lint = read('p1311-lint.json')
    lint_counts = {k: len(re.findall('(?m)^' + k + ':', lint['stdout'])) for k in ('error', 'warning', 'info')}
    assert lint_counts == {'error': 0, 'warning': 240, 'info': 1136}
    old = read('p1310-ab-frozen-r2.json')
    old_expected = {(c['id'], p): v for c in old['cases'] for p, v in c['expected'].items()}
    replay = read('p1311-p1310-replay.json')
    replay_rows = {(r['case'], r['profile']): r for r in replay['rows']}
    assert replay_rows.keys() == old_expected.keys() and len(replay_rows) == len(replay['rows'])
    for key, row in replay_rows.items():
        assert row['observable'] == old_expected[key] and row['observable']['kind'] != 'Unknown'
        assert row['verdict'] == 'Preserved'
        for channel in ('stdout', 'stderr'):
            raw = base64.b64decode(row[channel + '_base64'])
            assert hashlib.sha256(raw).hexdigest() == row[channel + '_sha256']
            assert raw.decode() == row['observable'][channel]
    p1308_current = json.loads(read('p1311-p1308-replay.json')['stdout'])
    p1308_previous = json.loads(read('p1310-p1308-replay.json')['stdout'])
    p1308_vanilla = json.loads(read('p1309-sentinels-p1308-vanilla.json')['stdout'])
    histories = []
    for history in (p1308_current, p1308_previous, p1308_vanilla):
        mapped = {(r['case'], r['profile']): r for r in history['rows']}
        assert len(mapped) == len(history['rows'])
        histories.append(mapped)
    current_rows, previous_rows, vanilla_rows = histories
    assert current_rows.keys() == previous_rows.keys() == vanilla_rows.keys()
    new_changes = {k for k in current_rows if current_rows[k]['observable'] != previous_rows[k]['observable']}
    required_changes = {(f'p1307.negative.{n}.encode', p) for n in ('csv', 'read', 'xml')
                        for p in ('default', 'html', 'a11y', 'html+a11y')}
    assert new_changes == required_changes
    old_changes = {(r['case'], r['profile']) for r in read('p1310-p1308-delta.json')['expected_changes']}
    for key in new_changes | old_changes:
        assert current_rows[key]['observable'] == vanilla_rows[key]['observable']
    assert {k for k, r in current_rows.items() if r['verdict'] == 'Violated'} == new_changes | old_changes
    assert all(r['observable']['kind'] != 'Unknown' for r in current_rows.values())
    assert p1308_current['binary_sha256'] == candidate_identity['sha256']
    assert p1308_current['oracle_sha256'] == p1308_previous['oracle_sha256'] == p1308_vanilla['oracle_sha256']
    for path, digest in baseline['prior_artifacts'].items():
        assert sha(ROOT / path) == digest, path
    for path in ('01_core/src/compiler/stdlib/loading.rs', '00_nucleo/prompts/compiler/stdlib/loading.md'):
        assert sha(ROOT / path) == baseline['files'][path], path
    assert not git('diff', '--cached', '--name-only')
    assert git('rev-parse', 'HEAD').strip() == baseline['state']['head']
    assert set(git('diff', 'HEAD', '--name-only').splitlines()) == {
        '01_core/src/compiler/stdlib/loading.rs', '00_nucleo/prompts/compiler/stdlib/loading.md',
        '01_core/src/compiler/eval/bindings/field_access.rs', '00_nucleo/prompts/compiler/eval/bindings/field_access.md'}
    result = {'schema': 'p1311-review-evidence-v1', 'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
        'issuer': '/root/p1311_review', 'head': baseline['state']['head'],
        'diff_stat': git('diff', 'HEAD', '--stat'), 'diff': git('diff', 'HEAD', '--binary'),
        'status': git('status', '--short'), 'inputs': inputs, 'checker_sha256': sha(__file__),
        'candidate': candidate_identity, 'source_sha256': sha(ROOT / '01_core/src/compiler/eval/bindings/field_access.rs'),
        'l0_sha256': sha(l0), 'l0_normative_sha256': frozen['l0']['normative_sha256'], 'code_hash': code_hash,
        'ab': {'cases': len(cases), 'policies': dict(Counter(c['kind'] for c in cases)),
               'per_order': len(expected), 'orders': 3, 'unknown': 0, 'unstable': 0},
        'p1310_replay': {'preserved': len(replay_rows), 'unknown': 0},
        'p1308_replay': {'preserved_against_p1310': len(current_rows) - len(new_changes),
                        'new_deltas_equal_vanilla': len(new_changes),
                        'preceding_deltas_equal_vanilla': len(old_changes), 'unknown': 0},
        'prior_artifacts_preserved': len(baseline['prior_artifacts']), 'workspace': totals,
        'lint': lint_counts, 'all_assertions_passed': True}
    path = D / 'p1311-review-evidence.json'
    assert not path.exists()
    payload = json.dumps(result, ensure_ascii=False, indent=2) + '\n'
    patch = '*** Begin Patch\n*** Add File: ' + str(path) + '\n' + ''.join('+' + line + '\n' for line in payload.splitlines()) + '*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, cwd=ROOT, text=True, check=True, capture_output=True)
    print(json.dumps({'path': str(path), 'sha256': sha(path), 'ab': result['ab'], 'workspace': totals}))


if __name__ == '__main__':
    main()
