import argparse,datetime,hashlib,json,pathlib,subprocess
ROOT=pathlib.Path('/repos/Antigravity/typst-crystalline')
HERE=pathlib.Path(__file__).resolve().parent
p=argparse.ArgumentParser();p.add_argument('--output',required=True);p.add_argument('--product');a=p.parse_args();out=pathlib.Path(a.output);out.mkdir(exist_ok=False,parents=True)
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
git=lambda *args:subprocess.check_output(['git',*args],cwd=ROOT,text=True)
oracle=ROOT/'lab/typst-original/target/release/typst'
assert sha(oracle)=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
binaries={'oracle':oracle}
if a.product:binaries['product']=ROOT/a.product
cases=[]
for native,arg in [('grid','columns'),('grid','rows'),('table','columns'),('circle','radius'),('place','dx'),('place','dy'),('line','dx'),('line','length')]:
 for value in ['none','auto','true','"bad"','"auto"','0','-1','1','1.5','0pt','-1pt','2pt','2em','20%','20% + 2pt','1fr','()','(1pt, true, 2pt)','(auto, 1fr, 2pt)']:
  expr=f'{native}({arg}: {value}'+(', [X])' if native in ['grid','table','place'] else ')')
  cases.append({'name':f'{native}-{arg}-{len(cases):03}','expression':f'{{ let x = {expr}; repr(x) }}'})
for body in ['table([A])','raw("X")','[pre #table([A])]','strong(table([A]))','emph(raw("X"))','hide(table([A]))','box(table([A]))','block(table([A]))','figure(table([A]))','[plain]']:
 cases.append({'name':f'figure-kind-{len(cases):03}','expression':f'figure({body}).kind'})
for value in ['table','raw','"table"','auto']:
 cases.append({'name':f'figure-explicit-{len(cases):03}','expression':f'figure([X], kind: {value}).kind'})
record={'utc_start':utc(),'head':git('rev-parse','HEAD').strip(),'diff_head_stat':git('diff','HEAD','--stat'),'status':git('status','--short'),'runner_sha256':sha(__file__),'binaries':{k:{'path':str(v),'sha256':sha(v)} for k,v in binaries.items()},'oracle_commit':'bd5ebec285505889e8ff1d4f68804ab147a699f3','cases':[]}
for case in cases:
 case=dict(case,utc_start=utc(),results={})
 for label,binary in binaries.items():
  argv=[str(binary),'eval',case['expression']];r=subprocess.run(argv,cwd=ROOT,text=True,capture_output=True,timeout=30)
  case['results'][label]={'argv':argv,'returncode':r.returncode,'stdout':r.stdout,'stderr':r.stderr}
 record['cases'].append(case)
record.update(utc_end=utc(),diff_head_stat_after=git('diff','HEAD','--stat'),binary_sha256_after={k:sha(v) for k,v in binaries.items()})
(out/'receipt.json').write_text(json.dumps(record,indent=2))
for c in record['cases']:
 print(c['name'],c['expression'],[(k,v['returncode'],v['stdout'].strip() or v['stderr'].splitlines()[0]) for k,v in c['results'].items()])
