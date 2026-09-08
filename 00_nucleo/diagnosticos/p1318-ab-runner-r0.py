"""P1318 independent public-observation A/B; no owner source or local tests."""
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
FIX = Path('/tmp/p1318-ab-fixtures')
CASES = D/'p1318-ab-cases.json'
BASE = '/tmp/p1317-target.y5u9ah/release/typst'
BASE_SHA = '9fcb4cbe830c74ec589506b0b86f52982abfdaab558f62cbf7a8f7de8665ccab'
VANILLA = '/usr/local/bin/typst'
VANILLA_SHA = '7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'
PROFILES = {'default': [], 'html':['--features','html'], 'a11y':['--features','a11y-extras'], 'html+a11y':['--features','html,a11y-extras']}

def sha(path): return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def read(path): return json.loads(Path(path).read_text())
def normative():
    raw=L0.read_bytes();pattern=rb'(?m)^Hash do C\xc3\xb3digo: [^\r\n]+\r?\n'
    assert len(re.findall(pattern,raw))==1
    return hashlib.sha256(re.sub(pattern,b'',raw)).hexdigest()
def save(path,value):
    path=Path(path);s=value if isinstance(value,str) else json.dumps(value,ensure_ascii=False,indent=2)+'\n'
    if path.exists():
        patch='*** Begin Patch\n*** Update File: '+str(path)+'\n@@\n'+''.join('-'+x+'\n' for x in path.read_text().splitlines())
    else: patch='*** Begin Patch\n*** Add File: '+str(path)+'\n'
    patch+=''.join('+'+x+'\n' for x in s.splitlines())+'*** End Patch\n'
    subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True)
def state():
    def git(*a):return subprocess.check_output(['git',*a],cwd=ROOT,text=True).strip()
    return dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=git('rev-parse','HEAD'),diff_stat=git('diff','HEAD','--stat'),status=git('status','--short'))
def obs(row):return {k:row[k] for k in ('exit','stdout','stderr')}
def b(raw):return 'bytes(('+','.join(map(str,raw))+',))'

def prepare(a):
    prior=read(D/'p1317-ab-cases.json')['cases']
    old={(r['id'],r['profile']):r for r in read(D/'p1317-ab-candidate-runs.json')['rows'] if r['order']=='normal'}
    cases=[]
    for c in prior:
        # Selection uses pre-candidate historical observations and public source;
        # all selected expressions have explicit Bytes and CSV parsing failure.
        target=old[c['id'],'default']['stderr'].startswith('error: failed to parse CSV (') and 'bytes(' in c['expr']
        x=dict(id='history-'+c['id'],prior_id=c['id'],expr=c['expr'],cwd=c['cwd'],kind='suffix' if target else 'literal',historical=True)
        if target and c.get('normative_only'):
            x['normative_only']=True
            assert ', bytes("another source")' in c['expr']
            x['position_probe']=c['expr'].replace(', bytes("another source")','')
        cases.append(x)
    def add(id,expr,kind='suffix',**kw):cases.append(dict(id=id,expr=expr,cwd=str(FIX),kind=kind,historical=False,**kw))
    data={
        'lf':'a,b\n1','crlf-between':'a,b\r\n1','cr':'a,b\r1',
        'crlf-unicode-cols':'α,😀\r\n1','cr-unicode-cols':'α,😀\r1',
        'bom-crlf':'\ufeffa,b\r\n1','empty-lines':'\r\n\na,b\r\n1',
        'quoted-crlf':'"α\r\n😀",b\r\n1','late-record':'a,b\r\n1,2\r\n3',
    }
    for name,sep in [('vt','\v'),('ff','\f'),('nel','\u0085'),('ls','\u2028'),('ps','\u2029')]:
        data[name+'-column']='α'+sep+'😀,b\r\n1'
        data[name+'-quoted']='"α'+sep+'😀",b\n1'
    raws={name:s.encode() for name,s in data.items()}
    raws.update({'invalid-crlf':b'a,b\r\n\xff,1','invalid-cr':b'a,b\r\xff,1','invalid-no-lf-prefix':b'\r\r\xff,a',
       'invalid-header':b'\xff,b','invalid-header-bom':b'\xef\xbb\xbf\xff,b',
       'invalid-after-earlier-crlf':b'a,b\r\n1\n\xff,2','invalid-after-earlier-ls':'α\u2028😀,b\r\n1\n'.encode()+b'\xff,2',
       'invalid-after-earlier-cr':'α,😀\r1\r'.encode()+b'\xff,2',
       'invalid-after-earlier-lf':b'a,b\n1\n\xff,2','invalid-quoted-ls':'"α\u2028😀",b\n'.encode()+b'\xff,2'})
    for mode in ('array','dictionary'):
        for name,raw in raws.items():add('new-'+name+'-'+mode,f'csv({b(raw)}, row-type: {mode})',raw=list(raw))
        for case in ('crlf-between','ls-column','invalid-after-earlier-ls'):
            src=b(raws[case]);opts='row-type: '+mode
            routes={'with':f'{{let f=csv.with({src}, {opts}); f()}}',
                'args':f'{{let a=arguments({opts}, {src}); csv(..a)}}',
                'detached':f'csv(..arguments(0).map((..a) => {src}), {opts})',
                'array-spread':f'csv(..({src},), {opts})',
                'sink':f'{{let f(..a)=csv(..a); f({src}, {opts})}}'}
            for route,expr in routes.items():add('new-route-'+case+'-'+route+'-'+mode,expr,detached=route=='detached')
            probe=f'csv({src}, {opts})'
            add('new-excess-'+case+'-'+mode,f'csv({src}, 42, {opts})',normative_only=True,position_probe=probe)
        for name in ('crlf-between','invalid-after-earlier-ls'):
            for route in ('str','path'):
                p='"'+name+'.csv"';p='path('+p+')' if route=='path' else p
                add('control-file-'+name+'-'+route+'-'+mode,f'csv({p}, row-type: {mode})','literal')
        for name,raw in [('empty',b''),('unicode','α,😀\r\nβ,δ'.encode()),('unicode-quotes','"α\u2028😀",b\r\n1,2'.encode())]:
            src='bytes(())' if not raw else b(raw)
            add('control-valid-'+name+'-'+mode,f'csv({src}, row-type: {mode})','literal',check='success')
    src=b(raws['invalid-after-earlier-ls'])
    controls={'delimiter':f'csv({src}, delimiter: "xx")','row-type':f'csv({src}, row-type: str)',
        'unknown':f'csv({src}, nope: true)','cast':'csv(42)', 'missing':'csv()',
        'option-before-io':'csv("missing.csv", delimiter: "xx")','namespace':'csv.encode',
        'json':'json(bytes((255,)))','yaml':'yaml(bytes((255,)))','toml':'toml(bytes((255,)))','xml':'xml(bytes((255,)))','cbor':'cbor(bytes((255,)))',
        'read':'read("invalid-after-earlier-ls.csv")','read-binary':'read("invalid-after-earlier-ls.csv", encoding: none)',
        'json-encode':'json.encode((a: 1))','yaml-encode':'yaml.encode((a: 1))','toml-encode':'toml.encode((a: 1))','cbor-encode':'cbor.encode((a: 1))'}
    for name,expr in controls.items():add('control-'+name,expr,'literal')
    FIX.mkdir(exist_ok=True);fixtures={}
    for name in ('crlf-between','invalid-after-earlier-ls'):
        p=FIX/(name+'.csv');raw=raws[name]
        assert not p.exists() or p.read_bytes()==raw
        if not p.exists():p.write_bytes(raw)
        fixtures[str(p)]=dict(bytes=list(raw),sha256=sha(p))
    save(CASES,dict(schema='p1318-ab-cases-v1',cases=cases,fixtures=fixtures,policy='Only declared suffix cases append measured at L:C before closing cause parenthesis; all other public observation literal baseline. Historical replay exact before transformation.'))
    print('cases',len(cases),'suffix',sum(c['kind']=='suffix' for c in cases),flush=True)

def run(a):
    cases=read(CASES)['cases']
    if a.ids:cases=[c for c in cases if c['id'] in a.ids.split(',')]
    binaries={'candidate':(a.binary,a.binary_sha256)} if a.binary else {'baseline':(BASE,BASE_SHA),'vanilla':(VANILLA,VANILLA_SHA)}
    for p,h in binaries.values():assert sha(p)==h,('Unknown binary identity',p)
    env=dict(os.environ)
    for k in ('TYPST_FEATURES','TYPST_DIAGNOSTIC_FORMAT','TYPST_ROOT'):env.pop(k,None)
    env.update(NO_COLOR='1',TERM='dumb',PYTHONDONTWRITEBYTECODE='1')
    before=state();start=time.monotonic();tasks=[]
    for order in a.orders.split(','):
        for profile,flags in PROFILES.items():
            for c in reversed(cases) if order=='reverse' else cases:
                for product,(p,h) in binaries.items():
                    if product=='vanilla' and c['kind']=='literal':continue
                    tasks.append(dict(id=c['id'],profile=profile,order=order,product=product,argv=[p,'eval',c['expr'],*flags],cwd=c['cwd']))
                if not a.binary and c.get('position_probe'):
                    tasks.append(dict(id=c['id'],profile=profile,order=order,product='position-probe',argv=[VANILLA,'eval',c['position_probe'],*flags],cwd=c['cwd']))
    def execute(t):
        tick=time.monotonic();utc=datetime.datetime.now(datetime.timezone.utc).isoformat()
        p=subprocess.run(t['argv'],cwd=t['cwd'],env=env,capture_output=True,timeout=30)
        return dict(t,utc=utc,seconds=time.monotonic()-tick,exit=p.returncode,stdout=p.stdout.decode(),stderr=p.stderr.decode())
    with ThreadPoolExecutor(max_workers=4) as pool:rows=list(pool.map(execute,tasks))
    for p,h in binaries.values():assert sha(p)==h
    save(a.output,dict(schema='p1318-ab-runs-v1',before=before,after=state(),wall_seconds=time.monotonic()-start,processes=len(rows),runner_sha256=sha(__file__),cases_sha256=sha(CASES),l0_normative_sha256=normative(),binaries={k:dict(path=p,sha256=h) for k,(p,h) in binaries.items()},rows=rows))
    print('runs',len(rows),'seconds',time.monotonic()-start,flush=True)

def freeze(a):
    m=read(a.measurement);cases=read(CASES)['cases']
    assert m['runner_sha256']==sha(__file__) and m['cases_sha256']==sha(CASES) and m['l0_normative_sha256']==normative()
    prior={(r['id'],r['profile']):r for r in read(D/'p1317-ab-candidate-runs.json')['rows'] if r['order']=='normal'}
    by={(r['id'],r['profile'],r['product']):r for r in m['rows']};assert len(by)==len(m['rows'])
    expected=[]
    for c in cases:
        for profile in PROFILES:
            row=by[c['id'],profile,'baseline'];e=obs(row);full_parity=False;position=None
            assert e['exit'] in (0,1),('Unknown process',c['id'])
            if c['historical']:
                p=prior[c['prior_id'],profile]
                assert row['cwd']==p['cwd'] and row['argv'][2]==p['argv'][2] and e==obs(p),('Unknown replay drift',c['id'])
            if c.get('check')=='success':assert e['exit']==0,('Unknown valid control',c['id'],e)
            if c['kind']=='suffix':
                assert e['exit']==1 and not e['stdout'] and len(re.findall(r'(?m)^error: ',e['stderr']))==1
                first,sep,rest=e['stderr'].partition('\n')
                assert first.startswith('error: failed to parse CSV (') and first.endswith(')') and not re.search(r' at \d+:\d+\)$',first),('Unknown parsing',c['id'],e)
                v=obs(by[c['id'],profile,'vanilla'])
                if c.get('normative_only'):
                    assert v['stderr'].startswith('error: unexpected argument'),('Unknown excess boundary',c['id'],v)
                    pv=obs(by[c['id'],profile,'position-probe'])
                else:pv=v
                line=pv['stderr'].splitlines()[0];match=re.fullmatch(re.escape(first[:-1])+r' at ([1-9][0-9]*):([1-9][0-9]*)\)',line)
                assert match,('Unknown position oracle',c['id'],first,line)
                position=[int(v) for v in match.groups()]
                e['stderr']=line+sep+rest
                if not c.get('normative_only'):
                    assert e==v,('Unknown non-position parity gap',c['id'],e,v)
                    full_parity=True
                if c.get('detached'):assert '┌─' not in rest,('Unknown detached',c['id'])
            expected.append(dict(id=c['id'],profile=profile,kind=c['kind'],expr=c['expr'],cwd=c['cwd'],expected=e,position=position,full_vanilla_parity=full_parity,normative_only=c.get('normative_only',False),baseline_red=e!=obs(row)))
    old=read(D/'p1317-ab-freeze.json')
    files=[Path(__file__),CASES,Path(a.measurement),D/'p1317-ab-cases.json',D/'p1317-ab-candidate-runs.json',D/'p1317-ab-freeze.json']
    files += [Path(p) for p in old['inputs'] if p.startswith('/tmp/')]
    files += [Path(p) for p in read(CASES)['fixtures']]
    files += [ROOT/'lab/typst-original/crates/typst-library/src'/p for p in ('loading/csv.rs','diag.rs')]
    files += [ROOT/'lab/typst-original/crates/typst-syntax/src'/p for p in ('lines.rs','lexer.rs')]
    skill=Path('/home/dikluwe/.codex/skills/tekt-materializacao-segregada')
    files += [skill/p for p in ('SKILL.md','references/papeis-e-capacidades.md','references/artefatos-e-gates.md')]
    save(a.output,dict(schema='p1318-ab-freeze-v1',utc=state()['utc'],regime='A/B executado sem atestação de isolamento técnico',capabilities=dict(executor='/root/p1318_tests',reads=['L0 loading.md','vanilla loading/csv.rs, diag.rs, syntax lines.rs/lexer.rs','p1317-ab-* and historical fixtures','binary identities and public executions','git HEAD/status/diff STAT','skill/references and applicable ADRs'],writes=['00_nucleo/diagnosticos/p1318-ab-*','/tmp/p1318-ab-fixtures'],context='Independent task-scoped agent; owner/local tests/patch/source receipts not read. Shared filesystem, no technical isolation attestation.'),unknown_policy='Missing, malformed, unsupported, timeout, crash, replay/input drift or identity ambiguity blocks; never defaults PASS.',budget='One full baseline, focal corrections only; two consecutive revisions without gain on same cause stop. Candidate normal/repeat/reverse after release only.',l0=dict(path=str(L0),raw_sha256=sha(L0),normative_sha256=normative(),exclusion='Only one canonical Hash do Código line'),inputs={str(p):sha(p) for p in files},binaries=m['binaries'],provenance=m['before'],expected=expected,calibration=dict(revisions=0,processes=m['processes'],wall_seconds=m['wall_seconds']),limitations=['CLI cannot attest decoder API, synthetic Rust Args without occurrences, offset overflow/fallback, World counts or single parser invocation. Local tests and source audit belong to other authority.','No mutation score, seal, general equivalence or isolation attestation. Excess parsing is normative separately from vanilla.']))
    print('freeze',sha(a.output),'observations',len(expected),'RED',sum(e['baseline_red'] for e in expected),flush=True)

def compare(a):
    f=read(a.freeze);m=read(a.measurement)
    for p,h in f['inputs'].items():assert sha(p)==h,('Unknown input drift',p)
    assert normative()==f['l0']['normative_sha256']==m['l0_normative_sha256']
    assert m['runner_sha256']==sha(__file__) and m['cases_sha256']==sha(CASES)
    expect={(r['id'],r['profile']):r for r in f['expected']}
    required={(i,p,o) for i,p in expect for o in ('normal','repeat','reverse')}
    keys=[(r['id'],r['profile'],r['order']) for r in m['rows']]
    assert len(keys)==len(required) and set(keys)==required,'Unknown missing/duplicate'
    failures=[]
    for r in m['rows']:
        e=expect[r['id'],r['profile']]
        assert r['product']=='candidate' and r['argv'][2]==e['expr'] and r['cwd']==e['cwd']
        if obs(r)!=e['expected']:failures.append(dict(id=r['id'],profile=r['profile'],order=r['order'],expected=e['expected'],actual=obs(r)))
    save(a.output,dict(schema='p1318-ab-comparison-v1',utc=state()['utc'],freeze_sha256=sha(a.freeze),measurement_sha256=sha(a.measurement),candidate=m['binaries'],comparisons=len(keys),unknown=0,failures=failures,status='FAIL' if failures else 'PASS',processes=m['processes'],wall_seconds=m['wall_seconds']))
    print('comparisons',len(keys),'failures',len(failures),flush=True)

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('mode',choices=['prepare','run','freeze','compare'])
    for name in ('binary','binary-sha256','ids','measurement','freeze','output'):p.add_argument('--'+name)
    p.add_argument('--orders',default='normal');args=p.parse_args();globals()[args.mode](args)
