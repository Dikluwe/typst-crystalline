from pathlib import Path
import subprocess,json,hashlib,datetime,gzip
root=Path.cwd();out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'))
def sha(p):
 h=hashlib.sha256()
 with Path(p).open('rb') as f:
  for chunk in iter(lambda:f.read(4*1024*1024),b''):h.update(chunk)
 return h.hexdigest()
hashes={p:sha(p) for p in b['hashes']};source=out/'workers/source_table.rs';lib=root/'target/debug/libtypst_core.rlib';binary=Path('/tmp/tekt-source-table-probe-after');cmd=['rustc','--edition=2021',str(source),'-L','dependency='+str(root/'target/debug/deps'),'--extern','typst_core='+str(lib),'-o',str(binary)];record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':b['head'],'command':cmd,'source_hashes':hashes,'library_sha256':sha(lib),'source_sha256':sha(source)};r=subprocess.run(cmd,capture_output=True,text=True);record.update(build_exit=r.returncode,stderr=r.stderr);assert r.returncode==0,r.stderr;r=subprocess.run([str(binary)],capture_output=True,text=True);record.update(exit=r.returncode,stdout=r.stdout,stderr=r.stderr,binary_sha256=sha(binary),sources_unchanged=all(sha(p)==h for p,h in hashes.items()));(out/'source-table-after.json').write_text(json.dumps(record,indent=2)+'\n');assert r.returncode==0 and r.stdout=='plain 0\nsource 0\n' and record['sources_unchanged'];print(r.stdout,flush=True)
