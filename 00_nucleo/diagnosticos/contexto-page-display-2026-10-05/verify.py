import datetime,gzip,hashlib,json,subprocess,re
from pathlib import Path
H=Path(__file__).resolve().parent;R=H.parents[2];T=Path('/tmp/tekt-page-display');sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();git=lambda *a:subprocess.check_output(['git',*a],cwd=R,text=True)
bins={'before':T/'baseline-typst','after':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'}
cases={'minimal':'#context counter(page).display()\n','two_pages':'#context counter(page).display()\n#pagebreak()\n#context counter(page).display()','updated':'#counter(page).update(7)\n#context counter(page).display()\n#pagebreak()\n#context counter(page).display()','pattern':'#context counter(page).display("i")\n#pagebreak()\n#context counter(page).display("i")','callback':'#context counter(page).display(n => [Page #n])','agreement':'#context [D#counter(page).display() G#counter(page).get().first() A#counter(page).at(here()).first()]','custom':'#counter("custom").update(3)\n#context counter("custom").display("1")','total_open':'#context counter(page).display("1 / 1", both: true)'}
d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'status':git('status','--short'),'diff_stat':git('diff','HEAD','--stat'),'binary_sha256':{n:sha(p) for n,p in bins.items()},'cases':{}}
for name,src in cases.items():
 p=T/(name+'.typ');p.write_text(src);entry={'source':src,'engines':{}}
 for engine,exe in bins.items():
  pdf=T/(name+'-'+engine+'.pdf');cmd=[str(exe),'compile',str(p),str(pdf)];r=subprocess.run(cmd,capture_output=True,text=True);assert r.returncode==0,(name,engine,r.stderr);text=subprocess.check_output(['pdftotext',str(pdf),'-'],text=True);entry['engines'][engine]={'command':cmd,'stderr':r.stderr,'text':text,'pages':[re.sub(r'\s+','',x) for x in text.split('\f') if x.strip()],'pdf_sha256':sha(pdf)}
 if name!='total_open':assert entry['engines']['after']['pages']==entry['engines']['vanilla']['pages'],entry
 d['cases'][name]=entry;print(name,{n:v['pages'] for n,v in entry['engines'].items()},flush=True)
d['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(H/'verification.json.gz','wt') as f:json.dump(d,f,ensure_ascii=False,indent=2)
