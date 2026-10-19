import datetime,gzip,hashlib,importlib.util,itertools,json,os,statistics,subprocess,time
from pathlib import Path
D=Path(__file__).resolve().parent;R=D.parents[2];O=Path('/tmp/context-snapshots-memoria4-2026-10-08');O.mkdir(exist_ok=True)
spec=importlib.util.spec_from_file_location('monitor',D.parent/'performance-c2-2026-10-07/monitor-helper.py');monitor=importlib.util.module_from_spec(spec);spec.loader.exec_module(monitor)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
B={'baseline':Path('/tmp/typst-before-context-snapshots-memory4'),'current':R/'target/release/typst'}
expected={'baseline':'4686d15bd91c1b9cd9ca4605fdec16b8c7babd59f4e125276ebcaba87969379c','current':sha(B['current'])}
assert all(sha(p)==expected[k] for k,p in B.items())
env=os.environ.copy()
for k in list(env):
 if k.startswith(('TYPST_','CRYSTALLINE_','CARGO_PROFILE_')):env.pop(k)
env.update(RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',NO_COLOR='1',TZ='UTC',LC_ALL='C');cpu=min(os.sched_getaffinity(0));fontdir=R/'00_nucleo/diagnosticos/performance-baseline/fonts'
cases={p.stem:p for p in (D.parent/'conjunto-prioridades-vanilla-2026-10-08/fixtures').glob('*.typ')}
for name in ['context10','context100','context300','plain300']:cases[name]=D.parent/'snapshot-scopes-prioridade7-2026-10-08/fixtures'/(name+'.typ')
paths=set()
for base in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts','.cargo']:
 paths.update(p for p in (R/base).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml'])
paths.update(p for p in fontdir.glob('*') if p.is_file());paths.update(p for p in (D.parent/'conjunto-prioridades-vanilla-2026-10-08/fixtures').glob('*') if p.is_file());paths.update(R/n for n in ['Cargo.toml','Cargo.lock']);paths.update(p for p in Path('/etc/fonts').rglob('*') if p.is_file())
for name in subprocess.check_output(['fc-list','--format','%{file}\n'],text=True).splitlines():
 p=Path(name)
 if p.is_file():paths.add(p)
for base in ['assets','lab/typst-original/assets']:paths.update(p for p in (R/base).rglob('*') if p.is_file() and p.suffix in ['.ttf','.otf','.ttc'])
paths.update(cases.values())
identity={str(p):sha(p) for p in sorted(paths)};samples=[]
r={'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'state_sha256':sha(D/'candidate.json.gz'),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'binaries':expected,'cpu':cpu,'environment':{k:env[k] for k in ['RAYON_NUM_THREADS','SOURCE_DATE_EPOCH','NO_COLOR','TZ','LC_ALL']},'input_sha256':identity,'samples':samples}
orders=list(itertools.permutations(B))
for round in range(3):
 for name,fixture in sorted(cases.items()):
  for pair in range(-1,4):
   order=orders[(pair+round)%2]
   for engine in order:
    hits=monitor.blockers(monitor.processes(),None);assert not hits,hits
    pdf=O/(fixture.stem+'-'+engine+'.pdf');usage=O/'usage.txt';cmd=['/usr/bin/time','-f','%M %U %S','-o',str(usage),str(B[engine]),'compile',str(fixture),str(pdf),'--root',str(fixture.parent),'--pdf-tagged=false','--pretty=false','--font-path',str(fontdir)]
    if engine=='vanilla':cmd+=['--jobs','1']
    start=time.perf_counter_ns();p=subprocess.run(cmd,cwd=R,env=env,text=True,capture_output=True,timeout=60,preexec_fn=lambda:os.sched_setaffinity(0,{cpu}));wall=(time.perf_counter_ns()-start)/1e6;assert p.returncode==0,(engine,fixture.name,p.stderr);rss,u,sys=usage.read_text().split()
    if pair>=0:samples.append({'round':round,'pair':pair,'fixture':fixture.stem,'engine':engine,'wall_ms':wall,'max_rss_kib':int(rss),'cpu_coarse_ms':(float(u)+float(sys))*1000,'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'command':cmd,'stderr':p.stderr})
  print(round,fixture.stem,'done',flush=True)
assert all(sha(Path(p))==h for p,h in identity.items());assert all(sha(p)==expected[k] for k,p in B.items())
r['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(D/'bench.json.gz','wt') as f:json.dump(r,f,ensure_ascii=False,indent=2)
print('saved',len(samples),flush=True)
