"""Three Tekt internal timing samples per output-equivalent table control."""
import datetime,gzip,hashlib,json,os,statistics,subprocess,tempfile
from pathlib import Path
D=Path(__file__).resolve().parent;R=D.parents[2];F=D/'fixtures';B=R/'target/release/typst';FONT=R/'00_nucleo/diagnosticos/performance-baseline/fonts';CPU=min(os.sched_getaffinity(0))
ENV=os.environ.copy()
for k in list(ENV):
 if k.startswith(('TYPST_','CRYSTALLINE_','CARGO_PROFILE_')):ENV.pop(k)
ENV.update(RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',NO_COLOR='1',TZ='UTC',LC_ALL='C')
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
status=subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R)
stat=subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R)
fixtures=[F/f'{v}-{n}.typ' for n in (100,400,1600) for v in ('local-set','no-fill','alias','inline-set')]
state={'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'working_tree':'uncommitted','git_status':status.decode(),'git_diff_stat':stat.decode(),'binary_sha256':sha(B),'fixtures':{p.name:sha(p) for p in fixtures},'script_sha256':sha(D/'phases.py'),'cpu':CPU,'samples':[]}
with tempfile.TemporaryDirectory(prefix='table-local-phases-') as d:
 out=Path(d)
 for f in fixtures:
  for rep in range(3):
   pdf=out/'a.pdf';js=out/'timing.json'
   cmd=[str(B),'compile',str(f),str(pdf),'--root',str(F),'--pdf-tagged=false','--pretty=false','--font-path',str(FONT),'--timings-json',str(js)]
   p=subprocess.run(cmd,cwd=R,env=ENV,capture_output=True,text=True,timeout=120,preexec_fn=lambda:os.sched_setaffinity(0,{CPU}));assert p.returncode==0,(cmd,p.stderr)
   state['samples'].append({'fixture':f.stem,'rep':rep,'utc':utc(),'command':cmd,'timings':json.loads(js.read_text())})
state['summary']={}
for f in fixtures:
 g=[x['timings'] for x in state['samples'] if x['fixture']==f.stem]
 state['summary'][f.stem]={k:statistics.median(x[k] for x in g) for k in g[0]}
 print(f.stem,{k:round(v,2) for k,v in state['summary'][f.stem].items()},flush=True)
assert state['binary_sha256']==sha(B) and state['script_sha256']==sha(D/'phases.py')
state['utc_end']=utc()
with gzip.open(D/'phases.json.gz','wt') as p:json.dump(state,p,ensure_ascii=False,indent=2)
