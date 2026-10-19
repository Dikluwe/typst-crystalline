from pathlib import Path
import json,gzip,datetime,hashlib,subprocess,sys
p=Path(__file__).resolve().parent;r=p.parents[2];label_run=sys.argv[1];out=p/('outputs/'+label_run);out.mkdir(parents=True,exist_ok=True)
from html.parser import HTMLParser
class BodyText(HTMLParser):
 def __init__(self):super().__init__();self.body=False;self.parts=[];self.skip=0
 def handle_starttag(self,t,a):
  if t=='body':self.body=True
  if t in ['style','script']:self.skip+=1
 def handle_endtag(self,t):
  if t=='body':self.body=False
  if t in ['style','script']:self.skip-=1
 def handle_data(self,data):
  if self.body and not self.skip:self.parts.append(data)
sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest()
cases={
 'equation-one': '#show math.equation.where():it=>[EQ]\n#context math.frac([1],[2])\n#context[Q#query(math.equation).len()]',
 'equation-two': '#show math.equation.where():it=>[EQ]\n#context math.frac([1],[2])\n#context math.frac([3],[4])\n#context[Q#query(math.equation).len()]',
 'equation-heading-prefix': '#show heading:it=>it\n= H\n#show math.equation.where():it=>[EQ]\n#context math.frac([1],[2])\n#context[Q#query(math.equation).len()]',
 'text-context-global': '#show "A":[L]\n#let c=context[A]\n#show "A":[R]\n#c',
 'text-context-local': '#let c=[#show "A":[L]\n#context[A]]\n#show "A":[R]\n#c',
}
cases.update({
 'text-created-before-rule': '#let c=context[A]\n#show "A":[R]\n#c',
 'text-local-inside-context': '#show "A":[R]\n#context { show "A":[L]; [A] }',
 'text-nested-context': '#show "A":[R]\n#context [#context[A]]',
 'text-local-nested-context': '#let c=[#show "A":[L]\n#context [#context[A]]]\n#show "A":[R]\n#c',
 'text-regex-context': '#show regex("A+"):it=>[R]\n#context[AAA]',
 'text-hidden-context': '#show strong:[H]\n#show "A":it=>panic("hidden")\n#context[*A*]',
 'text-context-product': '#show "A":[AA]\n#context[A]',
 'text-styled-context': '#show "A":[R]\n#context text(fill:red)[A]',
 'equation-locations': '#show math.equation.where():it=>context [#query(math.equation).position(e=>e.location()==it.location())]\n#context math.frac([1],[2])\n#context math.frac([3],[4])',
})
rule='#show math.equation.where():it=>[EQ]\n'
query='\n#context[Q#query(math.equation).len()]'
cases.update({
 'equation-equal-two':rule+'#context math.frac([1],[2])\n#context math.frac([1],[2])'+query,
 'equation-nested-contexts':rule+'#context [#context math.frac([1],[2])]\n#context [#context math.frac([3],[4])]'+query,
 'equation-alias': '#show math.equation.where():it=>[#it#it]\n#context math.frac([1],[2])\n#context math.frac([3],[4])\n#context assert.eq(query(math.equation).len(),2)',
 'equation-generated': '#show math.equation.where(block:false):it=>math.equation([x],block:true)\n#context math.frac([1],[2])\n#context math.frac([3],[4])'+query,
 'equation-grow': '= H\n'+rule+'#context { if counter(heading.where(level:1)).final() == (0,) { math.frac([1],[2]) } else { [#math.frac([1],[2]) #math.frac([3],[4])] } }'+query,
 'equation-shrink': '= H\n'+rule+'#context { if counter(heading.where(level:1)).final() == (0,) { [#math.frac([1],[2]) #math.frac([3],[4])] } else { math.frac([1],[2]) } }'+query,
})
cases['equation-grow-numbered']=cases['equation-grow'].replace('= H', '#set heading(numbering:"1.")\n= H',1)
cases['equation-shrink-numbered']=cases['equation-shrink'].replace('= H', '#set heading(numbering:"1.")\n= H',1)
cases['equation-alias-count']=cases['equation-alias'].replace('#context assert.eq(query(math.equation).len(),2)', '#context[Q#query(math.equation).len()]')
for kind in ['grow','shrink']:
 cases['equation-'+kind+'-selected-query']=cases['equation-'+kind+'-numbered'].replace('#context[Q#query(math.equation).len()]', '#context { let _ = counter(heading.where(level:1)).final(); [Q#query(math.equation).len()] }')
bins={'tekt' :r/'target/debug/typst','vanilla':r/'lab/typst-original/target/release/typst'}
d=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_state='baseline' if label_run=='before' else 'working tree',pin='bd5ebec285505889e8ff1d4f68804ab147a699f3',binaries={n:dict(path=str(b),sha256=sha(b)) for n,b in bins.items()},cases=[])
for label,source in cases.items():
 f=out/(label+'.typ');f.write_text(source);row=dict(label=label,source=source,source_sha256=sha(f),results={})
 for name,binary in bins.items():
  target=out/(label+'-'+name+'.html');cmd=[str(binary),'compile',str(f),str(target),'--features','html'];z=subprocess.run(cmd,text=True,capture_output=True,timeout=60);result=dict(command=cmd,exit=z.returncode,stderr=z.stderr,text='')
  if z.returncode==0:
   parser=BodyText();parser.feed(target.read_text());result.update(html_sha256=sha(target),text=' '.join(''.join(parser.parts).split()))
  row['results'][name]=result
 d['cases'].append(row);print(label,[(k,v['exit'],v['text']) for k,v in row['results'].items()],flush=True)
d['end']=datetime.datetime.now(datetime.timezone.utc).isoformat();assert all(sha(b)==d['binaries'][n]['sha256'] for n,b in bins.items());(p/(label_run+'.json.gz')).write_bytes(gzip.compress(json.dumps(d,indent=2).encode()))
