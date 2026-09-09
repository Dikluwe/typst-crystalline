"""Fresh sampled export/CLI audit with effective profiles and raw channels."""
import base64,hashlib,importlib.util,json,subprocess,sys,tempfile,time
from pathlib import Path
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
def load(n,path):
    spec=importlib.util.spec_from_file_location(n,path);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
r=load('r',D/'p1335-record.py');s=load('s',D/'p1335-sentinels-r2.py');m=s.m
sys.path.insert(0,str(r.ROOT/'lab/parity/matrix'))
t=load('matrix',r.ROOT/'lab/parity/matrix/runner.py')
PROFILES={p:dict(features=f) for p,f in m.PROFILES.items()}
def flags(args,features):
    result=[];i=0
    while i<len(args):
        if args[i]=='--features':i+=2;continue
        if args[i].startswith('--features='):i+=1;continue
        result.append(args[i]);i+=1
    if features:result+=['--features',','.join(features)]
    assert t.feature_flags(result)==set(features);return result
def freeze():
    before=r.state();r.verify(before);tmp=Path(tempfile.mkdtemp(prefix='p1335-transversal-',dir='/tmp'))
    internal=D/'p1335-fixtures'/'matrix';assert not internal.exists();internal.mkdir(parents=True)
    for dest in [tmp/'fixtures',internal/'fixtures']:subprocess.run(['cp','-a',str(r.ROOT/'lab/parity/matrix/fixtures'),str(dest)],check=True)
    html=D/'p1335-fixtures'/'html';assert not html.exists();html.mkdir()
    for name in ['hello.typ','ab-error.typ']:subprocess.run(['cp',str(D/'p1323-fixtures'/name),str(html/name)],check=True)
    corpus=json.loads((D/'p1322-transversal-cases.json').read_text())['matrix'];assert not t.validate_manifest(corpus)
    warning=json.loads((D/'p1323-ab-frozen-suite-r2.json').read_text())
    extras=['csv(bytes("\\\"a\\nb\\\",c\\n1"))','csv(bytes("a,b\\r\\n1"))','csv(bytes("á,😀\\r\\n1"))','csv(bytes((97,44,98,10,49,10,255)))','{let f=json; f(42)}','{let f=json.with(42); f()}','{let a=arguments(42); json(..a)}','{let f(..a)=json(..a); f(42)}',
        '{let abs(x)=[ok]; $abs(1)$}','{let sqrt(x)=[ok]; $sqrt(1)$}','{let f=calc.abs; $f(-1)$}','{import calc: abs; $abs(-1)$}']
    cases=dict(at=r.now(),matrix=corpus,profiles=PROFILES,temp=str(tmp),internal=str(internal),html=str(html),warning_cases=warning['cases'],warning_base64=warning['warning_base64'],extra=[dict(id='boundary-'+str(i),expression=e) for i,e in enumerate(extras)])
    r.save('transversal-cases',cases)
    paths=[Path(__file__),D/'p1335-transversal-cases.json',D/'p1335-record.py',D/'p1335-matrix.py',D/'p1335-sentinels-r2.py',D/'p1322-transversal-cases.json',D/'p1323-ab-frozen-suite-r2.json',r.ROOT/'lab/parity/matrix/runner.py',r.ROOT/'lab/parity/matrix/svg_morphology.py',r.ROOT/'lab/parity/matrix/manifest.yaml']
    paths += [p for folder in [tmp/'fixtures',internal/'fixtures',html] for p in folder.rglob('*') if p.is_file()]
    r.save('transversal-freeze',dict(at=r.now(),manifest_sha256=r.sha(D/'p1335-manifest.json'),inputs={str(p):r.sha(p) for p in paths},binaries=dict(vanilla=r.baseline()['vanilla'],crystalline=json.loads((D/'p1335-build.json').read_text())['candidate']),before=before,after=r.state(),profiles=PROFILES,
        policy='Selected export projections and raw channels separately; query feature rejection requires full known transcript; serialization flags unique to C are unilateral warning controls, not bilateral parity; all counts supplemental'))
def complete(o):
    if o.get('harness_error') or o['exit_code'] not in (0,1,2):return False
    if o['exit_code']==1:return bool(o['stderr'].strip())
    if o['exit_code']==2:
        return ('query' in o['command'] and '--features' in o['command'] and not o['stdout'] and o['stderr']=="error: unexpected argument '--features' found\n\n  tip: to pass '--features' as a value, use '-- --features'\n\nUsage: typst query --format <FORMAT> <INPUT> <SELECTOR>\n\nFor more information, try '--help'.\n")
    return True
def pair(c,p,bins,directory,root,reverse):
    t.HERE=root;gate=t.profile_gate(c,p,PROFILES)
    if gate is not None:return dict(id=c['id'],eixo=c['eixo'],estado=gate,classe='UNKNOWN' if gate=='UNKNOWN' else None,nota=c['nota'])
    source=root/c['fonte_typ'] if c['fonte_typ'] else None;obs={}
    for side,role,decl in reversed([('vanilla','oracle','oraculo'),('crystalline','crystalline','cristalino')]) if reverse else [('vanilla','oracle','oraculo'),('crystalline','crystalline','cristalino')]:
        config=c[decl];at=r.now();tick=time.monotonic();o=t.invoke(Path(bins[side]['path']),flags(config['args'],PROFILES[p]['features']),source,directory/role,config.get('env'))
        o.update(started_at=at,duration_seconds=time.monotonic()-tick,cwd=str(r.ROOT),source_sha256=r.sha(source) if source else None,binary_sha256=bins[side]['sha256'],features=PROFILES[p]['features'],env=config.get('env',{}));o['complete']=complete(o);obs[role]=o
    v,a=obs['oracle'],obs['crystalline'];state,cls=t.classify_for_profile(c,p,PROFILES,c['comparison'],v,a)
    try:values=t.comparison_observables(c['comparison'],v,a,directory)
    except Exception as e:state,cls,values='UNKNOWN','UNKNOWN',dict(harness_error=str(e))
    if not all(o['complete'] for o in obs.values()):state,cls='UNKNOWN','UNKNOWN'
    delta=[k for k in ['exit_code','stdout','stderr'] if v[k]!=a[k]]
    return dict(id=c['id'],eixo=c['eixo'],estado=state,classe=cls,observed=values,nota=c['nota'],comparison=c['comparison'],channel_state='RAW_DIFFERENCE' if delta else 'RAW_EQUAL',differing_channels=delta,**obs)
def warning_run(c,phase,f,data,outroot):
    row=dict(id=c['id'],phase=phase,universe='warning-contract-supplement',case=c,observations={});fixed=base64.b64decode(data['warning_base64']).decode()
    roles=['crystalline'] if '--html-serialization' in c['argv'] else ['vanilla','crystalline']
    row['bilateral']=len(roles)==2
    if phase=='reverse':roles.reverse()
    for side in roles:
        output=tmp/'warning'/f"{phase}-{side}-{c['id']}.{c['format']}";output.parent.mkdir(exist_ok=True)
        args=[a.replace(str(D/'p1323-fixtures'),data['html']).replace('{output}',str(output)) for a in c['argv']];argv=[f['binaries'][side]['path'],*args];at=r.now();tick=time.monotonic()
        try:
            o=subprocess.run(argv,cwd=r.ROOT,input=base64.b64decode(c['stdin_base64']),capture_output=True,timeout=30);code,out,err=o.returncode,o.stdout.decode(),o.stderr.decode();known=code in (0,1)
        except (OSError,subprocess.TimeoutExpired) as e:code,out,err,known=None,'',str(e),False
        row['observations'][side]=dict(command=argv,cwd=str(r.ROOT),started_at=at,duration_seconds=time.monotonic()-tick,binary_sha256=f['binaries'][side]['sha256'],stdin_base64=c['stdin_base64'],exit_code=code,stdout=out,stderr=err,complete=known,artifact=dict(path=str(output),sha256=r.sha(output)) if output.exists() else None,
            warning_contract=(err.startswith(fixed) and (not c['error_expected'] or err[len(fixed):].startswith('error:'))) if c['warning_expected'] else fixed.splitlines()[0] not in err)
    return row
def run(focal=False):
    f=json.loads((D/'p1335-transversal-freeze-r2.json').read_text());m.verify(f);data=json.loads((D/'p1335-transversal-cases.json').read_text());before=r.state();at=r.now();tmp=Path(data['temp']);rows=[];locations=[];extra=[];warnings=[]
    outroot=tmp/('r2-focal' if focal else 'r2-full');outroot.mkdir(exist_ok=False)
    for phase in (['focal'] if focal else ['normal','repeat','reverse']):
        reverse=phase=='reverse';cases=data['matrix']['cases'];cases=list(reversed(cases)) if reverse else cases
        if focal:cases=[c for c in cases if c['id'] in ['P1137-I-001','P1138-X-004','P1138-L-001']]
        for p in PROFILES:
            result=[pair(c,p,f['binaries'],outroot/phase/p/c['id'],tmp,reverse) for c in cases]
            rows.append(dict(phase=phase,profile=p,results=result,counts=t.closed_counts(result)))
            if not focal:
                for c in cases:
                    if c['id'] in ['P1138-S-001','P1138-S-002','P1138-S-003']:
                        o=pair(c,p,f['binaries'],outroot/'location'/phase/p/c['id'],Path(data['internal']),reverse);o.update(profile=p,phase=phase);locations.append(o)
                for c in (list(reversed(data['extra'])) if reverse else data['extra']):
                    x=dict(id=c['id'],expression=c['expression'],profile=p,phase=phase,family='boundary',universe='supplement')
                    for side in (['crystalline','vanilla'] if reverse else ['vanilla','crystalline']):x[side]=m.observe(f['binaries'][side],c,p,side,phase)
                    x['runtime_class']=m.classify(x['vanilla'],x['crystalline']);extra.append(x)
            print(phase,p,t.closed_counts(result),flush=True)
        selected=data['warning_cases'] if not focal else [c for c in data['warning_cases'] if c['id'] in ['html-compile','html-error-order','gate-default']]
        for c in (list(reversed(selected)) if reverse else selected):warnings.append(warning_run(c,phase,f,data,outroot))
    m.verify(f);r.save('transversal-focal-r2' if focal else 'transversal-r2',dict(at=at,end=r.now(),manifest_sha256=r.sha(D/'p1335-manifest.json'),freeze_sha256=r.sha(D/'p1335-transversal-freeze-r2.json'),before=before,after=r.state(),binaries=f['binaries'],matrix=rows,location=locations,extra=extra,warnings=warnings,temp=str(tmp),artifacts={str(p):r.sha(p) for p in outroot.rglob('*') if p.is_file()},output_root=str(outroot)))
if __name__=='__main__':
    if sys.argv[1]=='freeze':freeze()
    else:run(sys.argv[1]=='focal')
