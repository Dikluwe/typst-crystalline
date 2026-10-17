from pathlib import Path
import json,hashlib,gzip,datetime,subprocess
out=Path('00_nucleo/diagnosticos/clippy-enums-publicos-2026-10-16');now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(p):
 h=hashlib.sha256()
 with Path(p).open('rb') as f:
  while b:=f.read(1024*1024):h.update(b)
 return h.hexdigest()
before=json.load(gzip.open(out/'before.json.gz'));assert all(sha(p)==h for p,h in before['hashes'].items())
rlib=Path('target/debug/libtypst_core.rlib');eco=next(Path('target/debug/deps').glob('libecow-*.rlib'))
cmd=['rustc','--edition','2021','--crate-name','public_enums_probe','-O',str(out/'probe.rs'),'-L','dependency=target/debug/deps','--extern','typst_core='+str(rlib),'--extern','ecow='+str(eco),'-o',str(out/'probe')]
start=now();r=subprocess.run(cmd,capture_output=True,text=True);record=dict(start=start,head=before['head'],command=cmd,build_exit=r.returncode,build_stdout=r.stdout,build_stderr=r.stderr,rustc=subprocess.check_output(['rustc','-Vv'],text=True),inputs={str(p):sha(p) for p in [out/'probe.rs',rlib,eco]});(out/'probe-details.json').write_text(json.dumps(record,indent=2)+'\n');assert r.returncode==0,r.stderr
r=subprocess.run([str(out/'probe')],capture_output=True,text=True);record.update(end=now(),exit=r.returncode,stdout=r.stdout,stderr=r.stderr,binary_sha256=sha(out/'probe'),sources_unchanged=all(sha(p)==h for p,h in before['hashes'].items()));(out/'probe-details.json').write_text(json.dumps(record,indent=2)+'\n');assert r.returncode==0,r.stderr
sizes={};samples=[]
for line in r.stdout.splitlines():
 s=line.split('\t')
 if s[0]=='size':sizes[s[1]]=dict(size=int(s[2]),align=int(s[3]))
 else:samples.append(dict(label=s[1],representation=['inline','box','arc'][int(s[2])],n=int(s[3]),style_pct=int(s[4]),rich=s[5]=='true',keep_snapshots=s[6]=='true',allocations=int(s[7]),allocated_bytes=int(s[8]),peak_requested_bytes=int(s[9]),live_requested_bytes=int(s[10])))
(out/'measurements.json').write_text(json.dumps(dict(sizes=sizes,samples=samples,limitations='Synthetic carrier/clone/registration controls using actual domain values and mirrored candidates; no full compile/layout comparison, no general workload distribution, requested allocator bytes exclude allocator metadata/RSS. No product change.'),indent=2)+'\n');print(json.dumps(dict(sizes=sizes,samples=len(samples))),flush=True)
