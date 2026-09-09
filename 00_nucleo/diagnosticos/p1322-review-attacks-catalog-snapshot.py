#!/usr/bin/env python3
"""Execute the frozen D attacks on copies; does not mutate judged inputs."""
import base64
import collections
import copy
import importlib.util
import json
import pathlib
import subprocess
import sys
import time
sys.dont_write_bytecode = True
D = pathlib.Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('independent_checker', D/'p1322-review-check.py')
check = importlib.util.module_from_spec(spec)
spec.loader.exec_module(check)


def canonical_hash(obj):
    return check.sha(json.dumps(obj, sort_keys=True, separators=(',', ':'), ensure_ascii=False).encode())


def publish(name, obj):
    path = D/name
    assert name.startswith('p1322-review-') and not path.exists()
    payload = json.dumps(obj, ensure_ascii=False, indent=2)+'\n'
    patch = '*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join(
        '+'+line+'\n' for line in payload.splitlines())+'*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, text=True, capture_output=True, check=True)


def attack_result(id_, control, mutant, check_fn, changes, expected='Violated'):
    control_result = check_fn(control).result()
    result = check_fn(mutant).result()
    return {'id': id_, 'expected': expected, 'control_expected': 'Preserved',
            'control': control_result, 'observed': result,
            'control_sha256': canonical_hash(control), 'mutant_sha256': canonical_hash(mutant),
            'transformation': changes, 'valid': control_result['verdict']=='Preserved',
            'rejected': result['verdict']==expected and bool(result['violations'])}


def catalog_attacks():
    catalog = check.read('p1322-probe-catalog.json')
    reconciliation = check.read('p1322-inventory-reconciliation.json')
    history = check.read('p1309-probe-catalog.json')
    inv = [check.read(check.ROOT/s['path']) for s in catalog['inventory_inputs']]
    original = {'catalog': catalog, 'reconciliation': reconciliation}
    def judge(data):
        return check.verify_catalog(data['catalog'], data['reconciliation'], history, inv)
    old_id = history['probes'][0]['id']
    mutant = copy.deepcopy(original)
    mutant['reconciliation']['unchanged'] = [p for p in mutant['reconciliation']['unchanged'] if p['probe_id'] != old_id]
    rows = [attack_result('D01', original, mutant, judge,
            [{'remove': '/reconciliation/unchanged[probe_id='+old_id+']'}])]
    fresh = next(p for p in catalog['probes'] if p['id'] not in {x['id'] for x in history['probes']})
    mutant = copy.deepcopy(original)
    mutant['catalog']['probes'] = [p for p in mutant['catalog']['probes'] if p['id'] != fresh['id']]
    mutant['reconciliation']['added'] = [p for p in mutant['reconciliation']['added'] if p['probe_id'] != fresh['id']]
    rows.append(attack_result('D02', original, mutant, judge,
                [{'remove_probe_and_reconciliation': fresh}]))
    shuffled = copy.deepcopy(original)
    shuffled['catalog']['probes'].reverse()
    shuffled['reconciliation']['unchanged'].reverse()
    return rows, {'reordered_catalog': judge(shuffled).result()}


def runtime_attacks():
    catalog = check.read('p1322-probe-catalog.json')
    matrix = check.read('p1322-matrix-normal.json')
    frozen = check.read('p1322-runtime-freeze.json')
    rows = []
    def focal(probe_id):
        c = {**catalog, 'probes': [p for p in catalog['probes'] if p['id']==probe_id]}
        m = {**matrix, 'results': [copy.deepcopy(p) for p in matrix['results'] if p['id']==probe_id]}
        m['counts'] = dict(collections.Counter(p['runtime_class'] for p in m['results']))
        m['probes'], m['pairs'] = len(c['probes']), len(m['results'])
        return c, m
    candidate = next(p for p in matrix['results'] if p['runtime_class'] in check.MATCH)
    c, control = focal(candidate['id'])
    def judge(data):
        return check.verify_matrix(data, c, frozen['binaries'], 'normal')[0]
    mutant = copy.deepcopy(control)
    mutant['results'][0]['crystalline']['binary_sha256'] = frozen['binaries']['vanilla']['sha256']
    rows.append(attack_result('D03', control, mutant, judge,
                [{'replace': '/results/0/crystalline/binary_sha256', 'value': frozen['binaries']['vanilla']['sha256']}]))
    mutant = copy.deepcopy(control)
    mutant['results'][0]['crystalline']['features'] = ['html'] if not mutant['results'][0]['crystalline']['features'] else []
    rows.append(attack_result('D04', control, mutant, judge,
                [{'replace': '/results/0/crystalline/features', 'value': mutant['results'][0]['crystalline']['features']}]))
    diagnostic = next(p for p in matrix['results'] if p['runtime_class']=='MATCH_DIAGNOSTIC' and p['crystalline']['stderr'])
    dc, original = focal(diagnostic['id'])
    def djudge(data):
        return check.verify_matrix(data, dc, frozen['binaries'], 'normal')[0]
    mutant = copy.deepcopy(original)
    del mutant['results'][0]['crystalline']['stderr']
    rows.append(attack_result('D05', original, mutant, djudge,
                [{'remove': '/results/0/crystalline/stderr', 'preserve_declared_class': True}]))
    # The opaque control is deliberately constructed from a genuine transcript.
    # It is a test of missing completion, not a claim that the actual run timed out.
    opaque = copy.deepcopy(control)
    opaque['results'][0]['crystalline'].update(complete=False, reason_code='TIMEOUT', exit_code=None)
    opaque['results'][0]['runtime_class'] = 'EXECUTION_UNKNOWN'
    opaque['counts'] = dict(collections.Counter(p['runtime_class'] for p in opaque['results']))
    mutant = copy.deepcopy(opaque)
    mutant['results'][0]['runtime_class'] = 'MATCH_VALUE'
    mutant['counts'] = dict(collections.Counter(p['runtime_class'] for p in mutant['results']))
    opaque_result = judge(opaque).result()
    result = judge(mutant).result()
    rows.append({'id': 'D06', 'expected': 'Violated', 'control_expected': 'Unknown',
        'control': opaque_result, 'observed': result, 'control_sha256': canonical_hash(opaque),
        'mutant_sha256': canonical_hash(mutant), 'valid': opaque_result['verdict']=='Unknown',
        'rejected': result['verdict']=='Violated' and any(x['code']=='RUNTIME_CLASS' for x in result['violations']),
        'transformation': [{'control': 'genuine row with explicitly incomplete simulated timeout'},
                           {'replace': '/results/0/runtime_class', 'value': 'MATCH_VALUE'}]})
    mutant = copy.deepcopy(control)
    supplement = check.read('p1322-sentinels-normal.json')
    mutant['pairs'] += len(supplement['rows'])
    rows.append(attack_result('D10', control, mutant, judge,
                [{'add_supplement_rows_to_pairs': len(supplement['rows'])}]))
    shuffled = copy.deepcopy(control)
    shuffled['results'].reverse()
    return rows, {'reordered_runtime': judge(shuffled).result(), 'opaque_control': opaque_result}


def main():
    phase = sys.argv[1]
    start, t0 = check.utc(), time.monotonic()
    rows, controls = {'catalog': catalog_attacks, 'runtime': runtime_attacks}[phase]()
    result = {'schema': 'p1322-review-attacks-v1', 'phase': phase, 'start': start, 'end': check.utc(),
        'seconds': time.monotonic()-t0, 'plan_sha256': check.digest(D/'p1322-review-plan.md'),
        'checker_sha256': check.digest(D/'p1322-review-check.py'), 'adversary_sha256': check.digest(__file__),
        'inputs': check.INPUTS, 'isolation': 'not technically attested', 'attacks': rows,
        'controls': controls, 'valid': sum(a['valid'] for a in rows),
        'rejected': sum(a['valid'] and a['rejected'] for a in rows)}
    publish('p1322-review-attacks-'+phase+'.json', result)
    print(json.dumps({k:v for k,v in result.items() if k in ['phase','valid','rejected','seconds']}))
    return 0 if all(a['valid'] and a['rejected'] for a in rows) else 1


if __name__ == '__main__':
    sys.exit(main())
