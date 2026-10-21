"""Replay fixed batch-G expression/Context language evidence against exact candidate."""
from pathlib import Path
import argparse,subprocess,json,hashlib,datetime
A=argparse.ArgumentParser();A.add_argument('--product',type=Path,required=True);A.add_argument('--build-receipt',type=Path,required=True);A.add_argument('--out',type=Path,required=True);a=A.parse_args()
P=Path(__file__).resolve().parent;R=Path.cwd();sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def language_json_values(actual_stdout,oracle_stdout):
 # Invalid oracle evidence is a harness error; invalid candidate JSON fails its check.
 expected=json.loads(oracle_stdout)
 try:actual=json.loads(actual_stdout)
 except ValueError:return False,None,expected
 return True,actual,expected
b=json.loads(a.build_receipt.read_text());assert b['exit']==0 and b['changed_inputs']==[];assert sha(a.product)==b['product_sha256'];assert all(sha(R/n)==d for n,d in b['inputs'].items())
assert not a.out.exists();a.out.mkdir(parents=True)
r={'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'build_receipt':str(a.build_receipt),'build_receipt_sha256':sha(a.build_receipt),'product_sha256':sha(a.product),'inputs':b['inputs'],'runner_sha256':sha(__file__),'cases':[]}
product=a.product.resolve()
for group in ['equality-probes','context-intent-probes']:
 oracle_path=P/group/'receipt.json';o=json.loads(oracle_path.read_text());assert o['binary_sha256']['oracle']=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
 for c in o['cases']:
  s=Path(c.get('source',P/group/(c['name']+'.expr')))
  if not s.is_absolute():s=R/s
  assert sha(s)==c['source_sha256']
  oracle=c['runs']['oracle'];row={'group':group,'name':c['name'],'source':str(s),'source_sha256':sha(s),'oracle_receipt_sha256':sha(oracle_path),'checks':{}}
  if c['name']=='distinct-rendered':
   out=a.out/'distinct-rendered.pdf';argv=[str(product),'compile',str(s),str(out)];p=subprocess.run(argv,capture_output=True,text=True,timeout=30);row['compile']={'argv':argv,'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr};row['checks']['compile_success']=p.returncode==oracle['compile']['exit']==0
   argv=[str(product),'query',str(s),'<probe>','--field','value'];p=subprocess.run(argv,capture_output=True,text=True,timeout=30);row['query']={'argv':argv,'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr}
   try:row['checks']['language_query']=p.returncode==0 and json.loads(p.stdout)==json.loads(oracle['query']['stdout'])
   except ValueError:row['checks']['language_query']=False
   text=subprocess.run(['pdftotext','-layout',str(out),'-'],capture_output=True,text=True).stdout if out.exists() else ''
   row['rendered_text']=text;row['checks']['both_emissions']=' '.join(text.split())==' '.join(oracle['text'].split())=='AB';row['required']=True
  else:
   argv=[str(product),*oracle['argv'][1:]];p=subprocess.run(argv,capture_output=True,text=True,timeout=30);row['run']={'argv':argv,'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr}
   actual_json_valid,actual,expected=language_json_values(p.stdout,oracle['stdout'])
   row['checks']['json_output']=actual_json_valid
   if group=='equality-probes' and c['name']=='html-tristate-fields':
    row['required']=False;row['classification']='Existing HTML fields {} discrepancy; observed comparison retained open, not accepted as parity.';row['checks']['observed_fields_match']=actual==expected and p.returncode==0
   elif group=='equality-probes' and c['name']!='title-fields':
    row['required']=True;row['checks']['eval_success']=p.returncode==oracle['exit']==0
    for k in ['equal','unequal','member','nested']:row['checks'][k]=isinstance(actual,dict) and actual.get(k)==expected[k]
    row['repr_observation']={'actual':{k:actual.get(k) for k in ['left','right']} if isinstance(actual,dict) else None,'oracle':{k:expected[k] for k in ['left','right']}}
    row['repr_note']='Stored independently; known baseline title/frame/empty-attrs repr presentation differences are outside selected equality closure.'
   else:row['required']=True;row['checks']['language_values']=p.returncode==oracle['exit']==0 and actual_json_valid and actual==expected
  row['pass']=all(row['checks'].values());r['cases'].append(row);print(group,c['name'],row['required'],row['pass'],flush=True)
r['utc_end']=utc();r['changed_inputs']=[n for n,d in b['inputs'].items() if sha(R/n)!=d];r['binary_unchanged']=sha(a.product)==r['product_sha256'];r['required_failures']=[c['name'] for c in r['cases'] if c['required'] and not c['pass']];r['open_observed_comparisons']=[c['name'] for c in r['cases'] if not c['required'] and not c['pass']]
(a.out/'receipt.json').write_text(json.dumps(r,indent=2)+'\n');assert not r['changed_inputs'] and r['binary_unchanged'];print('required_failures',r['required_failures'])
