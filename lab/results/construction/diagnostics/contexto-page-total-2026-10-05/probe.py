import datetime,gzip,hashlib,json,subprocess,re
from pathlib import Path
H=Path(__file__).resolve().parent;R=H.parents[2];T=Path('/tmp/tekt-page-total');sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();git=lambda *a:subprocess.check_output(['git',*a],cwd=R,text=True)
bins={'tekt':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'}
cases={'single':'#context [B#counter(page).display("1 / 1", both: true) F#counter(page).final().first()]','two':'#context counter(page).display("1 / 1", both: true)\n#pagebreak()\n#context counter(page).final().first()','leading_update':'#counter(page).update(7)\n#context counter(page).display("1 / 1", both: true)\n#pagebreak()\n#context counter(page).final().first()','middle_update':'#context counter(page).display("1 / 1", both: true)\n#pagebreak()\n#counter(page).update(7)\n#context counter(page).display("1 / 1", both: true)','trailing_step':'#context [B#counter(page).display("1 / 1", both: true) F#counter(page).final().first()]\n#counter(page).step()','bundle':'#document("one.pdf")[#context counter(page).display("1 / 1", both: true)]\n#document("two.pdf")[#counter(page).update(7)\n#context counter(page).final().first() #pagebreak() Second]','mixed':'#document("one.pdf")[#context counter(page).final().first()]\n#document("middle.html")[#counter(page).update(7)\n#context counter(page).final().first()]\n#document("two.pdf")[#context counter(page).display("1 / 1", both: true) #pagebreak() Second]'}
files=git('diff','--name-only').splitlines()+git('ls-files','--others','--exclude-standard').splitlines();files=[f for f in files if f.endswith(('.rs','.md','.toml')) and f.startswith(('01_core/','03_infra/','00_nucleo/prompts/'))]
d={'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'status':git('status','--short'),'diff_stat':git('diff','HEAD','--stat'),'sources':{f:(R/f).read_text() for f in files},'binary_sha256':{n:sha(p) for n,p in bins.items()},'script':Path(__file__).read_text(),'cases':{}}
for name,src in cases.items():
 p=T/(name+'.typ');p.write_text(src);entry={'source':src,'engines':{}};bundle=name in ['bundle','mixed']
 for engine,exe in bins.items():
  out=T/(name+'-'+engine+('' if bundle else '.pdf'));cmd=[str(exe),'compile',str(p),str(out)]+(['--features','bundle,html','--format','bundle'] if bundle else []);r=subprocess.run(cmd,capture_output=True,text=True);e={'command':cmd,'returncode':r.returncode,'stderr':r.stderr,'outputs':{}}
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
d['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(H/'measurement.json.gz','wt') as f:json.dump(d,f,ensure_ascii=False,indent=2)
