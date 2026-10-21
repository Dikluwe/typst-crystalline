from pathlib import Path
import json,subprocess,hashlib,datetime,gzip
P=Path('lab/results/v16-corrections-2026-10-19/batch-g/agents/core/equality-probes')
P.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
binaries={'oracle':Path('lab/typst-original/target/release/typst').resolve(),'product':Path('lab/target/product/debug/typst').resolve()}
assert sha(binaries['oracle'])=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
build=Path('lab/results/v16-corrections-2026-10-19/figure-kind-implementation/candidate-build-4/receipt.json');b=json.loads(build.read_text())
assert sha(binaries['product'])==b['product_sha256']
pre={str(p):sha(p) for layer in ['01_core','02_shell','03_infra','04_wiring'] for p in Path(layer).rglob('*.rs')}
cases=[]
def pair(name,a,b,feature=False):
 expr='{ let a = '+a+'; let b = '+b+'; (equal: a == b, unequal: a != b, member: a in (b,), nested: (value: (a,)) == (value: (b,)), left: repr(a), right: repr(b)) }'
 cases.append({'name':name,'expression':expr,'features':['html'] if feature else []})
pair('title-alias','title([A])','a')
pair('title-equal','title([A])','title([A])')
pair('title-different','title([A])','title([B])')
pair('title-nested-sequence','[X#title([A])]','[X#title([A])]')
pair('title-nested-block','block(title([A]))','block(title([A]))')
pair('html-alias','html.elem("div", [A])','a',True)
pair('html-same','html.elem("div", [A])','html.elem("div", [A])',True)
pair('html-tag','html.elem("div", [A])','html.elem("span", [A])',True)
pair('html-body','html.elem("div", [A])','html.elem("div", [B])',True)
pair('html-unset-empty','html.elem("div")','html.elem("div", [])',True)
pair('html-unset-none','html.elem("div")','html.div()',True)
pair('html-none-empty','html.div()','html.div([])',True)
pair('html-attrs-absent-empty','html.elem("div", [A])','html.elem("div", [A], attrs: (:))',True)
pair('html-attrs-same','html.elem("div", [A], attrs: (id: "a", class: "b"))','html.elem("div", [A], attrs: (id: "a", class: "b"))',True)
pair('html-attrs-order','html.elem("div", [A], attrs: (id: "a", class: "b"))','html.elem("div", [A], attrs: (class: "b", id: "a"))',True)
pair('html-attrs-different','html.elem("div", [A], attrs: (id: "a"))','html.elem("div", [A], attrs: (id: "b"))',True)
left='html.elem("div", [A], attrs: (id: "a", class: "b"))';right='html.elem("div", [A], attrs: (class: "b", id: "a"))'
for name,wrap in [('sequence','[X#{x}]'),('title','title({x})'),('block','block({x})'),('frame','html.frame({x})')]:pair('html-attrs-order-'+name,wrap.format(x=left),wrap.format(x=right),True)
pair('html-frame-alias','html.frame([A])','a',True)
pair('html-frame-same','html.frame([A])','html.frame([A])',True)
pair('html-frame-different','html.frame([A])','html.frame([B])',True)
pair('context-alias','context [A]','a')
pair('context-separate-same','context [A]','context [A]')
pair('context-separate-different','context [A]','context [B]')
pair('context-nested-sequence','{let c = context [A]; [X#c]}','a')
for name,expr in [('html-tristate-fields','{let a=html.elem("div");let b=html.div();let c=html.div([]);(a: a.fields(),b: b.fields(),c: c.fields())}'),('title-fields','title([A]).fields()')]:cases.append({'name':name,'expression':expr,'features':['html'] if name.startswith('html') else []})
(P/'matrix.json').write_text(json.dumps({'cases':cases},indent=2)+'\n')
receipt={'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diff_stat_before':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'build_receipt':str(build),'build_sha256':sha(build),'binary_sha256':{n:sha(p) for n,p in binaries.items()},'matrix_sha256':sha(P/'matrix.json'),'source_before':pre,'oracle_pin':{'commit':'bd5ebec285505889e8ff1d4f68804ab147a699f3','tree':'cac836c07e4dec2a89219370428292393a3216b5'},'cases':[]}
for c in cases:
 path=P/(c['name']+'.expr');path.write_text(c['expression'])
 row={'name':c['name'],'source_sha256':sha(path),'source':str(path),'runs':{}}
 for n,binary in binaries.items():
  argv=[str(binary),'eval',c['expression'],'--format','json']
  if c['features']:argv+=['--features',','.join(c['features'])]
  start=utc()
  try:
   r=subprocess.run(argv,capture_output=True,text=True,timeout=20)
   result={'exit':r.returncode,'stdout':r.stdout,'stderr':r.stderr}
   try:result['value']=json.loads(r.stdout)
   except ValueError:pass
  except subprocess.TimeoutExpired as e:result={'timeout':True,'stdout':str(e.stdout),'stderr':str(e.stderr)}
  row['runs'][n]={'utc_start':start,'utc_end':utc(),'argv':argv,**result}
 receipt['cases'].append(row)
 print(c['name'],[(n,r.get('exit'),r.get('value')) for n,r in row['runs'].items()],flush=True)
receipt['utc_end']=utc();receipt['changed_source_inputs']=[n for n,d in pre.items() if sha(n)!=d];receipt['binary_unchanged']=all(sha(p)==receipt['binary_sha256'][n] for n,p in binaries.items());receipt['diff_stat_after']=subprocess.check_output(['git','diff','HEAD','--stat'],text=True)
(P/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print('RECEIPT',sha(P/'receipt.json'),'source drift',receipt['changed_source_inputs'])
