from pathlib import Path
import subprocess,json,gzip,hashlib,datetime,sys
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tekt-v16-introspect-walk-2026-10-16';phase=sys.argv[1];dest=out/('measurement-'+phase);dest.mkdir(exist_ok=False);before=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':before['head'],'sources':{p:sha(p) for p in before['hashes']},'binaries':{},'cases':[]}
for binary in ['lab/typst-original/target/release/typst','target/debug/typst']:
 record['binaries'][binary]=sha(binary)
 for fixture in sorted((out/'fixtures').glob('*.typ')):
  name=('vanilla' if binary.startswith('lab/') else 'tekt')+'-'+fixture.stem
  pdf=dest/(name+'.pdf');cmd=[str(root/binary),'compile',str(fixture),str(pdf)]
  r=subprocess.run(cmd,capture_output=True,text=True);(dest/(name+'-stderr.log')).write_text(r.stderr)
  item={'binary':binary,'fixture':str(fixture.relative_to(root)),'fixture_sha256':sha(fixture),'command':cmd,'exit':r.returncode,'stderr':r.stderr,'stdout':r.stdout}
  if r.returncode==0:
   t=subprocess.run(['pdftotext','-layout',str(pdf),'-'],capture_output=True,text=True);assert t.returncode==0;item['text']=t.stdout;(dest/(name+'.txt')).write_text(t.stdout);item['pdf_sha256']=sha(pdf)
  qcmd=[str(root/binary),'query',str(fixture),'outline.entry','--field','level'];q=subprocess.run(qcmd,capture_output=True,text=True);item['query']={'command':qcmd,'exit':q.returncode,'stdout':q.stdout,'stderr':q.stderr}
  record['cases'].append(item);print(json.dumps(item),flush=True)
record['sources_unchanged']=all(sha(p)==h for p,h in record['sources'].items());assert record['sources_unchanged'];(out/('measurement-'+phase+'.json')).write_text(json.dumps(record,indent=2)+'\n')
