"""Identify the exact public-preparation fixture failures with actual binaries."""
from pathlib import Path
import datetime,hashlib,importlib.util,json,re,subprocess
P=Path(__file__).resolve().parent;R=Path.cwd();out=P/'bundle-diagnostic';out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
spec=importlib.util.spec_from_file_location('validation',R/'lab/tools/ci/validate_layout_migration.py');v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)
build=P/'candidate-diagnostic-build/receipt.json';b=json.loads(build.read_text());assert b['inputs']==v.frozen_inputs() and b['product_sha256']==sha(R/'lab/target/product/debug/typst')
src=R/'03_infra/src/pipeline/bundle.rs';text=src.read_text();f=text.split('fn batch_g_html_bundle_public_preparation_projects_resolved_and_standalone_cells()',1)[1].split('let world=',1)[0]
qs=re.findall(r'\(\s*("(?:[^"\\]|\\.)*")\s*,\s*("(?:[^"\\]|\\.)*")\s*\)',f);assert len(qs)==4
oracle=R/'lab/typst-original/target/release/typst';assert sha(oracle)=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
r={'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diff_head_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'inputs':b['inputs'],'build_receipt_sha256':sha(build),'product_sha256':b['product_sha256'],'test_source':str(src),'test_source_sha256':sha(src),'fixture_block':f,'cases':[]}
for i,(s,e) in enumerate(qs):
 source=out/(str(i)+'.typ');source.write_text(json.loads(s));row={'index':i,'source':str(source),'source_sha256':sha(source),'expected_fragment':json.loads(e),'runs':{}}
 for name,binary in [('oracle',oracle),('product',R/'lab/target/product/debug/typst')]:
  dest=out/(str(i)+'-'+name);cmd=[str(binary),'compile','--features','html,bundle','--format','bundle',str(source),str(dest)];start=utc();p=subprocess.run(cmd,capture_output=True,text=True,timeout=60)
  html={str(h.relative_to(dest)):h.read_text() for h in dest.rglob('*.html')} if dest.exists() else {}
  row['runs'][name]={'argv':cmd,'utc_start':start,'utc_end':utc(),'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr,'html':html}
 r['cases'].append(row);print(i,row['runs']['oracle']['exit'],row['runs']['product']['exit'],flush=True)
r.update(utc_end=utc(),changed_inputs=[n for n,d in b['inputs'].items() if sha(R/n)!=d],binary_unchanged=sha(R/'lab/target/product/debug/typst')==b['product_sha256']);(out/'receipt.json').write_text(json.dumps(r,indent=2)+'\n');assert not r['changed_inputs'] and r['binary_unchanged']
