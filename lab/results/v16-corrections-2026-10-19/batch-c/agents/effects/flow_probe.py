import subprocess,hashlib,json,datetime,pathlib
out=pathlib.Path('lab/results/v16-corrections-2026-10-19/batch-c/agents/effects');root=pathlib.Path.cwd()
def run(a):
 try:
  r=subprocess.run(a,capture_output=True,text=True,timeout=25);return {'exit':r.returncode,'stdout':r.stdout,'stderr':r.stderr}
 except subprocess.TimeoutExpired as e:return {'timeout':True,'stdout':str(e.stdout),'stderr':str(e.stderr)}
state={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':run(['git','rev-parse','HEAD'])['stdout'].strip(),'diffstat':run(['git','diff','HEAD','--stat'])['stdout'],'product_receipt':'lab/results/v16-corrections-2026-10-19/batch-b/validation','binaries':{},'results':[]}
cases={
'flow-count-located':'#show par: it => {metadata("PAR"); it}\n#par[A] <source>\n#context query(<source>).first()',
'flow-count-plain':'#show par: it => {metadata("PAR"); it}\n#par[A]\n#par[B]',
'flow-count-surround':'#show par: it => {metadata("PAR"); it}\nBefore #par[A] after\n\n#par[B]',
'flow-count-produced':'#show par: it => {metadata("PAR"); it}\n#show strong: it => par(it.body)\n*ONE*\n\nTWO',
'flow-count-located-style':'#show par: it => {metadata("PAR"); it}\n#par[A] <source>\n#context text(fill:red,query(<source>).first())',
'flow-count-identity':'#show par: it => it\n#par[A] <source>\n#context {let c=query(<source>).first();metadata(repr([before #c after]))}',
}
for who,b in [('oracle','lab/typst-original/target/release/typst'),('product','lab/target/product/debug/typst')]:
 state['binaries'][who]={'path':b,'sha256':hashlib.sha256(pathlib.Path(b).read_bytes()).hexdigest()}
 for name,src in cases.items():
  p=out/(name+'.typ');p.write_text(src);argv=[str(root/b),'query',str(p),'metadata','--field','value'];result={'binary':who,'name':name,'source':src,'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'argv':argv,**run(argv)};state['results'].append(result);print(who,name,result.get('exit'),result['stdout'][:250],result['stderr'][:90]);(out/'flow-probes.json').write_text(json.dumps(state,indent=2)+'\n')
