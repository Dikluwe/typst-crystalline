from pathlib import Path
import subprocess,re,hashlib,gzip,json
p=Path(__file__).resolve().parent;pin='/tmp/tekt-linter-none-target/debug/crystalline-lint'
subprocess.run([pin,'.','--checks','v15,v26','--fail-on','warning'],check=True)
files=json.loads(gzip.decompress((p/'occurrences-candidate2-sources.json.gz').read_bytes()))['files']
for f in files:
 if not f.endswith('.rs'):continue
 raw=Path(f).read_bytes();owner=re.search(rb'^//! @prompt (.+)$',raw,re.M)
 if not owner:continue
 code=re.sub(rb'^//! @prompt-hash [0-9a-f]+\n',b'',raw,count=1,flags=re.M)
 prompt=Path(owner[1].decode());s=prompt.read_text();new=re.sub(r'^Hash do Código: [0-9a-f]+$', 'Hash do Código: '+hashlib.sha256(code).hexdigest()[:8],s,count=1,flags=re.M)
 if s!=new:prompt.write_text(new)
subprocess.run([pin,'.','--fix-hashes','--checks','v1,v5,v15,v26','--fail-on','warning'],check=True)
subprocess.run(['python3','tools/ci/check_reverse_lineage.py'],check=True)
