import datetime,hashlib,json,pathlib,subprocess
R=pathlib.Path('/repos/Antigravity/typst-crystalline');H=pathlib.Path(__file__).resolve().parent
utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest();git=lambda *a:subprocess.check_output(['git',*a],cwd=R,text=True)
build=H.parents[1]/'baseline-build-receipt.json';b=json.loads(build.read_text());bins={k:pathlib.Path(b[k]['path']) for k in ['product','oracle']}
assert sha(bins['product'])==b['product']['sha256']=='44178e4fecc5c66cba3aa134f4d82290c6d53fb340dd5966b67eec45b737c914'
assert sha(bins['oracle'])==b['oracle']['sha256']=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
cases={'decimal-arity':'decimal("2", "1")','array-invalid':'array(1)','decimal-invalid':'decimal("bad")','selector-invalid':'selector(1)','spot-shortcut':'color.spot(red) == color.spot(red)','spot-lazy':'().map(x => color.spot(x)) == ()','by-version':'(2,1).sorted(by: (a,b) => version(a,b))','by-arguments':'(2,1).sorted(by: (a,b) => arguments(a,b))'}
p=H/'native-controls-baseline.json';assert not p.exists()
r=dict(utc_start=utc(),head=git('rev-parse','HEAD').strip(),status=git('status','--short'),diff_head_stat=git('diff','HEAD','--stat'),build_receipt=dict(path=str(build),sha256=sha(build)),runner_sha256=sha(__file__),binaries={k:dict(path=str(v),sha256=sha(v)) for k,v in bins.items()},cases=[])
for name,expr in cases.items():
 c=dict(name=name,expression=expr,expression_sha256=hashlib.sha256(expr.encode()).hexdigest(),results={})
 for k,v in bins.items():
  argv=[str(v),'eval',expr,'--format','json'];t=utc();x=subprocess.run(argv,cwd=R,text=True,capture_output=True,timeout=30);c['results'][k]=dict(argv=argv,utc_start=t,utc_end=utc(),returncode=x.returncode,stdout=x.stdout,stderr=x.stderr)
 r['cases'].append(c);p.write_text(json.dumps(r,indent=2));print(name,{k:(v['returncode'],v['stdout'],v['stderr']) for k,v in c['results'].items()},flush=True)
r.update(utc_end=utc(),binaries_after={k:sha(v) for k,v in bins.items()},diff_head_stat_after=git('diff','HEAD','--stat'));p.write_text(json.dumps(r,indent=2));assert all(r['binaries'][k]['sha256']==v for k,v in r['binaries_after'].items())
