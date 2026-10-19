#!/usr/bin/env python3
import pathlib,json,hashlib,subprocess,datetime
D=pathlib.Path(__file__).resolve().parent;R=D.parents[5]
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
previous=json.loads((D/'boundaries.json').read_text())
cases={'supplement-and-numbering-invalid': '#figure([A],supplement:123,numbering:123)\n', 'numbering-and-supplement-invalid-reverse': '#figure([A],numbering:123,supplement:123)\n', 'kind-and-supplement-invalid': '#figure([A],supplement:123,kind:123)\n', 'supplement-str': '#figure([A],caption:[C],supplement:"Custom")\n', 'supplement-none': '#figure([A],caption:[C],supplement:none)\n', 'supplement-auto': '#figure([A],caption:[C],supplement:auto)\n', 'supplement-symbol': '#figure([A],caption:[C],supplement:sym.alpha)\n', 'numbering-function': '#figure([A],caption:[C],numbering:n=>[X])\n', 'numbering-none': '#figure([A],caption:[C],numbering:none)\n', 'numbering-integer': '#figure([A],caption:[C],numbering:123)\n', 'supplement-invalid-unknown': '#figure([A],supplement:123,unknown:123)\n', 'numbering-invalid-unknown': '#figure([A],numbering:123,unknown:123)\n'}

result={'utc_start':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'diff_head_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),'runner_sha256':sha(pathlib.Path(__file__)),'provenance_parent_sha256':sha(D/'boundaries.json'),'cases':[]}
for name,source in cases.items():
 p=D/'probes'/(name+'.typ');p.write_text(source);row={'name':name,'source':source,'source_sha256':sha(p),'runs':{}}
 for kind,rel in [('oracle','lab/typst-original/target/release/typst'),('product','lab/target/product/debug/typst')]:
  binary=R/rel; assert sha(binary)==previous[kind+'_sha256'];argv=[str(binary),'query',str(p),'<probe>','--field','value'];start=now();cp=subprocess.run(argv,cwd=R,capture_output=True,text=True,timeout=35);row['runs'][kind]={'utc_start':start,'utc_end':now(),'argv':argv,'returncode':cp.returncode,'stdout':cp.stdout,'stderr':cp.stderr}
 result['cases'].append(row)
result['utc_end']=now();(D/'constructor-validation.json').write_text(json.dumps(result,indent=2)+'\n')
for c in result['cases']:print(c['name'],[(k,r['returncode'],r['stdout'],r['stderr'].splitlines()[0]) for k,r in c['runs'].items()])
