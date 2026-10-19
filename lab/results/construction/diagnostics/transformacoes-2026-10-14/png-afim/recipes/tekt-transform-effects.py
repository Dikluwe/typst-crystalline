from pathlib import Path
import json,gzip,hashlib,subprocess,datetime
root=Path.cwd();base=root/'00_nucleo/diagnosticos/transformacoes-2026-10-14';out=base/'measurement-controls-corrected';out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
bins={'tekt':root/'target/debug/typst','vanilla':root/'lab/typst-original/target/release/typst'}
files={str(p):sha(p) for top in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'] for p in Path(top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']}
r=dict(start=now(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files=files,binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in bins.items()},rows=[])
cases={
'literal-trailing':'#context [#metadata(repr((measure(text("ONE TWO")).width,measure(text("ONE TWO ")).width))) <probe>]',
'markup-trailing':'#context [#metadata(repr((measure([ONE TWO]).width,measure([ONE TWO ]).width))) <probe>]',
'constrained':'#context [#metadata(repr(measure([ONE TWO THREE FOUR],width:50pt))) <probe>]',
'overlong':'#context [#metadata(repr(measure([THREE],width:25pt))) <probe>]',
'oversized-shape':'#context [#metadata(repr(measure(rect(width:100pt,height:20pt),width:25pt))) <probe>]',
'state-once':'#let s=state("transform-probe",0)\n#scale(200%,reflow:true)[#s.update(n=>n+1)TEXT]\n#context [#metadata(s.get()) <probe>]',
'label-once':'#scale(x:50pt,y:auto,reflow:true)[TEXT <mark>]\n#context [#metadata(query(<mark>).len()) <probe>]',
}
for name,source in cases.items():
 p=out/(name+'.typ');p.write_text(source)
 for key,exe in bins.items():
  cmd=[str(exe),'query',str(p),'<probe>','--field','value','--one'];q=subprocess.run(cmd,capture_output=True,text=True,timeout=30);r['rows'].append(dict(case=name,binary=key,command=cmd,source=source,exit=q.returncode,stdout=q.stdout,stderr=q.stderr));print(name,key,q.returncode,q.stdout.strip(),q.stderr.splitlines()[:1],flush=True)
r['end']=now();r['source_guard']=all(sha(p)==h for p,h in files.items());assert r['source_guard']
with gzip.open(out/'results.json.gz','wt') as f:json.dump(r,f)
