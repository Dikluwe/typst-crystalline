"""Build/bind one actual-root artifact, then run the public invariant probe."""
from pathlib import Path
import datetime,hashlib,importlib.util,json,os,subprocess
P=Path(__file__).resolve().parent;R=Path.cwd().resolve();out=P/'hash-artifact-before';out.mkdir(exist_ok=False)
spec=importlib.util.spec_from_file_location('validation',R/'lab/tools/ci/validate_layout_migration.py');v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)
b=json.loads((P/'candidate-build/receipt.json').read_text());assert b['inputs']==v.frozen_inputs();assert b['product_sha256']==v.digest(R/'lab/target/product/debug/typst')
env=dict(os.environ,CARGO_BUILD_JOBS='1',CARGO_PROFILE_DEV_DEBUG='0',CARGO_PROFILE_TEST_DEBUG='0',CARGO_INCREMENTAL='0')
cmd=['cargo','build','--offline','--locked','-p','typst-core','--lib','--message-format=json'];start=v.utc()
with (out/'cargo.jsonl').open('w') as log:r=subprocess.run(cmd,env=env,stdout=log,stderr=subprocess.STDOUT)
choices=[]
for line in (out/'cargo.jsonl').read_text().splitlines():
 try:a=json.loads(line)
 except ValueError:continue
 if a.get('reason')=='compiler-artifact' and a['target']['name']=='typst_core' and 'lib' in a['target']['kind']:
  assert Path(a['target']['src_path']).resolve()==R/'01_core/src/lib.rs'
  choices.extend(Path(n) for n in a['filenames'] if n.endswith('.rlib'))
receipt={'utc_start':start,'utc_end':v.utc(),'argv':cmd,'exit':r.returncode,'inputs':b['inputs'],'artifacts':[str(p) for p in choices],'changed_inputs':[n for n,d in b['inputs'].items() if v.digest(R/n)!=d]};(out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');assert r.returncode==0 and len(choices)==1 and not receipt['changed_inputs']
raise SystemExit(subprocess.call(['python3',str(P/'agents/core/run_public_hash_probe.py'),'--build-receipt',str(P/'candidate-build/receipt.json'),'--rlib',str(choices[0]),'--out',str(P/'hash-before')]))
