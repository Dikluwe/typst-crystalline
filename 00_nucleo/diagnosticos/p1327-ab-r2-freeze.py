"""Freeze measured R2 observations and literal success-path preservation before C2."""
import collections
import datetime
import hashlib
import importlib.util
import json
import pathlib
import re

root = pathlib.Path('/repos/Antigravity/typst-crystalline')
prefix = root / '00_nucleo/diagnosticos'
spec = importlib.util.spec_from_file_location('r2', prefix / 'p1327-ab-r2-runner.py')
r2 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r2)
original = r2.original
measurement = json.loads((prefix / 'p1327-ab-r2-measure.json').read_text())
assert len(measurement['c1']) == len(measurement['vanilla']) == 32
expected, classifications = [], []
for c1, vanilla in zip(measurement['c1'], measurement['vanilla']):
    assert (c1['id'], c1['profile']) == (vanilla['id'], vanilla['profile'])
    equal = original.observable(c1) == original.observable(vanilla)
    if c1['class'] == 'parity':
        assert equal, c1['id']
        want = vanilla
        reason = 'C1 and pinned vanilla full observations identical'
    elif c1['class'] == 'order-correction':
        assert not equal and c1['exit'] == vanilla['exit'] == 1
        assert c1['stdout'] == vanilla['stdout'] == ''
        blocks = re.findall(r'^(?:error|warning):.*?(?=^(?:error|warning):|\Z)', c1['stderr'], flags=re.MULTILINE | re.DOTALL)
        assert ''.join(blocks) == c1['stderr']
        assert blocks[-1].startswith('error:') and all(b.startswith('warning:') for b in blocks[:-1])
        assert blocks[-1] + ''.join(blocks[:-1]) == vanilla['stderr']
        want = vanilla
        reason = 'Only entire error block moves ahead of unchanged ordered warning blocks; expected is full literal vanilla transcript'
    elif c1['class'] == 'preserve-success-path':
        assert not equal and c1['exit'] == vanilla['exit'] == 1
        assert c1['stdout'] == vanilla['stdout'] == ''
        want = c1
        reason = 'Evaluation succeeds; raw serialization then rejects integer. Preserve full C1 transcript per wiring L0 Ok-path clause. Baseline int vs vanilla integer and baseline retained warning vs vanilla omitted warning are explicit preexisting debt.'
    else:
        raise AssertionError(c1['class'])
    expected.append(want)
    classifications.append({'id': c1['id'], 'profile': c1['profile'], 'class': c1['class'],
                            'baseline_equals_vanilla': equal, 'reason': reason})
manifest_path = prefix / 'p1327-r2-manifest.json'
assert original.sha(manifest_path) == '9ebf13c2cf9a1c1daf186671194c7c4e15db16d8d4e47a12ad5ff4fb597f8ad0'
wiring_path = root / '00_nucleo/prompts/wiring.md'
normative = re.sub(rb'^Hash do C\xc3\xb3digo: [0-9a-f]{8}\n', b'', wiring_path.read_bytes(), flags=re.MULTILINE)
oracle = {
    'frozen_at': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'pre_C2': True,
    'regime': 'A/B without technical isolation attestation',
    'runner_sha256': original.sha(prefix / 'p1327-ab-r2-runner.py'),
    'original_runner_sha256': original.sha(prefix / 'p1327-ab-runner.py'),
    'measurement_sha256': original.sha(prefix / 'p1327-ab-r2-measure.json'),
    'r2_manifest_sha256': original.sha(manifest_path),
    'wiring_raw_sha256': original.sha(wiring_path),
    'wiring_normative_sha256': hashlib.sha256(normative).hexdigest(),
    'binaries': measurement['binaries'],
    'cases': measurement['cases'], 'profiles': measurement['profiles'],
    'expected': expected, 'classifications': classifications,
    'counts_per_profile': dict(collections.Counter(c[3] for c in r2.CASES)),
    'observations_measured': 64,
    'measurement_interval': [min(r['started'] for r in measurement['c1'] + measurement['vanilla']),
                             max(r['started'] for r in measurement['c1'] + measurement['vanilla'])],
    'unknown_policy': 'Any invalid/missing fixture or observation blocks acceptance; no Unknown found',
    'comparison': 'Full exit/stdout/stderr exact equality; no normalization, generic stripping or candidate-derived expectations',
    'fixture_note': 'All eight cases are self-contained expressions; original cwd and fixture directory reused without edits',
    'budget': 'One measurement pass; no semantic revisions; eight bounded cases, four profiles, two binaries',
    'original_artifacts_policy': 'All previous P1327 R0/R1/original candidate failure artifacts remain immutable',
}
original.save(prefix / 'p1327-ab-r2-oracle.json', oracle)
print(json.dumps({'oracle_sha256': original.sha(prefix / 'p1327-ab-r2-oracle.json'),
                  'runner_sha256': oracle['runner_sha256'],
                  'measurement_sha256': oracle['measurement_sha256'],
                  'wiring_normative_sha256': oracle['wiring_normative_sha256'],
                  'counts_per_profile': oracle['counts_per_profile']}, indent=2))
