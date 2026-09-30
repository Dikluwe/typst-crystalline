from pathlib import Path
import json,subprocess,hashlib,os,time,threading,datetime,shutil,statistics,tarfile
ROOT=Path(__file__).resolve().parents[3];R=Path(__file__).resolve().parent
sha=lambda p:hashlib.file_digest(p.open('rb'),'sha256').hexdigest()
def save(p,x):p.write_text(json.dumps(x,indent=2)+'\n')
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def fingerprint():
 files=[p for base in ['01_core','02_shell','03_infra','04_wiring','benches','00_nucleo/prompts'] for p in (ROOT/base).rglob('*') if p.is_file() and not set(p.parts)&{'target','__pycache__'}]
 files += [ROOT/'Cargo.toml',ROOT/'Cargo.lock',R/'Cargo.toml',R/'Cargo.lock',R/'src/main.rs',R/'main.typ',R/'run.py',R/'bin/microbench']
 return {str(p.relative_to(ROOT)):sha(p) for p in files}
active=set();lock=threading.Lock();hits=[];stop=threading.Event()
def competitors():
 rows=[]
 for p in Path('/proc').iterdir():
  if not p.name.isdigit():continue
  try:
   name=(p/'comm').read_text().strip()
   if int(p.name) not in active and (name in ['cargo','rustc','rustdoc','typst','microbench'] or name.endswith('_bench') or name.startswith(('place-font','place-metrics','infra-font','font-microbench','font-vanilla','typst_core','typst_infra','typst_wiring'))):rows.append({'pid':int(p.name),'name':name,'cmd':(p/'cmdline').read_bytes().replace(b'\0',b' ').decode(errors='replace')})
  except (FileNotFoundError,PermissionError,ProcessLookupError):pass
 return rows
def monitor():
 while not stop.is_set():
  with lock:found=competitors()
  if found:hits.append({'utc':utc(),'processes':found})
  stop.wait(.1)
def launch(cmd,label):
 with lock:
  assert not competitors(),'concurrent build/test/benchmark'
  p=subprocess.Popen(cmd,cwd=ROOT,env=ENV,stdout=subprocess.PIPE,stderr=subprocess.PIPE);active.add(p.pid)
 out,err=p.communicate()
 with lock:active.remove(p.pid)
 (R/(label+'.stdout')).write_bytes(out);(R/(label+'.stderr')).write_bytes(err)
 save(R/(label+'.receipt.json'),{'utc':utc(),'command':cmd,'returncode':p.returncode})
 assert p.returncode==0,err.decode(errors='replace')
ENV=os.environ.copy()
for k in list(ENV):
 if k.startswith(('TYPST_','CRYSTALLINE_')):ENV.pop(k)
ENV['RAYON_NUM_THREADS']='1'
assert not (R/'round-receipts.json').exists(),'preserve recorded evidence'
artifacts=[json.loads(l) for l in (R/'build-final.jsonl').read_text().splitlines() if l.startswith('{')]
a=[a for a in artifacts if a.get('reason')=='compiler-artifact' and a.get('executable') and a['target']['name']=='place-metrics-microbench-diagnostic'][-1]
assert a['profile']['opt_level']=='3' and not a['profile']['debug_assertions'] and not a['profile']['overflow_checks']
(R/'bin').mkdir();shutil.copy2(a['executable'],R/'bin/microbench');save(R/'artifact.json',{'artifact':a,'sha256':sha(R/'bin/microbench'),'recipe':"RUSTFLAGS='' CARGO_ENCODED_RUSTFLAGS='' cargo build --release --offline --locked --manifest-path 00_nucleo/diagnosticos/place-metrics-microbenchmarks/Cargo.toml --target-dir target -j 1 --message-format=json"})
initial=fingerprint();save(R/'initial-fingerprints.json',initial)
with tarfile.open(R/'measured-sources.tar.gz','w:gz') as t:
 for rel in initial:
  if '/bin/' not in rel:t.add(ROOT/rel,arcname=rel,recursive=False)
thread=threading.Thread(target=monitor,daemon=True);thread.start();receipts=[]
try:
 launch([str(R/'bin/microbench'),str(R),str(R/'smoke'),'--smoke'],'smoke')
 assert not hits,'contamination during setup'
 for _ in range(50):
  with lock:assert not competitors(),'quiet gate failed'
  time.sleep(.1)
 for n in range(3):
  pre=fingerprint();start=len(hits);dest=R/f'round{n}'
  cmd=[str(R/'bin/microbench'),str(R),str(dest)]+(['--reverse'] if n==1 else [])
  launch(cmd,f'round{n}');post=fingerprint();valid=pre==post==initial and not hits[start:]
  save(R/f'round{n}-fingerprints.json',{'before':pre,'after':post})
  receipts.append({'round':n,'utc':utc(),'accepted':valid,'competitors':hits[start:],'loadavg':os.getloadavg()});save(R/'round-receipts.json',receipts)
  assert valid,'discarded collection round'
  print('round',n,'accepted',flush=True)
finally:
 stop.set();thread.join();save(R/'process-monitor.json',hits)
rows={}
for n in range(3):
 for path in (R/f'round{n}').rglob('new/sample.json'):
  key=str(path.parent.parent.relative_to(R/f'round{n}'));sample=json.loads(path.read_text());times=[t/i for t,i in zip(sample['times'],sample['iters'])];rows.setdefault(key,[]).append({'round':n,'ns_per_iteration':times,'sample_file':str(path.relative_to(R))})
assert len(rows)==15,len(rows)
summary={}
for key,rounds in rows.items():
 assert len(rounds)==3
 vals=[v for r in rounds for v in r['ns_per_iteration']];mean=statistics.mean(vals);median=statistics.median(vals);cv=statistics.stdev(vals)/mean;medians=[statistics.median(r['ns_per_iteration']) for r in rounds]
 summary[key]={'n':len(vals),'mean_ns':mean,'median_ns':median,'min_ns':min(vals),'max_ns':max(vals),'stdev_ns':statistics.stdev(vals),'cv':cv,'round_medians_ns':medians,'stable':cv<=.2 and all(abs(m/median-1)<=.1 for m in medians),'rounds':rounds}
save(R/'summary.json',summary);final=fingerprint();assert final==initial;save(R/'final-fingerprints.json',final)
setup=[json.loads((R/f'round{n}/setup.json').read_text()) for n in range(3)];assert all(s==setup[0] for s in setup);save(R/'setup-consistency.json',setup[0])
print('stable',sum(s['stable'] for s in summary.values()),'of',len(summary),flush=True)
