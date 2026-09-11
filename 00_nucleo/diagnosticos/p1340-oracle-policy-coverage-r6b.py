"""Synthetic R6b focals for strict Unknown and externally frozen coverage."""

import copy
import importlib.util
import json
from pathlib import Path


HERE = Path(__file__).parent


def load(name, filename):
    spec = importlib.util.spec_from_file_location(name, HERE / filename)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


fixture = load('p1340_oracle_r6_frozen', 'p1340-oracle-focal-r6.py')
old = load('p1340_observability_r6_frozen', 'p1340-contract-observability-r6.py')
new = load('p1340_observability_r6b', 'p1340-contract-observability-r6b.py')


def seeded():
    case, observed = fixture.base()
    span = copy.deepcopy(observed['r6_provenance']['counter_origins'][0]['span'])
    case['r6_expectations'] = {
        'status': 'frozen',
        'authority_sha256': 'a' * 64,
        'callback_dispatches': [
            {'callsite_witness_id': 'call-callback', 'callback_kind': 'counter_fold',
             'phase': 'validation', 'dispatch_role': 'entry', 'count': 1},
            {'callsite_witness_id': 'call-target', 'callback_kind': 'counter_fold',
             'phase': 'validation', 'dispatch_role': 'with_target', 'count': 1},
        ],
        'dict_values': [
            {'value_id': 'dict-func', 'entry_key_order': ['Func']},
            {'value_id': 'dict-recursive', 'entry_key_order': ['Dict']},
        ],
        'counter_events': [
            {'snapshot_id': 'I1', 'event_index': 0, 'location': 'loc-1',
             'origin_kind': 'manual', 'span': span},
        ],
    }
    return case, observed


def coherent_callback_omission(observed):
    observed['events'] = [row for row in observed['events']
                          if row.get('kind') != 'FunctionInvoked']
    p = observed['r6_provenance']
    p['function_dispatches'] = [row for row in p['function_dispatches']
                                if row['invocation_kind'] != 'callback']
    used = {row['func_id'] for row in p['function_dispatches']}
    p['func_identities'] = [row for row in p['func_identities']
                            if row['func_id'] in used]
    p['dispatch_coverage']['expected_dispatch_ids'] = [
        row['dispatch_id'] for row in p['function_dispatches']]
    p['dispatch_coverage']['expected_callsite_witness_ids'] = [
        row['callsite_witness_id'] for row in p['function_dispatches']]


def coherent_dict_omission(observed):
    p = observed['r6_provenance']
    p['typed_dict_values'] = []
    p['dict_coverage']['expected_value_ids'] = []


def coherent_counter_omission(observed):
    for row in observed['events']:
        if row.get('kind') == 'CandidateBuilt':
            row['counter_events'] = []
    p = observed['r6_provenance']
    p['counter_origins'] = []
    p['counter_occurrences'] = []
    p['content_lineage'] = []


def malformed_feature_opaque(observed):
    row = observed['r6_provenance']['feature_contexts'][0]
    del row['source']; row['status'] = 'opaque'


def malformed_func_opaque(observed):
    row = observed['r6_provenance']['func_identities'][1]
    del row['carrier_id']; row['status'] = 'opaque'


def malformed_dispatch_opaque(observed):
    row = observed['r6_provenance']['function_dispatches'][2]
    del row['func_id']; row['status'] = 'opaque'


def malformed_counter_opaque(observed):
    row = observed['r6_provenance']['counter_origins'][0]
    del row['span']; row['status'] = 'opaque'


def malformed_dict_opaque(observed):
    row = observed['r6_provenance']['typed_dict_values'][0]
    del row['value']; row['status'] = 'opaque'


def main():
    mutations = [
        ('coherent-callback-omission', coherent_callback_omission, 'Preserved',
         'R6b-external-callback-coverage'),
        ('coherent-dict-omission', coherent_dict_omission, 'Preserved',
         'R6b-external-Dict-coverage'),
        ('coherent-counter-event-omission', coherent_counter_omission, 'Preserved',
         'R6b-external-counter-event-coverage'),
        ('malformed-feature-plus-opaque', malformed_feature_opaque, 'Unknown',
         'R6b-malformed-ledger'),
        ('malformed-Func-plus-opaque', malformed_func_opaque, 'Unknown',
         'R6b-malformed-ledger'),
        ('malformed-dispatch-plus-opaque', malformed_dispatch_opaque, 'Unknown',
         'R6b-malformed-ledger'),
        ('malformed-counter-plus-opaque', malformed_counter_opaque, 'Unknown',
         'R6b-malformed-ledger'),
        ('malformed-Dict-plus-opaque', malformed_dict_opaque, 'Unknown',
         'R6b-malformed-ledger'),
    ]
    rows = []
    for name, mutate, expected_before, reason in mutations:
        case, observed = seeded()
        mutate(observed)
        before = old.classify(case, observed)
        after = new.classify(case, observed)
        rows.append({
            'id': name,
            'expected_before': expected_before,
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
        'schema': 'p1340-oracle-policy-coverage-r6b-result',
        'scope': 'synthetic-contract-unit-only-no-product-credit',
        'cases': rows,
        'delta': {'survived_or_unknown_before': len(rows),
                  'violated_after': sum(row['actual_after'] == 'Violated'
                                        for row in rows)},
    }, indent=2))
    return 0 if ok else 1


if __name__ == '__main__':
    raise SystemExit(main())
