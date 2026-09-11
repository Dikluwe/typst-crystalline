"""Conjunctive executable successor for the unfinished R4 lifecycle fragment."""

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


prior = load('p1340_contract_r4_base', 'p1340-contract-base-r4.py')
r6 = load('p1340_contract_observability_r6', 'p1340-contract-observability-r6.py')


def classify(case, observed):
    focal = r6.classify(case, observed)
    if focal['classification'] != 'Preserved':
        return focal
    try:
        prior.check(case, observed)
    except (AssertionError, KeyError, TypeError, IndexError) as error:
        return {'classification': 'Violated', 'reason_code': 'inherited-r4:' + str(error)}
    return {'classification': 'Preserved', 'case_id': case['id']}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--fixtures', required=True)
    parser.add_argument('--observations', required=True)
    args = parser.parse_args()
    fixture = json.loads(Path(args.fixtures).read_text())
    observations = json.loads(Path(args.observations).read_text())['cases']
    lookup = {case['id']: case for case in fixture['cases']}
    expected = {
        (case['id'], profile, order)
        for case in lookup.values()
        for profile in case['execution']['profiles']
        for order in case['execution']['orders']
    }
    actual = [(row['case_id'], row['profile'], row['order']) for row in observations]
    if len(actual) != len(set(actual)) or set(actual) != expected:
        print(json.dumps({'classification': 'Violated',
                          'reason_code': 'R6-exact-required-matrix'}))
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
