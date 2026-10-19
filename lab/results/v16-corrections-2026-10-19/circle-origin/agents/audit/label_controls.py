import datetime,json,hashlib,pathlib,subprocess
R=pathlib.Path('/repos/Antigravity/typst-crystalline');H=pathlib.Path(__file__).resolve().parent;out=H/'label-selector-controls';assert not out.exists();out.mkdir();sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();git=lambda *a:subprocess.check_output(['git',*a],cwd=R,text=True)
P=R/'lab/target/product/debug/typst';O=R/'lab/typst-original/target/release/typst';B=R/'lab/results/v16-corrections-2026-10-19/circle-origin/candidate-build-receipt.json';assert sha(P)in B.read_text()
cases=[('repeated-label-same-element','#metadata(40) <probe> <probe>\n','<probe>'),('equal-values-distinct-elements','#metadata(40) <probe>\n#metadata(40) <probe>\n','<probe>'),('interleaved-source-order','#metadata(30) <probe>\n#metadata(1) <other>\n#metadata(20) <probe>\n#metadata(10) <probe>\n','<probe>'),('duplicate-label-or','#metadata(40) <probe>\n#metadata(80) <probe>\n','selector(<probe>).or(<probe>)'),('duplicate-label-and','#metadata(40) <probe>\n#metadata(80) <probe>\n','selector(<probe>).and(metadata)'),('singlelookup-locate','#metadata(40) <probe>\n#metadata(80) <probe>\n#context [#metadata(locate(<probe>)) <result>]\n','<result>')]
rec={'utc_start':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'diff_head_stat':git('diff','HEAD','--stat'),'binaries':{'product':{'path':str(P),'sha256':sha(P)},'oracle':{'path':str(O),'sha256':sha(O)}},'build_receipt_sha256':sha(B),'cases':[]}
for name,source,sel in cases:
 p=out/(name+'.typ');p.write_text(source);c={'name':name,'source':source,'source_sha256':sha(p),'results':{}}
 for label,binary in [('product',P),('oracle',O)]:
  argv=[str(binary),'query',str(p),sel,'--field','value'];s=subprocess.run(argv,cwd=R,capture_output=True,text=True,timeout=30);c['results'][label]={'argv':argv,'returncode':s.returncode,'stdout':s.stdout,'stderr':s.stderr}
 rec['cases'].append(c)
rec.update(utc_end=datetime.datetime.now(datetime.timezone.utc).isoformat(),diff_head_stat_after=git('diff','HEAD','--stat'),binary_sha256_after={'product':sha(P),'oracle':sha(O)});(out/'receipt.json').write_text(json.dumps(rec,indent=2)+'\n')
for c in rec['cases']:print(c['name'],[(k,v['returncode'],v['stdout'].strip(),v['stderr'].splitlines()[:1])for k,v in c['results'].items()])
