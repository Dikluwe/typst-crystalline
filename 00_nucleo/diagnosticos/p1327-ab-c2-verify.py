"""Independent C2 final observable verdict; forbidden runtime/state data never opened."""
import collections
import datetime
import hashlib
import importlib.util
import json
import pathlib
import re

root = pathlib.Path('/repos/Antigravity/typst-crystalline')
prefix = root / '00_nucleo/diagnosticos'
spec = importlib.util.spec_from_file_location('original', prefix / 'p1327-ab-runner.py')
original = importlib.util.module_from_spec(spec)
spec.loader.exec_module(original)
load = lambda name: json.loads((prefix / name).read_text())
checks, failures, violations, summaries = {}, [], [], []
def check(name, condition):
    checks[name] = bool(condition)
    if not condition:
        failures.append(name)
pins = {
    'p1327-ab-freeze.json': 'f45fa705a4f0b1edb93c6f48d3c58adea159a983a296c161a3606000acf6c076',
    'p1327-ab-r1-format.json': 'a69490c148f7e490fbb5e9dad131b5aecd11af221eaf8c5fba0b0ccfd79c115c',
    'p1327-ab-hash-clarification.md': '2f62ef1508c07cc15ba855ebff1708f4773d9276ea585838720672c71047377f',
    'p1327-ab-cli-candidate.json': '6ffa5fc5ce6925eb0c85e40749adff5d1688ce006fe2400cdad3a4edeb47fafe',
    'p1327-ab-verdict.json': '976a2d160c8fc9095650f39b0f79ba33cc496656ba3ea14b08962ddccc1ed6e9',
    'p1327-ab-receipt.md': '7160f929b45a3b3547c4aa9735c2038b650c96aa3b22a7e9046ad11bad81de78',
    'p1327-ab-r2-oracle.json': '19d38cbf430e65a3ca67d2414ee8623f9fc7111e5e4077d4499e1d67e0d88984',
    'p1327-ab-r2-runner.py': '97529a351d83c19416538d91c5173ec5cead8efa39aeee82477a24fdffa019ed',
    'p1327-ab-r2-measure.json': '46abf67a60c7ebb04703cf70d6392b5927ac3e21bbab47753850aa6b1e34edf2',
    'p1327-r2-manifest.json': '9ebf13c2cf9a1c1daf186671194c7c4e15db16d8d4e47a12ad5ff4fb597f8ad0',
}
for name, expected in pins.items():
    check('immutable:' + name, original.sha(prefix / name) == expected)
freeze = load('p1327-ab-freeze.json')
manifest = load('p1327-r2-manifest.json')
for path, expected in freeze['hashes'].items():
    if not path.startswith('00_nucleo/prompts/'):
        check('frozen_r0:' + path, original.sha(root / path) == expected)
for path, expected in manifest['prompts'].items():
    raw = (root / path).read_bytes()
    normative, removed = re.subn(rb'^Hash do C\xc3\xb3digo: [0-9a-f]{8}\n', b'', raw, flags=re.MULTILINE)
    check('normative:' + path, removed == 1 and hashlib.sha256(normative).hexdigest() == expected)
binary = '/tmp/p1327-target.k9Mq0s/release/typst'
binary_sha = original.sha(binary)
check('current_c2_binary', binary_sha == '75e8b97b3788c0feaf457cb4c06b1c2c6735cff5ef3b9804a75ec57f8b148e31')
check('preserved_c1_binary', original.sha(manifest['c1_binary']['path']) == manifest['c1_binary']['sha256'])
check('vanilla_binary', original.sha('/usr/local/bin/typst') == '7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8')
check('original_baseline_binary', original.sha('/tmp/p1326-target.6Vi4Km/release/typst') == 'bd86b34602323a390a60a8f41b97b180f93077affcd30b6b1e85aa5926dee811')
original_oracle = load('p1327-ab-cli-oracle.json')
actual_fixtures = {p.name: {'sha256': original.sha(p), 'text': p.read_text()}
                   for p in sorted(pathlib.Path(original.CWD).glob('*.typ'))}
check('fixtures_current_exact', actual_fixtures == original_oracle['fixtures'])
for corpus, oracle_name, candidate_name, runner_name, expected_keys in [
    ('original', 'p1327-ab-cli-oracle.json', 'p1327-ab-cli-candidate-c2.json', 'p1327-ab-runner.py', 112),
    ('r2', 'p1327-ab-r2-oracle.json', 'p1327-ab-r2-candidate.json', 'p1327-ab-r2-runner.py', 32),
]:
    oracle, candidate = load(oracle_name), load(candidate_name)
    expected = {(r['profile'], r['id']): r for r in oracle['expected']}
    check(corpus + ':runner_identity', candidate['runner_sha256'] == oracle['runner_sha256'] == original.sha(prefix / runner_name))
    check(corpus + ':oracle_identity', candidate['oracle_sha256'] == original.sha(prefix / oracle_name))
    check(corpus + ':binary_identity', candidate['candidate_sha256'] == binary_sha)
    check(corpus + ':catalog_identity', candidate['cases'] == oracle['cases'] and candidate['profiles'] == oracle['profiles'])
    check(corpus + ':expected_complete_unique', len(expected) == len(oracle['expected']) == expected_keys)
    check(corpus + ':rows_complete', len(candidate['runs']) == expected_keys * 3)
    if corpus == 'original':
        check(corpus + ':fixture_receipt', candidate['fixtures'] == oracle['fixtures'])
    maps, counts, classes = {}, collections.Counter(), collections.Counter()
    for order in ['normal', 'repeat', 'reverse']:
        rows = [r for r in candidate['runs'] if r['order'] == order]
        keys = [(r['profile'], r['id']) for r in rows]
        check(corpus + ':keys:' + order, len(rows) == len(set(keys)) == expected_keys and set(keys) == set(expected))
        maps[order] = {}
        for row in rows:
            key = row['profile'], row['id']
            if key not in expected:
                continue
            want = expected[key]
            check(corpus + ':command:' + order + '/' + '/'.join(key), row['argv'][0] == binary and row['argv'][1:] == want['argv'][1:] and row['cwd'] == want['cwd'])
            actual_obs, expected_obs = original.observable(row), original.observable(want)
            verdict = 'Preserved' if actual_obs == expected_obs else 'Violated'
            counts[verdict] += 1
            classes[row['class']] += 1
            check(corpus + ':stored_verdict:' + order + '/' + '/'.join(key), row['verdict'] == verdict)
            maps[order][key] = actual_obs
            if verdict != 'Preserved':
                violations.append({'corpus': corpus, 'profile': key[0], 'id': key[1], 'order': order, 'actual': actual_obs, 'expected': expected_obs})
    check(corpus + ':normal_repeat_stable', maps['normal'] == maps['repeat'])
    check(corpus + ':normal_reverse_stable', maps['normal'] == maps['reverse'])
    check(corpus + ':orders_valid', all(row['order'] in maps for row in candidate['runs']))
    summaries.append({'corpus': corpus, 'counts': dict(counts), 'classes': dict(classes),
                      'keys_per_order': expected_keys,
                      'candidate_receipt': candidate_name, 'candidate_receipt_sha256': original.sha(prefix / candidate_name),
                      'oracle': oracle_name, 'oracle_sha256': original.sha(prefix / oracle_name),
                      'measurement_interval': [min(r['started'] for r in candidate['runs']), max(r['started'] for r in candidate['runs'])]})
result = {
    'verified_at': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'verdict': 'Violated' if failures or violations else 'PASS_SCOPED',
    'regime': 'A/B without technical isolation attestation',
    'candidate_binary': binary, 'candidate_sha256': binary_sha,
    'corpora': summaries, 'integrity_failures': failures, 'violations': violations, 'checks': checks,
    'immutable_pins': pins,
    'verification_script_sha256': original.sha(prefix / 'p1327-ab-c2-verify.py'),
    'original_failed_verdict_preserved': 'p1327-ab-verdict.json remains Violated for C1; C2 is a distinct binary and additive verification',
    'debt_boundary': 'Original redundant-rename/type-error and R2 raw serialization with/without preceding warning compare full frozen crystalline transcripts, not vanilla parity. No general stripping or normalization.',
    'unknown_policy': 'No mandatory Unknown or missing observation; any such condition or identity mismatch rejects acceptance',
    'limitations': 'Finite CLI observable fragment only; no runtime source or state receipt inspected, no general parity claim, no technical isolation attestation',
}
original.save(prefix / 'p1327-ab-c2-verdict.json', result)
print(json.dumps({'verdict': result['verdict'], 'integrity_failures': failures,
                  'counts': [s['counts'] for s in summaries],
                  'classes': [s['classes'] for s in summaries],
                  'verdict_sha256': original.sha(prefix / 'p1327-ab-c2-verdict.json')}, indent=2))
