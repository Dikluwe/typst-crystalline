from pathlib import Path
import gzip,json,hashlib,datetime,subprocess
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in b['hashes'].items());rows=[];binaries={k:Path(p).resolve() for k,p in [('tekt','target/debug/typst'),('vanilla','lab/typst-original/target/release/typst')]}
for name in ['styled-par','styled-empty','styled-empty-text','inline-style-control']:
 src=out/'attempt-positional-size/before-pdf'/(name+'.typ')
 for kind,bin in binaries.items():
  cmd=[str(bin),'compile',str(src),str(out/'attempt-positional-size'/('replay-'+name+'-'+kind+'.pdf')),'--font-path','/usr/share/fonts/truetype/dejavu'];r=subprocess.run(cmd,capture_output=True,text=True);rows.append(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),name=name,binary=kind,source_sha256=sha(src),argv=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr));assert r.returncode==(1 if kind=='tekt' else 0)
r=dict(head=b['head'],source_hashes=b['hashes'],status=b['status'],diffstat=b['diffstat'],binary_hashes={k:sha(p) for k,p in binaries.items()},cases=rows,conclusion='Four valid positional-size constructions accepted by vanilla and rejected by Tekt before paragraph layout. Remains open, separate from Source paragraph classification.')
with gzip.open(out/'positional-size.json.gz','wt') as f:json.dump(r,f)
print('Confirmed 4 positional-size divergences; separate open debt.')
