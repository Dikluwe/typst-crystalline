from pathlib import Path
import json,gzip,datetime,subprocess,hashlib,sys
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();bins={'tekt':'target/debug/typst','vanilla':'lab/typst-original/target/release/typst'};cases=[]
for expr in ['{ set std.html(pretty: panic("executed")); 42 }','{ set std.html(pretty: panic("executed")) if false; 42 }','{ set std.format.html(pretty: panic("executed")); 42 }','{ let m = std; set m.html(pretty: panic("executed")); 42 }','{ set missing_target(); 42 }','{ set missing_target(x: panic("executed")); 42 }','{ set std.missing_target(x: panic("executed")); 42 }','{ set (count: 42).count(x: panic("executed")); 42 }','{ set text(size: 12pt); 42 }','{ set heading(numbering: "1"); 42 }','std.html(panic("executed"))','format.html(panic("executed"))','std.asset(panic("executed"))','format.bundle(panic("executed"))']:
 for enabled in [False,True]:
  result={}
  for name,bin in bins.items():
   cmd=[bin,'eval',expr,'--in',str(out/'empty.typ'),'--format','json']+(['--features','html,bundle'] if enabled else []);r=subprocess.run(cmd,capture_output=True,text=True,timeout=30);result[name]=dict(argv=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr)
  cases.append(dict(expression=expr,enabled=enabled,results=result))
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes={p:sha(p) for p in b['hashes']},binaries={n:dict(path=p,sha256=sha(p)) for n,p in bins.items()},cases=cases)
with gzip.open(out/(phase+'-set-probe.json.gz'),'wt') as f:json.dump(record,f)
for c in cases:
 if not c['enabled']:print(c['expression'],[(k,v['exit'],v['stdout'].strip(),v['stderr'].splitlines()[:1]) for k,v in c['results'].items()])
