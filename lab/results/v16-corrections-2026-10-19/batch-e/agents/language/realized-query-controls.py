#!/usr/bin/env python3
"""Bounded independent realized Equation query evidence. Never builds or edits product."""
import datetime,gzip,hashlib,json,pathlib,subprocess
ROOT=pathlib.Path(__file__).resolve().parents[6]; OUT=pathlib.Path(__file__).resolve().parent/'realized-query-controls'; OUT.mkdir(exist_ok=False)
def sha(p):return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def git(*args):return subprocess.check_output(['git',*args],cwd=ROOT,text=True)
state=ROOT/'lab/results/v16-corrections-2026-10-19/batch-e/validation/validation-state.json.gz'
build=json.load(gzip.open(state,'rt')); inputs={p:sha(ROOT/p) for p in build['inputs']}; mismatches={p:[h,inputs[p]] for p,h in build['inputs'].items() if inputs[p]!=h}
assert not mismatches,mismatches
bins={'product':ROOT/'lab/target/product/debug/typst','oracle':ROOT/'lab/typst-original/target/release/typst'}
assert sha(bins['oracle'])=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
r={'utc_start':utc(),'head':git('rev-parse','HEAD').strip(),'diff_head_stat':git('diff','HEAD','--stat'),'runner_sha256':sha(__file__),'build_receipt':{'path':str(state.relative_to(ROOT)),'sha256':sha(state),'inputs':inputs,'mismatches':mismatches},'binaries':{k:{'path':str(p),'sha256':sha(p)} for k,p in bins.items()},'oracle_pin':{'commit':'bd5ebec285505889e8ff1d4f68804ab147a699f3','tree':'cac836c07e4dec2a89219370428292393a3216b5','recipe':'lab/results/construction/diagnostics/vanilla-resync-2026-10-17/relatorio.md'},'cases':[]}
sources={'explicit':'#math.equation(block:true,numbering:"(1)",alt:"probe")[x] <e>\n','captured':'#set math.equation(numbering:"(1)",alt:"probe")\n#math.equation(block:true)[x] <e>\n'}
for kind,source in sources.items():
 path=OUT/(kind+'.typ');path.write_text(source)
 cases=[('all','math.equation',None),('label','<e>',None)]+[('kind-field-'+f,'math.equation',f) for f in ['block','body','numbering','alt']]
 for name,selector,field in cases:
  c={'name':kind+'-'+name,'source':source,'source_sha256':sha(path),'selector':selector,'field':field,'surface':'realized occurrence query','utc_start':utc(),'results':{}}
  for label,binary in bins.items():
   argv=[str(binary),'query',str(path),selector,'--format','json']
   if field: argv+=['--field',field]
   t=utc()
   try:
    x=subprocess.run(argv,cwd=ROOT,text=True,capture_output=True,timeout=60);o={'returncode':x.returncode,'stdout':x.stdout,'stderr':x.stderr}
   except subprocess.TimeoutExpired as x:o={'timeout':True,'stdout':str(x.stdout),'stderr':str(x.stderr)}
   c['results'][label]={**o,'argv':argv,'cwd':str(ROOT),'utc_start':t,'utc_end':utc()}
  r['cases'].append(c);print(c['name'],{k:[v.get('returncode'),v['stdout'],v['stderr']] for k,v in c['results'].items()},flush=True)
  (OUT/'receipt.json').write_text(json.dumps(r,indent=2))
r.update(utc_end=utc(),head_after=git('rev-parse','HEAD').strip(),diff_head_stat_after=git('diff','HEAD','--stat'),changed_build_inputs={p:[h,sha(ROOT/p)] for p,h in inputs.items() if sha(ROOT/p)!=h},binary_sha256_after={k:sha(p) for k,p in bins.items()})
(OUT/'receipt.json').write_text(json.dumps(r,indent=2))
