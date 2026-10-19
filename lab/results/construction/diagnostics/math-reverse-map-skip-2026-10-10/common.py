from pathlib import Path
import datetime,hashlib,os,resource,subprocess,gzip,json
D=Path(__file__).resolve().parent;R=D.parents[2];FONT=R/'00_nucleo/diagnosticos/performance-baseline/fonts'
B={'before':Path('/tmp/tekt-math-map/old/typst'),'after':Path('/tmp/tekt-math-map/new/typst'),'orig':R/'lab/typst-original/target/release/typst'};CPU=min(os.sched_getaffinity(0))
C=R/'00_nucleo/diagnosticos/cmap-payload-reuse-2026-10-10/cost-fixtures';T=R/'00_nucleo/diagnosticos/font-primary-reuse-2026-10-10/fixtures'
F=[C/(n+'.typ')for n in ['plain','default-scopes']]+[T/(n+'.typ')for n in ['local-set-400','local-set-1600','no-fill-1600','inline-set-1600','families-1600','variants-1600']]
resource.setrlimit(resource.RLIMIT_STACK,(64*1024*1024,resource.getrlimit(resource.RLIMIT_STACK)[1]))
ENV=os.environ.copy()
for k in list(ENV):
 if k.startswith(('TYPST_','CRYSTALLINE_','CARGO_PROFILE_'))or k in ['LD_PRELOAD','CPU_PROFILE_OUT','CPU_PROFILE_INTERVAL_US']:ENV.pop(k)
ENV.update(RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',NO_COLOR='1',TZ='UTC',LC_ALL='C')
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def command(e,f,pdf):return [str(B[e]),'compile',str(f),str(pdf),'--root',str(f.parent),'--pdf-tagged=false','--pretty=false','--font-path',str(FONT)]+(['--jobs','1']if e=='orig'else [])
def state(script):
 return {'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'working_tree':'uncommitted','git_status':subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R,text=True),'git_diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),'binaries':{e:sha(p)for e,p in B.items()},'cpu':CPU,'stack_limit':resource.getrlimit(resource.RLIMIT_STACK)[0],'environment':{k:ENV[k]for k in ['RAYON_NUM_THREADS','SOURCE_DATE_EPOCH','NO_COLOR','TZ','LC_ALL']},'fixtures':{str(p.relative_to(R)):{'text':p.read_text(),'sha256':sha(p)}for p in F},'fonts':{str(p):sha(p)for p in FONT.rglob('*')if p.is_file()},'scripts':{p.name:{'text':p.read_text(),'sha256':sha(p)}for p in [D/'common.py',Path(script)]}}
def verify(x):
 assert x['binaries']=={e:sha(p)for e,p in B.items()}
 assert all(sha(R/p)==v['sha256']for p,v in x['fixtures'].items());assert all(sha(p)==h for p,h in x['fonts'].items())
 for p,v in x['scripts'].items():assert sha(D/p)==v['sha256']
def save(name,x):
 verify(x);x['utc_end']=utc()
 with gzip.open(D/name,'wt')as f:json.dump(x,f,ensure_ascii=False)
def run(cmd,env=ENV,timeout=120):
 p=subprocess.run(cmd,cwd=R,env=env,capture_output=True,text=True,timeout=timeout,preexec_fn=lambda:os.sched_setaffinity(0,{CPU}));assert p.returncode==0,(cmd,p.stderr);return p

def parse_profile(text):
 lines=text.splitlines();header=dict(t.split('=')for t in lines[0].split()[1:]);modules={};samples=[]
 for l in lines[1:]:
  if l.startswith('M\t'):
   _,idx,base,path=l.split('\t',3);modules[int(idx)]={'base':int(base,16),'path':path}
  elif l.startswith('S '):
   t=l.split();samples.append({'tid':int(t[1]),'depth':int(t[2]),'addresses':[(int(a.split(':')[0]),int(a.split(':')[1],16))for a in t[3:]]})
 assert len(samples)==int(header['samples']);assert int(header['reserved'])==int(header['delivered']),'overflow'
 return {'header':{k:int(v)for k,v in header.items()},'modules':modules,'samples':samples}
