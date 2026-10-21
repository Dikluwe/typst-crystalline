import ast,copy,json,hashlib,sys
from pathlib import Path
script=Path(sys.argv[1]);P=Path(__file__).resolve().parents[2];F=P.parent/'figure-kind-implementation';receipt=json.loads((F/'candidate-target-controls-4/receipt.json').read_text());historical=next(c for c in receipt['cases'] if c['name']=='header-figure-boundary')
tree=ast.parse(script.read_text());ns={};exec(compile(ast.Module(body=[n for n in tree.body if isinstance(n,ast.FunctionDef)],type_ignores=[]),str(script),'exec'),ns)
old=next(n for n in tree.body if isinstance(n,ast.Assert) and 'header-figure-boundary' in ast.unparse(n));old=compile(ast.Module(body=[old],type_ignores=[]),str(script),'exec')
fixtures=[('exact-historical-failure',{},True),('crash-in-known-gap',{'returncode':-11,'text':None,'stderr':'Segmentation fault\n'},False),('new-diagnostic-in-known-gap',{'stderr':'warning: newly introduced issue\n'},False)]
results=[]
for name,changes,wanted in fixtures:
 actual=copy.deepcopy(historical);actual.update(changes)
 try:
  rows=copy.deepcopy(receipt['cases']);rows[next(i for i,c in enumerate(rows) if c['name']=='header-figure-boundary')]=actual
  exec(old,dict(ns,r={'cases':rows}))
  if 'verify_header_debt' in ns:ns['verify_header_debt'](actual,historical)
  accepted=True
 except AssertionError:accepted=False
 results.append({'name':name,'accepted':accepted,'expected':wanted,'pass':accepted==wanted})
print(json.dumps({'script_sha256':hashlib.sha256(script.read_bytes()).hexdigest(),'historical_receipt_sha256':hashlib.sha256((F/'candidate-target-controls-4/receipt.json').read_bytes()).hexdigest(),'fixtures':results,'all_pass':all(x['pass'] for x in results)},indent=2))
raise SystemExit(not all(x['pass'] for x in results))
