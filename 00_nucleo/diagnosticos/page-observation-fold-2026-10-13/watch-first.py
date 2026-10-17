from common import *
import gzip,resource,sys,importlib.util
monitor_path=ROOT/'00_nucleo/diagnosticos/table-local-c2r1-2026-10-07/performance/monitor-helper.py'
spec=importlib.util.spec_from_file_location('monitor',monitor_path);monitor=importlib.util.module_from_spec(spec);spec.loader.exec_module(monitor)
resource.setrlimit(resource.RLIMIT_STACK,(64*1024*1024,resource.getrlimit(resource.RLIMIT_STACK)[1]))
BINS={'before':Path('/tmp/tekt-page-fold/before'),'after':Path('/tmp/tekt-page-fold/after')}
label=sys.argv[1];x={'utc_start':now(),'label':label,'state_sha256':sha(HERE/'candidate.json.gz'),'binaries':{k:sha(v) for k,v in BINS.items()},'script_sha256':sha(__file__),'common_sha256':sha(HERE/'common.py'),'cpu':CPU,'env':{k:ENV[k] for k in ['RAYON_NUM_THREADS','SOURCE_DATE_EPOCH','TZ','NO_COLOR','LC_ALL']},'samples':[]}
with tempfile.TemporaryDirectory(prefix='page-fold-watch-') as tmp:
 for round in range(3):
  for n in [300,1600]:
   body=(HERE/'fixtures'/f'context{n}.typ').read_text();x.setdefault('fixtures',{})[str(n)]={'text':body,'sha256':hashlib.sha256(body.encode()).hexdigest()}
   for engine in (['before','after'] if round%2==0 else ['after','before']):
    folder=Path(tmp)/f'{round}-{n}-{engine}';folder.mkdir();main=folder/'main.typ';pdf=folder/'main.pdf';log=folder/'watch.log';main.write_text(body)
    cmd=[str(BINS[engine]),'watch',str(main),str(pdf),'--pdf-tagged=false','--pretty=false']
    blocked=monitor.blockers(monitor.processes(),None);assert not blocked,blocked
    rows=[]
    with log.open('w') as out:
     p=subprocess.Popen(cmd,cwd=ROOT,env=ENV,stdout=out,stderr=out,preexec_fn=lambda:os.sched_setaffinity(0,{CPU}))
     try:
      assert wait_pdf(p,pdf,None,30)
      for cycle in range(16):
       blocked=monitor.blockers(monitor.processes(),p.pid);assert not blocked,blocked
       kind=['same','word','structure'][cycle%3];time.sleep(.18);old=generation(pdf);c=cpu_ns(p.pid);start=time.perf_counter_ns()
       text=body.replace('Alpha','Bravo' if cycle%2 else 'Alpha') if kind=='word' else body
       if kind=='structure':text=text.replace(f'range({n})',f'range({n+20})')
       main.write_text(text);assert wait_pdf(p,pdf,old,30);lat=(time.perf_counter_ns()-start)/1e6;time.sleep(.12)
       cpu=(cpu_ns(p.pid)-c)/1e6
       if cycle>=4:rows.append({'cycle':cycle,'kind':kind,'cpu_ms':cpu,'latency_ms':lat,**mem(p.pid),'source_sha256':sha(main),'utc':now()})
     finally:p.terminate();p.wait(timeout=5)
    x['samples'].append({'round':round,'n':n,'engine':engine,'command':cmd,'rows':rows,'log':log.read_text()});print(round,n,engine,statistics.median(r['cpu_ms'] for r in rows),flush=True)
x['summary']={str(n):{e:{'cpu_median_ms':statistics.median(s['cpu_ms'] for run in x['samples'] if run['n']==n and run['engine']==e for s in run['rows']),'rss_median_kib':statistics.median(s['VmRSS'] for run in x['samples'] if run['n']==n and run['engine']==e for s in run['rows'])} for e in BINS} for n in [300,1600]};x['utc_end']=now()
assert x['binaries']=={k:sha(v) for k,v in BINS.items()}
with gzip.open(HERE/f'watch-{label}.json.gz','wt') as f:json.dump(x,f)
print(x['summary'],flush=True)
