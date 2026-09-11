"""Independent public-execution transport for P1339; contains no expectations.

The caller supplies a contract-derived immutable execution plan. This module
never reads productive source and never creates the canonical RED before seal.
"""
import argparse
from concurrent.futures import ThreadPoolExecutor
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile
import time

ROOT=Path('/repos/Antigravity/typst-crystalline')
D=ROOT/'00_nucleo/diagnosticos'
MANIFEST=D/'p1339-authority-manifest-r2.json'
MANIFEST_SHA='842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b'
L0_FREEZE=D/'p1339-l0-freeze.json'
L0_FREEZE_SHA='397c136fc8710d44b2f7537193fe5b9c44ab89d40a99bf6b994296e05e7c4d04'
BINARIES={
    'vanilla':('/usr/local/bin/typst','7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'),
    'baseline':('/tmp/p1338-target.vlNAmp/release/typst','f7c8085f8453ecb6b10841b698092d41e7fbec64d9d46f9341b9b9b538bfefa1'),
}
PROFILES={'default':[],'html':['--features','html'],'a11y':['--features','a11y-extras'],'html+a11y':['--features','html,a11y-extras']}

def sha(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def read(path):return json.loads(Path(path).read_text())
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def state():
    def git(*args):return subprocess.check_output(['git',*args],cwd=ROOT,text=True).rstrip('\n')
    return dict(utc=utc(),head=git('rev-parse','HEAD'),status=git('status','--short'),diff_stat=git('diff','HEAD','--stat'))

def save_new(path,value):
    path=Path(path).resolve()
    assert path.parent==D and path.name.startswith('p1339-ab-'),('Output outside oracle authority',str(path))
    assert not path.exists(),('Immutable output already exists',str(path))
    text=json.dumps(value,ensure_ascii=True,indent=2)+'\n'
    patch='*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+line+'\n' for line in text.rstrip('\n').split('\n'))+'*** End Patch\n'
    subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True)

def checked_inputs(plan):
    assert sha(MANIFEST)==MANIFEST_SHA and sha(L0_FREEZE)==L0_FREEZE_SHA,'Authority input drift'
    assert plan['authority_manifest_sha256']==MANIFEST_SHA
    assert sha(D/'p1339-contract.json')==plan['contract_sha256'],'Contract identity drift'
    for path,digest in plan.get('input_sha256',{}).items():
        # Paths are public artifacts/fixtures or L0, never productive source.
        p=Path(path).resolve()
        assert not any(str(p).startswith(str(ROOT/x)+'/') for x in ('01_core','02_shell','03_infra','04_wiring','lab'))
        assert sha(p)==digest,('Input drift',path)
    for c in plan['cases']:
        if c.get('source_path'):assert sha(c['source_path'])==c['source_sha256'],('Fixture drift',c['id'])

def execute(case,binary,product,profile,order,outdir,ordinal,binary_digest):
    mode=case['mode'];flags=PROFILES[profile];input_text=case.get('stdin','')
    out=None
    if mode=='eval':
        argv=[binary,'eval','--format','json',case['expression'],*flags]
    elif mode=='compile':
        out=outdir/(str(ordinal)+'.pdf')
        argv=[binary,'compile',case.get('source_path','-'),str(out),*flags]
    elif mode=='query':
        assert profile=='default','Query transport cannot assert nondefault profile coverage'
        argv=[binary,'query',case.get('source_path','-'),case.get('selector','metadata'),'--field',case.get('field','value'),'--format','json']
    else:raise ValueError('Unknown execution mode: '+mode)
    # Use an explicit, reproducible environment rather than retaining unrelated
    # process credentials, mutation switches or product feature overrides.
    allowed=('PATH','HOME','LANG','LC_ALL','LC_CTYPE','TZ','XDG_CACHE_HOME','XDG_DATA_HOME','XDG_CONFIG_HOME','FONTCONFIG_FILE','FONTCONFIG_PATH','TYPST_FONT_PATHS')
    env={key:os.environ[key] for key in allowed if key in os.environ}
    env.update(NO_COLOR='1',TERM='dumb',PYTHONDONTWRITEBYTECODE='1')
    start=utc();tick=time.monotonic();timeout=False
    try:
        p=subprocess.run(argv,input=input_text.encode(),cwd=case.get('cwd',str(ROOT)),env=env,capture_output=True,timeout=case.get('timeout_seconds',30))
        code=p.returncode;stdout=p.stdout.decode('utf-8',errors='surrogateescape');stderr=p.stderr.decode('utf-8',errors='surrogateescape')
        execution='Observed' if code in (0,1) else 'Unknown'
    except subprocess.TimeoutExpired as e:
        timeout=True;code=None;stdout=(e.stdout or b'').decode('utf-8',errors='replace');stderr=(e.stderr or b'').decode('utf-8',errors='replace');execution='Unknown'
    artifacts=[] if out is None else [dict(kind='pdf',path=str(out),present=out.exists(),sha256=sha(out) if out.exists() else None,semantic_predicate='presence only; bytes are provenance, never parity')]
    return dict(schema='p1339-observation-v1',oracle_id=case.get('oracle_id',case['id']),binary_sha256=binary_digest,start_utc=start,end_utc=utc(),unknown_reason=('timeout' if timeout else 'abnormal_exit') if execution=='Unknown' else None,case_id=case['id'],id=case['id'],product=product,profile=profile,order=order,adapter=case.get('adapter',mode),mode=mode,argv=argv,env=env,cwd=case.get('cwd',str(ROOT)),stdin=input_text,start=start,end=utc(),seconds=time.monotonic()-tick,exit=code,stdout=stdout,stderr=stderr,artifacts=artifacts,execution=execution,timeout=timeout,source_sha256=case.get('source_sha256') or hashlib.sha256(input_text.encode()).hexdigest() if mode!='eval' else hashlib.sha256(case['expression'].encode()).hexdigest(),pdf_exists=out.exists() if out else None,pdf_sha256=sha(out) if out and out.exists() else None)

def run(args):
    plan=read(args.plan);checked_inputs(plan)
    if args.phase in ('red','candidate'):
        assert args.seal and args.seal_sha256,'RED/candidate require an identified verifier seal'
        assert sha(args.seal)==args.seal_sha256,'Seal identity drift'
        # Seal validity and coverage are judged by the separate verifier. The
        # operator supplies its accepted immutable identity; this tool records it.
    elif args.phase!='calibration':raise ValueError('Invalid execution phase')
    binaries={p:BINARIES[p] for p in args.products.split(',') if p!='candidate'}
    if 'candidate' in args.products.split(','):
        assert args.phase=='candidate' and args.binary and args.binary_sha256
        binaries['candidate']=(args.binary,args.binary_sha256)
    assert all(sha(p)==h for p,h in binaries.values()),'Binary identity drift'
    cases=plan['cases']
    if args.ids:
        wanted=set(args.ids.split(','));assert wanted <= {c['id'] for c in cases},'Unregistered requested case'
        cases=[c for c in cases if c['id'] in wanted]
    assert cases and len({c['id'] for c in cases})==len(cases),'Missing or duplicate case'
    orders=args.orders.split(',');assert all(o in ('normal','repeat','reverse') for o in orders)
    outdir=Path(tempfile.mkdtemp(prefix='p1339-ab-'));before=state();tick=time.monotonic();tasks=[]
    for order in orders:
        for profile in PROFILES:
            for c in reversed(cases) if order=='reverse' else cases:
                if profile not in c.get('profiles',list(PROFILES)):continue
                for product,(binary,digest) in binaries.items():tasks.append((c,binary,product,profile,order,outdir,len(tasks),digest))
    with ThreadPoolExecutor(max_workers=4) as pool:rows=list(pool.map(lambda t:execute(*t),tasks))
    checked_inputs(plan);assert all(sha(p)==h for p,h in binaries.values()),'Binary changed during run'
    result=dict(schema='p1339-ab-execution-v2',authority_manifest_sha256=MANIFEST_SHA,l0_freeze_sha256=L0_FREEZE_SHA,plan_sha256=sha(args.plan),runner_sha256=sha(__file__),phase=args.phase,seal=dict(path=args.seal,sha256=args.seal_sha256) if args.seal else None,regime='executado sem atestação de isolamento',executor='/root/p1319_tests',context='P1319 independent A/B history only; no P1339 productive source or candidate inspected',before=before,after=state(),binaries={p:dict(path=b,sha256=h) for p,(b,h) in binaries.items()},output_dir=str(outdir),processes=len(rows),wall_seconds=time.monotonic()-tick,rows=rows)
    save_new(args.output,result)
    print('Recorded',len(rows),'processes;',sum(r['execution']!='Observed' for r in rows),'unobserved;',round(result['wall_seconds'],3),'seconds',flush=True)

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--plan',required=True);p.add_argument('--phase',choices=['calibration','red','candidate'],required=True);p.add_argument('--products',default='vanilla');p.add_argument('--orders',default='normal');p.add_argument('--output',required=True)
    for name in ('ids','binary','binary-sha256','seal','seal-sha256'):p.add_argument('--'+name)
    run(p.parse_args())
