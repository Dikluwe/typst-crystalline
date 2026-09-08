"""Independent public-observation A/B P1317. No owner or candidate source access."""
import argparse
from concurrent.futures import ThreadPoolExecutor
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time

ROOT = Path('/repos/Antigravity/typst-crystalline')
D = ROOT/'00_nucleo/diagnosticos'
L0 = ROOT/'00_nucleo/prompts/compiler/stdlib/loading.md'
FIX = Path('/tmp/p1317-ab-fixtures')
CASES = D/'p1317-ab-cases.json'
BASE = '/tmp/p1316-target.1c6HK7/release/typst'
BASE_SHA = '1178fcde18dee54cb6b5c0feadcc79c8066346e0bb005060db7a2217a072e8ba'
VANILLA = '/usr/local/bin/typst'
VANILLA_SHA = '7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'
PROFILES = {'default': [], 'html': ['--features','html'], 'a11y':['--features','a11y-extras'], 'html+a11y':['--features','html,a11y-extras']}
NEW_MESSAGE = 'error: failed to parse CSV (file is not valid UTF-8)'

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def normative():
    raw = L0.read_bytes()
    assert len(re.findall(rb'(?m)^Hash do C\xc3\xb3digo: [^\r\n]+\r?\n',raw)) == 1
    return hashlib.sha256(re.sub(rb'(?m)^Hash do C\xc3\xb3digo: [^\r\n]+\r?\n',b'',raw)).hexdigest()

def save(path, value):
    path=Path(path)
    text = value if isinstance(value,str) else json.dumps(value,ensure_ascii=False,indent=2)+'\n'
    if path.exists():
        old=path.read_text()
        patch='*** Begin Patch\n*** Update File: '+str(path)+'\n@@\n'+''.join('-'+x+'\n' for x in old.splitlines())+''.join('+'+x+'\n' for x in text.splitlines())+'*** End Patch\n'
    else:
        patch='*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+x+'\n' for x in text.splitlines())+'*** End Patch\n'
    subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True)

def state():
    def git(*a): return subprocess.check_output(['git',*a],cwd=ROOT,text=True).strip()
    return dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=git('rev-parse','HEAD'),diff_stat=git('diff','HEAD','--stat'),status=git('status','--short'))

def obs(row): return {k:row[k] for k in ('exit','stdout','stderr')}

def prepare(a):
    prior=json.loads((D/'p1316-ab-cases-r1.json').read_text())['cases']
    cases=[]
    for c in prior:
        target=('utf8' in c['id'] and c['kind']=='origin')
        cases.append(dict(id='replay-'+c['id'],prior_id=c['id'],expr=c['expr'],cwd=c['cwd'],kind='utf8' if target else 'literal',historical=True))
    def add(id,expr,kind='literal',**kw):
        cases.append(dict(id=id,expr=expr,cwd=str(FIX),kind=kind,historical=False,**kw))
    def b(v): return 'bytes(('+','.join(map(str,v))+',))'
    variants={'header-ff':[255,44,97], 'header-continuation':[128,44,97], 'header-truncated':[226,130], 'data-ff':[97,44,98,10,255,44,49], 'data-overlong':[97,44,98,10,192,175,44,49], 'data-surrogate':[97,44,98,10,237,160,128,44,49], 'data-above-max':[97,44,98,10,244,144,128,128,44,49], 'quoted-multiline':[34,97,10,98,34,44,99,10,255,44,49]}
    source=b(variants['data-ff'])
    for mode in ('array','dictionary'):
        opts='row-type: '+mode
        for name,raw in variants.items(): add('new-'+name+'-'+mode,f'csv({b(raw)}, {opts})','utf8')
        routes={'with':f'{{ let f=csv.with({source}, {opts}); f() }}','args':f'{{ let a=arguments({opts}, {source}); csv(..a) }}','map-detached':f'csv(..arguments(0).map((..a) => {source}), {opts})','sink':f'{{ let f(..a)=csv(..a); f({source}, {opts}) }}','named-before':f'csv(delimiter: ",", {opts}, {source})','excess':f'csv({source}, bytes("another source"), {opts})'}
        for name,expr in routes.items(): add('new-route-'+name+'-'+mode,expr,'utf8',normative_only=name=='excess',detached=name=='map-detached')
        for route in ('str','path'):
            p='"invalid.csv"' if route=='str' else 'path("invalid.csv")'
            add('new-file-'+route+'-'+mode,f'csv({p}, {opts})','utf8',detached=True)
        for name,raw in [('unequal-first',[97,44,98,10,255]),('unequal-extra',[97,44,98,10,255,44,49,44,50]),('unequal-before-utf8',[97,44,98,10,49,10,255,44,50])]:
            add('control-'+name+'-'+mode,f'csv({b(raw)}, {opts})',check='unequal')
        for name,raw in [('unicode-valid',list('α,β\né,😀'.encode())),('unicode-quoted',list('"α\nβ",x\n"😀",y'.encode()))]:
            add('control-'+name+'-'+mode,f'csv({b(raw)}, {opts})',check='success')
    controls={'delimiter-before-utf8':f'csv({source}, delimiter: "xx")','row-before-utf8':f'csv({source}, row-type: str)','unknown-before-utf8':f'csv({source}, nope: true)','cast-before-options':'csv(42, delimiter: "xx")','path-option-before-io':'csv("absent.csv", delimiter: "xx")','missing-path':'csv("absent.csv")','json-utf8':'json(bytes((255,)))','yaml-utf8':'yaml(bytes((255,)))','toml-utf8':'toml(bytes((255,)))','xml-utf8':'xml(bytes((255,)))','cbor-ff':'cbor(bytes((255,)))','read-utf8':'read("invalid.csv")','read-binary':'read("invalid.csv", encoding: none)','namespace':'csv.encode','valid-excess':'csv(bytes("a,b"), 42)'}
    for name,expr in controls.items(): add('control-'+name,expr)
    FIX.mkdir(exist_ok=True)
    raw=bytes(variants['data-ff'])
    fixture=FIX/'invalid.csv'
    assert not fixture.exists() or fixture.read_bytes()==raw
    if not fixture.exists(): fixture.write_bytes(raw)
    save(CASES,dict(schema='p1317-ab-cases-v1',cases=cases,binary_fixture=dict(path=str(fixture),bytes=list(raw),sha256=sha(fixture)),policy='Only named UTF8 cases change first stderr line; replay verified against P1316 candidate first. Other exit/stdout/stderr literal baseline. Excess parsing normative only.'))
    print('cases',len(cases),'utf8',sum(c['kind']=='utf8' for c in cases))

def run(a):
    cases=json.loads(CASES.read_text())['cases']
    if a.ids: cases=[c for c in cases if c['id'] in a.ids.split(',')]
    binaries={'candidate':(a.binary,a.binary_sha256)} if a.binary else {'baseline':(BASE,BASE_SHA),'vanilla':(VANILLA,VANILLA_SHA)}
    for p,h in binaries.values(): assert sha(p)==h,('Unknown binary identity',p)
    env=dict(os.environ)
    for k in ('TYPST_FEATURES','TYPST_DIAGNOSTIC_FORMAT','TYPST_ROOT'):env.pop(k,None)
    env.update(NO_COLOR='1',TERM='dumb',PYTHONDONTWRITEBYTECODE='1')
    before=state();start=time.monotonic(); tasks=[]
    for order in a.orders.split(','):
        for profile,flags in PROFILES.items():
            for c in reversed(cases) if order=='reverse' else cases:
                for product,(p,h) in binaries.items():
                    if product=='vanilla' and c['historical'] and c['kind']!='utf8':continue
                    tasks.append(dict(id=c['id'],profile=profile,order=order,product=product,argv=[p,'eval',c['expr'],*flags],cwd=c['cwd']))
    def execute(t):
        tick=time.monotonic();utc=datetime.datetime.now(datetime.timezone.utc).isoformat()
        p=subprocess.run(t['argv'],cwd=t['cwd'],env=env,capture_output=True,timeout=30)
        return dict(t,utc=utc,seconds=time.monotonic()-tick,exit=p.returncode,stdout=p.stdout.decode(),stderr=p.stderr.decode())
    with ThreadPoolExecutor(max_workers=4) as pool: rows=list(pool.map(execute,tasks))
    for p,h in binaries.values():assert sha(p)==h
    save(a.output,dict(schema='p1317-ab-runs-v1',before=before,after=state(),wall_seconds=time.monotonic()-start,processes=len(rows),runner_sha256=sha(__file__),cases_sha256=sha(CASES),l0_normative_sha256=normative(),binaries={k:dict(path=p,sha256=h) for k,(p,h) in binaries.items()},rows=rows))
    print('runs',len(rows),'seconds',time.monotonic()-start,'output',a.output)

def freeze(a):
    m=json.loads(Path(a.measurement).read_text());cases=json.loads(CASES.read_text())['cases']
    assert m['runner_sha256']==sha(__file__) and m['cases_sha256']==sha(CASES) and m['l0_normative_sha256']==normative()
    prior=json.loads((D/'p1316-ab-candidate-runs.json').read_text())
    prior={(r['id'],r['profile']):r for r in prior['rows'] if r['order']=='normal'}
    by={(r['id'],r['profile'],r['product']):r for r in m['rows']};assert len(by)==len(m['rows'])
    expected=[]
    for c in cases:
        for profile in PROFILES:
            row=by[c['id'],profile,'baseline'];e=obs(row)
            assert e['exit'] in (0,1),('Unknown process',c['id'])
            if c['historical']:
                p=prior[c['prior_id'],profile]
                assert row['cwd']==p['cwd'] and row['argv'][2]==p['argv'][2] and e==obs(p),('Unknown replay drift',c['id'])
            if c.get('check')=='success':assert e['exit']==0,('Unknown valid control',c['id'],e)
            if c.get('check')=='unequal':assert e['stderr'].startswith('error: failed to parse CSV (found '),('Unknown precedence',c['id'],e)
            if c['kind']=='utf8':
                assert e['exit']==1 and e['stdout']=='' and len(re.findall(r'(?m)^error: ',e['stderr']))==1
                first,sep,rest=e['stderr'].partition('\n')
                assert first.startswith('error: failed to parse CSV (CSV parse error:') and 'invalid utf-8' in first,('Unknown UTF8 case',c['id'],e)
                v=obs(by[c['id'],profile,'vanilla'])
                if c.get('normative_only'):assert v['stderr'].startswith('error: unexpected argument'),('Unknown excess boundary',c['id'],v)
                else:assert v['stderr'].startswith('error: failed to parse CSV (file is not valid UTF-8'),('Unknown vanilla cause',c['id'],v)
                if c.get('detached'):assert '┌─' not in rest,('Unknown detached origin',c['id'])
                e['stderr']=NEW_MESSAGE+sep+rest
            expected.append(dict(id=c['id'],profile=profile,kind=c['kind'],expr=c['expr'],cwd=c['cwd'],expected=e,baseline_red=e!=obs(row)))
    old=json.loads((D/'p1316-ab-freeze.json').read_text())
    files=[Path(__file__),CASES,Path(a.measurement),D/'p1316-ab-cases-r1.json',D/'p1316-ab-candidate-runs.json',D/'p1316-ab-freeze.json',FIX/'invalid.csv']
    files += [Path(p) for p in old['inputs'] if p.startswith('/tmp/')]
    files += [ROOT/'lab/typst-original/crates/typst-library/src'/p for p in ('loading/csv.rs','diag.rs')]
    skill=Path('/home/dikluwe/.codex/skills/tekt-materializacao-segregada')
    files += [skill/p for p in ('SKILL.md','references/papeis-e-capacidades.md','references/artefatos-e-gates.md')]
    save(a.output,dict(schema='p1317-ab-freeze-v1',utc=state()['utc'],regime='A/B executado sem atestação de isolamento técnico',capabilities=dict(executor='/root/p1317_tests',reads=['L0 loading.md','vanilla loading/csv.rs and diag.rs','p1316-ab-* and historical fixtures','binaries','git HEAD/status/diff STAT','skill/references and ADR-0127'],writes=['00_nucleo/diagnosticos/p1317-ab-*','/tmp/p1317-ab-fixtures'],context='Task-scoped independent agent. Owner loading.rs, local tests, candidate diff and source-containing receipts not read; shared filesystem means no technical isolation attestation.'),unknown_policy='Missing, duplicate, malformed, unsupported, timeout, crash, input drift or binary ambiguity blocks; never defaults PASS.',budget='One baseline, at most one focal calibration after measurement; two revisions without discriminating gain require stop. Candidate normal/repeat/reverse only after release.',l0=dict(path=str(L0),raw_sha256=sha(L0),normative_sha256=normative(),exclusion='Only one canonical Hash do Código line'),inputs={str(p):sha(p) for p in files},binaries=m['binaries'],provenance=m['before'],expected=expected,calibration=dict(revisions=0,processes=m['processes'],wall_seconds=m['wall_seconds']),scope='Predeclared ErrorKind::Utf8 cases: exact baseline observation changes only stderr first line. Cause compared to vanilla while full vanilla diagnostic kept with position differences. Excess parsing normative only. Replay exact before transformation.',limitations=['CLI cannot attest pure decoder or World call count, synthetic Rust Args or enum selection implementation.','No mutation score, general parity, refinement seal or implementation verdict.']))
    print('freeze',sha(a.output),'observations',len(expected),'RED',sum(e['baseline_red'] for e in expected))

def compare(a):
    f=json.loads(Path(a.freeze).read_text());m=json.loads(Path(a.measurement).read_text())
    for p,h in f['inputs'].items():assert sha(p)==h,('Unknown changed input',p)
    assert normative()==f['l0']['normative_sha256']==m['l0_normative_sha256']
    assert m['runner_sha256']==sha(__file__) and m['cases_sha256']==sha(CASES)
    expect={(r['id'],r['profile']):r for r in f['expected']}
    required={(i,p,o) for i,p in expect for o in ('normal','repeat','reverse')}
    keys=[(r['id'],r['profile'],r['order']) for r in m['rows']]
    assert len(keys)==len(required) and set(keys)==required,'Unknown missing/duplicate observation'
    failures=[]
    for r in m['rows']:
        e=expect[r['id'],r['profile']]
        assert r['product']=='candidate' and r['argv'][2]==e['expr'] and r['cwd']==e['cwd']
        if obs(r)!=e['expected']:failures.append(dict(id=r['id'],profile=r['profile'],order=r['order'],expected=e['expected'],actual=obs(r)))
    save(a.output,dict(schema='p1317-ab-comparison-v1',utc=state()['utc'],freeze_sha256=sha(a.freeze),measurement_sha256=sha(a.measurement),candidate=m['binaries'],comparisons=len(keys),unknown=0,failures=failures,status='FAIL' if failures else 'PASS',processes=m['processes'],wall_seconds=m['wall_seconds']))
    print('comparison',len(keys),'failures',len(failures),'sha',sha(a.output))

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('mode',choices=['prepare','run','freeze','compare'])
    for name in ('binary','binary-sha256','ids','measurement','freeze','output'):parser.add_argument('--'+name)
    parser.add_argument('--orders',default='normal');args=parser.parse_args();globals()[args.mode](args)
