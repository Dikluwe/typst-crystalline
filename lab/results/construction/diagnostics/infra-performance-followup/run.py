"""Frozen release measurements; samples and provenance only in this new dataset."""
import os,sys,json,time,datetime,hashlib,subprocess,pathlib,threading,importlib.util,gzip,shutil
ROOT=pathlib.Path(__file__).resolve().parents[3];R=pathlib.Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('sealed_readonly',ROOT/'00_nucleo/diagnosticos/font-microbenchmarks-vanilla/measure.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
ENV=os.environ.copy()
for k in list(ENV):
 if k.startswith(('CARGO_PROFILE_','TYPST_','CRYSTALLINE_')):ENV.pop(k)
ENV.update(CARGO_ENCODED_RUSTFLAGS='',RUSTFLAGS='',CARGO_BUILD_JOBS='1',RAYON_NUM_THREADS='1',PYTHONDONTWRITEBYTECODE='1',SOURCE_DATE_EPOCH='0',TZ='UTC',LC_ALL='C.UTF-8')
ART=json.loads((R/'build-provenance.json').read_text())['artifacts'];V=ROOT/'lab/typst-original/target/release/typst';VSHA='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def save(path,value):path.parent.mkdir(parents=True,exist_ok=True);path.write_text(json.dumps(value,indent=2)+'\n')
stat_cache={}
def cached_sha(p):
 st=p.stat();key=(st.st_dev,st.st_ino,st.st_size,st.st_mtime_ns,st.st_ctime_ns)
 if str(p) not in stat_cache or stat_cache[str(p)][0]!=key:stat_cache[str(p)]=(key,m.sha(p))
 return stat_cache[str(p)][1]
def fp():
 f=m.fingerprint();files=f['files']
 for base in [R/'fontbench',R/'fixtures',R/'fonts']:
  for p in base.rglob('*'):
   if p.is_file() and 'target' not in p.parts:files[str(p.relative_to(ROOT))]=m.sha(p)
 for p in R.glob('*'):
  if p.is_file() and p.suffix in {'.py','.md'}:files[str(p.relative_to(ROOT))]=m.sha(p)
 paths=set(pathlib.Path(p) for p in subprocess.check_output(['fc-list','--format','%{file}\n'],text=True).splitlines())
 for base in ['/usr/share/fonts','/usr/local/share/fonts','/home/dikluwe/.local/share/fonts','/usr/share/texmf/fonts']:
  p=pathlib.Path(base)
  if p.exists():paths.update(x for x in p.rglob('*') if x.is_file() and x.suffix.lower() in {'.ttf','.otf','.ttc','.otc','.pfb','.pfa'})
 paths.update(p for p in pathlib.Path('/etc/fonts').rglob('*') if p.is_file())
 for p in paths:files[str(p)]=cached_sha(p)
 for name,row in ART.items():assert m.sha(pathlib.Path(row['path']))==row['sha256']
 assert m.sha(V)==VSHA
 f['sha256']=hashlib.sha256(json.dumps(files,sort_keys=True).encode()).hexdigest();return f
def policy():
 paths=['/proc/loadavg','/sys/devices/system/cpu/cpu4/cpufreq/scaling_governor','/sys/devices/system/cpu/cpu4/cpufreq/scaling_cur_freq','/sys/devices/system/cpu/cpufreq/boost']
 return {p:pathlib.Path(p).read_text().strip() if pathlib.Path(p).exists() else None for p in paths}
def blocked():return m.helpers.blockers(m.helpers.processes(),None)
FROZEN=None
def block(label,action):
 for attempt in range(1,4):
  directory=R/'blocks'/label/('a'+str(attempt));directory.mkdir(parents=True,exist_ok=False)
  start=utc();seen=[]
  for _ in range(30):
   seen.extend(blocked());time.sleep(1)
  if seen:save(directory/'receipt.json',{'valid':False,'quiet_gate_rejected':seen,'start_utc':start});print(label,'QUIET REJECTED',flush=True);continue
  before=fp();assert before['sha256']==FROZEN['sha256'] and before['head']==FROZEN['head'],'inputs changed before block'
  save(directory/'before.json',before);stop=threading.Event();contamination=[];telemetry=[]
  def monitor():
   while not stop.wait(.1):
    hits=blocked();telemetry.append({'utc':utc(),'load':pathlib.Path('/proc/loadavg').read_text().strip(),'blockers':hits});contamination.extend(hits)
  worker=threading.Thread(target=monitor);worker.start();p0=policy();error=None
  try:action(directory)
  except Exception as ex:error=repr(ex)
  finally:stop.set();worker.join()
  after=fp();save(directory/'after.json',after);valid=not error and not contamination and before['sha256']==after['sha256']==FROZEN['sha256'] and before['head']==after['head'];save(directory/'receipt.json',{'valid':valid,'start_utc':start,'end_utc':utc(),'error':error,'contamination':contamination,'telemetry':telemetry,'policy_before':p0,'policy_after':policy(),'fingerprint_before':before['sha256'],'fingerprint_after':after['sha256']});print(label,'VALID' if valid else 'DISCARDED',flush=True)
  if valid:return
  if error:raise RuntimeError(error)
 raise RuntimeError('No uncontaminated block after three attempts: '+label)
def invoke(cmd,directory,env=ENV):
 save(directory/'command.json',{'command':cmd,'environment_overrides':{k:env.get(k) for k in ['RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','RAYON_NUM_THREADS','CRITERION_HOME','FONT_MICRO_OUTPUT','FONT_MICRO_REVERSE']}})
 with (directory/'stdout.txt').open('w') as out,(directory/'stderr.txt').open('w') as err:subprocess.run(cmd,cwd=ROOT,env=env,stdout=out,stderr=err,check=True,preexec_fn=lambda:os.sched_setaffinity(0,{4}))
def micro(target,round_number,d):
 exe=ART[target]['path'];env=ENV|{'CRITERION_HOME':str(d/'criterion')}
 cmd=[exe] if target in {'observation_bench','stdlib_bench'} else [exe,'--bench','--sample-size','30','--warm-up-time','0.2','--measurement-time','0.6','--nresamples','5000','--noplot']
 invoke(cmd,d,env)
def font(round_number,d):invoke([ART['infra-font-followup']['path']],d,ENV|{'FONT_MICRO_OUTPUT':str(d/'criterion'),'FONT_MICRO_REVERSE':str(int(round_number==2))})
def cli(fixture,instrumented,round_number,d):
 samples=[]
 for number in range(-2,12 if not instrumented else 6):
  engines=['tekt','vanilla'] if (number+round_number)%2==0 else ['vanilla','tekt']
  for engine in engines:
   label=f'{number}-{engine}';output=d/(label+'.pdf');trace=d/(label+'.json');exe=ART['typst']['path'] if engine=='tekt' else str(V)
   cmd=[exe,'compile',str(fixture),str(output),'--root',str(fixture.parent),'--pdf-tagged=false','--pretty=false','--font-path',str(R/'fonts')]
   if engine=='vanilla':cmd+=['--jobs','1']
   if instrumented:cmd+=['--timings-json' if engine=='tekt' else '--timings',str(trace)]
   with (d/(label+'.stdout')).open('w') as out,(d/(label+'.stderr')).open('w') as err:
    begin=utc();start=time.perf_counter_ns();result=subprocess.run(cmd,cwd=ROOT,env=ENV,stdout=out,stderr=err,check=True,preexec_fn=lambda:os.sched_setaffinity(0,{4}));elapsed=(time.perf_counter_ns()-start)/1e6
   row={'pair':number,'engine':engine,'round':round_number,'fixture':fixture.stem,'instrumented':instrumented,'command':cmd,'utc':begin,'wall_ms':elapsed,'executable_sha256':ART['typst']['sha256'] if engine=='tekt' else VSHA,'input_sha256':m.sha(fixture),'output_sha256':m.sha(output),'output_bytes':output.stat().st_size,'output':str(output.relative_to(R))}
   if instrumented:row['trace']=str(trace.relative_to(R))
   if number>=0:samples.append(row)
 save(d/'samples.json',samples)
def main():
 global FROZEN
 original=json.loads((ROOT/'00_nucleo/diagnosticos/font-lifecycle-experiment/vanilla-provenance.json').read_text());staged=json.loads((ROOT/'00_nucleo/diagnosticos/font-microbenchmarks-vanilla/staged-provenance.json').read_text());assert original['pin']==staged['pin']=='9f2b6e8715237cb086899a42873660fe744622e8'
 for name,digest in staged['source_files'].items():assert m.sha(ROOT/'lab/typst-original'/name)==digest and m.sha(pathlib.Path(staged['staged_path'])/name)==digest,name
 save(R/'vanilla-identity.json',{'pin':original['pin'],'tree':original['tree'],'cli_sha256':m.sha(V),'original_staged_sources_verified':True,'source_deviations':original['source_differences'],'combined_harness_lockfile_sha256':m.sha(R/'fontbench/Cargo.lock')})
 FROZEN=fp();save(R/'frozen-state.json',FROZEN)
 save(R/'frozen-status.json',{'status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],cwd=ROOT,text=True),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True)})
 (R/'incoming.patch.gz').write_bytes(gzip.compress(subprocess.check_output(['git','diff','HEAD','--binary'],cwd=ROOT),mtime=0));save(R/'window-start.json',{'utc':utc(),'policy':policy(),'fingerprint':FROZEN['sha256']})
 targets=['scanner_bench','content_bench','layout_bench','pdf_builder_bench','stream_bench','pipeline_bench','observation_bench','stdlib_bench']
 for round_number in [1,2,3]:
  def micro_cohort(d):
   for target in (targets if round_number!=2 else targets[::-1]):
    child=d/target;child.mkdir();micro(target,round_number,child)
  block(f'micro-r{round_number}',micro_cohort)
  block(f'font-r{round_number}',lambda d,n=round_number:font(n,d))
  def cli_cohort(d):
   for instrumented in [False,True]:
    for fixture in sorted((R/'fixtures').glob('*.typ')):
     child=d/(('traced-' if instrumented else 'plain-')+fixture.stem);child.mkdir();cli(fixture,instrumented,round_number,child)
  block(f'cli-r{round_number}',cli_cohort)
 after=fp();assert after['sha256']==FROZEN['sha256'];save(R/'window-end.json',{'utc':utc(),'policy':policy(),'fingerprint':after['sha256']});print('Frozen measurement window complete.',flush=True)
if __name__=='__main__':main()
