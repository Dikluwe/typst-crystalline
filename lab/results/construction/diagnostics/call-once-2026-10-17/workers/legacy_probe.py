from pathlib import Path
import subprocess,json,datetime,hashlib
out=Path(__file__).resolve().parents[1];source=out/'legacy-array.typ';source.write_text('#array.nope');rows=[]
for bin in ['target/debug/typst','lab/typst-original/target/release/typst']:
 cmd=[bin,'eval','0','--in',str(source),'--format','json'];r=subprocess.run(cmd,capture_output=True,text=True);rows.append(dict(argv=cmd,binary_sha256=hashlib.sha256(Path(bin).read_bytes()).hexdigest(),exit=r.returncode,stdout=r.stdout,stderr=r.stderr))
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),rows=rows);(out/'legacy-array-probe.json').write_text(json.dumps(record,indent=2)+'\n');print(rows);assert all(r['exit']==1 for r in rows);assert rows[0]['stderr']==rows[1]['stderr']
