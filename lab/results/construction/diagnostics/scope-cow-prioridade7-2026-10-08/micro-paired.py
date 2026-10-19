import datetime,hashlib,json,os,statistics,subprocess
from pathlib import Path
D=Path(__file__).resolve().parent;R=D.parents[2];cpu=min(os.sched_getaffinity(0))
engines={'previous':Path('/tmp/p7-scope-micro-before'),'current':Path('/tmp/p7-scope-micro-current')}
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
r={'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'cpu':cpu,'executables':{k:sha(p) for k,p in engines.items()},'source_state_sha256':sha(D/'final-state.json.gz'),'samples':[]}
for round in range(-1,5):
 order=list(engines)
 if round%2:order.reverse()
 for engine in order:
  output=subprocess.check_output([str(engines[engine])],text=True,preexec_fn=lambda:os.sched_setaffinity(0,{cpu}))
  if round>=0:
   for line in output.splitlines():
    name,size,n,nanos=line.split(',');r['samples'].append({'round':round,'engine':engine,'name':name,'size':int(size),'iterations':int(n),'nanos':int(nanos)})
summary={}
for name,size in sorted({(s['name'],s['size']) for s in r['samples']}):
 rows=[s for s in r['samples'] if s['name']==name and s['size']==size];a=[s['nanos'] for s in rows if s['engine']=='previous'];b=[s['nanos'] for s in rows if s['engine']=='current'];summary[f'{name}/{size}']={'previous_ms':statistics.median(a)/1e6,'current_ms':statistics.median(b)/1e6,'paired_ratios':[next(s['nanos'] for s in rows if s['engine']=='current' and s['round']==i)/next(s['nanos'] for s in rows if s['engine']=='previous' and s['round']==i) for i in range(5)]}
r['summary']=summary;r['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat();(D/'micro-paired.json').write_text(json.dumps(r,indent=2));print(json.dumps(summary,indent=2))
