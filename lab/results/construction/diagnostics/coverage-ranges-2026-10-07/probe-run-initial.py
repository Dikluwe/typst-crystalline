from pathlib import Path
import subprocess,os,json,hashlib,time,datetime,gzip,threading,importlib.util
D=Path(__file__).resolve().parent;P=D/'performance';R=Path.cwd();OUT=D/'probe-results';OUT.mkdir(exist_ok=True)
spec=importlib.util.spec_from_file_location('monitor',P/'monitor-helper.py');c=importlib.util.module_from_spec(spec);spec.loader.exec_module(c);c.OUT=OUT;c.ROOT=R;c.FONT_CACHE_PATH=OUT/'font-digest-cache.json'
paths=set()
for base in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts','.cargo',str(D/'probe')]:
 for p in (R/base).rglob('*'):
  if p.is_file() and not any(x in {'target','__pycache__','.git'} for x in p.parts):paths.add(p)
for p in [R/'Cargo.toml',R/'Cargo.lock',Path(__file__),P/'protocol.md',P/'monitor-helper.py']:paths.add(p)
for root in ['/usr/share/fonts','/usr/local/share/fonts','/home/dikluwe/.local/share/fonts','/usr/share/texmf/fonts','/etc/fonts','/home/dikluwe/.cargo/git/checkouts/typst-assets-525e6d15ef7950cb/c0ae970/files/fonts']:
 for p in Path(root).rglob('*'):
  if p.is_file():paths.add(p)
for name in set(subprocess.check_output(['fc-list','--format','%{file}\n'],text=True).splitlines()):
 if Path(name).is_file():paths.add(Path(name))
c.source_files=lambda:sorted(paths)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
binary=R/'target/release/coverage-production-probe';digest=sha(binary)
meta={'utc':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'status':subprocess.check_output(['git','status','--short'],text=True),'probe_sha256':digest,'fonts_sha256':{str(p):sha(p) for p in paths if p.suffix.lower() in {'.ttf','.otf','.ttc'}},'source_snapshot':'../tested-final.json.gz','environment':c.ENV,'cpu':4}
(OUT/'identity.json').write_text(json.dumps(meta,indent=2)+'\n');before=c.snapshot('before');gate=[]
print('probe quiet 30s',flush=True)
for _ in range(30):
 hits=c.blockers(c.processes(),None);assert not hits,hits;gate.append({'utc':utc(),'blockers':hits});time.sleep(1)
(OUT/'quiet-gate.json').write_text(json.dumps(gate,indent=2)+'\n')
telemetry=[];stop=threading.Event()
def monitor():
 while not stop.is_set():
  telemetry.append({'utc':utc(),'blockers':c.blockers(c.processes(),None),'loadavg':Path('/proc/loadavg').read_text()});stop.wait(.1)
th=threading.Thread(target=monitor,daemon=True);th.start();rows=[]
try:
 for mode in ['validate','extract','catalog']:
  cmd=['/usr/bin/time','-f','%M','-o',str(OUT/(mode+'.rss')),'/usr/bin/taskset','-c','4',str(binary),mode];row={'command':cmd,'start_utc':utc()}
  with (OUT/(mode+'.json')).open('w') as out,(OUT/(mode+'.stderr')).open('w') as err:p=subprocess.run(cmd,cwd=R,env=c.ENV,stdout=out,stderr=err)
  row.update(end_utc=utc(),status=p.returncode,maxrss_kib=int((OUT/(mode+'.rss')).read_text()));rows.append(row);assert p.returncode==0,row;print(mode,'ok',flush=True)
finally:stop.set();th.join()
after=c.snapshot('after');assert before['sha256']==after['sha256'];assert sha(binary)==digest;assert not any(t['blockers'] for t in telemetry)
(OUT/'execution.json').write_text(json.dumps({'rows':rows,'source_stable':True,'telemetry':telemetry},indent=2)+'\n')
