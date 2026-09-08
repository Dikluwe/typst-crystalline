"""Role-scoped P1315 A/B: public CLI only, frozen before candidate exposure."""
import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time
from concurrent.futures import ThreadPoolExecutor

D = Path(__file__).resolve().parent
ROOT = D.parents[1]
FIX = Path('/tmp/p1315-ab-fixtures')
L0 = ROOT/'00_nucleo/prompts/compiler/stdlib/loading.md'
BASE = '/dev/shm/p1314-target.cswujn/release/typst'
BASE_SHA = 'cf9b997ac399cfb95cf15063cc417484ed28a75be18f6e7d55a28300d792114f'
VANILLA = '/usr/local/bin/typst'
VANILLA_SHA = '7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'
PROFILES = {'default': [], 'html': ['--features','html'], 'a11y': ['--features','a11y-extras'], 'html+a11y': ['--features','html,a11y-extras']}
FRAGMENT = re.compile(r'found (\d+) instead of (\d+) fields in line (\d+)')

def sha(path):
    with open(path, 'rb') as f: return hashlib.file_digest(f,'sha256').hexdigest()

def normative():
    raw,n = re.subn(rb'(?m)^Hash do C\xc3\xb3digo: [0-9a-f]+\r?\n',b'',L0.read_bytes())
    assert n == 1
    return hashlib.sha256(raw).hexdigest()

def state():
    def git(*a): return subprocess.check_output(['git',*a],cwd=ROOT,text=True).strip()
    return dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=git('rev-parse','HEAD'),diff_stat=git('diff','HEAD','--stat'),status=git('status','--short'))

def save(path, data):
    path=Path(path).resolve()
    assert not path.exists(), path
    raw=data if isinstance(data,str) else json.dumps(data,ensure_ascii=False,indent=2)+'\n'
    assert raw.endswith('\n')
    patch='*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+line+'\n' for line in raw.splitlines())+'*** End Patch\n'
    subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True)

def obs(r): return {k:r[k] for k in ('exit','stdout','stderr')}

def prepare():
    cases=[]
    historical=json.loads((D/'p1314-ab-cases.json').read_text())['cases']
    for c in historical:
        cases.append(dict(id='history-'+c['id'],kind='historical',expr=c['expr'],cwd='/tmp/p1313-ab-fixtures' if c['historical_p1313'] else '/tmp/p1314-ab-fixtures',prior_id=c['id']))
    samples=[
      ('plain-short','a,b\n1',1,2,2,','),
      ('plain-extra','a,b\n1,2,3',3,2,2,','),
      ('header-multiline','"a\nb",c\n1',1,2,2,','),
      ('header-multiline-extra','"a\nb",c\n1,2,3',3,2,2,','),
      ('leading-blank','\n\na,b\n1',1,2,2,','),
      ('middle-blank','a,b\n\n\n1,2\n\n3',1,2,3,','),
      ('late-multiline','a,b\n"x\ny",z\n"p\nq",r\ns',1,2,4,','),
      ('crlf','a,b\r\n"x\r\ny",z\r\nq',1,2,3,','),
      ('crlf-leading','\r\n\r\na,b\r\n1,2,3',3,2,2,','),
      ('multiline-error','a,b\n"x\ny"',1,2,2,','),
      ('escaped-quote','"a""b\nc",d\ne',1,2,2,','),
      ('semicolon','"a\nb";c\n1;2;3',3,2,2,';'),
      ('one-field-header','"a\nb"\nx,y',2,1,2,','),
      ('empty-field-record','a,b\n,\n\nx',1,2,3,','),
    ]
    q=lambda x:json.dumps(x,ensure_ascii=False)
    for name,data,found,want,ordinal,delimiter in samples:
        for mode in ('array','dictionary'):
            opts=f'row-type: {mode}, delimiter: {q(delimiter)}'
            cases.append(dict(id=name+'-'+mode,kind='ordinal',expr=f'csv(bytes({q(data)}), {opts})',cwd=str(FIX),fragment=f'found {found} instead of {want} fields in line {ordinal}'))
    for mode in ('array','dictionary'):
        data=q('\n"a\nb",c\n1,2\n3')
        opts=f'row-type: {mode}'
        routes={
          'with':f'csv.with({opts})(bytes({data}))',
          'args':f'csv(..arguments(bytes({data}), {opts}))',
          'args-map':f'csv(bytes({data}), ..arguments({opts}).map((..a) => {mode}))',
          'str-path':f'csv("multiline.csv", {opts})',
          'rooted-path':f'csv(path("multiline.csv"), {opts})',
        }
        for route,expr in routes.items():
            cases.append(dict(id=route+'-'+mode,kind='ordinal',expr=expr,cwd=str(FIX),fragment='found 1 instead of 2 fields in line 4' if 'path' in route else 'found 1 instead of 2 fields in line 3'))
        for name,data in [('empty',''),('only-blank','\n\n'),('header-only','"a\nb",c'),('valid-blank','\na,b\n\n1,2\n'),('valid-multiline','"a\nb",c\n"1\n2",3'),('duplicate-headers','a,a\n1,2')]:
            cases.append(dict(id=name+'-'+mode,kind='value',expr=f'csv(bytes({q(data)}), {opts})',cwd=str(FIX)))
        for name,byte_expr in [('utf8-header','bytes((255,44,97))'),('utf8-data','bytes((97,44,98,10,255,44,97))'),('utf8-after-multiline','bytes((34,97,10,98,34,44,99,10,255,44,97))')]:
            cases.append(dict(id=name+'-'+mode,kind='control',expr=f'csv({byte_expr}, {opts})',cwd=str(FIX)))
    save(FIX/'multiline.csv','\n"a\nb",c\n1,2\n"3\n4",5\n6\n')
    save(D/'p1315-ab-cases.json',dict(schema='p1315-ab-cases-v1',cases=cases))
    print('prepared',len(cases),'cases')

def run(args):
    cases_path=D/'p1315-ab-cases.json'
    cases=json.loads(cases_path.read_text())['cases']
    if args.ids: cases=[c for c in cases if c['id'] in args.ids.split(',')]
    binaries={'candidate':(args.binary,args.binary_sha256)} if args.binary else {'baseline':(BASE,BASE_SHA),'vanilla':(VANILLA,VANILLA_SHA)}
    for path,digest in binaries.values(): assert sha(path)==digest,('Unknown binary identity',path)
    env=dict(os.environ)
    for key in ('TYPST_FEATURES','TYPST_DIAGNOSTIC_FORMAT','TYPST_ROOT'):env.pop(key,None)
    env.update(NO_COLOR='1',TERM='dumb',PYTHONDONTWRITEBYTECODE='1')
    before=state()
    tasks=[]
    for order in args.orders.split(','):
        for profile,flags in PROFILES.items():
            for c in reversed(cases) if order=='reverse' else cases:
                for product,(binary,_) in binaries.items():
                    if c['kind']=='historical' and product=='vanilla':continue
                    tasks.append(dict(id=c['id'],order=order,profile=profile,product=product,argv=[binary,'eval',c['expr'],*flags],cwd=c['cwd']))
    def execute(task):
        tick=time.monotonic()
        utc=datetime.datetime.now(datetime.timezone.utc).isoformat()
        proc=subprocess.run(task['argv'],cwd=task['cwd'],env=env,capture_output=True,timeout=30)
        return dict(task,utc=utc,seconds=time.monotonic()-tick,exit=proc.returncode,stdout=proc.stdout.decode(),stderr=proc.stderr.decode())
    with ThreadPoolExecutor(max_workers=4) as pool: rows=list(pool.map(execute,tasks))
    for path,digest in binaries.values(): assert sha(path)==digest
    save(args.output,dict(schema='p1315-ab-runs-v1',before=before,after=state(),cases_sha256=sha(cases_path),runner_sha256=sha(__file__),l0_normative_sha256=normative(),binaries={k:dict(path=p,sha256=h) for k,(p,h) in binaries.items()},rows=rows))
    print('runs',len(rows),'saved',args.output)

def freeze(args):
    measurement=json.loads(Path(args.measurement).read_text())
    cases=json.loads((D/'p1315-ab-cases.json').read_text())['cases']
    assert measurement['cases_sha256']==sha(D/'p1315-ab-cases.json')
    assert measurement['runner_sha256']==sha(__file__)
    assert measurement['l0_normative_sha256']==normative()
    historical=json.loads((D/'p1314-ab-candidate-runs.json').read_text())
    by={(r['id'],r['profile'],r['product']):r for r in measurement['rows']}
    assert len(by)==len(measurement['rows'])
    prior={(r['id'],r['profile']):r for r in historical['runs'] if r['order']=='normal'}
    expected=[]
    for c in cases:
        for profile in PROFILES:
            baseline=by[c['id'],profile,'baseline']
            value=obs(baseline)
            assert value['exit'] in (0,1),'Unknown process'
            if c['kind']=='historical':
                p=prior[c['prior_id'],profile]
                assert baseline['argv'][2]==p['argv'][2] and baseline['cwd']==p['cwd']
                assert value==obs(p),('Unknown historical drift',c['id'],profile)
            elif c['kind']=='ordinal':
                vanilla=by[c['id'],profile,'vanilla']
                matches=FRAGMENT.findall(vanilla['stderr'])
                assert vanilla['exit']==1 and vanilla['stdout']=='' and len(matches)==1
                fragment=FRAGMENT.search(vanilla['stderr']).group(0)
                assert fragment==c['fragment'],('Unknown manual/vanilla discrepancy',c['id'],fragment,c['fragment'])
                matches=FRAGMENT.findall(value['stderr'])
                assert value['exit']==1 and value['stdout']=='' and len(matches)==1
                original=FRAGMENT.search(value['stderr'])
                wanted=FRAGMENT.fullmatch(fragment)
                assert original.group(1,2)==wanted.group(1,2)
                start,end=original.span(3)
                value['stderr']=value['stderr'][:start]+wanted.group(3)+value['stderr'][end:]
            elif c['kind']=='value':assert value['exit']==0 and value['stderr']==''
            elif c['kind']=='control':assert value['exit']==1 and 'invalid utf-8' in value['stderr'].lower()
            expected.append(dict(id=c['id'],profile=profile,kind=c['kind'],expr=c['expr'],cwd=c['cwd'],expected=value,baseline_red=value!=obs(baseline)))
    files=[Path(__file__),D/'p1315-ab-cases.json',Path(args.measurement),D/'p1315-measurement.json',D/'p1315-record.py',D/'p1314-ab-cases.json',D/'p1314-ab-freeze.json',D/'p1314-ab-candidate-runs.json',D/'p1314-ab-runner.py',ROOT/'lab/typst-original/crates/typst-library/src/loading/csv.rs',FIX/'multiline.csv']
    prior_freeze=json.loads((D/'p1314-ab-freeze.json').read_text())
    files += [Path(p) for p in prior_freeze['inputs'] if p.startswith('/tmp/')]
    result=dict(schema='p1315-ab-freeze-v1',utc=state()['utc'],regime='A/B executado sem atestação de isolamento técnico',capabilities=dict(executor='/root/p1315_tests',reads=['L0 loading.md','vanilla csv.rs','P1315 measurement without embedded diff and recorder source only','P1314 A/B cases/freezes/runs/runner and fixtures','git HEAD/status/diff STAT','public CLI binaries','skill and references'],writes=['00_nucleo/diagnosticos/p1315-ab-*','/tmp/p1315-ab-fixtures'],context='Role-scoped task; no owner code, owner tests or candidate patch read. Shared filesystem has no technical isolation.'),unknown_policy='Missing, duplicate, malformed, timeout, crash, changed input, unsupported expression or unverified identity blocks. No Unknown silently passes.',budget='Single preparation and baseline; focal calibration only if unknown, then candidate normal/repeat/reverse. Two revisions without a new discriminating witness stop.',l0=dict(path=str(L0),raw_sha256=sha(L0),normative_sha256=normative(),exclusion='Only one canonical Hash do Código line'),inputs={str(p):sha(p) for p in files},binaries=measurement['binaries'],provenance=measurement['before'],expected=expected,scope='Only N in UnequalLengths changes, manually declared ordinal verified by vanilla fragment; complete baseline exit/stdout/stderr preserved except N. Historical P1314 corpus literal in original cwd.',calibration=dict(revisions=1,reason='Four UTF-8 controls initially hit unsupported bytes+bytes before CSV. Replaced expression construction with literal byte tuples representing identical bytes; focal rerun before final corpus. Candidate not inspected.',preliminary_sha256=sha(D/'p1315-ab-baseline-runs.json'),focal_sha256=sha(D/'p1315-ab-focal-runs.json'),initial_processes=1384,focal_processes=32,final_processes=len(measurement['rows'])),limitations=['No diagnostic suffix/span parity claimed.','Historical evidence referenced by hash; no historical source diff copied.','CLI With/Args coverage does not attest internal detached Rust carriers.','No general loader equivalence or refinement seal.','Tactical step is not a frozen normative input.'])
    save(args.output,result)
    print('freeze',sha(args.output),'expectations',len(expected),'RED',sum(x['baseline_red'] for x in expected))

def compare(args):
    f=json.loads(Path(args.freeze).read_text())
    for path,digest in f['inputs'].items():assert sha(path)==digest,('Unknown frozen input drift',path)
    assert normative()==f['l0']['normative_sha256'],'Unknown normative L0 drift'
    r=json.loads(Path(args.measurement).read_text())
    assert r['cases_sha256']==sha(D/'p1315-ab-cases.json') and r['runner_sha256']==sha(__file__)
    assert r['l0_normative_sha256']==normative()
    expected={(x['id'],x['profile']):x for x in f['expected']}
    required={(i,p,o) for i,p in expected for o in ('normal','repeat','reverse')}
    actual=[(x['id'],x['profile'],x['order']) for x in r['rows']]
    assert len(actual)==len(required) and set(actual)==required,'Unknown missing/duplicate observation'
    failures=[]
    for row in r['rows']:
        e=expected[row['id'],row['profile']]
        assert row['product']=='candidate' and row['argv'][2]==e['expr'] and row['cwd']==e['cwd']
        if obs(row)!=e['expected']:failures.append(dict(id=row['id'],profile=row['profile'],order=row['order'],expected=e['expected'],actual=obs(row)))
    save(args.output,dict(schema='p1315-ab-comparison-v1',utc=state()['utc'],freeze_sha256=sha(args.freeze),runs_sha256=sha(args.measurement),candidate=r['binaries'],comparisons=len(actual),unknown=0,failures=failures,status='FAIL' if failures else 'PASS'))
    print('comparison',len(actual),'failures',len(failures),'sha',sha(args.output))

if __name__=='__main__':
    p=argparse.ArgumentParser()
    p.add_argument('mode',choices=['prepare','run','freeze','compare'])
    p.add_argument('--binary');p.add_argument('--binary-sha256')
    p.add_argument('--orders',default='normal')
    p.add_argument('--ids')
    p.add_argument('--measurement');p.add_argument('--freeze');p.add_argument('--output')
    a=p.parse_args()
    if a.mode=='prepare':prepare()
    else:globals()[a.mode](a)
