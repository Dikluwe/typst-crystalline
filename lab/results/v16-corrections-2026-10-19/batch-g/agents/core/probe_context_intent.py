from pathlib import Path
import json,subprocess,hashlib,datetime
P=Path('lab/results/v16-corrections-2026-10-19/batch-g/agents/core/context-intent-probes');P.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
bins={'oracle':Path('lab/typst-original/target/release/typst').resolve(),'product':Path('lab/target/product/debug/typst').resolve()}
cases={
'parenthesized-distinct':'{ let a = (context [A]); let b = (context [B]); (equal: a == b, left: repr(a), right: repr(b), left_fields: a.fields(), right_fields: b.fields(), left_type: repr(type(a)), right_type: repr(type(b))) }',
'opaque-unexecuted-panic':'{let a = (context panic("must-not-run-A")); let b = (context panic("must-not-run-B")); (equal: a == b, repr: repr(a), fields: a.fields())}',
'distinct-captured-environment':'{let f(x) = context x; let a=f("A");let b=f("B");(equal:a==b,not_equal:a!=b)}',
'context-vs-body':'{let a=(context [A]);(equal:a==[A],reverse:[A]==a,empty:a==[])}'}
r={'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'binary_sha256':{n:sha(p) for n,p in bins.items()},'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'note':'Historical verified build4 product, exact unchanged executable; current pending source changes are tests/L0 only. No claim rebuilt product.','cases':[]}
for name,expr in cases.items():
 p=P/(name+'.expr');p.write_text(expr);x={'name':name,'source_sha256':sha(p),'expression':expr,'runs':{}}
 for n,b in bins.items():
  argv=[str(b),'eval',expr,'--format','json'];a=subprocess.run(argv,capture_output=True,text=True,timeout=20);x['runs'][n]={'argv':argv,'exit':a.returncode,'stdout':a.stdout,'stderr':a.stderr}
 r['cases'].append(x)
p=P/'distinct-rendered.typ';p.write_text('#let a = (context [A])\n#let b = (context [B])\n#metadata((equal: a == b, first: a.fields(), second: b.fields())) <probe>\n#a #b\n')
x={'name':'distinct-rendered','source_sha256':sha(p),'source':str(p),'runs':{}}
for n,b in bins.items():
 out=P/(n+'.pdf');argv=[str(b),'compile',str(p),str(out)];a=subprocess.run(argv,capture_output=True,text=True,timeout=20);x['runs'][n]={'compile':{'argv':argv,'exit':a.returncode,'stdout':a.stdout,'stderr':a.stderr}}
 argv=[str(b),'query',str(p),'<probe>','--field','value'];a=subprocess.run(argv,capture_output=True,text=True,timeout=20);x['runs'][n]['query']={'argv':argv,'exit':a.returncode,'stdout':a.stdout,'stderr':a.stderr}
 if out.exists():x['runs'][n]['text']=subprocess.run(['pdftotext','-layout',str(out),'-'],capture_output=True,text=True).stdout;x['runs'][n]['pdf_sha256']=sha(out)
r['cases'].append(x);r['utc_end']=utc();r['binary_unchanged']=all(sha(p)==r['binary_sha256'][n] for n,p in bins.items());(P/'receipt.json').write_text(json.dumps(r,indent=2)+'\n')
print(json.dumps(r['cases'],indent=2));print('sha',sha(P/'receipt.json'))
