"""Synthetic contract-unit oracle for R6; never candidate or product evidence."""

import copy
import importlib.util
import json
import time
from pathlib import Path


path = Path(__file__).with_name('p1340-contract-observability-r6.py')
spec = importlib.util.spec_from_file_location('r6', path)
r6 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r6)


def base():
    span = {'source_id': 'fixture.typ', 'byte_start': 4, 'byte_end': 15,
            'raw_span': 'span:update:1'}
    events = [
        {'kind': 'BodyStarted', 'execution_id': 'body-default',
         'entry_dispatch_id': 'body-entry-default',
         'feature_context_id': 'ctx-default', 'resources': {'features': []}},
        {'kind': 'BodyStarted', 'execution_id': 'body-selected',
         'entry_dispatch_id': 'body-entry-selected',
         'feature_context_id': 'ctx-selected', 'resources': {'features': ['html']}},
        {'kind': 'RequestRecorded', 'execution_id': 'body-selected',
         'feature_context_id': 'ctx-selected', 'resources': {'features': ['html']}},
        {'kind': 'FunctionInvoked', 'dispatch_id': 'callback', 'func_id': 'func-callback',
         'callback_kind': 'counter_fold', 'phase': 'validation',
         'callsite_witness_id': 'call-callback'},
        {'kind': 'FunctionInvoked', 'dispatch_id': 'callback-target',
         'func_id': 'func-target', 'callback_kind': 'counter_fold',
         'phase': 'validation', 'callsite_witness_id': 'call-target'},
        {'kind': 'CandidateBuilt', 'snapshot_id': 'I1', 'counter_events': [
            {'occurrence_id': 'occ-1', 'location': 'loc-1',
             'producer_generation_id': 'gen-1', 'content_id': 'content-walked',
             'span': dict(span)},
        ]},
    ]
    witnesses = {
        'feature-default': {'kind': 'feature_origin', 'observation': 'actual',
                            'binding_point': 'EvalContext::new'},
        'feature-entry': {'kind': 'feature_origin', 'observation': 'actual',
                          'binding_point': 'entrypoint supplied Features'},
        'func-body': {'kind': 'func_identity', 'observation': 'actual',
                      'binding_point': 'ContextBlock carrier'},
        'func-callback': {'kind': 'func_identity', 'observation': 'actual',
                          'binding_point': 'counter action carrier'},
        'func-target': {'kind': 'func_identity', 'observation': 'actual',
                        'binding_point': 'With target carrier'},
        'call-body': {'kind': 'invocation_callsite', 'observation': 'actual',
                      'binding_point': 'ContextBlock body entry'},
        'call-callback': {'kind': 'invocation_callsite', 'observation': 'actual',
                          'binding_point': 'counter fold callback'},
        'call-target': {'kind': 'invocation_callsite', 'observation': 'actual',
                        'binding_point': 'With recursive target'},
        'counter-origin': {'kind': 'counter_origin', 'observation': 'actual',
                           'binding_point': 'counter update construction'},
        'counter-walk': {'kind': 'counter_occurrence', 'observation': 'actual',
                         'binding_point': 'walked Content and Location'},
        'content-map': {'kind': 'content_lineage', 'observation': 'actual',
                        'binding_point': 'Content transform'},
    }
    provenance = {
        'schema': 'p1340-lifecycle-provenance-r6', 'status': 'observed',
        'causal_witnesses': witnesses,
        'feature_contexts': [
            {'context_id': 'ctx-default', 'sequence': 0, 'status': 'observed',
             'source': 'eval_context_default', 'from_context_id': None, 'features': [],
             'origin_witness_id': 'feature-default'},
            {'context_id': 'ctx-selected', 'sequence': 1, 'status': 'observed',
             'source': 'entrypoint_supplied', 'from_context_id': None,
             'supplied_features': ['html'], 'features': ['html'],
             'origin_witness_id': 'feature-entry'},
        ],
        'func_identities': [
            {'func_id': 'func-body', 'carrier_id': 'carrier-body',
             'producer_generation_id': 'gen-body', 'status': 'observed',
             'origin_witness_id': 'func-body'},
            {'func_id': 'func-callback', 'carrier_id': 'carrier-callback',
             'producer_generation_id': 'gen-callback', 'status': 'observed',
             'origin_witness_id': 'func-callback'},
            {'func_id': 'func-target', 'carrier_id': 'carrier-target',
             'producer_generation_id': 'gen-callback', 'status': 'observed',
             'origin_witness_id': 'func-target'},
        ],
        'function_dispatches': [
            {'dispatch_id': 'body-entry-default', 'dispatch_sequence': 0,
             'func_id': 'func-body', 'invocation_kind': 'context_body_entry',
             'callback_kind': None, 'phase': 'body', 'dispatch_role': 'body_entry',
             'execution_id': 'body-default',
             'parent_dispatch_id': None, 'callsite_witness_id': 'call-body',
             'status': 'observed'},
            {'dispatch_id': 'body-entry-selected', 'dispatch_sequence': 1,
             'func_id': 'func-body', 'invocation_kind': 'context_body_entry',
             'callback_kind': None, 'phase': 'body', 'dispatch_role': 'body_entry',
             'execution_id': 'body-selected',
             'parent_dispatch_id': None, 'callsite_witness_id': 'call-body',
             'status': 'observed'},
            {'dispatch_id': 'callback', 'dispatch_sequence': 2,
             'func_id': 'func-callback', 'invocation_kind': 'callback',
             'callback_kind': 'counter_fold', 'phase': 'validation',
             'dispatch_role': 'entry', 'parent_dispatch_id': None,
             'callsite_witness_id': 'call-callback', 'status': 'observed'},
            {'dispatch_id': 'callback-target', 'dispatch_sequence': 3,
             'func_id': 'func-target', 'invocation_kind': 'callback',
             'callback_kind': 'counter_fold', 'phase': 'validation',
             'dispatch_role': 'with_target', 'parent_dispatch_id': 'callback',
             'callsite_witness_id': 'call-target', 'status': 'observed'},
        ],
        'dispatch_coverage': {
            'status': 'observed',
            'expected_dispatch_ids': ['body-entry-default', 'body-entry-selected',
                                      'callback', 'callback-target'],
            'expected_callsite_witness_ids': ['call-body', 'call-body',
                                              'call-callback', 'call-target'],
        },
        'counter_origins': [
            {'origin_id': 'origin-1', 'origin_kind': 'manual',
             'origin_status': 'lexical', 'source_content_id': 'content-source',
             'span': span, 'origin_witness_id': 'counter-origin', 'status': 'observed'},
        ],
        'counter_occurrences': [
            {'occurrence_id': 'occ-1', 'snapshot_id': 'I1', 'event_index': 0,
             'location': 'loc-1', 'producer_generation_id': 'gen-1',
             'origin_id': 'origin-1', 'walked_content_id': 'content-walked',
             'lineage_ids': ['lineage-1'], 'occurrence_witness_id': 'counter-walk',
             'status': 'observed'},
        ],
        'content_lineage': [
            {'lineage_id': 'lineage-1', 'from_content_id': 'content-source',
             'to_content_id': 'content-walked', 'lineage_witness_id': 'content-map',
             'status': 'observed'},
        ],
        'typed_dict_values': [
            {'value_id': 'dict-func', 'entry_key_order': ['Func'],
             'value': {'Dict': [['Func', 'runtime-identity']]}, 'status': 'observed'},
            {'value_id': 'dict-recursive', 'entry_key_order': ['Dict'],
             'value': {'Dict': [['Dict', {'Dict': [['Length',
                 {'Length': {'Dict': [['abs_pt_bits', '4024000000000000'],
                                     ['em_bits', '0000000000000000']]}}]]}]]},
             'status': 'observed'},
        ],
        'dict_coverage': {'status': 'observed',
                          'expected_value_ids': ['dict-func', 'dict-recursive']},
    }
    return {'id': 'synthetic-r6'}, {'events': events, 'r6_provenance': provenance}


def no_callback_case():
    case, observed = base()
    observed['events'] = [event for event in observed['events']
                          if event.get('kind') != 'FunctionInvoked']
    provenance = observed['r6_provenance']
    provenance['function_dispatches'] = provenance['function_dispatches'][:1]
    observed['events'] = [event for event in observed['events']
                          if not (event.get('kind') == 'BodyStarted' and
                                  event['execution_id'] == 'body-selected') and
                          event.get('kind') != 'RequestRecorded']
    provenance['feature_contexts'] = provenance['feature_contexts'][:1]
    provenance['dispatch_coverage']['expected_dispatch_ids'] = ['body-entry-default']
    provenance['dispatch_coverage']['expected_callsite_witness_ids'] = ['call-body']
    provenance['func_identities'] = provenance['func_identities'][:1]
    return case, observed


def vectors():
    rows = []

    def add(name, expected, mutate=None, factory=base):
        case, observed = factory()
        if mutate:
            mutate(observed)
        rows.append((name, expected, case, observed))

    add('positive-complete-causal-witnesses', 'Preserved')
    add('context-body-entry-without-callback-is-valid', 'Preserved', factory=no_callback_case)
    add('features-requested-instead-of-actual', 'Violated',
        lambda o: o['events'][0]['resources'].__setitem__('features', ['html']))
    add('feature-origin-explicitly-opaque', 'Unknown',
        lambda o: o['r6_provenance']['feature_contexts'][0].__setitem__('status', 'opaque'))
    add('wrong-Func-identity', 'Violated',
        lambda o: o['events'][3].__setitem__('func_id', 'func-body'))
    add('missing-callback-projection', 'Violated', lambda o: o['events'].pop(4))
    add('ContextBlock-entry-misclassified-as-callback', 'Violated',
        lambda o: o['r6_provenance']['function_dispatches'][0].__setitem__(
            'invocation_kind', 'callback'))
    add('comparison-dispatch-is-forbidden', 'Violated',
        lambda o: o['r6_provenance']['function_dispatches'][1].__setitem__('phase', 'comparison'))
    add('Func-origin-explicitly-opaque', 'Unknown',
        lambda o: o['r6_provenance']['func_identities'][1].__setitem__('status', 'opaque'))
    add('read-span-substituted-for-counter-origin', 'Violated',
        lambda o: o['events'][5]['counter_events'][0]['span'].__setitem__(
            'raw_span', 'span:read:wrong'))
    add('equal-action-wrong-occurrence', 'Violated',
        lambda o: o['r6_provenance']['counter_occurrences'][0].__setitem__(
            'location', 'loc-other'))
    add('counter-origin-explicitly-opaque', 'Unknown',
        lambda o: o['r6_provenance']['counter_origins'][0].__setitem__('status', 'opaque'))
    add('untagged-Dict-collides-with-Func', 'Violated',
        lambda o: o['r6_provenance']['typed_dict_values'][0].__setitem__(
            'value', {'Func': 'runtime-identity'}))
    add('nested-Dict-wrapper-removed', 'Violated',
        lambda o: o['r6_provenance']['typed_dict_values'][1]['value']['Dict'][0].__setitem__(
            1, {'Length': {'abs_pt_bits': '4024000000000000',
                           'em_bits': '0000000000000000'}}))
    add('Dict-entry-order-changed', 'Violated',
        lambda o: o['r6_provenance']['typed_dict_values'][1]['value']['Dict'].append(
            ['extra', 1]))
    add('Dict-coverage-explicitly-opaque', 'Unknown',
        lambda o: o['r6_provenance']['dict_coverage'].__setitem__('status', 'opaque'))
    return rows


def main():
    start = time.monotonic()
    results = []
    for name, expected, case, observed in vectors():
        result = r6.classify(case, observed)
        results.append({'id': name, 'expected': expected,
                        'actual': result['classification'],
                        'reason_code': result.get('reason_code')})
    payload = {
        'schema': 'p1340-oracle-focal-r6-result',
        'scope': 'synthetic-contract-unit-only-no-product-credit',
        'cases': results,
        'counts': {
            'preserved': sum(row['actual'] == 'Preserved' for row in results),
            'violated': sum(row['actual'] == 'Violated' for row in results),
            'unknown': sum(row['actual'] == 'Unknown' for row in results),
        },
        'seconds': time.monotonic() - start,
    }
    print(json.dumps(payload, indent=2, ensure_ascii=True))
    return 0 if all(row['expected'] == row['actual'] for row in results) else 1


if __name__ == '__main__':
    raise SystemExit(main())
