"""Paired CLI/watch-startup timing driver. See README.md for RAM layout.

TEKT_CATALOG_RAM must identify the exclusive RAM directory; no disk fallback.
"""
from pathlib import Path
import subprocess as sp,os,time,resource,select,signal,json,statistics as S,random
D=Path(os.environ['TEKT_CATALOG_RAM']);F=D/'fixtures';CPU=min(os.sched_getaffinity(0));envbase={k:v for k,v in os.environ.items() if not k.startswith(('TYPST_','CRYSTALLINE_','CAT_PROBE','E_JOINT','CPU_PROFILE')) and k!='LD_PRELOAD'};envbase.update(TMPDIR=str(D/'tmp'),RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',TZ='UTC',NO_COLOR='1')
bins={'original':D/'base-target/release/typst','native_off':D/'timing-target/release/typst','native_on':D/'timing-target/release/typst','joint_off':D/'timing-target/release/typst','joint_on':D/'timing-target/release/typst'}
if not D.resolve().is_relative_to(Path('/dev/shm')) or D.resolve()==Path('/dev/shm') or not D.is_dir():
 raise ValueError('TEKT_CATALOG_RAM must be an existing exclusive child of /dev/shm')
def setup():os.sched_setaffinity(0,{CPU});resource.setrlimit(resource.RLIMIT_STACK,(64*1024*1024,64*1024*1024))
def eof(p):
 try:
  with p.open('rb') as f:f.seek(0,2);n=f.tell();f.seek(max(0,n-256));return f.read().rstrip().endswith(b'%%EOF')
 except FileNotFoundError:return False
def run(v,mode,round,rep,name='plain',bucket='c02'):
 P=D/bucket/f'{round}-{rep}-{name}-{mode}-{v}';P.mkdir(parents=True,exist_ok=False);pdf=P/'out.pdf';dump=P/'probe.tsv';env=dict(envbase,CAT_PROBE='on' if v.endswith('_on') else 'off',CAT_PROBE_OUT=str(dump))
 if v.startswith('joint'):env['E_JOINT_DISCOVERY']='1'
 argv=[str(bins[v]),'compile' if mode=='cli' else 'watch',str(F/(name+'.typ')),str(pdf),'--root',str(F),'--pdf-tagged=false'];p=None;fd=None
 with (P/'log').open('wb') as log:
  try:
   started=time.time();t=time.perf_counter_ns();p=sp.Popen(argv,stdout=log,stderr=log,env=env,cwd=F,start_new_session=True,preexec_fn=setup);fd=os.pidfd_open(p.pid)
   if mode=='cli':
    assert select.select([fd],[],[],60)[0],P;_,status,ru=os.wait4(p.pid,0);p.returncode=os.waitstatus_to_exitcode(status);wall=(time.perf_counter_ns()-t)/1e6;assert p.returncode==0,P
   else:
    limit=time.monotonic()+60
    while not eof(pdf):assert not select.select([fd],[],[],0)[0] and time.monotonic()<limit,P;time.sleep(.001)
    wall=(time.perf_counter_ns()-t)/1e6;time.sleep(.03);os.killpg(p.pid,signal.SIGTERM);assert select.select([fd],[],[],5)[0],P;_,status,ru=os.wait4(p.pid,0);p.returncode=os.waitstatus_to_exitcode(status);assert p.returncode in [0,-signal.SIGTERM],P
   assert eof(pdf),P
   assert dump.exists()==v.endswith('_on'),(P,'probe On/Off mismatch')
  finally:
   if fd is not None:os.close(fd)
   if p is not None and p.returncode is None:
    try:os.killpg(p.pid,signal.SIGKILL)
    except ProcessLookupError:pass
    _,status,ru=os.wait4(p.pid,0);p.returncode=os.waitstatus_to_exitcode(status)
 return dict(variant=v,mode=mode,round=round,repeat=rep,fixture=name,sample_epoch=started,wall_ms=wall,cpu_ms=(ru.ru_utime+ru.ru_stime)*1000,rss_bytes=ru.ru_maxrss*1024,pdf=str(pdf),probe=str(dump) if dump.exists() else None)
def ci(x):
 q=random.Random(0);a=sorted(S.median(q.choices(x,k=len(x))) for _ in range(10000));return [a[249],a[9749]]
if __name__=='__main__':
 vs=list(bins);rows=[]
 for mode in ['cli','startup']:
  for v in vs:run(v,mode,'warmup',0)
 for r in range(12):
  for mode in (['cli','startup'] if r%2==0 else ['startup','cli']):
   for rep in range(3):
    order=vs[(r+rep)%5:]+vs[:(r+rep)%5]
    for v in order:rows.append(run(v,mode,r,rep))
  (D/'c02-raw.json').write_text(json.dumps(rows));print('round',r+1,flush=True)
 out=[]
 for mode in ['cli','startup']:
  g={v:[{m:S.mean(x[m] for x in rows if(x['variant'],x['mode'],x['round'])==(v,mode,r)) for m in ['wall_ms','cpu_ms','rss_bytes']} for r in range(12)] for v in vs};z=dict(mode=mode,medians={v:{m:S.median(x[m] for x in g[v]) for m in ['wall_ms','cpu_ms','rss_bytes']} for v in vs},pairs={})
  for a,b in [('native_off','original'),('native_on','native_off'),('joint_on','joint_off'),('joint_off','native_off'),('joint_off','original')]:
   z['pairs'][a+'/'+b]={}
   for m in ['wall_ms','cpu_ms']:
    x=[(u[m]/v[m]-1)*100 for u,v in zip(g[a],g[b])];z['pairs'][a+'/'+b][m]=dict(percent=S.median(x),ci=ci(x))
  out.append(z)
 (D/'c02-summary.json').write_text(json.dumps(out,indent=2));print('360 measured processes complete',flush=True)
