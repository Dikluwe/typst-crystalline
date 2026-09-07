#!/usr/bin/env python3
"""Independent P1308 literal measurement; immutable R6 input, no candidate access."""
import concurrent.futures, datetime, hashlib, json, os, pathlib, re, subprocess, time

ROOT = pathlib.Path(__file__).resolve().parents[2]
OUT = ROOT / '00_nucleo/diagnosticos'
sha = lambda p: hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
baseline_path = OUT / 'p1308-baseline.json'
assert sha(baseline_path) == '62c53690cbe3dd36b5b79168ea59a32f6eb88cc52ea52a8ca97365c5a9459394'
baseline = json.loads(baseline_path.read_text())
oracle_path = OUT / 'p1307-r6-oracle.json'
assert sha(oracle_path) == '99d2a67984a468c8e86e20010ecc1bd76a364f1d1adebbb5b873fc341f7790a6'
oracle = json.loads(oracle_path.read_text())
previous = json.loads(json.loads((OUT / 'p1307-r6-public-matrix-2.json').read_text())['stdout'])
pending = {r['case'] for r in previous['rows'] if r['verdict'] == 'Violated'}
assert len(pending) == 14
cases = [dict(id=c['id'], expression=c['expression'], origin='immutable R6 oracle', expected=c['observations']['default']['future_expected']) for c in oracle['cases'] if c['id'] in pending]
focal = {
 'filter.direct': 'arguments(1).filter(value => "bad")',
 'filter.alias': '{ let callback = (value) => "bad"; arguments(1).filter(callback) }',
 'filter.with': '{ let callback = (prefix, value) => "bad"; let bound = callback.with(9); arguments(1).filter(bound) }',
 'filter.args-spread': '{ let callback = (value) => "bad"; let saved = arguments(callback); arguments(1).filter(..saved) }',
 'filter.array-spread': '{ let callback = (value) => "bad"; let saved = (callback,); arguments(1).filter(..saved) }',
 'filter.native': 'arguments(1).filter(str)',
 'filter.empty': 'repr(arguments().filter(value => "bad"))',
 'panic.direct': 'panic("p1308")',
 'panic.alias': '{ let fail = panic; fail("p1308") }',
 'panic.with': '{ let fail = panic.with("p1308"); fail() }',
 'panic.named-invalid': 'panic("p1308", nope: 1)',
 'panic.alias-named-invalid': '{ let fail = panic; fail("p1308", nope: 1) }',
 'panic.with-named-invalid': '{ let fail = panic.with("p1308", nope: 1); fail() }',
 'panic.callback': 'arguments(1).map(value => panic("p1308"))',
 'panic.alias-callback': '{ let fail = panic; arguments(1).map(value => fail("p1308")) }',
 'panic.with-callback': '{ let fail = panic.with("p1308"); arguments(1).map(value => fail()) }',
 'join.none-left': 'repr(none + arguments(z: 1, 2))',
 'join.none-empty': 'repr(arguments() + none)',
 'join.unsupported': 'arguments(1) + 1',
 'repr.with': 'repr(json.encode.with())',
 'repr.args-empty': 'repr(arguments())',
 'repr.args-nested': 'repr(arguments((1, 2), a: (x: 3)))',
}
for n in (47,48,49,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81):
 focal[f'repr.args-width-{n}'] = 'repr(arguments("' + ('x' * n) + '"))'
focal['repr.args-many'] = 'repr(arguments(' + ', '.join(str(n) for n in range(30)) + '))'
focal['repr.args-unicode'] = 'repr(arguments("' + ('é' * 50) + '"))'
focal['repr.args-named-long'] = 'repr(arguments(first: "' + ('x' * 40) + '", second: "' + ('y' * 40) + '"))'
cases += [dict(id=k, expression=v, origin='P1308 independent boundary probe') for k,v in focal.items()]
profiles = {'default': [], 'html': ['--features', 'html'], 'a11y': ['--features', 'a11y-extras'], 'combined': ['--features', 'html,a11y-extras']}
bins = {'baseline': baseline['baseline_binary'], 'vanilla': baseline['vanilla']}
for entry in bins.values(): assert sha(entry['path']) == entry['sha256']
env = {k:v for k,v in os.environ.items() if not k.startswith(('TYPST_', 'CRYSTALLINE_'))}
env.update(NO_COLOR='1', TERM='dumb')

def run(job):
 order, c, profile, side = job
 argv = [bins[side]['path'], 'eval', c['expression'], '--format', 'json'] + profiles[profile]
 start = time.monotonic()
 p = subprocess.run(argv, cwd=ROOT, env=env, text=True, capture_output=True, timeout=15)
 ranges = []
 lines = c['expression'].splitlines(keepends=True)
 rendered = p.stderr.splitlines()
 for i,line in enumerate(rendered):
  m = re.search(r'<input-expression>:(\d+):(\d+)', line)
  if m:
   ln,col = map(int,m.groups()); length = None
   for underline in rendered[i+1:i+5]:
    if '^' in underline: length = len(re.search(r'\^+', underline).group()); break
   start_offset = len(''.join(lines[:ln-1]).encode()) + len(lines[ln-1][:col].encode())
   ranges.append(dict(line=ln,column=col,start=start_offset,end=start_offset+length if length is not None else None,role='trace' if 'while calling' in line else 'primary',range_basis='ASCII fixture columns/carets; raw stderr authoritative'))
 return dict(order=order,case=c['id'],profile=profile,side=side,argv=argv,exit=p.returncode,stdout=p.stdout,stderr=p.stderr,ranges=ranges,seconds=time.monotonic()-start)

at = datetime.datetime.now(datetime.timezone.utc).isoformat()
jobs = [(order,c,profile,side) for order,seq in [('normal',cases),('reverse',list(reversed(cases))),('repeat',cases)] for c in seq for profile in profiles for side in bins]
with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool: rows = list(pool.map(run,jobs))
instability = []
for c in cases:
 for profile in profiles:
  for side in bins:
   found = [r for r in rows if r['case']==c['id'] and r['profile']==profile and r['side']==side]
   if len({(r['exit'],r['stdout'],r['stderr']) for r in found}) != 1: instability.append([c['id'],profile,side])
report = dict(schema='p1308-independent-literal-v1',at=at,regime='executado sem atestação de isolamento técnico',candidate_read=False,baseline_sha256=sha(baseline_path),baseline_state=baseline['state'],binaries=bins,oracle_sha256=sha(oracle_path),script_sha256=sha(__file__),profiles=profiles,cases=cases,rows=rows,instability=instability,limitations=['Only literal eval routes; no candidate or implementation verdict.','Ranges preserve exact stderr; Unicode display-column derivation is not claimed.'])
(OUT / 'p1308-measure.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
print(json.dumps(dict(cases=len(cases),runs=len(rows),instability=instability,sha256=sha(OUT/'p1308-measure.json'))))
