"""Post-window instrumented Tekt control; no comparison to plain CLI medians."""
from pathlib import Path
import importlib.util,json,subprocess,shutil,os,time,threading,datetime,statistics
P=Path(__file__).resolve().parent;R=P.parent;ROOT=R.parents[2]
spec=importlib.util.spec_from_file_location('frozen_driver',R/'run.py');base=importlib.util.module_from_spec(spec);spec.loader.exec_module(base)
art=[]
for line in (P/'build.jsonl').read_text().splitlines():
 try:j=json.loads(line)
 except ValueError:continue
 if j.get('reason')=='compiler-artifact' and j.get('executable') and j['target']['name']=='infra-font-demand-control':art.append(j)
a=art[-1];assert a['profile']['opt_level']=='3' and not a['profile']['debug_assertions'] and not a['profile']['overflow_checks'];exe=R/'bin/infra-font-demand-control';shutil.copy2(a['executable'],exe);digest=base.m.sha(exe)
base.save(P/'artifact.json',{'sha256':digest,'cargo_artifact':a,'runtime_environment':{k:base.ENV.get(k) for k in ['RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','CARGO_BUILD_JOBS','RAYON_NUM_THREADS','SOURCE_DATE_EPOCH','TZ','LC_ALL']}})
frozen=json.loads((R/'frozen-state.json').read_text());before=base.fp();assert before['head']==frozen['head'] and before['sha256']==frozen['sha256']
for fixture in ['shapes','pipeline-single-font','context']:
 cmd=[str(exe),str(R/'fixtures'/(fixture+'.typ')),str(R/'fonts'),str(P/(fixture+'-stack.pdf')),'--stack']
 result=subprocess.run(cmd,cwd=ROOT,env=base.ENV,capture_output=True,text=True,check=True,preexec_fn=lambda:os.sched_setaffinity(0,{4}));(P/(fixture+'-stack.stderr')).write_text(result.stderr);(P/(fixture+'-stack.json')).write_text(result.stdout)
def probe_fp():return {name:base.m.sha(P/name) for name in ['Cargo.toml','Cargo.lock','src/main.rs','run.py']}
probe_source=probe_fp();base.save(P/'source-before.json',probe_source)
all_rows=[]
for round_number in [1,2,3]:
 directory=P/('r'+str(round_number));directory.mkdir(exist_ok=False)
 for _ in range(30):assert not base.blocked(),'concurrent workload during demand-control quiet gate';time.sleep(1)
 pre=base.fp();assert pre['sha256']==frozen['sha256'];assert probe_fp()==probe_source;stop=threading.Event();hits=[]
 def monitor():
  while not stop.wait(.1):hits.extend(base.blocked())
 thread=threading.Thread(target=monitor);thread.start();rows=[];start=base.utc()
 try:
  for fixture in ['shapes','pipeline-single-font','context']:
   for pair in range(4):
    for warm in ([False,True] if (pair+round_number)%2==0 else [True,False]):
     label=f'{fixture}-p{pair}-'+('warm' if warm else 'fresh');output=directory/(label+'.pdf');cmd=[str(exe),str(R/'fixtures'/(fixture+'.typ')),str(R/'fonts'),str(output)]+(['--warm'] if warm else []);utc=base.utc();result=subprocess.run(cmd,cwd=ROOT,env=base.ENV,capture_output=True,text=True,check=True,preexec_fn=lambda:os.sched_setaffinity(0,{4}));(directory/(label+'.stderr')).write_text(result.stderr);row=json.loads(result.stdout);row.update(round=round_number,pair=pair,fixture=fixture,utc=utc,command=cmd,output=str(output.relative_to(R)));rows.append(row)
 finally:stop.set();thread.join()
 post=base.fp();source_after=probe_fp();valid=source_after==probe_source and not hits and pre['sha256']==post['sha256']==frozen['sha256'] and pre['head']==post['head'] and base.m.sha(exe)==digest;base.save(directory/'samples.json',rows);base.save(directory/'receipt.json',{'valid':valid,'start_utc':start,'end_utc':base.utc(),'before':pre['sha256'],'after':post['sha256'],'contamination':hits,'executable_sha256':digest,'probe_sources_before':probe_source,'probe_sources_after':source_after});assert valid,'demand control discarded';all_rows.extend(rows);print('demand r'+str(round_number)+' VALID',flush=True)
summary={}
for fixture in ['shapes','pipeline-single-font','context']:
 groups={}
 for warm in [False,True]:
  rows=[r for r in all_rows if r['fixture']==fixture and r['warm']==warm];groups['warm' if warm else 'fresh']={k:{'mean':statistics.mean(x[k] for x in rows),'median':statistics.median(x[k] for x in rows),'min':min(x[k] for x in rows),'max':max(x[k] for x in rows)} for k in ['pipeline_ms','preload_ms','book_first_ms','book_total_ms','book_calls','book_entries']}
 groups['fresh_first_book_fraction_of_pipeline_mean']=groups['fresh']['book_first_ms']['mean']/groups['fresh']['pipeline_ms']['mean'];groups['warm_preload_plus_pipeline_mean_ms']=groups['warm']['preload_ms']['mean']+groups['warm']['pipeline_ms']['mean'];summary[fixture]=groups
base.save(P/'summary.json',summary)
controls=[]
for fixture in summary:
 for n in [1,2,3]:
  pair=[r for r in all_rows if r['fixture']==fixture and r['round']==n and r['pair']==0];data={}
  for row in pair:
   path=R/row['output'];text=subprocess.check_output(['pdftotext',str(path),'-'],text=True);info=subprocess.check_output(['pdfinfo',str(path)],text=True);data[str(row['warm'])]={'text':' '.join(text.split()),'page_fields':[l for l in info.splitlines() if l.startswith(('Pages:','Page size:'))]}
  equal=data['True']==data['False'];assert equal,data;controls.append({'fixture':fixture,'round':n,'equal':equal,'outputs':data})
base.save(P/'output-controls.json',controls);print(json.dumps(summary,indent=2),flush=True)
