import pathlib,json,hashlib,datetime,subprocess,argparse,gzip
ROOT=pathlib.Path('/repos/Antigravity/typst-crystalline');HERE=pathlib.Path(__file__).resolve().parent
p=argparse.ArgumentParser();p.add_argument('--output',required=True);p.add_argument('--product',default='lab/target/product/debug/typst');p.add_argument('--baseline',action='store_true');p.add_argument('--validation-state');a=p.parse_args();out=ROOT/a.output;out.mkdir(exist_ok=False,parents=True)
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest();utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();git=lambda *a:subprocess.check_output(['git',*a],cwd=ROOT,text=True)
b={'oracle':ROOT/'lab/typst-original/target/release/typst','product':ROOT/a.product};receipt=HERE.parent.parent/'baseline-build-receipt.json';build=json.loads(receipt.read_text())
assert sha(b['oracle'])==build['oracle']['sha256']
if a.baseline:assert sha(b['product'])==build['product']['sha256']
state=None
if a.validation_state:
 sp=ROOT/a.validation_state
 with (gzip.open(sp,'rt') if sp.suffix=='.gz' else sp.open()) as sf: state=json.load(sf)
 assert state.get('inputs') and all((ROOT/p).is_file() and sha(ROOT/p)==h for p,h in state['inputs'].items())
expressions=['circle(radius:2pt)==circle(radius:2)','circle(radius:2pt)==circle(radius:2.0)','circle(radius:2pt)==ellipse(width:4pt,height:4pt)','circle(radius:2pt)==ellipse(width:4.0,height:4.0)','circle(radius:2em)==ellipse(width:4em,height:4em)','circle(radius:2em)==circle(radius:0pt)','circle(radius:2em)==circle(radius:1em)','circle(radius:2em)==circle(radius:2em)','circle(radius:2pt+0em)==circle(radius:2pt)','circle(radius:1em+2pt)==circle(radius:2pt)','circle(radius:1em+2pt)==ellipse(width:2em+4pt,height:2em+4pt)']
r={'utc_start':utc(),'head':git('rev-parse','HEAD').strip(),'diff_head_stat':git('diff','HEAD','--stat'),'baseline_build_receipt':{'path':str(receipt),'sha256':sha(receipt)},'baseline_binary_requested':a.baseline,'source_state_note':'Source working tree has candidate edits; baseline flag binds executable to separately frozen baseline build, not current edited files.','binaries':{k:{'path':str(v),'sha256':sha(v)} for k,v in b.items()},'cases':[]}
if state is not None:r['validation_state']={'path':str(sp),'sha256':sha(sp),'inputs':state['inputs']}
r['runner_sha256']=sha(__file__)
for i,expr in enumerate(expressions):
 c={'name':f'circle-equality-{i:02}','expression':expr,'results':{}}
 for label,binary in b.items():
  argv=[str(binary),'eval',expr];p=subprocess.run(argv,cwd=ROOT,text=True,capture_output=True,timeout=30);c['results'][label]={'argv':argv,'returncode':p.returncode,'stdout':p.stdout,'stderr':p.stderr}
 r['cases'].append(c)
if state is not None:r['changed_inputs']=[p for p,h in state['inputs'].items() if not (ROOT/p).is_file() or sha(ROOT/p)!=h]
r.update(utc_end=utc(),binary_sha256_after={k:sha(v) for k,v in b.items()});(out/'receipt.json').write_text(json.dumps(r,indent=2))
for c in r['cases']:print(c['name'],c['expression'],[(k,v['stdout'].strip() or v['stderr'].splitlines()[0]) for k,v in c['results'].items()])
