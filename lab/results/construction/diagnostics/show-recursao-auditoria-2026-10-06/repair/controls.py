from pathlib import Path
import ast,json,gzip,subprocess,hashlib,datetime,os,re
from html.parser import HTMLParser
R=Path.cwd();H=R/'00_nucleo/diagnosticos/show-recursao-auditoria-2026-10-06/repair';T=Path('/tmp/tekt-show-occurrence/controls');T.mkdir(exist_ok=True)
bins={'tekt':Path('/tmp/tekt-show-occurrence/typst-final'),'vanilla':R/'lab/typst-original/target/release/typst'}
old=ast.parse((R/'00_nucleo/diagnosticos/memoizacao-indice-2026-10-05/verify.py').read_text());defs={}
for node in old.body:
 if isinstance(node,ast.Assign) and isinstance(node.targets[0],ast.Name) and node.targets[0].id in ['CASES','COMPILE']:defs[node.targets[0].id]=ast.literal_eval(node.value)
cases={n:{'source':s,'mode':'both'} for n,s in json.loads(Path('/tmp/tekt-show-occurrence/controls.json').read_text()).items()}
cases.update({'legacy-query-'+n:{'source':s,'mode':'query'} for n,s in defs['CASES'].items()})
cases.update({'legacy-pdf-'+n:{'source':s,'mode':'pdf'} for n,s in defs['COMPILE'].items()})
cases.update({'legacy-html-'+n:{'source':s,'mode':'html'} for n,s in {'show':'#show strong: it => emph(it.body)\nHello *world*','text':'#show "ab": [MATCH]\na#text[b]','nested':'#show strong: it => [a] + [b]\n#show "ab": [MATCH]\n*x*'}.items()})
cases['markup-recursive']={'source':'#show heading: it => [= Z]\n= Original','mode':'both'}
(T/'included.typ').write_text('#show heading.where(level: 1): it => heading(level: 2)[Generated]\n= Original');(T/'imported.typ').write_text('#let value = 42')
class Body(HTMLParser):
 def __init__(self):super().__init__();self.inside=False;self.nodes=[]
 def handle_starttag(self,t,a):
  if t=='body':self.inside=True
  if self.inside:self.nodes.append(['start',t,sorted(a)])
 def handle_endtag(self,t):
  if self.inside:self.nodes.append(['end',t])
  if t=='body':self.inside=False
 def handle_data(self,s):
  if self.inside and s.strip():self.nodes.append(['text',re.sub(r'\s+',' ',s)])
git=lambda *a:subprocess.check_output(['git',*a],text=True)
files=['01_core/src/compiler/eval/rules/apply.rs','01_core/src/compiler/eval/mod.rs','01_core/src/compiler/eval/tests.rs','01_core/src/compiler/eval/markup_stream.rs','01_core/src/compiler/eval/call_dispatch/invoke.rs','01_core/src/compiler/realize/mod.rs','01_core/src/compiler/eval/markup.rs']
d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'diff_stat':git('diff','HEAD','--stat'),'status':git('status','--short'),'binaries':{k:hashlib.sha256(b.read_bytes()).hexdigest() for k,b in bins.items()},'files':{p:(R/p).read_text() for p in files},'cases':{}}
for name,c in cases.items():
 f=T/(name+'.typ');f.write_text('#set text(font: "DejaVu Sans")\n'+c['source']);e={'source':f.read_text(),'mode':c['mode'],'engines':{}}
 for k,b in bins.items():
  mode='pdf' if c['mode']=='both' else c['mode'];out=T/(name+'-'+k+'.'+mode)
  args=['query',str(f),'heading','--field','body'] if mode=='query' else ['compile',str(f),str(out),'--root',str(T)]
  if mode=='html':args+=['--features','html','--pretty=false']
  if mode=='pdf':args+=['--pdf-tagged=false','--pretty=false']
  cmd=[str(b),*args];r=subprocess.run(cmd,capture_output=True,text=True,timeout=60,env=dict(os.environ,SOURCE_DATE_EPOCH='0',RAYON_NUM_THREADS='1',NO_COLOR='1'));a={'command':cmd,'returncode':r.returncode,'stdout':r.stdout,'stderr':r.stderr}
  if not r.returncode:
   if mode=='query':a['value']=json.loads(r.stdout)
   elif mode=='pdf':a['text']=subprocess.check_output(['pdftotext',str(out),'-'],text=True)
   else:p=Body();p.feed(out.read_text());a['body_dom']=p.nodes
   if c['mode']=='both':
    qcmd=[str(b),'query',str(f),'heading','--field','body'];q=subprocess.run(qcmd,capture_output=True,text=True,timeout=60);a['query']={'command':qcmd,'returncode':q.returncode,'stdout':q.stdout,'stderr':q.stderr}
    if not q.returncode:a['query']['value']=json.loads(q.stdout)
  e['engines'][k]=a
 a,b=e['engines']['tekt'],e['engines']['vanilla'];norm=lambda t:re.sub(r'[\s\u202a-\u202e]','',t)
 if a['returncode'] or b['returncode']:
  expected=name in ['constant_heading','markup-recursive'];e['acceptance_match']=expected and a['returncode']!=0 and b['returncode']!=0 and all(x in a['stderr'] and x in b['stderr'] for x in ['maximum show rule depth exceeded','maybe a show rule matches its own output','maybe there are too deeply nested elements'])
 else:
  if c['mode']=='query':e['acceptance_match']=a['value']==b['value']
  elif c['mode']=='html':e['acceptance_match']=a['body_dom']==b['body_dom']
  else:e['acceptance_match']=norm(a['text'])==norm(b['text'])
  if c['mode']=='both':e['query_match']=a['query']['returncode']==b['query']['returncode']==0 and a['query'].get('value')==b['query'].get('value');e['acceptance_match'] &= e['query_match']
 e['exact_stderr_match']=a['stderr']==b['stderr'];d['cases'][name]=e;print(name,e['acceptance_match'],flush=True)
d['finished_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(H/'controls.json.gz','wt') as f:json.dump(d,f,ensure_ascii=False)
