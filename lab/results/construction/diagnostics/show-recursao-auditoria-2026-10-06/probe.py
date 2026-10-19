from pathlib import Path
import json,gzip,datetime,hashlib,subprocess,os
R=Path.cwd();H=R/'00_nucleo/diagnosticos/show-recursao-auditoria-2026-10-06';T=Path('/tmp/tekt-show-revisit');bins={'tekt':Path('/tmp/tekt-stroke-none/typst-before'),'vanilla':R/'lab/typst-original/target/release/typst'}
cases={'alias':'#show heading: it => it\n= Original','alias_twice':'#show heading: it => [#it #it]\n= Original','bounded_generated':'#show heading.where(level: 1): it => [#it #heading(level: 2)[Generated]]\n= Original','fresh_same_body':'#show heading: it => heading(it.body)\n= Original','fresh_z':'#show heading: it => heading[Z]\n= Original','alias_plus_generated':'#show heading: it => [#it #heading(level: 2)[Generated]]\n= Original','text_and_generated':'#show heading: it => [Prefix #heading[Z]]\n= Original','bounded_replacement':'#show heading.where(level: 1): it => heading(level: 2)[Z]\n= Original'}
files=['01_core/src/compiler/eval/rules/apply.rs','01_core/src/compiler/eval/show_rule_termination.rs','01_core/src/compiler/realize/mod.rs','00_nucleo/prompts/compiler/eval/rules/apply.md','00_nucleo/prompts/compiler/eval/show_rule_termination.md','00_nucleo/prompts/compiler/realize/mod.md'];git=lambda *a:subprocess.check_output(['git',*a],text=True)
d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'diff_stat':git('diff','HEAD','--stat'),'status':git('status','--short'),'files':{f:(R/f).read_text() for f in files},'binaries':{k:hashlib.sha256(v.read_bytes()).hexdigest() for k,v in bins.items()},'cases':{}}
for n,src in cases.items():
 fp=T/(n+'.typ');fp.write_text('#set text(font: "DejaVu Sans")\n'+src);entry={'source':fp.read_text(),'engines':{}}
 for k,b in bins.items():
  pdf=T/(n+'-'+k+'.pdf');cmd=[str(b),'compile',str(fp),str(pdf),'--pdf-tagged=false','--pretty=false'];r=subprocess.run(cmd,capture_output=True,text=True,env=dict(os.environ,SOURCE_DATE_EPOCH='0',RAYON_NUM_THREADS='1'));v={'command':cmd,'returncode':r.returncode,'stderr':r.stderr}
  if not r.returncode:
   v['text']=subprocess.check_output(['pdftotext',str(pdf),'-'],text=True);v['pdf_sha256']=hashlib.sha256(pdf.read_bytes()).hexdigest();cmd=[str(b),'query',str(fp),'heading','--field','body'];q=subprocess.run(cmd,capture_output=True,text=True);v['query_command']=cmd;v['query_stdout']=q.stdout;v['query_stderr']=q.stderr;v['query_returncode']=q.returncode
  entry['engines'][k]=v
 d['cases'][n]=entry;print(n,{k:v['returncode'] for k,v in entry['engines'].items()},flush=True)
with gzip.open(H/'measurement.json.gz','wt') as f:json.dump(d,f,ensure_ascii=False)
(H/'probe.py').write_text(Path(__file__).read_text())
