"""Pre-C formatting, classification and hash receipt; no product/runtime reads."""
import collections
import datetime
import hashlib
import importlib.util
import json
import pathlib
import subprocess

root = pathlib.Path('/repos/Antigravity/typst-crystalline')
prefix = root / '00_nucleo/diagnosticos'
spec = importlib.util.spec_from_file_location('runner', prefix / 'p1327-ab-runner.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)
snippet = prefix / 'p1327-ab-tests.rs'
old = snippet.read_text()
wrapped = 'mod p1327_format_holder {\n' + old + '}\n'
formatted = subprocess.run(['rustfmt', '--emit', 'stdout', '--edition', '2021'], input=wrapped,
                           text=True, capture_output=True, check=True).stdout
new = '\n'.join(formatted.splitlines()[1:-1]) + '\n'
if new != old:
    patch = '*** Begin Patch\n*** Update File: 00_nucleo/diagnosticos/p1327-ab-tests.rs\n@@\n'
    patch += ''.join('-' + line + '\n' for line in old.splitlines())
    patch += ''.join('+' + line + '\n' for line in new.splitlines())
    patch += '*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, text=True, cwd=root, check=True)
oracle = json.loads((prefix / 'p1327-ab-cli-oracle.json').read_text())
assert len(oracle['baseline']) == len(oracle['vanilla']) == 112
for b, v in zip(oracle['baseline'], oracle['vanilla']):
    assert (b['id'], b['profile']) == (v['id'], v['profile'])
    if b['class'] == 'parity':
        assert runner.observable(b) == runner.observable(v), b['id']
    else:
        assert runner.observable(b) != runner.observable(v), b['id']
        assert (b['exit'], b['stdout']) == (v['exit'], v['stdout']), b['id']
    if b['class'] == 'correction':
        assert 'warning: this import has no effect\n' in v['stderr']
        assert 'warning:' not in b['stderr']
        assert v['stderr'].startswith(b['stderr'])
files = [
    root / '00_nucleo/prompts/compiler/eval/modules.md',
    root / '00_nucleo/prompts/compiler/eval/tests.md',
    prefix / 'p1327-manifest.json',
    snippet,
    prefix / 'p1327-ab-legacy-replacements.json',
    prefix / 'p1327-ab-runner.py',
    prefix / 'p1327-ab-cli-oracle.json',
    prefix / 'p1327-ab-author-replacements.py',
    prefix / 'p1327-ab-freeze.py',
]
receipt = {
    'frozen_at': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'regime': 'A/B executed without technical isolation attestation',
    'role': '/root/p1327_tests: independent oracle author; fresh conversational context',
    'read_allowlist': ['two full L0 owners', 'test-only eval/tests.rs', 'public Source APIs',
                       'skill and both references', 'ADR name search (no segregation ADR found)',
                       'safe p1327-manifest.json', 'pinned baseline and vanilla binaries',
                       'own p1327-ab artifacts and fixtures'],
    'forbidden_not_read': ['runtime modules.rs', 'candidate or runtime diff',
                           'p1327-baseline.json and full-state dumps', 'materialization', 'context'],
    'hashes': {str(p.relative_to(root)): runner.sha(p) for p in files},
    'pre_C': True,
    'binary_provenance': oracle['binaries'],
    'state_provenance': 'p1327-manifest.json links parent-owned state receipt; author does not read prohibited runtime state',
    'measurement_interval': [min(r['started'] for r in oracle['baseline'] + oracle['vanilla']),
                             max(r['started'] for r in oracle['baseline'] + oracle['vanilla'])],
    'cases_per_profile': dict(collections.Counter(c[2] for c in runner.CASES)),
    'pre_C_invocations': 224,
    'classification': oracle['precheck'],
    'internal_test_count': 6,
    'legacy_observations_changed': 6,
    'legacy_test_functions_changed': 4,
    'comparison': 'full exit/stdout/stderr, no normalization or generic warning stripping',
    'correction_contract': 'full pinned vanilla transcript; baseline exit/stdout unchanged',
    'debt_contract': 'full pinned baseline transcript for only redundant-rename and type-error',
    'unknown': 'none in pre-C corpus; any missing/invalid required observation blocks',
    'calibration': 'one complete pre-C corpus; zero semantic revisions; rustfmt-only before freeze',
    'gate_commands': [
        'cargo test -p typst-core p1327 -- --nocapture',
        'cargo test -p typst-core p1305_global_bare -- --nocapture',
        'cargo test -p typst-core p1305_global_alias_bare -- --nocapture',
        'cargo test -p typst-core p1305_import_binding_negatives_and_dynamic_spans -- --nocapture',
        'cargo test -p typst-core p1306_existing_lookup_and_repr_all_profiles -- --nocapture',
    ],
    'candidate_cli_template': 'python3 00_nucleo/diagnosticos/p1327-ab-runner.py candidate /repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos/p1327-ab-cli-candidate.json --candidate BINARY --oracle 00_nucleo/diagnosticos/p1327-ab-cli-oracle.json',
    'limitations': 'Package name in gate_commands must be confirmed by parent; no runtime/cargo manifest access granted to author. Frozen fragment only, no general parity or isolation attestation.',
}
runner.save(prefix / 'p1327-ab-freeze.json', receipt)
print(json.dumps({'hashes': receipt['hashes'], 'receipt_sha256': runner.sha(prefix / 'p1327-ab-freeze.json')}, indent=2))
