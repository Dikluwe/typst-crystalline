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


def semantic_attacks():
    s = importlib.util.spec_from_file_location('semantic_review', D/'p1322-review-semantic.py')
    semantic = importlib.util.module_from_spec(s)
    s.loader.exec_module(semantic)
    catalog = check.read('p1322-probe-catalog.json')
    principal = check.read('p1322-matrix-normal.json')['results']
    supplementary = check.read('p1322-sentinels-normal.json')['rows']
    ledger = semantic.table('p1322-classification-owner-ledger.tsv')
    supplemental_ledger = semantic.table('p1322-classification-supplemental-ledger.tsv')
    transitions = semantic.table('p1322-classification-transitions.tsv')
    selection = check.read('p1322-classification-selection.json')
    previous = check.read('p1309-matrix-normal.json')['results']
    rows = []
    entry = next(x for x in ledger if x['current_language_class']=='CLOSED_MEASURED_LOOKUP_REPR_ONLY' and x['path']=='json')
    focal_catalog = {**catalog, 'probes':[p for p in catalog['probes'] if p['id']==entry['probe_id']]}
    focal_rows = [x for x in principal if x['id']==entry['probe_id']]
    def pjudge(data):
        return semantic.verify_principal(data, focal_catalog, focal_rows)
    mutant = [copy.deepcopy(entry)]
    mutant[0]['current_language_class']='CLOSED_MEASURED_FUNCTIONAL_SENTINEL'
    rows.append(attack_result('D07', [entry], mutant, pjudge,
        [{'probe_id':entry['probe_id'], 'replace':'current_language_class', 'value':'CLOSED_MEASURED_FUNCTIONAL_SENTINEL',
          'fault':'lookup/type/repr alone promoted to functional closure'}]))
    closure = next(x for x in supplemental_ledger if x['probe_id']=='p1311.named-closure')
    native = next(x for x in supplementary if x['profile']=='default' and x['expression']=='csv.encode')
    cr = [x for x in supplementary if x['id']==closure['probe_id']]
    def ijudge(data):
        return semantic.verify_supplement(data, cr)
    mutant = [copy.deepcopy(closure)]
    mutant[0]['measurement_ref']='00_nucleo/diagnosticos/p1322-sentinels-normal.json#'+native['id']
    mutant[0]['current_language_class']='CLOSED_MEASURED_FUNCTIONAL_SENTINEL'
    rows.append(attack_result('D08', [closure], mutant, ijudge,
        [{'replace':'named-closure measurement_ref', 'value':mutant[0]['measurement_ref'],
          'fault':'same nominal spelling csv used to substitute native evidence for a user closure',
          'native_expression':native['expression'], 'closure_expression':cr[0]['expression']}]))
    # Simulate a regression only inside audit data. The genuine run has no such regression.
    original = next(x for x in principal if x['id']==entry['probe_id'] and x['profile']=='default')
    prior = next(x for x in previous if x['id']==original['id'] and x['profile']==original['profile'])
    changed = copy.deepcopy(original)
    changed['crystalline']['stdout']='"D09 simulated changed public value"\n'
    raw = changed['crystalline']['stdout'].encode()
    changed['crystalline']['stdout_base64']=base64.b64encode(raw).decode()
    changed['crystalline']['stdout_sha256']=check.sha(raw)
    changed['crystalline']['parsed_value']='D09 simulated changed public value'
    changed['runtime_class']=check.classify(changed['vanilla'],changed['crystalline'])
    tr = copy.deepcopy(next(x for x in transitions if x['probe_id']==original['id'] and x['profile']=='default'))
    tr.update(current_runtime_class=changed['runtime_class'], transition='REGRESSION_CANDIDATE')
    def tjudge(data):
        return semantic.verify_transitions(data, [changed], [prior])
    rows.append(attack_result('D09', [tr], [], tjudge,
        [{'scenario':'genuine historical MATCH and explicit simulated changed current value in copied row',
          'current_copy_sha256':canonical_hash(changed), 'previous_row_sha256':canonical_hash(prior)},
         {'remove':'the resulting REGRESSION_CANDIDATE transition'}]))
    ext = next(x for x in ledger if x['path']=='calc.deg')
    ec = {**catalog, 'probes':[p for p in catalog['probes'] if p['id']==ext['probe_id']]}
    er = [x for x in principal if x['id']==ext['probe_id']]
    def ejudge(data):
        return semantic.verify_principal(data, ec, er)
    mutant = [copy.deepcopy(ext)]
    mutant[0].update(current_language_class='DOCUMENTED_PRODUCT_EXTENSION',normative_evidence='')
    rows.append(attack_result('D11', [ext], mutant, ejudge,
        [{'probe_id':ext['probe_id'], 'replace':'current_language_class', 'value':'DOCUMENTED_PRODUCT_EXTENSION', 'normative_evidence':''}]))
    alternatives = [c for c in selection['cohorts'] if c.get('eligible') and c['id']!='namespace-function-missing-field']
    assert alternatives, 'Need a genuinely eligible competing cohort'
    mutant = copy.deepcopy(selection)
    alternate = sorted(alternatives,key=lambda c:c['rank_key'])[-1]
    if isinstance(mutant.get('selected'), dict):
        mutant['selected']=copy.deepcopy(alternate)
    else:
        mutant['selected']=alternate['id']
    rows.append(attack_result('D12', selection, mutant, semantic.verify_selection,
        [{'replace':'selected', 'value':alternate['id'], 'fault':'select inferior priority/tie-break instead of complete causal rank'}]))
    mutant = copy.deepcopy(selection)
    omitted = next(c for c in mutant['cohorts'] if c['id']=='loader-data-source-missing')
    omitted['owners']=[p for p in omitted['owners'] if p!=semantic.DISPATCH]
    omitted['owner_prompts']=[p for p in omitted['owner_prompts'] if not p.endswith('/call_dispatch.md')]
    omitted['owner_count']=len(omitted['owners'])
    if 'rank_key' in omitted:
        omitted['rank_key'][1]=omitted['owner_count']
    rows.append(attack_result('D13', selection, mutant, semantic.verify_selection,
        [{'cohort':'loader-data-source-missing', 'omit_owner':semantic.DISPATCH,
          'fault':'remove whole-call origin transport while claiming complete diagnostic',
          'independent_source_anchors':['01_core/src/compiler/stdlib/loading.rs:1197','01_core/src/compiler/eval/call_dispatch.rs:455','01_core/src/compiler/eval/call_dispatch.rs:1633']}]))
    check.INPUTS.update(semantic.core.INPUTS)
    check.INPUTS[str(D/'p1322-review-semantic.py')]=check.digest(D/'p1322-review-semantic.py')
    return rows, {'selection_control':semantic.verify_selection(selection).result()}


def main():
    phase = sys.argv[1]
    suffix = sys.argv[2] if len(sys.argv)>2 else ''
    start, t0 = check.utc(), time.monotonic()
    rows, controls = {'catalog': catalog_attacks, 'runtime': runtime_attacks, 'semantic':semantic_attacks}[phase]()
    result = {'schema': 'p1322-review-attacks-v1', 'phase': phase, 'start': start, 'end': check.utc(),
        'seconds': time.monotonic()-t0, 'plan_sha256': check.digest(D/'p1322-review-plan.md'),
        'checker_sha256': check.digest(D/'p1322-review-check.py'), 'adversary_sha256': check.digest(__file__),
        'inputs': check.INPUTS, 'isolation': 'not technically attested', 'attacks': rows,
        'controls': controls, 'valid': sum(a['valid'] for a in rows),
        'rejected': sum(a['valid'] and a['rejected'] for a in rows)}
    publish('p1322-review-attacks-'+phase+suffix+'.json', result)
    print(json.dumps({k:v for k,v in result.items() if k in ['phase','valid','rejected','seconds']}))
    return 0 if all(a['valid'] and a['rejected'] for a in rows) else 1


if __name__ == '__main__':
    sys.exit(main())
