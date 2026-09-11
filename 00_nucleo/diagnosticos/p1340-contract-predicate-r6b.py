"""R6b conjunction: full R4 scenario predicate, then the R6 fragment."""

import argparse
import importlib.util
import json
from pathlib import Path


HERE = Path(__file__).parent


def load(name, filename):
    spec = importlib.util.spec_from_file_location(name, HERE / filename)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


predecessor = load('p1340_contract_predicate_r4_full',
                   'p1340-contract-predicate-r4.py')
r6 = load('p1340_contract_observability_r6b',
          'p1340-contract-observability-r6b.py')


def classify(case, observed):
    try:
        predecessor.check(case, observed)
    except (AssertionError, KeyError, TypeError, IndexError, StopIteration) as error:
        return {'classification': 'Violated',
                'reason_code': 'inherited-r4-full:' + str(error)}
    focal = r6.classify(case, observed)
    if focal['classification'] != 'Preserved':
        return focal
    return {'classification': 'Preserved', 'case_id': case['id']}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--fixtures', required=True)
    parser.add_argument('--observations', required=True)
    parser.add_argument('--r6-expectations', required=True)
    args = parser.parse_args()
    fixture = json.loads(Path(args.fixtures).read_text())
    observations = json.loads(Path(args.observations).read_text())['cases']
    expectation_rows = json.loads(Path(args.r6_expectations).read_text())
    expectation_lookup = {row['case_id']: row['expectations']
                          for row in expectation_rows['cases']}
    lookup = {case['id']: case for case in fixture['cases']}
    if set(expectation_lookup) != set(lookup):
        print(json.dumps({'classification': 'Violated',
                          'reason_code': 'R6b-external-expectation-case-set'}))
        return 1
    for case_id, case in lookup.items():
        case['r6_expectations'] = expectation_lookup[case_id]
    expected = {
        (case['id'], profile, order)
        for case in lookup.values()
        for profile in case['execution']['profiles']
        for order in case['execution']['orders']
    }
    actual = [(row['case_id'], row['profile'], row['order']) for row in observations]
    if len(actual) != len(set(actual)) or set(actual) != expected:
        print(json.dumps({'classification': 'Violated',
                          'reason_code': 'R6b-exact-required-matrix'}))
        return 1
    rows = [classify(lookup[row['case_id']], row) for row in observations]
    for row in rows:
        print(json.dumps(row, ensure_ascii=True))
    if any(row['classification'] == 'Violated' for row in rows):
        return 1
    if any(row['classification'] == 'Unknown' for row in rows):
        return 2
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
