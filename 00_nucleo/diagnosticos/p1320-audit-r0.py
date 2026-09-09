"""Read-only product audit. New immutable receipts; no general parity claim."""
import argparse
from collections import Counter
from concurrent.futures import ThreadPoolExecutor
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / '00_nucleo/diagnosticos'
FIX = D / 'p1320-fixtures'
BINARIES = {
    'crystalline': ('/tmp/p1319-target.VqXtmj/release/typst',
        '37a8a23d6e2b5daf355d90d510bca7efb399547730e1d807e8ae871be75748bd'),
    'vanilla': ('/usr/local/bin/typst',
        '7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'),
}

def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def sha(path):
    with open(path, 'rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, text=True)

def inputs():
    paths = git('ls-files', '00_nucleo/prompts', '01_core', '02_shell',
                '03_infra', '04_wiring', 'Cargo.toml', 'Cargo.lock',
                'lab/parity/matrix', 'lab/typst-original/crates/typst-library/src/loading',
                'lab/typst-original/crates/typst-library/src/diag.rs').split('\n')
    paths += [str(p.relative_to(ROOT)) for p in FIX.rglob('*') if p.is_file()]
    paths += [str(Path(__file__).relative_to(ROOT))]
    paths += [str(p.relative_to(ROOT)) for p in D.glob('p1319-*') if p.is_file()]
    return {p: sha(ROOT / p) for p in sorted(set(paths)) if p and (ROOT / p).is_file()}

def snapshot():
    return dict(utc=utc(), head=git('rev-parse', 'HEAD').strip(),
        diff_stat=git('diff', 'HEAD', '--stat'), diff=git('diff', 'HEAD'),
        staged=git('diff', '--cached'), status=git('status', '--short'),
        inputs=inputs(), binaries={k: dict(path=v[0], sha256=sha(v[0]))
                                  for k, v in BINARIES.items()})

def save(path, value):
    if path.exists():
        raise FileExistsError(path)
    payload = json.dumps(value, ensure_ascii=True, indent=2) + '\n'
    patch = '*** Begin Patch\n*** Add File: ' + str(path) + '\n'
    patch += ''.join('+' + s + '\n' for s in payload.rstrip('\n').split('\n'))
    subprocess.run(['apply_patch'], input=patch + '*** End Patch\n', text=True,
                   capture_output=True, check=True)

def invoke(argv, cwd=ROOT):
    start = utc()
    env = os.environ.copy()
    env.update(LC_ALL='C.UTF-8', NO_COLOR='1', PYTHONDONTWRITEBYTECODE='1')
    try:
        p = subprocess.run(argv, cwd=cwd, env=env, capture_output=True, timeout=180)
        return dict(argv=list(map(str, argv)), cwd=str(cwd), start=start, end=utc(),
                    exit=p.returncode, stdout=p.stdout.decode('utf-8'),
                    stderr=p.stderr.decode('utf-8'))
    except (OSError, subprocess.TimeoutExpired, UnicodeError) as error:
        return dict(argv=list(map(str, argv)), cwd=str(cwd), start=start, end=utc(),
                    exit=None, stdout='', stderr=str(error), unknown=True)

def typed(value):
    if isinstance(value, dict):
        return ('dict', tuple((k, typed(v)) for k, v in sorted(value.items())))
    if isinstance(value, list):
        return ('array', tuple(map(typed, value)))
    return (type(value).__name__, value)

def classify(left, right, observation):
    if left.get('unknown') or right.get('unknown'):
        return 'UNKNOWN'
    if observation == 'value' and left['exit'] != 0:
        return 'UNKNOWN_BASELINE'
    if left['exit'] != right['exit']:
        return 'DIFFERENCE'
    if left['exit'] == 0:
        if observation == 'compile_assertions':
            return 'MATCH_ASSERTIONS_ONLY'
        try:
            equal = typed(json.loads(left['stdout'])) == typed(json.loads(right['stdout']))
        except (ValueError, TypeError):
            return 'UNKNOWN'
        return 'MATCH_VALUE' if equal else 'DIFFERENCE'
    equal = (left['stdout'], left['stderr']) == (right['stdout'], right['stderr'])
    return 'MATCH_DIAGNOSTIC' if equal else 'DIFFERENCE'

def cases():
    rows = []
    def add(name, expr, observation='value', family='csv'):
        rows.append(dict(id=name, expression=expr, observation=observation, family=family))
    for mode in ['array', 'dictionary']:
        for route, source in [('str', '"good.csv"'), ('path', 'path("good.csv")'),
                              ('bytes', 'bytes("a,b\\n1,2\\n")')]:
            add(f'csv-good-{mode}-{route}', f'csv({source}, row-type: {mode})')
        for route, source in [('str', '"unequal.csv"'), ('path', 'path("unequal.csv")'),
                              ('bytes', 'bytes("a,b\\n1\\n")')]:
            add(f'csv-unequal-{mode}-{route}', f'csv({source}, row-type: {mode})', 'diagnostic')
    for name, expr in [
        ('empty', 'csv(bytes(()))'),
        ('duplicate-headers', 'csv(bytes("a,a\\n1,2"), row-type: dictionary)'),
        ('quoted-multiline', 'csv(bytes("a,b\\n\\"one\\ntwo\\",3"))'),
        ('symbol-delimiter', 'csv(bytes("a,b"), delimiter: sym.comma)'),
        ('symbol-source', 'csv(sym.comma)'),
        ('spread', '{let a=arguments("good.csv", row-type: dictionary); csv(..a)}'),
        ('with', '{let f=csv.with("good.csv"); f()}'),
        ('subdir-path', 'csv(path("sub/./data.csv"))'),
        ('captured-eval', '{import "sub/module.typ": captured; csv(captured)}'),
        ('closure-eval', '{import "sub/module.typ": load; load()}'),
    ]:
        add('csv-' + name, expr)
    for name, expr in [
        ('missing-source', 'csv()'),
        ('unknown-option', 'csv(bytes("a,b"), nope: 1)'),
        ('extra-valid', 'csv(bytes("a,b"), 42)'),
        ('extra-bad', 'csv(bytes("a,b\\n1"), 42)'),
        ('missing-file', 'csv("missing.csv")'),
        ('missing-file-path', 'csv(path("missing.csv"))'),
        ('source-cast', 'csv(42)'),
        ('delimiter-cast', 'csv(bytes("a,b"), delimiter: 42)'),
        ('delimiter-long', 'csv(bytes("a,b"), delimiter: "xx")'),
        ('row-type-invalid', 'csv(bytes("a,b"), row-type: int)'),
        ('options-precedence', 'csv(bytes("a,b"), delimiter: "xx", row-type: int)'),
        ('unknown-before-source', 'csv(42, nope: 1)'),
        ('unknown-before-missing', 'csv(nope: 1)'),
        ('detached-good', 'csv(..arguments(0).map((..a) => "good.csv"))'),
        ('binary-bytes', 'csv(bytes((97,44,98,10,49,44,255)))'),
        ('encode-member', 'csv.encode'),
    ]:
        add('csv-' + name, expr, 'diagnostic')
    for name, expr in [
        ('sys-version', 'repr(sys.version)'),
        ('content-heading-body', 'heading([a]).body == [a]'),
        ('content-strong', 'strong([a]).body == [a]'),
        ('numeric', '(1 == 1.0, calc.gcd(12, 18), calc.pow(2, 10))'),
        ('bytes-roundtrip', 'array(bytes((0, 127, 255)))'),
        ('json-roundtrip', 'json(json.encode((a: (1, true, none))))'),
        ('yaml-load', 'yaml(bytes("a: 1\\nb: [true, false]"))'),
        ('toml-load', 'toml(bytes("a = 1\\nb = true"))'),
        ('path-normalized', 'repr(path("sub/../good.csv"))'),
        ('html-binding', 'type(html.elem)'),
        ('a11y-binding', 'type(pdf.table-summary)'),
    ]:
        add(name, expr, 'value', 'transversal')
    return rows

class ClassifierTests(unittest.TestCase):
    def row(self, stdout='1', exit=0, stderr=''):
        return dict(stdout=stdout, exit=exit, stderr=stderr)
    def test_bool_is_not_integer(self):
        self.assertEqual(classify(self.row('true'), self.row('1'), 'value'), 'DIFFERENCE')
    def test_dict_order_is_not_semantics(self):
        self.assertEqual(classify(self.row('{"a":1,"b":2}'), self.row('{"b":2,"a":1}'), 'value'), 'MATCH_VALUE')
    def test_diagnostic_keeps_span_and_unicode(self):
        self.assertEqual(classify(self.row('', 1, 'a\u2028b'), self.row('', 1, 'a\nb'), 'diagnostic'), 'DIFFERENCE')
    def test_both_fail_not_value_success(self):
        self.assertEqual(classify(self.row('', 1), self.row('', 1), 'value'), 'UNKNOWN_BASELINE')
    def test_invalid_json_is_unknown(self):
        self.assertEqual(classify(self.row('bad'), self.row('bad'), 'value'), 'UNKNOWN')
    def test_timeout_is_unknown(self):
        self.assertEqual(classify(dict(unknown=True), self.row(), 'value'), 'UNKNOWN')

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--output', required=True, type=Path)
    parser.add_argument('--self-test', action='store_true')
    args = parser.parse_args()
    if args.output.exists():
        raise FileExistsError(args.output)
    before = snapshot()
    assert all(before['binaries'][k]['sha256'] == v[1] for k, v in BINARIES.items())
    if args.self_test:
        tests = [invoke([sys.executable, str(Path(__file__)), 'test-classifier']),
                 invoke([sys.executable, '-m', 'unittest', 'lab/parity/matrix/test_runner.py'])]
        save(args.output, dict(schema='p1320-tests/v1', before=before,
                              tests=tests, after=snapshot()))
        return 0 if all(x['exit'] == 0 for x in tests) else 1
    tmp = Path(tempfile.mkdtemp(prefix='p1320-audit-', dir='/tmp'))
    profiles = {'default': [], 'html': ['html'], 'a11y-extras': ['a11y-extras'],
                'html+a11y': ['html', 'a11y-extras']}
    def run_one(item):
        case, profile, features = item
        observations = {}
        for side, (binary, _) in BINARIES.items():
            command = [binary, 'eval', case['expression'], '--format', 'json', '--color=never']
            if features:
                command += ['--features', ','.join(features)]
            observations[side] = invoke(command, FIX)
        return dict(case=case, profile=profile, **observations,
                    state=classify(observations['vanilla'], observations['crystalline'], case['observation']))
    corpus = cases()
    work = [(case, profile, features) for case in corpus for profile, features in profiles.items()]
    with ThreadPoolExecutor(max_workers=4) as pool:
        focal = list(pool.map(run_one, work))
    print('Focal:', dict(Counter(r['state'] for r in focal)), flush=True)
    compile_rows = {}
    for side, (binary, _) in BINARIES.items():
        compile_rows[side] = invoke([binary, 'compile', str(FIX / 'captured.typ'),
                                    str(tmp / (side + '-captured.svg')), '--format', 'svg', '--color=never'], FIX)
    matrix = []
    for profile in ['default', 'html', 'a11y-extras']:
        output = tmp / (profile + '.json')
        run = invoke([sys.executable, str(ROOT / 'lab/parity/matrix/runner.py'),
                      '--vanilla', BINARIES['vanilla'][0], '--crystalline', BINARIES['crystalline'][0],
                      '--profile', profile, '--output', str(output)])
        value = json.loads(output.read_text()) if output.exists() else None
        matrix.append(dict(profile=profile, run=run, result=value))
        print('Matrix:', profile, value['counts'] if value else 'UNKNOWN', flush=True)
    after = snapshot()
    stable = before['inputs'] == after['inputs'] and before['binaries'] == after['binaries']
    artifacts = {str(p): sha(p) for p in tmp.rglob('*') if p.is_file()}
    receipt = dict(schema='p1320-parity-audit/v1', upstream='a51e02804', before=before,
        after=after, unchanged=stable, cases=corpus, profiles=profiles, focal=focal,
        focal_counts=dict(Counter(r['state'] for r in focal)), captured_compile=compile_rows,
        captured_state=classify(compile_rows['vanilla'], compile_rows['crystalline'], 'compile_assertions'),
        matrix=matrix, artifacts=artifacts, temp=str(tmp),
        limitations=['diagnostic audit, not independent verification',
                      'sample counts are not global parity', 'matrix profiles do not inject features into unrelated cases'])
    save(args.output, receipt)
    print(args.output, sha(args.output), flush=True)
    return 0 if stable else 2

if __name__ == '__main__':
    if sys.argv[1:] == ['test-classifier']:
        unittest.main(argv=[sys.argv[0]])
    else:
        sys.exit(main())
