"""R4 passive DTO assertions. Source-path audit remains a mandatory conjunct.

No product code, callback dispatch, span reconstruction or feature correction.
"""
PROFILES = {'default': [], 'html': ['html'], 'a11y': ['a11y-extras'],
            'html+a11y': ['html', 'a11y-extras']}
SCOPES = {'ordinary', 'context_body_entry', 'counter_fold', 'display_callback',
          'projection', 'comparator_category_test', 'other_callback'}


def require(value, reason):
    if not value:
        raise AssertionError(reason)


def unique(rows, key, reason):
    answer = {r[key]: r for r in rows}
    require(len(answer) == len(rows), reason)
    require(all(isinstance(k, str) and k for k in answer), reason)
    return answer


def edge(edges, ref, kind):
    item = edges[ref]
    require(item['kind'] == kind, 'R4-source-edge-kind')
    require(isinstance(item['path'], str) and item['path'], 'R4-source-edge-path')
    require(isinstance(item['sha256'], str) and len(item['sha256']) == 64,
            'R4-source-edge-pin')
    require(type(item['line']) is int and item['line'] > 0, 'R4-source-edge-line')
    return item


def check_features(profile, bodies, requests, edges):
    by_id = unique(bodies, 'execution_id', 'R4-feature-execution-bijection')
    for body in bodies:
        origin = body['feature_origin']
        mode = origin['mode']
        edge(edges, origin['source_edge_ref'], 'features:' + mode)
        if mode == 'ordinary_default':
            expected = []
        elif mode == 'selected_profile':
            require(body['attempt'] > 0, 'R4-selected-profile-not-ordinary-discovery')
            expected = PROFILES[profile]
        elif mode == 'inherited':
            parent = by_id[origin['from_execution_id']]
            require(parent['sequence'] < body['sequence'], 'R4-feature-causal-parent-order')
            require(body['parent_generation_id'] == parent['generation_id'],
                    'R4-feature-causal-parent-generation')
            expected = parent['resources']['features']
        else:
            require(False, 'R4-unknown-feature-origin')
        if body['attempt'] == 0:
            require(mode != 'selected_profile' and expected == [],
                    'R4-ordinary-discovery-keeps-legacy-default')
        require(body['resources']['features'] == expected, 'R4-real-features-from-actual-origin')
    for request in requests:
        require(request['resources']['features'] ==
                by_id[request['execution_id']]['resources']['features'],
                'R4-request-keeps-real-body-features')


def check_invocations(dispatches, invoked, edges):
    by_id = unique(dispatches, 'dispatch_id', 'R4-actual-dispatch-identity')
    require([d['dispatch_sequence'] for d in dispatches] == list(range(len(dispatches))),
            'R4-complete-dispatch-order')
    for d in dispatches:
        require(isinstance(d['func_id'], str) and d['func_id'], 'R4-actual-Func-required')
        require(d['scope'] in SCOPES, 'R4-actual-invocation-scope')
        require(d['phase'] in ('body', 'validation', 'diagnostics', 'comparison'),
                'R4-actual-invocation-phase')
        edge(edges, d['dispatch_edge_ref'], 'func_dispatch')
        causal = edge(edges, d['causal_edge_ref'], 'invocation_scope')
        require(causal['scope'] == d['scope'], 'R4-scope-matches-audited-causal-edge')
        if d['parent_dispatch_id'] is not None:
            parent = by_id[d['parent_dispatch_id']]
            require(parent['dispatch_sequence'] < d['dispatch_sequence'],
                    'R4-dispatch-parent-precedes-child')
        require(d['dispatch_role'] in ('entry', 'with_target'), 'R4-real-dispatch-role')
        if d['dispatch_role'] == 'with_target':
            require(d['parent_dispatch_id'] is not None, 'R4-With-has-actual-wrapper')
            parent = by_id[d['parent_dispatch_id']]
            require((parent['scope'], parent['phase']) == (d['scope'], d['phase']),
                    'R4-With-preserves-causal-scope')
    relevant = [d for d in dispatches if d['scope'] not in ('ordinary', 'context_body_entry')]
    require([e['dispatch_id'] for e in invoked] == [d['dispatch_id'] for d in relevant],
            'R4-callback-dispatch-transcript-bijection')
    for event, dispatch in zip(invoked, relevant):
        expected_origin = ('other_actual_origin' if dispatch['scope'] == 'other_callback'
                           else dispatch['scope'])
        require(event['func_id'] == dispatch['func_id'], 'R4-invoked-real-Func-identity')
        require(event['origin'] == expected_origin, 'R4-invoked-real-causal-origin')
        require(event['phase'] == dispatch['phase'], 'R4-invoked-real-phase')
    require(not any(d['scope'] == 'comparator_category_test' or d['phase'] == 'comparison'
                    for d in dispatches), 'R4-no-Func-during-comparison')


def lexical_span(span):
    require(isinstance(span, dict), 'R4-lexical-Span-required')
    require(set(('file_id', 'source_sha256', 'byte_start', 'byte_end', 'raw_span')) <= set(span),
            'R4-complete-lexical-Span')
    require(isinstance(span['source_sha256'], str) and len(span['source_sha256']) == 64,
            'R4-lexical-Source-pin')
    require(type(span['byte_start']) is int and type(span['byte_end']) is int and
            0 <= span['byte_start'] < span['byte_end'], 'R4-actual-lexical-byte-range')
    require(bool(span['file_id']) and bool(span['raw_span']), 'R4-actual-lexical-Span-identity')


def check_counter_origins(candidates, origins, occurrences, derivations, edges):
    by_origin = unique(origins, 'origin_id', 'R4-source-origin-identities')
    by_occurrence = unique(occurrences, 'occurrence_id', 'R4-occurrence-identities')
    by_derivation = unique(derivations, 'derivation_id', 'R4-derivation-identities')
    used = []
    for candidate in candidates:
        for index, event in enumerate(candidate['counter_events']):
            occurrence = by_occurrence[event['occurrence_id']]
            used.append(event['occurrence_id'])
            require(occurrence['snapshot_id'] == candidate['snapshot_id'] and
                    occurrence['event_index'] == index, 'R4-exact-current-event-occurrence')
            require(occurrence['location'] == event['location'], 'R4-event-real-Location')
            require(occurrence['producer_generation_id'] == event['producer_generation_id'],
                    'R4-event-real-producer')
            edge(edges, occurrence['source_edge_ref'], 'counter_occurrence')
            origin = by_origin[occurrence['origin_id']]
            require(origin['kind'] in ('manual', 'automatic'), 'R4-actual-counter-origin-kind')
            edge(edges, origin['source_edge_ref'], 'counter_origin:' + origin['kind'])
            require(origin['span_status'] == 'lexical', 'R4-required-lexical-origin-not-lost')
            require(origin['span_role'] in ('call', 'arguments', 'markup', 'element_source'),
                    'R4-origin-span-role-not-read-or-root')
            lexical_span(origin['span'])
            require(event['span'] == origin['span'], 'R4-event-keeps-exact-origin-Span')
            content_id = origin['content_resource_id']
            require(isinstance(content_id, str) and content_id, 'R4-real-origin-Content')
            seen = set()
            for derivation_id in occurrence['derivation_ids']:
                require(derivation_id not in seen, 'R4-origin-derivation-no-cycle')
                seen.add(derivation_id)
                derivation = by_derivation[derivation_id]
                edge(edges, derivation['source_edge_ref'], 'content_derivation')
                require(derivation['from_content_resource_id'] == content_id,
                        'R4-origin-continuous-actual-Content-chain')
                content_id = derivation['to_content_resource_id']
            require(content_id == occurrence['content_resource_id'],
                    'R4-origin-reaches-actual-walked-Content')
    require(len(used) == len(set(used)) and set(used) == set(by_occurrence),
            'R4-counter-event-occurrence-bijection')


def check(observed):
    events = observed['events']
    rows = lambda kind: [e for e in events if e['kind'] == kind]
    provenance = observed['r4_provenance']
    edges = provenance['audited_edges']
    require(isinstance(edges, dict) and edges, 'R4-audited-source-edges-required')
    check_features(observed['profile'], rows('BodyStarted'), rows('RequestRecorded'), edges)
    check_invocations(provenance['function_dispatches'], rows('FunctionInvoked'), edges)
    check_counter_origins(rows('CandidateBuilt'), provenance['counter_origins'],
                          provenance['counter_occurrences'], provenance['content_derivations'], edges)
