"""Record actual focused semantic RED/GREEN without accepting compiler failures."""
from pathlib import Path
import argparse,importlib.util,json,os,re,subprocess
ROOT=Path.cwd();P=Path(__file__).resolve().parent
parser=argparse.ArgumentParser();parser.add_argument('--output',required=True);parser.add_argument('--package',required=True);parser.add_argument('--filter',default='batch_g_');parser.add_argument('--expected-fail',action='append',default=[]);parser.add_argument('--expected-total',type=int,required=True)
a=parser.parse_args();out=P/a.output;assert not out.exists();out.mkdir()
spec=importlib.util.spec_from_file_location('validation',ROOT/'lab/tools/ci/validate_layout_migration.py');v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)
inputs=v.frozen_inputs();env=dict(os.environ,CARGO_BUILD_JOBS='1',CARGO_PROFILE_DEV_DEBUG='0',CARGO_PROFILE_TEST_DEBUG='0',CARGO_INCREMENTAL='0',TYPST_VANILLA=str(ROOT/'lab/typst-original/target/release/typst'))
argv=['cargo','test','--offline','--locked','-p',a.package,'--lib',a.filter,'--','--test-threads=1']
r={'utc_start':v.utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diff_head_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'inputs':inputs,'argv':argv,'environment':{k:env[k] for k in ['CARGO_BUILD_JOBS','CARGO_PROFILE_DEV_DEBUG','CARGO_PROFILE_TEST_DEBUG','CARGO_INCREMENTAL','TYPST_VANILLA']},'worker_sha256':v.digest(Path(__file__)),'expected_failure_names':a.expected_fail,'expected_total':a.expected_total}
with (out/'tests.log').open('w') as log:run=subprocess.run(argv,env=env,stdout=log,stderr=subprocess.STDOUT)
log=(out/'tests.log').read_text();results=re.findall(r'^test (\S+) \.\.\. (ok|FAILED|ignored)$',log,re.M)
failed=[name for name,status in results if status=='FAILED'];after=v.frozen_inputs()
r.update(utc_end=v.utc(),exit=run.returncode,tests=[{'name':name,'status':status} for name,status in results],failed=failed,compiler_error_codes=sorted(set(re.findall(r'error\[(E\d+)\]',log))),changed_inputs=sorted(n for n in inputs.keys()|after.keys() if inputs.get(n)!=after.get(n)))
(out/'receipt.json').write_text(json.dumps(r,indent=2)+'\n')
print(json.dumps({k:r[k] for k in ['exit','failed','compiler_error_codes','changed_inputs']},indent=2),flush=True)
assert not r['compiler_error_codes'] and 'could not compile' not in log
assert not r['changed_inputs'] and len(results)==a.expected_total and all(status!='ignored' for _,status in results)
assert {name.split('::')[-1] for name in failed}==set(a.expected_fail)
assert run.returncode==(101 if failed else 0)
print('Actual focused',('RED' if failed else 'GREEN'),'recorded:',len(results),'tests',flush=True)
