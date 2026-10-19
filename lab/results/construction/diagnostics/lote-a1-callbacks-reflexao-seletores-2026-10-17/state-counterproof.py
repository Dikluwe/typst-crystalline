from pathlib import Path
import json,gzip,subprocess,datetime,hashlib
p=Path(__file__).resolve().parent;root=p.parents[2];work=p/'outputs';now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
sources={
 'reader-init':'#let a=state("k",1)\n#let b=state("k",10)\n#a.update(n=>n+1)\n#context assert.eq((a.get(),b.get()),(2,11))',
 'other-reader-init':'#let a=state("k",1)\n#a.update(n=>n+1)\n#context assert.eq(state("k",10).final(),11)',
 'unused-callback':'#let s=state("k",0)\n#s.update(n=>panic("UNUSED"))\nA',
 'discarded-update':'#let s=state("k",7)\n#let update=s.update(n=>panic("DISCARDED"))\n#context assert.eq(s.final(),7)',
 'inserted-twice':'#let s=state("k",0)\n#let update=s.update(n=>n+1)\n#update\n#update\n#context assert.eq(s.final(),2)',
 'independent-keys':'#let a=state("a",1)\n#let b=state("b",10)\n#a.update(n=>n+1)\n#context assert.eq((a.final(),b.final()),(2,10))',
 'error-before-set':'#let s=state("k",0)\n#s.update(n=>panic("BEFORE_SET"))\n#s.update(1)\n#context s.final()',
 'display-reader-init':'#let a=state("k",1)\n#a.update(n=>n+1)\n#context state("k",10).display(n=>str(n))',
 'set-shares-update':'#let a=state("k",1)\n#let b=state("k",10)\n#a.update(8)\n#context assert.eq((a.final(),b.final()),(8,8))',
 'legacy-extension':'#state_update("k",1)\n#state_update_with("k",n=>n+1)\n#context state_final("k")',
}
bins={'tekt':root/'target/debug/typst','tekt-before':work/'typst-before','vanilla':root/'lab/typst-original/target/release/typst'};rows=[]
for label,source in sources.items():
 f=work/('state-proof-'+label+'.typ');f.write_text(source+'\n');row={'label':label,'source':source,'source_sha256':hashlib.sha256(f.read_bytes()).hexdigest(),'results':{}}
 for name,binary in bins.items():
  output=work/('state-proof-'+label+'-'+name+'.pdf');cmd=[str(binary),'compile',str(f),str(output)];start=now();r=subprocess.run(cmd,text=True,capture_output=True,timeout=45)
  text=subprocess.run(['pdftotext',str(output),'-'],capture_output=True,text=True).stdout if r.returncode==0 and output.exists() else ''
  row['results'][name]={'command':cmd,'start':start,'end':now(),'exit':r.returncode,'stdout':r.stdout,'stderr':r.stderr,'text':text}
 rows.append(row);print(label,[(n,r['exit']) for n,r in row['results'].items()],flush=True)
(p/'state-counterproof.json.gz').write_bytes(gzip.compress(json.dumps({'utc':now(),'cases':rows},ensure_ascii=False,indent=2).encode()))
