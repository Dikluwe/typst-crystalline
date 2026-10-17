from pathlib import Path
import json,gzip,datetime,subprocess,hashlib,os
R=Path.cwd();H=R/'00_nucleo/diagnosticos/diagnostic-paths-2026-10-06';T=Path('/tmp/tekt-diagnostic-paths/paired');project=T/'project/src';work=T/'work/a';project.mkdir(parents=True,exist_ok=True);work.mkdir(parents=True,exist_ok=True);(project/'deep/a').mkdir(parents=True,exist_ok=True)
main=project/'main.typ';lib=project/'lib.typ';special=project/'sub pasta/ação.typ';special.parent.mkdir(exist_ok=True)
bins={'tekt':Path('/tmp/tekt-diagnostic-paths/typst-final'),'vanilla':R/'lab/typst-original/target/release/typst'}
cases={
'absolute-sibling':{'cwd':work,'source':'#unknown','file':main,'args':['compile',str(main),str(T/'out.pdf')]},
'relative-sibling':{'cwd':work,'source':'#unknown','file':main,'args':['compile','../../project/src/main.typ',str(T/'out.pdf')]},
'inside-cwd':{'cwd':project,'source':'#unknown','file':main,'args':['compile','main.typ',str(T/'out.pdf')]},
'absolute-inside-cwd':{'cwd':project,'source':'#unknown','file':main,'args':['compile',str(main),str(T/'out.pdf')]},
'ancestor':{'cwd':project/'deep/a','source':'#unknown','file':main,'args':['compile',str(main),str(T/'out.pdf')]},
'import-and-trace':{'cwd':work,'source':'#import "lib.typ": broken\n#broken(1)','file':main,'extra':{'lib.typ':'#let broken(x) = x + missing'},'args':['compile',str(main),str(T/'out.pdf')]},
'query':{'cwd':work,'source':'#unknown','file':main,'args':['query',str(main),'heading']},
'eval-in':{'cwd':work,'source':'= Valid','file':main,'args':['eval','--in',str(main),'unknown']},
'eval-in-relative':{'cwd':work,'source':'= Valid','file':main,'args':['eval','--in','../../project/src/main.typ','unknown']},
'unicode-space':{'cwd':work,'source':'#unknown','file':special,'args':['compile',str(special),str(T/'out.pdf')]},
'stdin-compile':{'cwd':work,'stdin':'#unknown','args':['compile','-',str(T/'out.pdf')]},
'stdin-eval':{'cwd':work,'stdin':'= Valid','args':['eval','--in','-','unknown']},
'eval-no-file':{'cwd':work,'args':['eval','unknown']},
'warning-before-error':{'cwd':work,'source':'**\n#unknown','file':main,'args':['compile',str(main),str(T/'out.pdf')]},
'no-error':{'cwd':work,'source':'= Valid','file':main,'args':['compile',str(main),str(T/'out.pdf')]},
}
git=lambda *a:subprocess.check_output(['git',*a],text=True)
files=['04_wiring/src/diagnostics.rs','04_wiring/tests/cli.rs','04_wiring/Cargo.toml','Cargo.lock','00_nucleo/prompts/wiring/diagnostics.md','00_nucleo/prompts/wiring/tests/cli.md']
d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'diff_stat':git('diff','HEAD','--stat'),'status':git('status','--short'),'files':{p:(R/p).read_text() for p in files},'binaries':{k:hashlib.sha256(b.read_bytes()).hexdigest() for k,b in bins.items()},'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','cases':{}}
for n,c in cases.items():
 if 'file' in c:c['file'].write_text(c['source'])
 for p,s in c.get('extra',{}).items():(project/p).write_text(s)
 e={'cwd':str(c['cwd']),'source':c.get('source'), 'stdin':c.get('stdin'),'engines':{}}
 for k,b in bins.items():
  cmd=[str(b),*c['args']];r=subprocess.run(cmd,cwd=c['cwd'],input=c.get('stdin'),capture_output=True,text=True,timeout=45,env=dict(os.environ,NO_COLOR='1',SOURCE_DATE_EPOCH='0',RAYON_NUM_THREADS='1'));e['engines'][k]={'command':cmd,'returncode':r.returncode,'stderr':r.stderr,'stdout':r.stdout}
 a,b=e['engines']['tekt'],e['engines']['vanilla'];e['exact_diagnostic_match']=a['returncode']==b['returncode'] and a['stderr']==b['stderr'];d['cases'][n]=e;print(n,e['exact_diagnostic_match'],flush=True)
d['finished_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(H/'measurement.json.gz','wt') as f:json.dump(d,f,ensure_ascii=False)
