"""Conjunctive successor: exact causal resources, errors and scenario witnesses.

Consumes only the existing passive DTO. No product execution or reconstruction.
"""
import argparse, importlib.util, json
from pathlib import Path
P = Path(__file__).with_name('p1340-contract-base-r4.py')
spec = importlib.util.spec_from_file_location('p1339_lifecycle_v2', P)
prior = importlib.util.module_from_spec(spec); spec.loader.exec_module(prior)
require = prior.require
R4 = Path(__file__).with_name('p1340-contract-observability-r4.py')
r4spec = importlib.util.spec_from_file_location('p1340_observability_r4', R4)
r4 = importlib.util.module_from_spec(r4spec); r4spec.loader.exec_module(r4)

def check_stable_requests(actual):
    """Frozen four stable bodies each call final once against an empty fold."""
    require([r['operation'] for r in actual] == ['CounterFold', 'CounterFinal'],
            'R3-stable-complete-fold-final-pair')
    require(actual[0]['result'] == {'kind':'Ok','value':[]},
            'R3-stable-actual-empty-fold')
    require(actual[1]['result'] == {'kind':'Ok','value':[0]},
            'R3-stable-language-final-zero')

def check(case, observed):
    r4.check(observed)
    answer = prior.check(case, observed)
    events = observed['events']
    def rows(kind): return [e for e in events if e['kind'] == kind]
    def bodies(index):
        offset = case['context_coordinates'][index]['byte_offset']
        return [e for e in rows('BodyStarted') if e['attempt'] > 0 and e['source_context_offset'] == offset]
    def requests(body): return [r for r in rows('RequestRecorded') if r['execution_id'] == body['execution_id']]
    def completion(body):
        return next(e for e in rows('BodyCompleted') if e['execution_id'] == body['execution_id'])
    candidates = rows('CandidateBuilt'); starts = rows('AttemptStarted')
    final = rows('CompilationReturned')[0]
    published = [e['sink_id'] for e in rows('SinkDisposition') if e['disposition'] == 'published']
    require(final['published_sink_ids'] == published, 'V3-final-published-sink-bijection')
    cid = case['id']
    if cid in ('lifecycle-retained-sibling-growth', 'lifecycle-replaced-capture-descendant', 'lifecycle-replaced-chain-descendant'):
        require(all(e['counter_values']['stable'] == [0] for e in starts + candidates), 'V3-stable-zero-every-read-and-candidate')
        index = 0 if cid == 'lifecycle-retained-sibling-growth' else 1
        selected = bodies(index)
        require(selected and all(requests(b) for b in selected), 'V3-stable-requests-exist')
        for b in selected: check_stable_requests(requests(b))
        if index == 1:
            require(len({b['resources']['capture_id'] for b in selected}) == 5, 'V3-replacement-capture-identities-distinct')
            require(len({b['parent_generation_id'] for b in selected}) == 5 and all(b['parent_generation_id'] is not None for b in selected), 'V3-replacement-parent-generations-distinct')
            for old, new in zip(selected, selected[1:]):
                invalidations = [i for i in rows('GenerationInvalidated') if old['generation_id'] == i['generation_id'] or old['generation_id'] in i['descendant_ids']]
                require(any(i['sequence'] < new['sequence'] for i in invalidations), 'V3-old-child-invalidated-before-new-body')
            for b in selected:
                require(all(r['resources'] == b['resources'] for r in requests(b)), 'V3-child-request-capture-resources-at-boundary')
            if cid == 'lifecycle-replaced-chain-descendant':
                require(len({b['resources']['entry_chain_id'] for b in selected}) == 5, 'V3-replacement-entry-chain-identities-distinct')
                require(all(r['chain_id'] == b['resources']['entry_chain_id'] for b in selected for r in requests(b)), 'V3-child-request-actual-entry-chain')
    if cid == 'lifecycle-request-chain-points':
        first, second = bodies(1), bodies(2)
        require(first and second, 'V3-two-child-contexts-exist')
        require({b['generation_id'] for b in first}.isdisjoint({b['generation_id'] for b in second}), 'V3-sibling-record-generations-disjoint')
        require({b['resources']['entry_chain_id'] for b in first}.isdisjoint({b['resources']['entry_chain_id'] for b in second}), 'V3-sibling-entry-chains-disjoint')
        for b in first + second:
            require(requests(b), 'V3-sibling-request-exists')
            require(all(r['chain_id'] == b['resources']['entry_chain_id'] for r in requests(b)), 'V3-sibling-request-chain')
            check_stable_requests(requests(b))
    if cid in ('lifecycle-increasing_set', 'lifecycle-oscillating_set'):
        expected = case['typed_predicates']
        history = [starts[0]['counter_values']['c']] + [e['counter_values']['c'] for e in candidates]
        require(history == [expected['counter_values_read'][0]] + expected['counter_values_produced'], 'V3-exact-historical-counter-projection-I0-I5')
        require(not any(b['attempt'] > 5 for b in rows('BodyStarted')), 'V3-no-A6-body')
        require(final['attempt'] == 5, 'V3-return-at-A5')
        require(all(len(c['counter_events']) == 1 and c['counter_events'][0]['action']['kind'] == 'Set' for c in candidates), 'V3-one-current-Set-zero-automatic-events')
    if cid in ('lifecycle-nested_producer', 'lifecycle-fresh_callback_stable', 'lifecycle-nan_key_stable_zero'):
        require(final['result']['kind'] == 'Ok', 'V3-stable-success-not-error')
        require(not any(d['severity'] == 'Warning' for s in rows('SinkDisposition') if s['disposition'] == 'published' for d in s['diagnostics']), 'V3-stable-no-published-warning')
    if cid in ('lifecycle-provisional-assert-final-panic', 'lifecycle-legacy-sibling-error', 'lifecycle-selected-stable-error'):
        matches = [b for b in rows('BodyCompleted') if b['result'] == final['result'] and b['result']['kind'] == 'Err']
        require(matches, 'V3-final-complete-error-equals-real-body-error')
        require(not any(d['severity'] == 'Warning' for s in rows('SinkDisposition') if s['disposition'] == 'published' for d in s['diagnostics']), 'V3-error-not-downgraded-by-published-warning')
    if cid == 'lifecycle-selected-stable-error':
        actual = bodies(0)
        require(actual and all(completion(b)['selected'] is True for b in actual), 'V3-missing-label-body-selected')
        require(all(requests(b) and all(r['result']['kind'] == 'Err' for r in requests(b)) for b in actual), 'V3-missing-label-real-error-requests')
        require(not rows('FunctionInvoked'), 'V3-missing-label-no-callback')
        require(all(completion(b)['output_id'] is None for b in actual), 'V3-missing-label-no-success-output')
    return answer

if __name__ == '__main__':
    p = argparse.ArgumentParser(); p.add_argument('--fixtures', required=True); p.add_argument('--observations', required=True); a = p.parse_args()
    fixture = json.loads(Path(a.fixtures).read_text()); obs = json.loads(Path(a.observations).read_text())['cases']
    lookup = {c['id']: c for c in fixture['cases']}
    expected = {(c['id'], profile, order) for c in lookup.values() for profile in c['execution']['profiles'] for order in c['execution']['orders']}
    actual = [(x['case_id'], x['profile'], x['order']) for x in obs]
    require(len(actual) == len(set(actual)) and set(actual) == expected, 'V3-exact-144-required-cells')
    for row in obs: print(json.dumps(check(lookup[row['case_id']], row), ensure_ascii=True))
