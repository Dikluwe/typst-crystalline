from pathlib import Path
import subprocess,os,json,hashlib,shutil,datetime,platform
ROOT=Path(__file__).resolve().parents[3];R=Path(__file__).resolve().parent
ENV=os.environ.copy()
for k in list(ENV):
 if k.startswith(('CARGO_PROFILE_','TYPST_','CRYSTALLINE_')):ENV.pop(k)
ENV.update(CARGO_ENCODED_RUSTFLAGS='',RUSTFLAGS='',CARGO_BUILD_JOBS='1',RAYON_NUM_THREADS='1',PYTHONDONTWRITEBYTECODE='1',SOURCE_DATE_EPOCH='0',TZ='UTC',LC_ALL='C.UTF-8')
commands=[('cli',['cargo','build','--release','--offline','--locked','--message-format=json']),('benches',['cargo','bench','--manifest-path','benches/Cargo.toml','--no-run','--offline','--locked','--message-format=json']),('core-tests',['cargo','test','-p','typst-core','--release','--lib','--no-run','--offline','--locked','--message-format=json']),('fontbench',['cargo','build','--manifest-path',str(R/'fontbench/Cargo.toml'),'--release','--offline','--locked','--target-dir',str(ROOT/'target'),'--message-format=json'])]
records=[];artifacts={};(R/'build').mkdir(exist_ok=True);(R/'bin').mkdir(exist_ok=True)
for label,cmd in commands:
 print('Preparing '+label,flush=True)
 start=datetime.datetime.now(datetime.timezone.utc).isoformat()
 with (R/'build'/(label+'.jsonl')).open('w') as out,(R/'build'/(label+'.stderr')).open('w') as err:code=subprocess.run(cmd,cwd=ROOT,env=ENV,stdout=out,stderr=err).returncode
 records.append({'label':label,'command':cmd,'start_utc':start,'end_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'exit_code':code});assert code==0,label
 for line in (R/'build'/(label+'.jsonl')).read_text().splitlines():
  try:j=json.loads(line)
  except ValueError:continue
  if j.get('reason')!='compiler-artifact' or not j.get('executable'):continue
  prof=j['profile'];assert prof['opt_level']=='3' and prof['debug_assertions'] is False and prof['overflow_checks'] is False,(label,prof)
  name=j['target']['name'];dest=R/'bin'/(name if label!='core-tests' else 'core-benchmark-tests');shutil.copy2(j['executable'],dest)
  artifacts[dest.name]={'path':str(dest),'built_path':j['executable'],'sha256':hashlib.sha256(dest.read_bytes()).hexdigest(),'profile':prof,'target':j['target'],'features':j['features']}
(R/'build-provenance.json').write_text(json.dumps({'commands':records,'artifacts':artifacts,'machine':platform.uname()._asdict(),'cpu':subprocess.check_output(['lscpu'],text=True),'rustc':subprocess.check_output(['rustc','-vV'],text=True),'cargo':subprocess.check_output(['cargo','-V'],text=True),'environment':{k:ENV.get(k) for k in ['RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','CARGO_BUILD_JOBS','RAYON_NUM_THREADS','SOURCE_DATE_EPOCH','TZ','LC_ALL']}},indent=2)+'\n')
for name in ['observation_bench','stdlib_bench']:
 with (R/'build'/(name+'-preflight.stdout')).open('w') as out,(R/'build'/(name+'-preflight.stderr')).open('w') as err:subprocess.run([str(R/'bin'/name),'--test'],cwd=ROOT,env=ENV,stdout=out,stderr=err,check=True)
subprocess.run([str(R/'bin/infra-font-followup'),'--check'],cwd=ROOT,env=ENV,stdout=(R/'font-fixture-controls.json').open('w'),check=True)
print('All optimized executables prepared; no samples collected.',flush=True)
