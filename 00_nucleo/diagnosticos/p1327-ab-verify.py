"""Independent full-transcript candidate verdict; no runtime or state-receipt reads."""
import collections
import datetime
import importlib.util
import json
import pathlib
import re

root = pathlib.Path('/repos/Antigravity/typst-crystalline')
prefix = root / '00_nucleo/diagnosticos'
spec = importlib.util.spec_from_file_location('runner', prefix / 'p1327-ab-runner.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
read = lambda name: json.loads((prefix / name).read_text())
oracle = read('p1327-ab-cli-oracle.json')
candidate = read('p1327-ab-cli-candidate.json')
freeze = read('p1327-ab-freeze.json')
manifest = read('p1327-manifest.json')
integrity_failures = []
checks = {}
def check(name, condition):
    checks[name] = bool(condition)
    if not condition:
        integrity_failures.append(name)
check('freeze_identity', runner.sha(prefix / 'p1327-ab-freeze.json') == 'f45fa705a4f0b1edb93c6f48d3c58adea159a983a296c161a3606000acf6c076')
check('r1_format_identity', runner.sha(prefix / 'p1327-ab-r1-format.json') == 'a69490c148f7e490fbb5e9dad131b5aecd11af221eaf8c5fba0b0ccfd79c115c')
check('hash_clarification_identity', runner.sha(prefix / 'p1327-ab-hash-clarification.md') == '2f62ef1508c07cc15ba855ebff1708f4773d9276ea585838720672c71047377f')
for path, expected in freeze['hashes'].items():
    if path.startswith('00_nucleo/prompts/'):
        import hashlib
        raw = (root / path).read_bytes()
        normative = re.sub(rb'^Hash do C\xc3\xb3digo: [0-9a-f]{8}\n', b'', raw, flags=re.MULTILINE)
        check('normative:' + path, hashlib.sha256(normative).hexdigest() == manifest['prompts'][path])
    else:
        check('frozen:' + path, runner.sha(root / path) == expected)
check('runner_receipt_identity', candidate['runner_sha256'] == oracle['runner_sha256'] == runner.sha(prefix / 'p1327-ab-runner.py'))
check('oracle_receipt_identity', candidate['oracle_sha256'] == runner.sha(prefix / 'p1327-ab-cli-oracle.json'))
binary = '/tmp/p1327-target.k9Mq0s/release/typst'
check('candidate_binary_identity', candidate['candidate_sha256'] == runner.sha(binary))
check('fixtures_receipt_matches_frozen', candidate['fixtures'] == oracle['fixtures'])
actual_fixtures = {p.name: {'sha256': runner.sha(p), 'text': p.read_text()}
                   for p in sorted(pathlib.Path(runner.CWD).glob('*.typ'))}
check('fixtures_on_disk_match_frozen', actual_fixtures == oracle['fixtures'])
check('case_catalog_unchanged', candidate['cases'] == oracle['cases'])
check('profiles_unchanged', candidate['profiles'] == oracle['profiles'])
expected = {(r['profile'], r['id']): r for r in oracle['expected']}
check('exact_112_expected_keys', len(expected) == len(oracle['expected']) == 112)
check('exact_336_rows', len(candidate['runs']) == 336)
maps = {}
violations = []
counts = collections.Counter()
for order in ['normal', 'repeat', 'reverse']:
    rows = [r for r in candidate['runs'] if r['order'] == order]
    keys = [(r['profile'], r['id']) for r in rows]
    check('complete_unique_keys:' + order, len(rows) == 112 and len(set(keys)) == 112 and set(keys) == set(expected))
    maps[order] = {}
    for row in rows:
        key = row['profile'], row['id']
        if key not in expected:
            continue
        want = expected[key]
        actual_obs, expected_obs = runner.observable(row), runner.observable(want)
        same = actual_obs == expected_obs
        verdict = 'Preserved' if same else 'Violated'
        counts[verdict] += 1
        check('stored_verdict_consistent:' + order + '/' + '/'.join(key), row['verdict'] == verdict)
        maps[order][key] = actual_obs
        check('argv_binary:' + order + '/' + '/'.join(key), row['argv'][0] == binary)
        check('argv_expression:' + order + '/' + '/'.join(key), row['argv'][1:] == want['argv'][1:])
        if not same:
            def blocks(stderr):
                return re.findall(r'^(?:error|warning):.*?(?=^(?:error|warning):|\Z)', stderr, flags=re.MULTILINE | re.DOTALL)
            actual_blocks, expected_blocks = blocks(row['stderr']), blocks(want['stderr'])
            reversed_only = (actual_obs[:2] == expected_obs[:2]
                             and len(actual_blocks) == len(expected_blocks) == 2
                             and ''.join(actual_blocks) == row['stderr']
                             and ''.join(expected_blocks) == want['stderr']
                             and actual_blocks == expected_blocks[::-1])
            violations.append({'profile': key[0], 'id': key[1], 'order': order,
                               'actual': actual_obs, 'expected': expected_obs,
                               'diagnosis': 'EXACT_TWO_DIAGNOSTIC_BLOCKS_REVERSED' if reversed_only else 'OTHER_FULL_TRANSCRIPT_DELTA'})
check('normal_repeat_stable', maps['normal'] == maps['repeat'])
check('normal_reverse_stable', maps['normal'] == maps['reverse'])
check('no_unrecognized_order', all(r['order'] in maps for r in candidate['runs']))
result = {
    'verified_at': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'regime': 'A/B executed without technical isolation attestation',
    'verdict': 'Violated' if violations or integrity_failures else 'Preserved',
    'counts': dict(counts),
    'integrity_failures': integrity_failures,
    'checks': checks,
    'candidate_sha256': candidate['candidate_sha256'],
    'candidate_receipt_sha256': runner.sha(prefix / 'p1327-ab-cli-candidate.json'),
    'oracle_sha256': runner.sha(prefix / 'p1327-ab-cli-oracle.json'),
    'freeze_sha256': runner.sha(prefix / 'p1327-ab-freeze.json'),
    'r1_format_sha256': runner.sha(prefix / 'p1327-ab-r1-format.json'),
    'manifest_sha256': runner.sha(prefix / 'p1327-manifest.json'),
    'verification_script_sha256': runner.sha(prefix / 'p1327-ab-verify.py'),
    'measurements_started': [min(r['started'] for r in candidate['runs']), max(r['started'] for r in candidate['runs'])],
    'violations': violations,
    'owner_implication': 'Observed diagnostic blocks are exact but CLI ordering fails frozen full-transcript contract. Runtime source not read: attribution to CLI presentation owner is an inference requiring owner inspection; modules-only sufficiency is unproven. No oracle amendment or product change authorized to this role.',
    'unknown_policy': 'Missing keys, changed fixtures, invalid stored verdict, unknown order or identity drift rejects acceptance; no required Unknown detected in this complete corpus.',
    'limitations': 'Finite frozen fragment; no general parity or technical isolation attestation. Original candidate failure receipt remains immutable.',
}
runner.save(prefix / 'p1327-ab-verdict.json', result)
print(json.dumps({'verdict': result['verdict'], 'counts': dict(counts), 'integrity_failures': integrity_failures,
                  'causes': dict(collections.Counter(v['diagnosis'] for v in violations)),
                  'verdict_sha256': runner.sha(prefix / 'p1327-ab-verdict.json'),
                  'candidate_receipt_sha256': result['candidate_receipt_sha256']}))
