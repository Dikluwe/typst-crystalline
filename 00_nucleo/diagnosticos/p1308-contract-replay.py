#!/usr/bin/env python3
"""Read-only replay of the independently frozen P1308 literal contract."""
import argparse, concurrent.futures, datetime, hashlib, json, os, pathlib, re, subprocess

ROOT = pathlib.Path(__file__).resolve().parents[2]
MEASURE = ROOT / '00_nucleo/diagnosticos/p1308-measure.json'
sha = lambda p: hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
assert sha(MEASURE) == 'ce758b5c2a18288bf9c8433178f577b50c52df80cedcddcb1f4daa4e785573fc'
m = json.loads(MEASURE.read_text())
p = argparse.ArgumentParser()
p.add_argument('--binary', required=True)
p.add_argument('--case', default='.*')
p.add_argument('--profile', choices=['default','html','a11y','combined','all'], default='all')
p.add_argument('--reverse', action='store_true', help='Reverse only execution order; comparison is unchanged.')
a = p.parse_args()
cases = [c for c in m['cases'] if re.search(a.case, c['id'])]
if not cases: p.error('case expression selected no frozen fixture')
profiles = list(m['profiles']) if a.profile == 'all' else [a.profile]
if a.reverse:
    cases.reverse()
    profiles.reverse()
env = {k:v for k,v in os.environ.items() if not k.startswith(('TYPST_', 'CRYSTALLINE_'))}
env.update(NO_COLOR='1', TERM='dumb')

def expectation(c, profile):
    """Named panic retains baseline primary debt, not missing-Source trace debt."""
    row = next(r for r in m['rows'] if r['order']=='normal' and r['profile']==profile
               and r['side']=='vanilla' and r['case']==c['id'])
    expected = {k:row[k] for k in ('exit','stdout','stderr')}
    policy = 'ratified-vanilla-literal'
    named = {
        'panic.named-invalid': 'panic("p1308", nope: 1)',
        'panic.alias-named-invalid': 'fail("p1308", nope: 1)',
        'panic.with-named-invalid': 'fail()',
    }
    if c['id'] in named:
        baseline = next(r for r in m['rows'] if r['order']=='normal' and r['profile']==profile
                        and r['side']=='baseline' and r['case']==c['id'])
        call = named[c['id']]
        column = c['expression'].index(call)
        expected = {k:baseline[k] for k in ('exit','stdout','stderr')}
        expected['stderr'] += f'  while calling `panic` at <input-expression>:1:{column}\n    {call}\n\n'
        policy = 'baseline-primary-debt-plus-L0-source-resolved-trace'
    return expected, policy

def run(job):
    c, profile = job
    argv = [a.binary,'eval',c['expression'],'--format','json'] + m['profiles'][profile]
    expected, policy = expectation(c, profile)
    try:
        result = subprocess.run(argv,cwd=ROOT,env=env,text=True,capture_output=True,timeout=15)
        observable = dict(exit=result.returncode,stdout=result.stdout,stderr=result.stderr)
        verdict = 'Preserved' if observable == expected else 'Violated'
        if result.returncode not in (0,1): verdict = 'Unknown'
    except (OSError, subprocess.TimeoutExpired) as error:
        observable = dict(execution_error=str(error))
        verdict = 'Unknown'
    return dict(case=c['id'],profile=profile,argv=argv,expression=c['expression'],
                observable=observable,expected=expected,policy=policy,verdict=verdict)

with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:
    rows = list(pool.map(run,[(c,profile) for c in cases for profile in profiles]))
print(json.dumps(dict(schema='p1308-independent-replay-v1',
    at=datetime.datetime.now(datetime.timezone.utc).isoformat(),
    regime='executado sem atestação de isolamento técnico',
    binary=a.binary,binary_sha256=sha(a.binary),measurement_sha256=sha(MEASURE),
    script_sha256=sha(__file__),rows=rows,
    counts={v:sum(r['verdict']==v for r in rows) for v in ('Preserved','Violated','Unknown')}),ensure_ascii=False))
