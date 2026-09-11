"""Executable R6 lifecycle/profile fragment over passive observations.

This module never executes the product.  It checks causal witnesses emitted by
an independently audited binding.  Source hashes and source-code line numbers
are receipt metadata, not semantic DTO fields.
"""

from collections import Counter


class ContractUnknown(Exception):
    """Explicit opacity in an otherwise well-formed observation."""


def require(condition, reason):
    if not condition:
        raise AssertionError(reason)


def unique(rows, key, reason):
    require(isinstance(rows, list), reason)
    answer = {}
    for row in rows:
        require(isinstance(row, dict) and key in row, reason)
        value = row[key]
        require(isinstance(value, str) and value and value not in answer, reason)
        answer[value] = row
    return answer


def explicit_status(row, label):
    require('status' in row, label + '-status-missing')
    status = row['status']
    if status == 'opaque':
        raise ContractUnknown(label + '-opaque')
    require(status == 'observed', label + '-status-invalid')


def witness(witnesses, witness_id, kind):
    require(isinstance(witness_id, str) and witness_id in witnesses,
            'R6-causal-witness-missing')
    item = witnesses[witness_id]
    require(item.get('kind') == kind, 'R6-causal-witness-kind')
    require(item.get('observation') == 'actual', 'R6-causal-witness-not-actual')
    require(isinstance(item.get('binding_point'), str) and item['binding_point'],
            'R6-causal-binding-point-missing')
    return item


def check_features(events, provenance, witnesses):
    contexts = provenance['feature_contexts']
    by_context = unique(contexts, 'context_id', 'R6-feature-context-identity')
    for context in contexts:
        explicit_status(context, 'R6-feature-context')
        source = context['source']
        witness(witnesses, context['origin_witness_id'], 'feature_origin')
        features = context['features']
        require(isinstance(features, list) and all(isinstance(x, str) for x in features),
                'R6-feature-vector-shape')
        if source == 'eval_context_default':
            require(features == [] and context.get('from_context_id') is None,
                    'R6-real-default-features')
        elif source == 'entrypoint_supplied':
            require(features == context['supplied_features'],
                    'R6-entrypoint-features-not-actual-argument')
            require(context.get('from_context_id') is None,
                    'R6-entrypoint-has-fabricated-parent')
        elif source == 'inherited':
            parent = by_context[context['from_context_id']]
            require(parent['sequence'] < context['sequence'],
                    'R6-feature-parent-not-causal')
            require(features == parent['features'],
                    'R6-inherited-features-not-parent-value')
        else:
            require(False, 'R6-feature-origin-variant')

    bodies = [event for event in events if event.get('kind') == 'BodyStarted']
    requests = [event for event in events if event.get('kind') == 'RequestRecorded']
    by_execution = unique(bodies, 'execution_id', 'R6-feature-body-identity')
    used = []
    for body in bodies:
        context = by_context[body['feature_context_id']]
        used.append(context['context_id'])
        require(body['resources']['features'] == context['features'],
                'R6-body-features-not-created-context')
    for request in requests:
        body = by_execution[request['execution_id']]
        require(request['feature_context_id'] == body['feature_context_id'],
                'R6-request-context-origin-changed')
        require(request['resources']['features'] == body['resources']['features'],
                'R6-request-features-not-body-features')
    require(set(used) == set(by_context), 'R6-unbound-feature-context')


CALLBACK_KINDS = {
    'counter_fold', 'state_display', 'counter_display', 'projection',
    'numbering', 'other_callback',
}


def check_invocations(events, provenance, witnesses):
    identities = provenance['func_identities']
    by_func = unique(identities, 'func_id', 'R6-Func-identity-bijection')
    for identity in identities:
        explicit_status(identity, 'R6-Func-identity')
        require(isinstance(identity['carrier_id'], str) and identity['carrier_id'],
                'R6-Func-carrier-required')
        require(isinstance(identity['producer_generation_id'], str) and
                identity['producer_generation_id'], 'R6-Func-producer-required')
        witness(witnesses, identity['origin_witness_id'], 'func_identity')

    dispatches = provenance['function_dispatches']
    by_dispatch = unique(dispatches, 'dispatch_id', 'R6-dispatch-identity-bijection')
    require([row['dispatch_sequence'] for row in dispatches] == list(range(len(dispatches))),
            'R6-complete-dispatch-order')
    for row in dispatches:
        explicit_status(row, 'R6-dispatch')
        require(row['func_id'] in by_func, 'R6-dispatch-Func-unidentified')
        witness(witnesses, row['callsite_witness_id'], 'invocation_callsite')
        kind = row['invocation_kind']
        callback_kind = row.get('callback_kind')
        if kind == 'context_body_entry':
            require(callback_kind is None and row['dispatch_role'] == 'body_entry',
                    'R6-ContextBlock-entry-is-not-callback')
        elif kind == 'ordinary':
            require(callback_kind is None and row['dispatch_role'] in ('entry', 'with_target'),
                    'R6-ordinary-dispatch-shape')
        elif kind == 'callback':
            require(callback_kind in CALLBACK_KINDS and
                    row['dispatch_role'] in ('entry', 'with_target'),
                    'R6-callback-causal-kind')
        else:
            require(False, 'R6-invocation-kind')
        require(row['phase'] in ('body', 'validation', 'diagnostics'),
                'R6-no-Func-during-comparison')
        parent_id = row.get('parent_dispatch_id')
        if parent_id is not None:
            parent = by_dispatch[parent_id]
            require(parent['dispatch_sequence'] < row['dispatch_sequence'],
                    'R6-dispatch-parent-order')
        if row['dispatch_role'] == 'with_target':
            require(parent_id is not None, 'R6-With-target-without-wrapper')
            parent = by_dispatch[parent_id]
            require((parent['invocation_kind'], parent.get('callback_kind'), parent['phase']) ==
                    (kind, callback_kind, row['phase']),
                    'R6-With-target-lost-causal-origin')

    coverage = provenance['dispatch_coverage']
    explicit_status(coverage, 'R6-dispatch-coverage')
    require(coverage['expected_dispatch_ids'] == [row['dispatch_id'] for row in dispatches],
            'R6-reachable-dispatch-ledger-incomplete')
    require(Counter(coverage['expected_callsite_witness_ids']) ==
            Counter(row['callsite_witness_id'] for row in dispatches),
            'R6-reachable-callsite-coverage-incomplete')

    bodies = [event for event in events if event.get('kind') == 'BodyStarted']
    body_entries = []
    for body in bodies:
        dispatch = by_dispatch[body['entry_dispatch_id']]
        body_entries.append(dispatch['dispatch_id'])
        require(dispatch['invocation_kind'] == 'context_body_entry' and
                dispatch['execution_id'] == body['execution_id'],
                'R6-ContextBlock-body-entry-binding')
    require(len(body_entries) == len(set(body_entries)),
            'R6-ContextBlock-body-entry-bijection')
    require(set(body_entries) == {
        row['dispatch_id'] for row in dispatches
        if row['invocation_kind'] == 'context_body_entry'
    }, 'R6-unbound-ContextBlock-body-entry')

    projected = [row for row in dispatches if row['invocation_kind'] == 'callback']
    invoked = [event for event in events if event.get('kind') == 'FunctionInvoked']
    require([event['dispatch_id'] for event in invoked] ==
            [row['dispatch_id'] for row in projected],
            'R6-callback-transcript-dispatch-bijection')
    for event, row in zip(invoked, projected):
        require(event['func_id'] == row['func_id'], 'R6-invoked-Func-identity')
        require(event['callback_kind'] == row['callback_kind'],
                'R6-invoked-causal-origin')
        require(event['phase'] == row['phase'], 'R6-invoked-phase')
        require(event['callsite_witness_id'] == row['callsite_witness_id'],
                'R6-invoked-callsite-origin')


def lexical_span(span):
    require(isinstance(span, dict), 'R6-lexical-span-shape')
    require(set(span) == {'source_id', 'byte_start', 'byte_end', 'raw_span'},
            'R6-lexical-span-fields')
    require(isinstance(span['source_id'], str) and span['source_id'],
            'R6-lexical-source-identity')
    require(type(span['byte_start']) is int and type(span['byte_end']) is int and
            0 <= span['byte_start'] < span['byte_end'], 'R6-lexical-byte-range')
    require(isinstance(span['raw_span'], str) and span['raw_span'],
            'R6-raw-span-identity')


def check_counter_events(events, provenance, witnesses):
    origins = provenance['counter_origins']
    occurrences = provenance['counter_occurrences']
    lineage = provenance['content_lineage']
    by_origin = unique(origins, 'origin_id', 'R6-counter-origin-identity')
    by_occurrence = unique(occurrences, 'occurrence_id', 'R6-counter-occurrence-identity')
    by_lineage = unique(lineage, 'lineage_id', 'R6-content-lineage-identity')
    for origin in origins:
        explicit_status(origin, 'R6-counter-origin')
        require(origin['origin_kind'] in ('manual', 'automatic'),
                'R6-counter-origin-kind')
        require(origin['origin_status'] == 'lexical',
                'R6-lifecycle-event-requires-lexical-origin')
        witness(witnesses, origin['origin_witness_id'], 'counter_origin')
        lexical_span(origin['span'])
        require(isinstance(origin['source_content_id'], str) and origin['source_content_id'],
                'R6-origin-content-required')

    used = []
    candidates = [event for event in events if event.get('kind') == 'CandidateBuilt']
    for candidate in candidates:
        for event_index, event in enumerate(candidate['counter_events']):
            occurrence = by_occurrence[event['occurrence_id']]
            used.append(event['occurrence_id'])
            explicit_status(occurrence, 'R6-counter-occurrence')
            require((occurrence['snapshot_id'], occurrence['event_index']) ==
                    (candidate['snapshot_id'], event_index),
                    'R6-current-event-occurrence-coordinate')
            require(occurrence['location'] == event['location'],
                    'R6-counter-occurrence-location')
            require(occurrence['producer_generation_id'] == event['producer_generation_id'],
                    'R6-counter-occurrence-producer')
            require(occurrence['walked_content_id'] == event['content_id'],
                    'R6-counter-occurrence-walked-content')
            witness(witnesses, occurrence['occurrence_witness_id'], 'counter_occurrence')
            origin = by_origin[occurrence['origin_id']]
            require(event['span'] == origin['span'], 'R6-counter-event-origin-span')
            content_id = origin['source_content_id']
            seen = set()
            for lineage_id in occurrence['lineage_ids']:
                require(lineage_id not in seen, 'R6-content-lineage-cycle')
                seen.add(lineage_id)
                edge = by_lineage[lineage_id]
                explicit_status(edge, 'R6-content-lineage')
                witness(witnesses, edge['lineage_witness_id'], 'content_lineage')
                require(edge['from_content_id'] == content_id,
                        'R6-content-lineage-discontinuous')
                content_id = edge['to_content_id']
            require(content_id == occurrence['walked_content_id'],
                    'R6-origin-does-not-reach-walked-content')
    require(len(used) == len(set(used)) and set(used) == set(by_occurrence),
            'R6-counter-event-occurrence-bijection')


def check_typed_value(value):
    if value is None or isinstance(value, (bool, int, float, str)):
        return
    if isinstance(value, list):
        for item in value:
            check_typed_value(item)
        return
    require(isinstance(value, dict), 'R6-typed-value-shape')
    require(len(value) == 1, 'R6-typed-value-single-variant')
    variant, payload = next(iter(value.items()))
    if variant == 'Dict':
        require(isinstance(payload, list), 'R6-Dict-ordered-pairs')
        for pair in payload:
            require(isinstance(pair, list) and len(pair) == 2 and
                    isinstance(pair[0], str), 'R6-Dict-exact-string-key-pair')
            check_typed_value(pair[1])
    else:
        require(variant in {
            'Array', 'Args', 'Content', 'Counter', 'Func', 'Label', 'Length',
            'Location', 'Module', 'Selector', 'State', 'Type',
        }, 'R6-typed-value-unsupported-variant')
        check_typed_value(payload)


def check_dicts(provenance):
    coverage = provenance['dict_coverage']
    explicit_status(coverage, 'R6-Dict-coverage')
    values = provenance['typed_dict_values']
    by_id = unique(values, 'value_id', 'R6-Dict-value-identity')
    require(coverage['expected_value_ids'] == [row['value_id'] for row in values],
            'R6-Dict-coverage-incomplete')
    for row in values:
        explicit_status(row, 'R6-Dict-value')
        value = row['value']
        require(isinstance(value, dict) and set(value) == {'Dict'},
                'R6-Dict-must-be-injectively-tagged')
        require([pair[0] for pair in value['Dict']] == row['entry_key_order'],
                'R6-Dict-entry-order-not-preserved')
        check_typed_value(value)
    require(set(by_id) == set(coverage['expected_value_ids']),
            'R6-Dict-coverage-bijection')


def check(case, observed):
    provenance = observed['r6_provenance']
    require(provenance['schema'] == 'p1340-lifecycle-provenance-r6',
            'R6-provenance-schema')
    explicit_status(provenance, 'R6-provenance')
    witnesses = provenance['causal_witnesses']
    require(isinstance(witnesses, dict) and witnesses,
            'R6-causal-witness-ledger-required')
    check_features(observed['events'], provenance, witnesses)
    check_invocations(observed['events'], provenance, witnesses)
    check_counter_events(observed['events'], provenance, witnesses)
    check_dicts(provenance)
    return {'case_id': case['id'], 'r6_fragment': 'Preserved'}


def classify(case, observed):
    try:
        return {'classification': 'Preserved', 'detail': check(case, observed)}
    except ContractUnknown as error:
        return {'classification': 'Unknown', 'reason_code': str(error)}
    except (AssertionError, KeyError, TypeError, IndexError) as error:
        return {'classification': 'Violated', 'reason_code': str(error)}
