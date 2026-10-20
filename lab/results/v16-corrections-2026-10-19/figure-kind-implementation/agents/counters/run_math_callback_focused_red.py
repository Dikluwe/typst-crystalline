from pathlib import Path
import subprocess,os,importlib.util,json,gzip
root=Path.cwd();base=root/'lab/results/v16-corrections-2026-10-19/figure-kind-implementation/agents/counters'
out=base/'math-callback-focused-red';out.mkdir()
spec=importlib.util.spec_from_file_location('validation',root/'lab/tools/ci/validate_layout_migration.py');v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)
inputs=v.frozen_inputs();record={'utc_start':v.utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'state':'uncommitted working tree','diff_head_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'inputs':inputs,'worker_sha256':v.digest(Path(__file__)),'resource_limits':{'jobs':1,'memory_max':'12G','memory_high':'8G','swap_max':0},'runs':[]}
env=dict(os.environ,CARGO_BUILD_JOBS='1',CARGO_PROFILE_DEV_DEBUG='0',CARGO_PROFILE_TEST_DEBUG='0',CARGO_INCREMENTAL='0',TYPST_VANILLA=str(root/'lab/typst-original/target/release/typst'))
for target,filter_ in [('math_cancel_callback_runtime','p1291_'),('math_decoration_and_float_flush','p1292_a_cancel_callback_cross_context_span_and_background')]:
 command=['cargo','test','--offline','--locked','-p','typst-wiring','--test',target,filter_,'--','--test-threads=1']
 log=out/(target+'.log');start=v.utc()
 with log.open('w') as f:r=subprocess.run(command,env=env,stdout=f,stderr=subprocess.STDOUT)
 text=log.read_text();record['runs'].append({'command':command,'utc_start':start,'utc_end':v.utc(),'exit':r.returncode,'log':str(log.relative_to(root)),'log_sha256':v.digest(log),'semantic_red':r.returncode==101 and 'test result: FAILED.' in text and 'error[E' not in text})
 print(text[-7000:],flush=True)
after=v.frozen_inputs();record.update(utc_end=v.utc(),changed_inputs=[p for p in inputs.keys()|after.keys() if inputs.get(p)!=after.get(p)])
with gzip.open(out/'state.json.gz','wt') as f:json.dump(record,f,indent=2)
assert not record['changed_inputs'];assert all(x['semantic_red'] for x in record['runs'])
