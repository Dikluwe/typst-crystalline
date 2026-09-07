#!/usr/bin/env python3
"""Independent read-only checkpoint: exact existing observations, no product runs."""
import collections
import datetime as dt
import hashlib
import json
from pathlib import Path
import re
import runpy
import subprocess

H = Path(__file__).resolve().parent
R = H.parent.parent
v = runpy.run_path(str(H / 'p1308-verify.py'))
sha, pin, read = v['sha'], v['pin'], v['read']
b = read(H / 'p1308-baseline.json')
m = read(H / 'p1308-manifest.json')
successor = read(H / 'p1308-manifest-successor-1.json')
freeze = read(H / 'p1308-verification-candidate-freeze.json')
failures = []
for name, digest in b['predecessors'].items():
    if sha((H / name).read_bytes()) != digest:
        failures.append('predecessor:' + name)
for ref in m['inputs']:
    if sha((R / ref['path']).read_bytes()) != ref['sha256']:
        failures.append('protected input:' + ref['path'])
for name, digest in freeze['source_inventory'].items():
    if sha((R / name).read_bytes()) != digest:
        failures.append('candidate changed after freeze:' + name)
for contract in m['contracts']:
    if contract['path'] == successor['successor_L0']['path']:
        contract = successor['successor_L0']
    if v['L0_semantic']((R / contract['path']).read_bytes()) != contract['semantic_sha256']:
        failures.append('semantic L0:' + contract['path'])
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=R).decode().strip() == b['state']['head'].strip()
assert subprocess.check_output(['git','diff','--cached','--binary'],cwd=R).decode() == b['state']['staged']

blocks = {}
for name in ('p1308-tests.patch','p1308-tests-supplement.patch'):
    for block in re.split(r'(?m)(?=^--- a/)', (H/name).read_text()):
        match = re.match(r'--- a/(.+)\n\+\+\+ b/(.+)\n',block)
        if match:
            assert match[1] == match[2]
            blocks.setdefault(match[1],[]).append(block)
for path, patches in blocks.items():
    data = v['baseline_bytes'](b,path)
    for patch in patches:
        data = v['apply_block'](data,patch,path)
    if v['formatted_semantic'](data) != v['formatted_semantic']((R/path).read_bytes()):
        failures.append('independent test integration:' + path)

oracle_path = H/'p1307-r6-oracle.json'
assert sha(oracle_path.read_bytes()) == '99d2a67984a468c8e86e20010ecc1bd76a364f1d1adebbb5b873fc341f7790a6'
oracle = {c['id']:c for c in read(oracle_path)['cases']}
helper = runpy.run_path(str(H/'p1307-r4-oracle.py'))
receipt = read(H/'p1308-public-matrix.json')
matrix = json.loads(receipt['stdout'])
old = json.loads(read(H/'p1307-r6-public-matrix-2.json')['stdout'])
results, proofs = {}, []
for row in matrix['rows']:
    case = dict(oracle[row['case']])
    assert row['source_sha256'] == case['source_sha256']
    if case['route'] == 'compile':
        assert row['argv'][1] == 'compile'
        case['fixture'] = row['argv'][2]
    expected = case['observations'][row['profile']]['future_expected']
    observed = helper['envelope'](row['returncode'],row['stdout'],row['stderr'],case)
    assert observed == row['observable']
    verdict = helper['classify'](expected,observed)
    assert verdict == row['verdict']
    results[(row['case'],row['profile'])] = verdict
    if verdict == 'Violated':
        assert {k:x for k,x in expected.items() if k!='stderr'} == {k:x for k,x in observed.items() if k!='stderr'}
        assert observed['stderr'].startswith(expected['stderr'])
        suffix = observed['stderr'][len(expected['stderr']):]
        assert re.fullmatch(r'  while calling `[^`]+` at <input-expression>:\d+:\d+\n    [^\n]*\n\n',suffix)
        proofs.append(dict(case=row['case'],profile=row['profile'],unchanged_non_stderr=True,
                           expected_stderr_preserved_as_prefix=True,exact_added_trace=suffix))
assert len(results) == len(matrix['rows']) == 1982
counts = dict(collections.Counter(results.values()))
assert counts == matrix['counts'] or {**counts,'Unknown':0} == matrix['counts']
old_failed = {(r['case'],r['profile']) for r in old['rows'] if r['verdict']=='Violated'}
new_failed = {key for key,value in results.items() if value=='Violated'}
assert len(old_failed)==56 and all(results[key]=='Preserved' for key in old_failed)
assert not old_failed & new_failed and len(new_failed)==76
grouped = []
for case in sorted({p['case'] for p in proofs}):
    rows = [p for p in proofs if p['case']==case]
    assert {p['profile'] for p in rows} == {'default','html','a11y','html+a11y'}
    assert len({p['exact_added_trace'] for p in rows}) == 1
    grouped.append(dict(case=case,profiles=[p['profile'] for p in rows],exact_added_trace=rows[0]['exact_added_trace']))
workspace = read(H/'p1308-workspace-tests.json')
totals = [tuple(map(int,x)) for x in re.findall(r'test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',workspace['stdout'])]
assert tuple(map(sum,zip(*totals))) == (6619,1,3) and workspace['exit']==101
gates = {}
for name in ('green-2','build','lint','fmt','workspace-tests','public-focal','public-focal-reverse','public-matrix'):
    path = H/f'p1308-{name}.json'; row = read(path)
    gates[name] = dict(receipt=pin(path),exit=row['exit'],at=row['at'],end=row['end'])
    if name.startswith('public-'):
        data = json.loads(row['stdout']); gates[name]['counts']=data['counts']
        assert data['binary_sha256'] == freeze['release_binary_sha256']
    if name in ('public-focal','public-focal-reverse'):
        assert data['counts'] == {'Preserved':236,'Violated':0,'Unknown':0}
assert '18 passed; 0 failed' in read(H/'p1308-green-2.json')['stdout']
result = dict(schema='p1308-independent-final-verification-v1',at=dt.datetime.now(dt.timezone.utc).isoformat(),
    executor='/root/p1306_oracle',regime=m['regime'],verdict='BLOQUEADO_POR_DECISAO_CONTRATO',
    checker=pin(__file__),candidate_freeze=pin(H/'p1308-verification-candidate-freeze.json'),
    manifest=pin(H/'p1308-manifest.json'),normative_successor=pin(H/'p1308-manifest-successor-1.json'),
    integrity_failures=failures,head_and_index_unchanged=True,protected_predecessors_unchanged=True,
    canonical_independent_test_integrations=sorted(blocks),test_migrations=3,new_tests=18,
    gates=gates,global_workspace=dict(passed=6619,failed=1,ignored=3,status='FAIL',
    failure='p1293_d_preexisting_arity_and_serializer_transcript_is_frozen'),
    exact_frozen_classifier_replayed_offline=True,product_processes_repeated_by_verifier=0,
    previous_56_failed_cells_now_preserved=True,new_violated_cells=76,new_violated_cases=19,
    new_violations_all_only_append_one_literal_trace=True,trace_deltas=grouped,
    causal_review='Candidate Source overlay exposes the already parsed expression to existing trace_call containment. Seven productive owners changed within declared scope; closed native panic identity, callback origin and method routing conform to reviewed L0. No CBOR/decoder validation was changed. Literal new traces explain the 76 deltas but remain Violated against protected expectations.',
    architecture_limit='No new public World method, I/O or global mutable state was introduced by reviewed deltas. Overlay delegates every current World method. This source audit and focal GREEN are not a full architecture/product certificate.',
    contract_decision_required='Decide explicitly whether protected 19 R6 cases and the remaining P1293 transcript should accept natural Source-resolved traces while keeping primary messages/ranges/hints and validation. No expected is migrated by this verification.',
    mutation_campaign=dict(status='SUSPENDED_BEFORE_ANY_BUILD',control_builds=0,actual_Rust_mutants=0,score=None,pending_P1308=6,pending_P1307=37,lease_suspension=pin(H/'p1308-mutation-lease-suspension.json')),
    full_discriminatory_seal=False,candidate_accepted=False,
    chronology='Successful pre-candidate integrity checkpoint preceded edits; manifest file followed coordinator interpretation of the success message as GO. No retroactive timestamp or isolation attestation.',
    preserved_disposable_paths=['/dev/shm/p1308-mutant-source.PmokqQ','/dev/shm/p1308-mutant-target.NV7BUV'])
assert not failures, failures
print(json.dumps(result,ensure_ascii=False,indent=2))
