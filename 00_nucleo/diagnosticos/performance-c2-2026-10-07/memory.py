from pathlib import Path
import subprocess,os,time,json,hashlib,threading,importlib.util,itertools
D=Path(__file__).resolve().parent;R=D.parents[2];M=D/'memory';M.mkdir(exist_ok=True)
spec=importlib.util.spec_from_file_location('memory_monitor',D/'monitor-helper.py');c=importlib.util.module_from_spec(spec);spec.loader.exec_module(c);c.OUT=M;c.ROOT=R;c.FONT_CACHE_PATH=M/'font-digest-cache.json'
start=json.loads((D/'state/start.json').read_text());paths=[Path(p) if p.startswith('/') else R/p for p in start['files']];paths+=[D/'memory.py',D/'protocol-memory.md'];c.source_files=lambda:sorted(paths)
meta=json.loads((D/'identity.json').read_text());B={k:Path(v) for k,v in meta['binaries'].items()};expected=meta['bin_sha256'];sha=c.sha;env=dict(c.ENV);env['NO_COLOR']='1';fontdir=R/'00_nucleo/diagnosticos/performance-baseline/fonts'
before=c.snapshot('start');print('memory quiet gate30s',flush=True);gate=[]
for _ in range(30):
 hits=c.blockers(c.processes(),None);gate.append({'utc':c.utc(),'blockers':hits});assert not hits,hits;time.sleep(1)
(M/'quiet-gate.json').write_text(json.dumps(gate,indent=2)+'\n');assert c.snapshot('quiet-after')['sha256']==before['sha256']
def one(engine,fixture,directory,label):
 assert sha(B[engine])==expected[engine];pdf=directory/(label+'.pdf');metric=directory/(label+'.metric.json');cmd=['/usr/bin/time','-f','{"maxrss_kib":%M,"user_s":%U,"system_s":%S}','-o',str(metric),'/usr/bin/taskset','-c','4',str(B[engine]),'compile',str(fixture),str(pdf),'--root',str(fixture.parent),'--pdf-tagged=false','--pretty=false','--font-path',str(fontdir)]
 if engine=='vanilla':cmd+=['--jobs','1']
 start=c.utc()
 with (directory/(label+'.stdout')).open('wb') as out,(directory/(label+'.stderr')).open('wb') as err:r=subprocess.run(cmd,cwd=R,env=env,stdout=out,stderr=err)
 row={'engine':engine,'command':cmd,'start_utc':start,'end_utc':c.utc(),'status':r.returncode,'stderr':(directory/(label+'.stderr')).read_text(),'metrics':json.loads(metric.read_text()) if r.returncode==0 else None,'pdf_sha256':sha(pdf) if pdf.exists() else None};assert r.returncode==0,row;assert sha(B[engine])==expected[engine];return row
orders=list(itertools.permutations(B));index=[]
for rnd in [1,2,3]:
 for fixture in sorted((D/'fixtures').glob('*.typ')):
  label=f'r{rnd}-{fixture.stem}';directory=M/'samples'/label;directory.mkdir(parents=True,exist_ok=False);fp=c.snapshot(label+'-before');contamination=[];telemetry=[];stop=threading.Event()
  def monitor():
   while not stop.is_set():
    hits=c.blockers(c.processes(),None);contamination.extend(hits);telemetry.append({'utc':c.utc(),'blockers':hits,'loadavg':Path('/proc/loadavg').read_text().strip()});stop.wait(.1)
  assert not c.blockers(c.processes(),None);thread=threading.Thread(target=monitor,daemon=True);thread.start();samples=[];warmups=[]
  try:
   for warm in range(2):
    for engine in B:warmups.append(one(engine,fixture,directory,f'warm{warm}-{engine}'))
   for pair in range(6):
    for engine in orders[(pair+rnd-1)%6]:
     row=one(engine,fixture,directory,f'p{pair}-{engine}');row.update(pair=pair,round=rnd,fixture=fixture.stem);samples.append(row)
  finally:stop.set();thread.join()
  after=c.snapshot(label+'-after');valid=fp['sha256']==after['sha256'] and not contamination;p=directory/'samples.json';p.write_text(json.dumps({'valid':valid,'fingerprint_before':fp['sha256'],'fingerprint_after':after['sha256'],'samples':samples,'warmups':warmups,'telemetry':telemetry,'contamination':contamination},indent=2)+'\n');index.append(str(p.relative_to(M)));(M/'index.json').write_text(json.dumps({'utc':c.utc(),'blocks':index},indent=2)+'\n');print('memory',label,'VALID' if valid else 'REJECTED',flush=True);assert valid
assert c.snapshot('end')['sha256']==before['sha256'];print('MEMORY FINISHED',flush=True)
