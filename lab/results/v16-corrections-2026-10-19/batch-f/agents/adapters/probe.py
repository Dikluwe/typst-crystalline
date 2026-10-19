import argparse,datetime,hashlib,json,pathlib,subprocess
ROOT=pathlib.Path('/repos/Antigravity/typst-crystalline');HERE=pathlib.Path(__file__).resolve().parent
UTC=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(p):return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def git(*a):return subprocess.check_output(['git',*a],cwd=ROOT,text=True)
def run(argv):
 t=UTC();p=subprocess.run(argv,cwd=ROOT,capture_output=True,text=True,timeout=30)
 return dict(argv=argv,utc_start=t,utc_end=UTC(),returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
constructors={'int':'"2"','float':'"2.5"','str':'2','type':'2','counter':'"x"','state':'"x"','symbol':'"x"','bytes':'"x"','datetime':'2024','decimal':'"2.5"','duration':'1','regex':'"a"','selector':'heading','stroke':'red','tiling':'[x]','version':'1','label':'"x"','path':'"x"','array':'bytes("x")','arguments':'1','color.spot':'"x"','bool':'1','content':'[x]','length':'1','color':'red','location':'1','dictionary':'(:)'}
cases=[]
for typ,arg in constructors.items():
 cases += [(typ+'-direct',f'repr({typ}({arg}))'),(typ+'-map',f'({arg},).map({typ}).map(repr)')]
cases += [('decimal-key','("10", "2").sorted(key: decimal)'),('version-key','(10, 2).sorted(key: version)'),('label-filter','("x",).filter(label)'),('str-filter','(1,).filter(str)'),('decimal-by','("2", "1").sorted(by: decimal)'),('int-map-error','("bad",).map(int)'),('empty-decimal-map','().map(decimal)'),('empty-duration-map','().map(duration)'),('empty-arguments-map','().map(arguments)')]
for locale in [None,'none','"en"','"bad_locale"','true','3','[x]']:
 suffix='omitted' if locale is None else locale
 for src in ['"works.yml"','"missing.yml"']:
  cases.append(('locale-'+suffix+'-'+src,f'repr(bibliography({src}'+('' if locale is None else ', locale: '+locale)+'))'))
for expr in ['selector()','selector(heading, figure)','selector(1)','selector(regex("a"))','selector("")']:
 cases.append(('selector-'+expr,expr))
ap=argparse.ArgumentParser();ap.add_argument('--product');ap.add_argument('--output',required=True);a=ap.parse_args()
out=HERE/a.output
if out.exists():raise SystemExit('will not overwrite')
bins={'oracle':ROOT/'lab/typst-original/target/release/typst'}
if a.product:bins['product']=pathlib.Path(a.product).resolve()
assert sha(bins['oracle'])=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
(HERE/'works.yml').write_text('test:\n  type: Book\n  title: Test\n  author: Doe, Jane\n  date: 2020\n')
receipt=dict(utc_start=UTC(),head=git('rev-parse','HEAD').strip(),status=git('status','--short'),diff_head_stat=git('diff','HEAD','--stat'),runner_sha256=sha(__file__),oracle_pin=dict(commit='bd5ebec285505889e8ff1d4f68804ab147a699f3',tree='cac836c07e4dec2a89219370428292393a3216b5',recipe='lab/results/construction/diagnostics/vanilla-resync-2026-10-17/relatorio.md'),binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in bins.items()},cases=[])
for name,expr in cases:
 # File path is the exact local fixture in every run, not cwd-dependent.
 expr=expr.replace('"works.yml"',json.dumps(str((HERE/'works.yml').relative_to(ROOT)))).replace('"missing.yml"',json.dumps(str((HERE/'missing.yml').relative_to(ROOT))))
 c=dict(name=name,expression=expr,expression_sha256=hashlib.sha256(expr.encode()).hexdigest(),results={k:run([str(p),'eval',expr,'--format','json']) for k,p in bins.items()});receipt['cases'].append(c)
 out.write_text(json.dumps(receipt,indent=2));print(name,{k:(v['returncode'],v['stdout'].strip(),v['stderr'].splitlines()[:1]) for k,v in c['results'].items()},flush=True)
receipt.update(utc_end=UTC(),diff_head_stat_after=git('diff','HEAD','--stat'),binaries_after={k:sha(p) for k,p in bins.items()});out.write_text(json.dumps(receipt,indent=2))
