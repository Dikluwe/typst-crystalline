import datetime,hashlib,json,pathlib,subprocess,sys
R=pathlib.Path('/repos/Antigravity/typst-crystalline');H=pathlib.Path(__file__).resolve().parent
utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
git=lambda *a:subprocess.check_output(['git',*a],cwd=R,text=True)
expressions={
'spot-positive':'(red,).map(color.spot).first() == color.spot(red)',
'decimal-semantic':'("2.5",).map(decimal).first() == decimal("2.5")',
'regex-semantic':'("a",).map(regex).first() == regex("a")',
'selector-semantic':'(heading,).map(selector).first() == selector(heading)',
'stroke-semantic':'(red,).map(stroke).first() == stroke(red)',
'array-semantic':'(bytes((0,255)),).map(array).first() == (0,255)',
'arguments-semantic':'(1,).map(arguments).first().pos() == (1,)',
'tiling-type':'type(([x],).map(tiling).first()) == tiling',
'version-semantic':'(2,).map(version).first() == version(2)',
'label-semantic':'("x",).map(label).first() == <x>',
'duration-lazy':'().map(duration) == ()',
'by-error':'(2,1).sorted(by: version)',
'duration-native-error':'(1,).map(duration)',
'array-native-error':'(1,).map(array)',
'decimal-native-error':'("bad",).map(decimal)',
'selector-native-error':'(1,).map(selector)',
'spot-native-error':'("x",).map(color.spot)',
'arguments-by-error':'(2,1).sorted(by: arguments)',
'dictionary-direct-module':'dictionary(math).keys().contains("equation")',
'path-map-context':'("x",).map(path)',
}
bins={'oracle':R/'lab/typst-original/target/release/typst','product':R/'lab/target/product/debug/typst'}
p=H/sys.argv[1]
if p.exists():raise SystemExit('will not overwrite')
r=dict(utc_start=utc(),head=git('rev-parse','HEAD').strip(),status=git('status','--short'),diff_head_stat=git('diff','HEAD','--stat'),runner_sha256=sha(__file__),binaries={k:dict(path=str(v),sha256=sha(v)) for k,v in bins.items()},cases=[])
for name,expr in expressions.items():
 c=dict(name=name,expression=expr,results={})
 for k,b in bins.items():
  argv=[str(b),'eval',expr,'--format','json'];t=utc();x=subprocess.run(argv,cwd=R,text=True,capture_output=True,timeout=30);c['results'][k]=dict(argv=argv,utc_start=t,utc_end=utc(),returncode=x.returncode,stdout=x.stdout,stderr=x.stderr)
 r['cases'].append(c);p.write_text(json.dumps(r,indent=2));print(name,{k:(v['returncode'],v['stdout'],v['stderr'].splitlines()[:1]) for k,v in c['results'].items()})
r.update(utc_end=utc(),binaries_after={k:sha(v) for k,v in bins.items()},diff_head_stat_after=git('diff','HEAD','--stat'));p.write_text(json.dumps(r,indent=2))
