"""Read-only verification of the frozen R5 terminal candidate receipt; no product run."""
import base64
import datetime
import hashlib
import json
import pathlib
import re
import subprocess

D = pathlib.Path('00_nucleo/diagnosticos')
def sha(p):
    return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def get(name):
    return json.loads((D / name).read_text())

seal = get('p1340-verifier-terminal-r5-seal.json')
for p, h in seal['protected_inputs'].items():
    assert sha(p) == h, p
run_path = D / 'p1340-terminal-r5-candidate-focal.json'
assert sha(run_path) == 'e9ce7ab13bca0669a43c388a2aa6d13b936a716e7e04b5fba0d700f8e6eef384'
run = json.loads(run_path.read_text())
baseline = get('p1340-terminal-r5-baseline.json')
assert run['exit'] == 0 and run['source_unchanged'] is True
for field in ['head', 'modified', 'new_sources']:
    assert baseline['before'][field] == run['before'][field] == run['after'][field], field
for field in ['modified', 'new_sources']:
    for p, h in run['after'][field].items():
        assert sha(p) == h, p
for channel in ['stdout', 'stderr']:
    assert base64.b64decode(run[channel + '_base64']).decode() == run[channel]
assert 'test result: ok. 1 passed; 0 failed;' in run['stdout']
rows = [json.loads(s) for s in re.findall(r'P1340_TERMINAL_OBSERVATION (\{[^\n]*\})', run['stdout'])]
names = ['OpaqueAtCeiling', 'MixedDifferentAndOpaque', 'QueryDifferentThenStable',
         'OriginalErrorPrecedence', 'ProvenImpossibleTopologyTerminal', 'ClosedIncreasing']
assert [r['case'] for r in rows] == names + names + names[::-1]
harness = (D / 'p1340-contract-terminal-harness-r5.rs').read_text()
sources = dict((name, json.loads(literal)) for name, literal in re.findall(
    r'Case::(\w+) => ("(?:\\.|[^"\\])*")', harness))
terminal = 'contextual stability could not be verified'
for i, r in enumerate(rows, 1):
    assert r['source_text'] == sources[r['case']]
    root = r['warnings'][0]['span']
    assert root['file_id'] == str(i)
    assert all(w['severity'] == 'Warning' for w in r['warnings'])
    messages = [w['message'] for w in r['warnings']]
    assert messages.count('P1340_GLOBAL') == 1
    assert not set(messages) & {'P1340_DISCOVERY', 'P1340_REPLACED', 'P1340_VALIDATION'}
    for w in r['warnings']:
        if w['message'].startswith('P1340_'):
            assert w['span'] == root and not w['hints'] and not w['trace']
    assert r['attempts'] == list(range(1, len(r['attempts']) + 1))
    assert len(r['attempts']) <= 5
    case = r['case']
    if case in names[:2] + [names[4]]:
        assert r['result_errors'] == [dict(severity='Error', span=root, message=terminal, hints=[], trace=[])]
        assert r['export_calls'] == 0 and r['original_errors'] == []
    if case in names[:2]:
        assert r['selected_body_ok'] and r['opaque_query_seen']
        assert r['attempts'] == [1, 2, 3, 4, 5]
        assert messages == ['P1340_GLOBAL', 'P1340_RETAINED']
    elif case == names[2]:
        assert r['result_errors'] is None and r['metadata'] == [1, 1]
        assert r['attempts'] == [1, 2] and r['export_calls'] == 1
    elif case == names[3]:
        assert r['result_errors'] == r['original_errors']
        assert len(r['result_errors']) == 1
        assert r['result_errors'][0]['message'] == 'panicked with: P1340_ORIGINAL'
        assert r['result_errors'][0]['span'] != root
        assert r['export_calls'] == 0 and r['opaque_query_seen']
    elif case == names[4]:
        assert r['topology_impossibility'] and r['attempts'] == []
        assert messages == ['P1340_GLOBAL']
    elif case == names[5]:
        assert r['result_errors'] is None and r['metadata'] == [4]
        assert r['final_read_counter'] == [4] and r['attempts'] == [1, 2, 3, 4, 5]
        assert r['export_calls'] == 1 and messages.count('P1340_RETAINED') == 1
        assert 'document did not converge within five attempts' in messages

def relative(v):
    # Span::NUMBER_BITS is 48 (entities/span.rs:41-43). Preserve all within-file bits.
    # Fresh Source IDs and opaque Arc addresses are invocation identities, not equal
    # across fresh worlds. All raw identities remain in the original receipt.
    if isinstance(v, dict):
        if set(v) == {'raw_span', 'file_id'}:
            raw = int(v['raw_span'])
            if v['file_id'] is not None:
                assert raw >> 48 == int(v['file_id'])
                return {'file': 'this-invocation', 'low48': raw & ((1 << 48) - 1)}
            return v
        if set(v) == {'Func'}:
            assert re.fullmatch(r'0x[0-9a-f]+', v['Func'])
            return {'Func': 'opaque-retained-input-per-invocation'}
        return {k: relative(x) for k, x in v.items()}
    if isinstance(v, list):
        return [relative(x) for x in v]
    return v
for n in names:
    three = [relative(r) for r in rows if r['case'] == n]
    assert three[0] == three[1] == three[2], n

binary = pathlib.Path(re.search(r'Running unittests src/lib.rs \(([^)]+)\)', run['stderr'])[1])
adapter = D / 'p1340-implementation-pipeline-observer.rs'
old = base64.b64decode(get('p1340-terminal-binding-inputs-r4.json')['observer_base64']).decode()
new = adapter.read_text()
delta = list(__import__('difflib').unified_diff(old.splitlines(True), new.splitlines(True)))
receipt = {
    'schema': 'p1340-verifier-terminal-r5-candidate-fragment',
    'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'verdict': 'PASS_TERMINAL_R5_CANDIDATE_FRAGMENT_ONLY',
    'regime': 'executado sem atestação de isolamento',
    'seal_sha256': sha(D / 'p1340-verifier-terminal-r5-seal.json'),
    'candidate_receipt_sha256': sha(run_path),
    'candidate_run': {k: run[k] for k in ['command', 'cwd', 'start', 'end', 'seconds', 'exit', 'environment']},
    'head': run['before']['head'], 'working_tree': 'uncommitted; exact status/diff and source hashes in candidate receipt',
    'test_binary': {'path': str(binary), 'sha256': sha(binary), 'pin_timing': 'independently hashed after completed run, not claimed as a before-run binary pin'},
    'adapter': {'path': str(adapter), 'sha256': sha(adapter), 'delta_from_earliest_r4_adapter': ''.join(delta),
                'review': 'R5 include/comment switch plus previously disclosed R4 private visibility/format/exhaustive Json leaf match. No change to observation values, opaque insertion, sink hooks, real compile/export binding or assertions.'},
    'protected_inputs': seal['protected_inputs'],
    'checks': {'all_sealed_pins_intact': True, 'baseline_product_snapshot_equals_candidate_before_after_and_current': True,
               'lossless_channels': True, 'real_rust_tests_passed': 1, 'actual_observations': 18,
               'cases': 6, 'orders': ['normal', 'repeat', 'reverse'], 'unknown_in_fragment': 0,
               'exact_diagnostics_hints_traces': True, 'original_error_vector_preserved': True,
               'same_sources': True, 'relative_identity_repeat_reverse_equal': True},
    'binding_review': [
        'Source::new receives frozen literal; R5 harness is included directly unchanged. Real compile_to_pdf_bytes_impl for T01-T04/T06, document Err/Ok and exporter events captured at real boundaries.',
        'Post-eval input transform replaces exactly one Int(0) metadata at opaque label, before discovery; actual request/replay query result contains retained Func identity. This is a carrier witness, not independent comparison-relation proof.',
        'before_decision reads actual session.errors and selected execution results; terminal root-span assertions execute in unchanged Rust harness against the actual Source root, not inferred solely from sentinel equality.',
        'Warnings injected at actual sinks; only global and retained sentinels survive. Discarded sink inputs remain counted in raw sink_inputs.',
        'T05 directly exercises real terminal<T> with normed impossible session. It does not prove that a source program discovers impossible topology.',
        'Repeat comparison retains all channel fields and low48 span numbers; only per-invocation Source identity and Func allocation identity are mapped. Each raw remains intact.'
    ],
    'limits': [
        'No new product processes executed by verifier; this is independent audit of the recorded actual candidate test.',
        'R5 input mutation score15/15 is not terminal-product branch mutation score. NT01-NT06 real branch attacks remain pending.',
        'Ordinary public cases/historical exact nonconvergence warnings remain their separate gate; their success is not inferred from this instrumented test.',
        'R4 observability/lifecycle remains NOT_SEALED; no full F, P1340/P1339 closure, architectural zero-violations, or general equivalence claim.'
    ]
}
out = D / 'p1340-verifier-terminal-r5-candidate-verdict.json'
assert not out.exists()
body = json.dumps(receipt, ensure_ascii=False, indent=2) + '\n'
patch = '*** Begin Patch\n*** Add File: ' + str(out) + '\n' + ''.join('+' + line + '\n' for line in body.splitlines()) + '*** End Patch\n'
subprocess.run(['apply_patch'], input=patch, text=True, check=True, capture_output=True)
print(json.dumps({'path': str(out), 'sha256': sha(out), 'verdict': receipt['verdict'], 'observations': len(rows), 'binary_sha256': sha(binary)}))
