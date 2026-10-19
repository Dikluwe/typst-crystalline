from pathlib import Path
import json,datetime,hashlib,subprocess,os,time,statistics,random
R=Path.cwd();H=R/'00_nucleo/diagnosticos/eixos-variaveis-export-2026-10-05';T=Path('/tmp/tekt-axes');bins={'before':T/'typst-before','after':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'}
fixtures={'grid320':'#table(columns: 8,'+','.join('[cell '+str(i)+']' for i in range(320))+')', 'variable-axes': json.loads((H/'fixtures.json').read_text())['variable-axes'], 'variable-cff2': json.loads((H/'fixtures.json').read_text())['variable-cff2']}
env=dict(os.environ,SOURCE_DATE_EPOCH='0',RAYON_NUM_THREADS='1');d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'binaries':{k:hashlib.sha256(p.read_bytes()).hexdigest() for k,p in bins.items()},'cases':{},'status':subprocess.check_output(['git','status','--short'],text=True),'source_hashes':{x['source']:hashlib.sha256((R/x['source']).read_bytes()).hexdigest() for x in json.loads((H/'scope.json').read_text())},'measurement':'Entire CLI wall clock; 2 warmups and 15 randomized paired rounds; same font/catalog env; no phase attribution'}
for name,source in fixtures.items():
 fp=T/('bench-'+name+'.typ');fp.write_text(source);cmds={k:[str(b),'compile',str(fp),str(T/('bench-'+name+'-'+k+'.pdf')),'--pdf-tagged=false','--pretty=false'] for k,b in bins.items()};samples={k:[] for k in bins};rounds=[]
 if name.startswith("variable-"):
  for cmd in cmds.values():cmd += ["--font-path",str(R/"03_infra/fixtures/fonts")]
 for i in range(17):
  order=list(bins);random.Random(730+i).shuffle(order);one={}
  for k in order:
   start=time.perf_counter_ns();run=subprocess.run(cmds[k],env=env,capture_output=True,text=True);elapsed=(time.perf_counter_ns()-start)/1e6;assert run.returncode==0,run.stderr
   if i>=2:samples[k].append(elapsed);one[k]=elapsed
  if i>=2:rounds.append(one)
 paired=[v['after']-v['before'] for v in rounds];case={'source':source,'commands':cmds,'samples_ms':samples,'paired_rounds':rounds,'median_ms':{k:statistics.median(v) for k,v in samples.items()},'paired_delta_median_ms':statistics.median(paired),'paired_delta_range_ms':[min(paired),max(paired)]};d['cases'][name]=case;print(name,case['median_ms'],'paired delta',case['paired_delta_median_ms'],flush=True)
(H/'benchmark.json').write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n');(H/'bench.py').write_text(Path(__file__).read_text())
