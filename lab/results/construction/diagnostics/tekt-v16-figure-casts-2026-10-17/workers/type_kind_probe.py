from pathlib import Path
import subprocess,json,gzip,datetime,hashlib
out=Path(__file__).resolve().parents[1];expressions=['int','str','array','float','bool','type','type(1)','type("A")','type((1,2))','type(ltr)','type(left)','type(none)','type(auto)','type([A])','type(1fr)','type(50%)','type(1pt)','type(1deg)','type(assert)','type(red)'];rows=[]
for expr in expressions:
 for binary in ['target/debug/typst','lab/typst-original/target/release/typst']:
  for query in ['figure([A],kind:'+expr+')','type('+expr+')']:
   cmd=[str(Path(binary).resolve()),'eval',query,'--in',str(out/'empty.typ'),'--format','json'];r=subprocess.run(cmd,capture_output=True,text=True);rows.append(dict(binary=binary,query=query,exit=r.returncode,stdout=r.stdout,stderr=r.stderr,argv=cmd))
(out/'type-kind-probe.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source_hashes=json.load(open(out/'validation-result.json'))['source_hashes'],binaries={p:hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in ['target/debug/typst','lab/typst-original/target/release/typst']},rows=rows),indent=2)+'\n')
for r in rows:
 if r['binary'].startswith('lab/'):print(r['query'],r['exit'],r['stdout'].strip(),r['stderr'].splitlines()[:1])
