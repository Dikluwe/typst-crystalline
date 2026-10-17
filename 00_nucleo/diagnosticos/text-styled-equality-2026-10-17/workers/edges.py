from pathlib import Path
import datetime,json,hashlib,subprocess
out=Path(__file__).resolve().parents[1];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();cases={'zero':'text(size:0pt)[A]','negative':'text(size:-2pt)[A]','nan':'text(size:float("nan")*1pt)[A]','infinity':'text(size:float("inf")*1pt)[A]','negative-infinity':'text(size:-float("inf")*1pt)[A]','nan-em':'text(size:float("nan")*1em)[A]','infinity-em':'text(size:float("inf")*1em)[A]'};rows=[]
for name,expr in cases.items():
 for kind,path in [('tekt','target/debug/typst'),('vanilla','lab/typst-original/target/release/typst')]:
  cmd=[str(Path(path).resolve()),'eval','repr('+expr+')','--in',str(out/'empty.typ'),'--format','json'];r=subprocess.run(cmd,capture_output=True,text=True,timeout=20);assert r.returncode in [0,1];rows.append(dict(name=name,binary=kind,utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),argv=cmd,binary_sha256=sha(path),exit=r.returncode,stdout=r.stdout,stderr=r.stderr))
(out/'edge-measurement.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),scope='Constructor admission/repr only; nonfinite layout is not certified',rows=rows),indent=2)+'\n');print(json.dumps(rows,indent=2))
