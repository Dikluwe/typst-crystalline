"""Frozen independent P1324 observable runner; never reads production source.

Writes no artifacts itself: stdout is a JSON receipt, retained by apply_patch.
All observable channels are compared exactly, without text normalization.
"""
import argparse
import copy
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time

ROOT = Path('/repos/Antigravity/typst-crystalline')
HERE = ROOT / '00_nucleo/diagnosticos'
CASES = HERE / 'p1324-ab-cases.json'
MANIFEST = HERE / 'p1324-manifest.json'
FROZEN = HERE / 'p1324-ab-freeze-v2.json'


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def observed(run):
    if not isinstance(run, dict) or run.get('execution') != 'complete':
        return None
    if not all(k in run for k in ('exit', 'stdout', 'stderr')):
        return None
    if not isinstance(run['exit'], int) or run['exit'] not in (0, 1):
        return None
    return {k: run[k] for k in ('exit', 'stdout', 'stderr')}


def compare(reference, result):
    expected, actual = observed(reference), observed(result)
    if expected is None or actual is None:
        return {'status': 'Unknown', 'reason': 'missing_or_incomplete_observable'}
    differences = [key for key in expected if expected[key] != actual[key]]
    return {'status': 'Violated' if differences else 'Preserved', 'differences': differences}


def collect(binary, case, profile, features, order, env):
    argv = [binary, 'eval', '--format', 'json']
    if features:
        argv += ['--features', ','.join(features)]
    argv += [case['expr']]
    record = {'case': case['id'], 'profile': profile, 'order': order, 'argv': argv, 'started': now()}
    start = time.monotonic()
    try:
        process = subprocess.run(argv, cwd=ROOT, env=env, capture_output=True, timeout=20)
        record.update(execution='complete', exit=process.returncode,
                      stdout=process.stdout.decode('utf-8'), stderr=process.stderr.decode('utf-8'))
    except (OSError, subprocess.TimeoutExpired, UnicodeError) as error:
        record.update(execution='Unknown', failure=str(error))
    record.update(seconds=time.monotonic()-start, ended=now())
    return record


def calibration(runs):
    diagnostic = next(r for r in runs if r['case'] == 'yaml-call-trace' and r['profile'] == 'default')
    positive = next(r for r in runs if r['case'] == 'json-present-call' and r['profile'] == 'default')
    samples = []
    def check(name, reference, mutant, expected):
        result = compare(reference, mutant)
        samples.append({'name': name, 'expected': expected, 'result': result, 'mutant': mutant})
    check('unchanged-error', diagnostic, copy.deepcopy(diagnostic), 'Preserved')
    check('unchanged-success', positive, copy.deepcopy(positive), 'Preserved')
    transforms = {
        'wrong-function-name': lambda s: s.replace('function `yaml`', 'function `json`', 1),
        'wrong-field-name': lambda s: s.replace('field `missing_xyz`', 'field `absent`', 1),
        'legacy-message': lambda s: s.replace('function `yaml` does not contain field `missing_xyz`', 'function does not contain field "missing_xyz"', 1),
        'wrong-line': lambda s: s.replace('<input-expression>:1:21', '<input-expression>:2:21', 1),
        'wrong-column': lambda s: s.replace('<input-expression>:1:21', '<input-expression>:1:20', 1),
        'wrong-caret-width': lambda s: s.replace('^^^^^^^^^^^', '^^^^^^^^^^^^', 1),
        'wrong-severity': lambda s: s.replace('error:', 'warning:', 1),
        'new-hint': lambda s: s + 'hint: manufactured recovery\n',
        'new-diagnostic': lambda s: s + 'error: unrelated\n',
        'trace-removed': lambda s: s.split('  while calling ')[0],
        'trace-name-changed': lambda s: s.replace('while calling `relay`', 'while calling `impostor`', 1),
    }
    for name, transform in transforms.items():
        mutant = copy.deepcopy(diagnostic)
        mutant['stderr'] = transform(mutant['stderr'])
        assert mutant['stderr'] != diagnostic['stderr'], ('invalid_mutation', name)
        check(name, diagnostic, mutant, 'Violated')
    for name, ref, key, value in [
        ('error-exit-success', diagnostic, 'exit', 0),
        ('error-stdout-leak', diagnostic, 'stdout', 'invented\n'),
        ('success-value-corrupted', positive, 'stdout', '["function","encode","43"]\n'),
        ('success-extra-warning', positive, 'stderr', 'warning: new\n'),
    ]:
        mutant = copy.deepcopy(ref)
        mutant[key] = value
        assert mutant[key] != ref[key], ('invalid_mutation', name)
        check(name, ref, mutant, 'Violated')
    opaque = copy.deepcopy(diagnostic)
    del opaque['stderr']
    check('opaque-missing-diagnostic-channel', diagnostic, opaque, 'Unknown')
    return {'samples': samples, 'valid_mutations': 15, 'rejected': sum(s['result']['status'] == 'Violated' for s in samples),
            'all_expected': all(s['expected'] == s['result']['status'] for s in samples),
            'processes': 0, 'scope': 'copied public observables only; not source mutants; no refinement seal'}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--binary', required=True)
    parser.add_argument('--label', choices=['oracle', 'baseline', 'candidate'], required=True)
    parser.add_argument('--oracle')
    parser.add_argument('--baseline')
    parser.add_argument('--orders', default='normal')
    parser.add_argument('--focal', action='store_true')
    options = parser.parse_args()
    spec = json.loads(CASES.read_text())
    if options.label != 'oracle':
        frozen = json.loads(FROZEN.read_text())
        for artifact in frozen['protected']:
            assert sha(ROOT / artifact['path']) == artifact['sha256'], ('freeze_drift', artifact['path'])
    oracle = json.loads(Path(options.oracle).read_text()) if options.oracle else None
    baseline = json.loads(Path(options.baseline).read_text()) if options.baseline else None
    lookup = lambda receipt: {(r['case'], r['profile']): r for r in receipt['runs']} if receipt else {}
    oracle_by_key, baseline_by_key = lookup(oracle), lookup(baseline)
    environment = {key:value for key,value in os.environ.items() if not key.startswith('TYPST_')}
    environment.update(LANG='C.UTF-8', LC_ALL='C.UTF-8', NO_COLOR='1', TERM='dumb')
    start = time.monotonic()
    receipt = {'started': now(), 'label': options.label, 'binary': options.binary,
               'binary_sha256': sha(options.binary), 'manifest_sha256': sha(MANIFEST),
               'case_sha256': sha(CASES), 'runner_sha256': sha(__file__),
               'cwd': str(ROOT), 'command': list(os.sys.argv),
               'environment_policy': 'Inherited environment minus all TYPST_*; LANG/LC_ALL=C.UTF-8 NO_COLOR=1 TERM=dumb',
               'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
               'diff_stat': subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], cwd=ROOT, text=True),
               'status': subprocess.check_output(['git', 'status', '--short'], cwd=ROOT, text=True), 'runs': []}
    cases = [case for case in spec['cases'] if not options.focal or case.get('focal')]
    first = {}
    for order in options.orders.split(','):
        assert order in ('normal', 'repeat', 'reverse')
        for profile, features in spec['profiles'].items():
            for case in reversed(cases) if order == 'reverse' else cases:
                result = collect(options.binary, case, profile, features, order, environment)
                key = (case['id'], profile)
                expected = case['expect']
                if expected == 'feature':
                    expected = 'success' if 'a11y-extras' in features else 'error'
                obs = observed(result)
                result['shape_expected'] = expected
                result['shape_ok'] = obs is not None and (obs['exit'] == 0 if expected == 'success' else obs['exit'] == 1)
                if options.label != 'oracle':
                    result['vs_vanilla'] = compare(oracle_by_key.get(key), result)
                if options.label == 'candidate':
                    reference = oracle_by_key.get(key) if case['rule'] == 'vanilla' else baseline_by_key.get(key)
                    result['obligation'] = compare(reference, result)
                    result['vs_baseline'] = compare(baseline_by_key.get(key), result)
                if key in first:
                    result['stability'] = compare(first[key], result)
                else:
                    first[key] = result
                receipt['runs'].append(result)
    receipt.update(ended=now(), seconds=time.monotonic()-start, processes=len(receipt['runs']))
    if options.label == 'oracle' and not options.focal:
        receipt['calibration'] = calibration(receipt['runs'])
    receipt['counts'] = {}
    for metric in ('obligation', 'vs_vanilla', 'vs_baseline', 'stability'):
        counts = {}
        for run in receipt['runs']:
            if metric in run:
                status = run[metric]['status']
                counts[status] = counts.get(status, 0) + 1
        receipt['counts'][metric] = counts
    receipt['unexpected_shape'] = [r['case'] + '/' + r['profile'] for r in receipt['runs'] if not r['shape_ok']]
    print(json.dumps(receipt, ensure_ascii=False, indent=2))


if __name__ == '__main__':
    main()
