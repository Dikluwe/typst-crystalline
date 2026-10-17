"""Same-window paired baseline/candidate/vanilla table benchmark."""
import datetime, gzip, hashlib, json, os, random, statistics, subprocess, tempfile, time
from pathlib import Path
D=Path(__file__).resolve().parent;R=D.parents[2];F=D/'fixtures';FONT=R/'00_nucleo/diagnosticos/performance-baseline/fonts'
B={'before':Path('/tmp/tekt-before-page-recipe-cache-2026-10-09'),'after':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'}
CPU=min(os.sched_getaffinity(0));ENV=os.environ.copy()
for k in list(ENV):
 if k.startswith(('TYPST_','CRYSTALLINE_','CARGO_PROFILE_')):ENV.pop(k)
ENV.update(RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',NO_COLOR='1',TZ='UTC',LC_ALL='C')
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def run(e,fixture,out,label):
 pdf=out/(label+'.pdf');usage=out/(label+'.usage')
 cmd=['/usr/bin/time','-f','%M %U %S','-o',str(usage),str(B[e]),'compile',str(fixture),str(pdf),'--root',str(F),'--pdf-tagged=false','--pretty=false','--font-path',str(FONT)]+(['--jobs','1'] if e=='vanilla' else [])
 t=time.perf_counter_ns();p=subprocess.run(cmd,cwd=R,env=ENV,capture_output=True,text=True,timeout=120,preexec_fn=lambda:os.sched_setaffinity(0,{CPU}));wall=(time.perf_counter_ns()-t)/1e6
 assert p.returncode==0,(cmd,p.stderr)
 rss,user,system=usage.read_text().split()
 return {'engine':e,'fixture':fixture.stem,'utc':utc(),'command':cmd,'wall_ms':wall,'peak_rss_kib':int(rss),'user_s':float(user),'system_s':float(system),'stderr':p.stderr,'pdf_sha256':sha(pdf)}
selected=[F/f'{v}-{n}.typ' for v,n in [('local-set',400),('local-set',1600),('no-fill',1600),('inline-set',1600)]]
state={'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'working_tree':'uncommitted','git_status':subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R,text=True),'git_diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),'binaries':{e:sha(b) for e,b in B.items()},'fixtures':{f.name:{'sha256':sha(f),'text':f.read_text()} for f in selected},'fonts':{p.name:sha(p) for p in FONT.iterdir() if p.is_file()},'script_sha256':sha(D/'measure_page_ref_empty.py'),'cpu':CPU,'environment':{k:ENV[k] for k in ('RAYON_NUM_THREADS','SOURCE_DATE_EPOCH','NO_COLOR','TZ','LC_ALL')},'samples':[]}
with tempfile.TemporaryDirectory(prefix='table-cell-defaults-') as temp:
 out=Path(temp)
 for round_no in range(3):
  cases=selected[:];random.Random(20261010+round_no).shuffle(cases)
  for fixture in cases:
   for pair in range(-1,4):
    engines=list(B);random.Random(20261011+round_no*101+pair*7+len(fixture.stem)).shuffle(engines)
    for e in engines:
     row=run(e,fixture,out,f'{round_no}-{fixture.stem}-{pair}-{e}')
     if pair>=0:state['samples'].append({'round':round_no,'pair':pair,**row})
   print(round_no,fixture.stem,'done',flush=True)
assert state['binaries']=={e:sha(b) for e,b in B.items()}
assert all(sha(F/name)==v['sha256'] for name,v in state['fixtures'].items())
assert state['script_sha256']==sha(D/'measure_page_ref_empty.py')
state['utc_end']=utc();state['summary']={}
for f in selected:
 name=f.stem;case={}
 for e in B:
  rows=[x for x in state['samples'] if x['fixture']==name and x['engine']==e]
  case[e]={'wall_median_ms':statistics.median(x['wall_ms'] for x in rows),'rss_median_kib':statistics.median(x['peak_rss_kib'] for x in rows)}
 for left,right,label in [('after','before','after_before'),('after','vanilla','after_vanilla'),('before','vanilla','before_vanilla')]:
  ratios=[]
  for round_no in range(3):
   for pair in range(4):
    p={x['engine']:x for x in state['samples'] if x['fixture']==name and x['round']==round_no and x['pair']==pair}
    ratios.append({'round':round_no,'wall':p[left]['wall_ms']/p[right]['wall_ms']-1,'rss':p[left]['peak_rss_kib']/p[right]['peak_rss_kib']-1})
  case[label]={'wall_ratio_median':statistics.median(x['wall'] for x in ratios),'rss_ratio_median':statistics.median(x['rss'] for x in ratios),'wall_round_median':[statistics.median(x['wall'] for x in ratios if x['round']==r) for r in range(3)]}
 state['summary'][name]=case
with gzip.open(D/'measurements-page-ref-empty.json.gz','wt') as p:json.dump(state,p,ensure_ascii=False,indent=2)
for n,c in sorted(state['summary'].items()): print(n,{e:round(c[e]['wall_median_ms'],2) for e in B},'after/before',round(c['after_before']['wall_ratio_median']*100,1),'after/vanilla',round(c['after_vanilla']['wall_ratio_median']*100,1),flush=True)
