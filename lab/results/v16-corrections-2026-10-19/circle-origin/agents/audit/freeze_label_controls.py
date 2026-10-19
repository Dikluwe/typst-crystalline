import copy,datetime,hashlib,json,pathlib,sys
H=pathlib.Path(__file__).resolve().parent;sys.path.insert(0,str(H));from replay import obs
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();R=pathlib.Path('/repos/Antigravity/typst-crystalline')
assert not (H/'label-controls-policy.json').exists();extra={c['name']:c for c in json.loads((H/'label-control-independent-values.json').read_text())['cases']};keep={'shared-circle-unique-tags','repeated-label-same-element','duplicate-label-or','duplicate-label-and','singlelookup-locate'};repair={'shared-circle','distinct-circle','shared-rect','shared-text','direct-metadata','two-context-constants','same-context-constants','shared-circle-different-body','shared-circle-blocks','equal-values-distinct-elements','interleaved-source-order'};cases=[]
for folder in ['context-investigation','label-selector-controls']:
 p=H/folder/'receipt.json';receipt=json.loads(p.read_text())
 for c in receipt['cases']:
  old=obs(c['results']['product'],c);oracle=obs(c['results']['oracle'],c);expected=copy.deepcopy(old);name=c['name']
  if name in keep:rule='preserve exact existing behavior; unrelated diagnosis/lookup/warning debt excluded'
  elif name in repair:expected=copy.deepcopy(oracle);rule='repair Label-selector cardinality/order using exact pinned oracle observation'
  else:
   assert name in extra;ref=extra[name]['results']['product'];assert ref['returncode']==0;expected['value']=json.loads(ref['stdout']);rule='repair cardinality with exact values independently measured through metadata selector; preserve unrelated position/Context-style debt'
  cases.append({'name':name,'source':c['source'],'selector':c['results']['product']['argv'][3],'baseline_receipt':{'path':str(p.relative_to(R)),'sha256':sha(p)},'rule':rule,'baseline':old,'oracle':oracle,'expected':expected})
assert len(cases)==18;d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'cases':cases,'independent_values_sha256':sha(H/'label-control-independent-values.json'),'scope':'Only measured Label cardinality/causal-order prerequisite. The original224Circle policies remain unchanged.'};(H/'label-controls-policy.json').write_text(json.dumps(d,indent=2)+'\n');print(sha(H/'label-controls-policy.json'))
