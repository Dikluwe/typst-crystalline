import datetime,gzip,hashlib,importlib.util,json,os,shutil,statistics,subprocess,tempfile,threading,time
from pathlib import Path
ROOT=Path('/repos/Antigravity/typst-crystalline');D=Path(__file__).resolve().parent
s=importlib.util.spec_from_file_location('watch_state',ROOT/'00_nucleo/diagnosticos/watch-retencao-2026-10-07/run.py');r=importlib.util.module_from_spec(s);s.loader.exec_module(r);r.OUT=D
s=importlib.util.spec_from_file_location('monitor',ROOT/'00_nucleo/diagnosticos/catalogo-seletivo-2026-10-07/performance-final/monitor-helper.py');m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
W=Path(tempfile.mkdtemp(prefix='watch-generation-probe-'));probe=W/'probe';shutil.copyfile(ROOT/'target/release/watch-validation-probe',probe);probe.chmod(0o755)
before=r.state('before');(D/'dirty-before.patch.gz').write_bytes(gzip.compress(subprocess.check_output(['git','diff','HEAD','--binary'],cwd=ROOT),mtime=0))
control=subprocess.check_output([str(probe),'control',str(W/'controls')],cwd=ROOT,env=r.b.ENV,text=True);(D/'controls.json').write_text(json.dumps(json.loads(control),indent=2)+'\n')
blockers=[];done=threading.Event()
cmd=[str(probe)];p=subprocess.Popen(cmd,cwd=ROOT,env=r.b.ENV,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,preexec_fn=lambda:os.sched_setaffinity(0,{r.b.CPU}))
def monitor():
 while not done.is_set():
  bad=m.blockers(m.processes(),p.pid)
  if bad:blockers.append({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'processes':bad})
  done.wait(.05)
t=threading.Thread(target=monitor);t.start();out,err=p.communicate();done.set();t.join();assert p.returncode==0,err
result=json.loads(out);after=r.state('after');assert before['files']==after['files'];assert before['binaries']==after['binaries'];assert before['font_files']==after['font_files']
summary={k:statistics.median(x[k] for x in result['samples'])/1e6 for k in ('validation_ns','acquisition_ns')}
rounds={str(i):{k:statistics.median(x[k] for x in result['samples'] if x['round']==i)/1e6 for k in ('validation_ns','acquisition_ns')} for i in range(3)}
result.update(summary_ms=summary,round_medians_ms=rounds,blockers=blockers,command=cmd,stderr=err,cpu=r.b.CPU,probe_sha256=sha(probe),scripts={str(f.relative_to(ROOT)):sha(f) for f in [Path(__file__),D/'probe/src/main.rs',D/'probe/Cargo.toml',D/'probe/Cargo.lock']},fontdb_source={str(f):sha(f) for f in Path('/home/dikluwe/.cargo/registry/src').glob('*/fontdb-0.23.0/src/lib.rs')},note='Validation isolated, not a retained catalog implementation or CLI gain.')
(D/'measurement.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({'summary_ms':summary,'rounds':rounds,'blockers':len(blockers),'controls':json.loads(control)},indent=2),flush=True)
