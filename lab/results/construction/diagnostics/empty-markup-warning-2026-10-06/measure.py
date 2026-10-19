from pathlib import Path
import datetime,subprocess,hashlib,json,gzip,os
R=Path.cwd();H=R/'00_nucleo/diagnosticos/empty-markup-warning-2026-10-06';T=Path('/tmp/tekt-empty-warning/paired');project=T/'project';cwd=T/'cwd';project.mkdir(parents=True,exist_ok=True);cwd.mkdir(exist_ok=True)
main=project/'main.typ'
bins={'before':Path('/tmp/tekt-source-identity/typst-final'),'tekt':Path('/tmp/tekt-empty-warning/typst-final'),'vanilla':R/'lab/typst-original/target/release/typst'}
cases={
'document-error':{'source':'#document_missing','expr':'0'},
'document-syntax':{'source':'#let x = (','expr':'0'},
'document-warning':{'source':'**\n= Document','expr':'0'},
'expression-error':{'source':'= Document','expr':'expression_missing'},
'expression-syntax':{'source':'= Document','expr':'('},
'warning-and-expression-error':{'source':'**\n= Document','expr':'expression_missing'},
'stdin-document-error':{'stdin':'#stdin_missing','expr':'0'},
'stdin-expression-error':{'stdin':'= Document','expr':'expression_missing'},
'stdin-query':{'stdin':'= Document','expr':'query(heading).len()'},
'isolated-error':{'expr':'expression_missing'},
'import-document-error':{'source':'#import "lib.typ": broken\n#broken()','extra':{'lib.typ':'#let broken() = import_missing'},'expr':'0'},
'import-expression-error':{'source':'= Document','extra':{'lib.typ':'#let broken() = import_missing'},'expr':'{ import "lib.typ": broken; broken() }'},
'callback-document-origin':{'source':'#let broken() = callback_missing\n#metadata(broken)<f>','expr':'(query(<f>).first().value)()'},
'relative-read':{'source':'= Document','extra':{'asset.txt':'project'},'expr':'read("asset.txt")'},
'relative-import':{'source':'= Document','extra':{'lib.typ':'#let value = 42'},'expr':'{ import "lib.typ": value; value }'},
'context-query':{'source':'= Document','expr':'query(heading).len()'},
}
git=lambda *a:subprocess.check_output(['git',*a],text=True)
files=['01_core/src/compiler/eval/mod.rs','03_infra/src/world.rs','03_infra/src/world/cache.rs','03_infra/src/pipeline.rs','04_wiring/src/eval.rs']
d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'working_tree':'uncommitted','diff_stat':git('diff','HEAD','--stat'),'status':git('status','--short'),'files':{p:(R/p).read_text() for p in files},'binaries':{k:hashlib.sha256(v.read_bytes()).hexdigest() for k,v in bins.items()},'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','cases':{}}
for name,c in cases.items():
 if 'source' in c:main.write_text(c['source'])
 for path,text in c.get('extra',{}).items():(project/path).write_text(text)
 (cwd/'asset.txt').write_text('wrong base')
 args=['eval']
 if 'source' in c:args+=['--in',str(main)]
 if 'stdin' in c:args+=['--in','-']
 args+=[c['expr']]
 case={'cwd':str(cwd),'fixture':c,'engines':{}}
 for key,bin in bins.items():
  cmd=[str(bin),*args];r=subprocess.run(cmd,cwd=cwd,input=c.get('stdin'),capture_output=True,text=True,timeout=45,env=dict(os.environ,NO_COLOR='1',SOURCE_DATE_EPOCH='0',RAYON_NUM_THREADS='1'))
  case['engines'][key]={'command':cmd,'returncode':r.returncode,'stdout':r.stdout,'stderr':r.stderr}
 compare=lambda a,b: all(a[f]==b[f] for f in ['returncode','stdout','stderr'])
 case['exact_match']=compare(case['engines']['tekt'],case['engines']['vanilla']);case['before_match']=compare(case['engines']['before'],case['engines']['vanilla']);d['cases'][name]=case
 print(name,case['before_match'],'->',case['exact_match'],flush=True)
d['finished_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(H/'measurement.json.gz','wt') as f:json.dump(d,f,ensure_ascii=False)
(H/'paired-summary.json').write_text(json.dumps({'exact':sum(c['exact_match'] for c in d['cases'].values()),'before_exact':sum(c['before_match'] for c in d['cases'].values()),'total':len(cases),'remaining':[n for n,c in d['cases'].items() if not c['exact_match']]},indent=2)+'\n')
