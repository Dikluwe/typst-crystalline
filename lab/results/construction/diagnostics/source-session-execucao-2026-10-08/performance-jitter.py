from pathlib import Path
import datetime,gzip,hashlib,importlib.util,json,os,statistics,subprocess,threading,time,random
jitter=random.Random(61008)
R=Path.cwd();D=Path(__file__).resolve().parent;W=Path('/tmp/p6-watch-performance-jitter');W.mkdir(exist_ok=True)
s=importlib.util.spec_from_file_location('b',R/'00_nucleo/diagnosticos/macro-micro-2026-10-05/watch_probe.py');b=importlib.util.module_from_spec(s);s.loader.exec_module(b)
s=importlib.util.spec_from_file_location('m',R/'00_nucleo/diagnosticos/catalogo-seletivo-2026-10-07/performance-final/monitor-helper.py');m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
bins={'previous':Path('/tmp/p6-previous-897aa126'),'current':R/'target/release/typst'};sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
env=b.ENV.copy()
for k in list(env):
 if k.startswith(('TYPST_','CRYSTALLINE_','CARGO_PROFILE_')):env.pop(k)
fixtures=json.loads((R/'00_nucleo/diagnosticos/incremental-prioridade6-2026-10-08/fixtures.json').read_text());fixtures['minimal_import']='#let value = 3999\n'
files={str(p):sha(p) for root in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'] for p in (R/root).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']};fonts={p:sha(Path(p)) for p in set(subprocess.check_output(['fc-list','--format','%{file}\n'],text=True).splitlines()) if Path(p).is_file()}
rec={'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True),'status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'cpu':b.CPU,'binaries':{k:sha(p) for k,p in bins.items()},'files':files,'fonts':fonts,'fixtures':fixtures,'runs':[],'blockers':[]};assert rec['binaries']['previous']!=rec['binaries']['current']
done=threading.Event()
def monitor():
 while not done.is_set():
  bad=m.blockers(m.processes(),None)
  if bad:rec['blockers'].append({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'processes':bad})
  done.wait(.05)
t=threading.Thread(target=monitor);t.start()
def rss(pid):
 data=dict(line.split(':',1) for line in Path(f'/proc/{pid}/status').read_text().splitlines() if ':' in line);return {k:int(data[k].split()[0]) for k in ['VmRSS','VmHWM']}
try:
 for round in range(3):
  for name,library in fixtures.items():
   live={};logs=[];row={'round':round,'fixture':name,'samples':[],'initial':{}}
   try:
    for e in (list(bins) if round%2==0 else list(reversed(bins))):
     root=W/f'{name}-{e}-{round}';root.mkdir(exist_ok=True);(root/'import.typ').write_text(library);main=root/'main.typ';pdf=root/'main.pdf';timing=root/'timing.json';pdf.unlink(missing_ok=True)
     text=lambda rev:'#set text(font: "Libertinus Serif")\n#import "import.typ": value\n#value\nRevision '+str(rev)+'\n'
     main.write_text(text(0));log=(root/'watch.log').open('w');logs.append(log);cmd=[str(bins[e]),'watch',str(main),str(pdf),'--root',str(root),'--pdf-tagged=false','--pretty=false','--timings-json',str(timing)]
     start=time.perf_counter_ns();p=subprocess.Popen(cmd,cwd=R,env=env,stdout=log,stderr=log,preexec_fn=lambda:os.sched_setaffinity(0,{b.CPU}));live[e]=(p,main,pdf,timing);b.wait_pdf(p,pdf,None)
     row['initial'][e]={'command':cmd,'wall_ms':(time.perf_counter_ns()-start)/1e6,'cpu_ms':sum(b.cpu_runtime(p.pid).values())/1e6,**rss(p.pid)}
    for rev in range(1,12):
     time.sleep(jitter.uniform(.13,.31))
     for e in (list(bins) if (round+rev)%2==0 else list(reversed(bins))):
      p,main,pdf,timing=live[e];prev=b.generation(pdf);cpu=b.cpu_runtime(p.pid);start=time.perf_counter_ns();temp=main.with_suffix('.edit');temp.write_text(text(rev));os.replace(temp,main);b.wait_pdf(p,pdf,prev);latency=(time.perf_counter_ns()-start)/1e6;after=b.cpu_runtime(p.pid);assert set(cpu)==set(after)
      output=subprocess.check_output(['pdftotext','-layout',str(pdf),'-'],text=True);assert f'Revision {rev}' in output
      if rev>1:row['samples'].append({'pair':rev,'engine':e,'cpu_ms':sum(after[t]-cpu[t] for t in cpu)/1e6,'latency_ms':latency,**rss(p.pid),'text':output,'timing':json.loads(timing.read_text())})
     if rev>1:assert row['samples'][-2]['text']==row['samples'][-1]['text']
    rec['runs'].append(row);print(round,name,'done',flush=True)
   finally:
    for p,*_ in live.values():p.terminate();p.wait(timeout=5)
    for log in logs:log.close()
finally:done.set();t.join()
assert not rec['blockers'],rec['blockers'];assert all(sha(Path(p))==v for p,v in files.items());assert all(sha(Path(p))==v for p,v in fonts.items());assert all(sha(p)==rec['binaries'][k] for k,p in bins.items())
rec['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat()
summary={}
for name in fixtures:
 rows=[x for row in rec['runs'] if row['fixture']==name for x in row['samples']];out={}
 for measure in ['cpu_ms','latency_ms','VmRSS','eval_ms','layout_ms']:
  value=lambda x:x['timing'][measure] if measure in ['eval_ms','layout_ms'] else x[measure]
  rounds=[];ratios=[]
  for round in range(3):
   rr=[];samples=next(row['samples'] for row in rec['runs'] if row['fixture']==name and row['round']==round)
   for pair in range(2,12):
    values={x['engine']:value(x) for x in samples if x['pair']==pair};rr.append(values['current']/values['previous'])
   ratios+=rr;rounds.append(statistics.median(rr))
  out[measure]={'previous':statistics.median(value(x) for x in rows if x['engine']=='previous'),'current':statistics.median(value(x) for x in rows if x['engine']=='current'),'paired_ratio':statistics.median(ratios),'round_ratios':rounds,'lower_pairs':sum(r<1 for r in ratios)}
 summary[name]=out
rec['summary']=summary
with gzip.open(D/'jitter-performance.json.gz','wt') as f:json.dump(rec,f,ensure_ascii=False,indent=2)
(D/'jitter-performance-summary.json').write_text(json.dumps(summary,indent=2));print(json.dumps(summary,indent=2))
