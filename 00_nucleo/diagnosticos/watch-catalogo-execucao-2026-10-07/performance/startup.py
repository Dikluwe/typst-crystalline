"""Short dedicated startup pairs, no edits or concurrently live partner."""
from pathlib import Path
import importlib.util,json,os,subprocess,time,statistics
R=Path('/repos/Antigravity/typst-crystalline');D=Path(__file__).resolve().parent
s=importlib.util.spec_from_file_location('r',R/'00_nucleo/diagnosticos/watch-retencao-2026-10-07/run.py');r=importlib.util.module_from_spec(s);s.loader.exec_module(r);r.OUT=D
base=json.loads((D/'results.json').read_text());W=Path('/tmp/watch-catalogo-2026-10-07/performance');bins={e:next(W.glob(e+'-'+base['frozen_binaries'][e])) for e in ('previous','current')};rows=[];before=r.state('startup-before')
for round_ in range(3):
 for pair in range(3):
  for name in ('hello','grid320'):
   for e in (('previous','current') if (round_+pair)%2==0 else ('current','previous')):
    src=W/f'startup-{name}.typ';pdf=src.with_suffix('.pdf');src.write_text(r.b.source(name,0));pdf.unlink(missing_ok=True)
    cmd=[str(bins[e]),'watch',str(src),str(pdf),'--root',str(W),'--pdf-tagged=false','--pretty=false']
    with (W/'startup.log').open('w') as log:
     start=time.perf_counter_ns();p=subprocess.Popen(cmd,cwd=R,env=r.b.ENV,stdout=log,stderr=log,preexec_fn=lambda:os.sched_setaffinity(0,{r.b.CPU}))
     try:r.b.wait_pdf(p,pdf,None);wall=(time.perf_counter_ns()-start)/1e6;cpu=sum(r.b.cpu_runtime(p.pid).values())/1e6;rows.append({'round':round_,'pair':pair,'fixture':name,'engine':e,'wall_ms':wall,'cpu_ms':cpu,**r.rss(p.pid)})
     finally:p.terminate();p.wait(timeout=5)
after=r.state('startup-after');assert before['files']==after['files'];assert before['binaries']==after['binaries'];assert before['font_files']==after['font_files']
summary={n:{e:{k:statistics.median(x[k] for x in rows if x['fixture']==n and x['engine']==e) for k in ('wall_ms','cpu_ms')} for e in bins} for n in ('hello','grid320')}
(D/'startup-results.json').write_text(json.dumps({'samples':rows,'summary':summary,'cpu':r.b.CPU,'frozen_binaries':{e:r.b.sha(p) for e,p in bins.items()},'source_sha256':r.b.sha(Path(__file__))},indent=2)+'\n');print(json.dumps(summary),flush=True)
