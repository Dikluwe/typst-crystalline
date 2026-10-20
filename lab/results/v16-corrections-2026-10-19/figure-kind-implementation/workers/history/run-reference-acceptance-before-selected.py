"""Run every frozen rich-reference corpus with exact build/input binding."""
from pathlib import Path
import argparse,importlib.util,json,subprocess,hashlib
ROOT=Path.cwd()
HERE=Path(__file__).resolve().parent
BASE=HERE/'agents/entities/reference-acceptance'
p=argparse.ArgumentParser()
p.add_argument('--build',required=True)
p.add_argument('--output',required=True)
a=p.parse_args()
out=Path(a.output);out.mkdir(exist_ok=False)
spec=importlib.util.spec_from_file_location('validation',ROOT/'lab/tools/ci/validate_layout_migration.py')
v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
product=ROOT/'lab/target/product/debug/typst'
build=Path(a.build);b=json.loads(build.read_text());before=v.frozen_inputs()
assert b['inputs']==before and b['product_sha256']==sha(product)
r={'utc_start':v.utc(),'inputs':before,'build_receipt':str(build),'build_sha256':sha(build),'product_sha256':sha(product),'commands':[]}
for prefix,policy in [('', 'render-order-policies-v2.json'),('additional-', 'additional-render-order-policies-v2.json'),('targeted-', 'targeted-render-order-policies-v2.json'),('origin-', 'origin-policies.json'),('origin-edge-', 'origin-edge-policies.json')]:
 name=prefix.rstrip('-') or 'main'
 cmd=['python3',str(BASE/'run_product.py'),'--product',str(product),'--build-receipt',str(build),'--out',str(out/name),'--matrix',prefix+'matrix.json','--oracle-receipt',prefix+'oracle/receipt.json','--policies',policy]
 start=v.utc();run=subprocess.run(cmd)
 r['commands'].append({'argv':cmd,'utc_start':start,'utc_end':v.utc(),'exit':run.returncode})
 if run.returncode:break
 r['commands'][-1]['required_failures']=json.loads((out/name/'receipt.json').read_text())['required_failures']
after=v.frozen_inputs();r.update(utc_end=v.utc(),changed_inputs=[p for p in before.keys()|after.keys() if before.get(p)!=after.get(p)],binary_unchanged=sha(product)==b['product_sha256'])
(out/'receipt.json').write_text(json.dumps(r,indent=2)+'\n')
assert not r['changed_inputs'] and r['binary_unchanged']
assert len(r['commands'])==5 and all(c['exit']==0 and not c['required_failures'] for c in r['commands'])
