"""Check phase instrumentation against ordinary compilation in one paired window."""
import resource
resource.setrlimit(resource.RLIMIT_STACK,(64*1024*1024,resource.getrlimit(resource.RLIMIT_STACK)[1]))
from pathlib import Path
import subprocess,json,gzip,os,time,datetime,hashlib,statistics,tempfile
D=Path(__file__).resolve().parent;R=D.parents[2];B=R/'target/release/typst';F=D/'fixtures';FONT=R/'00_nucleo/diagnosticos/performance-baseline/fonts';CPU=min(os.sched_getaffinity(0))
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
env=os.environ.copy()
for k in list(env):
 if k.startswith(('TYPST_','CRYSTALLINE_','CARGO_PROFILE_')):env.pop(k)
env.update(RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',NO_COLOR='1',TZ='UTC',LC_ALL='C')
fixtures=[F/(n+'.typ') for n in ['local-set-1600','no-fill-1600']]
state={'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'working_tree':'uncommitted','status':subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R,text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),'binary_sha256':sha(B),'script_sha256':sha(__file__),'fixtures':{f.name:{'text':f.read_text(),'sha256':sha(f)} for f in fixtures},'cpu':CPU,'samples':[],'scope':'timing instrumentation and plain compilation paired, five reps alternating orders'}
with tempfile.TemporaryDirectory(prefix='check-phases-') as tmp:
 out=Path(tmp)
 for rep in range(-1,5):
  for fixture in (fixtures if rep%2 else list(reversed(fixtures))):
   for mode in (['timed','plain'] if rep%2 else ['plain','timed']):
    pdf=out/'a.pdf';j=out/'timings.json';cmd=[str(B),'compile',str(fixture),str(pdf),'--root',str(F),'--pdf-tagged=false','--pretty=false','--font-path',str(FONT)]+(['--timings-json',str(j)] if mode=='timed' else [])
    t=time.perf_counter_ns();p=subprocess.run(cmd,cwd=R,env=env,capture_output=True,text=True,timeout=120,preexec_fn=lambda:os.sched_setaffinity(0,{CPU}));wall=(time.perf_counter_ns()-t)/1e6;assert p.returncode==0,p.stderr
    if rep>=0:state['samples'].append({'rep':rep,'fixture':fixture.stem,'mode':mode,'utc':utc(),'command':cmd,'wall_ms':wall,'timings':json.loads(j.read_text()) if mode=='timed' else None,'pdf_sha256':sha(pdf),'stderr':p.stderr})
summary={}
for f in fixtures:
 name=f.stem;rows=[s for s in state['samples'] if s['fixture']==name];timed=[r for r in rows if r['mode']=='timed'];plain=[r for r in rows if r['mode']=='plain']
 summary[name]={'timed_wall_median_ms':statistics.median(r['wall_ms'] for r in timed),'plain_wall_median_ms':statistics.median(r['wall_ms'] for r in plain),'phases':{k:statistics.median(r['timings'][k] for r in timed) for k in timed[0]['timings']},'paired_overhead':statistics.median(next(r['wall_ms'] for r in timed if r['rep']==rep)/next(r['wall_ms'] for r in plain if r['rep']==rep)-1 for rep in range(5))}
state['summary']=summary;state['utc_end']=utc();assert sha(B)==state['binary_sha256']
with gzip.open(D/'checked-phases.json.gz','wt') as f:json.dump(state,f,ensure_ascii=False,indent=2)
print(json.dumps(summary,indent=2))
