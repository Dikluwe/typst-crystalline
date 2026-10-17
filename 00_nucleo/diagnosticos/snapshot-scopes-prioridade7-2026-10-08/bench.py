import datetime,gzip,hashlib,importlib.util,json,os,statistics,subprocess,time
from pathlib import Path
D=Path(__file__).resolve().parent;R=D.parents[2];O=Path('/tmp/p7-snapshot-cli')
spec=importlib.util.spec_from_file_location('monitor',D.parent/'table-local-c2r1-2026-10-07/performance/monitor-helper.py');monitor=importlib.util.module_from_spec(spec);spec.loader.exec_module(monitor)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();git=lambda *a:subprocess.check_output(['git',*a],cwd=R,text=True)
engines={'previous':Path('/tmp/p7-snapshot-before-17e72149'),'current':R/'target/release/typst'}
env=os.environ.copy()
for k in list(env):
 if k.startswith(('TYPST_','CRYSTALLINE_','CARGO_PROFILE_')):env.pop(k)
env.update(RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',NO_COLOR='1')
O.mkdir(exist_ok=True)
cpu=min(os.sched_getaffinity(0));samples=[]
paths=set()
for base in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts']:
 paths.update(p for p in (R/base).rglob('*') if p.is_file() and p.suffix in ['.rs','.toml','.md','.ttf','.otf','.ttc'])
for path in subprocess.check_output(['fc-list','--format','%{file}\n'],text=True).splitlines():
 p=Path(path)
 if p.is_file():paths.add(p)
paths.update(p for p in Path('/etc/fonts').rglob('*') if p.is_file())
paths.update(p for p in (R/'.cargo').rglob('*') if p.is_file())
paths.update(R/n for n in ['Cargo.toml','Cargo.lock'] if (R/n).is_file())
for base in ['assets','lab/typst-original/assets']:
 paths.update(p for p in (R/base).rglob('*') if p.is_file() and p.suffix in ['.ttf','.otf','.ttc'])
identity={str(p):sha(p) for p in sorted(paths)}
rec={'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD'),'status':git('status','--short'),'diff_stat':git('diff','HEAD','--stat'),'files':identity,'cpu':cpu,'env':{k:env[k] for k in ['RAYON_NUM_THREADS','SOURCE_DATE_EPOCH','NO_COLOR']},'binaries':{k:sha(v) for k,v in engines.items()},'fixtures':json.loads((D/'fixtures.json').read_text()),'samples':samples}
assert rec['binaries']['current'] != rec['binaries']['previous']
for round in range(3):
 for name in ['hello','control1000','closures1000','context300','plain300','one_closure_many_calls']:
  for pair in range(-1,6):
   order=list(engines)
   if (round+pair)%2:order.reverse()
   for engine in order:
    blocked=monitor.blockers(monitor.processes(),None)
    assert not blocked,blocked
    pdf=O/(name+'-'+engine+'.pdf');timing=O/(name+'-'+engine+'.timings.json');usage=O/'usage.txt'
    cmd=['/usr/bin/time','-f','%M %U %S','-o',str(usage),str(engines[engine]),'compile',str(D/'fixtures'/(name+'.typ')),str(pdf),'--pdf-tagged=false','--pretty=false','--timings-json',str(timing)]
    start=time.perf_counter_ns();p=subprocess.run(cmd,cwd=R,env=env,capture_output=True,text=True,timeout=60,preexec_fn=lambda:os.sched_setaffinity(0,{cpu}));wall=(time.perf_counter_ns()-start)/1e6
    assert p.returncode==0,p.stderr
    rss,u,s=usage.read_text().split()
    if pair>=0:samples.append({'round':round,'pair':pair,'fixture':name,'engine':engine,'wall_ms':wall,'max_rss_kib':int(rss),'cpu_coarse_ms':(float(u)+float(s))*1000,'timings':json.loads(timing.read_text()),'command':cmd,'utc':datetime.datetime.now(datetime.timezone.utc).isoformat()})
  print(round,name,'done',flush=True)
assert all(sha(Path(p))==h for p,h in identity.items())
rec['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(D/'bench.json.gz','wt') as f:json.dump(rec,f,ensure_ascii=False,indent=2)
print('saved',len(samples),flush=True)
