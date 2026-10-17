"""Paired alternating edit events, two live watchers, observed CPU belongs to each watcher."""
from pathlib import Path
import importlib.util,inspect,json,os,subprocess,statistics,threading,time,hashlib,datetime
R=Path('/repos/Antigravity/typst-crystalline');D=Path(__file__).resolve().parent
s=importlib.util.spec_from_file_location('r',R/'00_nucleo/diagnosticos/watch-retencao-2026-10-07/run.py');r=importlib.util.module_from_spec(s);s.loader.exec_module(r);r.OUT=D
s=importlib.util.spec_from_file_location('m',R/'00_nucleo/diagnosticos/catalogo-seletivo-2026-10-07/performance-final/monitor-helper.py');m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
W=Path('/tmp/watch-catalogo-2026-10-07/performance/interleaved');W.mkdir(parents=True,exist_ok=True)
prior=json.loads((D/'results.json').read_text());bins={e:next(W.parent.glob(e+'-'+prior['frozen_binaries'][e])) for e in ('previous','current')};r.b.BIN.update(bins)
original=r.b.source;r.b.source=lambda name,rev:original(name,rev).replace('[C000]',f'[C000 revision {rev}]') if name=='grid320' else original(name,rev)
before=r.state('interleaved-before');rows=[];blockers=[];done=threading.Event()
def monitor():
 while not done.is_set():
  bad=m.blockers(m.processes(),None)
  if bad:blockers.append({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'processes':bad})
  done.wait(.05)
t=threading.Thread(target=monitor);t.start()
try:
 for round_ in range(3):
  for name in ('hello','grid320'):
   live={};logs={};initial={};commands={}
   try:
    for e in (('previous','current') if round_%2==0 else ('current','previous')):
     src=W/f'{name}-{round_}-{e}.typ';pdf=src.with_suffix('.pdf');timing=src.with_suffix('.json');src.write_text(r.b.source(name,0));pdf.unlink(missing_ok=True)
     logpath=src.with_suffix('.log');log=logpath.open('w');logs[e]=log
     cmd=[str(bins[e]),'watch',str(src),str(pdf),'--root',str(W),'--pdf-tagged=false','--pretty=false','--timings-json',str(timing)]
     started=time.perf_counter_ns();p=subprocess.Popen(cmd,cwd=R,env=r.b.ENV,stdout=log,stderr=log,preexec_fn=lambda:os.sched_setaffinity(0,{r.b.CPU}));live[e]=(p,src,pdf,timing,logpath)
     r.b.wait_pdf(p,pdf,None);initial[e]={'wall_ms':(time.perf_counter_ns()-started)/1e6,'cpu_ms':sum(r.b.cpu_runtime(p.pid).values())/1e6,**r.rss(p.pid)};commands[e]=cmd
    samples=[]
    # Warm both watchers once before the measured ten paired events.
    for rev in range(0,11):
     time.sleep(.16)
     for e in (('previous','current') if (round_+rev)%2==0 else ('current','previous')):
      p,src,pdf,timing,_=live[e];prev=r.b.generation(pdf);cpu=r.b.cpu_runtime(p.pid);start=time.perf_counter_ns();src.write_text(r.b.source(name,rev+1));r.b.wait_pdf(p,pdf,prev);latency=(time.perf_counter_ns()-start)/1e6;after=r.b.cpu_runtime(p.pid);assert set(cpu)==set(after)
      if not rev:continue
      text=subprocess.check_output(['pdftotext','-raw',str(pdf),'-'],text=True);sample={'revision':rev+1,'engine':e,'pair':rev,'cpu_ms':sum(after[x]-cpu[x] for x in after)/1e6,'latency_ms':latency,**r.rss(p.pid),'text':text,'source':src.read_text(),'timing':json.loads(timing.read_text())};samples.append(sample)
     if rev:
      pair=samples[-2:];assert pair[0]['text']==pair[1]['text']
    rows.append({'fixture':name,'round':round_,'initial':initial,'commands':commands,'samples':samples});print('completed',round_,name,flush=True)
   finally:
    for p,*_ in live.values():p.terminate();p.wait(timeout=5)
    for log in logs.values():log.close()
finally:done.set();t.join()
after=r.state('interleaved-after');assert before['files']==after['files'];assert before['binaries']==after['binaries'];assert before['font_files']==after['font_files']
summary={n:{e:{k:statistics.median(x[k] for row in rows if row['fixture']==n for x in row['samples'] if x['engine']==e) for k in ('cpu_ms','latency_ms','VmRSS','VmHWM')} for e in bins} for n in ('hello','grid320')}
rounds={n:{e:[statistics.median(x['cpu_ms'] for row in rows if row['fixture']==n and row['round']==i for x in row['samples'] if x['engine']==e) for i in range(3)] for e in bins} for n in summary}
result={'runs':rows,'summary':summary,'round_cpu_ms':rounds,'blockers':blockers,'cpu':r.b.CPU,'frozen_binaries':{e:r.b.sha(p) for e,p in bins.items()},'source_sha256':r.b.sha(Path(__file__)),'note':'Two idle/live watchers coexist; only one is edited/compiles at a time. Unchosen polling is included in environmental interference, not in its partner CPU.'};(D/'interleaved-results.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({'summary':summary,'round_cpu_ms':rounds,'blockers':len(blockers)},indent=2),flush=True)
