import datetime,gzip,hashlib,json,os,subprocess,importlib.util
from pathlib import Path
D=Path(__file__).resolve().parent;R=D.parents[2];O=Path('/tmp/memoria-fases3-2026-10-08');O.mkdir(exist_ok=True)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
state=json.loads(gzip.open(D/'state.json.gz','rt').read());assert sha(R/'target/release/typst')==state['binary_sha256']
spec=importlib.util.spec_from_file_location('monitor',D.parent/'performance-c2-2026-10-07/monitor-helper.py');monitor=importlib.util.module_from_spec(spec);spec.loader.exec_module(monitor)
cases={n:D.parent/'snapshot-scopes-prioridade7-2026-10-08/fixtures'/(n+'.typ') for n in ['context10','context100','plain10','plain100']}
old=json.loads(gzip.open(D.parent/'embedded-fonts-memoria2-2026-10-08/bench.json.gz','rt').read());identity=old['input_sha256'].copy();identity.update({str(p):sha(p) for p in cases.values()});identity[str(D/'profile.gdb')]=sha(D/'profile.gdb');assert all(sha(Path(p))==h for p,h in identity.items())
env=os.environ.copy()
for k in list(env):
 if k.startswith(('TYPST_','CRYSTALLINE_','CARGO_PROFILE_')):env.pop(k)
env.update(RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',NO_COLOR='1',TZ='UTC',LC_ALL='C');cpu=min(os.sched_getaffinity(0));rows=[];r={'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'binary_sha256':state['binary_sha256'],'head':state['head'],'state_sha256':sha(D/'state.json.gz'),'inputs':identity,'cpu':cpu,'environment':{k:env[k] for k in ['RAYON_NUM_THREADS','SOURCE_DATE_EPOCH','NO_COLOR','TZ','LC_ALL']},'cases':rows}
for repeat in range(2):
 for name,fixture in sorted(cases.items()):
  assert not monitor.blockers(monitor.processes(),None)
  common=['compile',str(fixture),str(O/(name+'-profile.pdf')),'--root',str(fixture.parent),'--pdf-tagged=false','--pretty=false','--font-path',str(D.parent/'performance-baseline/fonts')]
  cmd=['gdb','-q','-nx','-batch','-x',str(D/'profile.gdb'),'--args',str(R/'target/release/typst')]+common
  p=subprocess.run(cmd,cwd=R,env=env,text=True,capture_output=True,timeout=90,preexec_fn=lambda:os.sched_setaffinity(0,{cpu}));log=p.stdout+p.stderr;parts=[x for x in log.splitlines() if x.startswith('MEMORY_PHASES ')];assert p.returncode==0 and len(parts)==1,(name,log[-1000:]);data=json.loads(parts[0][len('MEMORY_PHASES '):]);assert data['inferior_exit_code']==0 and data['rows'],(name,data)
  profile_hash=sha(O/(name+'-profile.pdf'));direct=O/(name+'-direct.pdf');usage=O/'usage.txt';directcmd=['/usr/bin/time','-f','%M','-o',str(usage),str(R/'target/release/typst')]+[str(direct) if a==str(O/(name+'-profile.pdf')) else a for a in common]
  p=subprocess.run(directcmd,cwd=R,env=env,text=True,capture_output=True,timeout=60,preexec_fn=lambda:os.sched_setaffinity(0,{cpu}));assert p.returncode==0,(name,p.stderr)
  text=lambda f:subprocess.check_output(['pdftotext','-bbox',str(f),'-'],text=True)
  same=text(O/(name+'-profile.pdf'))==text(direct);assert same,name
  rows.append({'repeat':repeat,'case':name,'command':cmd,'direct_command':directcmd,'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'profile':data,'direct_max_rss_kib':int(usage.read_text()),'bbox_xml_equal':same,'profile_pdf_sha256':profile_hash,'direct_pdf_sha256':sha(direct),'gdb_notes':[x for x in log.splitlines() if x.startswith(('MISSING','warning:'))]});print(repeat,name,'direct',rows[-1]['direct_max_rss_kib'],'stops',len(data['rows']),flush=True)
assert all(sha(Path(p))==h for p,h in identity.items());assert all((R/n).read_text()==v for n,v in state['files'].items());assert sha(R/'target/release/typst')==state['binary_sha256'];r['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(D/'profiles-growth.json.gz','wt') as f:json.dump(r,f)
print('SAVED',len(rows),flush=True)
