"""Independent synthetic adversary for the published P1340 R6b contract.

No product, candidate source, adapter, output, receipt, or diff is read or
executed.  Every run creates a fresh synthetic fixture and mutates only the
contract input in memory.
"""

import copy
import importlib.util
import json
import time
from pathlib import Path


HERE = Path(__file__).parent


def load(name, filename):
    spec = importlib.util.spec_from_file_location(name, HERE / filename)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


inheritance = load('p1340_adversary_inheritance_r6b',
                   'p1340-oracle-inheritance-focal-r6b.py')
policy = load('p1340_adversary_policy_r6b',
              'p1340-oracle-policy-coverage-r6b.py')


def dispatch(observed, dispatch_id):
    return next(row for row in observed['r6_provenance']['function_dispatches']
                if row['dispatch_id'] == dispatch_id)


def invoked(observed, dispatch_id):
    return next(row for row in observed['events']
                if row.get('kind') == 'FunctionInvoked' and
                row['dispatch_id'] == dispatch_id)


def candidate_event(observed):
    candidate = next(row for row in observed['events']
                     if row.get('kind') == 'CandidateBuilt')
    return candidate['counter_events'][0]


def selected_feature_context(observed):
    return next(row for row in observed['r6_provenance']['feature_contexts']
                if row['context_id'] == 'ctx-selected')


def feature_coordinated_substitution(observed):
    context = selected_feature_context(observed)
    context['features'] = ['a11y-extras']
    context['supplied_features'] = ['a11y-extras']
    for row in observed['events']:
        if row.get('feature_context_id') == 'ctx-selected':
            row['resources']['features'] = ['a11y-extras']


def feature_origin_relabel(observed):
    context = observed['r6_provenance']['feature_contexts'][0]
    context['source'] = 'entrypoint_supplied'
    context['supplied_features'] = []


def feature_fictitious_inheritance(observed):
    context = selected_feature_context(observed)
    context['source'] = 'inherited'
    context['from_context_id'] = 'ctx-default'
    context['features'] = []
    context.pop('supplied_features', None)
    for row in observed['events']:
        if row.get('feature_context_id') == 'ctx-selected':
            row['resources']['features'] = []


def feature_witness_alias(observed):
    selected_feature_context(observed)['origin_witness_id'] = 'feature-default'


def body_entry_uses_callback_func(observed):
    dispatch(observed, 'body-entry-default')['func_id'] = 'func-callback'


def callback_joint_func_substitution(observed):
    dispatch(observed, 'callback')['func_id'] = 'func-body'
    invoked(observed, 'callback')['func_id'] = 'func-body'


def func_carrier_alias_collapse(observed):
    identities = observed['r6_provenance']['func_identities']
    body = next(row for row in identities if row['func_id'] == 'func-body')
    callback = next(row for row in identities if row['func_id'] == 'func-callback')
    callback['carrier_id'] = body['carrier_id']


def func_wrong_producer(observed):
    identities = observed['r6_provenance']['func_identities']
    target = next(row for row in identities if row['func_id'] == 'func-target')
    target['producer_generation_id'] = 'unrelated-generation'


def with_callsite_substitution(observed):
    dispatch(observed, 'callback-target')['callsite_witness_id'] = 'call-callback'
    invoked(observed, 'callback-target')['callsite_witness_id'] = 'call-callback'
    coverage = observed['r6_provenance']['dispatch_coverage']
    coverage['expected_callsite_witness_ids'][-1] = 'call-callback'


def callback_phase_substitution(observed):
    for dispatch_id in ('callback', 'callback-target'):
        dispatch(observed, dispatch_id)['phase'] = 'body'
        invoked(observed, dispatch_id)['phase'] = 'body'


def fictional_extra_callback(observed):
    p = observed['r6_provenance']
    p['causal_witnesses']['call-extra'] = {
        'kind': 'invocation_callsite', 'observation': 'actual',
        'binding_point': 'fictional callback',
    }
    p['causal_witnesses']['func-extra'] = {
        'kind': 'func_identity', 'observation': 'actual',
        'binding_point': 'fictional carrier',
    }
    p['func_identities'].append({
        'func_id': 'func-extra', 'carrier_id': 'carrier-extra',
        'producer_generation_id': 'gen-extra', 'status': 'observed',
        'origin_witness_id': 'func-extra',
    })
    p['function_dispatches'].append({
        'dispatch_id': 'callback-extra', 'dispatch_sequence': 4,
        'func_id': 'func-extra', 'invocation_kind': 'callback',
        'callback_kind': 'other_callback', 'phase': 'diagnostics',
        'dispatch_role': 'entry', 'parent_dispatch_id': None,
        'callsite_witness_id': 'call-extra', 'status': 'observed',
    })
    p['dispatch_coverage']['expected_dispatch_ids'].append('callback-extra')
    p['dispatch_coverage']['expected_callsite_witness_ids'].append('call-extra')
    observed['events'].append({
        'kind': 'FunctionInvoked', 'dispatch_id': 'callback-extra',
        'func_id': 'func-extra', 'callback_kind': 'other_callback',
        'phase': 'diagnostics', 'callsite_witness_id': 'call-extra',
    })


def add_counter_join_fields(observed):
    event = candidate_event(observed)
    occurrence = observed['r6_provenance']['counter_occurrences'][0]
    event['key_id'] = 'counter:heading'
    event['action'] = {'kind': 'Step', 'level': 1}
    occurrence['key_id'] = 'counter:heading'
    occurrence['action'] = {'kind': 'Step', 'level': 1}


def counter_wrong_key(observed):
    add_counter_join_fields(observed)
    candidate_event(observed)['key_id'] = 'counter:other'


def counter_wrong_action(observed):
    add_counter_join_fields(observed)
    candidate_event(observed)['action'] = {'kind': 'Set', 'value': [1]}


def counter_wrong_location_coordinated(observed):
    candidate_event(observed)['location'] = 'loc-other'
    observed['r6_provenance']['counter_occurrences'][0]['location'] = 'loc-other'


def counter_wrong_producer_coordinated(observed):
    candidate_event(observed)['producer_generation_id'] = 'gen-other'
    observed['r6_provenance']['counter_occurrences'][0][
        'producer_generation_id'] = 'gen-other'


def counter_wrong_content_coordinated(observed):
    candidate_event(observed)['content_id'] = 'content-other'
    p = observed['r6_provenance']
    p['counter_occurrences'][0]['walked_content_id'] = 'content-other'
    p['content_lineage'][0]['to_content_id'] = 'content-other'


def counter_read_span_substitution(observed):
    span = copy.deepcopy(observed['r6_provenance']['counter_origins'][0]['span'])
    span['source_id'] = 'read-site.typ'
    span['raw_span'] = 'span:read:wrong'
    observed['r6_provenance']['counter_origins'][0]['span'] = span
    candidate_event(observed)['span'] = copy.deepcopy(span)


def counter_root_role_substitution(observed):
    origin = observed['r6_provenance']['counter_origins'][0]
    origin['span_role'] = 'root'


def content_lineage_distinct_id_cycle(observed):
    p = observed['r6_provenance']
    p['content_lineage'] = [
        {'lineage_id': 'lineage-a', 'from_content_id': 'content-source',
         'to_content_id': 'content-mid', 'lineage_witness_id': 'content-map',
         'status': 'observed'},
        {'lineage_id': 'lineage-b', 'from_content_id': 'content-mid',
         'to_content_id': 'content-source', 'lineage_witness_id': 'content-map',
         'status': 'observed'},
        {'lineage_id': 'lineage-c', 'from_content_id': 'content-source',
         'to_content_id': 'content-walked', 'lineage_witness_id': 'content-map',
         'status': 'observed'},
    ]
    p['counter_occurrences'][0]['lineage_ids'] = [
        'lineage-a', 'lineage-b', 'lineage-c']


def content_lineage_break(observed):
    observed['r6_provenance']['content_lineage'][0][
        'from_content_id'] = 'unrelated-content'


def nested_dict_retyped_as_func(observed):
    row = observed['r6_provenance']['typed_dict_values'][1]
    row['value']['Dict'][0][1] = {'Func': 'runtime-identity'}


def nested_dict_order_reversed(observed):
    row = observed['r6_provenance']['typed_dict_values'][1]
    pairs = row['value']['Dict'][0][1]['Dict'][0][1]['Length']['Dict']
    pairs.reverse()


def dict_payload_substitution(observed):
    row = observed['r6_provenance']['typed_dict_values'][0]
    row['value']['Dict'][0][1] = 'other-runtime-identity'


def dict_duplicate_nested_key(observed):
    row = observed['r6_provenance']['typed_dict_values'][1]
    pairs = row['value']['Dict'][0][1]['Dict'][0][1]['Length']['Dict']
    pairs.append(copy.deepcopy(pairs[0]))


def top_level_dict_order_changed(observed):
    row = observed['r6_provenance']['typed_dict_values'][0]
    row['value']['Dict'].append(['extra', 1])
    row['entry_key_order'].append('extra')


def opaque_provenance_hides_callback_omission(observed):
    policy.coherent_callback_omission(observed)
    observed['r6_provenance']['status'] = 'opaque'


def opaque_counter_hides_invalid_range(observed):
    origin = observed['r6_provenance']['counter_origins'][0]
    origin['span']['byte_start'] = 20
    origin['span']['byte_end'] = 10
    origin['status'] = 'opaque'


def opaque_feature_hides_wrong_actual(observed):
    context = observed['r6_provenance']['feature_contexts'][0]
    context['features'] = ['html']
    context['status'] = 'opaque'


def opaque_dict_hides_order_mismatch(observed):
    row = observed['r6_provenance']['typed_dict_values'][0]
    row['entry_key_order'] = ['wrong']
    row['status'] = 'opaque'


def valid_feature_opacity(observed):
    observed['r6_provenance']['feature_contexts'][0]['status'] = 'opaque'


def inheritance_case(factory):
    def run():
        case, observed, _reason = factory()
        return inheritance.new.classify(case, observed)
    return run


def policy_case(mutate):
    def run():
        case, observed = policy.seeded()
        mutate(observed)
        return policy.new.classify(case, observed)
    return run


MUTANTS = [
    ('K-INHERIT-SINK', 'known-r6-finding:inheritance', inheritance_case(inheritance.sink_mutant)),
    ('K-INHERIT-FOLD', 'known-r6-finding:inheritance', inheritance_case(inheritance.fold_mutant)),
    ('K-INHERIT-HISTORY', 'known-r6-finding:inheritance', inheritance_case(inheritance.history_mutant)),
    ('K-INHERIT-CALLBACK', 'known-r6-finding:inheritance', inheritance_case(inheritance.callback_mutant)),
    ('K-OMIT-CALLBACK', 'known-r6-finding:coverage', policy_case(policy.coherent_callback_omission)),
    ('K-OMIT-DICT', 'known-r6-finding:coverage', policy_case(policy.coherent_dict_omission)),
    ('K-OMIT-COUNTER', 'known-r6-finding:coverage', policy_case(policy.coherent_counter_omission)),
    ('K-OPAQUE-FEATURE-MISSING', 'known-r6-finding:unknown', policy_case(policy.malformed_feature_opaque)),
    ('K-OPAQUE-FUNC-MISSING', 'known-r6-finding:unknown', policy_case(policy.malformed_func_opaque)),
    ('K-OPAQUE-DISPATCH-MISSING', 'known-r6-finding:unknown', policy_case(policy.malformed_dispatch_opaque)),
    ('K-OPAQUE-COUNTER-MISSING', 'known-r6-finding:unknown', policy_case(policy.malformed_counter_opaque)),
    ('K-OPAQUE-DICT-MISSING', 'known-r6-finding:unknown', policy_case(policy.malformed_dict_opaque)),
    ('N-FEATURE-COORDINATED', 'new:feature-actual-anchor', policy_case(feature_coordinated_substitution)),
    ('N-FEATURE-ORIGIN-RELABEL', 'new:feature-actual-anchor', policy_case(feature_origin_relabel)),
    ('N-FEATURE-FICTITIOUS-INHERIT', 'new:feature-actual-anchor', policy_case(feature_fictitious_inheritance)),
    ('N-FEATURE-WITNESS-ALIAS', 'new:feature-actual-anchor', policy_case(feature_witness_alias)),
    ('N-BODY-CALLBACK-FUNC', 'new:func-carrier-origin', policy_case(body_entry_uses_callback_func)),
    ('N-CALLBACK-JOINT-FUNC', 'new:func-carrier-origin', policy_case(callback_joint_func_substitution)),
    ('N-FUNC-CARRIER-ALIAS', 'new:func-carrier-origin', policy_case(func_carrier_alias_collapse)),
    ('N-FUNC-WRONG-PRODUCER', 'new:func-carrier-origin', policy_case(func_wrong_producer)),
    ('B-WITH-CALLSITE', 'boundary:external-callback', policy_case(with_callsite_substitution)),
    ('B-CALLBACK-PHASE', 'boundary:external-callback', policy_case(callback_phase_substitution)),
    ('B-EXTRA-CALLBACK', 'boundary:external-callback', policy_case(fictional_extra_callback)),
    ('N-COUNTER-WRONG-KEY', 'new:counter-key-action', policy_case(counter_wrong_key)),
    ('N-COUNTER-WRONG-ACTION', 'new:counter-key-action', policy_case(counter_wrong_action)),
    ('B-COUNTER-WRONG-LOCATION', 'boundary:external-counter', policy_case(counter_wrong_location_coordinated)),
    ('N-COUNTER-WRONG-PRODUCER', 'new:counter-occurrence-anchor', policy_case(counter_wrong_producer_coordinated)),
    ('N-COUNTER-WRONG-CONTENT', 'new:counter-occurrence-anchor', policy_case(counter_wrong_content_coordinated)),
    ('B-COUNTER-READ-SPAN', 'boundary:external-counter', policy_case(counter_read_span_substitution)),
    ('N-COUNTER-ROOT-ROLE', 'new:counter-span-role', policy_case(counter_root_role_substitution)),
    ('N-LINEAGE-CYCLE', 'new:lineage-global-acyclicity', policy_case(content_lineage_distinct_id_cycle)),
    ('B-LINEAGE-BREAK', 'boundary:lineage-continuity', policy_case(content_lineage_break)),
    ('N-DICT-NESTED-RETYPE', 'new:dict-payload-anchor', policy_case(nested_dict_retyped_as_func)),
    ('N-DICT-NESTED-REORDER', 'new:dict-payload-anchor', policy_case(nested_dict_order_reversed)),
    ('N-DICT-PAYLOAD', 'new:dict-payload-anchor', policy_case(dict_payload_substitution)),
    ('N-DICT-DUPLICATE-NESTED-KEY', 'new:dict-payload-anchor', policy_case(dict_duplicate_nested_key)),
    ('B-DICT-TOP-ORDER', 'boundary:external-dict', policy_case(top_level_dict_order_changed)),
    ('N-OPAQUE-HIDES-COVERAGE', 'new:unknown-semantic-short-circuit', policy_case(opaque_provenance_hides_callback_omission)),
    ('N-OPAQUE-HIDES-SPAN-RANGE', 'new:unknown-semantic-short-circuit', policy_case(opaque_counter_hides_invalid_range)),
    ('N-OPAQUE-HIDES-FEATURE', 'new:unknown-semantic-short-circuit', policy_case(opaque_feature_hides_wrong_actual)),
    ('N-OPAQUE-HIDES-DICT-ORDER', 'new:unknown-semantic-short-circuit', policy_case(opaque_dict_hides_order_mismatch)),
]


def run_order(label, rows):
    start = time.monotonic()
    results = []
    for mutant_id, failure_class, run in rows:
        answer = run()
        actual = answer['classification']
        results.append({
            'id': mutant_id,
            'failure_class': failure_class,
            'expected': 'Violated',
            'actual': actual,
            'killed': actual == 'Violated',
            'reason_code': answer.get('reason_code'),
        })
    return {'order': label, 'seconds': time.monotonic() - start,
            'results': results}


def positive_control():
    case, observed = policy.seeded()
    return policy.new.classify(case, observed)


def opaque_control():
    case, observed = policy.seeded()
    valid_feature_opacity(observed)
    return policy.new.classify(case, observed)


def main():
    start = time.monotonic()
    runs = [
        run_order('normal', MUTANTS),
        run_order('repeat', MUTANTS),
        run_order('reverse', list(reversed(MUTANTS))),
    ]
    by_order = {
        run['order']: {row['id']: row['actual'] for row in run['results']}
        for run in runs
    }
    deterministic = (by_order['normal'] == by_order['repeat'] ==
                     by_order['reverse'])
    normal = runs[0]['results']
    survivors = [row for row in normal if not row['killed']]
    killed = len(normal) - len(survivors)
    known = [row for row in normal if row['failure_class'].startswith('known-')]
    new = [row for row in normal if row['failure_class'].startswith('new:')]
    new_survivor_classes = sorted({row['failure_class'] for row in new
                                   if not row['killed']})
    payload = {
        'schema': 'p1340-adversary-mutations-r6b-result',
        'scope': 'synthetic-contract-only-no-product-credit',
        'orders': runs,
        'controls': {
            'positive': positive_control(),
            'well_formed_opaque': opaque_control(),
        },
        'summary': {
            'unique_valid_negative_mutants': len(normal),
            'killed': killed,
            'survived': len(survivors),
            'mutation_score': killed / len(normal),
            'known_findings_killed': sum(row['killed'] for row in known),
            'known_findings_total': len(known),
            'new_mutants_killed': sum(row['killed'] for row in new),
            'new_mutants_total': len(new),
            'survivor_ids': [row['id'] for row in survivors],
            'new_survivor_classes': new_survivor_classes,
            'deterministic_normal_repeat_reverse': deterministic,
            'total_classifications': sum(len(run['results']) for run in runs),
            'seconds_total': time.monotonic() - start,
        },
        'stop_required': bool(new_survivor_classes),
        'product_runs': 0,
        'candidate_reads': 0,
        'seal': None,
        'product_verdict': None,
    }
    print(json.dumps(payload, indent=2, ensure_ascii=True))
    controls_ok = (payload['controls']['positive']['classification'] == 'Preserved' and
                   payload['controls']['well_formed_opaque']['classification'] == 'Unknown')
    return 0 if deterministic and controls_ok else 1


if __name__ == '__main__':
    raise SystemExit(main())
