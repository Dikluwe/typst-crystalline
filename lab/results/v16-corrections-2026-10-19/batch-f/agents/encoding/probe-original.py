#!/usr/bin/env python3
import datetime,hashlib,json,pathlib,subprocess,sys
ROOT=pathlib.Path(__file__).resolve().parents[6]
HERE=pathlib.Path(__file__).resolve().parent
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
out=HERE/sys.argv[1]
assert not out.exists()
receipt=json.loads((ROOT/'lab/results/v16-corrections-2026-10-19/batch-f/baseline-build-receipt.json').read_text())
product=ROOT/'lab/target/product/debug/typst';oracle=ROOT/'lab/typst-original/target/release/typst'
assert sha(oracle)=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
r={'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'diff_head_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True),'baseline_build_receipt':receipt,'runner_sha256':sha(pathlib.Path(__file__)),'case_manifest_sha256':sha(HERE/'probe-cases.json'),'binaries':{k:{'path':str(p),'sha256':sha(p)} for k,p in [('product',product),('oracle',oracle)]},'cases':[]}
for c in json.loads((HERE/'probe-cases.json').read_text())['cases']:
 item=dict(c,results={})
 for name,binary in [('product',product),('oracle',oracle)]:
  argv=[str(binary),'eval',c['expression'],'--format','json'] + (['--features','html'] if c['name'].startswith('html-') else []);start=utc();p=subprocess.run(argv,cwd=ROOT,capture_output=True,text=True,timeout=20)
  item['results'][name]={'argv':argv,'returncode':p.returncode,'stdout':p.stdout,'stderr':p.stderr,'utc_start':start,'utc_end':utc()}
 r['cases'].append(item)
r['utc_end']=utc();r['binary_sha256_after']={k:sha(p) for k,p in [('product',product),('oracle',oracle)]};out.write_text(json.dumps(r,indent=2)+'\n'); print(out)
