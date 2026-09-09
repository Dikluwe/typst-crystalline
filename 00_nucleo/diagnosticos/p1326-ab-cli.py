import argparse, datetime, hashlib, json, pathlib, re, subprocess, time

BASE = pathlib.Path('00_nucleo/diagnosticos')
MANIFEST = BASE / 'p1326-manifest.json'
BASELINE = '/tmp/p1325-target.KQl8cD/release/typst'
VANILLA = '/usr/local/bin/typst'
CASES = [
    ('named', '{let f(x)=x; f.nope}', 'closure'),
    ('anonymous', '(x => x).absent', 'closure'),
    ('alias', '{let f(x)=x; let alias=f; alias.unknown}', 'closure'),
    ('with', '{let f(x)=x; f.with(7).nope}', 'closure'),
    ('nested-with', '{let f(x,y)=x+y; f.with(7).with(8).missing-long-field}', 'closure'),
    ('unicode', '{let função(x)=x; função.café}', 'closure'),
    ('multiline', '{let f(x)=x;\n (\n f\n .ausência\n )}', 'closure'),
    ('callee', '{let f(x)=panic("must not run"); f.nope(7)}', 'closure'),
    ('with-callee', '{let f(x)=x; let alias=f.with(4); alias.nope()}', 'closure'),
    ('call', '{let f(x)=x+1; f(8)}', 'preserve'),
    ('with-call', '{let f(x,y)=x+y; let a=f.with(8); a(3)}', 'preserve'),
    ('nested-call', '{let f(x,y)=x+y; f.with(8).with(3)()}', 'preserve'),
    ('kind', '{let f(x)=x; type(f)==function}', 'preserve'),
    ('native-some', 'json.nope', 'preserve'),
    ('native-none', 'csv.nope', 'preserve'),
    ('native-present', 'json.encode((x: 1))', 'preserve'),
    ('module', 'calc.nope', 'preserve'),
    ('dict', '(x: 1).nope', 'preserve'),
    ('content', 'strong[x].nope', 'preserve'),
    ('float', '(1.0).nope', 'preserve'),
    ('pdf-gate', 'repr(type(pdf.data-cell))', 'preserve'),
    ('raw-content-residual', '[x].text', 'residual'),
    ('argument-order-residual', '{let f(x)=x; f.nope(panic("arg"))}', 'residual'),
]
PROFILES = {'default': [], 'html': ['--features', 'html'], 'a11y': ['--features', 'a11y-extras'], 'html+a11y': ['--features', 'html,a11y-extras']}
def utc(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(path): return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()
def command(argv):
    p = subprocess.run(argv, capture_output=True, text=True)
    return {'exit': p.returncode, 'stdout': p.stdout, 'stderr': p.stderr}
def save(name, data):
    path = BASE / name
    assert not path.exists(), path
    content = json.dumps(data, ensure_ascii=False, indent=2) + '\n'
    patch = f'*** Begin Patch\n*** Add File: {path}\n' + ''.join('+' + x + '\n' for x in content.splitlines()) + '*** End Patch\n'
    subprocess.run(['apply_patch', patch], check=True, capture_output=True)
    return sha(path)
def run(binary, expression, flags):
    argv = [binary, '--color', 'never', 'eval', '--format', 'json'] + flags + [expression]
    return {'argv': argv, 'at': utc(), 'output': command(argv)}
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('mode', choices=['baseline', 'final'])
    parser.add_argument('--candidate')
    parser.add_argument('--focal', action='store_true')
    args = parser.parse_args()
    start = time.monotonic()
    assert sha(MANIFEST) == '800db40f2300d0fda0275d06365c5f3019ca0848577ee54a9c834975b231146f'
    assert sha(BASELINE) == '3511b08aa89d088e908dd239d8942f1eeea1978d31140ef9510e81de06023dab'
    assert sha(VANILLA) == '7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'
    records = []
    data = {'started': utc(), 'manifest_sha256': sha(MANIFEST), 'head': command(['git','rev-parse','HEAD']), 'working_tree_stat': command(['git','diff','HEAD','--stat']), 'mode': args.mode, 'focal': args.focal, 'cases': CASES, 'profiles': PROFILES, 'binaries': {BASELINE: sha(BASELINE), VANILLA: sha(VANILLA)}, 'records': records}
    cases = [x for x in CASES if x[0] in ('named','unicode','callee','with-call','native-present','argument-order-residual')] if args.focal else CASES
    old = None
    if args.mode == 'final':
        assert args.candidate
        data['binaries'][args.candidate] = sha(args.candidate)
        old = json.loads((BASE / 'p1326-ab-baseline.json').read_text())
    orders = ['normal'] if args.mode == 'baseline' else ['normal','repeat','reverse']
    for order in orders:
        for profile, flags in PROFILES.items():
            for case_id, expression, policy in (list(reversed(cases)) if order == 'reverse' else cases):
                if args.mode == 'baseline':
                    baseline = run(BASELINE, expression, flags)
                    vanilla = run(VANILLA, expression, flags)
                    b, v = baseline['output'], vanilla['output']
                    if policy == 'closure':
                        valid = b['exit'] == v['exit'] == 1 and not b['stdout'] and not v['stdout'] and b['stderr'].startswith('error: cannot access fields on type function\n') and v['stderr'].startswith('error: cannot access fields on user-defined functions\n')
                        verdict = 'RequiredCorrection' if valid else 'Unknown'
                    elif policy == 'residual':
                        verdict = 'ResidualPreserveBaseline' if b != v and b['exit'] == 1 else 'Unknown'
                    else:
                        verdict = 'PreservedParity' if b == v else 'Unknown'
                    record = {'order': order, 'profile': profile, 'id': case_id, 'policy': policy, 'baseline': baseline, 'vanilla': vanilla, 'classification': verdict}
                else:
                    frozen = next(x for x in old['records'] if x['profile'] == profile and x['id'] == case_id)
                    candidate = run(args.candidate, expression, flags)
                    expected = frozen['vanilla' if policy == 'closure' else 'baseline']['output']
                    record = {'order': order, 'profile': profile, 'id': case_id, 'policy': policy, 'candidate': candidate, 'expected_source': 'vanilla' if policy == 'closure' else 'baseline', 'classification': 'Preserved' if candidate['output'] == expected else 'Violated'}
                records.append(record)
    data['finished'] = utc()
    data['elapsed_seconds'] = time.monotonic() - start
    data['counts'] = {v: sum(x['classification'] == v for x in records) for v in sorted(set(x['classification'] for x in records))}
    name = f'p1326-ab-{args.mode}' + ('-focal' if args.focal else '') + '.json'
    digest = save(name, data)
    print(json.dumps({'path': str(BASE/name), 'sha256': digest, 'counts': data['counts'], 'elapsed_seconds': data['elapsed_seconds']}))
    for record in records:
        if record['classification'] in ('Unknown','Violated'):
            print(json.dumps(record, ensure_ascii=False))
if __name__ == '__main__': main()
