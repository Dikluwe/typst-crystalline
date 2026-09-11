"""Authored predicate-unit controls only; never actual-product evidence.

Run before the product corpus. The verifier separately audits/executes real
bindings and records actual discriminatory delta/cost and acceptance.
"""
import copy
import importlib.util
import json
import time
from pathlib import Path

p = Path(__file__).with_name('p1340-contract-observability-r4.py')
spec = importlib.util.spec_from_file_location('r4_observability', p)
contract = importlib.util.module_from_spec(spec)
spec.loader.exec_module(contract)


def source_edge(kind, **extra):
    return dict(kind=kind, path='unit-fixture.rs', sha256='a' * 64, line=1, **extra)


def vectors():
    cases = []
    def add(name, function, args, expected):
        cases.append((name, function, args, expected))

    feature_edges = {'ordinary': source_edge('features:ordinary_default'),
                     'selected': source_edge('features:selected_profile'),
                     'inherit': source_edge('features:inherited')}
    ordinary = dict(execution_id='ordinary', generation_id='g0', sequence=0,
                    attempt=0, parent_generation_id=None, resources={'features': []},
                    feature_origin={'mode': 'ordinary_default', 'source_edge_ref': 'ordinary'})
    selected = dict(execution_id='selected', generation_id='g1', sequence=1,
                    attempt=1, parent_generation_id=None, resources={'features': ['html']},
                    feature_origin={'mode': 'selected_profile', 'source_edge_ref': 'selected'})
    inherited = dict(execution_id='child', generation_id='g2', sequence=2,
                     attempt=1, parent_generation_id='g1', resources={'features': ['html']},
                     feature_origin={'mode': 'inherited', 'source_edge_ref': 'inherit',
                                     'from_execution_id': 'selected'})
    f = ['html', [ordinary, selected, inherited], [], feature_edges]
    add('features-real-three-origins', contract.check_features, f, 'Preserved')
    for name, index, payload in [('discovery-claims-profile', 0, ['html']),
                                 ('selected-loses-profile', 1, []),
                                 ('inherited-loses-real-parent', 2, [])]:
        a = copy.deepcopy(f); a[1][index]['resources']['features'] = payload
        add(name, contract.check_features, a, 'Violated')
    a = copy.deepcopy(f); a[1][2]['parent_generation_id'] = 'unrelated'
    add('inherited-wrong-generation', contract.check_features, a, 'Violated')
    a = copy.deepcopy(f); a[2] = [{'execution_id': 'ordinary', 'resources': {'features': ['html']}}]
    add('request-fabricates-profile', contract.check_features, a, 'Violated')

    dispatch_edges = {'dispatch': source_edge('func_dispatch'),
                      'body': source_edge('invocation_scope', scope='context_body_entry'),
                      'fold': source_edge('invocation_scope', scope='counter_fold'),
                      'comparison': source_edge('invocation_scope', scope='comparator_category_test')}
    body = dict(dispatch_id='body', dispatch_sequence=0, func_id='context-closure',
                scope='context_body_entry', phase='body', dispatch_edge_ref='dispatch',
                causal_edge_ref='body', parent_dispatch_id=None, dispatch_role='entry')
    fold = dict(dispatch_id='fold', dispatch_sequence=1, func_id='callback-wrapper',
                scope='counter_fold', phase='validation', dispatch_edge_ref='dispatch',
                causal_edge_ref='fold', parent_dispatch_id=None, dispatch_role='entry')
    target = dict(fold, dispatch_id='target', dispatch_sequence=2, func_id='callback-target',
                  parent_dispatch_id='fold', dispatch_role='with_target')
    invoked = [dict(dispatch_id=d['dispatch_id'], func_id=d['func_id'], origin='counter_fold',
                    phase='validation') for d in (fold, target)]
    calls = [[body, fold, target], invoked, dispatch_edges]
    add('real-callback-With-body-excluded', contract.check_invocations, calls, 'Preserved')
    for name, field, value in [('wrong-Func', 'func_id', 'invented'),
                               ('wrong-origin', 'origin', 'body'),
                               ('wrong-phase', 'phase', 'body')]:
        a = copy.deepcopy(calls); a[1][0][field] = value
        add(name, contract.check_invocations, a, 'Violated')
    a = copy.deepcopy(calls); a[1].pop()
    add('missing-With-target-callback', contract.check_invocations, a, 'Violated')
    a = copy.deepcopy(calls); a[0][2]['scope'] = 'ordinary'; a[0][2]['causal_edge_ref'] = 'body'
    add('With-fabricates-noncallback-origin', contract.check_invocations, a, 'Violated')
    a = copy.deepcopy(calls); a[0][1]['scope'] = 'comparator_category_test'; a[0][1]['causal_edge_ref'] = 'comparison'
    a[0][1]['phase'] = 'comparison'; a[0] = a[0][:2]
    a[1] = [dict(dispatch_id='fold', func_id='callback-wrapper', origin='comparator_category_test', phase='comparison')]
    add('actual-comparison-invocation', contract.check_invocations, a, 'Violated')

    span = dict(file_id='source', source_sha256='b' * 64, byte_start=12, byte_end=21, raw_span='real-origin')
    origin = dict(origin_id='o', kind='manual', source_edge_ref='origin', span_status='lexical',
                  span_role='arguments', span=span, content_resource_id='constructed')
    occurrence = dict(occurrence_id='occ', snapshot_id='I1', event_index=0, location='loc',
                      producer_generation_id='g', source_edge_ref='walk', origin_id='o',
                      content_resource_id='mapped', derivation_ids=['map'])
    derivation = dict(derivation_id='map', source_edge_ref='map',
                     from_content_resource_id='constructed', to_content_resource_id='mapped')
    event = dict(occurrence_id='occ', location='loc', producer_generation_id='g', span=span)
    edges = {'origin': source_edge('counter_origin:manual'), 'walk': source_edge('counter_occurrence'),
             'map': source_edge('content_derivation')}
    counter = [[{'snapshot_id': 'I1', 'counter_events': [event]}], [origin], [occurrence], [derivation], edges]
    add('actual-span-through-content-map', contract.check_counter_origins, counter, 'Preserved')
    # Detach the event Span to model substitution without changing the origin.
    a = copy.deepcopy(counter); a[0][0]['counter_events'][0]['span'] = dict(span, raw_span='read-span')
    add('read-span-substituted-for-origin', contract.check_counter_origins, a, 'Violated')
    a = copy.deepcopy(counter); a[1][0]['span_status'] = 'detached'
    add('lost-origin-called-synthetic', contract.check_counter_origins, a, 'Violated')
    a = copy.deepcopy(counter); a[2][0]['derivation_ids'] = []
    add('recreated-Heading-without-origin-edge', contract.check_counter_origins, a, 'Violated')
    a = copy.deepcopy(counter); a[2][0]['location'] = 'equal-action-other-location'
    add('equal-action-wrong-occurrence', contract.check_counter_origins, a, 'Violated')
    a = copy.deepcopy(counter); a[2].append(dict(occurrence, occurrence_id='unprojected'))
    add('missing-actual-event-projection', contract.check_counter_origins, a, 'Violated')
    a = copy.deepcopy(counter); del a[1][0]['span']
    add('explicit-opaque-origin-gap', contract.check_counter_origins, a, 'Unknown')
    return cases


if __name__ == '__main__':
    start = time.monotonic()
    rows = []
    for name, function, args, expected in vectors():
        reason = None
        try:
            function(*args); actual = 'Preserved'
        except AssertionError as error:
            actual = 'Violated'; reason = str(error)
        except KeyError as error:
            actual = 'Unknown'; reason = str(error)
        rows.append(dict(id=name, expected=expected, actual=actual, reason=reason))
    print(json.dumps({'scope': 'predicate-unit-only-no-product-credit', 'cases': rows,
                      'seconds': time.monotonic() - start}, indent=2))
    raise SystemExit(0 if all(r['expected'] == r['actual'] for r in rows) else 1)
