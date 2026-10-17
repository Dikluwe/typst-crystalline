from pathlib import Path
import datetime,gzip,json,hashlib,subprocess,sys
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];assert phase in ['before','after'];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in b['hashes']};empty=out/'empty.typ';empty.write_text('');rows=[];bins={k:Path(p).resolve() for k,p in [('tekt','target/debug/typst'),('vanilla','lab/typst-original/target/release/typst')]}
for val in ['none','auto','true','1','1.5','"x"','50%','2em','-5pt','0pt']:
 for kind,binary in bins.items():
  expression='repr(par(spacing:'+val+')[])';cmd=[str(binary),'eval',expression,'--in',str(empty),'--format','json'];r=subprocess.run(cmd,capture_output=True,text=True);assert r.returncode in [0,1];rows.append(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),value=val,binary=kind,expression=expression,argv=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr))
record=dict(head=b['head'],source_hashes=hashes,binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in bins.items()},status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),cases=rows)
with gzip.open(out/('spacing-types-'+phase+'.json.gz'),'wt') as f:json.dump(record,f)
for r in rows:print(r['value'],r['binary'],r['exit'],r['stderr'].splitlines()[:1])
