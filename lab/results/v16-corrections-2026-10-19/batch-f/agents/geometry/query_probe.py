import argparse,datetime,hashlib,json,pathlib,subprocess
ROOT=pathlib.Path('/repos/Antigravity/typst-crystalline');p=argparse.ArgumentParser();p.add_argument('--output',required=True);p.add_argument('--product');a=p.parse_args();out=ROOT/a.output;out.mkdir(exist_ok=False,parents=True)
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest();utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();git=lambda *args:subprocess.check_output(['git',*args],cwd=ROOT,text=True)
binaries={'oracle':ROOT/'lab/typst-original/target/release/typst'}
if a.product:binaries['product']=ROOT/a.product
assert sha(binaries['oracle'])=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
cases=[]
for i,body in enumerate(['table([A])','raw("X")','[pre #table([A])]','strong(table([A]))','emph(raw("X"))','hide(table([A]))','box(table([A]))','block(table([A]))','figure(table([A]))','[plain]']):
 source=f'#show figure: it => [#metadata((kind: repr(it.kind), kind-type: repr(type(it.kind)))) <probe>]\n#figure({body})\n'
 cases.append({'name':f'figure-synth-{i:02}','source':source})
for i,radius in enumerate(['2pt','2em','auto']):
 for size in ['10pt','20pt']:
  source=f'#set text(size: {size})\n#context [#metadata(measure(circle(radius: {radius}))) <probe>]\n'
  cases.append({'name':f'circle-geometry-{i}-{size}','source':source})
record={'utc_start':utc(),'head':git('rev-parse','HEAD').strip(),'diff_head_stat':git('diff','HEAD','--stat'),'runner_sha256':sha(__file__),'binaries':{k:{'path':str(v),'sha256':sha(v)} for k,v in binaries.items()},'cases':[]}
for case in cases:
 src=out/(case['name']+'.typ');src.write_text(case['source']);case.update(source_sha256=sha(src),utc_start=utc(),results={})
 for label,binary in binaries.items():
  argv=[str(binary),'query',str(src),'<probe>','--field','value'];r=subprocess.run(argv,cwd=ROOT,text=True,capture_output=True,timeout=30)
  case['results'][label]={'argv':argv,'returncode':r.returncode,'stdout':r.stdout,'stderr':r.stderr}
 record['cases'].append(case)
record.update(utc_end=utc(),diff_head_stat_after=git('diff','HEAD','--stat'),binary_sha256_after={k:sha(v) for k,v in binaries.items()});(out/'receipt.json').write_text(json.dumps(record,indent=2))
for c in record['cases']:print(c['name'],[(k,v['returncode'],v['stdout'].strip() or v['stderr'].splitlines()[0]) for k,v in c['results'].items()])
