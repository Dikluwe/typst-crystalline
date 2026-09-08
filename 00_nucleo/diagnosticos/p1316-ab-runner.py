"""P1316 independent A/B; authorized public inputs only, no owner/candidate sources."""
import argparse
import importlib.util
import json
from pathlib import Path
import re

D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('prior_ab', D/'p1315-ab-runner.py')
P = importlib.util.module_from_spec(spec)
spec.loader.exec_module(P)
P.BASE = '/dev/shm/p1315-target.V6TWEF/release/typst'
P.BASE_SHA = '6a4a75787060ce8015ebde85ef2deb078f4b6a0b827b5533602785261ba890a1'
FIX = Path('/tmp/p1316-ab-fixtures')
CASEFILE = D/'p1316-ab-cases-r1.json'

def prepare():
    old = json.loads((D/'p1315-ab-cases.json').read_text())['cases']
    explicit = {'history-bytes-legacy-unequal','history-bytes-legacy-unequal-dict','history-options-utf8-invalid','history-options-utf8-invalid-dict','history-options-parse-unequal-with'}
    cases=[]
    for c in old:
        target = c['id'] in explicit or (c['kind']=='ordinal' and not ('str-path' in c['id'] or 'rooted-path' in c['id'])) or c['id'].startswith('utf8-')
        cases.append(dict(id='replay-'+c['id'], prior_id=c['id'], kind='origin' if target else 'literal', expr=c['expr'],cwd=c['cwd'], historical=True))
    for mode in ('array','dictionary'):
        for failure,source in [('unequal','bytes("a,b\\n1")'),('utf8','bytes((97,44,98,10,255,44,97))')]:
            opts=f'row-type: {mode}'
            routes={
                'named-before':f'csv(delimiter: ",", {opts}, {source})',
                'bound-source':f'{{ let f = csv.with({source}, {opts}); f() }}',
                'args-named-before':f'{{ let a = arguments({opts}, {source}); csv(..a) }}',
                'array-spread':f'csv(..({source},), {opts})',
                'sink':f'{{ let f(..a) = csv(..a); f({source}, {opts}) }}',
                'args-map-detached':f'csv(..arguments(0).map((..a) => {source}), {opts})',
                'second-positional':f'csv(bytes("a,b\\n1,2"), bytes("valid"), {opts})',
            }
            for route,expr in routes.items():
                cases.append(dict(id=f'focal-{failure}-{route}-{mode}',kind='literal' if route=='second-positional' else 'origin',detached=route=='args-map-detached',expr=expr,cwd=str(FIX),historical=False))
    controls={
       'str-parse':'csv("unequal.csv")',
       'path-parse':'csv(path("unequal.csv"), row-type: dictionary)',
       'valid-bytes':'csv(bytes("a,b\\n1,2"))',
       'valid-dict':'csv(bytes("a,b\\n1,2"), row-type: dictionary)',
       'invalid-option-before-parse':'csv(bytes("a,b\\n1"), delimiter: "bad")',
       'unknown-before-parse':'csv(bytes("a,b\\n1"), nope: true)',
       'missing-path':'csv("absent.csv")',
       'json-parse':'json(bytes("{"))',
       'yaml-parse':'yaml(bytes("["))',
       'toml-parse':'toml(bytes("["))',
       'xml-parse':'xml(bytes("<"))',
       'cbor-parse':'cbor(bytes((255,)))',
    }
    cases += [dict(id='control-'+k,kind='literal',expr=v,cwd=str(FIX),historical=False) for k,v in controls.items()]
    P.save(FIX/'unequal.csv','a,b\n1\n')
    P.save(CASEFILE,dict(schema='p1316-ab-cases-v1',cases=cases,policy='Only predeclared origin cases use baseline message plus vanilla remainder. All other observations literal baseline. Historical cases keep exact expressions and cwd.'))
    print('cases',len(cases),'origin',sum(c['kind']=='origin' for c in cases))

def run(a):
    cases=json.loads(CASEFILE.read_text())['cases']
    if a.ids: cases=[c for c in cases if c['id'] in a.ids.split(',')]
    binaries={'candidate':(a.binary,a.binary_sha256)} if a.binary else {'baseline':(P.BASE,P.BASE_SHA),'vanilla':(P.VANILLA,P.VANILLA_SHA)}
    for path,digest in binaries.values():assert P.sha(path)==digest,('Unknown binary',path)
    env=dict(P.os.environ)
    for k in ('TYPST_FEATURES','TYPST_DIAGNOSTIC_FORMAT','TYPST_ROOT'):env.pop(k,None)
    env.update(NO_COLOR='1',TERM='dumb',PYTHONDONTWRITEBYTECODE='1')
    before=P.state();tasks=[]
    for order in a.orders.split(','):
        for profile,flags in P.PROFILES.items():
            for c in reversed(cases) if order=='reverse' else cases:
                for product,(binary,_) in binaries.items():
                    if product=='vanilla' and c['kind']!='origin':continue
                    tasks.append(dict(id=c['id'],profile=profile,order=order,product=product,argv=[binary,'eval',c['expr'],*flags],cwd=c['cwd']))
    def execute(t):
        tick=P.time.monotonic();utc=P.datetime.datetime.now(P.datetime.timezone.utc).isoformat()
        p=P.subprocess.run(t['argv'],cwd=t['cwd'],env=env,capture_output=True,timeout=30)
        return dict(t,utc=utc,seconds=P.time.monotonic()-tick,exit=p.returncode,stdout=p.stdout.decode(),stderr=p.stderr.decode())
    with P.ThreadPoolExecutor(max_workers=4) as pool:rows=list(pool.map(execute,tasks))
    for path,digest in binaries.values():assert P.sha(path)==digest
    P.save(a.output,dict(schema='p1316-ab-runs-v1',before=before,after=P.state(),runner_sha256=P.sha(__file__),cases_sha256=P.sha(CASEFILE),l0_normative_sha256=P.normative(),binaries={k:dict(path=p,sha256=h) for k,(p,h) in binaries.items()},rows=rows))
    print('runs',len(rows),a.output)

def freeze(a):
    m=json.loads(Path(a.measurement).read_text());cases=json.loads(CASEFILE.read_text())['cases']
    assert m['runner_sha256']==P.sha(__file__) and m['cases_sha256']==P.sha(CASEFILE)
    assert m['l0_normative_sha256']==P.normative()
    old=json.loads((D/'p1315-ab-candidate-runs.json').read_text())
    prior={(r['id'],r['profile']):r for r in old['rows'] if r['order']=='normal'}
    by={(r['id'],r['profile'],r['product']):r for r in m['rows']}
    assert len(by)==len(m['rows'])
    expected=[]
    for c in cases:
        for profile in P.PROFILES:
            b=by[c['id'],profile,'baseline'];e=P.obs(b)
            assert e['exit'] in (0,1),('Unknown process',c['id'])
            if c['historical']:
                p=prior[c['prior_id'],profile]
                assert b['cwd']==p['cwd'] and b['argv'][2]==p['argv'][2] and e==P.obs(p),('Unknown historical drift',c['id'],profile)
            if c['kind']=='origin':
                v=by[c['id'],profile,'vanilla'];ve=P.obs(v)
                assert e['exit']==ve['exit']==1 and e['stdout']==ve['stdout']==''
                assert len(re.findall(r'(?m)^error: ',e['stderr']))==1 and len(re.findall(r'(?m)^error: ',ve['stderr']))==1
                bm,sep,bt=e['stderr'].partition('\n');vm,vs,vt=ve['stderr'].partition('\n')
                assert sep==vs=='\n' and bm.startswith('error: failed to parse CSV (') and vm.startswith('error: failed to parse CSV ('),('Unknown non-CSV',c['id'],e,ve)
                if c.get('detached'):assert '┌─' not in vt,('Unknown map origin',c['id'])
                e['stderr']=bm+'\n'+vt
            expected.append(dict(id=c['id'],profile=profile,kind=c['kind'],expr=c['expr'],cwd=c['cwd'],expected=e,baseline_red=e!=P.obs(b)))
    prev=json.loads((D/'p1315-ab-freeze.json').read_text())
    files=[Path(__file__),CASEFILE,Path(a.measurement),D/'p1315-ab-cases.json',D/'p1315-ab-candidate-runs.json',D/'p1315-ab-freeze.json',D/'p1315-ab-runner.py',FIX/'unequal.csv']
    files += [Path(p) for p in prev['inputs'] if p.startswith('/tmp/')]
    files += [P.ROOT/'lab/typst-original/crates/typst-library/src'/p for p in ('loading/mod.rs','loading/csv.rs','diag.rs','foundations/args.rs')]
    files += [D/p for p in ('p1316-ab-cases.json','p1316-ab-baseline-runs.json','p1316-ab-focal-runs.json')]
    skill=Path('/home/dikluwe/.codex/skills/tekt-materializacao-segregada')
    files += [skill/p for p in ('SKILL.md','references/papeis-e-capacidades.md','references/artefatos-e-gates.md')]
    P.save(a.output,dict(schema='p1316-ab-freeze-v1',utc=P.state()['utc'],regime='A/B executado sem atestação de isolamento técnico',capabilities=dict(executor='/root/p1316_tests',reads=['L0 loading.md','vanilla loading/mod.rs csv.rs diag.rs foundations/args.rs','P1315 A/B artifacts and fixtures','public CLI binaries','git HEAD/status/diff STAT','skill and references'],writes=['00_nucleo/diagnosticos/p1316-ab-*','/tmp/p1316-ab-fixtures'],context='Task-scoped, no owner code/tests/candidate diff or measurements containing source read. Filesystem shared; no technical isolation attested.'),unknown_policy='Missing/duplicate/malformed input or observation, timeout/crash, unsupported expression, altered normative input or ambiguous binary identity blocks; never defaults to PASS.',budget='One baseline; focal correction only with new witness; two revisions without discriminating gain stop. Candidate normal/repeat/reverse after review.',l0=dict(path=str(P.L0),raw_sha256=P.sha(P.L0),normative_sha256=P.normative(),exclusion='Only one canonical Hash do Código line'),inputs={str(p):P.sha(p) for p in files},binaries=m['binaries'],provenance=m['before'],expected=expected,scope='Only predeclared native csv(Bytes) parse origins/traces change: exact baseline first diagnostic line + exact vanilla remaining diagnostic. No message normalization, suffix removal, generic parity, internal World or pure-decoder attestations.',calibration=dict(revisions=1,reason='Diagnostic cardinality now counts line-start error only (UTF8 message contains CSV parse error internally). Extra-positional controls now use valid CSV: malformed source exposed pre-existing validation-order debt against vanilla. Candidate uninspected. Focal rerun, unchanged initial rows reused with exact source checks.',processes_initial=1596,processes_focal=48),limitations=['CLI cannot create Rust synthetic Args without occurrences or distinguish value_span from occurrence.span for positional arguments.','Map detached explicitly covered.','Malformed first-source plus excess positional has pre-existing order debt, not claimed as vanilla parity.','No mutation score or complete refinement seal.','Tactical steps are not normative inputs.']))
    print('freeze',P.sha(a.output),'expected',len(expected),'RED',sum(e['baseline_red'] for e in expected))

def compare(a):
    f=json.loads(Path(a.freeze).read_text());m=json.loads(Path(a.measurement).read_text())
    for p,h in f['inputs'].items():assert P.sha(p)==h,('Unknown changed input',p)
    assert P.normative()==f['l0']['normative_sha256']==m['l0_normative_sha256']
    assert m['runner_sha256']==P.sha(__file__) and m['cases_sha256']==P.sha(CASEFILE)
    expect={(e['id'],e['profile']):e for e in f['expected']}
    required={(i,p,o) for i,p in expect for o in ('normal','repeat','reverse')}
    actual=[(r['id'],r['profile'],r['order']) for r in m['rows']]
    assert len(actual)==len(required) and set(actual)==required,'Unknown missing/duplicate observations'
    failures=[]
    for r in m['rows']:
        e=expect[r['id'],r['profile']]
        assert r['product']=='candidate' and r['argv'][2]==e['expr'] and r['cwd']==e['cwd']
        if P.obs(r)!=e['expected']:failures.append(dict(id=r['id'],profile=r['profile'],order=r['order'],expected=e['expected'],actual=P.obs(r)))
    P.save(a.output,dict(schema='p1316-ab-comparison-v1',utc=P.state()['utc'],freeze_sha256=P.sha(a.freeze),measurement_sha256=P.sha(a.measurement),candidate=m['binaries'],comparisons=len(actual),unknown=0,failures=failures,status='FAIL' if failures else 'PASS'))
    print('comparisons',len(actual),'failures',len(failures),'sha',P.sha(a.output))

def reconcile(a):
    original_path=D/'p1316-ab-baseline-runs.json';focal_path=D/'p1316-ab-focal-runs.json'
    original=json.loads(original_path.read_text());focal=json.loads(focal_path.read_text())
    assert original['runner_sha256']=='a534b5b2543ee852704ce6d82b89d0e60f96a152ff80e425c3e7730cf8d85f66'
    assert original['cases_sha256']==P.sha(D/'p1316-ab-cases.json')
    assert focal['cases_sha256']==P.sha(CASEFILE)
    assert original['binaries']==focal['binaries']
    cases={c['id']:c for c in json.loads(CASEFILE.read_text())['cases']}
    key=lambda r:(r['id'],r['profile'],r['product'])
    replacements={key(r):r for r in focal['rows']}
    rows=[replacements.get(key(r),r) for r in original['rows']]
    for r in rows:
        c=cases[r['id']]
        assert r['argv'][2]==c['expr'] and r['cwd']==c['cwd'],'Unknown stale expression'
    P.save(a.output,dict(schema='p1316-ab-reconciled-measurement-v1',before=original['before'],after=focal['after'],runner_sha256=P.sha(__file__),cases_sha256=P.sha(CASEFILE),l0_normative_sha256=P.normative(),binaries=original['binaries'],rows=rows,composition=dict(utc=P.state()['utc'],original_sha256=P.sha(original_path),focal_sha256=P.sha(focal_path),original_l0=original['l0_normative_sha256'],focal_l0=focal['l0_normative_sha256'],reason='Reuse unchanged public observations with original UTC/argv/cwd. Only explicitly revised excess-positional controls and UTF8 boundary reruns replaced. Any L0 update before freeze clarifies mixed excess/parse debt; it does not change these expressions or expected full observations. Not a fresh full corpus execution.')))
    print('reconciled',len(rows),'sha',P.sha(a.output))

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('mode',choices=['prepare','run','freeze','compare','reconcile'])
    for k in ('binary','binary-sha256','ids','measurement','freeze','output'):p.add_argument('--'+k)
    p.add_argument('--orders',default='normal');a=p.parse_args()
    if a.mode=='prepare':prepare()
    else:globals()[a.mode](a)
