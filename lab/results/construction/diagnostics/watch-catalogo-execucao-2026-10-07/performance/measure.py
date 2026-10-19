"""Actual prior/current watchers, semantic edits, paired fresh processes."""
from pathlib import Path
import importlib.util,inspect,json,os,shutil,subprocess,statistics,threading,time,hashlib,gzip,datetime
R=Path('/repos/Antigravity/typst-crystalline');D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('r',R/'00_nucleo/diagnosticos/watch-retencao-2026-10-07/run.py');r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
spec=importlib.util.spec_from_file_location('monitor',R/'00_nucleo/diagnosticos/catalogo-seletivo-2026-10-07/performance-final/monitor-helper.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
r.OUT=D;r.WORK=Path('/tmp/watch-catalogo-2026-10-07/performance');r.WORK.mkdir(parents=True,exist_ok=True);r.b.OUT=r.WORK
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
binpaths={'previous':Path('/tmp/watch-catalogo-2026-10-07/previous-96f22a1d'),'current':R/'target/release/typst'}
assert sha(binpaths['previous'])=='96f22a1d63d51bfcc258ca2849563b037e66a67b7e34febaf4779da2dd146a4b'
for name,p in binpaths.items():
 dest=r.WORK/(name+'-'+sha(p));shutil.copyfile(p,dest);dest.chmod(0o755);r.b.BIN[name]=dest
original=r.b.source;r.b.source=lambda name,rev:original(name,rev).replace('[C000]',f'[C000 revision {rev}]') if name=='grid320' else original(name,rev)
# Both versions are Tekt and accept timings-json; preserve the historical run body.
code=inspect.getsource(r.run).replace("engine=='tekt'", "engine!='vanilla'");exec(code,r.__dict__)
process_id=[None];popen=subprocess.Popen
class WatchedPopen(popen):
 def __init__(self,*args,**kwargs):
  super().__init__(*args,**kwargs)
  if args and args[0] and 'watch' in args[0]:process_id[0]=self.pid
subprocess.Popen=WatchedPopen
before=r.state('before');rows=[];blockers=[];done=threading.Event()
def monitor():
 while not done.is_set():
  bad=m.blockers(m.processes(),process_id[0])
  if bad:blockers.append({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'processes':bad})
  done.wait(.05)
t=threading.Thread(target=monitor);t.start()
try:
 for round_ in range(3):
  for name in ('hello','grid320'):
   for engine in (('previous','current') if round_%2==0 else ('current','previous')):
    print('run',round_,name,engine,flush=True);rows.append(r.run(name,engine,round_));process_id[0]=None
finally:done.set();t.join()
after=r.state('after');assert before['files']==after['files'];assert before['binaries']==after['binaries'];assert before['font_files']==after['font_files']
for i in range(3):
 for name in ('hello','grid320'):
  a=next(x for x in rows if x['round']==i and x['fixture']==name and x['engine']=='previous');b=next(x for x in rows if x['round']==i and x['fixture']==name and x['engine']=='current');assert [x['text'] for x in a['samples']]==[x['text'] for x in b['samples']]
summary={name:{e:{k:statistics.median(x[k] for row in rows if row['fixture']==name and row['engine']==e for x in row['samples']) for k in ('cpu_ms','latency_ms','VmRSS','VmHWM')} for e in ('previous','current')} for name in ('hello','grid320')}
rounds={name:{e:[statistics.median(x['cpu_ms'] for row in rows if row['fixture']==name and row['engine']==e and row['round']==i for x in row['samples']) for i in range(3)] for e in ('previous','current')} for name in ('hello','grid320')}
result={'runs':rows,'summary':summary,'round_cpu_ms':rounds,'blockers':blockers,'cpu':r.b.CPU,'source_sha256':sha(Path(__file__)),'frozen_binaries':{n:sha(p) for n,p in r.b.BIN.items()},'env':{k:r.b.ENV.get(k) for k in ('RAYON_NUM_THREADS','SOURCE_DATE_EPOCH','TZ','LC_ALL','HOME','FONTCONFIG_FILE','FONTCONFIG_PATH')}}
(D/'results.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({'summary':summary,'round_cpu_ms':rounds,'blockers':len(blockers)},indent=2),flush=True)
