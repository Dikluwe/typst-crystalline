from pathlib import Path
import subprocess,os,time,json,gzip,datetime,hashlib,shutil,threading,importlib.util,itertools,platform,xml.etree.ElementTree as ET
D=Path(__file__).resolve().parent;R=D.parents[3]
spec=importlib.util.spec_from_file_location('bench_monitor',D/'monitor-helper.py');c=importlib.util.module_from_spec(spec);spec.loader.exec_module(c);c.OUT=D;c.ROOT=R;c.FONT_CACHE_PATH=D/'font-digest-cache.json'
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
B={'current':R/'target/release/typst','previous':Path('/tmp/coverage-ranges-2026-10-07/previous-0a2410ec'),'vanilla':R/'lab/typst-original/target/release/typst'}
expected={'current':sha(B['current']),'previous':'0a2410ec5b4a075417943d25a4d24ac860da6ff0adf4845707d4f18dc9cc7add','vanilla':'7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'}
W=Path('/tmp/tekt-performance-coverage-2026-10-07');W.mkdir(exist_ok=True)
for name,p in list(B.items()):
 assert sha(p)==expected[name],(name,sha(p));q=W/(name+'-'+expected[name]);shutil.copyfile(p,q);q.chmod(0o755);B[name]=q
ENV=dict(c.ENV);ENV['NO_COLOR']='1'
fontdir=R/'00_nucleo/diagnosticos/performance-baseline/fonts'
paths=set()
for base in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts','.cargo']:
 for p in (R/base).rglob('*'):
  if p.is_file() and not any(x in {'target','__pycache__','.git'} for x in p.parts):paths.add(p)
for name in ['Cargo.toml','Cargo.lock','rust-toolchain','rust-toolchain.toml','build.rs']:
 if (R/name).is_file():paths.add(R/name)
paths.update(p for p in (D/'fixtures').glob('*') if p.is_file());paths.update(p for p in fontdir.glob('*') if p.is_file());paths.update([D/'measure.py',D/'monitor-helper.py',D/'protocol.md'])
for name in set(subprocess.check_output(['fc-list','--format','%{file}\n'],text=True).splitlines()):
 if Path(name).is_file():paths.add(Path(name))
for root in ['/usr/share/fonts','/usr/local/share/fonts','/home/dikluwe/.local/share/fonts','/usr/share/texmf/fonts','/etc/fonts']:
 for p in Path(root).rglob('*'):
  if p.is_file():paths.add(p)
embedded=R/'00_nucleo/diagnosticos/performance-baseline/embedded-fonts.json'
if embedded.exists():
 for row in json.loads(embedded.read_text()):
  for key in ['tekt_path','vanilla_path']:
   p=Path(row[key]);
   if p.is_file():paths.add(p)
c.source_files=lambda:sorted(paths)
# Archives supplement patches with dirty/untracked full consumer content.
source={str(p.relative_to(R)):p.read_text() for p in paths if p.is_relative_to(R) and p.suffix in {'.rs','.toml','.md','.typ','.lock'} and p.stat().st_size<5000000}
(D/'consumer-inputs.json.gz').write_bytes(gzip.compress(json.dumps(source,ensure_ascii=False).encode()))
for receipt in ['tested-final.json.gz','seals.json']:
 p=R/'00_nucleo/diagnosticos/coverage-ranges-2026-10-07'/receipt;shutil.copyfile(p,D/('current-'+receipt))
shutil.copyfile(R/'00_nucleo/diagnosticos/table-local-c2r1-2026-10-07/tested-final.json.gz',D/'previous-tested-final.json.gz')
current=json.loads(gzip.decompress((D/'current-tested-final.json.gz').read_bytes()))
for p,digest in current['sha256'].items():
 if p.startswith(('01_core/','02_shell/','03_infra/','04_wiring/','00_nucleo/prompts/')):assert sha(R/p)==digest,p
meta={'utc':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','bin_sha256':expected,'binaries':{k:str(v) for k,v in B.items()},'environment_overrides':{k:ENV[k] for k in ['RAYON_NUM_THREADS','SOURCE_DATE_EPOCH','TZ','LC_ALL','NO_COLOR']},'host':platform.uname()._asdict(),'cpu':subprocess.check_output(['lscpu'],text=True),'affinity':4,'siblings':Path('/sys/devices/system/cpu/cpu4/topology/thread_siblings_list').read_text(),'governor':Path('/sys/devices/system/cpu/cpu4/cpufreq/scaling_governor').read_text(),'fixtures':{p.name:{'source':p.read_text(),'sha256':sha(p)} for p in (D/'fixtures').glob('*.typ')},'font_files':{str(p):sha(p) for p in fontdir.glob('*') if p.is_file()}}
(D/'identity.json').write_text(json.dumps(meta,indent=2,ensure_ascii=False)+'\n')
print('prepared; quiet gate 30s',flush=True)
start_fp=c.snapshot('start');gate=[];deadline=time.monotonic()+30
while time.monotonic()<deadline:
 hits=c.blockers(c.processes(),None);gate.append({'utc':utc(),'blockers':hits});assert not hits,hits;time.sleep(1)
after_gate=c.snapshot('quiet-after');assert start_fp['sha256']==after_gate['sha256'];(D/'quiet-gate.json').write_text(json.dumps(gate,indent=2)+'\n')
def one(engine,fixture,directory,label):
 assert sha(B[engine])==expected[engine];pdf=directory/(label+'.pdf');cmd=[str(B[engine]),'compile',str(fixture),str(pdf),'--root',str(fixture.parent),'--pdf-tagged=false','--pretty=false','--font-path',str(fontdir)]
 if engine=='vanilla':cmd+=['--jobs','1']
 metric=directory/(label+'.rss');cmd=['/usr/bin/time','-f','%M','-o',str(metric),'/usr/bin/taskset','-c','4',*cmd]
 with (directory/(label+'.stdout')).open('wb') as out,(directory/(label+'.stderr')).open('wb') as err:
  start=utc();tick=time.perf_counter_ns();p=subprocess.Popen(cmd,cwd=R,env=ENV,stdout=out,stderr=err);pid,status,usage=os.wait4(p.pid,0);elapsed=(time.perf_counter_ns()-tick)/1e6;p.returncode=os.waitstatus_to_exitcode(status)
 assert sha(B[engine])==expected[engine];row={'engine':engine,'start_utc':start,'end_utc':utc(),'command':cmd,'cwd':str(R),'wall_ms':elapsed,'user_ms':usage.ru_utime*1000,'system_ms':usage.ru_stime*1000,'maxrss_kib':int(metric.read_text().strip()),'voluntary_context_switches':usage.ru_nvcsw,'involuntary_context_switches':usage.ru_nivcsw,'status':p.returncode,'stderr':(directory/(label+'.stderr')).read_text(),'stdout':(directory/(label+'.stdout')).read_text(),'pdf':str(pdf),'pdf_sha256':sha(pdf) if pdf.exists() else None}
 assert p.returncode==0,(engine,fixture,row);return row
NS={'x':'http://www.w3.org/1999/xhtml'}
def output(row):
 pdf=row['pdf'];bbox=subprocess.check_output(['pdftotext','-bbox',pdf,'-'],text=True);tree=ET.fromstring(bbox);pages=[e.attrib for e in tree.iter() if e.tag.endswith('page')];words=[{'text':''.join(e.itertext()),'box':e.attrib} for e in tree.iter() if e.tag.endswith('word')]
 xml=subprocess.check_output(['pdftohtml','-xml','-stdout','-i',pdf],text=True);fonts=[e.attrib for e in ET.fromstring(xml).iter('fontspec')]
 return {'text':subprocess.check_output(['pdftotext',pdf,'-'],text=True),'pages':pages,'words':words,'fonts':fonts,'pdffonts':subprocess.check_output(['pdffonts',pdf],text=True)}
orders=list(itertools.permutations(B));results=[]
for rnd in [1,2,3]:
 for fixture in sorted((D/'fixtures').glob('*.typ')):
  label=f'r{rnd}-{fixture.stem}';directory=D/'samples'/label;directory.mkdir(parents=True,exist_ok=False);before=c.snapshot(label+'-before');contamination=[];telemetry=[];stop=threading.Event()
  def monitor():
   while not stop.is_set():
    hits=c.blockers(c.processes(),None);contamination.extend(hits);telemetry.append({'utc':utc(),'blockers':hits,'loadavg':Path('/proc/loadavg').read_text().strip()});stop.wait(.1)
  assert not c.blockers(c.processes(),None);thread=threading.Thread(target=monitor,daemon=True);thread.start();rows=[];qual={};warmups=[]
  try:
   for warm in range(2):
    for engine in B:warmups.append(one(engine,fixture,directory,f'warm{warm}-{engine}'))
   for pair in range(12):
    for engine in orders[(pair+rnd-1)%6]:
     row=one(engine,fixture,directory,f'p{pair:02d}-{engine}');row.update(pair=pair,round=rnd,fixture=fixture.stem);rows.append(row)
  finally:stop.set();thread.join()
  after=c.snapshot(label+'-after');valid=before['sha256']==after['sha256'] and not contamination
  for row in rows:
   if row['pair']==0:qual[row['engine']]=output(row)
  block={'valid':valid,'source_stable':before['sha256']==after['sha256'],'fingerprint_before':before['sha256'],'fingerprint_after':after['sha256'],'warmups':warmups,'samples':rows,'qualification':qual,'contamination':contamination,'telemetry':telemetry};p=directory/'samples.json';p.write_text(json.dumps(block,ensure_ascii=False,indent=2)+'\n');results.append(str(p.relative_to(D)));(D/'index.json').write_text(json.dumps({'utc':utc(),'blocks':results},indent=2)+'\n');print(label,'VALID' if valid else 'REJECTED',flush=True);assert valid,(label,contamination)
end=c.snapshot('end');assert end['sha256']==start_fp['sha256'];assert all(sha(p)==expected[k] for k,p in B.items());print('FINISHED',flush=True)
