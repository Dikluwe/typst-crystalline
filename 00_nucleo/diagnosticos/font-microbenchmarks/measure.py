#!/usr/bin/env python3
"""Build-independent, gated measurement runner; writes only this new dataset."""
import datetime,hashlib,importlib.util,json,os,pathlib,platform,subprocess,threading,time
P=pathlib.Path(__file__).resolve().parent;ROOT=P.parents[2]
spec=importlib.util.spec_from_file_location('prior_read_only_helpers',ROOT/'00_nucleo/diagnosticos/font-lifecycle-experiment/run.py');helpers=importlib.util.module_from_spec(spec);spec.loader.exec_module(helpers)
# Only helpers.processes/blockers are used. Never call its writers/fingerprint.
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(path):
 h=hashlib.sha256()
 with open(path,'rb') as f:
  for block in iter(lambda:f.read(1048576),b''):h.update(block)
 return h.hexdigest()
def save(path,x):path.parent.mkdir(parents=True,exist_ok=True);path.write_text(json.dumps(x,indent=2)+'\n')
def command(args):return subprocess.check_output(args,cwd=ROOT,text=True)
def fingerprint():
 paths=set()
 for base in ['01_core','02_shell','03_infra','04_wiring','benches','.cargo']:
  paths.update(f for f in (ROOT/base).rglob('*') if f.is_file() and not any(t in {'target','.git','__pycache__'} for t in f.parts))
 paths.update(ROOT/f for f in ['Cargo.toml','Cargo.lock'] if (ROOT/f).exists())
 paths.update(f for f in P.glob('*') if f.is_file() and f.suffix in {'.py','.rs','.toml'} or f.name in {'Cargo.lock','inputs.json','protocol.md','historical-provenance.json'})
 paths.update((P/'src').glob('*.rs'))
 paths.add(ROOT/'00_nucleo/diagnosticos/font-lifecycle-experiment/originals/01_core/src/entities/font_book.rs.snapshot')
 paths.add(ROOT/'00_nucleo/diagnosticos/font-lifecycle-experiment/run.py')
 paths.update(pathlib.Path(r['path']) for r in json.loads((P/'inputs.json').read_text()))
 files={str(f.relative_to(ROOT)) if f.is_relative_to(ROOT) else str(f):sha(f) for f in sorted(paths)}
 return {'utc':utc(),'head':command(['git','rev-parse','HEAD']).strip(),'sha256':hashlib.sha256(json.dumps(files,sort_keys=True).encode()).hexdigest(),'files':files}
def snapshot(name):
 x=fingerprint();save(P/'state'/(name+'.json'),x)
 for suffix,args in [('status.txt',['git','status','--porcelain=v1','--untracked-files=all']),('diff-stat.txt',['git','diff','HEAD','--stat']),('patch',['git','diff','HEAD','--binary'])]: (P/'state'/(name+'.'+suffix)).write_text(command(args))
 return x
def readings():
 return {str(p):p.read_text().strip() for p in [pathlib.Path('/sys/devices/system/cpu/cpu4/cpufreq/scaling_governor'),pathlib.Path('/sys/devices/system/cpu/cpu4/cpufreq/scaling_cur_freq'),pathlib.Path('/sys/devices/system/cpu/cpufreq/boost')] if p.exists()}
def blocked():return helpers.blockers(helpers.processes(),None)
def main():
 os.sched_setaffinity(0,{6})
 artifacts=[]
 for line in (P/'logs/build.log').read_text().splitlines():
  try:r=json.loads(line)
  except ValueError:continue
  if r.get('reason')=='compiler-artifact' and r.get('executable') and r['target']['name']=='font-microbench-diagnostic':artifacts.append(r)
 r=artifacts[-1];assert r['profile']['opt_level']=='3' and not r['profile']['debug_assertions'] and not r['profile']['overflow_checks']
 exe=pathlib.Path(r['executable']);exe_sha=sha(exe)
 env=os.environ.copy()
 for k in list(env):
  if k.startswith(('CARGO_PROFILE_','TYPST_','CRYSTALLINE_')):env.pop(k)
 env.update(RUSTFLAGS='',CARGO_ENCODED_RUSTFLAGS='',CARGO_BUILD_JOBS='1',RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',TZ='UTC',LC_ALL='C.UTF-8',PYTHONDONTWRITEBYTECODE='1')
 save(P/'build-provenance.json',{'utc':utc(),'executable':str(exe),'sha256':exe_sha,'profile':r['profile'],'features':r['features'],'machine':platform.uname()._asdict(),'cpu':command(['lscpu']),'rustc':command(['rustc','-vV']),'cargo':command(['cargo','-V']),'environment':{k:env[k] for k in ['RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','RAYON_NUM_THREADS','SOURCE_DATE_EPOCH','TZ','LC_ALL']},'build_command':'CARGO_BUILD_JOBS=1 CARGO_ENCODED_RUSTFLAGS= RUSTFLAGS= cargo build --manifest-path '+str(P/'Cargo.toml')+' --release --offline --locked --target-dir target --message-format=json','policy_before':readings(),'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','vanilla_executable_sha256_auxiliary':sha(ROOT/'lab/typst-original/target/release/typst')})
 frozen=snapshot('frozen');save(P/'frozen-state.json',frozen)
 check=subprocess.run([str(exe),'--check'],env=env,cwd=ROOT,capture_output=True,text=True);assert check.returncode==0,check.stderr
 save(P/'fixture-controls.json',json.loads(check.stdout))
 for round_number in [1,2,3]:
  directory=P/'rounds'/('r'+str(round_number));assert not directory.exists();directory.mkdir(parents=True)
  print('Quiet gate r'+str(round_number),flush=True)
  start=time.monotonic()
  while time.monotonic()-start<30:
   assert not blocked(),'Concurrent workload during quiet gate';time.sleep(.1)
  before=snapshot('r'+str(round_number)+'-before');assert before['sha256']==frozen['sha256'] and before['head']==frozen['head'];assert sha(exe)==exe_sha
  stop=threading.Event();contamination=[];telemetry=[]
  def monitor():
   while not stop.wait(.1):
    hits=blocked();telemetry.append({'utc':utc(),'loadavg':pathlib.Path('/proc/loadavg').read_text().strip(),'blockers':hits})
    if hits:contamination.append({'utc':utc(),'blockers':hits})
  worker=threading.Thread(target=monitor);worker.start();started=utc();power=readings();code=None
  try:
   with (directory/'stdout.txt').open('w') as stdout,(directory/'stderr.txt').open('w') as stderr:
    code=subprocess.run([str(exe)],cwd=ROOT,env=env|{'FONT_MICRO_OUTPUT':str(directory/'criterion'),'FONT_MICRO_REVERSE':'1' if round_number==2 else '0'},stdout=stdout,stderr=stderr,preexec_fn=lambda:os.sched_setaffinity(0,{4})).returncode
  finally:stop.set();worker.join()
  after=snapshot('r'+str(round_number)+'-after');valid=code==0 and not contamination and before['sha256']==after['sha256']==frozen['sha256'] and before['head']==after['head'] and sha(exe)==exe_sha
  save(directory/'receipt.json',{'valid':valid,'start_utc':started,'end_utc':utc(),'exit_code':code,'before':before['sha256'],'after':after['sha256'],'head':before['head'],'executable_sha256':exe_sha,'command':[str(exe)],'affinity':4,'reverse_order':round_number==2,'contamination':contamination,'telemetry':telemetry,'policy_before':power,'policy_after':readings()})
  print('r'+str(round_number), 'VALID' if valid else 'DISCARDED',flush=True)
  assert valid,'Round rejected; preserve raw results and use a new matched round set'
if __name__=='__main__':main()
