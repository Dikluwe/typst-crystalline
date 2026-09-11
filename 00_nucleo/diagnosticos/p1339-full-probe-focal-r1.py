"""P1339 A.1 measurement only; independent of crystalline source/candidate.

All persistent output uses apply_patch. No contract, seal, or final PASS is emitted.
"""
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / '00_nucleo/diagnosticos'
PROFILES = {'default': [], 'html': ['--features', 'html'], 'a11y': ['--features', 'a11y-extras'], 'html+a11y': ['--features', 'html,a11y-extras']}
INPUTS = ['00_nucleo/materialization/typst-passo-1339.md'] + ['00_nucleo/diagnosticos/p1339-'+s+'.json' for s in ['resume-r1', 'authority-manifest-r1', 'a0', 'probe-manifest']]

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def sha(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

def save(suffix, obj):
    path = D / ('p1339-full-' + suffix + '.json')
    assert not path.exists(), path
    body = json.dumps(obj, ensure_ascii=True, indent=2) + '\n'
    patch = '*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+line+'\n' for line in body.splitlines())+'*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, text=True, capture_output=True, check=True, cwd=ROOT)
    print(suffix, sha(path), flush=True)

def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT).decode()

def provenance():
    return dict(at=now(), head=git('rev-parse', 'HEAD').strip(), status_short=git('status', '--short'), diff_stat=git('diff', 'HEAD', '--stat'), input_sha256={p:sha(ROOT/p) for p in INPUTS}, runner_sha256=sha(__file__))

def cases():
    out=[]
    def add(id, route, body, obligation, kind='exploratory'):
        out.append(dict(id=id, route=route, expression=body if body.startswith('{') else 'repr('+body+')', obligation=obligation, preclassification='Violated' if kind=='discovery' else 'Unknown', preclassification_basis='P1335 absence' if kind=='discovery' else 'Unmeasured boundary; classification awaits bilateral observations', kind=kind))
    routes=['angle.deg','angle.rad','float.inf','float.nan','float.signum','float.from-bytes','float.to-bytes','function.with','function.where','version.at']
    for route in routes:
        add('discovery-'+route, route, '(type('+route+'), repr('+route+'))', 'discovery,type,repr,name', 'discovery')
    for unit in ['deg','rad']:
        route='angle.'+unit
        for val in ['0deg','-0deg','90deg','-90deg','0rad','-0rad','1rad','-1rad','(float("inf") * 1deg)','(float("nan") * 1rad)']:
            for form in ['static','bound']:
                call=f'{route}({val})' if form=='static' else f'({val}).{unit}()'
                add(f'{route}-{form}-{val}',route,f'{{ let x = {call}; repr((type(x), x, x == x, x.signum())) }}','Angle to float; signed, units, nonfinite')
        for args in ['', '1', '1.5', '-0.0', '"bad"', '90deg, 1', '90deg, other: 1', 'value: 90deg']:
            add(route+'-invalid-'+args,route,f'{route}({args})','missing,extra,named,type rejection')
        add(route+'-method-value',route,f'{{ let f = (90deg).{unit}; repr((type(f), repr(f), f())) }}','bound method extraction')
        add(route+'-zero-observable',route,f'({route}(0deg), {route}(-0deg), {route}(0deg).signum(), {route}(-0deg).signum())','signed zero public signum')
    for constant in ['inf','nan']:
        route='float.'+constant
        add(route+'-arithmetic',route,f'{{ let x = {route}; repr((type(x),repr(x),x == x,x != x,x + 1,x * -1,x - x)) }}','constant type,repr,equality,arithmetic')
    for val in ['1.5','-1.5','0.0','-0.0','float("inf")','-float("inf")','float("nan")','1','-1','true','"1"','1deg','none']:
        for form in ['static','bound']:
            call=f'float.signum({val})' if form=='static' else f'({val}).signum()'
            add(f'signum-{form}-{val}','float.signum',f'{{ let x = {call}; repr((type(x), x, x == x)) }}','signum signed zero,finite,infinite,nan,coercion')
    for args in ['', '1.0, 2', '1.0, other: 1', 'value: 1.0']:
        add('signum-invalid-'+args,'float.signum',f'float.signum({args})','arity,named')
    add('signum-method-value','float.signum','{ let f = (-0.0).signum; repr((type(f),repr(f),f())) }','bound extraction')
    for size in [4,8]:
        for endian in ['little','big']:
            for val in ['0.0','-0.0','1.5','-2.25','float("inf")','-float("inf")','float("nan")']:
                for form in ['static','bound']:
                    call=f'float.to-bytes({val}, size: {size}, endian: "{endian}")' if form=='static' else f'({val}).to-bytes(size: {size}, endian: "{endian}")'
                    add(f'to-bytes-{size}-{endian}-{form}-{val}','float.to-bytes',f'{{ let b = {call}; repr((type(b), array(b))) }}','binary32/64,endian,exact byte API result')
                add(f'roundtrip-{size}-{endian}-{val}','float.from-bytes',f'{{ let b = float.to-bytes({val}, size: {size}, endian: "{endian}"); let x = float.from-bytes(b, endian: "{endian}"); repr((type(x), x, x == x, x.signum())) }}','roundtrip zero,signedzero,finite,infinite,nan')
    for endian,raw in [('little','(0,0,192,63)'),('big','(63,192,0,0)'),('little','(0,0,0,0,0,0,248,63)'),('big','(63,248,0,0,0,0,0,0)')]:
        add('from-bytes-direct-'+endian+raw,'float.from-bytes',f'float.from-bytes(bytes({raw}), endian: "{endian}")','independent 4/8 input length,endian')
    for n in [0,3,5,7,9]:
        add('from-bytes-length-'+str(n),'float.from-bytes',f'float.from-bytes(bytes(({"0,"*n})))','invalid byte lengths')
    for args in ['', 'bytes((0,0,192,63)), 1', 'bytes((0,0,192,63)), other: 1', 'bytes((0,0,192,63)), endian: "middle"', 'bytes((0,0,192,63)), endian: 1', '(0,0,192,63)', '"bad"', '1', 'value: bytes((0,0,192,63))']:
        add('from-bytes-invalid-'+args,'float.from-bytes',f'float.from-bytes({args})','arity,named,endian,type diagnostics')
    add('from-bytes-default','float.from-bytes','float.from-bytes(bytes((0,0,192,63)))','default endian')
    add('from-bytes-bound-bytes','float.from-bytes','bytes((0,0,192,63)).from-bytes()','bound applicability')
    add('from-bytes-bound-float','float.from-bytes','(1.0).from-bytes(bytes((0,0,192,63)))','bound applicability')
    for size in [0,1,3,5,7,9,-1]:
        add('to-bytes-size-'+str(size),'float.to-bytes',f'float.to-bytes(1.5, size: {size})','invalid sizes')
    for args in ['', '1.5, 2', '1.5, other: 1', '1.5, endian: "middle"', '1.5, endian: 1', '1.5, size: "4"', '1.5, size: 4.0', 'true', '"1.5"', '1', 'value: 1.5']:
        add('to-bytes-invalid-'+args,'float.to-bytes',f'float.to-bytes({args})','arity,named,endian,size,type')
    add('to-bytes-default','float.to-bytes','{ let b = float.to-bytes(1.5); repr((type(b), array(b))) }','default endian,size')
    add('to-bytes-default-endian','float.to-bytes','array(float.to-bytes(1.5, size: 4))','default endian independent size')
    add('to-bytes-method-value','float.to-bytes','{ let f = (1.5).to-bytes; repr((type(f),repr(f),array(f()))) }','bound extraction')
    with_cases=[
        ('closure-pos','let f = (a,b,c) => (a,b,c); let g = CALL(f,1,2); repr((type(g),repr(g),g(3)))'),
        ('closure-named','let f = (a:0,b:0) => (a,b); let g = CALL(f,a:1); repr(g(b:2))'),
        ('closure-mixed','let f = (a,b,c:0) => (a,b,c); let g = CALL(f,1,c:3); repr(g(2))'),
        ('closure-variadic','let f = (..args) => (args.pos(),args.named()); let g = CALL(f,1,z:2); repr(g(3,y:4))'),
        ('native','let g = CALL(calc.pow,2); repr((type(g),repr(g),g(3)))'),
        ('element','let g = CALL(heading,level:2); repr((type(g),repr(g),g[Hi]))'),
        ('multi','let f = (..args) => (args.pos(),args.named()); let g = CALL(f,1,a:2); let h = CALL(g,3,b:4); repr(h(5,c:6))'),
        ('named-override','let f = (a:0) => a; let g = CALL(f,a:1); repr(g(a:2))'),
        ('duplicate-named','let f = (a:0) => a; let g = CALL(f,a:1,a:2); repr(g())'),
        ('spread','let f = (..args) => (args.pos(),args.named()); let g = CALL(f,..(1,2),..(a:3)); repr(g(..(4,5),..(b:6)))'),
        ('spread-duplicate','let f = (a:0) => a; let g = CALL(f,..(a:1),..(a:2)); repr(g())'),
        ('panic-preargs','let f = (..args) => args; CALL(f,panic("first"),panic("second"))'),
        ('panic-callargs','let f = (..args) => args; let g = CALL(f,1); g(panic("first"),panic("second"))'),
        ('invalid-receiver-panic','CALL(1,panic("argument"))'),
        ('receiver-panic','CALL(panic("receiver"),panic("argument"))'),
        ('lazy-receiver','let f = () => panic("body"); repr(CALL(f))'),
        ('missing-receiver','repr(function.with())'),
        ('invalid-receiver','repr(CALL(1))'),
    ]
    for id,body in with_cases:
        for form in ['static','bound']:
            expr=body.replace('CALL(', 'function.with(') if form=='static' else body
            if form=='bound':
                # Deliberately finite substitutions preserve original argument occurrence order.
                for receiver in ['calc.pow','heading','panic("receiver")','f','g','1']:
                    expr=expr.replace('CALL('+receiver+',', '('+receiver+').with(').replace('CALL('+receiver+')','('+receiver+').with()')
            add('with-'+form+'-'+id,'function.with','{ '+expr+' }','with '+id)
    where_cases=[('element','heading,level:2'),('element-multifield','heading,level:2,outlined:false'),('closure','(() => 1)'),('native','calc.abs'),('nonfunction','1'),('missing',''),('positional','heading,1'),('unknown-field','heading,not-a-field:3'),('duplicate','heading,level:1,level:2'),('spread','heading,..(level:2,outlined:false)'),('spread-duplicate','heading,..(level:1),..(level:2)'),('panic-order','heading,level:panic("first"),outlined:panic("second")'),('invalid-receiver-panic','1,level:panic("argument")'),('receiver-panic','panic("receiver"),level:panic("argument")')]
    for id,args in where_cases:
        add('where-static-'+id,'function.where',f'function.where({args})','where '+id)
        if args and id!='receiver-panic':
            rec,sep,rest=args.partition(',')
            add('where-bound-'+id,'function.where',f'({rec}).where({rest})','where bound '+id)
    add('where-closure-noexecute','function.where','function.where(() => panic("body"))','where must not execute receiver')
    add('where-selector-morphology','function.where','{ let s = function.where(heading,level:2); repr((type(s),repr(s))) }','selector type and morphology')
    add('with-method-value','function.with','{ let f = calc.pow.with; repr((type(f),repr(f),f(2)(3))) }','method extraction')
    add('where-method-value','function.where','{ let f = heading.where; repr((type(f),repr(f),f(level:2))) }','method extraction')
    for elem,field in [('heading','level:2'),('figure','kind:"image"'),('strong','body:[Hi]'),('emph','body:[Hi]'),('raw','lang:"rust"'),('text','text:"Hi"'),('table','columns:2')]:
        for args in ['',field,'not-a-field:1',field+',..('+field+')']:
            for form in ['static','bound']:
                call=f'function.where({elem}'+(','+args if args else '')+')' if form=='static' else f'{elem}.where({args})'
                add('where-expanded-'+form+'-'+elem+'-'+args,'function.where',f'{{ let s = {call}; repr((type(s),repr(s))) }}','element family,empty fields,unknown field,spread duplicate')
        add('where-with-element-'+elem,'function.where',f'function.where({elem}.with())','With element is not direct element')
    for elem,content in [('strong','strong[BASE]'),('emph','emph[BASE]'),('text','text("BASE")'),('heading','heading(level:2)[BASE]'),('raw','raw("BASE")')]:
        add('where-show-morphology-'+elem,'function.where',f'{{ show function.where({elem}): it => [MATCH]; repr({content}) }}','show selector application/content morphology (eval may defer realization)')
    for route,receiver in [('angle.deg','90deg'),('angle.rad','90deg'),('float.signum','1.0'),('float.to-bytes','1.0'),('version.at','version(1,2,3)')]:
        method=route.split('.')[1]
        base='0,' if route=='version.at' else ''
        for tail in ['1,2','other:1']:
            add(route+'-bound-invalid-'+tail,route,f'({receiver}).{method}({base}{tail})','bound arity and named diagnostics')
    for raw in ['(0,0,0,0)','(0,0,0,128)','(0,0,128,127)','(0,0,128,255)','(1,0,192,127)','(0,0,0,0,0,0,0,128)','(1,0,0,0,0,0,248,127)']:
        add('from-bytes-independent-bits-'+raw,'float.from-bytes',f'{{ let x = float.from-bytes(bytes({raw})); repr((type(x),x,x==x,x.signum())) }}','independent signed zero,infinity,NaN payload')
    for route,receiver in [('angle.deg','90deg'),('float.signum','1.0'),('float.to-bytes','1.0')]:
        add(route+'-named-self',route,f'{route}(self:{receiver})','self positional receiver')
    for version in ['version(1,2,3)','version(1)','version()']:
        for index in [0,1,2,3,9,-1,-2,-3,-4,9223372036854775807,-9223372036854775808]:
            for form in ['static','bound']:
                index_expr='(-9223372036854775807 - 1)' if index==-9223372036854775808 else str(index)
                call=f'version.at({version},{index_expr})' if form=='static' else f'{version}.at({index_expr})'
                add(f'version-{form}-{version}-{index}','version.at',f'{{ let x = {call}; repr((type(x),x)) }}','index bounds,padding,explicit length,empty')
    for args in ['', 'version(1)', 'version(1),0,1', 'version(1),index:0', 'version(1),0,other:1', 'version(1),"0"', 'version(1),0.0', 'version(1),true', '1,0']:
        add('version-invalid-'+args,'version.at',f'version.at({args})','arity,named,type diagnostics')
    for val in ['version()','version(1)','version(1,2,3)','version(1,2,3,4)']:
        add('version-control-'+val,'version.at',f'{{ let v = {val}; repr((v.major,v.minor,v.patch,repr(v),str(v))) }}','preserve major minor patch repr display')
        add('version-display-'+val,'version.at',f'(repr({val}),str({val}),array({val}))','repr,display,explicit components independently')
        for field in ['major','minor','patch']:
            add('version-field-'+val+'-'+field,'version.at',f'{val}.{field}','individual named component preservation')
    add('version-method-value','version.at','{ let f = version(1,2,3).at; repr((type(f),repr(f),f(-1))) }','bound extraction')
    assert len({x['id'] for x in out})==len(out)
    return out

def main():
    stage=sys.argv[1]
    frozen=provenance()
    a0=json.loads((D/'p1339-a0.json').read_text())
    assert a0['baseline_matches_p1338']
    for binary in a0['binaries'].values():
        assert sha(binary['path'])==binary['sha256']
    corpus=cases()
    save(stage+'-manifest',dict(**frozen, stage=stage, regime='executado sem atestação de isolamento', executor='/root/p1339_measure_r1', context='fresh delegated task; no crystalline source or L0 read', reads=INPUTS+['old focal runner','skill and references','pinned binaries'], writes=['p1339-full-*'], profiles=PROFILES, cases=corpus, budget='at most two corrective runner slices per cause; Unknown mandatory blocks seal; no final PASS', binaries=a0['binaries']))
    env=dict(os.environ,LC_ALL='C',NO_COLOR='1')
    for key in list(env):
        if key.startswith('TYPST_'):
            del env[key]
    focal=stage.startswith('focal')
    if focal:
        corpus=[c for c in corpus if c['route'].startswith('angle.') or c['id'].startswith(('where-expanded','where-with-element','where-show-morphology')) or '--9223372036854775808' in c['id']]
    names=[('vanilla','vanilla')] if stage=='explore' or focal else [('vanilla','vanilla'),('crystalline-before','candidate')]
    for name,key in names:
        rows=[]
        orders=['normal'] if stage=='explore' or focal else ['normal','reverse']
        start=now()
        for order in orders:
            for n,case in enumerate(corpus if order=='normal' else list(reversed(corpus))):
                for profile,flags in (list(PROFILES.items())[:1] if stage=='explore' or focal else PROFILES.items()):
                    argv=[a0['binaries'][key]['path'],'eval','--format','json',*flags,case['expression']]
                    tick=time.monotonic(); begin=now()
                    try:
                        p=subprocess.run(argv,cwd=ROOT,env=env,capture_output=True,timeout=15)
                        channels=dict(exit=p.returncode,stdout=p.stdout.decode(errors='replace'),stderr=p.stderr.decode(errors='replace'),execution='Observed')
                    except subprocess.TimeoutExpired as e:
                        channels=dict(exit=None,stdout=(e.stdout or b'').decode(errors='replace'),stderr=(e.stderr or b'').decode(errors='replace'),execution='Unknown')
                    rows.append(dict(id=case['id'],order=order,profile=profile,argv=argv,cwd=str(ROOT),start=begin,end=now(),seconds=time.monotonic()-tick,**channels))
                if n%100==0:
                    print(stage,name,order,n,len(corpus),flush=True)
        save(stage+'-'+name+'-runs',dict(start=start,end=now(),provenance=frozen,end_provenance=provenance(),manifest_sha256=sha(D/('p1339-full-'+stage+'-manifest.json')),binary=a0['binaries'][key],environment={'LC_ALL':'C','NO_COLOR':'1','TYPST_*':'removed'},rows=rows))

if __name__=='__main__':
    main()
