"""Fixture mutations exercise the actual replay formula, without invoking a product."""
import ast
import copy
import hashlib
import json
from pathlib import Path
from types import SimpleNamespace
import sys

HERE=Path(__file__).resolve().parent
runner=Path(sys.argv[1]) if len(sys.argv)>1 else HERE/'replay_cli.py'
policy_path=Path(sys.argv[2]) if len(sys.argv)>2 else HERE/'cli-policies.json'
policy=json.loads(policy_path.read_text())
source=runner.read_text();tree=ast.parse(source)
setup=[]
for node in tree.body:
    if isinstance(node,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='parser' for t in node.targets):break
    setup.append(node)
ns={'__file__':str(runner)}
exec(compile(ast.Module(body=setup,type_ignores=[]),str(runner),'exec'),ns)
loop=next(n for n in tree.body if isinstance(n,ast.For) and isinstance(n.target,ast.Name) and n.target.id=='case')
checks=[];started=False
for node in loop.body:
    if isinstance(node,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='actual_diagnostics' for t in node.targets):started=True
    if started:checks.append(node)
    if isinstance(node,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='required_pass' for t in node.targets):break
code=compile(ast.Module(body=checks,type_ignores=[]),str(runner),'exec')
case=next(c for c in policy['cases'] if c['name']=='heading-six')
feature=case['oracle']['stderr'].split('\n\n')[0]+'\n\n'
fixtures=[('exact-oracle',case['oracle']['html'],case['oracle']['stderr'],True),
 ('exact-deep-warning-omission',case['oracle']['html'],feature,True),
 ('missing-doctype',case['oracle']['html'].replace('<!DOCTYPE html>',''),feature,False),
 ('wrong-doctype',case['oracle']['html'].replace('<!DOCTYPE html>','<!DOCTYPE HTML PUBLIC "old">'),feature,False),
 ('missing-feature-warning',case['oracle']['html'],'',False),
 ('extra-warning',case['oracle']['html'],feature+'warning: unrelated\n',False),
 ('wrong-deep-warning',case['oracle']['html'],feature+'warning: heading warning with different contract\n',False),
 ('error-with-success-exit',case['oracle']['html'],feature+'error: unexpected failure\n',False)]
results=[]
for name,html,stderr,expected in fixtures:
    env=dict(ns,case=case,run=SimpleNamespace(returncode=0,stderr=stderr),observed_html=html,html_files={'one.html':html})
    exec(code,env)
    actual=bool(env['required_pass']);results.append({'name':name,'expected':expected,'actual':actual,'pass':actual==expected})
print(json.dumps({'runner_sha256':hashlib.sha256(runner.read_bytes()).hexdigest(),'policy_sha256':hashlib.sha256(policy_path.read_bytes()).hexdigest(),'fixtures':results,'failed':[r['name'] for r in results if not r['pass']]},indent=2))
raise SystemExit(not all(r['pass'] for r in results))
