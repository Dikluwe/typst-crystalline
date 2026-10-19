#!/usr/bin/env python3
"""Independent serial language evidence, no productive writes or builds."""
import datetime,gzip,hashlib,json,pathlib,subprocess,sys
ROOT=pathlib.Path(__file__).resolve().parents[6]
OUT=pathlib.Path(__file__).resolve().parent
assert (ROOT/'Cargo.toml').exists(),ROOT
stamp=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(p): return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def git(*a):return subprocess.check_output(['git',*a],cwd=ROOT,text=True)
product=ROOT/'lab/target/product/debug/typst';oracle=ROOT/'lab/typst-original/target/release/typst'
build=ROOT/'lab/results/v16-corrections-2026-10-19/batch-d/validation/validation-state.json.gz'
bs=json.load(gzip.open(build,'rt'))
head=git('rev-parse','HEAD').strip()
inputs={p:{'recorded':h,'head':hashlib.sha256(subprocess.check_output(['git','show',f'{head}:{p}'],cwd=ROOT)).hexdigest()} for p,h in bs['inputs'].items()}
mismatch={p:h for p,h in inputs.items() if h['recorded']!=h['head']}
assert not mismatch,mismatch
assert sha(oracle)=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
receipts={}
for agent in ['grid','measurement']:
 p=ROOT/f'lab/results/v16-corrections-2026-10-19/batch-d/agents/{agent}/accepted-replay/receipt.json';r=json.load(open(p)); receipts[str(p.relative_to(ROOT))]={'sha256':sha(p),'binary':r['binaries']['product']};assert r['binaries']['product']['sha256']==sha(product)
record={'utc_start':stamp(),'head':head,'diff_head_stat':git('diff','HEAD','--stat'),'status':git('status','--short'),'build_receipt':{'path':str(build.relative_to(ROOT)),'sha256':sha(build),'utc':bs['utc'],'head':bs['checkpoint'],'diff_head_stat':bs['diff_head_stat'],'inputs_against_committed_head':inputs,'mismatches':mismatch},'accepted_receipts':receipts,'oracle':{'commit':'bd5ebec285505889e8ff1d4f68804ab147a699f3','tree':'cac836c07e4dec2a89219370428292393a3216b5','recipe':'lab/results/construction/diagnostics/vanilla-resync-2026-10-17/relatorio.md'},'binaries':{k:{'path':str(p),'sha256':sha(p)} for k,p in [('product',product),('oracle',oracle)]},'runner_sha256':sha(__file__),'cases':[]}
cases=[]
def add(name,expr,**kw):cases.append(dict(name=name,expression=expr,**kw))
def observe(name,expr,fields,**kw):
 s='(' + ','.join(json.dumps(x) for x in fields) + ',)'
 add(name,'{ let x = '+expr+'; (fields: x.fields(), checks: '+s+'.map(k => (key:k, has:x.has(k), at:x.at(k, default:"__ABSENT__"))), func: repr(x.func())) }',**kw)
for name,expr,fields in [
 ('empty','[]',['children','body']),('sequence','[a][b]',['children','body']),('text','[x]',['text','body']),
 ('raw-omitted','raw("x")',['text','block','lang']),('raw-explicit','raw("x",block:false,lang:none)',['text','block','lang']),
 ('box-omitted','box()',['body','width','height']),('box-empty','box([])',['body','width','height']),('box-body','box(width:20pt)[x]',['body','width','height']),
 ('block','block(width:20pt)[x]',['body','width','height']),('h-omitted','h(1pt)',['amount','weak']),('h-explicit','h(1pt,weak:false)',['amount','weak']),
 ('mat-omitted','math.mat((1,2),(3,4))',['rows','delim','align','augment','row-gap','column-gap']),('mat-explicit','math.mat(delim:none,align:center,augment:none,row-gap:0.5em,column-gap:0.5em,(1,2))',['rows','delim','align','augment','row-gap','column-gap']),
 ('cell-omitted','table.cell[x]',['body','x','fill','stroke','align','colspan']),('cell-explicit','table.cell(x:auto,fill:none,stroke:none,align:auto,colspan:1)[x]',['body','x','fill','stroke','align','colspan']),
 ('caption','figure.caption(position:bottom,separator:[: ])[x]',['position','separator','body','kind']),
 ('heading-omitted','heading[x]',['level','depth','numbering','supplement','body']),('heading-explicit','heading(level:1,numbering:none,supplement:none)[x]',['level','depth','numbering','supplement','body']),
 ('move','move(dx:0pt)[x]',['dx','dy','body']),('rotate','rotate(0deg,reflow:false)[x]',['angle','origin','reflow','body']),('scale','scale(x:100%,y:100%,reflow:false)[x]',['x','y','origin','reflow','body']),('skew','skew(ax:0deg,ay:0deg,reflow:false)[x]',['ax','ay','origin','reflow','body']),
 ('eq-omitted','math.equation($x$)',['block','numbering','number-align','supplement','alt','body']),('eq-explicit','math.equation(block:false,numbering:none,supplement:[E],alt:none,$x$)',['block','numbering','number-align','supplement','alt','body']),
 ('ordinary-styled','text(fill:red)[x]',['body','text','block','numbering']),('styled-equation','text(fill:red,math.equation(numbering:"(1)",$x$))',['body','block','numbering']),('sequence-equation','[a #math.equation(numbering:"(1)",$x$)]',['body','children','numbering']),
 ('lr','math.lr(size:100%,$x$)',['size','body']),('underbrace','math.underbrace($x$,none)',['body','annotation']),('stretch','math.stretch(size:100%,$x$)',['body','size']),
 ('outline-entry','outline.entry(level:1,element:heading[x],fill:none)',['level','element','fill','body']),('smartquote','smartquote(double:false,enabled:false)',['double','enabled','quotes']),
]:observe(name,expr,fields)
for name,expr in [('eq-native-equality','math.equation($x$).func() == math.equation'),('eq-call','math.equation($x$).func()(block:true,$y$).fields()'),('eq-alias','{let alias=math.equation; alias($x$).func()==alias}'),('eq-homonym','{let equation(x)=x; math.equation($x$).func()==equation}'),('eq-styled-call','math.equation(numbering:"(1)",$x$).func()($y$).fields()'),('mat-call','math.mat((1,)).func()((2,)).fields()'),('cell-call','table.cell[x].func()[y].fields()'),('caption-call','figure.caption[x].func()[y].fields()'),('heading-call','heading(supplement:none)[x].func()[y].fields()'),('rotate-call','rotate(0deg)[x].func()(10deg)[y].fields()'),('box-unset-error','box().at("width")'),('box-body-error','box[x].at("body")'),('eq-unknown-error','math.equation($x$).at("bogus")'),('cell-invalid-stroke-auto','table.cell(stroke:auto)[x]'),('cell-invalid-colspan-auto','table.cell(colspan:auto)[x]')]:add(name,expr)
for name,expr in [('raw','raw("x",lang:none)'),('empty','[]'),('box','box(width:20pt)[x]'),('block','block(width:20pt)[x]'),('mat','math.mat(delim:none,(1,2))'),('cell','table.cell(x:auto,fill:none,stroke:none)[x]'),('caption','figure.caption(position:bottom,separator:[: ])[x]'),('heading','heading(level:2,supplement:none)[x]'),('transform','rotate(0deg,origin:center,reflow:false)[x]'),('entry','outline.entry(level:1,element:heading[x],fill:none)'),('eq','math.equation(numbering:none,$x$)')]:
 for fmt in ['json','yaml']:add('serialize-'+name+'-'+fmt,expr,format=fmt)
query_source='#heading(level:2)[Title]\n#figure(rect(width:10pt,height:10pt),caption:[Cap])\n#table(columns:1,table.cell(fill:none)[Cell])\n$ mat(1,2;3,4) $\n'
for selector in ['heading','figure.caption','table.cell','math.mat']:
 for fmt in ['json','yaml']:add('query-'+selector+'-'+fmt,selector,format=fmt,source=query_source,command='query')
for case in cases:
 item=dict(case,utc_start=stamp(),head=git('rev-parse','HEAD').strip(),diff_head_stat=git('diff','HEAD','--stat'),results={})
 for label,binary in [('product',product),('oracle',oracle)]:
  fmt=case.get('format','json');cmd=case.get('command','eval')
  if cmd=='query':
   source=OUT/'query.typ';source.write_text(case['source']);argv=[str(binary),'query',str(source),case['expression'],'--format',fmt]
  else:argv=[str(binary),'eval',case['expression'],'--format',fmt]
  start=stamp()
  try:r=subprocess.run(argv,cwd=ROOT,text=True,capture_output=True,timeout=60);result={'returncode':r.returncode,'stdout':r.stdout,'stderr':r.stderr}
  except subprocess.TimeoutExpired as e:result={'timeout':True,'stdout':str(e.stdout),'stderr':str(e.stderr)}
  item['results'][label]=dict(result,argv=argv,cwd=str(ROOT),utc_start=start,utc_end=stamp())
 p,o=item['results']['product'],item['results']['oracle']
 try:item['parsed_equal']=p['returncode']==o['returncode']==0 and json.loads(p['stdout'])==json.loads(o['stdout'])
 except (ValueError,KeyError):item['parsed_equal']=None
 record['cases'].append(item)
 print(case['name'],p.get('returncode'),o.get('returncode'),item['parsed_equal'],flush=True)
 (OUT/'receipt.json').write_text(json.dumps(record,indent=2))
record.update(utc_end=stamp(),diff_head_stat_after=git('diff','HEAD','--stat'),binary_sha256_after={k:sha(p) for k,p in [('product',product),('oracle',oracle)]})
(OUT/'receipt.json').write_text(json.dumps(record,indent=2))
