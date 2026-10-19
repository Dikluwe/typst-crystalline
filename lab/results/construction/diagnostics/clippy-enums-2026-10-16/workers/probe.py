from pathlib import Path
import subprocess,json,datetime,hashlib
out=Path('00_nucleo/diagnosticos/clippy-enums-2026-10-16')
rlib=Path('target/debug/libtypst_core.rlib');eco=next(Path('target/debug/deps').glob('libecow-*.rlib'))
def sha(p):
 h=hashlib.sha256()
 with Path(p).open('rb') as f:
  while chunk:=f.read(1024*1024):h.update(chunk)
 return h.hexdigest()
cmd=['rustc','--edition','2021','--crate-name','enum_probe','-O',str(out/'enum_probe.rs'),'-L','dependency=target/debug/deps','--extern','typst_core='+str(rlib),'--extern','ecow='+str(eco),'-o',str(out/'enum-probe')]
start=datetime.datetime.now(datetime.timezone.utc).isoformat();r=subprocess.run(cmd,capture_output=True,text=True)
record=dict(start=start,command=cmd,build_exit=r.returncode,build_stdout=r.stdout,build_stderr=r.stderr,rlib_sha256=sha(rlib),rustc=subprocess.check_output(['rustc','-Vv'],text=True))
(out/'probe-details.json').write_text(json.dumps(record,indent=2)+'\n');assert r.returncode==0,r.stderr
x=subprocess.run([str(out/'enum-probe')],capture_output=True,text=True);record.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(),exit=x.returncode,stdout=x.stdout,stderr=x.stderr,binary_sha256=sha(out/'enum-probe'))
(out/'probe-details.json').write_text(json.dumps(record,indent=2)+'\n');assert x.returncode==0
sizes={};samples=[]
for line in x.stdout.splitlines():
 p=line.split('\t')
 if p[0]=='size':sizes[p[1]]=dict(size=int(p[2]),alignment=int(p[3]))
 elif p[0]=='transport':samples.append(dict(sample=int(p[1]),kind=int(p[2]),allocations=int(p[3]),bytes=int(p[4]),nanos=int(p[5]),iterations=20000))
assert sizes['context_original']['size']==432
assert all(s['allocations']==(20000 if s['kind']%2 else 0) for s in samples),samples
(out/'measurements.json').write_text(json.dumps(dict(sizes=sizes,transport=samples,limitations='x86_64 Rust 1.92; shadow private enum copied from source; Complete-only transport control, not whole layout or production workload; allocator observer unsafe only in diagnostic executable'),indent=2)+'\n')
print(json.dumps(dict(sizes=sizes,transport_samples=len(samples))),flush=True)
