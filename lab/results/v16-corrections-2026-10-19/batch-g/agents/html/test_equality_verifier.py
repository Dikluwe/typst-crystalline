"""Exercise actual parser/check AST without running product/Cargo or policies."""
import ast,json,sys,hashlib
from pathlib import Path
from types import SimpleNamespace
p=Path(sys.argv[1]);tree=ast.parse(p.read_text());ns={'json':json}
functions=[n for n in tree.body if isinstance(n,ast.FunctionDef)]
exec(compile(ast.Module(body=functions,type_ignores=[]),str(p),'exec'),ns)
old=next((n for n in ast.walk(tree) if isinstance(n,ast.Try) and {'actual','expected'} <= {t.id for a in n.body if isinstance(a,ast.Assign) for t in a.targets if isinstance(t,ast.Name)}),None)
if old:parse=old
else:parse=next(n for n in ast.walk(tree) if isinstance(n,ast.Assign) and any(isinstance(t,ast.Tuple) and any(isinstance(e,ast.Name) and e.id=='actual_json_valid' for e in t.elts) for t in n.targets))
check=next(n for n in ast.walk(tree) if isinstance(n,ast.Assign) and any(isinstance(t,ast.Subscript) and isinstance(t.slice,ast.Constant) and t.slice.value=='language_values' for t in n.targets))
code=compile(ast.Module(body=[parse,check],type_ignores=[]),str(p),'exec')
fixtures=[('valid-equal','{"x":1}','{"x":1}',True,False),('invalid-actual-valid-oracle','not-json','{"x":1}',False,False),('invalid-oracle','{"x":1}','not-json',False,True),('legitimate-json-null','null','null',True,False)]
rows=[]
for name,actual,expected,want,raises in fixtures:
 env=dict(ns,p=SimpleNamespace(stdout=actual,returncode=0),oracle={'stdout':expected,'exit':0},row={'checks':{}})
 try:exec(code,env);value=env['row']['checks']['language_values'];raised=False
 except ValueError:value=False;raised=True
 rows.append({'name':name,'pass_value':value,'raised_invalid_oracle':raised,'expected_value':want,'expected_oracle_rejection':raises,'pass':value==want and raised==raises})
print(json.dumps({'runner_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'fixtures':rows,'all_pass':all(r['pass'] for r in rows)},indent=2))
raise SystemExit(not all(r['pass'] for r in rows))
