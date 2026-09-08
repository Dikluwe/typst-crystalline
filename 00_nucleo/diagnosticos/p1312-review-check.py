"""Independent evidence recount. Writes only the reviewer evidence receipt."""
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
PROFILES = ('default', 'html', 'a11y', 'html+a11y')

def sha(p):
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()

def digest(b):
    return hashlib.sha256(b).hexdigest()

def read(name):
    p = D / name
    inputs[str(p.relative_to(ROOT))] = sha(p)
    return json.loads(p.read_text())

def obs(r):
    return {k: r[k] for k in ('exit', 'stdout', 'stderr')}

def keyed(rows, fields):
    result = {tuple(r[k] for k in fields): r for r in rows}
    assert len(result) == len(rows), 'duplicate keys'
    return result

def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, text=True)

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--workspace', default='p1312-workspace-tests.json')
    args = parser.parse_args()
    baseline = read('p1312-baseline.json')
    frozen = read('p1312-ab-freeze.json')
    assert inputs['00_nucleo/diagnosticos/p1312-ab-freeze.json'] == '440d7e744067dffab23371700bf421bf6dd2830c7d8a60cf26ccc3b6ea54f725'
    for p, h in frozen['inputs'].items():
        assert sha(ROOT / p) == h, p
    l0 = ROOT / frozen['l0']['path']
    normative, n = re.subn(rb'(?m)^Hash do C\xc3\xb3digo: [a-f0-9]+\r?\n', b'', l0.read_bytes())
    assert n == 1 and digest(normative) == frozen['l0']['normative_sha256']
    cases = read('p1312-ab-cases.json')['cases']
    case_map = keyed(cases, ('id',))
    pre = read('p1312-ab-baseline-sealed.json')
    before = keyed(pre['runs'], ('id', 'profile', 'product'))
    expected = keyed(frozen['expected'], ('id', 'profile'))
    assert len(before) == len(cases) * 8 and len(expected) == len(cases) * 4
    assert expected.keys() == {(c['id'], p) for c in cases for p in PROFILES}
    for key, e in expected.items():
        c = case_map[(key[0],)]
        product = 'baseline' if c['kind'] == 'control' else 'vanilla'
        row = before[(*key, product)]
        assert row['argv'][2] == c['expr']
        value = obs(row)
        if c['kind'] == 'symbol':
            assert value['stderr'].startswith('error: file not found (searched at /tmp/p1312-ab-fixtures/α)\n')
            value['stderr'] = 'error: expected path or string, found symbol\n' + value['stderr'].split('\n', 1)[1]
        assert e['expected'] == value, key
        if c['kind'] == 'target':
            assert value['exit'] == 1 and value['stdout'] == ''
            assert value['stderr'].startswith('error: expected path or string, found ')
            assert obs(before[(*key, 'baseline')]) != value
    runs = read('p1312-ab-candidate-runs.json')
    actual = keyed(runs['runs'], ('id', 'profile', 'order'))
    orders = ('normal', 'repeat', 'reverse')
    assert actual.keys() == {(*k, o) for k in expected for o in orders}
    for key, row in actual.items():
        assert row['product'] == 'candidate'
        assert row['argv'][2] == case_map[(key[0],)]['expr']
        assert obs(row) == expected[key[:2]]['expected'], key
        assert obs(row) == obs(actual[(*key[:2], 'normal')])
    build = read('p1312-build.json')
    candidate = build['candidate']
    assert runs['binaries']['candidate'] == candidate
    for identity in (candidate, baseline['baseline_binary'], baseline['vanilla']):
        assert sha(identity['path']) == identity['sha256']
    assert all(r['utc'] > frozen['utc'] for r in runs['runs'])
    red = read('p1312-unit-red.json')
    green = read('p1312-unit-green.json')
    assert red['exit'] == 101 and '1 passed; 3 failed;' in red['stdout']
    assert red['stderr'].count('assertion `left == right` failed') == 3
    assert green['exit'] == 0 and '4 passed; 0 failed;' in green['stdout']
    assert frozen['utc'] < green['before']['at']
    for name in ('build', 'workspace-tests', 'lint', 'fmt', 'diff-check', 'lineage-preflight'):
        r = read(args.workspace if name == 'workspace-tests' else 'p1312-' + name + '.json')
        assert r['exit'] == 0
        assert r['baseline_sha256'] == inputs['00_nucleo/diagnosticos/p1312-baseline.json']
        assert r['before']['head'] == r['after']['head'] == baseline['state']['head']
    workspace = read(args.workspace)
    workspace_incident = None
    if args.workspace != 'p1312-workspace-tests.json':
        original = read('p1312-workspace-tests.json')
        focal = read('p1312-workspace-focal.json')
        assert original['exit'] == 101
        assert '2 failed;' in original['stdout'] and 'unclosed delimiter' in original['stdout']
        assert focal['exit'] == 0 and '3 passed; 0 failed;' in focal['stdout']
        assert original['before']['diff'] == original['after']['diff'] == focal['before']['diff'] == focal['after']['diff'] == workspace['before']['diff'] == workspace['after']['diff']
        assert original['argv'] == workspace['argv']
        assert original['candidate'] == focal['candidate'] == workspace['candidate'] == candidate
        workspace_incident = {'original_exit': 101, 'failed': 2, 'focal_passed': 3, 'successor': args.workspace, 'same_product_diff_and_candidate': True, 'cause': 'Transient non-reproduced unclosed-delimiter failures in PDF integration tests. Race is a hypothesis, not a proven cause.'}
    counts = [tuple(map(int, m)) for m in re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;', workspace['stdout'])]
    totals = [sum(r[i] for r in counts) for i in range(3)]
    assert totals[0] > 0 and totals[1] == 0
    lint = read('p1312-lint.json')
    lint_counts = Counter(re.findall(r'(?m)^(error|warning|info):', lint['stdout'] + '\n' + lint['stderr']))
    assert lint_counts['error'] == 0
    replay_counts = {}
    for version, case_id in (('p1310', 'read.wrong'), ('p1311', 'read-loader-error')):
        replay = read('p1312-' + version + '-replay.json')
        rows = keyed(replay['rows'], ('case', 'profile'))
        if version == 'p1310':
            f = read('p1310-ab-frozen-r2.json')
            old = {(c['id'], p): v for c in f['cases'] for p, v in c['expected'].items()}
            vanilla = {(r['case'], r['profile']): r['observable'] for r in read('p1310-ab-freeze-measurement.json')['rows'] if r['side'] == 'vanilla'}
        else:
            f = read('p1311-ab-freeze.json')
            old = {(e['id'], e['profile']): e['expected'] for e in f['expected']}
            vanilla = {(r['id'], r['profile']): obs(r) for r in read('p1311-ab-baseline-final.json')['runs'] if r['product'] == 'vanilla'}
        assert rows.keys() == old.keys()
        changed = {k for k in rows if rows[k]['observable'] != old[k]}
        assert changed == {(case_id, p) for p in PROFILES}
        for key, row in rows.items():
            assert row['observable'].get('kind') != 'Unknown'
            assert row['observable'] == (vanilla[key] if key in changed else old[key])
            for channel in ('stdout', 'stderr'):
                if channel + '_base64' in row:
                    raw = base64.b64decode(row[channel + '_base64'])
                    assert digest(raw) == row[channel + '_sha256']
                    assert raw.decode() == row['observable'][channel]
        assert replay['candidate'] == candidate
        replay_counts[version] = {'preserved': len(rows)-len(changed), 'new_deltas_equal_vanilla': len(changed), 'unknown': 0}
    histories = [json.loads(read(n)['stdout']) for n in ('p1312-p1308-replay.json', 'p1311-p1308-replay.json', 'p1309-sentinels-p1308-vanilla.json')]
    current, previous, vanilla = [keyed(h['rows'], ('case', 'profile')) for h in histories]
    assert current.keys() == previous.keys() == vanilla.keys()
    changed = {k for k in current if current[k]['observable'] != previous[k]['observable']}
    assert changed == {('p1307.decoder.read.wrong-type', p) for p in PROFILES}
    old = {k for k in previous if previous[k]['verdict'] == 'Violated'}
    assert {k for k in current if current[k]['verdict'] == 'Violated'} == old | changed
    for k in old | changed:
        assert current[k]['observable'] == vanilla[k]['observable']
    assert all(r['observable'].get('kind') != 'Unknown' for r in current.values())
    assert len({h['oracle_sha256'] for h in histories}) == 1
    assert histories[0]['binary_sha256'] == candidate['sha256']
    replay_counts['p1308'] = {'preserved_against_p1311': len(current)-len(changed), 'new_deltas_equal_vanilla': len(changed), 'preceding_deltas_equal_vanilla': len(old), 'unknown': 0}
    changed_files = []
    for p, h in baseline['files'].items():
        assert not p.startswith(('00_nucleo/context/', '00_nucleo/materialization/'))
        if sha(ROOT / p) != h:
            changed_files.append(p)
    assert set(changed_files) == {'01_core/src/compiler/stdlib/loading.rs', '00_nucleo/prompts/compiler/stdlib/loading.md'}
    for p, h in baseline['prior_artifacts'].items():
        assert sha(ROOT / p) == h, p
    source = ROOT / '01_core/src/compiler/stdlib/loading.rs'
    code = source.read_bytes()
    code_hash = digest(b''.join(line for line in code.splitlines(keepends=True) if not line.startswith(b'//! @prompt-hash ')))[:8]
    assert re.search(r'(?m)^Hash do Código: ([a-f0-9]{8})$', l0.read_text()).group(1) == code_hash
    restored = source.read_text()
    restored, n = re.subn(r'    #\[test\]\n    fn p1312_read_path_cast_public_type_names\(\)[\s\S]*?(?=    #\[test\]\n    fn p1310_data_source_cast_public_type_names)', '', restored)
    assert n == 1
    restored, n = re.subn(r'/// Read accepts PathOrStr; CSV retains its separately scoped legacy cast\.\nfn arg_read_path\([\s\S]*?(?=fn reject_named)', '', restored)
    assert n == 1
    assert restored.count('let path_value = arg_read_path(args)?;') == 1
    restored = restored.replace('let path_value = arg_read_path(args)?;', 'let path_value = arg_path(args, "read")?;')
    restored = re.sub(r'(?m)^//! @prompt-hash [a-f0-9]+', '//! @prompt-hash ec1dd115', restored)
    assert digest(restored.encode()) == baseline['files'][str(source.relative_to(ROOT))], 'unreviewed source delta'
    assert not git('diff', '--cached', '--name-only')
    assert git('rev-parse', 'HEAD').strip() == baseline['state']['head']
    result = {'schema': 'p1312-review-evidence-v1', 'issuer': '/root/p1312_review', 'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'head': baseline['state']['head'], 'diff_stat': git('diff', 'HEAD', '--stat'), 'diff': git('diff', 'HEAD', '--binary'), 'status': git('status', '--short'), 'inputs': inputs, 'checker_sha256': sha(__file__), 'candidate': candidate, 'source_sha256': sha(source), 'l0_sha256': sha(l0), 'l0_normative_sha256': frozen['l0']['normative_sha256'], 'code_hash': code_hash, 'ab': {'cases': len(cases), 'kinds': dict(Counter(c['kind'] for c in cases)), 'per_order': len(expected), 'orders': 3, 'comparisons': len(actual), 'unknown': 0, 'unstable': 0}, 'replays': replay_counts, 'workspace': totals, 'lint': {k: lint_counts[k] for k in ('error', 'warning', 'info')}, 'baseline_files_checked': len(baseline['files']), 'changed_files': changed_files, 'prior_artifacts_preserved': len(baseline['prior_artifacts']), 'all_assertions_passed': True}
    result['workspace_receipt'] = args.workspace
    result['workspace_incident'] = workspace_incident
    out = D / 'p1312-review-evidence.json'
    assert not out.exists()
    raw = json.dumps(result, ensure_ascii=False, indent=2) + '\n'
    patch = '*** Begin Patch\n*** Add File: ' + str(out) + '\n' + ''.join('+' + line + '\n' for line in raw.splitlines()) + '*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, text=True, check=True, capture_output=True)
    print(json.dumps({k: result[k] for k in ('utc', 'ab', 'replays', 'workspace', 'lint', 'prior_artifacts_preserved')}))

if __name__ == '__main__':
    main()
