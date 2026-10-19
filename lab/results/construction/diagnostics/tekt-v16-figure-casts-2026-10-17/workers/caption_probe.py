from pathlib import Path
import subprocess,json,datetime,sys,hashlib
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];cases={'omitted':'figure([A]).caption','none':'figure([A],caption:none).caption','empty':'figure([A],caption:[]).caption','content':'figure([A],caption:[C]).caption','explicit-caption':'figure([A],caption:figure.caption([C])).caption','body':'figure([A],caption:[C]).body'};rows=[]
for name,expr in cases.items():
 for kind,p in [('tekt','target/debug/typst'),('vanilla','lab/typst-original/target/release/typst')]:
  cmd=[str(Path(p).resolve()),'eval',expr,'--in',str(out/'empty.typ'),'--format','json'];r=subprocess.run(cmd,capture_output=True,text=True);rows.append(dict(name=name,binary=kind,expression=expr,argv=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr))
(out/('caption-'+phase+'.json')).write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source_hashes=json.load(open(out/'validation-result.json'))['source_hashes'],binaries={p:hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in ['target/debug/typst','lab/typst-original/target/release/typst']},rows=rows),indent=2)+'\n')
for r in rows:print(r['name'],r['binary'],r['exit'],r['stdout'].strip(),r['stderr'].splitlines()[:1])
