import datetime, hashlib, json, pathlib, re, subprocess
from importlib.machinery import SourceFileLoader

runner = SourceFileLoader('p1326_ab_cli', '00_nucleo/diagnosticos/p1326-ab-cli.py').load_module()
base = pathlib.Path('00_nucleo/diagnosticos')
freeze = json.loads((base/'p1326-ab-freeze.json').read_text())
baseline = json.loads((base/'p1326-ab-baseline.json').read_text())
baseline_focal = json.loads((base/'p1326-ab-baseline-focal.json').read_text())
focal = json.loads((base/'p1326-ab-final-focal.json').read_text())
final = json.loads((base/'p1326-ab-final.json').read_text())
integrity = {path: runner.sha(path) == digest for path, digest in freeze['artifacts'].items()}
prompt = pathlib.Path('00_nucleo/prompts/compiler/eval/bindings/field_access.md')
normalized = re.sub('^Hash do Código: [0-9a-f]{8}\n'.encode(), b'', prompt.read_bytes(), flags=re.M)
integrity['prompt_normalized'] = hashlib.sha256(normalized).hexdigest() == freeze['prompt_norm_sha256']
nucleus = pathlib.Path('00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml')
integrity['nucleus_effective'] = hashlib.sha256(nucleus.read_bytes()+b'\0TEKT-NUCLEUS-DEPS-V1\0').hexdigest() == freeze['nucleus_effective_sha256']
candidate = '/tmp/p1326-target.6Vi4Km/release/typst'
integrity['candidate_binary'] = runner.sha(candidate) == 'bd86b34602323a390a60a8f41b97b180f93077affcd30b6b1e85aa5926dee811'
integrity['baseline_binary'] = runner.sha(runner.BASELINE) == baseline['binaries'][runner.BASELINE]
integrity['vanilla_binary'] = runner.sha(runner.VANILLA) == baseline['binaries'][runner.VANILLA]
repeat_equal = []
for key in {(r['id'], r['profile']) for r in final['records']}:
    outputs = [r['candidate']['output'] for r in final['records'] if (r['id'], r['profile']) == key]
    repeat_equal.append(len(outputs) == 3 and outputs[0] == outputs[1] == outputs[2])
overlap_equal = []
for r in baseline_focal['records']:
    other = next(x for x in baseline['records'] if (x['id'],x['profile']) == (r['id'],r['profile']))
    overlap_equal.append(all(r[k]['output'] == other[k]['output'] for k in ['baseline','vanilla']))
passed = all(integrity.values()) and all(repeat_equal) and all(overlap_equal) and focal['counts'] == {'Preserved':72} and final['counts'] == {'Preserved':276}
receipt = {
    'at': runner.utc(),
    'verdict': 'PASS_SCOPED' if passed else 'FAIL',
    'regime': 'A/B executed without technical isolation attestation; no refinement seal',
    'executor': '/root/p1326_tests',
    'manifest_sha256': runner.sha(base/'p1326-manifest.json'),
    'freeze_sha256': runner.sha(base/'p1326-ab-freeze.json'),
    'candidate': {'path':candidate,'sha256':runner.sha(candidate)},
    'head': runner.command(['git','rev-parse','HEAD']),
    'working_tree_stat': runner.command(['git','diff','HEAD','--stat']),
    'artifacts': {str(base/f):runner.sha(base/f) for f in ['p1326-ab-baseline-focal.json','p1326-ab-baseline.json','p1326-ab-final-focal.json','p1326-ab-final.json','p1326-ab-tests-r1.rs','p1326-ab-legacy-successor.rs','p1326-ab-verdict.py','p1326-ab-cli-transport-r1.py']},
    'integrity_checks':integrity,
    'focal': {'started':focal['started'],'finished':focal['finished'],'counts':focal['counts'],'elapsed_seconds':focal['elapsed_seconds']},
    'full': {'started':final['started'],'finished':final['finished'],'counts':final['counts'],'elapsed_seconds':final['elapsed_seconds']},
    'scope_counts_full': {p:sum(r['policy']==p for r in final['records']) for p in ['closure','preserve','residual']},
    'unknown':0 if passed else 'not asserted',
    'candidate_repeat_reverse_stable_keys':sum(repeat_equal),
    'baseline_focal_full_equal_keys':sum(overlap_equal),
    'temporal_limits':'Full baseline and vanilla corpus was measured once before C; focal overlap additionally measured twice. Candidate normal/repeat/reverse compared complete outputs against frozen pre-C envelopes. Candidate repeat stability is observed over the recorded final interval only; no full baseline reverse run, concurrent race gate, technical isolation or general parity claim.',
    'residuals':freeze['residuals'],
    'unit_tests':'Parent reports GREEN 18 passed, 0 failed. Unit receipt and runtime owner not read by A/B author; independent reviewer owns unit-result audit.',
    'transport_revision': {'cause':'First full final process reached receipt persistence and exited with OSError [Errno 7] Argument list too long: apply_patch. Its unpersisted observations receive no verdict.', 'change':'Immutable runner loaded by wrapper; only save() transport changes identical apply_patch text from argv to stdin. Corpus, classifiers, baselines and candidate unchanged.', 'hypothesis':'stdin removes per-argument OS size limit and permits full receipt persistence.', 'cost':'One additional full execution required after persistence failure; failed attempt exact observation times and outputs unavailable and not invented.', 'discriminatory_change':'None; only observability of receipt persistence repaired.', 'revisions_same_cause':1},
    'read_constraints':'No runtime owner, owner diff, candidate source, forbidden p1326-baseline.json, context or materialization read.',
}
print(runner.save('p1326-ab-verdict.json',receipt))
print(json.dumps({'verdict':receipt['verdict'],'candidate_repeat_reverse_stable_keys':sum(repeat_equal),'integrity':all(integrity.values()),'counts':final['counts']}))
