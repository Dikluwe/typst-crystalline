from pathlib import Path
import subprocess,os,importlib.util,json,gzip
root=Path.cwd();base=root/'lab/results/v16-corrections-2026-10-19/figure-kind-implementation';out=base/'agents/counters/math-callback-focused-green';out.mkdir()
spec=importlib.util.spec_from_file_location('validation',root/'lab/tools/ci/validate_layout_migration.py');v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)
inputs=v.frozen_inputs();record={'utc_start':v.utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'state':'uncommitted working tree','diff_head_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'inputs':inputs,'worker_sha256':v.digest(Path(__file__)),'resource_limits':{'jobs':1,'memory_max':'12G','memory_high':'8G','swap_max':0},'runs':[]}
env=dict(os.environ,CARGO_BUILD_JOBS='1',CARGO_PROFILE_DEV_DEBUG='0',CARGO_PROFILE_TEST_DEBUG='0',CARGO_INCREMENTAL='0',TYPST_VANILLA=str(root/'lab/typst-original/target/release/typst'))
for target in ['math_cancel_callback_runtime','math_decoration_and_float_flush']:
 command=['cargo','test','--offline','--locked','-p','typst-wiring','--test',target,'--','--test-threads=1'];log=out/(target+'.log');start=v.utc()
 with log.open('w') as f:r=subprocess.run(command,env=env,stdout=f,stderr=subprocess.STDOUT)
 text=log.read_text();record['runs'].append({'command':command,'utc_start':start,'utc_end':v.utc(),'exit':r.returncode,'log':str(log.relative_to(root)),'log_sha256':v.digest(log),'green':r.returncode==0 and 'test result: ok.' in text})
 print(text[-5000:],flush=True)
 if r.returncode:break
if all(x['green'] for x in record['runs']) and len(record['runs'])==2:
 command=['python3',str(base/'build.py'),'--output',str(base/'math-callback-build-1')];start=v.utc();r=subprocess.run(command,env=env);record['build']={'command':command,'utc_start':start,'utc_end':v.utc(),'exit':r.returncode}
after=v.frozen_inputs();record.update(utc_end=v.utc(),changed_inputs=[p for p in inputs.keys()|after.keys() if inputs.get(p)!=after.get(p)])
with gzip.open(out/'state.json.gz','wt') as f:json.dump(record,f,indent=2)
assert not record['changed_inputs'];assert len(record['runs'])==2 and all(x['green'] for x in record['runs']);assert record['build']['exit']==0
