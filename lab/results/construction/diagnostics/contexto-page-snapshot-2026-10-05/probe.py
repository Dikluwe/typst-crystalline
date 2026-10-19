import datetime,gzip,hashlib,json,subprocess,re
from pathlib import Path
H=Path(__file__).resolve().parent;R=H.parents[2];T=Path('/tmp/tekt-page-snapshot');T.mkdir(exist_ok=True);sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();git=lambda *a:subprocess.check_output(['git',*a],cwd=R,text=True)
bins={'tekt':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'}
cases={'single':'#context [B#counter(page).display("1 / 1", both: true) F#counter(page).final().first()]','two':'#context counter(page).display("1 / 1", both: true)\n#pagebreak()\n#context counter(page).final().first()','leading_update':'#counter(page).update(7)\n#context counter(page).display("1 / 1", both: true)\n#pagebreak()\n#context counter(page).final().first()','middle_update':'#context counter(page).display("1 / 1", both: true)\n#pagebreak()\n#counter(page).update(7)\n#context counter(page).display("1 / 1", both: true)','trailing_step':'#context [B#counter(page).display("1 / 1", both: true) F#counter(page).final().first()]\n#counter(page).step()','bundle':'#document("one.pdf")[#context counter(page).display("1 / 1", both: true)]\n#document("two.pdf")[#counter(page).update(7)\n#context counter(page).final().first() #pagebreak() Second]','mixed':'#document("one.pdf")[#context counter(page).final().first()]\n#document("middle.html")[#counter(page).update(7)\n#context counter(page).final().first()]\n#document("two.pdf")[#context counter(page).display("1 / 1", both: true) #pagebreak() Second]'}
cases.update({
'callback':'#context counter(page).display("1 / 1", both: true)\n#pagebreak()\n#counter(page).update(n => n + 5)\n#context counter(page).final().first()',
'empty_state':'#counter(page).update(())\n#context [G#repr(counter(page).get()) F#repr(counter(page).final())]\n#pagebreak()\n#context [G#repr(counter(page).get()) F#repr(counter(page).final())]',
'multilevel':'#counter(page).update((7, 2))\n#context [G#repr(counter(page).get()) F#repr(counter(page).final())]\n#pagebreak()\n#context [G#repr(counter(page).get()) F#repr(counter(page).final())]',
'empty_margin':'#set page(numbering: "1 / 1")\n#counter(page).update(())\nFirst #pagebreak() Second',
'zero':'#counter(page).update(0)\n#context [G#counter(page).get().first() F#counter(page).final().first()]\n#pagebreak()\n#context counter(page).final().first()',
'last_page_without_read':'#context counter(page).final().first()\n#pagebreak()\nLast',
'html':'#counter(page).update(7)\n#context counter(page).final().first()',
'bundle_callback':'#document("one.pdf")[#context counter(page).display("1 / 1", both: true)]\n#document("two.pdf")[#context counter(page).display("1 / 1", both: true) #pagebreak() #counter(page).update(n => n + 5) #context counter(page).final().first()]',
'bundle_empty_html':'#document("one.pdf")[#context counter(page).final().first()]\n#document("empty.html")[]\n#document("two.pdf")[#context counter(page).display("1 / 1", both: true) #pagebreak() Second]',
'bundle_margin':'#document("one.pdf")[#set page(numbering: (current, total) => [V#current/#total])\n#counter(page).update(10) First #pagebreak() Second]\n#document("two.pdf")[#set page(numbering: (current, total) => [V#current/#total])\nThird #pagebreak() #counter(page).update(7) Fourth]'
})
files=git('diff','--name-only').splitlines()+git('ls-files','--others','--exclude-standard').splitlines();files=[f for f in files if f.endswith(('.rs','.md','.toml')) and f.startswith(('01_core/','03_infra/','00_nucleo/prompts/'))]
d={'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'status':git('status','--short'),'diff_stat':git('diff','HEAD','--stat'),'sources':{f:(R/f).read_text() for f in files},'binary_sha256':{n:sha(p) for n,p in bins.items()},'script':Path(__file__).read_text(),'cases':{}}
for name,src in cases.items():
 p=T/(name+'.typ');p.write_text(src);entry={'source':src,'engines':{}};bundle=name.startswith('bundle') or name=='mixed'
 for engine,exe in bins.items():
  out=T/(name+'-'+engine+('' if bundle else '.html' if name=='html' else '.pdf'));cmd=[str(exe),'compile',str(p),str(out)]+(['--features','bundle,html','--format','bundle'] if bundle else ['--features','html','--format','html'] if name=='html' else []);r=subprocess.run(cmd,capture_output=True,text=True);e={'command':cmd,'returncode':r.returncode,'stderr':r.stderr,'outputs':{}}
  if r.returncode==0:
   paths=sorted(out.rglob('*')) if bundle else [out]
   for x in paths:
    if not x.is_file():continue
    if x.suffix=='.pdf':txt=subprocess.check_output(['pdftotext',str(x),'-'],text=True)
    elif x.suffix=='.html':txt=x.read_text()
    else:continue
    e['outputs'][x.name]={'text':txt,'sha256':sha(x)}
  entry['engines'][engine]=e
 d['cases'][name]=entry;print(name,{n:{k:re.sub(r'\s+','',v['text']) for k,v in e['outputs'].items()} or e['stderr'] for n,e in entry['engines'].items()},flush=True)
from html.parser import HTMLParser
class BodyText(HTMLParser):
 def __init__(self):super().__init__();self.inside=False;self.parts=[]
 def handle_starttag(self,tag,attrs):
  if tag=='body':self.inside=True
 def handle_endtag(self,tag):
  if tag=='body':self.inside=False
 def handle_data(self,data):
  if self.inside:self.parts.append(data)
def normalized(outputs):
 result={}
 for name,item in outputs.items():
  txt=item['text']
  if name.endswith('.html'):
   parser=BodyText();parser.feed(txt);txt=''.join(parser.parts)
  result[name.replace("-tekt.",".").replace("-vanilla.",".")]=re.sub(r'\s+','',txt)
 return result
mismatches=[]
for name,case in d['cases'].items():
 left,right=case['engines']['tekt'],case['engines']['vanilla']
 same=left['returncode']==right['returncode']==0 and normalized(left['outputs'])==normalized(right['outputs'])
 case['language_match']=same
 if not same:mismatches.append(name)
d['mismatches']=mismatches
print('MISMATCHES',mismatches)
d['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(H/'measurement.json.gz','wt') as f:json.dump(d,f,ensure_ascii=False,indent=2)
