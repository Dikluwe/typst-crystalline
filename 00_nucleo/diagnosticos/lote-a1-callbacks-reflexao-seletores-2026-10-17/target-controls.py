from pathlib import Path
from html.parser import HTMLParser
import json,gzip,subprocess,hashlib,datetime
p=Path(__file__).resolve().parent;root=p.parents[2];work=p/'outputs';now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
class Text(HTMLParser):
 def __init__(self):super().__init__();self.parts=[];self.skip=0
 def handle_starttag(self,tag,attrs):
  if tag in ['style','script','title']:self.skip+=1
 def handle_endtag(self,tag):
  if tag in ['style','script','title']:self.skip-=1
 def handle_data(self,data):
  if not self.skip:self.parts.append(data)
sources={
 'state-future-error':'#let s=state("k",0)\n#context s.get()\n#s.update(n=>panic("FUTURE"))',
 'state-at-update-boundary':'#let s=state("k",0)\n#s.update(n=>n+1)<u>\n#context assert.eq(s.at(<u>),1)',
 'heading-raw-constructor-func':'#assert.eq(heading(supplement:[Custom])[A].func(),heading)',
 'heading-raw-constructor-field':'#assert.eq(heading(supplement:[Custom])[A].supplement,[Custom])',
 'query-equation-direct':'#let e=math.equation\n$x$\n$y$\n#context assert.eq(query(e).len(),2)',
 'counter-equation-direct':'#set math.equation(numbering:"1")\n$ x $\n$ y $\n#context assert.eq(counter(math.equation).get(),(2,))',
 'heading-query-callback':'#set heading(numbering:"1",supplement:h=>[Custom])\n= A <a>\n#context {let h=query(heading).first();assert.eq(h.func(),heading);assert.eq(h.supplement,[Custom])}\nSee @a.',
 'heading-query-isolated':'#set heading(supplement:h=>[Custom])\n= A\n#context {let h=query(heading).first();assert.eq(h.func(),heading);assert.eq(h.supplement,[Custom])}',
 'heading-query-causal':'#let prefix="Captured"\n#let supplement(h)={assert.eq(prefix,"Captured");assert.eq(h.body,[A]);assert.eq(h.depth,1);[First]}\n#(prefix="Changed")\n#set heading(supplement:supplement)\n= A\n#set heading(supplement:h=>[Later])\n= B\n#context {let hs=query(heading);assert.eq(hs.map(h=>h.supplement),([First],[Later]))}',
 'heading-default-auto-none':'#set text(lang:"de")\n= A\n#heading(supplement:auto)[B]\n#heading(supplement:none)[C]\n#context {let hs=query(heading);assert.eq(hs.at(0).supplement,hs.at(1).supplement);assert.eq(hs.at(2).supplement,[])}',
 'heading-context-absent':'#set heading(supplement:h=>here())\n= A',
 'html-marker-spaces':'A #metadata("m") B',
 'html-labeled-body':'= A <a>\nB',
 'heading-panic':'#set heading(supplement:h=>panic("TARGET_HEADING"))\n= A',
 'heading-local':'#set heading(numbering:"1")\n#[#set heading(supplement:h=>[Local])\n= A <a>\n]\n= B <b>\nSee @a; @b.',
}
bins={'tekt':root/'target/debug/typst','before':work/'typst-before','vanilla':root/'lab/typst-original/target/release/typst'};rows=[]
for label,source in sources.items():
 f=work/('target-'+label+'.typ');f.write_text(source+'\n')
 for target in ['pdf','html']:
  row={'label':label,'target':target,'source':source,'source_sha256':hashlib.sha256(f.read_bytes()).hexdigest(),'results':{}}
  for name,binary in bins.items():
   out=work/('target-'+label+'-'+name+'.'+target);cmd=[str(binary),'compile',str(f),str(out)]+(['--features','html'] if target=='html' else []);start=now();r=subprocess.run(cmd,text=True,capture_output=True,timeout=45);text=''
   if r.returncode==0:
    if target=='pdf':text=subprocess.run(['pdftotext',str(out),'-'],text=True,capture_output=True).stdout
    else:
     parser=Text();parser.feed(out.read_text());text=' '.join(parser.parts)
   row['results'][name]={'command':cmd,'start':start,'end':now(),'exit':r.returncode,'stderr':r.stderr,'text':' '.join(text.split())}
  rows.append(row);print(label,target,[(n,v['exit'],v['text']) for n,v in row['results'].items()],flush=True)
(p/'target-controls.json.gz').write_bytes(gzip.compress(json.dumps({'utc':now(),'cases':rows},indent=2).encode()))
