"""Count glibc allocation requests with a diagnostic LD_PRELOAD interposer."""
import datetime,gzip,hashlib,json,os,statistics,subprocess,tempfile
from pathlib import Path
D=Path(__file__).resolve().parent;R=D.parents[2];F=D/'fixtures';FONT=R/'00_nucleo/diagnosticos/performance-baseline/fonts';SO=D/'alloc_probe.so';CPU=min(os.sched_getaffinity(0))
B={'tekt':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'}
ENV=os.environ.copy()
for k in list(ENV):
 if k.startswith(('TYPST_','CRYSTALLINE_','CARGO_PROFILE_')):ENV.pop(k)
ENV.update(RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',NO_COLOR='1',TZ='UTC',LC_ALL='C',LD_PRELOAD=str(SO))
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
status=subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R)
stat=subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R)
fixtures=[F/f'{v}-{n}.typ' for n in (100,400,1600) for v in ('local-set','no-fill','alias','inline-set')]
state={'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'working_tree':'uncommitted','git_status':status.decode(),'git_diff_stat':stat.decode(),'binaries':{e:sha(b) for e,b in B.items()},'interposer_sha256':sha(SO),'interposer_source_sha256':sha(D/'alloc_probe.c'),'driver_sha256':sha(D/'alloc.py'),'compiler':subprocess.check_output(['gcc','--version'],text=True).splitlines()[0],'build_command':['gcc','-shared','-fPIC','-O2','-std=c11','-Wall','-Wextra','-o','alloc_probe.so','alloc_probe.c'],'fixtures':{p.name:{'text':p.read_text(),'sha256':sha(p)} for p in fixtures},'cpu':CPU,'samples':[],'scope':'glibc malloc/calloc/realloc/aligned requests; requested_bytes sums requested sizes, including realloc; no live-heap or mmap accounting'}
with tempfile.TemporaryDirectory(prefix='table-local-alloc-') as d:
 out=Path(d)
 for fixture in fixtures:
  for e,b in B.items():
   for rep in range(3):
    pdf=out/(fixture.stem+'-'+e+str(rep)+'.pdf');profile=out/(fixture.stem+'-'+e+str(rep)+'.json')
    env=ENV.copy();env['ALLOC_PROFILE_OUT']=str(profile)
    cmd=[str(b),'compile',str(fixture),str(pdf),'--root',str(F),'--pdf-tagged=false','--pretty=false','--font-path',str(FONT)]+(['--jobs','1'] if e=='vanilla' else [])
    p=subprocess.run(cmd,cwd=R,env=env,capture_output=True,text=True,timeout=120,preexec_fn=lambda:os.sched_setaffinity(0,{CPU}));assert p.returncode==0,(cmd,p.stderr)
    counts=json.loads(profile.read_text());counts['total_calls']=sum(counts[k] for k in ('malloc_calls','calloc_calls','realloc_calls','memalign_calls'))
    state['samples'].append({'fixture':fixture.stem,'engine':e,'rep':rep,'utc':utc(),'command':cmd,'profile':counts,'stderr':p.stderr,'pdf_sha256':sha(pdf)})
  print(fixture.stem,'done',flush=True)
state['summary']={}
for f in fixtures:
 state['summary'][f.stem]={}
 for e in B:
  group=[s['profile'] for s in state['samples'] if s['fixture']==f.stem and s['engine']==e]
  state['summary'][f.stem][e]={k:statistics.median(x[k] for x in group) for k in group[0]}
assert state['binaries']=={e:sha(b) for e,b in B.items()}
assert state['interposer_sha256']==sha(SO)
assert state['driver_sha256']==sha(D/'alloc.py')
state['utc_end']=utc()
with gzip.open(D/'allocations.json.gz','wt') as p:json.dump(state,p,ensure_ascii=False,indent=2)
for name,c in sorted(state['summary'].items()):print(name,{e:{k:v for k,v in x.items() if k in ('total_calls','requested_bytes')} for e,x in c.items()},flush=True)
