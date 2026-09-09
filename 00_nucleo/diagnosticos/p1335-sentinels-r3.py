"""Fresh bilateral functional witnesses; explicit dedup and provenance, no oracle edits."""
import base64,hashlib,importlib.util,json,subprocess,sys,tempfile,time
from collections import Counter
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
def load(n):
    spec=importlib.util.spec_from_file_location(n,D/(n+'.py'));m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
r=load('p1335-record');m=load('p1335-matrix');legacy=load('p1307-r4-oracle')
FIX=D/'p1335-fixtures'/'sentinels-r1'
def create(path,body):
    assert not path.exists(),path
    patch='*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+line+'\n' for line in body.rstrip('\n').split('\n'))+'*** End Patch\n'
    subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True,cwd=r.ROOT)
def freeze():
    before=r.state();r.verify(before);tmp=Path(tempfile.mkdtemp(prefix='p1335-functional-',dir='/tmp'))
    assert not FIX.exists();FIX.parent.mkdir(exist_ok=True)
    subprocess.run(['cp','-a',str(D/'p1322-fixtures'),str(FIX)],check=True)
    focal=FIX.parent/'abs-origins-r1.typ';subprocess.run(['cp',str(D/'p1334-ab-fixtures/origins.typ'),str(focal)],check=True)
    historic=json.loads((D/'p1322-sentinels-cases.json').read_text());cases=[];aliases=[];index={}
    def add(c):
        c=dict(c);c.setdefault('cwd',str(r.ROOT));c.setdefault('route','eval');c.setdefault('format','json');c.setdefault('profiles',list(m.PROFILES))
        c['source_sha256']=hashlib.sha256(c.get('document',c['expression']).encode()).hexdigest()
        key=(c['route'],c.get('document',c['expression']),c['cwd'],c['format'],tuple(c['profiles']))
        if key in index:
            aliases.append(dict(id=c['id'],canonical=index[key],origin=c.get('origin'),reason='same literal source/route/cwd/format/profiles'));return
        index[key]=c['id'];c['aliases']=[];cases.append(c)
    for old in historic['cases']:
        c=dict(old);c['origin']=dict(path=str(D/'p1322-sentinels-cases.json'),id=c['id'])
        c['expression']=c['expression'].replace('00_nucleo/diagnosticos/p1322-fixtures/',str(FIX.relative_to(r.ROOT))+'/')
        c['cwd']=c['cwd'].replace(str(D/'p1322-fixtures'),str(FIX))
        if c.get('observations'):c['profiles']=list(c['observations'])
        if c['route']=='compile':
            c['fixture']=str(tmp/(hashlib.sha256(c['id'].encode()).hexdigest()[:16]+'.typ'));create(Path(c['fixture']),c['document'])
        add(c)
    for name in ['p1334-ab-cli-baseline.json','p1334-ab-cross-baseline.json']:
        seen=set()
        for row in json.loads((D/name).read_text())['cases']:
            if row['case'] in seen:continue
            seen.add(row['case']);expr=row['expression'].replace('00_nucleo/diagnosticos/p1334-ab-fixtures/origins.typ',str(focal.relative_to(r.ROOT)))
            add(dict(id='p1334.'+row['case'],expression=expr,format='repr',family='p1334-abs',origin=dict(path=str(D/name),id=row['case'])))
    required=json.loads((D/'p1335-classification-required-sentinels.json').read_text())
    for row in required['cases']:
        c=dict(row);local=FIX.parent/'recent-r1'/c['historical_step'];local.mkdir(parents=True,exist_ok=True)
        for name,fixture in c.get('fixtures',{}).items():
            body=fixture['text'];assert hashlib.sha256(body.encode()).hexdigest()==fixture['sha256']
            path=local/name
            if not path.exists():create(path,body)
            else:assert path.read_text().rstrip('\n')==body.rstrip('\n')
        c['cwd']=str(local) if c.get('fixtures') else str(r.ROOT);c['family']='recent-fields-imports';add(c)
    for name in ['captured-only','closure-only']:
        c=dict(id='closure.'+name,expression='#include "'+name+'.typ"',document='#include "'+name+'.typ"',fixture=str(FIX/(name+'.typ')),route='compile',family='closure-origin',cwd=str(FIX))
        add(c)
    for a in aliases:next(c for c in cases if c['id']==a['canonical'])['aliases'].append(a['id'])
    assert len({c['id'] for c in cases})==len(cases)
    r.save('sentinels-cases',dict(at=r.now(),cases=cases,aliases=aliases,profiles=m.PROFILES,fixture_root=str(FIX),temp=str(tmp),historical_input=dict(path=str(D/'p1322-sentinels-cases.json'),sha256=r.sha(D/'p1322-sentinels-cases.json'))))
    names=['p1335-sentinels-cases-r2.json','p1335-sentinels-r1.py','p1335-record.py','p1335-matrix.py','p1309-matrix.py','p1309-record-r2.py','p1307-r4-oracle.py','p1322-sentinels-cases.json','p1334-ab-cli-baseline.json','p1334-ab-cross-baseline.json','p1335-classification-required-sentinels.json']
    inputs={str(D/n):r.sha(D/n) for n in names}
    inputs.update({str(p):r.sha(p) for folder in [FIX.parent,tmp] for p in folder.rglob('*') if p.is_file()})
    r.save('sentinels-freeze',dict(at=r.now(),manifest_sha256=r.sha(D/'p1335-manifest.json'),inputs=inputs,binaries=dict(vanilla=r.baseline()['vanilla'],crystalline=json.loads((D/'p1335-build.json').read_text())['candidate']),before=before,after=r.state(),profiles=m.PROFILES,policy='full bilateral raw channels; legacy projection is separately labelled, dedup aliases do not inflate cells; relocated sources cannot prove temporal raw-channel regression without location control'))
def observe(c,profile,side,binary,phase,tmp):
    argv=[binary['path'],'--color=never',c['route']]
    if c['route']=='eval':
        argv += [c['expression']]
        assert c['format'] in ('json','raw','repr')
        if c['format'] in ('json','raw'):argv += ['--format',c['format']]
    else:argv += [c['fixture'],str(tmp/(hashlib.sha256(c['id'].encode()).hexdigest()[:16]+f'-{profile}-{side}-{phase}.pdf')),'--format','pdf']
    argv+=m.old.features(profile);start=r.now();tick=time.monotonic();reason=None
    try:
        p=subprocess.run(argv,cwd=c['cwd'],capture_output=True,timeout=30);code,out,err=p.returncode,p.stdout,p.stderr
        if code not in (0,1):reason='CLI_EXECUTION_UNKNOWN'
    except (OSError,subprocess.TimeoutExpired) as e:code,out,err=None,b'',str(e).encode();reason='EXECUTION_UNKNOWN'
    if code==0 and c['route']=='eval' and c['format']=='json':
        try:json.loads(out)
        except (ValueError,UnicodeError):reason='UNPARSEABLE_JSON'
    if code==1 and not err.strip():reason='MISSING_DIAGNOSTIC'
    o=dict(id=c['id'],profile=profile,side=side,phase=phase,argv=argv,cwd=c['cwd'],started_at=start,duration_seconds=time.monotonic()-tick,exit_code=code,
        stdout=out.decode(errors='replace'),stderr=err.decode(errors='replace'),stdout_base64=base64.b64encode(out).decode(),stderr_base64=base64.b64encode(err).decode(),
        stdout_sha256=hashlib.sha256(out).hexdigest(),stderr_sha256=hashlib.sha256(err).hexdigest(),complete=reason is None,reason_code=reason,source_sha256=c['source_sha256'],features=m.PROFILES[profile],binary_path=binary['path'],binary_sha256=binary['sha256'])
    if c.get('legacy_projection'):o['observable']=legacy.envelope(code,o['stdout'],o['stderr'],c)
    return o
def run(phase,ids=None):
    f=json.loads((D/'p1335-sentinels-freeze-r3.json').read_text());m.verify(f);corpus=json.loads((D/'p1335-sentinels-cases-r2.json').read_text());before=r.state();start=r.now()
    cases=corpus['cases'];cases=[c for c in cases if c['id'] in ids] if ids else cases
    jobs=[(c,p) for c in cases for p in c['profiles']]
    if phase=='reverse':jobs.reverse()
    def pair(job):
        c,p=job;row=dict(id=c['id'],aliases=c['aliases'],expression=c['expression'],profile=p,family=c['family'],phase=phase,universe='supplement')
        for side in (['crystalline','vanilla'] if phase=='reverse' else ['vanilla','crystalline']):row[side]=observe(c,p,side,f['binaries'][side],phase,Path(corpus['temp']))
        row['runtime_class']=m.classify(row['vanilla'],row['crystalline'])
        row['alias_historical_preservation']={}
        for alias,definition in c.get('alias_policies',{}).items():
            if definition.get('legacy_projection'):
                actual=legacy.envelope(row['crystalline']['exit_code'],row['crystalline']['stdout'],row['crystalline']['stderr'],definition)
                row['alias_historical_preservation'][alias]=legacy.classify(definition['observations'][p]['future_expected'],actual)
        if c.get('legacy_projection'):
            row['historical_preservation']=legacy.classify(c['observations'][p]['future_expected'],row['crystalline']['observable'])
            row['language_projection']=legacy.classify(row['vanilla']['observable'],row['crystalline']['observable'])
        return row
    with ThreadPoolExecutor(max_workers=4) as pool:rows=list(pool.map(pair,jobs))
    m.verify(f);r.save('sentinels-'+phase+'-r3',dict(at=start,end=r.now(),manifest_sha256=r.sha(D/'p1335-manifest.json'),freeze_sha256=r.sha(D/'p1335-sentinels-freeze-r3.json'),before=before,after=r.state(),binaries=f['binaries'],rows=rows,counts=dict(Counter(x['runtime_class'] for x in rows))))
    print(Counter(x['runtime_class'] for x in rows))
if __name__=='__main__':
    if sys.argv[1]=='freeze':raise RuntimeError('R2 already frozen by separate revision script')
    else:run(sys.argv[1],sys.argv[2:] or None)
