from pathlib import Path
import subprocess,json,hashlib
out=Path(__file__).resolve().parent
resource=json.loads((out/'vanilla-build-resources.json').read_text());assert resource['exit']==0
source=Path('lab/typst-original');binary=source/'target/release/typst'
def sha(path):
 d=hashlib.sha256()
 with path.open('rb') as f:
  for chunk in iter(lambda:f.read(1024*1024),b''):d.update(chunk)
 return d.hexdigest()
record=dict(resource,pin='bd5ebec285505889e8ff1d4f68804ab147a699f3',upstream_tree='cac836c07e4dec2a89219370428292393a3216b5',binary_path=str(binary.resolve()),binary_sha256=sha(binary),cargo_lock_sha256=sha(source/'Cargo.lock'),rustc=subprocess.check_output(['rustc','+1.92.0','-Vv'],text=True),version_auxiliary=subprocess.check_output([str(binary),'--version'],text=True).strip(),sys_version=subprocess.check_output([str(binary),'eval','sys.version','--format','json'],text=True).strip(),source_verification=json.loads((out/'source-verification.json').read_text()))
(out/'oracle-build.json').write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps({k:record[k] for k in ['pin','binary_sha256','version_auxiliary','sys_version']},indent=2))
