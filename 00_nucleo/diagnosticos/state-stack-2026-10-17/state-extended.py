from pathlib import Path
import json,gzip,subprocess,hashlib,datetime
from html.parser import HTMLParser
class Text(HTMLParser):
 def __init__(self):super().__init__();self.parts=[];self.skip=0
 def handle_starttag(self,tag,attrs):
  if tag in ["style","script","title"]:self.skip+=1
 def handle_endtag(self,tag):
  if tag in ["style","script","title"]:self.skip-=1
 def handle_data(self,data):
  if not self.skip:self.parts.append(data)
p=Path(__file__).resolve().parent;root=p.parents[2];work=p/'outputs';now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
cases={
 'reader-init':('#let a=state("k",1)\n#let b=state("k",10)\n#a.update(n=>n+1)\n#context assert.eq((a.get(),b.get(),a.final(),b.final()),(2,11,2,11))',0),
 'init-none':('#let s=state("k")\n#s.update(v=>if v==none {2} else {panic("INIT")})\n#context assert.eq(s.final(),2)',0),
 'ordered':('#let s=state("k",1)\n#s.update(v=>v+1)\n#s.update(7)\n#s.update(v=>v*2)\n#context assert.eq(s.final(),14)',0),
 'future-error':('#let s=state("k",0)\n#context s.get()\n#s.update(v=>panic("FUTURE"))',1),
 'error-before-set':('#let s=state("k",0)\n#s.update(v=>panic("BEFORE_SET"))\n#s.update(1)\n#context s.final()',1),
 'unused':('#let s=state("k",0)\n#s.update(v=>panic("UNUSED"))',0),
 'discarded':('#let s=state("k",7)\n#let u=s.update(v=>panic("DISCARDED"))\n#context assert.eq(s.final(),7)',0),
 'inserted-twice':('#let s=state("k",0)\n#let u=s.update(v=>v+1)\n#u\n#u\n#context assert.eq(s.final(),2)',0),
 'independent-keys':('#let a=state("a",1)\n#let b=state("b",10)\n#a.update(v=>v+1)\n#b.update(v=>panic("OTHER_KEY"))\n#context assert.eq(a.final(),2)',0),
 'captured-update':('#let s=state("k",1)\n#let update=s.update\n#update(v=>v+1)\n#context assert.eq(s.final(),2)',1),
 'captured-reader':('#let s=state("k",1)\n#let final=s.final\n#s.update(v=>v+1)\n#context assert.eq(final(),2)',1),
 'closure-capture':('#let step=2\n#let callback(v)=v+step\n#(step=100)\n#let s=state("k",1)\n#s.update(callback)\n#context assert.eq(s.final(),3)',0),
 'at-prefix':('#let s=state("k",1)\n#metadata("before")<before>\n#s.update(v=>v+1)<update>\n#metadata("after")<after>\n#s.update(v=>v*10)\n#context assert.eq((s.at(<before>),s.at(<update>),s.at(<after>),s.final()),(1,2,2,20))',0),
 'context-not-inherited':('#let s=state("k",0)\n#s.update(v=>here())\n#context s.final()',1),
 'update-extra-named':('#let s=state("k",0)\n#s.update(1, extra:2)',1),
 'get-extra':('#let s=state("k",0)\n#context s.get(1)',1),
}
extensions={
 'legacy-mixed':('#state_update("k",1)\n#let s=state("k",10)\n#s.update(v=>v+1)\n#state_update_with("k",v=>v*2)\n#context assert.eq((s.final(),state_final("k")),(4,4))','A'),
 'modern-display':('#let a=state("k",1)\n#a.update(v=>v+1)\n#context state("k",10).display(v=>str(v))','11 A'),
 'legacy-display':('#state_update("k",1)\n#state_update_with("k",v=>v+1)\n#state_display("k",v=>str(v))','2A'),
}
bins={'tekt':root/'target/debug/typst','before':work/'typst-before','vanilla':root/'lab/typst-original/target/release/typst'};rows=[]
for extension,group in [(False,cases),(True,extensions)]:
 for label,(source,expected) in group.items():
  f=work/('state-extended-'+label+'.typ');f.write_text(source+'\nA\n')
  for target in ['pdf','html']:
   row={'label':label,'extension':extension,'target':target,'source':source+'\nA\n','expected_exit':0 if extension else expected,'expected_text':expected if extension else None,'source_sha256':hashlib.sha256(f.read_bytes()).hexdigest(),'results':{}}
   for name,binary in bins.items():
    out=work/('state-extended-'+label+'-'+name+'.'+target);cmd=[str(binary),'compile',str(f),str(out)]+(['--features','html'] if target=='html' else []);start=now();r=subprocess.run(cmd,text=True,capture_output=True,timeout=60)
    text=subprocess.run(['pdftotext',str(out),'-'],capture_output=True,text=True).stdout if target=='pdf' and r.returncode==0 else (out.read_text() if target=='html' and r.returncode==0 else '')
    if target=='html' and r.returncode==0:
     parser=Text();parser.feed(text);text=' '.join(parser.parts)
    row['results'][name]={'command':cmd,'start':start,'end':now(),'exit':r.returncode,'stderr':r.stderr,'text':' '.join(text.split())}
   rows.append(row);print(label,target,[(n,v['exit']) for n,v in row['results'].items()],flush=True)
(p/'state-extended.json.gz').write_bytes(gzip.compress(json.dumps({'utc':now(),'cases':rows},indent=2).encode()))
