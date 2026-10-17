import datetime,gzip,hashlib,json,os,resource,statistics,subprocess,sys,tempfile
from pathlib import Path
D=Path(__file__).resolve().parent; R=D.parents[2]; label=sys.argv[1]
B=Path('/tmp/tekt-page-fold/'+('before' if label=='red' else 'after'))
SO=R/'00_nucleo/diagnosticos/table-local-scaling-followup-2026-10-09/alloc_probe.so'
F=D/'fixtures'; F.mkdir(exist_ok=True)
if label=='red':
 body=(R/'00_nucleo/diagnosticos/watch-context-current-2026-10-10/fixtures/context300.typ').read_text()
 for n in [300,1600]: (F/f'context{n}.typ').write_text(body.replace('range(300)',f'range({n})'))
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest(); now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
resource.setrlimit(resource.RLIMIT_STACK,(64*1024*1024,resource.getrlimit(resource.RLIMIT_STACK)[1]))
env=os.environ.copy()
for k in list(env):
 if k.startswith(('TYPST_','CRYSTALLINE_','CARGO_PROFILE_')) or k=='LD_PRELOAD':env.pop(k)
env.update(RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',TZ='UTC',LC_ALL='C',NO_COLOR='1',LD_PRELOAD=str(SO))
x={'utc_start':now(),'binary_sha256':sha(B),'probe_sha256':sha(SO),'driver_sha256':sha(__file__),'before_receipt_sha256':sha(D/'before.json.gz'),'criterion':'at least 1000 fewer allocation calls in context300 than before','fixtures':{p.name:{'sha256':sha(p),'text':p.read_text()} for p in F.glob('*.typ')},'samples':[]};cpu=min(os.sched_getaffinity(0))
with tempfile.TemporaryDirectory() as tmp:
 for n in [300,1600]:
  for rep in range(3):
   profile=Path(tmp)/'alloc.json';env['ALLOC_PROFILE_OUT']=str(profile)
   cmd=[str(B),'compile',str(F/f'context{n}.typ'),tmp+'/out.pdf','--pdf-tagged=false','--pretty=false']
   p=subprocess.run(cmd,cwd=R,env=env,capture_output=True,text=True,check=True,preexec_fn=lambda:os.sched_setaffinity(0,{cpu}))
   v=json.loads(profile.read_text());calls=sum(v[k] for k in ['malloc_calls','calloc_calls','realloc_calls','memalign_calls'])
   x['samples'].append({'n':n,'rep':rep,'calls':calls,'profile':v,'command':cmd,'utc':now(),'stderr':p.stderr})
x['summary']={str(n):statistics.median(s['calls'] for s in x['samples'] if s['n']==n) for n in [300,1600]}
x['passed']=False
if label=='green':
 old=json.load(gzip.open(D/'alloc-red.json.gz','rt'));x['red_summary']=old['summary'];x['passed']=x['summary']['300']<=old['summary']['300']-1000
x['utc_end']=now()
with gzip.open(D/f'alloc-{label}.json.gz','wt') as f:json.dump(x,f)
print(x['summary'],'passed',x['passed']);sys.exit(0 if x['passed'] else 1)
