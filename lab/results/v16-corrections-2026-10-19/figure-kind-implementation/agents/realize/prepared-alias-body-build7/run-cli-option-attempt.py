from pathlib import Path
import hashlib,json,datetime,subprocess
ROOT=Path.cwd();BASE=Path(__file__).resolve().parent
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
product=ROOT/'lab/target/product/debug/typst';oracle=ROOT/'lab/typst-original/target/release/typst'
assert sha(product)=='9d441003a1fb8cc1b0ba27c254563a0a89d06ae08f4112ffd094e53cb0465021'
assert sha(oracle)=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
tail='''\n#context { let prepared=query(<alias>).first(); [#figure([OUTER],kind:image,supplement:prepared)<outer>] }\n@outer\n@outer\n#context [#metadata((query(figure).len(), counter(figure.where(kind:raw)).final(), counter(figure.where(kind:table)).final(), counter(figure.where(kind:image)).final(), query(figure).map(it => it.location()).dedup().len())) <probe>]\n'''
sources={
 'prepared-alias-body-context':'#figure([#context figure([NESTED],kind:raw,supplement:[Raw])],kind:table)<alias>'+tail,
 'prepared-alias-body-figure':'#figure(figure([NESTED],kind:raw,supplement:[Raw]),kind:table)<alias>'+tail,
}
for name,source in sources.items():(BASE/(name+'.typ')).write_text(source)
receipt={'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'working_tree':'uncommitted; current tree differs from build7 after origin-only fixes; product evidence bound to frozen build7 receipt, not current source','diff_head_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'build_receipt':'lab/results/v16-corrections-2026-10-19/figure-kind-implementation/reference-build-7/receipt.json','build_receipt_sha256':sha(ROOT/'lab/results/v16-corrections-2026-10-19/figure-kind-implementation/reference-build-7/receipt.json'),'runner_sha256':sha(Path(__file__)),'oracle_commit':'bd5ebec285505889e8ff1d4f68804ab147a699f3','oracle_tree':'cac836c07e4dec2a89219370428292393a3216b5','binary_sha256':{'oracle':sha(oracle),'product7':sha(product)},'cases':[]}
for name in sources:
 source=BASE/(name+'.typ');case={'name':name,'source_sha256':sha(source),'runs':[]}
 for label,binary in [('oracle',oracle),('product7',product)]:
  for action in ['compile','query']:
   output=BASE/(name+'-'+label+'.pdf')
   argv=[str(binary),action,*(['-j','1',str(source),str(output)] if action=='compile' else [str(source),'<probe>','--field','value'])]
   start=utc();run=subprocess.run(argv,cwd=ROOT,capture_output=True,text=True,timeout=45)
   case['runs'].append({'target':label,'action':action,'utc_start':start,'utc_end':utc(),'argv':argv,'exit':run.returncode,'stdout':run.stdout,'stderr':run.stderr})
   print(name,label,action,run.returncode,run.stdout.strip(),run.stderr[:160],flush=True)
 receipt['cases'].append(case)
receipt['utc_end']=utc();receipt['binary_sha256_after']={'oracle':sha(oracle),'product7':sha(product)}
assert receipt['binary_sha256_after']==receipt['binary_sha256']
(BASE/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
