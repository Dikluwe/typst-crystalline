#!/usr/bin/env python3
"""Independent P1310 public CLI suite. No candidate source or Rust tests read."""
import argparse, base64, concurrent.futures, datetime, hashlib, json, os
from pathlib import Path
import re, subprocess, sys, tempfile, time

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
D = ROOT / '00_nucleo/diagnosticos'
PROFILES = {'default': [], 'html': ['--features','html'], 'a11y': ['--features','a11y-extras'], 'html+a11y': ['--features','html,a11y-extras']}
BINS = {'vanilla': {'path':'/usr/local/bin/typst','sha256':'7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'}, 'baseline': {'path':'/dev/shm/p1309-r2-target.R2ZoSj/release/typst','sha256':'e54dcc9bb6a4c45cdc710066027ea255af6fb13f17b3715583ba4e9f6f3287c7'}}
FIXTURES = {'valid.json':'{"x":1}\n','valid.toml':'x = 1\n','valid.yaml':'x: 1\n','valid.xml':'<r>x</r>\n','valid.cbor':'ax\n','valid.csv':'a,b\n1,2\n','text.txt':'hello\n'}
def now(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(path): return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def digest(data): return hashlib.sha256(data).hexdigest()
def patchfiles(files):
    patch = '*** Begin Patch\n'
    for p, content in files.items():
        assert not Path(p).exists(), str(p)
        patch += '*** Add File: '+str(p)+'\n'+''.join('+'+line+'\n' for line in content.splitlines())
    patch += '*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, text=True, capture_output=True, check=True, cwd=ROOT)
def save(name, data): patchfiles({D/('p1310-ab-'+name+'.json'):json.dumps(data,ensure_ascii=False,indent=2)+'\n'})
def state():
    def git(*args): return subprocess.check_output(['git',*args],cwd=ROOT,text=True)
    return {'head':git('rev-parse','HEAD').strip(),'diff_stat':git('diff','HEAD','--stat'),'status':git('status','--short'),'at':now()}
def corpus():
    rows=[]
    def add(id,expr,policy='vanilla',kind='diagnostic',required=None):
        rows.append(dict(id=id,expression=expr,policy=policy,kind=kind,required_message=required,source_sha256=digest(expr.encode())))
    types={'integer':'42','float':'2.5','boolean':'true','none':'none','auto':'auto','dictionary':'(:)','array':'(1,2)','function':'(x=>x)','content':'[hello]','length':'3pt','angle':'4deg','ratio':'25%','fraction':'2fr','color':'red','label':'<tag>','type':'int','version':'version(1,2,3)','duration':'duration(seconds: 2)','datetime':'datetime(year: 2026, month: 1, day: 2)','decimal':'decimal("1.25")','symbol':'sym.alpha','direction':'ltr','alignment':'center','regex':'regex("x")','state':'state("x",0)','counter':'counter("x")','arguments':'arguments(1)','module':'calc'}
    for f in ('cbor','json','toml','xml','yaml'):
        for name,v in types.items():
            add(f+'.type.'+name, f+'('+v+')',required='expected path, string, or bytes, found ')
        routes={
          'multiline':'{\n let sentinel = 1;\n '+f+'( 42 )\n}',
          'alias':'{let f='+f+'; f(42)}',
          'with':'{let f='+f+'.with(42); f()}',
          'with-empty':'{let f='+f+'.with(); f(42)}',
          'args-spread':f+'(..arguments(42))',
          'array-spread':f+'(..(42,))',
          'sink':'{let forward(..args) = '+f+'(..args); forward(42)}',
          'filter':f+'(..arguments(42).filter(v=>true))',
          'map-detached':f+'(..arguments(42).map(v=>v))',
          'map-change-detached':f+'(..arguments(true).map(v=>42))',
          'join':f+'(..(arguments(42)+arguments()))',
        }
        for name,expr in routes.items(): add(f+'.route.'+name,expr,required='expected path, string, or bytes, found integer')
        data={'cbor':'bytes((1,))','json':'bytes("{\\"x\\":1}")','toml':'bytes("x = 1")','xml':'bytes("<r>x</r>")','yaml':'bytes("x: 1")'}[f]
        for name,expr in {'bytes':f+'('+data+')','str':f+'("valid.'+f+'")','path':f+'(path("valid.'+f+'"))'}.items(): add(f+'.valid.'+name,expr,'baseline','value')
        for name,expr in {'missing':f+'()','named':f+'(source: '+data+')','extra':f+'('+data+',42)','unknown-named':f+'('+data+',nope:42)'}.items(): add(f+'.preserve.'+name,expr,'baseline','any')
    for f in ('read','csv'): add(f+'.wrong',f+'(42)','baseline','diagnostic'); add(f+'.valid',f+'("'+('text.txt' if f=='read' else 'valid.csv')+'")','baseline','value')
    for f in ('json','toml','yaml','cbor'): add(f+'.encoder',f+'.encode((x: 1))','baseline','value')
    add('read.encoding','read("text.txt",encoding:42)','baseline','diagnostic')
    return rows
def resolved(stderr,expr):
    lines=expr.splitlines(keepends=True); rendered=stderr.splitlines(); primary=[]; traces=[]
    for i,line in enumerate(rendered):
        m=re.search(r'<input-expression>:(\d+):(\d+)',line)
        if not m: continue
        ln,col=map(int,m.groups())
        if ln<1 or ln>len(lines) or col>len(lines[ln-1]): raise ValueError('unresolved diagnostic range')
        start=len(''.join(lines[:ln-1]).encode())+len(lines[ln-1][:col].encode())
        if 'while calling' in line:
            snippet=rendered[i+1][4:]; name=re.search(r'while calling `([^`]+)`',line).group(1)
            if not expr.encode()[start:].startswith(snippet.encode()): raise ValueError('trace source mismatch')
            traces.append(dict(name=name,start=start,end=start+len(snippet.encode()),source='<input-expression>'))
        else:
            mark=next((re.search(r'\^+',x) for x in rendered[i+1:i+5] if '^' in x),None)
            if not mark: raise ValueError('missing primary underline')
            primary.append(dict(start=start,end=start+len(mark.group()),source='<input-expression>'))
    return dict(primary=primary or [{'kind':'detached'}],traces=traces)
def observe(job,folder):
    c,profile,side,binary=job
    argv=[binary['path'],'eval',c['expression'],'--format','json',*PROFILES[profile]]
    env={k:v for k,v in os.environ.items() if not k.startswith(('TYPST_','CRYSTALLINE_'))}; env.update(NO_COLOR='1',TERM='dumb')
    tick=time.monotonic(); at=now(); out=err=b''; code=None
    try:
        p=subprocess.run(argv,cwd=folder,env=env,capture_output=True,timeout=20); code,out,err=p.returncode,p.stdout,p.stderr
        stdout,stderr=out.decode(),err.decode()
        if code==0:
            if stderr: raise ValueError('success side diagnostic')
            obs={'kind':'value','exit':code,'value':json.loads(stdout),'stdout':stdout,'stderr':stderr}
        elif code==1 and stderr.startswith('error: '):
            obs={'kind':'diagnostic','exit':code,'stdout':stdout,'stderr':stderr,'messages':re.findall(r'^error: (.*)$',stderr,re.M),'hints':re.findall(r'^\s*= hint: (.*)$',stderr,re.M),**resolved(stderr,c['expression'])}
        else: raise ValueError('unsupported exit or diagnostic')
    except (OSError,ValueError,UnicodeError,subprocess.TimeoutExpired) as e: obs={'kind':'Unknown','reason':str(e)}
    return dict(case=c['id'],profile=profile,side=side,argv=argv,cwd=str(folder),binary=binary,at=at,seconds=time.monotonic()-tick,exit=code,stdout_base64=base64.b64encode(out).decode(),stderr_base64=base64.b64encode(err).decode(),stdout_sha256=digest(out),stderr_sha256=digest(err),observable=obs)
def run(cases,bins,folder,reverse=False):
    jobs=[(c,p,s,b) for c in cases for p in PROFILES for s,b in bins.items()]
    if reverse: jobs.reverse()
    with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool: return list(pool.map(lambda j:observe(j,folder),jobs))
def main():
    p=argparse.ArgumentParser(); p.add_argument('phase',choices=['freeze','normal','repeat','reverse']); p.add_argument('--candidate'); a=p.parse_args()
    before=state(); tick=time.monotonic()
    if a.phase=='freeze':
        assert sha(D/'p1310-baseline.json')=='3a19c3b842c5bdcc2d4e6df3ea8778a407a9843b2fc5ec183f3b07620c25d65f'
        cases=corpus(); folder=Path(tempfile.mkdtemp(prefix='p1310-ab-',dir='/tmp')); patchfiles({folder/k:v for k,v in FIXTURES.items()})
        for b in BINS.values(): assert sha(b['path'])==b['sha256']
        rows=run(cases,BINS,folder); lookup={(r['case'],r['profile'],r['side']):r['observable'] for r in rows}
        blockers=[]
        for c in cases:
            c['expected']={}
            for profile in PROFILES:
                o=lookup[c['id'],profile,c['policy']]; c['expected'][profile]=o
                if o['kind']=='Unknown' or (c['kind']!='any' and o['kind']!=c['kind']): blockers.append([c['id'],profile,'shape',o])
                if c['required_message'] and (o.get('messages') is None or not o['messages'][0].startswith(c['required_message'])): blockers.append([c['id'],profile,'required-message',o])
        base_red=sum(lookup[c['id'],p,'baseline']!=c['expected'][p] for c in cases if c['policy']=='vanilla' for p in PROFILES)
        manifest=dict(schema='p1310-ab-frozen-v1',at=now(),regime='A/B executado sem atestacao de isolamento tecnico',executor='/root/p1310_tests',candidate_source_read=False,permissions='Shared filesystem; logical allowlist only. Read L0/baseline/vanilla/historical diagnostic adapters; write p1310-ab-* only.',before=before,after=state(),baseline_receipt_sha256=sha(D/'p1310-baseline.json'),L0_path='00_nucleo/prompts/compiler/stdlib/loading.md',L0_sha256=sha(ROOT/'00_nucleo/prompts/compiler/stdlib/loading.md'),suite_sha256=sha(__file__),binaries=BINS,fixture_dir=str(folder),fixtures={k:dict(content=v,sha256=sha(folder/k)) for k,v in FIXTURES.items()},profiles=PROFILES,cases=cases,blockers=blockers,baseline_red_cells=base_red,adapter='p1307-r4-oracle.py full diagnostic envelope plus p1308-measure.py source resolution; eval-only ASCII fixtures, whole stderr and stdout exact; primary half-open source offsets and full trace source snippets; no diagnostic normalization.',adapter_pins={n:sha(D/n) for n in ('p1307-r4-oracle.py','p1308-measure.py')},unknown_policy='Any required Unknown blocks; never implicit success',revision_budget='Two focal revisions without gain require pause; no product mutants or full protocol attestation',seconds=time.monotonic()-tick)
        save('freeze-measurement',dict(before=before,rows=rows,after=state(),seconds=time.monotonic()-tick)); save('frozen',manifest)
        print(json.dumps(dict(cases=len(cases),cells=len(cases)*4,baseline_red_cells=base_red,blockers=blockers,frozen_sha256=sha(D/'p1310-ab-frozen.json'))))
    else:
        frozen=json.loads((D/'p1310-ab-frozen-r1.json').read_text()); assert not frozen['blockers']; assert sha(__file__)==frozen['suite_sha256']
        cases=frozen['cases']; folder=Path(frozen['fixture_dir']); assert a.candidate
        for k,v in frozen['fixtures'].items(): assert sha(folder/k)==v['sha256']
        candidate=dict(path=a.candidate,sha256=sha(a.candidate)); rows=run(cases,{'candidate':candidate},folder,a.phase=='reverse')
        lookup={c['id']:c for c in cases}; counts={}
        for r in rows:
            expected=lookup[r['case']]['expected'][r['profile']]; observed=r['observable']
            verdict='Unknown' if observed['kind']=='Unknown' else 'Preserved' if observed==expected else 'Violated'
            r['verdict']=verdict; counts[verdict]=counts.get(verdict,0)+1
        assert sha(a.candidate)==candidate['sha256']
        save(a.phase,dict(schema='p1310-ab-execution-v1',at=now(),phase=a.phase,before=before,after=state(),frozen_sha256=sha(D/'p1310-ab-frozen-r1.json'),suite_sha256=sha(__file__),candidate=candidate,seconds=time.monotonic()-tick,counts=counts,rows=rows))
        print(json.dumps(dict(counts=counts,failures=[dict(case=r['case'],profile=r['profile'],observable=r['observable']) for r in rows if r['verdict']!='Preserved'])))
if __name__=='__main__': main()
