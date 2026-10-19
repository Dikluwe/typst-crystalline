"""Current fixed-cost audit: unmodified CLI, diagnostic World and catalog composition."""
import datetime,gzip,hashlib,importlib.util,json,os,statistics,subprocess,time
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[2];OUT=Path('/tmp/tekt-fixed-cost-2026-10-05');CPU=min(os.sched_getaffinity(0))
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
git=lambda *a:subprocess.check_output(['git',*a],cwd=ROOT,text=True)
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def capture(cmd):
 start=time.perf_counter_ns();p=subprocess.run(cmd,cwd=ROOT,env=dict(os.environ,RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',NO_COLOR='1'),capture_output=True,text=True,timeout=60,preexec_fn=lambda:os.sched_setaffinity(0,{CPU}));end=now()
 assert p.returncode==0,(cmd,p.stdout,p.stderr)
 return {'command':list(map(str,cmd)),'wall_ms':(time.perf_counter_ns()-start)/1e6,'utc':end,'stdout':p.stdout,'stderr':p.stderr}
spec=importlib.util.spec_from_file_location('macro',HERE.parent/'macro-micro-2026-10-05/probe.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
fixtures={n:p.read_text() for n,p in m.sources().items() if n in ['hello','grid320','context','eval30000']}
fixtures['empty']='';fixtures['shapes']=(HERE.parent/'infra-performance-followup/fixtures/shapes.typ').read_text();fixtures['image']=(HERE.parent/'infra-performance-followup/fixtures/image.typ').read_text();fixtures['unicode']='#set text(font: "Libertinus Serif")\nOlá Ελληνικά हिन्दी العربية 中文\n'
(OUT/'tiny.jpg').write_bytes((HERE.parent/'infra-performance-followup/fixtures/tiny.jpg').read_bytes())
for n,s in fixtures.items():(OUT/(n+'.typ')).write_text(s)
binaries={'tekt':OUT/'typst-baseline','vanilla':ROOT/'lab/typst-original/target/release/typst','probe':ROOT/'target/release/fixed-cost-probe'}
files=[ROOT/n for n in git('diff','--name-only').splitlines() if n.endswith(('.rs','.md','.toml'))]+[ROOT/n for n in git('ls-files','--others','--exclude-standard').splitlines() if n.startswith(('01_core/','00_nucleo/prompts/'))]
r={'started':now(),'head':git('rev-parse','HEAD').strip(),'status':git('status','--short'),'diff_stat':git('diff','HEAD','--stat'),'source_hashes':{str(p.relative_to(ROOT)):sha(p) for p in files},'helper_hashes':{str(p.relative_to(ROOT)):sha(p) for p in HERE.glob('probe/**/*') if p.is_file()},'binaries':{n:{'path':str(p),'sha256':sha(p)} for n,p in binaries.items()},'cpu':CPU,'fixtures':fixtures,'cli':[],'pipeline':[],'catalog':[]}
for name in fixtures:
 for j in range(-1,9):
  for engine in (['tekt','vanilla'] if j%2==0 else ['vanilla','tekt']):
   output=OUT/f'{name}-{engine}.pdf';trace=OUT/f'{name}-{engine}-timings.json';usage=OUT/f'{name}-{engine}.usage'
   cmd=['/usr/bin/time','-f','%M %U %S','-o',str(usage),str(binaries[engine]),'compile',str(OUT/(name+'.typ')),str(output),'--root',str(OUT),'--pdf-tagged=false','--pretty=false']
   if engine=='tekt':cmd+=['--timings-json',str(trace)]
   else:cmd+=['--jobs','1']
   row=capture(cmd);rss,user,system=usage.read_text().split();row.update(fixture=name,engine=engine,index=j,max_rss_kib=int(rss),cpu_s_coarse=float(user)+float(system),pdf_sha256=sha(output),text=subprocess.check_output(['pdftotext',str(output),'-'],text=True));row.pop('stdout')
   if engine=='tekt':row['timings']=json.loads(trace.read_text())
   if j>=0:r['cli'].append(row)
for name in ['empty','hello','grid320','shapes','unicode']:
 for j in range(-1,5):
  for warm in ([False,True] if j%2==0 else [True,False]):
   cmd=[str(binaries['probe']),str(OUT/(name+'.typ')),str(OUT/f'{name}-probe-{warm}.pdf')]+(['--warm'] if warm else [])
   row=capture(cmd);row['measurement']=json.loads(row.pop('stdout'));row.update(fixture=name,index=j)
   if j>=0:r['pipeline'].append(row)
for j in range(-1,5):
 row=capture([str(binaries['probe']),'catalog']);row['measurement']=json.loads(row.pop('stdout'));row['index']=j
 if j>=0:r['catalog'].append(row)
r['fonts']={p:sha(Path(p)) for p in sorted({x['path'] for x in r['catalog'][0]['measurement']['rows']})}
r['finished']=now()
with gzip.open(HERE/'audit.json.gz','wt') as f:json.dump(r,f,ensure_ascii=False,indent=2)
for n in fixtures:
 print(n,{e:round(statistics.median(x['wall_ms'] for x in r['cli'] if x['fixture']==n and x['engine']==e),2) for e in ['tekt','vanilla']})
for n in ['empty','hello','grid320','shapes','unicode']:
 rows=[x['measurement'] for x in r['pipeline'] if x['fixture']==n and not x['measurement']['warm']]
 print('pipeline',n,{k:round(statistics.median(x[k] for x in rows),3) for k in ['setup_ms','pipeline_ms','book_first_ms','book_calls','write_ms']})
rows=[x['measurement'] for x in r['catalog']];print('catalog',{k:round(statistics.median(x[k] for x in rows),3) for k in ['total_ms','embedded_ms','discovery_ms','processing_ms','database_drop_ms']})
for k in ['metadata_ms','second_parse_ms','enumerate_ms','compress_ms']:print(k,statistics.median(sum(y[k] for y in x['rows']) for x in rows))
