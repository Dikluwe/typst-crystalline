from pathlib import Path
import datetime,gzip,hashlib,importlib.util,json,os,statistics,subprocess,time
R=Path.cwd();D=Path(__file__).resolve().parent;W=Path('/tmp/incremental-prioridade6-2026-10-08');W.mkdir(exist_ok=True)
s=importlib.util.spec_from_file_location('b',R/'00_nucleo/diagnosticos/macro-micro-2026-10-05/watch_probe.py');b=importlib.util.module_from_spec(s);s.loader.exec_module(b)
env=b.ENV.copy()
for k in list(env):
 if k.startswith(('TYPST_','CRYSTALLINE_','CARGO_PROFILE_')):env.pop(k)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();git=lambda *x:subprocess.check_output(['git',*x],text=True)
files={str(p):p.read_text() for base in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'] for p in (R/base).rglob('*') if p.is_file() and p.suffix in ['.rs','.toml','.md']}
state={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD'),'status':git('status','--porcelain=v1','--untracked-files=all'),'diff_stat':git('diff','HEAD','--stat'),'files':files,'binaries':{k:sha(v) for k,v in b.BIN.items()},'fonts':{p:sha(Path(p)) for p in set(subprocess.check_output(['fc-list','--format','%{file}\n'],text=True).splitlines()) if Path(p).is_file()}}
assert state['binaries']['tekt']=='897aa1261a069ef1dc601a0e5cb256380e901ac6a0324d1a288b405d5f6d0ff0';assert state['binaries']['vanilla']=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
with gzip.open(D/'before.json.gz','wt') as f:json.dump(state,f,ensure_ascii=False)
fixtures={'eval_import':'#let value = { let total = 0; for i in range(30000) { total += i }; total }\n','parse_import':''.join(f'#let v{i} = {i}\n' for i in range(4000))+'#let value = v3999\n'}
(D/'fixtures.json').write_text(json.dumps(fixtures))
rec={'started':state['utc'],'state_sha256':sha(D/'before.json.gz'),'cpu':b.CPU,'env':{k:env[k] for k in ['SOURCE_DATE_EPOCH','RAYON_NUM_THREADS','NO_COLOR']},'runs':[]}
for round in range(3):
 for name,library in fixtures.items():
  for engine in (['tekt','vanilla'] if round%2==0 else ['vanilla','tekt']):
   root=W/f'{name}-{engine}-{round}';root.mkdir(exist_ok=True);(root/'import.typ').write_text(library);main=root/'main.typ';pdf=root/'main.pdf';timing=root/'timing.json';logpath=root/'watch.log';pdf.unlink(missing_ok=True)
   source=lambda rev:'#set text(font: "Libertinus Serif")\n#import "import.typ": value\n#value\n'+f'Revision {rev}\n'
   def edit(rev):
    temp=root/'main.edit';temp.write_text(source(rev));os.replace(temp,main)
   edit(0);cmd=[str(b.BIN[engine]),'watch',str(main),str(pdf),'--root',str(root),'--pdf-tagged=false','--pretty=false']
   cmd+=['--timings-json',str(timing)] if engine=='tekt' else ['--jobs','1']
   rows=[]
   with logpath.open('w') as log:
    p=subprocess.Popen(cmd,cwd=R,env=env,stdout=log,stderr=log,preexec_fn=lambda:os.sched_setaffinity(0,{b.CPU}))
    try:
     b.wait_pdf(p,pdf,None)
     for rev in range(1,7):
      time.sleep(.16);prev=b.generation(pdf);cpu=b.cpu_runtime(p.pid);start=time.perf_counter_ns();edit(rev);b.wait_pdf(p,pdf,prev);latency=(time.perf_counter_ns()-start)/1e6;after=b.cpu_runtime(p.pid)
      assert set(cpu)==set(after)
      text=subprocess.check_output(['pdftotext','-layout',str(pdf),'-'],text=True);assert f'Revision {rev}' in text
      if rev>1:rows.append({'revision':rev,'cpu_ms':sum(after[t]-cpu[t] for t in cpu)/1e6,'latency_ms':latency,'timings':json.loads(timing.read_text()) if engine=='tekt' else None,'text':text})
    finally:p.terminate();p.wait(timeout=5)
   rec['runs'].append({'fixture':name,'engine':engine,'round':round,'command':cmd,'main':source(0),'import':library,'samples':rows,'log':logpath.read_text()});print(round,name,engine,statistics.median(x['cpu_ms'] for x in rows),flush=True)
assert all(Path(p).read_text()==v for p,v in files.items())
assert all(sha(p)==state['binaries'][k] for k,p in b.BIN.items())
rec['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat()
rec['summary']={name:{engine:{m:statistics.median(x[m] for row in rec['runs'] if row['fixture']==name and row['engine']==engine for x in row['samples']) for m in ['cpu_ms','latency_ms']} for engine in b.BIN} for name in fixtures}
with gzip.open(D/'measurement.json.gz','wt') as f:json.dump(rec,f,ensure_ascii=False,indent=2)
print(json.dumps(rec['summary'],indent=2))
