#!/usr/bin/env python3
"""Frozen P1325 bilateral public CLI measurement; never reads product source."""
import argparse, datetime, hashlib, json, pathlib, subprocess

BASE = '/tmp/p1324-target.x5jDkw/release/typst'
VANILLA = '/usr/local/bin/typst'
CASES = [
    ('dict-direct', 'scope', '(x: 1).nope'),
    ('dict-alias', 'scope', '{ let d = (x: 1); let alias = d; alias.absent }'),
    ('dict-multiline', 'scope', '{\n let d = (x: 1)\n (d).long-missing-field\n}'),
    ('dict-unicode', 'scope', '{ let d = (x: 1); d.ausência }'),
    ('dict-empty', 'scope', '(:).missing'),
    ('content-direct', 'scope', '[x].nope'),
    ('strong-direct', 'scope', 'strong[x].absent'),
    ('content-alias-unicode', 'scope', '{\n let c = strong[x]\n let alias = c\n alias.ausência\n}'),
    ('content-lookup-residual', 'lookup-residual', '[x].text'),
    ('float-direct', 'scope', '(1.0).nope'),
    ('float-alias', 'scope', '{ let f = 1.0; f.is-infinite }'),
    ('float-unicode', 'scope', '{\n let f = 1.0\n f.ausência\n}'),
    ('float-is-nan', 'preserve', '(1.0).is-nan'),
    ('dict-value', 'preserve', '(x: 1).x'),
    ('dict-unicode-value', 'preserve', '(ausência: 17).ausência'),
    ('dict-nested-value', 'preserve', '(x: (y: 4)).x.y'),
    ('content-morphology', 'preserve', 'strong[x].body == [x]'),
    ('float-call-value', 'preserve', '(1.0).is-nan()'),
    ('dict-callee', 'preserve', '(x: 1).nope(2)'),
    ('content-callee', 'preserve', 'strong[x].nope(2)'),
    ('float-callee', 'preserve', '(1.0).nope(2)'),
    ('integer-boundary', 'preserve', '(1).nope'),
    ('string-boundary', 'preserve', '"abc".nope'),
    ('closure-boundary', 'preserve', '{ let f = x => x; f.nope }'),
    ('native-boundary', 'preserve', 'json.nope'),
    ('module-boundary', 'preserve', 'calc.nope'),
    ('pdf-gate', 'preserve', 'type(pdf.data-cell) == function'),
    ('receiver-first', 'preserve', '{ let d = panic("receiver-first"); d.nope }'),
]
PROFILES = [('default', []), ('html', ['--features', 'html']), ('a11y', ['--features', 'a11y-extras']), ('html+a11y', ['--features', 'html,a11y-extras'])]

def sha(path):
    return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()

def run(argv):
    started = datetime.datetime.now(datetime.timezone.utc).isoformat()
    p = subprocess.run(argv, capture_output=True, text=True, timeout=30)
    return dict(argv=argv, at=started, exit=p.returncode, stdout=p.stdout, stderr=p.stderr)

def observable(row):
    return (row['exit'], row['stdout'], row['stderr'])

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--candidate')
    parser.add_argument('--focal', action='store_true')
    parser.add_argument('--output', required=True)
    args = parser.parse_args()
    binaries = {'baseline': BASE, 'vanilla': VANILLA}
    if args.candidate: binaries['candidate'] = args.candidate
    cases = CASES[:1] + CASES[5:6] + CASES[9:10] + CASES[13:14] if args.focal else CASES
    profiles = PROFILES[:1] if args.focal else PROFILES
    receipt = dict(protocol='A/B without technical isolation attestation; no refinement seal', at=datetime.datetime.now(datetime.timezone.utc).isoformat(), head=run(['git', 'rev-parse', 'HEAD']), diffstat=run(['git', 'diff', 'HEAD', '--stat']), untracked=run(['git', 'ls-files', '--others', '--exclude-standard', '00_nucleo/diagnosticos/p1325-ab-*']), binaries={k: dict(path=v, sha256=sha(v)) for k,v in binaries.items()}, inputs={p:sha(p) for p in ['00_nucleo/diagnosticos/p1325-ab-runner.py', '00_nucleo/diagnosticos/p1325-ab-tests.rs', '00_nucleo/diagnosticos/p1325-manifest.json']}, cases=cases, rows=[])
    orders = ['normal', 'repeat', 'reverse'] if args.candidate else ['normal']
    for order in orders:
        for profile, flags in profiles:
            for name, category, expression in (reversed(cases) if order == 'reverse' else cases):
                for product, binary in binaries.items():
                    result = run([binary, '--color', 'never', 'eval', '--format', 'json', *flags, expression])
                    receipt['rows'].append(dict(order=order, profile=profile, case=name, category=category, product=product, **result))
    receipt['final_test_snippet'] = dict(path='00_nucleo/diagnosticos/p1325-ab-tests-r1.rs', sha256=sha('00_nucleo/diagnosticos/p1325-ab-tests-r1.rs'))
    path = pathlib.Path(args.output)
    path.write_text(json.dumps(receipt, ensure_ascii=False, indent=2) + '\n')
    print(json.dumps(dict(path=str(path), sha256=sha(path), rows=len(receipt['rows']))))

if __name__ == '__main__': main()
