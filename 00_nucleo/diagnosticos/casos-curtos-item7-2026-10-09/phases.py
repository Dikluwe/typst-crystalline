"""Collect Tekt's internal phase times on the same scaled fixtures."""
import datetime,hashlib,json,os,statistics,subprocess,tempfile
from pathlib import Path
D=Path(__file__).resolve().parent;R=D.parents[2];F=D/'fixtures';B=R/'target/release/typst';FONT=R/'00_nucleo/diagnosticos/performance-baseline/fonts'
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
ENV=os.environ.copy()
for k in list(ENV):
 if k.startswith(('TYPST_','CRYSTALLINE_','CARGO_PROFILE_')):ENV.pop(k)
ENV.update(RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',NO_COLOR='1',TZ='UTC',LC_ALL='C')
status=subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R)
stat=subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R)
(D/'phases-git-status.txt').write_bytes(status)
(D/'phases-git-diff-stat.txt').write_bytes(stat)
state={'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),
       'working_tree':'uncommitted','git_status_sha256':hashlib.sha256(status).hexdigest(),
       'git_diff_stat_sha256':hashlib.sha256(stat).hexdigest(),
       'binary_sha256':sha(B),'fixtures':{f.name:sha(f) for f in F.glob('*.typ')},'samples':[]}
with tempfile.TemporaryDirectory(prefix='item7-phases-') as out:
 for fixture in sorted(F.glob('*.typ')):
  for rep in range(3):
   pdf=Path(out)/'a.pdf';timing=Path(out)/'timing.json'
   cmd=[str(B),'compile',str(fixture),str(pdf),'--root',str(F),'--pdf-tagged=false','--pretty=false',
        '--font-path',str(FONT),'--timings-json',str(timing)]
   p=subprocess.run(cmd,cwd=R,env=ENV,capture_output=True,text=True,timeout=120)
   assert p.returncode==0,(cmd,p.stderr)
   state['samples'].append({'fixture':fixture.stem,'rep':rep,'utc':utc(),'command':cmd,'timings':json.loads(timing.read_text())})
state['summary']={}
for name in sorted({x['fixture'] for x in state['samples']}):
 group=[x['timings'] for x in state['samples'] if x['fixture']==name]
 state['summary'][name]={k:statistics.median(x[k] for x in group) for k in group[0]}
 print(name,{k:round(v,2) for k,v in state['summary'][name].items()},flush=True)
assert sha(B)==state['binary_sha256']
assert state['fixtures']=={f.name:sha(f) for f in F.glob('*.typ')}
state['utc_end']=utc()
(D/'tekt-phases.json').write_text(json.dumps(state,indent=2)+'\n')
