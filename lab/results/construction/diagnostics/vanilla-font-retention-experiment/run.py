from pathlib import Path
import json,hashlib,os,threading,time,datetime,subprocess,shutil,tarfile
ROOT=Path(__file__).resolve().parents[3];R=Path(__file__).resolve().parent;LAB=ROOT/'lab/vanilla-font-retention'
def sha(p):
 with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
def save(name,x):(R/name).write_text(json.dumps(x,indent=2)+'\n')
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
ENV=os.environ.copy()
for k in list(ENV):
 if k.startswith(('TYPST_','CRYSTALLINE_')):ENV.pop(k)
ENV['RAYON_NUM_THREADS']='1'
assert not (R/'round-receipts.json').exists(),'preserve admitted evidence'
art=[json.loads(l) for l in (R/'build-ready.jsonl').read_text().splitlines() if l.startswith('{')]
a=[x for x in art if x.get('reason')=='compiler-artifact' and x.get('executable') and x['target']['name']=='vanilla-font-retention-lab'][-1]
assert a['profile']['opt_level']=='3' and not a['profile']['debug_assertions'] and not a['profile']['overflow_checks']
(R/'bin').mkdir();shutil.copy2(a['executable'],R/'bin/experiment');EXE=R/'bin/experiment'
save('artifact.json',{'artifact':a,'sha256':sha(EXE),'recipe':"RUSTFLAGS='' CARGO_ENCODED_RUSTFLAGS='' cargo build --release --offline --locked --manifest-path lab/vanilla-font-retention/Cargo.toml --target-dir target -j 1 --message-format=json"})
vanilla=ROOT/'lab/typst-original/target/release/typst'
assert sha(vanilla)=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
font_roots=[Path('/usr/share/fonts'),Path('/usr/local/share/fonts'),Path.home()/'.local/share/fonts',Path.home()/'.fonts',Path('/etc/fonts')]
font_files=set(p for base in font_roots if base.exists() for p in base.rglob('*') if p.is_file())
# Include configured font files even when fontconfig points outside common roots.
listed=subprocess.check_output(['fc-list','--format','%{file}\n'],text=True)
(R/'fontconfig-list.txt').write_text(listed)
font_files.update(Path(l) for l in listed.splitlines() if Path(l).is_file())
save('font-inventory.json',{str(p):sha(p) for p in sorted(font_files)})
def fingerprint():
 files=[p for base in ['01_core','02_shell','03_infra','04_wiring','benches','00_nucleo/prompts','lab/vanilla-font-retention'] for p in (ROOT/base).rglob('*') if p.is_file() and not set(p.parts)&{'target','__pycache__'}]
 files += [ROOT/'Cargo.toml',ROOT/'Cargo.lock',EXE,R/'run.py',R/'protocol.md',vanilla]
 files += list((R/'inputs').rglob('*.typ'))
 files += [ROOT/'lab/typst-original/crates/typst-cli/src/world.rs',ROOT/'lab/typst-original/crates/typst-cli/src/watch.rs',ROOT/'lab/typst-original/crates/typst-kit/src/fonts.rs']
 files += list(font_files)
 return {str(p):sha(p) for p in files}
initial=fingerprint();save('initial-fingerprints.json',initial)
with tarfile.open(R/'measured-sources.tar.gz','w:gz') as archive:
 for rel in json.loads((R/'before-sources.json').read_text()):archive.add(ROOT/rel,arcname=rel,recursive=False)
 for p in LAB.rglob('*'):
  if p.is_file():archive.add(p,arcname=str(p.relative_to(ROOT)),recursive=False)
active=set();lock=threading.Lock();stop=threading.Event();hits=[]
def competitors():
 result=[]
 for p in Path('/proc').iterdir():
  if not p.name.isdigit():continue
  try:
   name=(p/'comm').read_text().strip()
   if int(p.name) not in active and (name in ['cargo','rustc','rustdoc','typst','experiment','microbench'] or name.endswith('_bench') or name.startswith(('vanilla-font','place-font','place-metrics','infra-font','font-microbench','font-vanilla','typst_core','typst_infra','typst_wiring'))):result.append({'pid':int(p.name),'name':name,'cmd':(p/'cmdline').read_bytes().replace(b'\0',b' ').decode(errors='replace')})
  except (FileNotFoundError,PermissionError,ProcessLookupError):pass
 return result
def monitor():
 while not stop.is_set():
  with lock:found=competitors()
  if found:hits.append({'utc':utc(),'processes':found})
  stop.wait(.1)
def launch(case,policy,label):
 dest=R/label;dest.mkdir(parents=True)
 command=[str(EXE),str(R/'inputs'/case),str(dest),policy]
 with lock:
  assert not competitors(),'concurrent workload'
  start=time.perf_counter();p=subprocess.Popen(command,cwd=ROOT,env=ENV,stdout=subprocess.PIPE,stderr=subprocess.PIPE);active.add(p.pid)
 out,err=p.communicate();wall_ms=(time.perf_counter()-start)*1000
 with lock:active.remove(p.pid)
 (dest/'stdout.json').write_bytes(out);(dest/'stderr.txt').write_bytes(err)
 save(label+'/receipt.json',{'utc':utc(),'command':command,'returncode':p.returncode,'launcher_wall_ms':wall_ms})
 assert p.returncode==0,err.decode(errors='replace')
 return json.loads(out)
# Compile all direct and adapter policy output controls before admission.
for case in ['latin','dense','placed-shape']:
 for policy in ['direct','fresh','retained','prewarmed']:launch(case,policy,'controls/'+case+'/'+policy)
assert fingerprint()==initial,'controls changed inputs'
thread=threading.Thread(target=monitor,daemon=True);thread.start();receipts=[];rows=[]
try:
 for _ in range(50):
  with lock:assert not competitors(),'quiet gate failed'
  time.sleep(.1)
 for round in range(5):
  pre=fingerprint();start_hits=len(hits);started=utc()
  cases=['latin','dense','placed-shape'];policies=['fresh','retained','prewarmed']
  if round%2:cases.reverse();policies.reverse()
  else:policies=policies[round%3:]+policies[:round%3]
  for case in cases:
   for policy in policies:
    label=f'raw/round{round}/{case}/{policy}';result=launch(case,policy,label)
    rows.append({'round':round,'case':case,'policy':policy,'result':result,'raw':label})
  post=fingerprint();valid=pre==post==initial and not hits[start_hits:]
  save(f'round{round}-fingerprints.json',{'before':pre,'after':post})
  receipts.append({'round':round,'utc_start':started,'utc_end':utc(),'accepted':valid,'competitors':hits[start_hits:],'loadavg':os.getloadavg()});save('round-receipts.json',receipts)
  assert valid,'discard contaminated/changed round'
  save('samples.json',rows);print('round',round,'accepted',flush=True)
finally:
 stop.set();thread.join();save('process-monitor.json',hits)
assert fingerprint()==initial;save('final-fingerprints.json',initial)
print('five rounds complete',flush=True)
