from pathlib import Path
import subprocess,sys,json,gzip,datetime,hashlib
p=Path(__file__).resolve().parent
candidate=json.loads(gzip.decompress((p/'candidate3-sources.json.gz').read_bytes()))
assert all(Path(f).read_text()==s for f,s in candidate['files'].items())
cmd=['cargo','test','--offline','--locked','-p','typst-core','--lib','--no-fail-fast','--','--test-threads=1']
with (p/'core-candidate3.log').open('w') as out:z=subprocess.run(cmd,stdout=out,stderr=subprocess.STDOUT)
print((p/'core-candidate3.log').read_text()[-10000:],flush=True)
assert all(Path(f).read_text()==s for f,s in candidate['files'].items())
sys.exit(z.returncode)
