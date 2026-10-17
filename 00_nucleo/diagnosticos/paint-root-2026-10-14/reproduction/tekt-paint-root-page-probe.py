from pathlib import Path
import json,gzip,hashlib,datetime,subprocess,sys
out=Path('00_nucleo/diagnosticos/paint-root-2026-10-14')/sys.argv[1];out.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();root=Path.cwd()
with gzip.open(out.parent/'before/results.json.gz','rt') as f:baseline=json.load(f)
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),binary_source_state='For before: same T1a binary as before/results.json.gz; current source already contains unbuilt paint-root edits.',rows=[])
for clip in ['false','true']:
 src=out/('clip-'+clip+'.typ');src.write_text('#block(width:60pt,height:40pt,inset:3pt,clip:'+clip+',fill:blue)[#rect(width:8pt,height:6pt,fill:red,stroke:none)]')
 for label,exe in [('tekt',root/'target/debug/typst'),('vanilla',root/'lab/typst-original/target/release/typst')]:
  if sys.argv[1].endswith('before'):assert sha(exe)==baseline['binaries'][label]['sha256']
  pdf=out/('clip-'+clip+'-'+label+'.pdf');cmd=[str(exe),'compile',str(src),str(pdf)];c=subprocess.run(cmd,capture_output=True,text=True);q=subprocess.run(['qpdf','--show-npages',str(pdf)],capture_output=True,text=True);check=subprocess.run(['qpdf','--check',str(pdf)],capture_output=True,text=True)
  row=dict(case=clip,binary=label,binary_sha256=sha(exe),source=src.read_text(),command=cmd,exit=c.returncode,stdout=c.stdout,stderr=c.stderr,pages=q.stdout.strip(),qpdf_exit=check.returncode,qpdf_stdout=check.stdout,qpdf_stderr=check.stderr);r['rows'].append(row);print(clip,label,row['pages'],flush=True)
with gzip.open(out/'results.json.gz','wt') as f:json.dump(r,f)
