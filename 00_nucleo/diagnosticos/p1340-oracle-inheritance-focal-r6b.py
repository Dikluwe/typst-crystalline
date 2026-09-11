"""Synthetic wiring focal for the one authorized R6b inheritance correction.

It runs the real scenario-specific body of p1340-contract-predicate-r4.py.
Only its already-tested lower conjunctions are replaced with accepting stubs,
so no product/candidate observation is needed to isolate composition.
"""

import importlib.util
import json
from pathlib import Path


HERE = Path(__file__).parent


def load(name, filename):
    spec = importlib.util.spec_from_file_location(name, HERE / filename)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


old = load('p1340_predicate_r6_frozen', 'p1340-contract-predicate-r6.py')
new = load('p1340_predicate_r6b', 'p1340-contract-predicate-r6b.py')


def accept_lower(*_args, **_kwargs):
    return {'lower': 'Preserved'}


def accept_fragment(_case, _observed):
    return {'classification': 'Preserved'}


# Isolate only the composition edge. The old entrypoint called this lower
# checker directly; the new one must enter the real R4 scenario checker.
old.prior.check = accept_lower
old.r6.classify = accept_fragment
new.predecessor.prior.check = accept_lower
new.predecessor.r4.check = lambda _observed: None
new.r6.classify = accept_fragment


def event(kind, **fields):
    return dict(kind=kind, **fields)


def sink_mutant():
    case = {'id': 'synthetic-sink'}
    observed = {'events': [
        event('CompilationReturned', published_sink_ids=['published-but-absent'])
    ]}
    return case, observed, 'V3-final-published-sink-bijection'


def fold_mutant():
    case = {'id': 'lifecycle-retained-sibling-growth',
            'context_coordinates': [{'byte_offset': 7}],
            'typed_predicates': {}}
    observed = {'events': [
        event('AttemptStarted', counter_values={'stable': [0]}),
        event('BodyStarted', attempt=1, source_context_offset=7,
              execution_id='body', resources={}),
        event('RequestRecorded', execution_id='body', operation='CounterFinal',
              result={'kind': 'Ok', 'value': [0]}),
        event('CandidateBuilt', counter_values={'stable': [0]}),
        event('CompilationReturned', published_sink_ids=[]),
    ]}
    return case, observed, 'R3-stable-complete-fold-final-pair'


def history_mutant():
    case = {'id': 'lifecycle-increasing_set',
            'context_coordinates': [],
            'typed_predicates': {
                'counter_key_id': 'c',
                'counter_values_read': [[0]],
                'counter_values_produced': [[1]],
            }}
    observed = {'events': [
        event('AttemptStarted', counter_values={'c': [0]}),
        event('CandidateBuilt', counter_values={'c': [9]},
              counter_events=[{'action': {'kind': 'Set'}}]),
        event('CompilationReturned', published_sink_ids=[], attempt=5),
    ]}
    return case, observed, 'V3-exact-historical-counter-projection-I0-I5'


def callback_mutant():
    error = {'kind': 'Err', 'diagnostics': [{'message': 'missing label'}]}
    case = {'id': 'lifecycle-selected-stable-error',
            'context_coordinates': [{'byte_offset': 11}],
            'typed_predicates': {}}
    observed = {'events': [
        event('BodyStarted', attempt=1, source_context_offset=11,
              execution_id='body', resources={}),
        event('RequestRecorded', execution_id='body', result=error),
        event('BodyCompleted', execution_id='body', result=error,
              selected=True, output_id=None),
        event('FunctionInvoked'),
        event('CompilationReturned', published_sink_ids=[], result=error),
    ]}
    return case, observed, 'V3-missing-label-no-callback'


def main():
    rows = []
    for name, factory in [
        ('sink-bijection', sink_mutant),
        ('stable-fold-pair', fold_mutant),
        ('historical-I0-I5', history_mutant),
        ('selected-error-callback', callback_mutant),
    ]:
        case, observed, reason = factory()
        before = old.classify(case, observed)
        after = new.classify(case, observed)
        rows.append({
            'id': name,
            'expected_before': 'Preserved',
            'actual_before': before['classification'],
            'expected_after': 'Violated',
            'actual_after': after['classification'],
            'expected_reason_fragment': reason,
            'actual_reason': after.get('reason_code'),
        })
    ok = all(row['actual_before'] == row['expected_before'] and
             row['actual_after'] == row['expected_after'] and
             row['expected_reason_fragment'] in row['actual_reason']
             for row in rows)
    print(json.dumps({
        'schema': 'p1340-oracle-inheritance-focal-r6b-result',
        'scope': 'synthetic-composition-only-no-product-credit',
        'predecessor_module': Path(new.predecessor.__file__).name,
        'cases': rows,
        'delta': {'survived_before': sum(row['actual_before'] == 'Preserved' for row in rows),
                  'killed_after': sum(row['actual_after'] == 'Violated' for row in rows)},
    }, indent=2))
    return 0 if ok else 1


if __name__ == '__main__':
    raise SystemExit(main())
