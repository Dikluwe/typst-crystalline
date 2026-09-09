#!/usr/bin/env python3
"""Independent consistency checks on C's ledger and selection, not its author."""
import csv
import importlib.util
import json
import pathlib
import re
import sys
sys.dont_write_bytecode = True
D = pathlib.Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('dchecker', D/'p1322-review-check.py')
core = importlib.util.module_from_spec(spec)
spec.loader.exec_module(core)
LOADING = '01_core/src/compiler/stdlib/loading.rs'
DISPATCH = '01_core/src/compiler/eval/call_dispatch.rs'
FIELD = '01_core/src/compiler/eval/bindings/field_access.rs'
# Minimum edit owners separately checked by D against current code:
# loading.rs:1197-1226 makes the decoder missing diagnostic; call_dispatch.rs:455
# and :1633 restrict whole-call transport to encoders/panic/CSV, excluding decoders.
MINIMUM_OWNERS = {'loader-data-source-missing': {LOADING, DISPATCH},
                  'namespace-function-missing-field': {FIELD}}


def table(name):
    data = (D/name).read_bytes()
    core.INPUTS[str(D/name)] = core.sha(data)
    return list(csv.DictReader(data.decode().splitlines(), delimiter='\t'))


def ledger_display(expression):
    # C's TSV is a one-line display. The complete source and its hash remain
    # authenticated in the frozen catalog/matrix; literal backslashes are intact.
    return re.sub(r'[\t\r\n]', ' ', expression)


def verify_transitions(transitions, current, previous):
    r = core.Review()
    old = {(x['id'], x['profile']): x for x in previous}
    by_key = {(x['probe_id'], x['profile']): x for x in transitions}
    expected = {(x['id'], x['profile']) for x in current}
    r.require(len(by_key) == len(transitions), 'TRANSITION_DUPLICATE', len(transitions))
    r.require(set(by_key) == expected, 'TRANSITION_OMITTED', sorted(expected-set(by_key)))
    for row in current:
        key = row['id'], row['profile']
        t = by_key.get(key)
        if not t:
            continue
        prior = old.get(key)
        if not prior:
            expected_transition = 'ADDED_FRESH'
        elif prior['runtime_class'] == row['runtime_class']:
            expected_transition = 'UNCHANGED_RUNTIME_CLASS'
        elif prior['runtime_class'] in core.MATCH and row['runtime_class'] not in core.MATCH:
            expected_transition = 'REGRESSION_CANDIDATE'
        elif prior['runtime_class'] not in core.MATCH and row['runtime_class'] in core.MATCH:
            expected_transition = 'CLOSED_NOW'
        else:
            expected_transition = 'CHANGED_NONMATCH'
        r.require(t['current_runtime_class'] == row['runtime_class'], 'TRANSITION_CURRENT_CLASS', key)
        r.require(t['previous_runtime_class'] == (prior['runtime_class'] if prior else 'NOT_IN_P1309'), 'TRANSITION_PRIOR_CLASS', key)
        r.require(t['transition'] == expected_transition, 'HIDDEN_REGRESSION' if expected_transition=='REGRESSION_CANDIDATE' else 'TRANSITION_CLASS', [key, t['transition'], expected_transition])
    return r


def verify_principal(ledger, catalog, current):
    r = core.Review()
    lookup = {x['probe_id']: x for x in ledger}
    probes = {p['id']: p for p in catalog['probes']}
    grouped = {}
    for row in current:
        grouped.setdefault(row['id'], []).append(row)
    r.require(set(lookup) == set(probes) and len(lookup) == len(ledger), 'LEDGER_COVERAGE', len(ledger))
    for id_, entry in lookup.items():
        if id_ not in probes:
            continue
        p = probes[id_]
        rows = grouped[id_]
        r.require(entry['language_observable'] == ledger_display(p['expression']), 'LEDGER_EXPRESSION_BINDING', id_)
        r.require(entry['measurement_ref'].endswith('#'+id_), 'LEDGER_MEASUREMENT_BINDING', id_)
        reported = dict(v.split(':', 1) for v in entry['runtime_class_by_profile'].split(';'))
        r.require(reported == {x['profile']:x['runtime_class'] for x in rows}, 'LEDGER_RUNTIME_CLASSES', id_)
        if all(x['runtime_class'] in core.MATCH for x in rows):
            expected = 'EXPECTED_FEATURE_DISABLED' if p['path']=='html' else 'CLOSED_MEASURED_LOOKUP_REPR_ONLY'
            r.require(entry['current_language_class'] == expected, 'PRESENCE_ONLY_CLOSURE', [id_, entry['current_language_class'], expected])
        if entry['current_language_class'] == 'DOCUMENTED_PRODUCT_EXTENSION':
            evidence = entry.get('normative_evidence', '')
            file = evidence.split(':')[0]
            r.require(file.startswith('00_nucleo/prompts/') and (core.ROOT/file).is_file(),
                      'EXTENSION_WITHOUT_L0', [id_, evidence])
            r.require(p['path'] not in ('calc.deg','calc.log10','calc.rad'), 'UNRESOLVED_EXTENSION_PROMOTED', p['path'])
        prompt = entry['owner_prompt']
        r.require((core.ROOT/prompt).is_file() and core.digest(core.ROOT/prompt) == entry['owner_prompt_sha256'], 'LEDGER_OWNER_PIN', id_)
        consumer = core.ROOT/entry['consumer_unique']
        text = consumer.read_text() if consumer.is_file() else ''
        declared = re.findall(r'^\s*//[!/]?\s*@prompt\s+(\S+)\s*$', text, re.M)
        r.require(declared == [prompt], 'LEDGER_OWNER_LINK', [id_, declared, prompt])
        for field in ('causal_hypothesis', 'refutation', 'crystalline_owner_file_line', 'vanilla_source_file_line'):
            r.require(bool(entry.get(field)), 'LEDGER_CAUSAL_EVIDENCE_MISSING', [id_, field])
    return r


def verify_supplement(ledger, current):
    r = core.Review()
    grouped = {}
    for row in current:
        grouped.setdefault(row['id'], []).append(row)
    entries = {x['probe_id']:x for x in ledger}
    r.require(set(entries) == set(grouped) and len(entries)==len(ledger), 'FUNCTIONAL_LEDGER_COVERAGE', len(ledger))
    for id_, entry in entries.items():
        rows = grouped[id_]
        r.require(entry['language_observable'] == ledger_display(rows[0]['expression']), 'FUNCTIONAL_SOURCE_BINDING', id_)
        r.require(entry['measurement_ref'].endswith('#'+id_), 'NOMINAL_IDENTITY_CAPTURE', [id_, entry['measurement_ref']])
        closed = all(x['runtime_class'] in core.MATCH or x.get('language_projection')=='Preserved' for x in rows)
        if entry['current_language_class'] in ('CLOSED_MEASURED_FUNCTIONAL_SENTINEL', 'MATCH_PROJECTED_LANGUAGE_RAW_TRANSPORT_DIFFERENCE'):
            r.require(closed, 'FALSE_FUNCTIONAL_CLOSURE', id_)
    return r


def verify_selection(selection):
    r = core.Review()
    cohorts = selection['cohorts']
    candidates = []
    for c in cohorts:
        owners = set(c['owners'])
        r.require(c['owner_count'] == len(owners) and len(owners)==len(c['owners']), 'OWNER_COUNT', c['id'])
        r.require(MINIMUM_OWNERS.get(c['id'], set()) <= owners, 'OWNER_UNDERCOUNT',
                  [c['id'], sorted(MINIMUM_OWNERS.get(c['id'], set())-owners)])
        if not c.get('eligible'):
            continue
        r.require(bool(owners) and bool(c.get('paths')) and bool(c.get('witnesses')), 'ELIGIBLE_WITHOUT_EVIDENCE', c['id'])
        priority = c['priority']
        if c['id']=='namespace-function-missing-field':
            r.require(priority == 3, 'PRIORITY_L0_SCOPEOUT', c['id'])
        risk = c['regression_surface'].get('rank')
        r.require(risk is not None, 'UNKNOWN_RISK_PROMOTED', c['id'])
        rank = [priority, len(owners), -len(set(c['paths'])), risk if risk is not None else float('inf'), c['id']]
        if 'rank_key' in c:
            r.require(c['rank_key'] == rank, 'COHORT_RANK_KEY', [c['id'],c['rank_key'],rank])
        candidates.append((rank, c['id']))
    candidates.sort()
    selected = selection.get('selected')
    selected_id = selected.get('id') if isinstance(selected, dict) else selected
    if selection.get('blockers'):
        r.require(selected_id is None, 'SELECTION_DESPITE_BLOCKER', selected_id)
    else:
        r.require(selected_id == (candidates[0][1] if candidates else None), 'INVALID_PRIORITY_SELECTION', [selected_id, candidates[0][1] if candidates else None])
    return r


def run():
    r = core.preflight()
    catalog = core.read('p1322-probe-catalog.json')
    normal = core.read('p1322-matrix-normal.json')['results']
    previous = core.read('p1309-matrix-normal.json')['results']
    supplementary = core.read('p1322-sentinels-normal.json')['rows']
    selection = core.read('p1322-classification-selection.json')
    pieces = [verify_principal(table('p1322-classification-owner-ledger.tsv'), catalog, normal),
              verify_supplement(table('p1322-classification-supplemental-ledger.tsv'), supplementary),
              verify_transitions(table('p1322-classification-transitions.tsv'), normal, previous),
              verify_selection(selection)]
    for piece in pieces:
        core.merge_review(r, piece)
    return {'at': core.utc(), 'checker_sha256': core.digest(__file__), 'base_checker_sha256': core.digest(D/'p1322-review-check.py'),
            'plan_sha256': core.digest(D/'p1322-review-plan.md'), 'inputs': core.INPUTS, **r.result()}


if __name__ == '__main__':
    print(json.dumps(run(), ensure_ascii=False, indent=2))
