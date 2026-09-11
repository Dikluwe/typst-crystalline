"""R6b structural preflight followed by the frozen R6 semantic checker.

Opacity is a classification only after every mandatory ledger field is known
to be structurally present and typed.  Malformed plus opaque is Violated.
"""

import importlib.util
from pathlib import Path


path = Path(__file__).with_name('p1340-contract-observability-r6.py')
spec = importlib.util.spec_from_file_location('p1340_observability_r6_frozen', path)
frozen = importlib.util.module_from_spec(spec)
spec.loader.exec_module(frozen)


def require(condition, reason):
    if not condition:
        raise AssertionError(reason)


def fields(row, names, label):
    require(isinstance(row, dict), label + '-object')
    require(set(names) <= set(row), label + '-mandatory-fields')


def identity(value, label):
    require(isinstance(value, str) and value, label)


def status(row, label):
    require(row['status'] in ('observed', 'opaque'), label + '-status')


def id_list(value, label):
    require(isinstance(value, list) and
            all(isinstance(item, str) and item for item in value), label)


def span_shape(span):
    fields(span, ('source_id', 'byte_start', 'byte_end', 'raw_span'),
           'R6b-span')
    require(set(span) == {'source_id', 'byte_start', 'byte_end', 'raw_span'},
            'R6b-span-fields')
    identity(span['source_id'], 'R6b-span-source-id')
    require(type(span['byte_start']) is int and type(span['byte_end']) is int,
            'R6b-span-byte-types')
    identity(span['raw_span'], 'R6b-span-raw-identity')


def expectation_preflight(case):
    fields(case, ('id', 'r6_expectations'), 'R6b-case')
    expected = case['r6_expectations']
    fields(expected, ('status', 'authority_sha256', 'callback_dispatches',
                      'dict_values', 'counter_events'), 'R6b-expectations')
    require(expected['status'] == 'frozen', 'R6b-expectations-not-frozen')
    require(isinstance(expected['authority_sha256'], str) and
            len(expected['authority_sha256']) == 64,
            'R6b-expectations-authority-pin')
    require(isinstance(expected['callback_dispatches'], list),
            'R6b-expected-callback-list')
    for row in expected['callback_dispatches']:
        fields(row, ('callsite_witness_id', 'callback_kind', 'phase',
                     'dispatch_role', 'count'), 'R6b-expected-callback')
        for key in ('callsite_witness_id', 'callback_kind', 'phase', 'dispatch_role'):
            identity(row[key], 'R6b-expected-callback-' + key)
        require(type(row['count']) is int and row['count'] > 0,
                'R6b-expected-callback-count')
    require(isinstance(expected['dict_values'], list), 'R6b-expected-Dict-list')
    for row in expected['dict_values']:
        fields(row, ('value_id', 'entry_key_order'), 'R6b-expected-Dict')
        identity(row['value_id'], 'R6b-expected-Dict-id')
        id_list(row['entry_key_order'], 'R6b-expected-Dict-key-order')
    require(isinstance(expected['counter_events'], list),
            'R6b-expected-counter-event-list')
    for row in expected['counter_events']:
        fields(row, ('snapshot_id', 'event_index', 'location', 'origin_kind',
                     'span'), 'R6b-expected-counter-event')
        for key in ('snapshot_id', 'location', 'origin_kind'):
            identity(row[key], 'R6b-expected-counter-' + key)
        require(type(row['event_index']) is int, 'R6b-expected-event-index')
        span_shape(row['span'])


def structural_preflight(case, observed):
    expectation_preflight(case)
    fields(observed, ('events', 'r6_provenance'), 'R6b-observation')
    require(isinstance(observed['events'], list), 'R6b-events-list')
    p = observed['r6_provenance']
    fields(p, ('schema', 'status', 'causal_witnesses', 'feature_contexts',
               'func_identities', 'function_dispatches', 'dispatch_coverage',
               'counter_origins', 'counter_occurrences', 'content_lineage',
               'typed_dict_values', 'dict_coverage'), 'R6b-provenance')
    status(p, 'R6b-provenance')

    witnesses = p['causal_witnesses']
    require(isinstance(witnesses, dict), 'R6b-witness-ledger-object')
    for witness_id, item in witnesses.items():
        identity(witness_id, 'R6b-witness-id')
        fields(item, ('kind', 'observation', 'binding_point'), 'R6b-witness')
        identity(item['kind'], 'R6b-witness-kind')
        identity(item['observation'], 'R6b-witness-observation')
        identity(item['binding_point'], 'R6b-witness-binding-point')

    require(isinstance(p['feature_contexts'], list), 'R6b-feature-contexts-list')
    for row in p['feature_contexts']:
        fields(row, ('context_id', 'sequence', 'status', 'source',
                     'from_context_id', 'features', 'origin_witness_id'),
               'R6b-feature-context')
        status(row, 'R6b-feature-context')
        identity(row['context_id'], 'R6b-feature-context-id')
        require(type(row['sequence']) is int, 'R6b-feature-sequence')
        require(row['source'] in ('eval_context_default', 'entrypoint_supplied',
                                  'inherited'), 'R6b-feature-source')
        require(isinstance(row['features'], list) and
                all(isinstance(item, str) for item in row['features']),
                'R6b-feature-vector')
        identity(row['origin_witness_id'], 'R6b-feature-witness-id')
        if row['source'] == 'entrypoint_supplied':
            fields(row, ('supplied_features',), 'R6b-feature-entrypoint')
            require(isinstance(row['supplied_features'], list),
                    'R6b-feature-supplied-vector')
        if row['source'] == 'inherited':
            identity(row['from_context_id'], 'R6b-feature-parent-id')

    require(isinstance(p['func_identities'], list), 'R6b-Func-identities-list')
    for row in p['func_identities']:
        fields(row, ('func_id', 'carrier_id', 'producer_generation_id',
                     'origin_witness_id', 'status'), 'R6b-Func-identity')
        status(row, 'R6b-Func-identity')
        for key in ('func_id', 'carrier_id', 'producer_generation_id',
                    'origin_witness_id'):
            identity(row[key], 'R6b-Func-' + key)

    require(isinstance(p['function_dispatches'], list), 'R6b-dispatches-list')
    for row in p['function_dispatches']:
        fields(row, ('dispatch_id', 'dispatch_sequence', 'func_id',
                     'invocation_kind', 'callback_kind', 'phase', 'dispatch_role',
                     'parent_dispatch_id', 'callsite_witness_id', 'status'),
               'R6b-dispatch')
        status(row, 'R6b-dispatch')
        for key in ('dispatch_id', 'func_id', 'invocation_kind', 'phase',
                    'dispatch_role', 'callsite_witness_id'):
            identity(row[key], 'R6b-dispatch-' + key)
        require(type(row['dispatch_sequence']) is int, 'R6b-dispatch-sequence')
        if row['invocation_kind'] == 'context_body_entry':
            fields(row, ('execution_id',), 'R6b-body-entry-dispatch')
            identity(row['execution_id'], 'R6b-body-entry-execution-id')

    coverage = p['dispatch_coverage']
    fields(coverage, ('status', 'expected_dispatch_ids',
                      'expected_callsite_witness_ids'), 'R6b-dispatch-coverage')
    status(coverage, 'R6b-dispatch-coverage')
    id_list(coverage['expected_dispatch_ids'], 'R6b-expected-dispatch-ids')
    id_list(coverage['expected_callsite_witness_ids'],
            'R6b-expected-callsite-witness-ids')

    require(isinstance(p['counter_origins'], list), 'R6b-counter-origins-list')
    for row in p['counter_origins']:
        fields(row, ('origin_id', 'origin_kind', 'origin_status',
                     'source_content_id', 'span', 'origin_witness_id', 'status'),
               'R6b-counter-origin')
        status(row, 'R6b-counter-origin')
        for key in ('origin_id', 'origin_kind', 'origin_status',
                    'source_content_id', 'origin_witness_id'):
            identity(row[key], 'R6b-counter-origin-' + key)
        span_shape(row['span'])

    require(isinstance(p['counter_occurrences'], list),
            'R6b-counter-occurrences-list')
    for row in p['counter_occurrences']:
        fields(row, ('occurrence_id', 'snapshot_id', 'event_index', 'location',
                     'producer_generation_id', 'origin_id', 'walked_content_id',
                     'lineage_ids', 'occurrence_witness_id', 'status'),
               'R6b-counter-occurrence')
        status(row, 'R6b-counter-occurrence')
        for key in ('occurrence_id', 'snapshot_id', 'location',
                    'producer_generation_id', 'origin_id', 'walked_content_id',
                    'occurrence_witness_id'):
            identity(row[key], 'R6b-counter-occurrence-' + key)
        require(type(row['event_index']) is int, 'R6b-event-index')
        id_list(row['lineage_ids'], 'R6b-lineage-ids')

    require(isinstance(p['content_lineage'], list), 'R6b-content-lineage-list')
    for row in p['content_lineage']:
        fields(row, ('lineage_id', 'from_content_id', 'to_content_id',
                     'lineage_witness_id', 'status'), 'R6b-content-lineage')
        status(row, 'R6b-content-lineage')
        for key in ('lineage_id', 'from_content_id', 'to_content_id',
                    'lineage_witness_id'):
            identity(row[key], 'R6b-content-lineage-' + key)

    require(isinstance(p['typed_dict_values'], list), 'R6b-Dict-values-list')
    for row in p['typed_dict_values']:
        fields(row, ('value_id', 'entry_key_order', 'value', 'status'),
               'R6b-Dict-value')
        status(row, 'R6b-Dict-value')
        identity(row['value_id'], 'R6b-Dict-value-id')
        id_list(row['entry_key_order'], 'R6b-Dict-key-order')
        frozen.check_typed_value(row['value'])

    dict_coverage = p['dict_coverage']
    fields(dict_coverage, ('status', 'expected_value_ids'), 'R6b-Dict-coverage')
    status(dict_coverage, 'R6b-Dict-coverage')
    id_list(dict_coverage['expected_value_ids'], 'R6b-Dict-expected-value-ids')


def external_coverage(case, observed):
    expected = case['r6_expectations']
    p = observed['r6_provenance']
    actual_callbacks = [
        (row['callsite_witness_id'], row['callback_kind'], row['phase'],
         row['dispatch_role'])
        for row in p['function_dispatches']
        if row['invocation_kind'] == 'callback'
    ]
    expected_callbacks = []
    for row in expected['callback_dispatches']:
        expected_callbacks.extend([
            (row['callsite_witness_id'], row['callback_kind'], row['phase'],
             row['dispatch_role'])
        ] * row['count'])
    require(sorted(actual_callbacks) == sorted(expected_callbacks),
            'R6b-external-callback-coverage')

    actual_dicts = [(row['value_id'], row['entry_key_order'])
                    for row in p['typed_dict_values']]
    expected_dicts = [(row['value_id'], row['entry_key_order'])
                      for row in expected['dict_values']]
    require(actual_dicts == expected_dicts, 'R6b-external-Dict-coverage')

    origins = {row['origin_id']: row for row in p['counter_origins']}
    occurrences = {row['occurrence_id']: row for row in p['counter_occurrences']}
    actual_events = []
    for candidate in [event for event in observed['events']
                      if event.get('kind') == 'CandidateBuilt']:
        for event_index, event in enumerate(candidate['counter_events']):
            occurrence = occurrences[event['occurrence_id']]
            origin = origins[occurrence['origin_id']]
            actual_events.append({
                'snapshot_id': candidate['snapshot_id'],
                'event_index': event_index,
                'location': event['location'],
                'origin_kind': origin['origin_kind'],
                'span': origin['span'],
            })
    require(actual_events == expected['counter_events'],
            'R6b-external-counter-event-coverage')


def check(case, observed):
    structural_preflight(case, observed)
    answer = frozen.check(case, observed)
    external_coverage(case, observed)
    return answer


def classify(case, observed):
    try:
        structural_preflight(case, observed)
    except (AssertionError, KeyError, TypeError, IndexError) as error:
        return {'classification': 'Violated',
                'reason_code': 'R6b-malformed-ledger:' + str(error)}
    answer = frozen.classify(case, observed)
    if answer['classification'] != 'Preserved':
        return answer
    try:
        external_coverage(case, observed)
    except (AssertionError, KeyError, TypeError, IndexError) as error:
        return {'classification': 'Violated',
                'reason_code': str(error)}
    return answer
