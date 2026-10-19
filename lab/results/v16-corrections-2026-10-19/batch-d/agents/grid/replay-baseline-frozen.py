"""Frozen baseline/candidate grid controls; compares language text and page/word geometry."""
import argparse,datetime,gzip,hashlib,json,os,subprocess,xml.etree.ElementTree as ET
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--product',default='lab/target/product/debug/typst');p.add_argument('--sha',required=True);p.add_argument('--receipt',required=True);p.add_argument('--output',required=True);p.add_argument('--baseline');a=p.parse_args()
root=Path.cwd(); base=Path(__file__).resolve().parent; out=base/a.output;out.mkdir(exist_ok=False);fixtures=base/'fixtures';fixtures.mkdir(exist_ok=True)
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def run(cmd):
 t=utc();r=subprocess.run(cmd,cwd=root,capture_output=True,text=True,timeout=45,env=env);return dict(argv=list(map(str,cmd)),utc_start=t,utc_end=utc(),exit=r.returncode,stdout=r.stdout,stderr=r.stderr)
env=os.environ.copy();env.update(RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',NO_COLOR='1',TZ='UTC',LC_ALL='C')
oracle=root/'lab/typst-original/target/release/typst'; product=root/a.product
assert sha(product)==a.sha
assert sha(oracle)=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
assert Path(a.receipt).exists()
font=root/'lab/results/construction/diagnostics/performance-baseline/fonts'
old=root/'lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09'
q=json.load(gzip.open(old/'qualification-local-cache.json.gz','rt'))
selected=[n for n in q['cases'] if n.endswith('-100')]
sources={n:((old/'fixtures'/f'{n}.typ').read_text(),str(old/'fixtures'/f'{n}.typ')) for n in selected}
prefix='#set page(width: 400pt, height: 300pt, margin: 24pt)\n#set text(font: "Libertinus Serif", size: 10pt)\n'
extra={
'grid-literal-auto':'#grid(columns: (100pt,100pt), [12.34], [item-2], [a/b], [abc_def])',
'grid-refused-space':'#grid(columns: (100pt,100pt), [12 34], [item 2], [café], [*bold*])',
'grid-source-interpolation':'#grid(columns: (100pt,100pt), ..range(4).map(i => [#("item-"+str(i)+".25")]))',
'grid-fixed':'#grid(columns: (100pt,100pt), rows: (20pt,30pt), [12.34], [item-2], [a/b], [abc_def])',
'raw-defaults':'#grid(columns: (100pt,100pt), fill:red, stroke:2pt, inset:7pt, [RAW], [BODY])',
'prepared-none':'#table(columns: (100pt,100pt), fill:red, stroke:2pt, inset:7pt, table.cell(fill:none,stroke:none)[NONE], [DEFAULT])',
'prepared-replaced':'#show table.cell: it => [REPLACED]\n#table(columns: (100pt,), fill:red, stroke:2pt, inset:7pt, table.cell(fill:none,stroke:none)[HIDDEN])',
'effects-once':'#let s = state("hits",0)\n#show table.cell: it => { s.update(n=>n+1); it }\n#table(columns: (100pt,100pt), [12.34], [item-2], [a/b], [abc_def])\n#context [HITS: #s.get()]',
'context-state':'#let s = state("k",0)\n#table(columns: (100pt,100pt), [#s.update(n=>n+1)A], [#context s.get()], [#s.update(n=>n+1)B], [#context s.get()])\n#context [FINAL: #s.get()]',
}
sources.update({k:(prefix+v,'new focused fixture derived from current L0 controls') for k,v in extra.items()})
paths=list(json.load(open(base/'phase1-before.json'))['files'])
def state():return {k:subprocess.check_output(cmd,text=True).strip() for k,cmd in {'head':['git','rev-parse','HEAD'],'diffstat':['git','diff','HEAD','--stat'],'status':['git','status','--short']}.items()}
report=dict(utc_start=utc(),state=state(),source_sha256={p:sha(p) for p in paths},runner_sha256=sha(__file__),binaries={'product':{'path':str(product),'sha256':a.sha,'build_receipt':a.receipt,'receipt_sha256':sha(a.receipt),'accepted_build_chain':'batch-c/validation-run/commands.json + agents/selectors/accepted-replay.json'},'oracle':{'path':str(oracle),'sha256':sha(oracle),'commit':'bd5ebec285505889e8ff1d4f68804ab147a699f3','tree':'cac836c07e4dec2a89219370428292393a3216b5','recipe':'lab/results/construction/diagnostics/vanilla-resync-2026-10-17/relatorio.md'}},historical_corpus={'receipt':str(old/'qualification-local-cache.json.gz'),'sha256':sha(old/'qualification-local-cache.json.gz'),'all_cases':list(q['cases']),'selected_cases':selected,'scope':'All nine 100-cell style variants from the historical 23-fixture corpus; larger sizes remain unmeasured, no performance claim.'},fonts={p.name:sha(p) for p in font.iterdir() if p.is_file()},cases={})
ns={'x':'http://www.w3.org/1999/xhtml'}
for name,(src,origin) in sources.items():
 f=fixtures/(name+'.typ');f.write_text(src);entry={'source':src,'sha256':sha(f),'origin':origin,'runs':{}}
 for who,binary in [('product',product),('oracle',oracle)]:
  pdf=out/(name+'-'+who+'.pdf');cmd=[str(binary),'compile',str(f),str(pdf),'--font-path',str(font)];r=run(cmd)
  if r['exit']==0:
   text=run(['pdftotext','-layout',str(pdf),'-']);bbox=run(['pdftotext','-bbox',str(pdf),'-']);tree=ET.fromstring(bbox['stdout'])
   r['observables']={'layout_text':text['stdout'],'pages':[{'size':pg.attrib,'words':[{'text':w.text,'box':w.attrib} for w in pg.findall('.//x:word',ns)]} for pg in tree.findall('.//x:page',ns)]};r['extract_commands']=[text['argv'],bbox['argv']]
  entry['runs'][who]=r
 report['cases'][name]=entry
 print(name,{who:r['exit'] for who,r in entry['runs'].items()},flush=True)
report['utc_end']=utc();report['end_state']=state();report['product_unchanged']=sha(product)==a.sha;report['oracle_unchanged']=sha(oracle)==report['binaries']['oracle']['sha256'];report['source_unchanged']=all(sha(p)==h for p,h in report['source_sha256'].items())
if a.baseline:
 before=json.load(open(base/a.baseline));report['preservation']={n:entry['runs']['product'].get('observables')==before['cases'][n]['runs']['product'].get('observables') and entry['runs']['product']['exit']==before['cases'][n]['runs']['product']['exit'] for n,entry in report['cases'].items()}
(out/'receipt.json').write_text(json.dumps(report,indent=2)+'\n')
