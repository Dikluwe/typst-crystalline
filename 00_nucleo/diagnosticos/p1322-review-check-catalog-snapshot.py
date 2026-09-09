#!/usr/bin/env python3
"""P1322 D: independent audit of receipts; never imports the judged runner.

Rules frozen in p1322-review-plan.md. Data adapters are explicit, not oracles.
Only audit artifacts bearing p1322-review- may be written by this program.
"""
import argparse
import base64
import collections
import copy
import datetime
import hashlib
import json
import pathlib
import re
import subprocess
import sys
import time

ROOT = pathlib.Path(__file__).resolve().parents[2]
DIAG = ROOT / '00_nucleo/diagnosticos'
PROFILES = {'default': [], 'html': ['html'], 'a11y': ['a11y-extras'],
            'html+a11y': ['html', 'a11y-extras']}
MATCH = {'MATCH_VALUE', 'MATCH_DIAGNOSTIC'}
INPUTS = {}


def sha(value):
    return hashlib.sha256(value).hexdigest()


def digest(path):
    return sha(pathlib.Path(path).read_bytes())


def read(name):
    path = DIAG / name
    data = path.read_bytes()
    INPUTS[str(path)] = sha(data)
    return json.loads(data)


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


class Review:
    def __init__(self):
        self.checks = 0
        self.violations = []
        self.unknowns = []

    def require(self, condition, code, witness):
        self.checks += 1
        if not condition:
            self.violations.append({'code': code, 'witness': witness})

    def unknown(self, code, witness):
        self.unknowns.append({'code': code, 'witness': witness})

    def result(self):
        return {'verdict': 'Violated' if self.violations else
                'Unknown' if self.unknowns else 'Preserved',
                'checks': self.checks, 'violations': self.violations,
                'unknowns': self.unknowns}


def preflight():
    r = Review()
    baseline, manifest, build = (read('p1322-' + x + '.json')
                                 for x in ('baseline', 'manifest', 'build'))
    r.require(manifest['baseline_sha256'] == digest(DIAG/'p1322-baseline.json'),
              'BASELINE_PIN', 'p1322-manifest.json')
    r.require(build['manifest_sha256'] == digest(DIAG/'p1322-manifest.json'),
              'MANIFEST_PIN', 'p1322-build.json')
    r.require(build['exit'] == 0, 'BUILD_FAILED', build['exit'])
    r.require('--locked' in build['argv'] and '--release' in build['argv'],
              'BUILD_PROFILE', build['argv'])
    r.require(build['before']['product_inventory'] == baseline['state']['product_inventory'],
              'BUILD_BEFORE_STATE', 'product_inventory')
    r.require(build['after']['product_inventory'] == baseline['state']['product_inventory'],
              'BUILD_AFTER_STATE', 'product_inventory')
    for p, h in baseline['state']['product_inventory'].items():
        # Baseline product inventory is not a permit to open restricted history.
        r.require(not p.startswith(('00_nucleo/materialization/', '00_nucleo/context/')),
                  'RESTRICTED_PATH_IN_PRODUCT_INVENTORY', p)
        if p.startswith(('00_nucleo/materialization/', '00_nucleo/context/')):
            continue
        r.require((ROOT/p).is_file() and digest(ROOT/p) == h, 'PRODUCT_CHANGED', p)
    for side, spec in [('vanilla', baseline['vanilla']), ('crystalline', build['candidate'])]:
        r.require(pathlib.Path(spec['path']).is_file() and digest(spec['path']) == spec['sha256'],
                  'BINARY_PIN', side)
    r.require(baseline['vanilla']['sha256'] ==
              '7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8',
              'VANILLA_RATIFICATION', baseline['vanilla'])
    return r


def classify(left, right, require_json=True):
    """Reconstruct class from complete transcripts, preserving all stderr."""
    required = ('exit_code', 'stdout', 'stderr', 'complete')
    if any(any(k not in item for k in required) for item in (left, right)):
        return 'EXECUTION_UNKNOWN'
    if any(not x['complete'] or x.get('reason_code') or
           x['exit_code'] not in (0, 1) for x in (left, right)):
        return 'EXECUTION_UNKNOWN'
    values = []
    for x in (left, right):
        if x['exit_code'] == 0 and require_json:
            try:
                values.append(json.loads(x['stdout'], parse_constant=lambda value:
                                         (_ for _ in ()).throw(ValueError(value))))
            except (ValueError, TypeError):
                return 'EXECUTION_UNKNOWN'
        elif x['exit_code'] == 1:
            if not x['stderr'].strip():
                return 'EXECUTION_UNKNOWN'
            values.append(None)
    if left['exit_code'] == right['exit_code'] == 0:
        # Raw eval JSON is the frozen runtime projection; causal language
        # classification is a separate ledger, not inferred by this function.
        if left['stdout'] != right['stdout']:
            return 'DIFFERENT_VALUE'
        return 'MATCH_VALUE' if left['stderr'] == right['stderr'] else 'DIFFERENT_DIAGNOSTIC'
    if left['exit_code'] == 0:
        return 'VANILLA_ONLY'
    if right['exit_code'] == 0:
        return 'CRYSTALLINE_ONLY'
    return 'MATCH_DIAGNOSTIC' if all(left[x] == right[x] for x in
                                   ('exit_code', 'stdout', 'stderr')) else 'DIFFERENT_DIAGNOSTIC'


def verify_matrix(matrix, catalog, binaries, phase, trusted=None):
    r = Review()
    probes = {x['id']: x for x in catalog['probes']}
    required = {(id_, profile) for id_, p in probes.items()
                for profile in p.get('profiles', PROFILES)}
    results = matrix.get('results', [])
    actual = [(x.get('id'), x.get('profile')) for x in results]
    r.require(len(actual) == len(set(actual)), 'DUPLICATE_CELL', phase)
    r.require(set(actual) == required, 'CELL_COVERAGE',
              {'phase': phase, 'missing': sorted(required-set(actual)),
               'extra': sorted(set(actual)-required)})
    reconstructed = collections.Counter()
    by_key = {}
    for row in results:
        key = (row.get('id'), row.get('profile'))
        if key not in required:
            continue
        probe = probes[key[0]]
        for side in ('vanilla', 'crystalline'):
            item = row.get(side, {})
            for field in ('id', 'profile'):
                r.require(item.get(field) == row[field], 'CELL_IDENTITY', [key, side, field])
            r.require(item.get('phase') == phase, 'PHASE_IDENTITY', [key, side])
            r.require(item.get('features') == PROFILES[key[1]], 'PROFILE_FEATURES', [key, side])
            r.require(item.get('binary_path') == binaries[side]['path'] and
                      item.get('binary_sha256') == binaries[side]['sha256'],
                      'BINARY_IDENTITY', [key, side])
            argv = [binaries[side]['path'], 'eval', probe['expression'], '--format', 'json']
            if PROFILES[key[1]]:
                argv += ['--features', ','.join(PROFILES[key[1]])]
            r.require(item.get('argv') == argv, 'ARGV_IDENTITY', [key, side, item.get('argv'), argv])
            r.require(item.get('expression') == probe['expression'] and
                      item.get('source_sha256') == sha(probe['expression'].encode()),
                      'SOURCE_IDENTITY', [key, side])
            for field in ('stdout', 'stderr'):
                if field not in item:
                    r.unknown('MISSING_CHANNEL', [key, side, field])
                    continue
                encoded = item[field].encode()
                r.require(sha(encoded) == item.get(field+'_sha256'), 'CHANNEL_HASH', [key, side, field])
                r.require(base64.b64encode(encoded).decode() == item.get(field+'_base64'),
                          'CHANNEL_BASE64', [key, side, field])
            if trusted:
                original = trusted.get(key, {}).get(side, {})
                for field in ('stdout', 'stderr', 'exit_code', 'binary_sha256', 'features', 'argv'):
                    r.require(item.get(field) == original.get(field), 'TRANSCRIPT_ALTERED', [key, side, field])
        current = classify(row.get('vanilla', {}), row.get('crystalline', {}))
        r.require(row.get('runtime_class') == current, 'RUNTIME_CLASS', [key, row.get('runtime_class'), current])
        if current == 'EXECUTION_UNKNOWN':
            r.unknown('MANDATORY_EXECUTION_UNKNOWN', key)
        reconstructed[current] += 1
        by_key[key] = row
    r.require(dict(reconstructed) == {k: v for k, v in matrix.get('counts', {}).items() if v},
              'COUNTS', [dict(reconstructed), matrix.get('counts')])
    r.require(matrix.get('pairs') == len(required), 'DENOMINATOR', [matrix.get('pairs'), len(required)])
    r.require(matrix.get('probes') == len(probes), 'PROBE_DENOMINATOR', len(probes))
    return r, by_key, dict(reconstructed)


def verify_catalog(catalog, reconciliation, history, inventories):
    r = Review()
    probes = catalog.get('probes', [])
    current = {p['id']: p for p in probes}
    r.require(len(current) == len(probes), 'CATALOG_DUPLICATE_ID', len(probes))
    r.require(catalog.get('profiles') == PROFILES, 'CATALOG_PROFILES', catalog.get('profiles'))
    reconciled = {}
    for kind in ('unchanged', 'added', 'removed', 'renamed', 'split', 'merged'):
        for item in reconciliation.get(kind, []):
            id_ = item.get('probe_id')
            r.require(id_ not in reconciled, 'RECONCILIATION_DUPLICATE', id_)
            reconciled[id_] = kind
    for p in history['probes']:
        r.require(p['id'] in reconciled, 'HISTORICAL_ID_OMITTED', p['id'])
        if reconciled.get(p['id']) == 'unchanged':
            r.require(current.get(p['id']) == p, 'HISTORICAL_PROBE_ALTERED', p['id'])
    discovered = set().union(*(set(i['entries']) for i in inventories))
    paths = {p['path'] for p in probes}
    r.require(discovered <= paths, 'DISCOVERED_MEMBER_OMITTED', sorted(discovered-paths))
    r.require({'csv.encode', 'xml.encode', 'read.encode'} <= paths,
              'NEGATIVE_CONTROL_OMITTED', sorted({'csv.encode', 'xml.encode', 'read.encode'}-paths))
    if catalog.get('unknowns'):
        r.unknown('INVENTORY_UNKNOWN', catalog['unknowns'])
    return r


def runtime_review():
    r = preflight()
    catalog = read('p1322-probe-catalog.json')
    reconciliation = read('p1322-inventory-reconciliation.json')
    history = read('p1309-probe-catalog.json')
    inventories = []
    for source in catalog['inventory_inputs']:
        path = ROOT/source['path']
        r.require(digest(path) == source['sha256'], 'INVENTORY_PIN', str(path))
        inventories.append(read(path))
        obs_path = ROOT/source['language_observations']['path']
        r.require(digest(obs_path) == source['language_observations']['sha256'],
                  'OBSERVATIONS_PIN', str(obs_path))
        observed = read(obs_path)
        r.require(set(observed['entries']) == set(inventories[-1]['entries']),
                  'INVENTORY_OBSERVATION_COVERAGE', str(obs_path))
        for route, item in observed['entries'].items():
            execution = item['execution']
            if execution.get('unknown') or execution.get('exit_code') not in (0, 1):
                r.unknown('INVENTORY_OBSERVATION_UNKNOWN', [str(obs_path), route])
            r.require(execution.get('stdout') is not None and execution.get('stderr') is not None,
                      'INVENTORY_CHANNEL_MISSING', [str(obs_path), route])
    checks = verify_catalog(catalog, reconciliation, history, inventories)
    merge_review(r, checks)
    frozen = read('p1322-runtime-freeze.json')
    for p, expected in frozen['inputs'].items():
        r.require(digest(p) == expected, 'RUNTIME_INPUT_CHANGED', p)
    runs, counts = {}, {}
    for phase in ('normal', 'repeat', 'reverse'):
        path = DIAG/f'p1322-matrix-{phase}.json'
        if not path.exists():
            r.unknown('MATRIX_NOT_AVAILABLE', phase)
            continue
        matrix = read(path.name)
        r.require(matrix['catalog_sha256'] == digest(DIAG/'p1322-probe-catalog.json'),
                  'MATRIX_CATALOG_PIN', phase)
        r.require(matrix['freeze_sha256'] == digest(DIAG/'p1322-runtime-freeze.json'),
                  'MATRIX_FREEZE_PIN', phase)
        for moment in ('before', 'after'):
            r.require(matrix[moment]['product_inventory'] == read('p1322-baseline.json')['state']['product_inventory'],
                      'MATRIX_PRODUCT_CHANGED', [phase, moment])
        check, rows, counts[phase] = verify_matrix(matrix, catalog, frozen['binaries'], phase)
        merge_review(r, check)
        runs[phase] = rows
    if 'normal' in runs:
        for phase in ('repeat', 'reverse'):
            if phase not in runs:
                continue
            for key, row in runs['normal'].items():
                other = runs[phase].get(key)
                if not other:
                    continue
                r.require(row['runtime_class'] == other['runtime_class'], 'UNSTABLE_CLASS', [phase, key])
                for side in ('vanilla', 'crystalline'):
                    for field in ('exit_code', 'stdout_base64', 'stderr_base64', 'complete',
                                  'reason_code', 'source_sha256', 'binary_sha256', 'features'):
                        r.require(row[side][field] == other[side][field], 'UNSTABLE_OBSERVABLE', [phase, key, side, field])
    return r, {'counts': counts, 'catalog_paths': len({p['path'] for p in catalog['probes']}),
               'catalog_probes': len(catalog['probes'])}


def merge_review(target, source):
    target.checks += source.checks
    target.violations.extend(source.violations)
    target.unknowns.extend(source.unknowns)


def gate_review():
    r = preflight()
    summaries = {}
    baseline = read('p1322-baseline.json')['state']['product_inventory']
    for name in ('build', 'workspace-tests', 'fmt', 'lint', 'lineage', 'diff-check'):
        path = DIAG/f'p1322-{name}.json'
        if not path.exists():
            r.unknown('GATE_NOT_AVAILABLE', name)
            continue
        data = read(path)
        r.require(data.get('exit') == 0, 'GATE_FAILED', [name, data.get('exit')])
        r.require(data.get('manifest_sha256') == digest(DIAG/'p1322-manifest.json'),
                  'GATE_MANIFEST', name)
        for stage in ('before', 'after'):
            r.require(data[stage]['product_inventory'] == baseline, 'GATE_PRODUCT_CHANGED', [name, stage])
        summaries[name] = {'exit': data['exit'], 'argv': data['argv'], 'seconds': data['seconds']}
        if name == 'workspace-tests':
            matches = re.findall(r'test result: \w+\. (\d+) passed; (\d+) failed; (\d+) ignored;', data['stdout'])
            totals = [sum(int(t[i]) for t in matches) for i in range(3)]
            summaries[name]['test_totals'] = dict(zip(('passed', 'failed', 'ignored'), totals))
            summaries[name]['ignored_lines'] = [line for line in data['stdout'].splitlines() if line.endswith('... ignored')]
            r.require('--locked' in data['argv'] and '--release' in data['argv'], 'WORKSPACE_TEST_ARGS', data['argv'])
            r.require(bool(matches) and totals[1] == 0, 'WORKSPACE_TEST_RESULTS', totals)
        if name == 'lint':
            summaries[name]['error_lines'] = len(re.findall(r'^error:', data['stdout'], re.M))
            summaries[name]['warning_lines'] = len(re.findall(r'^warning:', data['stdout'], re.M))
            summaries[name]['info_lines'] = len(re.findall(r'^info:', data['stdout'], re.M))
        if name == 'lineage':
            r.require('v5,v15,v26' in data['argv'], 'LINEAGE_SCOPE', data['argv'])
    return r, {'gates': summaries}


def envelope_class(a, b):
    required = {'value': {'kind', 'value', 'stderr', 'exit'},
                'diagnostic': {'kind', 'exit', 'stdout', 'stderr', 'messages', 'hints'}}
    for value in (a, b):
        if not isinstance(value, dict) or value.get('kind') not in required or not required[value['kind']] <= value.keys():
            return 'Unknown'
    return 'Preserved' if a == b else 'Violated'


def supplement_review():
    r = preflight()
    frozen = read('p1322-sentinels-freeze.json')
    corpus = read('p1322-sentinels-cases.json')
    cases = {c['id']: c for c in corpus['cases']}
    expected = {(c['id'], profile) for c in cases.values() for profile in PROFILES
                if not c.get('observations') or profile in c['observations']}
    for path, h in frozen['inputs'].items():
        r.require(digest(path) == h, 'SUPPLEMENT_INPUT_CHANGED', path)
    matrices, summaries = {}, {}
    for phase in ('normal', 'repeat', 'reverse'):
        path = DIAG/f'p1322-sentinels-{phase}.json'
        if not path.exists():
            r.unknown('SUPPLEMENT_ORDER_NOT_AVAILABLE', phase)
            continue
        matrix = read(path)
        r.require(matrix['freeze_sha256'] == digest(DIAG/'p1322-sentinels-freeze.json'),
                  'SUPPLEMENT_FREEZE_PIN', phase)
        rows = {(row['id'], row['profile']): row for row in matrix['rows']}
        r.require(set(rows) == expected and len(rows) == len(matrix['rows']), 'SUPPLEMENT_COVERAGE', phase)
        counts = collections.Counter()
        for key, row in rows.items():
            case = cases[key[0]]
            r.require(row['universe'] == 'supplement', 'SUPPLEMENT_UNIVERSE', key)
            for side in ('vanilla', 'crystalline'):
                item = row[side]
                binary = frozen['binaries'][side]
                r.require(item['binary_path'] == binary['path'] and item['binary_sha256'] == binary['sha256'],
                          'SUPPLEMENT_BINARY', [key, side])
                r.require(item['source_sha256'] == case['source_sha256'] == sha(case.get('document', case['expression']).encode()),
                          'SUPPLEMENT_SOURCE', [key, side])
                r.require(item['cwd'] == case['cwd'] and item['features'] == PROFILES[key[1]],
                          'SUPPLEMENT_ENVIRONMENT', [key, side])
                argv = [binary['path'], '--color=never', case['route']]
                if case['route'] == 'eval':
                    argv += [case['expression'], '--format', 'json']
                else:
                    outfile = pathlib.Path(corpus['temp'])/f"out-{sha(case['id'].encode())[:16]}-{key[1]}-{phase}-{side}.pdf"
                    argv += [case['fixture'], str(outfile), '--format', 'pdf']
                if PROFILES[key[1]]:
                    argv += ['--features', ','.join(PROFILES[key[1]])]
                r.require(item['argv'] == argv, 'SUPPLEMENT_ARGV', [key, side])
                for channel in ('stdout', 'stderr'):
                    raw = base64.b64decode(item[channel+'_base64'], validate=True)
                    r.require(sha(raw) == item[channel+'_sha256'] and raw.decode(errors='replace') == item[channel],
                              'SUPPLEMENT_CHANNEL', [key, side, channel])
            calculated = classify(row['vanilla'], row['crystalline'], require_json=case['route']=='eval')
            r.require(row['runtime_class'] == calculated, 'SUPPLEMENT_CLASS', [key, row['runtime_class'], calculated])
            if calculated == 'EXECUTION_UNKNOWN':
                r.unknown('SUPPLEMENT_EXECUTION_UNKNOWN', key)
            counts[calculated] += 1
            if case.get('legacy_projection'):
                left, right = (row[s]['observable'] for s in ('vanilla', 'crystalline'))
                r.require(row['historical_preservation'] == envelope_class(case['observations'][key[1]]['future_expected'], right),
                          'PRESERVATION_CLASS', key)
                r.require(row['language_projection'] == envelope_class(left, right), 'PROJECTION_CLASS', key)
                if row['historical_preservation'] == 'Unknown' or row['language_projection'] == 'Unknown':
                    r.unknown('SUPPLEMENT_PROJECTION_UNKNOWN', key)
        r.require(dict(counts) == matrix['counts'], 'SUPPLEMENT_COUNTS', phase)
        summaries[phase] = dict(counts)
        matrices[phase] = rows
    if 'normal' in matrices:
        for phase in ('repeat', 'reverse'):
            if phase not in matrices:
                continue
            for key, row in matrices['normal'].items():
                other = matrices[phase].get(key)
                if not other:
                    continue
                for side in ('vanilla', 'crystalline'):
                    for field in ('exit_code', 'stdout', 'stderr', 'complete', 'reason_code', 'observable'):
                        r.require(row[side].get(field) == other[side].get(field), 'SUPPLEMENT_UNSTABLE', [phase, key, side, field])
    return r, {'supplement_counts': summaries, 'supplement_cases': len(cases),
               'supplement_pairs_per_order': len(expected),
               'scope_limit': 'Raw outputs, identity, counts and envelope comparisons checked; legacy envelope extraction requires separate semantic review.'}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('mode', choices=['preflight', 'runtime', 'gates', 'supplement'])
    parser.add_argument('--output')
    args = parser.parse_args()
    started = utc()
    t0 = time.monotonic()
    r, extra = {'preflight': lambda: (preflight(), {}), 'runtime': runtime_review,
                'gates': gate_review, 'supplement': supplement_review}[args.mode]()
    receipt = {'role': 'D', 'isolation': 'not technically attested', 'start': started,
               'end': utc(), 'seconds': time.monotonic()-t0,
               'checker_sha256': digest(__file__), 'plan_sha256': digest(DIAG/'p1322-review-plan.md'),
               'inputs': INPUTS,
               'scope': args.mode, **extra, **r.result()}
    payload = json.dumps(receipt, indent=2, ensure_ascii=False)+'\n'
    if args.output:
        path = pathlib.Path(args.output).resolve()
        if path.parent != DIAG or not path.name.startswith('p1322-review-') or path.exists():
            raise RuntimeError('Reviewer outputs must be new p1322-review-* artifacts')
        patch = '*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join(
            '+'+line+'\n' for line in payload.splitlines())+'*** End Patch\n'
        subprocess.run(['apply_patch'], input=patch, text=True, check=True, capture_output=True)
    else:
        print(payload)
    return 1 if r.violations else 2 if r.unknowns else 0


if __name__ == '__main__':
    sys.exit(main())
