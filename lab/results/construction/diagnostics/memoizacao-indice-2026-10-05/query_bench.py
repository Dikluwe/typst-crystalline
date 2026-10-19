"""Paired CLI query timings, separate from fonts/layout/export workloads."""
import datetime,gzip,hashlib,json,os,statistics,subprocess,time
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[2];OUT=Path('/tmp/tekt-index-reuse-2026-10-05')
engines={'baseline':OUT/'typst-baseline','candidate':ROOT/'target/release/typst'}
fixtures={}
for name,count,show in [('plain500',500,False),('show1',1,True),('show500',500,True),('show2000',2000,True)]:
 source=('#show heading: it => upper(it.body)\n' if show else '')+'\n'.join('= Heading '+str(i) for i in range(count))
 path=OUT/f'query-{name}.typ';path.write_text(source);fixtures[name]=path
cpu=min(os.sched_getaffinity(0));env=dict(os.environ,RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',NO_COLOR='1')
git=lambda *args:subprocess.check_output(['git',*args],cwd=ROOT,text=True)
r={'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'status':git('status','--short'),'diff_stat':git('diff','HEAD','--stat'),'binaries':{name:{'path':str(p),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for name,p in engines.items()},'cpu':cpu,'fixtures':{name:path.read_text() for name,path in fixtures.items()},'samples':[]}
for fixture,path in fixtures.items():
 for i in range(-1,15):
  order=list(engines)
  if i%2:order.reverse()
  for name in order:
   usage=OUT/'query.usage'
   cmd=['/usr/bin/time','-f','%M','-o',str(usage),str(engines[name]),'query',str(path),'heading','--field','body']
   start=time.perf_counter_ns();p=subprocess.run(cmd,cwd=ROOT,env=env,capture_output=True,text=True,timeout=60,preexec_fn=lambda:os.sched_setaffinity(0,{cpu}));wall_ms=(time.perf_counter_ns()-start)/1e6
   if p.returncode:raise RuntimeError(p.stderr)
   if i>=0:r['samples'].append({'fixture':fixture,'engine':name,'index':i,'command':cmd,'wall_ms':wall_ms,'max_rss_kib':int(usage.read_text()),'stdout':p.stdout,'stderr':p.stderr,'finished':datetime.datetime.now(datetime.timezone.utc).isoformat()})
r['summary']={fixture:{name:{'wall_ms':statistics.median(x['wall_ms'] for x in r['samples'] if x['fixture']==fixture and x['engine']==name),'max_rss_kib':statistics.median(x['max_rss_kib'] for x in r['samples'] if x['fixture']==fixture and x['engine']==name)} for name in engines} for fixture in fixtures}
r['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(HERE/'query-paired.json.gz','wt') as f:json.dump(r,f,ensure_ascii=False,indent=2)
print(json.dumps(r['summary'],indent=2))
for fixture in fixtures:
 groups={name:{x['stdout'] for x in r['samples'] if x['fixture']==fixture and x['engine']==name} for name in engines};print(fixture,'outputs_equal',groups['baseline']==groups['candidate'])
