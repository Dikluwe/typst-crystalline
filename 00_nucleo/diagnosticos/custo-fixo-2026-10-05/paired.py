"""Final production baseline/candidate/vanilla CLI comparison, three rounds."""
import datetime,gzip,hashlib,json,os,statistics,subprocess,time
from pathlib import Path
H=Path(__file__).resolve().parent;R=H.parents[2];O=Path('/tmp/tekt-fixed-cost-2026-10-05');cpu=min(os.sched_getaffinity(0));sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();git=lambda *a:subprocess.check_output(['git',*a],cwd=R,text=True)
audit=json.load(gzip.open(H/'audit.json.gz','rt'));fixtures=dict(audit['fixtures']);fixtures['default_hello']='Hello World\n';fixtures['show_grid320']='#show "C": [C]\n'+fixtures['grid320'];fixtures['numbered_grid320']='#set page(numbering: "1 / 1")\n'+fixtures['grid320']
for n,s in fixtures.items():(O/(n+'.typ')).write_text(s)
engines={'baseline':O/'typst-baseline','candidate':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'}
files=[R/n for n in git('diff','--name-only').splitlines() if n.endswith(('.rs','.md','.toml'))]+[R/n for n in git('ls-files','--others','--exclude-standard').splitlines() if n.startswith(('01_core/','00_nucleo/prompts/'))]
r={'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'status':git('status','--short'),'diff_stat':git('diff','HEAD','--stat'),'source_hashes':{str(p.relative_to(R)):sha(p) for p in files},'binaries':{n:{'path':str(p),'sha256':sha(p)} for n,p in engines.items()},'cpu':cpu,'fixtures':fixtures,'rows':[]}
for round in range(3):
 for n in (list(fixtures) if round%2==0 else list(reversed(fixtures))):
  for i in range(-1,5):
   order=list(engines);order=order[(i+round)%3:]+order[:(i+round)%3]
   for e in order:
    out=O/f'{n}-{e}-paired.pdf';trace=O/f'{n}-{e}-paired.json';usage=O/f'{n}-{e}-paired.usage'
    cmd=['/usr/bin/time','-f','%M %U %S','-o',str(usage),str(engines[e]),'compile',str(O/(n+'.typ')),str(out),'--root',str(O),'--pdf-tagged=false','--pretty=false']
    cmd+=['--jobs','1'] if e=='vanilla' else ['--timings-json',str(trace)]
    start=time.perf_counter_ns();p=subprocess.run(cmd,text=True,capture_output=True,timeout=60,cwd=R,env=dict(os.environ,RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',NO_COLOR='1'),preexec_fn=lambda:os.sched_setaffinity(0,{cpu}));wall=(time.perf_counter_ns()-start)/1e6
    assert p.returncode==0,(cmd,p.stderr)
    if i>=0:
     rss,u,s=usage.read_text().split();row={'fixture':n,'engine':e,'round':round,'index':i,'wall_ms':wall,'rss_kib':int(rss),'cpu_s_coarse':float(u)+float(s),'stderr':p.stderr,'command':cmd,'pdf_sha256':sha(out),'finished':datetime.datetime.now(datetime.timezone.utc).isoformat()}
     if e!='vanilla':row['timings']=json.loads(trace.read_text())
     r['rows'].append(row)
 for e,p in engines.items():assert sha(p)==r['binaries'][e]['sha256']
 print('round complete',round,flush=True)
r['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat();r['summary']={n:{e:{'median_ms':statistics.median(x['wall_ms'] for x in r['rows'] if x['fixture']==n and x['engine']==e),'min_ms':min(x['wall_ms'] for x in r['rows'] if x['fixture']==n and x['engine']==e),'max_ms':max(x['wall_ms'] for x in r['rows'] if x['fixture']==n and x['engine']==e),'round_medians_ms':[statistics.median(x['wall_ms'] for x in r['rows'] if x['fixture']==n and x['engine']==e and x['round']==k) for k in range(3)],'rss_kib':statistics.median(x['rss_kib'] for x in r['rows'] if x['fixture']==n and x['engine']==e)} for e in engines} for n in fixtures}
with gzip.open(H/'paired.json.gz','wt') as f:json.dump(r,f,ensure_ascii=False,indent=2)
print(json.dumps(r['summary'],indent=2))
