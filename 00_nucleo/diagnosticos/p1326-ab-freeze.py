import hashlib, json, pathlib, re
from importlib.machinery import SourceFileLoader

runner = SourceFileLoader('p1326_ab_cli', '00_nucleo/diagnosticos/p1326-ab-cli.py').load_module()
base = pathlib.Path('00_nucleo/diagnosticos')
prompt = pathlib.Path('00_nucleo/prompts/compiler/eval/bindings/field_access.md')
nucleus = pathlib.Path('00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml')
normalized = re.sub('^Hash do Código: [0-9a-f]{8}\n'.encode(), b'', prompt.read_bytes(), flags=re.M)
assert hashlib.sha256(normalized).hexdigest() == '4b3789c3decc0463ba553937ce7595c3a9c0ecc5fae545726ed009ad928d7451'
assert hashlib.sha256(nucleus.read_bytes() + b'\0TEKT-NUCLEUS-DEPS-V1\0').hexdigest() == '5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24'
measurement = json.loads((base/'p1326-ab-baseline.json').read_text())
assert 'Unknown' not in measurement['counts']
files = ['p1326-manifest.json','p1326-legacy-tests-original.rs','p1326-ab-tests.rs','p1326-ab-tests-r1.rs','p1326-ab-tests-r1.py','p1326-ab-legacy-successor.rs','p1326-ab-legacy-successor.py','p1326-ab-cli.py','p1326-ab-baseline-focal.json','p1326-ab-baseline.json','p1326-ab-freeze.py']
receipt = {
    'at': runner.utc(),
    'regime': 'A/B executed without technical isolation attestation; no refinement seal',
    'executor': '/root/p1326_tests',
    'environment': 'shared filesystem and tools; restrictions procedural, not technically enforced',
    'manifest_sha256': runner.sha(base/'p1326-manifest.json'),
    'prompt_norm_sha256': hashlib.sha256(normalized).hexdigest(),
    'nucleus_effective_sha256': hashlib.sha256(nucleus.read_bytes()+b'\0TEKT-NUCLEUS-DEPS-V1\0').hexdigest(),
    'artifacts': {str(base/f): runner.sha(base/f) for f in files},
    'permitted_reads': ['task and user instructions','skill and both references','root CLAUDE.md/AGENTS.md and 01_core/CLAUDE.md','ADRs','whole frozen field_access L0 and pinned nucleus','p1326-manifest.json','public World, Source, Value, Func, Feature, Span, SourceDiagnostic, Sink, Library, Route, eval entry signatures','p1326-legacy-tests-original.rs test-only extract','pinned baseline and vanilla CLI outputs','own p1326-ab-* artifacts'],
    'forbidden_reads_not_taken': ['runtime owner field_access.rs','owner git diff','p1326-baseline.json','materialization and context folders','candidate source'],
    'permitted_writes': ['00_nucleo/diagnosticos/p1326-ab-* via apply_patch; rustfmt only formatting own Rust snippets'],
    'head': runner.command(['git','rev-parse','HEAD']),
    'working_tree_stat': runner.command(['git','diff','HEAD','--stat']),
    'baseline_classification': measurement['counts'],
    'scope': 'Closure and With closure missing fields; complete public CLI outputs compare against vanilla for correction, baseline for all other cases',
    'residuals': ['raw Content [x].text remains baseline error while vanilla succeeds','f.nope(panic("arg")) preserves argument-before-lookup failure while vanilla fails lookup first'],
    'budget': '23 cases x 4 profiles; focal first; final normal/repeat/reverse; two consecutive no-gain revisions on one cause reopen design',
    'test_revision': 'r1 replaces only expected literal in native-present control with measured pretty JSON from baseline and vanilla; r0 preserved. No obligation changed. Hypothesis: default pretty encoder output resolves instrumentation mismatch. Positive control must pass before C; no claim of gain until execution.',
    'authoritative_unit_snippet': '00_nucleo/diagnosticos/p1326-ab-tests-r1.rs',
    'condition': 'before any candidate C; parent runs frozen unit tests for real RED; candidate release waits for freeze and RED',
}
print(runner.save('p1326-ab-freeze.json', receipt))
