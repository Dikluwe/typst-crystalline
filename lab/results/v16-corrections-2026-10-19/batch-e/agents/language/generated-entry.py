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
for fmt in ['json','yaml']:
 add('query-generated-entry-level-'+fmt,'outline.entry',format=fmt,source='#outline()\n= X\n',command='query',field='level')
for case in cases:
 item=dict(case,utc_start=stamp(),head=git('rev-parse','HEAD').strip(),diff_head_stat=git('diff','HEAD','--stat'),results={})
 for label,binary in [('product',product),('oracle',oracle)]:
  fmt=case.get('format','json');cmd=case.get('command','eval')
  if cmd=='query':
   source=OUT/'generated-entry.typ';source.write_text(case['source']);argv=[str(binary),'query',str(source),case['expression'],'--format',fmt]
  else:argv=[str(binary),'eval',case['expression'],'--format',fmt]
  if case.get('field'): argv += ['--field',case['field']]
  if case.get('one'): argv += ['--one']
  start=stamp()
  try:r=subprocess.run(argv,cwd=ROOT,text=True,capture_output=True,timeout=60);result={'returncode':r.returncode,'stdout':r.stdout,'stderr':r.stderr}
  except subprocess.TimeoutExpired as e:result={'timeout':True,'stdout':str(e.stdout),'stderr':str(e.stderr)}
  item['results'][label]=dict(result,argv=argv,cwd=str(ROOT),utc_start=start,utc_end=stamp())
 p,o=item['results']['product'],item['results']['oracle']
 try:item['parsed_equal']=p['returncode']==o['returncode']==0 and json.loads(p['stdout'])==json.loads(o['stdout'])
 except (ValueError,KeyError):item['parsed_equal']=None
 record['cases'].append(item)
 print(case['name'],p.get('returncode'),o.get('returncode'),item['parsed_equal'],flush=True)
 (OUT/'generated-entry-receipt.json').write_text(json.dumps(record,indent=2))
record.update(utc_end=stamp(),diff_head_stat_after=git('diff','HEAD','--stat'),binary_sha256_after={k:sha(p) for k,p in [('product',product),('oracle',oracle)]})
(OUT/'generated-entry-receipt.json').write_text(json.dumps(record,indent=2))
